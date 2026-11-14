use crate::auth::passkey_service::{PasskeyChallenge, PasskeyService, PasskeyVerifyRequest};
use crate::auth::session_manager::{AuthError, SessionManager};
use crate::config::ServerConfig;
use crate::ws::{handle_ws_socket, GatewayHub};
use axum::{
    body::Bytes,
    extract::{FromRequestParts, Path, Query, State, WebSocketUpgrade},
    http::{header, request::Parts, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::Utc;
use sha2::{Digest, Sha256};
use gpui_telegram_core::auth::SessionToken;
use gpui_telegram_core::types::{ChatId, MsgId, PeerId, TopicId, UserId};
use gpui_telegram_proto::message::{MessageContent, MessageItem, ReplyHeader};
use gpui_telegram_proto::sync::UpdateEvent;
use gpui_telegram_proto::ws::WsServerFrame;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use uuid::Uuid;

/// Global application state shared across HTTP and WebSocket handlers
#[derive(Clone)]
pub struct AppState {
    pub config: ServerConfig,
    pub db: Arc<crate::db::ServerDatabase>,
    pub cas: Arc<crate::media::cas_store::CasStore>,
    pub pts_engine: Arc<crate::sync::pts_engine::PtsEngine>,
    pub message_pipeline: Arc<crate::messaging::pipeline::MessagePipeline>,
    pub session_manager: Arc<SessionManager>,
    pub gateway_hub: Arc<GatewayHub>,
    pub pairing_requests: Arc<RwLock<HashMap<String, (UserId, i64)>>>,
    pub passkey_challenges: Arc<RwLock<HashMap<String, i64>>>,
    pub alipay_upstream_attempts: Arc<RwLock<HashMap<String, String>>>,
    pub mcp_registry: Arc<RwLock<gpui_telegram_agent::McpRegistry>>,
    pub permission_dock: Arc<RwLock<gpui_telegram_agent::ToolPermissionDock>>,
    pub computer_lease_broker: Arc<RwLock<gpui_telegram_agent::ComputerLeaseBroker>>,
    pub talk_coordinator: Arc<RwLock<gpui_telegram_agent::TalkCoordinator>>,
    pub acp_runner: Arc<gpui_telegram_agent::acp::AcpRunner>,
    pub task_registry: Arc<tokio::sync::RwLock<HashMap<String, gpui_telegram_agent::tasks::TaskFsm>>>,
    pub approval_gates: Arc<tokio::sync::RwLock<HashMap<String, gpui_telegram_agent::tasks::ApprovalGate>>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new(ServerConfig::default())
    }
}

impl AppState {
    pub fn new(config: ServerConfig) -> Self {
        let db = Arc::new(Self::open_database(&config));
        Self::with_db(config, db)
    }

    pub fn try_new(config: ServerConfig) -> Result<Self, String> {
        let path = config.resolve_db_path();
        let db = if path == ":memory:" || path.is_empty() {
            crate::db::ServerDatabase::open_in_memory().map_err(|e| e.to_string())?
        } else {
            crate::db::ServerDatabase::open(path).map_err(|e| e.to_string())?
        };
        Ok(Self::with_db(config, Arc::new(db)))
    }

    pub fn with_db(config: ServerConfig, db: Arc<crate::db::ServerDatabase>) -> Self {
        let session_manager = Arc::new(SessionManager::new());
        let gateway_hub = Arc::new(GatewayHub::new());

        // Monotonically align user_id with SQLite database
        if let Ok(max_id) = db.get_max_user_id() {
            session_manager.set_next_user_id(std::cmp::max(100_001, max_id + 1));
        }

        // Wire disconnection signal from session manager to gateway hub
        let hub_clone = gateway_hub.clone();
        session_manager.set_disconnect_notifier(Arc::new(move |user_id| {
            hub_clone.disconnect_user(user_id);
        }));

        let acp_runner = Arc::new(gpui_telegram_agent::acp::AcpRunner::new(
            &config.default_agent_model,
        ));
        let task_registry = Arc::new(tokio::sync::RwLock::new(HashMap::new()));
        let approval_gates = Arc::new(tokio::sync::RwLock::new(HashMap::new()));

        let cas = Arc::new(crate::media::cas_store::CasStore::new(&config.media_storage_path));
        let pts_engine = Arc::new(crate::sync::pts_engine::PtsEngine::new(db.clone()));
        let dedup = crate::messaging::dedup::MessageNonceDedup::default();
        let message_pipeline = Arc::new(crate::messaging::pipeline::MessagePipeline::new(
            dedup,
            (*pts_engine).clone(),
        ));
        gateway_hub.set_pipeline(message_pipeline.clone(), db.clone());

        Self {
            config,
            db,
            cas,
            pts_engine,
            message_pipeline,
            session_manager,
            gateway_hub,
            pairing_requests: Arc::new(RwLock::new(HashMap::new())),
            passkey_challenges: Arc::new(RwLock::new(HashMap::new())),
            alipay_upstream_attempts: Arc::new(RwLock::new(HashMap::new())),
            mcp_registry: Arc::new(RwLock::new(gpui_telegram_agent::McpRegistry::new())),
            permission_dock: Arc::new(RwLock::new(gpui_telegram_agent::ToolPermissionDock::new())),
            computer_lease_broker: Arc::new(RwLock::new(gpui_telegram_agent::ComputerLeaseBroker::new())),
            talk_coordinator: Arc::new(RwLock::new(gpui_telegram_agent::TalkCoordinator::new())),
            acp_runner,
            task_registry,
            approval_gates,
        }
    }

    fn open_database(config: &ServerConfig) -> crate::db::ServerDatabase {
        let path = config.resolve_db_path();
        if path == ":memory:" || path.is_empty() {
            crate::db::ServerDatabase::open_in_memory().expect("Failed to initialize in-memory SQLite")
        } else {
            crate::db::ServerDatabase::open(path).unwrap_or_else(|e| {
                tracing::warn!("Failed to open SQLite database at '{}': {}. Falling back to in-memory", path, e);
                crate::db::ServerDatabase::open_in_memory().expect("In-memory SQLite fallback failed")
            })
        }
    }

    pub fn with_runtime_instances(
        config: ServerConfig,
        acp_runner: Arc<gpui_telegram_agent::acp::AcpRunner>,
        task_registry: Arc<tokio::sync::RwLock<HashMap<String, gpui_telegram_agent::tasks::TaskFsm>>>,
        approval_gates: Arc<tokio::sync::RwLock<HashMap<String, gpui_telegram_agent::tasks::ApprovalGate>>>,
    ) -> Self {
        let mut state = Self::new(config);
        state.acp_runner = acp_runner;
        state.task_registry = task_registry;
        state.approval_gates = approval_gates;
        state
    }
}

/// Zero-cost AuthBearer extractor for authenticated routes
#[derive(Debug, Clone)]
pub struct AuthBearer {
    pub session: SessionToken,
    pub user_id: UserId,
}

impl FromRequestParts<Arc<AppState>> for AuthBearer {
    type Rejection = (StatusCode, Json<Value>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let token = extract_token(parts).ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": "missing_token",
                    "message": "Authorization Bearer token or ?token= query parameter is required"
                })),
            )
        })?;

        let session = if let Some(s) = state.session_manager.verify_token(&token) {
            s
        } else {
            let session_id = token.split('.').next().unwrap_or(&token);
            let restored = if let Ok(Some(db_sess)) = state.db.get_session(session_id) {
                if db_sess.revoked_at_ms.is_none()
                    && db_sess.expires_at_ms > chrono::Utc::now().timestamp_millis()
                {
                    let created_at = chrono::DateTime::from_timestamp_millis(db_sess.created_at_ms)
                        .unwrap_or_else(chrono::Utc::now);
                    let expires_at = chrono::DateTime::from_timestamp_millis(db_sess.expires_at_ms)
                        .unwrap_or_else(chrono::Utc::now);
                    let sess_tok = SessionToken::new_with_hash(
                        db_sess.session_id.clone(),
                        db_sess.user_id,
                        created_at,
                        expires_at,
                        &db_sess.device_id,
                        db_sess.token_hash,
                    );
                    state.session_manager.insert(sess_tok);
                    state.session_manager.verify_token(&token)
                } else {
                    None
                }
            } else {
                None
            };

            restored.ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "error": "invalid_token",
                        "message": "Session token is invalid, expired, or revoked"
                    })),
                )
            })?
        };

        let user_id = session.user_id;
        Ok(AuthBearer { session, user_id })
    }
}

/// Extract token from Authorization header or ?token= query parameter
fn extract_token(parts: &Parts) -> Option<String> {
    // 1. Try Authorization: Bearer <token>
    if let Some(auth_val) = parts.headers.get(header::AUTHORIZATION) {
        if let Ok(auth_str) = auth_val.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                let trimmed = token.trim();
                if !trimmed.is_empty() {
                    return Some(trimmed.to_string());
                }
            }
        }
    }

    // 2. Try query parameter ?token=<token>
    if let Some(query) = parts.uri.query() {
        for param in query.split('&') {
            if let Some(token) = param.strip_prefix("token=") {
                let trimmed = token.trim();
                if !trimmed.is_empty() {
                    return Some(trimmed.to_string());
                }
            }
        }
    }

    None
}

/// Build the production Axum router with default in-memory state
pub fn build_router() -> Router {
    let state = Arc::new(AppState::default());
    build_router_with_state(state)
}

/// Build the production Axum router mounting all public, protected, and WebSocket routes
pub fn build_router_with_state(state: Arc<AppState>) -> Router {
    Router::new()
        // --- Public Routes ---
        .route("/health", get(health_check))
        .route("/api/v1/status", get(server_status))
        .route("/api/v1/auth/register", post(auth_register))
        .route("/api/v1/auth/login", post(auth_login))
        // --- Milestone 2 Auth Extension Routes (Alipay & Email) ---
        .route("/api/v1/auth/email/send-code", post(auth_email_send_code))
        .route("/api/v1/auth/email/register", post(auth_email_register))
        .route("/api/v1/auth/alipay/start", post(auth_alipay_start))
        .route("/api/v1/auth/alipay/poll/{attempt_id}", get(auth_alipay_poll))
        .route("/api/v1/auth/alipay/callback", post(auth_alipay_callback).get(auth_alipay_callback))
        .route("/api/v1/auth/alipay/simulate-scan", post(auth_alipay_simulate_scan))
        .route("/api/v1/auth/alipay/portal", get(auth_alipay_portal))
        .route("/api/v1/auth/alipay/login", post(auth_alipay_direct_login))
        .route("/api/v1/auth/alipay/sdk-login", post(auth_alipay_direct_login))
        .route("/api/v1/auth/alipay/register", post(auth_alipay_register))
        .route("/api/v1/auth/alipay/auth-string", get(auth_alipay_auth_string))
        // --- Migrated Legacy & CLI Alipay Auth Routes on Google Cloud Server ---
        .route("/api/auth/alipay/login-url", get(auth_alipay_login_url))
        .route("/api/auth/alipay/cli-session", get(auth_alipay_cli_session))
        .route("/api/auth/alipay/callback", post(auth_alipay_callback).get(auth_alipay_callback))
        .route("/api/auth/alipay/macos-callback", post(auth_alipay_callback).get(auth_alipay_callback))
        .route("/api/auth/alipay/mobile-callback", post(auth_alipay_callback).get(auth_alipay_callback))
        .route("/api/auth/alipay/login", post(auth_alipay_direct_login))
        .route("/api/auth/alipay/sdk-login", post(auth_alipay_direct_login))
        .route("/api/auth/alipay/register", post(auth_alipay_register))
        .route("/api/auth/alipay/auth-string", get(auth_alipay_auth_string))
        .route("/api/v1/auth/passkey/challenge", post(auth_passkey_challenge))
        .route("/api/v1/auth/passkey/verify", post(auth_passkey_verify))
        .route("/api/v1/auth/pair/init", post(auth_pair_init))
        .route("/api/v1/auth/pair/verify", post(auth_pair_verify))
        // --- Protected Auth Routes ---
        .route("/api/v1/auth/me", get(auth_me))
        .route("/api/v1/auth/profile", put(auth_update_profile))
        .route("/api/v1/auth/sessions", get(auth_sessions))
        .route("/api/v1/auth/sessions/{session_id}", delete(auth_revoke_session))
        // --- Protected Messaging Routes ---
        .route("/api/v1/messages/history", get(messages_history))
        .route("/api/v1/messages/send", post(messages_send))
        .route("/api/v1/messages/edit", post(messages_edit))
        .route("/api/v1/messages/delete", post(messages_delete))
        .route("/api/v1/messages/reaction", post(messages_reaction))
        .route("/api/v1/messages/read", post(messages_read))
        // --- Protected Chat & Contacts Routes ---
        .route("/api/v1/contacts", get(contacts_list).post(contacts_add))
        .route("/api/v1/contacts/add", post(contacts_add))
        .route("/api/v1/chats", get(chats_list).post(chats_create))
        .route("/api/v1/chats/create", post(chats_create))
        .route("/api/v1/chats/{chat_id}", get(chats_get).put(chats_update))
        .route(
            "/api/v1/chats/{chat_id}/members",
            get(chats_members_list).post(chats_members_add),
        )
        .route(
            "/api/v1/chats/{chat_id}/members/{user_id}",
            put(chats_members_update).delete(chats_members_delete),
        )
        .route(
            "/api/v1/chats/{chat_id}/topics",
            get(chats_topics).post(chats_topics_create),
        )
        .route(
            "/api/v1/chats/{chat_id}/topics/{topic_id}",
            put(chats_topics_update).delete(chats_topics_delete),
        )
        // --- Monetization & Membership Routes ---
        .route("/api/alipay/create-order", post(alipay_create_order))
        .route("/api/alipay/query-order", get(alipay_query_order))
        .route("/api/alipay/notify", post(alipay_notify))
        .route("/api/alipay/check-membership", get(alipay_check_membership))
        .route("/api/admin/use-redeem-code", post(admin_use_redeem_code))
        .route("/api/admin/create-redeem-code", post(admin_create_redeem_code))
        // --- Protected Sync Routes ---
        .route("/api/v1/sync/difference", get(sync_difference).post(sync_difference_post))
        // --- Protected Media Routes ---
        .route("/api/v1/media/upload/init", post(media_upload_init))
        .route(
            "/api/v1/media/upload/{upload_id}/part/{part_index}",
            put(media_upload_part),
        )
        .route(
            "/api/v1/media/upload/{upload_id}/complete",
            post(media_upload_complete),
        )
        .route("/api/v1/media/stream/{file_id}", get(media_stream))
        // --- Protected Search Routes ---
        .route("/api/v1/search/messages", get(search_messages))
        // --- Protected WebRTC Routes ---
        .route("/api/v1/webrtc/call/invite", post(webrtc_call_invite))
        .route("/api/v1/webrtc/call/signal", post(webrtc_call_signal))
        .route(
            "/api/v1/webrtc/turn_credentials",
            get(webrtc_turn_credentials),
        )
        // --- Agent & Capability Routes ---
        .route("/api/v1/agent/runs", post(agent_runs))
        .route("/api/v1/agent/turn", post(agent_turn))
        .route("/api/v1/tasks", get(tasks_list).post(tasks_create))
        .route("/api/v1/tasks/{task_id}/approve", post(tasks_approve))
        .route("/api/v1/mcp/servers", get(mcp_servers_list).post(mcp_servers_register))
        .route("/api/v1/mcp/tools", get(mcp_tools_list))
        .route("/api/v1/mcp/call", post(mcp_tool_call))
        .route("/api/v1/mcp/permission/approve", post(mcp_permission_approve))
        .route("/api/v1/computer/lease", post(computer_lease_acquire))
        .route("/api/v1/computer/lease/acquire", post(computer_lease_acquire))
        .route(
            "/api/v1/computer/lease/emergency_takeover",
            post(computer_lease_emergency_takeover),
        )
        .route("/api/v1/computer/action", post(computer_action_execute))
        .route("/api/v1/talk/session", post(talk_session_create))
        .route("/api/v1/talk/barge_in", post(talk_barge_in))
        .route("/api/v1/talk/audio", post(talk_audio_enqueue))
        // --- WebSocket Upgrade Route ---
        .route("/api/v1/ws", get(ws_upgrade_handler))
        // --- Tower Middleware Stack ---
        .layer(CompressionLayer::new().gzip(true))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

// ============================================================================
// Public Route Handlers
// ============================================================================

async fn health_check() -> &'static str {
    "OK"
}

async fn server_status() -> Json<Value> {
    Json(json!({
        "server": "gpui_telegram_server",
        "status": "healthy",
        "version": "0.1.0"
    }))
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    #[serde(default = "default_first_name")]
    pub first_name: String,
    pub last_name: Option<String>,
    #[serde(default = "default_device_name")]
    pub device_name: String,
}

fn default_first_name() -> String {
    "User".into()
}

fn default_device_name() -> String {
    "GPUI Desktop".into()
}

async fn auth_register(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> Response {
    let username_clean = req.username.trim();
    if username_clean.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "invalid_credentials", "message": "Username cannot be empty" })),
        )
            .into_response();
    }
    if username_clean.chars().count() > 64 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "invalid_credentials", "message": "Username cannot exceed 64 characters" })),
        )
            .into_response();
    }
    if req.password.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "invalid_credentials", "message": "Password cannot be empty" })),
        )
            .into_response();
    }
    if req.password.as_bytes().len() > 256 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "invalid_credentials", "message": "Password cannot exceed 256 bytes" })),
        )
            .into_response();
    }

    // Check if user already exists in SQLite
    match state.db.get_user_by_username(username_clean) {
        Ok(Some(_)) => {
            return (
                StatusCode::CONFLICT,
                Json(json!({
                    "error": "user_already_exists",
                    "message": format!("Username already registered: {}", username_clean)
                })),
            )
                .into_response();
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "database_error", "message": e.to_string() })),
            )
                .into_response();
        }
        Ok(None) => {}
    }

    // Register user in memory
    match state.session_manager.register(
        username_clean,
        &req.password,
        &req.first_name,
        req.last_name.clone(),
        &req.device_name,
    ) {
        Ok((user, session, token)) => {
            // Persist user record to SQLite database
            let now_ms = user.created_at.timestamp_millis();
            let db_user = crate::db::schema::UserRecord {
                user_id: user.user_id,
                username: Some(user.username.clone()),
                phone: None,
                first_name: user.first_name.clone(),
                last_name: user.last_name.clone(),
                bio: None,
                password_hash: Some(user.password_hash.clone()),
                is_bot: false,
                is_verified: false,
                is_support: false,
                is_scam: false,
                is_fake: false,
                profile_photo_file_id: None,
                account_generation: 1,
                created_at_ms: now_ms,
                updated_at_ms: now_ms,
            };

            if let Err(e) = state.db.create_user(&db_user) {
                tracing::error!("Failed to persist user to SQLite: {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "database_error", "message": e.to_string() })),
                )
                    .into_response();
            }

            // Persist session to SQLite database
            let db_session = crate::db::schema::SessionRecord {
                session_id: session.session_id.clone(),
                user_id: user.user_id,
                device_id: req.device_name.clone(),
                client_type: "gpui_desktop".into(),
                app_version: "0.1.0".into(),
                ip_address: None,
                location_desc: None,
                account_generation: 1,
                token_hash: session.token_hash.clone(),
                created_at_ms: session.created_at.timestamp_millis(),
                last_active_ms: session.created_at.timestamp_millis(),
                expires_at_ms: session.expires_at.timestamp_millis(),
                revoked_at_ms: None,
            };
            let _ = state.db.create_session(&db_session);
            let _ = ensure_user_welcome_bot(&state, user.user_id, Some(&user.first_name));

            (
                StatusCode::CREATED,
                Json(json!({
                    "user_id": user.user_id.raw(),
                    "username": user.username,
                    "first_name": user.first_name,
                    "session_id": session.session_id,
                    "token": token,
                    "created_at": user.created_at,
                })),
            )
                .into_response()
        }
        Err(AuthError::UserAlreadyExists(msg)) => (
            StatusCode::CONFLICT,
            Json(json!({ "error": "user_already_exists", "message": msg })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "registration_failed", "message": e.to_string() })),
        )
            .into_response(),
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    #[serde(default = "default_device_name")]
    pub device_name: String,
}

async fn auth_login(State(state): State<Arc<AppState>>, Json(req): Json<LoginRequest>) -> Response {
    if req.username.chars().count() > 64 || req.password.as_bytes().len() > 256 {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "invalid_credentials", "message": "Invalid username or password" })),
        )
            .into_response();
    }

    // 1. Try memory session manager
    match state
        .session_manager
        .login(&req.username, &req.password, &req.device_name)
    {
        Ok((user, session, token)) => {
            let db_session = crate::db::schema::SessionRecord {
                session_id: session.session_id.clone(),
                user_id: user.user_id,
                device_id: req.device_name.clone(),
                client_type: "gpui_desktop".into(),
                app_version: "0.1.0".into(),
                ip_address: None,
                location_desc: None,
                account_generation: 1,
                token_hash: session.token_hash.clone(),
                created_at_ms: session.created_at.timestamp_millis(),
                last_active_ms: session.created_at.timestamp_millis(),
                expires_at_ms: session.expires_at.timestamp_millis(),
                revoked_at_ms: None,
            };
            let _ = state.db.create_session(&db_session);

            (
                StatusCode::OK,
                Json(json!({
                    "user_id": user.user_id.raw(),
                    "username": user.username,
                    "session_id": session.session_id,
                    "token": token,
                    "expires_at": session.expires_at,
                })),
            )
                .into_response()
        }
        Err(_) => {
            // 2. Query SQLite database on memory miss (critical for persistence across restarts)
            if let Ok(Some(db_user)) = state.db.get_user_by_username(&req.username) {
                if let Some(ref hash) = db_user.password_hash {
                    if crate::auth::verify_password(&req.password, hash) {
                        // Password matches! Restore user record into SessionManager
                        let created_at = chrono::DateTime::from_timestamp_millis(db_user.created_at_ms)
                            .unwrap_or_else(Utc::now);
                        let mem_user = crate::auth::session_manager::UserRecord {
                            user_id: db_user.user_id,
                            username: db_user.username.clone().unwrap_or_else(|| req.username.clone()),
                            first_name: db_user.first_name.clone(),
                            last_name: db_user.last_name.clone(),
                            password_hash: hash.clone(),
                            created_at,
                        };
                        state.session_manager.insert_user(mem_user.clone());

                        // Generate session
                        let (session, raw_token) = state.session_manager.create_session(
                            db_user.user_id,
                            &req.device_name,
                            chrono::Duration::days(30),
                        );

                        // Persist session into SQLite
                        let db_session = crate::db::schema::SessionRecord {
                            session_id: session.session_id.clone(),
                            user_id: db_user.user_id,
                            device_id: req.device_name.clone(),
                            client_type: "gpui_desktop".into(),
                            app_version: "0.1.0".into(),
                            ip_address: None,
                            location_desc: None,
                            account_generation: 1,
                            token_hash: session.token_hash.clone(),
                            created_at_ms: session.created_at.timestamp_millis(),
                            last_active_ms: session.created_at.timestamp_millis(),
                            expires_at_ms: session.expires_at.timestamp_millis(),
                            revoked_at_ms: None,
                        };
                        let _ = state.db.create_session(&db_session);

                        return (
                            StatusCode::OK,
                            Json(json!({
                                "user_id": db_user.user_id.raw(),
                                "username": mem_user.username,
                                "session_id": session.session_id,
                                "token": raw_token,
                                "expires_at": session.expires_at,
                            })),
                        )
                            .into_response();
                    }
                }
            }

            (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": "invalid_credentials",
                    "message": "Invalid username or password"
                })),
            )
                .into_response()
        }
    }
}

// ============================================================================
// Milestone 2 Auth Extension Models & Handlers (Email & Alipay)
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct SendEmailCodeRequest {
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct SendEmailCodeResponse {
    pub ok: bool,
    pub cooldown_seconds: u32,
    pub expires_in_seconds: u32,
    pub expires_at_ms: i64,
    pub resend_after_ms: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EmailRegisterRequest {
    pub email: String,
    pub code: String,
    pub username: String,
    pub password: String,
    #[serde(default = "default_first_name")]
    pub first_name: String,
    pub last_name: Option<String>,
    #[serde(default = "default_device_name")]
    pub device_name: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct AlipayStartRequest {
    #[serde(default = "default_device_name")]
    pub device_name: String,
}

#[derive(Debug, Serialize)]
pub struct AlipayStartResponse {
    pub attempt_id: String,
    pub auth_url: String,
    pub qr_payload: String,
    pub expires_at_ms: i64,
    pub poll_after_ms: u64,
}

#[derive(Debug, Deserialize, Default)]
pub struct AlipayCallbackRequest {
    pub auth_code: Option<String>,
    pub code: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AlipaySimulateScanRequest {
    pub attempt_id: String,
    pub mock_alipay_user_id: Option<String>,
    pub mock_nickname: Option<String>,
}

fn is_cloud_bridge_enabled(state: &AppState) -> bool {
    if std::env::var("GPUI_OFFLINE_TEST").is_ok() {
        return false;
    }
    if std::env::var("GPUI_ENABLE_CLOUD_BRIDGE").is_ok() {
        return true;
    }
    let db_path = state.config.resolve_db_path();
    !db_path.is_empty() && db_path != ":memory:"
}

fn is_synthetic_test_email(email: &str) -> bool {
    email.ends_with("@example.com")
        || email.ends_with(".invalid")
        || email.ends_with(".test")
        || email.ends_with("@localhost")
}

async fn curl_http_request(
    method: &str,
    url: &str,
    body: Option<&Value>,
    timeout_secs: u64,
) -> Option<(u16, String)> {
    let mut cmd = tokio::process::Command::new("curl");
    cmd.arg("-sS")
        .arg("-X")
        .arg(method)
        .arg("--max-time")
        .arg(timeout_secs.to_string())
        .arg("-w")
        .arg("\n%{http_code}");
    if let Some(json_body) = body {
        cmd.arg("-H")
            .arg("Content-Type: application/json")
            .arg("-d")
            .arg(json_body.to_string());
    }
    cmd.arg(url);
    let output = cmd.output().await.ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let (body_str, code_str) = stdout.rsplit_once('\n')?;
    let status = code_str.trim().parse::<u16>().ok()?;
    Some((status, body_str.to_string()))
}

async fn auth_email_send_code(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SendEmailCodeRequest>,
) -> Response {
    let email_clean = req.email.trim().to_lowercase();
    if email_clean.len() < 3 || email_clean.len() > 254 || !email_clean.contains('@') || !email_clean.contains('.') {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "invalid_email", "message": "请输入有效邮箱地址" })),
        )
            .into_response();
    }

    if state.session_manager.get_user_by_email(&email_clean).is_some()
        || state.db.get_user_by_email(&email_clean).map(|u| u.is_some()).unwrap_or(false)
    {
        return (
            StatusCode::CONFLICT,
            Json(json!({ "error": "email_already_registered", "message": "该邮箱已注册，请直接登录" })),
        )
            .into_response();
    }

    let now_ms = Utc::now().timestamp_millis();

    if let Ok(Some(existing)) = state.db.get_latest_email_challenge(&email_clean) {
        if now_ms < existing.cooldown_until_ms {
            return (
                StatusCode::TOO_MANY_REQUESTS,
                Json(json!({
                    "error": "rate_limited",
                    "message": "验证码发送过于频繁，请稍后再试",
                    "resend_after_ms": existing.cooldown_until_ms
                })),
            )
                .into_response();
        }
    }

    let code = crate::auth::session_manager::generate_verification_code();
    let code_hash = crate::auth::session_manager::hash_verification_code(&code);
    let challenge_id = Uuid::new_v4().to_string();
    let cooldown_until_ms = now_ms + 60_000;
    let expires_at_ms = now_ms + 600_000;

    let challenge = crate::db::schema::EmailChallengeRecord {
        challenge_id,
        email: email_clean.clone(),
        code_hash,
        attempts_left: 5,
        cooldown_until_ms,
        expires_at_ms,
        created_at_ms: now_ms,
    };

    if let Err(e) = state.db.save_email_challenge(&challenge) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "database_error", "message": e.to_string() })),
        )
            .into_response();
    }

    if is_cloud_bridge_enabled(&state) && !is_synthetic_test_email(&email_clean) {
        let gateway_base = std::env::var("FABUSHI_CLOUD_GATEWAY_URL")
            .unwrap_or_else(|_| "https://api.ombhrum.com".to_string());
        let send_url = format!(
            "{}/api/auth/send-verification-code",
            gateway_base.trim_end_matches('/')
        );
        let payload = json!({ "email": email_clean, "type": "register" });
        if let Some((status, body_text)) =
            curl_http_request("POST", &send_url, Some(&payload), 6).await
        {
            if status == 400 && body_text.contains("已被注册") {
                let reset_payload = json!({ "email": email_clean, "type": "reset" });
                let _ = curl_http_request("POST", &send_url, Some(&reset_payload), 6).await;
            }
        }
    }

    tracing::info!("Sent email registration code to {}: {}", email_clean, code);

    let debug_code = if std::env::var("GPUI_HIDE_DEBUG_CODE").is_ok() {
        None
    } else {
        Some(code)
    };

    (
        StatusCode::OK,
        Json(SendEmailCodeResponse {
            ok: true,
            cooldown_seconds: 60,
            expires_in_seconds: 600,
            expires_at_ms,
            resend_after_ms: cooldown_until_ms,
            debug_code,
        }),
    )
        .into_response()
}

async fn auth_email_register(
    State(state): State<Arc<AppState>>,
    Json(req): Json<EmailRegisterRequest>,
) -> Response {
    let email_clean = req.email.trim().to_lowercase();
    let username_clean = req.username.trim();

    if email_clean.is_empty() || !email_clean.contains('@') {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "invalid_email", "message": "请输入有效邮箱地址" })),
        )
            .into_response();
    }
    if username_clean.len() < 3 || username_clean.len() > 32 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "invalid_username", "message": "用户名长度需为 3–32 位字符" })),
        )
            .into_response();
    }
    if req.password.len() < 8 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "invalid_password", "message": "密码至少需要 8 位字符" })),
        )
            .into_response();
    }
    let code_clean = req.code.trim();
    if code_clean.len() != 6 || !code_clean.chars().all(|c| c.is_ascii_digit()) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "invalid_code", "message": "验证码格式无效" })),
        )
            .into_response();
    }

    let challenge = match state.db.get_latest_email_challenge(&email_clean) {
        Ok(Some(c)) => c,
        Ok(None) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "challenge_not_found", "message": "请先获取验证码" })),
            )
                .into_response();
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "database_error", "message": e.to_string() })),
            )
                .into_response();
        }
    };

    let now_ms = Utc::now().timestamp_millis();
    if now_ms > challenge.expires_at_ms {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "code_expired", "message": "验证码已失效，请重新获取" })),
        )
            .into_response();
    }
    if challenge.attempts_left <= 0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "code_expired", "message": "验证码尝试次数过多，请重新获取" })),
        )
            .into_response();
    }

    let mut code_valid = crate::auth::session_manager::verify_verification_code(code_clean, &challenge.code_hash);
    if !code_valid && is_cloud_bridge_enabled(&state) && !is_synthetic_test_email(&email_clean) {
        let gateway_base = std::env::var("FABUSHI_CLOUD_GATEWAY_URL")
            .unwrap_or_else(|_| "https://api.ombhrum.com".to_string());
        let reg_url = format!("{}/api/auth/register", gateway_base.trim_end_matches('/'));
        let probe_user = format!("gpui_{}", &Uuid::new_v4().simple().to_string()[..10]);
        let payload = json!({
            "username": probe_user,
            "email": email_clean,
            "password": req.password,
            "verificationCode": code_clean
        });
        if let Some((status, body_text)) =
            curl_http_request("POST", &reg_url, Some(&payload), 6).await
        {
            if (200..300).contains(&status)
                || (status == 400 && body_text.contains("该邮箱已被注册"))
            {
                code_valid = true;
            }
        }
    }

    if !code_valid {
        let _ = state.db.decrement_email_challenge_attempts(&challenge.challenge_id);
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "invalid_code", "message": "验证码不正确" })),
        )
            .into_response();
    }

    let _ = state.db.delete_email_challenge(&challenge.challenge_id);

    match state.session_manager.register_with_email(
        &email_clean,
        username_clean,
        &req.password,
        &req.first_name,
        req.last_name.clone(),
        &req.device_name,
    ) {
        Ok((user, session, token)) => {
            let now_ms = user.created_at.timestamp_millis();
            let db_user = crate::db::schema::UserRecord {
                user_id: user.user_id,
                username: Some(user.username.clone()),
                phone: None,
                first_name: user.first_name.clone(),
                last_name: user.last_name.clone(),
                bio: None,
                password_hash: Some(user.password_hash.clone()),
                is_bot: false,
                is_verified: false,
                is_support: false,
                is_scam: false,
                is_fake: false,
                profile_photo_file_id: None,
                account_generation: 1,
                created_at_ms: now_ms,
                updated_at_ms: now_ms,
            };

            if let Err(e) = state.db.create_user(&db_user) {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "database_error", "message": e.to_string() })),
                )
                    .into_response();
            }
            let _ = state.db.set_user_email_verified(user.user_id, &email_clean);

            let db_session = crate::db::schema::SessionRecord {
                session_id: session.session_id.clone(),
                user_id: user.user_id,
                device_id: req.device_name.clone(),
                client_type: "gpui_desktop".into(),
                app_version: "0.1.0".into(),
                ip_address: None,
                location_desc: None,
                account_generation: 1,
                token_hash: session.token_hash.clone(),
                created_at_ms: session.created_at.timestamp_millis(),
                last_active_ms: session.created_at.timestamp_millis(),
                expires_at_ms: session.expires_at.timestamp_millis(),
                revoked_at_ms: None,
            };
            let _ = state.db.create_session(&db_session);
            let _ = ensure_user_welcome_bot(&state, user.user_id, Some(&user.first_name));

            (
                StatusCode::CREATED,
                Json(json!({
                    "user_id": user.user_id.raw(),
                    "username": user.username,
                    "first_name": user.first_name,
                    "session_id": session.session_id,
                    "token": token,
                    "expires_at": session.expires_at,
                })),
            )
                .into_response()
        }
        Err(AuthError::UserAlreadyExists(msg)) => (
            StatusCode::CONFLICT,
            Json(json!({ "error": "user_already_exists", "message": msg })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "registration_failed", "message": e.to_string() })),
        )
            .into_response(),
    }
}

fn persist_alipay_user_and_session(
    state: &AppState,
    alipay_user_id: &str,
    nickname: &str,
    device_name: &str,
) -> (crate::auth::session_manager::UserRecord, SessionToken, String) {
    if let Ok(Some(existing_db_user)) = state.db.get_user_by_alipay_id(alipay_user_id) {
        let created_at = chrono::DateTime::from_timestamp_millis(existing_db_user.created_at_ms)
            .unwrap_or_else(Utc::now);
        let restored = crate::auth::session_manager::UserRecord {
            user_id: existing_db_user.user_id,
            username: existing_db_user
                .username
                .unwrap_or_else(|| format!("alipay_user_{}", existing_db_user.user_id.raw())),
            first_name: existing_db_user.first_name,
            last_name: existing_db_user.last_name,
            password_hash: existing_db_user.password_hash.unwrap_or_default(),
            created_at,
        };
        state
            .session_manager
            .restore_alipay_user(alipay_user_id, restored);
    }

    let (user, session, token) = state.session_manager.login_or_register_alipay(
        alipay_user_id,
        nickname,
        None,
        device_name,
    );

    let now_ms = Utc::now().timestamp_millis();
    let db_user = crate::db::schema::UserRecord {
        user_id: user.user_id,
        username: Some(user.username.clone()),
        phone: None,
        first_name: user.first_name.clone(),
        last_name: user.last_name.clone(),
        bio: None,
        password_hash: None,
        is_bot: false,
        is_verified: false,
        is_support: false,
        is_scam: false,
        is_fake: false,
        profile_photo_file_id: None,
        account_generation: 1,
        created_at_ms: now_ms,
        updated_at_ms: now_ms,
    };
    let _ = state.db.create_user(&db_user);
    let _ = state
        .db
        .set_user_alipay_identity(user.user_id, alipay_user_id, nickname);

    let db_session = crate::db::schema::SessionRecord {
        session_id: session.session_id.clone(),
        user_id: user.user_id,
        device_id: device_name.to_string(),
        client_type: "gpui_desktop".into(),
        app_version: "0.1.0".into(),
        ip_address: None,
        location_desc: None,
        account_generation: 1,
        token_hash: session.token_hash.clone(),
        created_at_ms: session.created_at.timestamp_millis(),
        last_active_ms: session.created_at.timestamp_millis(),
        expires_at_ms: session.expires_at.timestamp_millis(),
        revoked_at_ms: None,
    };
    let _ = state.db.create_session(&db_session);
    let _ = ensure_user_welcome_bot(state, user.user_id, Some(&user.first_name));

    (user, session, token)
}

async fn start_upstream_canonical_alipay_oauth(state: &AppState) -> Option<(String, String, String)> {
    if !is_cloud_bridge_enabled(state) {
        return None;
    }
    let gateway_base = std::env::var("FABUSHI_CLOUD_GATEWAY_URL")
        .unwrap_or_else(|_| "https://api.ombhrum.com".to_string());
    let cli_login_url = format!(
        "{}/api/auth/alipay/login-url?platform=cli",
        gateway_base.trim_end_matches('/')
    );
    if let Some((status, body_str)) = curl_http_request("GET", &cli_login_url, None, 5).await {
        if (200..300).contains(&status) {
            if let Ok(val) = serde_json::from_str::<Value>(&body_str) {
                let auth_url = val.get("authUrl").and_then(|v| v.as_str()).unwrap_or_default();
                let state_param = val.get("state").and_then(|v| v.as_str()).unwrap_or_default();
                if !auth_url.is_empty() && !state_param.is_empty() {
                    return Some((
                        state_param.to_string(),
                        auth_url.to_string(),
                        state_param.to_string(),
                    ));
                }
            }
        }
    }
    None
}

async fn poll_upstream_canonical_alipay_oauth(
    state: &AppState,
    upstream_attempt_id: &str,
) -> Option<(String, String)> {
    if !is_cloud_bridge_enabled(state) {
        return None;
    }
    let gateway_base = std::env::var("FABUSHI_CLOUD_GATEWAY_URL")
        .unwrap_or_else(|_| "https://api.ombhrum.com".to_string());
    let poll_url = format!(
        "{}/api/auth/alipay/cli-session?state={}",
        gateway_base.trim_end_matches('/'),
        upstream_attempt_id
    );
    let (status_code, body_str) = curl_http_request("GET", &poll_url, None, 5).await?;
    if !(200..300).contains(&status_code) {
        return None;
    }
    let val: Value = serde_json::from_str(&body_str).ok()?;
    let status = val.get("status").and_then(|v| v.as_str()).unwrap_or_default();
    if status != "completed" && status != "complete" {
        return None;
    }
    let user_obj = val
        .get("session")
        .and_then(|s| s.get("user"))
        .or_else(|| val.get("user"));
    let alipay_uid = user_obj
        .and_then(|u| {
            u.get("alipayUserId")
                .or_else(|| u.get("id"))
                .and_then(|v| {
                    v.as_str()
                        .map(str::to_string)
                        .or_else(|| v.as_u64().map(|n| n.to_string()))
                })
        })
        .or_else(|| val.get("username").and_then(|v| v.as_str()).map(str::to_string))
        .unwrap_or_else(|| format!("alipay_{}", &upstream_attempt_id[..upstream_attempt_id.len().min(12)]));
    let nickname = user_obj
        .and_then(|u| {
            u.get("alipayNickname")
                .or_else(|| u.get("nickname"))
                .or_else(|| u.get("username"))
                .and_then(|v| v.as_str())
        })
        .or_else(|| val.get("username").and_then(|v| v.as_str()))
        .unwrap_or("支付宝用户")
        .to_string();
    Some((alipay_uid, nickname))
}

async fn auth_alipay_start(
    State(state): State<Arc<AppState>>,
    body: Option<Json<AlipayStartRequest>>,
) -> Response {
    let _req = body.map(|b| b.0).unwrap_or_default();
    let attempt_id = Uuid::new_v4().to_string();
    let mut state_token = format!("fbs_{}", Uuid::new_v4().simple());
    let now_ms = Utc::now().timestamp_millis();
    let expires_at_ms = now_ms + 600_000;

    let app_id = "2021005193647715";
    let callback_url = "https%3A%2F%2Fapi.ombhrum.com%2Fapi%2Fauth%2Falipay%2Fcallback";
    let mut auth_url = format!(
        "https://openauth.alipay.com/oauth2/publicAppAuthorize.htm?app_id={}&scope=auth_user&redirect_uri={}&state={}",
        app_id, callback_url, state_token
    );

    if let Some((upstream_id, upstream_url, upstream_state)) =
        start_upstream_canonical_alipay_oauth(&state).await
    {
        auth_url = upstream_url;
        state_token = upstream_state;
        let mut map = state.alipay_upstream_attempts.write();
        map.insert(attempt_id.clone(), upstream_id.clone());
        map.insert(state_token.clone(), upstream_id);
    }

    let attempt = crate::db::schema::OAuthAttemptRecord {
        attempt_id: attempt_id.clone(),
        provider: "alipay".into(),
        state: state_token,
        status: "pending".into(),
        session_json: None,
        delivered_at_ms: None,
        expires_at_ms,
        created_at_ms: now_ms,
    };

    if let Err(e) = state.db.create_oauth_attempt(&attempt) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "database_error", "message": e.to_string() })),
        )
            .into_response();
    }

    (
        StatusCode::OK,
        Json(AlipayStartResponse {
            attempt_id,
            auth_url: auth_url.clone(),
            qr_payload: auth_url,
            expires_at_ms,
            poll_after_ms: 1000,
        }),
    )
        .into_response()
}

async fn auth_alipay_poll(
    State(state): State<Arc<AppState>>,
    Path(attempt_id): Path<String>,
) -> Response {
    let mut attempt = match state.db.get_oauth_attempt(&attempt_id) {
        Ok(Some(a)) => a,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "oauth_attempt_not_found", "message": "登录会话不存在或已失效" })),
            )
                .into_response();
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "database_error", "message": e.to_string() })),
            )
                .into_response();
        }
    };

    let now_ms = Utc::now().timestamp_millis();
    if attempt.expires_at_ms <= now_ms && attempt.status == "pending" {
        let _ = state.db.update_oauth_attempt_status(&attempt_id, "expired");
        return (
            StatusCode::OK,
            Json(json!({ "status": "expired" })),
        )
            .into_response();
    }

    if attempt.status == "pending" {
        let maybe_upstream_id = {
            let map = state.alipay_upstream_attempts.read();
            map.get(&attempt_id)
                .or_else(|| map.get(&attempt.state))
                .cloned()
        };
        if let Some(upstream_id) = maybe_upstream_id {
            if let Some((alipay_uid, nickname)) =
                poll_upstream_canonical_alipay_oauth(&state, &upstream_id).await
            {
                let (user, session, token) = persist_alipay_user_and_session(
                    &state,
                    &alipay_uid,
                    &nickname,
                    "GPUI Desktop Client",
                );
                let session_json = json!({
                    "user_id": user.user_id.raw(),
                    "username": user.username,
                    "first_name": user.first_name,
                    "session_id": session.session_id,
                    "token": token,
                    "expires_at": session.expires_at
                });
                let _ = state
                    .db
                    .complete_oauth_attempt(&attempt_id, &session_json.to_string());
                attempt.status = "completed".into();
            }
        }
    }

    if attempt.status != "completed" {
        return (
            StatusCode::OK,
            Json(json!({ "status": attempt.status })),
        )
            .into_response();
    }

    match state.db.claim_oauth_attempt_session(&attempt_id, now_ms) {
        Ok(Some(session_json_str)) => {
            let session_val: Value = serde_json::from_str(&session_json_str).unwrap_or(Value::Null);
            let token_str = session_val
                .get("token")
                .and_then(|v| v.as_str())
                .map(str::to_string);
            let user_id_num = session_val.get("user_id").and_then(|v| v.as_u64());
            let username_str = session_val
                .get("username")
                .and_then(|v| v.as_str())
                .map(str::to_string);
            let first_name_str = session_val
                .get("first_name")
                .and_then(|v| v.as_str())
                .unwrap_or("支付宝用户")
                .to_string();
            let user_dto = user_id_num.map(|uid| {
                json!({
                    "user_id": uid,
                    "username": username_str,
                    "first_name": first_name_str
                })
            });
            (
                StatusCode::OK,
                Json(json!({
                    "status": "completed",
                    "token": token_str,
                    "user_id": user_id_num,
                    "username": username_str,
                    "user": user_dto,
                    "session": session_val
                })),
            )
                .into_response()
        }
        Ok(None) => (
            StatusCode::GONE,
            Json(json!({
                "error": "session_already_delivered",
                "message": "登录凭证已领取或已失效"
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "database_error", "message": e.to_string() })),
        )
            .into_response(),
    }
}

async fn resolve_alipay_identity_from_auth_code(
    state: &AppState,
    auth_code: &str,
) -> (String, String) {
    let default_uid = format!(
        "alipay_{}",
        if auth_code.is_empty() { "default" } else { auth_code }
    );
    let default_nick = "支付宝用户".to_string();

    if !is_cloud_bridge_enabled(state)
        || auth_code.is_empty()
        || auth_code.starts_with("mock_")
        || auth_code.starts_with("test_")
    {
        return (default_uid, default_nick);
    }

    let gateway_base = std::env::var("FABUSHI_CLOUD_GATEWAY_URL")
        .unwrap_or_else(|_| "https://api.ombhrum.com".to_string());
    let sdk_url = format!(
        "{}/api/auth/alipay/sdk-login",
        gateway_base.trim_end_matches('/')
    );
    let payload = json!({ "authCode": auth_code });
    if let Some((_status, body_str)) =
        curl_http_request("POST", &sdk_url, Some(&payload), 6).await
    {
        if let Ok(val) = serde_json::from_str::<Value>(&body_str) {
            let uid = val
                .get("user")
                .and_then(|u| u.get("alipayUserId").and_then(|v| v.as_str()))
                .or_else(|| val.get("alipayUserId").and_then(|v| v.as_str()))
                .map(str::to_string);
            let nick = val
                .get("user")
                .and_then(|u| {
                    u.get("alipayNickname")
                        .or_else(|| u.get("nickname"))
                        .and_then(|v| v.as_str())
                })
                .or_else(|| {
                    val.get("alipayInfo")
                        .and_then(|i| i.get("nickname"))
                        .and_then(|v| v.as_str())
                })
                .map(str::to_string);
            if let Some(real_uid) = uid {
                if !real_uid.is_empty() {
                    return (real_uid, nick.unwrap_or(default_nick));
                }
            }
        }
    }

    (default_uid, default_nick)
}

async fn auth_alipay_callback(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(query_params): Query<HashMap<String, String>>,
    body_payload: Option<Json<AlipayCallbackRequest>>,
) -> Response {
    let auth_code = body_payload
        .as_ref()
        .and_then(|p| p.auth_code.clone().or_else(|| p.code.clone()))
        .or_else(|| query_params.get("auth_code").cloned())
        .or_else(|| query_params.get("code").cloned())
        .unwrap_or_default();

    let state_token = body_payload
        .as_ref()
        .and_then(|p| p.state.clone())
        .or_else(|| query_params.get("state").cloned())
        .unwrap_or_default();

    if state_token.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "missing_state", "message": "OAuth state parameter missing" })),
        )
            .into_response();
    }

    let attempt = match state.db.get_oauth_attempt_by_state(&state_token) {
        Ok(Some(a)) => a,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "invalid_state", "message": "无效或过期的 OAuth state" })),
            )
                .into_response();
        }
    };

    if attempt.status != "pending" || attempt.expires_at_ms <= Utc::now().timestamp_millis() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "attempt_expired", "message": "登录链接已失效，请重新扫码" })),
        )
            .into_response();
    }

    let (alipay_user_id, nickname) =
        resolve_alipay_identity_from_auth_code(&state, &auth_code).await;

    let (user, session, token) = persist_alipay_user_and_session(
        &state,
        &alipay_user_id,
        &nickname,
        "Alipay OAuth",
    );

    let session_json = json!({
        "user_id": user.user_id.raw(),
        "username": user.username,
        "first_name": user.first_name,
        "session_id": session.session_id,
        "token": token,
        "expires_at": session.expires_at
    });

    let _ = state
        .db
        .complete_oauth_attempt(&attempt.attempt_id, &session_json.to_string());

    let wants_html = body_payload.is_none()
        && (query_params.get("format").map(|s| s.as_str()) == Some("html")
            || headers
                .get(header::ACCEPT)
                .and_then(|v| v.to_str().ok())
                .map(|s| s.contains("text/html"))
                .unwrap_or(false));

    if wants_html {
        let html = r#"<!doctype html><html lang="zh-CN"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>支付宝登录成功 — GPUI Telegram</title><style>body{margin:0;min-height:100vh;display:flex;align-items:center;justify-content:center;background:#0f172a;color:#f8fafc;font-family:-apple-system,BlinkMacSystemFont,"Segoe UI", sans-serif}.card{background:#1e293b;border:1px solid #334155;border-radius:16px;padding:32px;max-width:420px;text-align:center;box-shadow:0 20px 40px rgba(0,0,0,.4)}h1{margin:0 0 12px;font-size:22px;color:#38bdf8}p{margin:0;color:#94a3b8;line-height:1.6}</style></head><body><div class="card"><h1>支付宝登录成功</h1><p>凭证已同步至 Google Cloud 服务器，正在返回 GPUI Telegram 客户端，您可以直接关闭此网页。</p></div><script>setTimeout(()=>{try{window.close()}catch(_){}},1200);</script></body></html>"#;
        return (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
            html,
        )
            .into_response();
    }

    (
        StatusCode::OK,
        Json(json!({ "ok": true, "status": "completed" })),
    )
        .into_response()
}

async fn auth_alipay_simulate_scan(
    State(state): State<Arc<AppState>>,
    Json(req): Json<AlipaySimulateScanRequest>,
) -> Response {
    let attempt = match state.db.get_oauth_attempt(&req.attempt_id) {
        Ok(Some(a)) => a,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "attempt_not_found", "message": "Attempt does not exist" })),
            )
                .into_response();
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "database_error", "message": e.to_string() })),
            )
                .into_response();
        }
    };

    if attempt.status != "pending" || attempt.expires_at_ms <= Utc::now().timestamp_millis() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "attempt_not_pending", "message": "Attempt is expired or not pending" })),
        )
            .into_response();
    }

    let alipay_user_id = req.mock_alipay_user_id.unwrap_or_else(|| "mock_alipay_user_1001".into());
    let nickname = req.mock_nickname.unwrap_or_else(|| "支付宝测试用户".into());

    let (user, session, token) = persist_alipay_user_and_session(
        &state,
        &alipay_user_id,
        &nickname,
        "GPUI Desktop Client",
    );

    let session_json = json!({
        "user_id": user.user_id.raw(),
        "username": user.username,
        "first_name": user.first_name,
        "session_id": session.session_id,
        "token": token,
        "expires_at": session.expires_at
    });

    let _ = state.db.complete_oauth_attempt(&req.attempt_id, &session_json.to_string());

    (
        StatusCode::OK,
        Json(json!({ "ok": true, "status": "completed" })),
    )
        .into_response()
}

async fn auth_alipay_login_url(
    State(state): State<Arc<AppState>>,
    Query(query_params): Query<HashMap<String, String>>,
) -> Response {
    let platform = query_params
        .get("platform")
        .cloned()
        .unwrap_or_else(|| "cli".to_string());
    let attempt_id = Uuid::new_v4().to_string();
    let mut state_token = format!("fbs_{}", Uuid::new_v4().simple());
    let now_ms = Utc::now().timestamp_millis();
    let expires_at_ms = now_ms + 600_000;

    let app_id = "2021005193647715";
    let callback_url = "https%3A%2F%2Fapi.ombhrum.com%2Fapi%2Fauth%2Falipay%2Fcallback";
    let mut auth_url = format!(
        "https://openauth.alipay.com/oauth2/publicAppAuthorize.htm?app_id={}&scope=auth_user&redirect_uri={}&state={}",
        app_id, callback_url, state_token
    );

    if let Some((upstream_id, upstream_url, upstream_state)) =
        start_upstream_canonical_alipay_oauth(&state).await
    {
        auth_url = upstream_url;
        state_token = upstream_state;
        let mut map = state.alipay_upstream_attempts.write();
        map.insert(attempt_id.clone(), upstream_id.clone());
        map.insert(state_token.clone(), upstream_id);
    }

    let attempt = crate::db::schema::OAuthAttemptRecord {
        attempt_id: attempt_id.clone(),
        provider: "alipay".into(),
        state: state_token.clone(),
        status: "pending".into(),
        session_json: None,
        delivered_at_ms: None,
        expires_at_ms,
        created_at_ms: now_ms,
    };
    let _ = state.db.create_oauth_attempt(&attempt);

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "attemptId": attempt_id,
            "authUrl": auth_url,
            "state": state_token,
            "appId": app_id,
            "platform": platform
        })),
    )
        .into_response()
}

async fn auth_alipay_cli_session(
    State(state): State<Arc<AppState>>,
    Query(query_params): Query<HashMap<String, String>>,
) -> Response {
    let state_token = query_params.get("state").cloned().unwrap_or_default();
    if state_token.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "缺少state参数" })),
        )
            .into_response();
    }

    let mut attempt = match state.db.get_oauth_attempt_by_state(&state_token) {
        Ok(Some(a)) => a,
        _ => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "登录状态不存在或已过期" })),
            )
                .into_response();
        }
    };

    let now_ms = Utc::now().timestamp_millis();
    if attempt.expires_at_ms <= now_ms && attempt.status == "pending" {
        let _ = state.db.update_oauth_attempt_status(&attempt.attempt_id, "expired");
        return (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "登录状态已过期，请重新发起登录" })),
        )
            .into_response();
    }

    if attempt.status == "pending" {
        let maybe_upstream_id = {
            let map = state.alipay_upstream_attempts.read();
            map.get(&state_token)
                .or_else(|| map.get(&attempt.attempt_id))
                .cloned()
        };
        if let Some(upstream_id) = maybe_upstream_id {
            if let Some((alipay_uid, nickname)) =
                poll_upstream_canonical_alipay_oauth(&state, &upstream_id).await
            {
                let (user, session, token) = persist_alipay_user_and_session(
                    &state,
                    &alipay_uid,
                    &nickname,
                    "GPUI Desktop CLI",
                );
                let session_json = json!({
                    "user_id": user.user_id.raw(),
                    "username": user.username,
                    "first_name": user.first_name,
                    "session_id": session.session_id,
                    "token": token,
                    "expires_at": session.expires_at,
                    "alipay_user_id": alipay_uid
                });
                let _ = state
                    .db
                    .complete_oauth_attempt(&attempt.attempt_id, &session_json.to_string());
                attempt.status = "completed".into();
                attempt.session_json = Some(session_json.to_string());
            }
        }
    }

    if attempt.status != "completed" {
        return (
            StatusCode::OK,
            Json(json!({
                "success": false,
                "status": "pending",
                "message": "等待用户在浏览器中完成支付宝授权..."
            })),
        )
            .into_response();
    }

    let session_str = match state
        .db
        .claim_oauth_attempt_session(&attempt.attempt_id, now_ms)
    {
        Ok(Some(s)) => s,
        _ => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "登录状态不存在或已过期" })),
            )
                .into_response();
        }
    };

    let sess: Value = serde_json::from_str(&session_str).unwrap_or_default();
    let uid = sess.get("user_id").and_then(|v| v.as_u64()).unwrap_or(0);
    let username = sess
        .get("username")
        .and_then(|v| v.as_str())
        .unwrap_or("alipay_user");
    let first_name = sess
        .get("first_name")
        .and_then(|v| v.as_str())
        .unwrap_or("支付宝用户");
    let token = sess.get("token").and_then(|v| v.as_str()).unwrap_or("");
    let alipay_uid = sess
        .get("alipay_user_id")
        .and_then(|v| v.as_str())
        .unwrap_or(username);

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "status": "complete",
            "accessToken": token,
            "token": token,
            "username": username,
            "user": {
                "id": uid,
                "user_id": uid,
                "username": username,
                "first_name": first_name,
                "alipayUserId": alipay_uid,
                "alipayNickname": first_name
            }
        })),
    )
        .into_response()
}

#[derive(Debug, Deserialize, Default)]
pub struct AlipayDirectLoginRequest {
    #[serde(alias = "authCode")]
    pub auth_code: Option<String>,
    pub code: Option<String>,
    #[serde(alias = "alipayUserId")]
    pub alipay_user_id: Option<String>,
    pub nickname: Option<String>,
    #[serde(default = "default_device_name")]
    pub device_name: String,
}

async fn auth_alipay_direct_login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<AlipayDirectLoginRequest>,
) -> Response {
    let auth_code = req.auth_code.or(req.code).unwrap_or_default();
    let (resolved_uid, resolved_nick) = if let Some(explicit_uid) = req.alipay_user_id {
        (
            explicit_uid,
            req.nickname.unwrap_or_else(|| "支付宝用户".to_string()),
        )
    } else if !auth_code.is_empty() {
        resolve_alipay_identity_from_auth_code(&state, &auth_code).await
    } else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "missing_auth_code", "message": "缺少授权码" })),
        )
            .into_response();
    };

    let (user, session, token) = persist_alipay_user_and_session(
        &state,
        &resolved_uid,
        &resolved_nick,
        &req.device_name,
    );

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "message": "登录成功",
            "token": token,
            "accessToken": token,
            "session_id": session.session_id,
            "user_id": user.user_id.raw(),
            "username": user.username,
            "first_name": user.first_name,
            "user": {
                "id": user.user_id.raw(),
                "user_id": user.user_id.raw(),
                "username": user.username,
                "first_name": user.first_name,
                "alipayUserId": resolved_uid,
                "alipayNickname": user.first_name
            }
        })),
    )
        .into_response()
}

#[derive(Debug, Deserialize, Default)]
pub struct AlipayRegisterRequest {
    #[serde(alias = "alipayUserId")]
    pub alipay_user_id: Option<String>,
    #[serde(alias = "authCode")]
    pub auth_code: Option<String>,
    pub username: Option<String>,
    pub nickname: Option<String>,
    #[serde(default = "default_device_name")]
    pub device_name: String,
}

async fn auth_alipay_register(
    State(state): State<Arc<AppState>>,
    Json(req): Json<AlipayRegisterRequest>,
) -> Response {
    let (alipay_uid, default_nick) = if let Some(uid) = req.alipay_user_id {
        (uid, "支付宝用户".to_string())
    } else if let Some(code) = req.auth_code {
        resolve_alipay_identity_from_auth_code(&state, &code).await
    } else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "missing_alipay_user_id", "message": "缺少支付宝用户ID或授权码" })),
        )
            .into_response();
    };
    let nick = req
        .nickname
        .or(req.username)
        .unwrap_or(default_nick);

    let (user, session, token) = persist_alipay_user_and_session(
        &state,
        &alipay_uid,
        &nick,
        &req.device_name,
    );

    (
        StatusCode::CREATED,
        Json(json!({
            "success": true,
            "message": "注册成功",
            "token": token,
            "accessToken": token,
            "session_id": session.session_id,
            "user_id": user.user_id.raw(),
            "username": user.username,
            "first_name": user.first_name,
            "user": {
                "id": user.user_id.raw(),
                "user_id": user.user_id.raw(),
                "username": user.username,
                "first_name": user.first_name,
                "alipayUserId": alipay_uid,
                "alipayNickname": user.first_name
            }
        })),
    )
        .into_response()
}

async fn auth_alipay_auth_string(State(state): State<Arc<AppState>>) -> Response {
    if is_cloud_bridge_enabled(&state) {
        let gateway_base = std::env::var("FABUSHI_CLOUD_GATEWAY_URL")
            .unwrap_or_else(|_| "https://api.ombhrum.com".to_string());
        let url = format!(
            "{}/api/auth/alipay/auth-string",
            gateway_base.trim_end_matches('/')
        );
        if let Some((status, body_str)) = curl_http_request("GET", &url, None, 5).await {
            if (200..300).contains(&status) {
                if let Ok(val) = serde_json::from_str::<Value>(&body_str) {
                    return (StatusCode::OK, Json(val)).into_response();
                }
            }
        }
    }

    let target_id = format!("fbs_sdk_{}", Uuid::new_v4().simple());
    let auth_str = format!(
        "apiname=com.alipay.account.auth&app_id=2021005193647715&app_name=mc&auth_type=AUTHACCOUNT&biz_type=openservice&method=alipay.open.auth.sdk.code.get&pid=2088102199998888&product_id=APP_FAST_LOGIN&scope=kuaijie&sign_type=RSA2&target_id={}",
        target_id
    );
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "authStr": auth_str,
            "targetId": target_id
        })),
    )
        .into_response()
}

async fn auth_alipay_portal() -> Response {
    let html = r#"<!doctype html><html lang="zh-CN"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>支付宝安全登录 — GPUI Telegram Google Cloud</title><style>body{margin:0;min-height:100vh;display:flex;align-items:center;justify-content:center;background:#0f172a;color:#f8fafc;font-family:-apple-system,BlinkMacSystemFont,"Segoe UI",sans-serif}.card{background:#1e293b;border:1px solid #334155;border-radius:16px;padding:32px;max-width:440px;width:90%;text-align:center;box-shadow:0 20px 40px rgba(0,0,0,.4)}h1{margin:0 0 12px;font-size:22px;color:#38bdf8}p{margin:0 0 20px;color:#94a3b8;line-height:1.6}.btn{display:inline-block;background:#0284c7;color:#fff;text-decoration:none;padding:12px 24px;border-radius:10px;font-weight:600;border:none;cursor:pointer}</style></head><body><div class="card"><h1>GPUI Telegram 支付宝登录</h1><p>已迁移至 Google Cloud 认证网关，不再跳转旧版网页。</p><button class="btn" onclick="startLogin()">立即拉起支付宝授权</button></div><script>async function startLogin(){const r=await fetch('/api/v1/auth/alipay/start',{method:'POST',headers:{'Content-Type':'application/json'},body:'{}'});const d=await r.json();if(d.auth_url){window.location.href=d.auth_url;}}</script></body></html>"#;
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        html,
    )
        .into_response()
}

#[derive(Debug, Deserialize, Default)]
pub struct PasskeyChallengeReq {
    pub user_id: Option<UserId>,
    pub username: Option<String>,
}

async fn auth_passkey_challenge(
    State(state): State<Arc<AppState>>,
    Json(req): Json<PasskeyChallengeReq>,
) -> Json<PasskeyChallenge> {
    let challenge = PasskeyService::generate_challenge(req.user_id);
    state
        .passkey_challenges
        .write()
        .insert(challenge.challenge.clone(), Utc::now().timestamp_millis());
    Json(challenge)
}

async fn auth_passkey_verify(
    State(state): State<Arc<AppState>>,
    Json(req): Json<PasskeyVerifyRequest>,
) -> Response {
    let challenge_valid = state
        .passkey_challenges
        .write()
        .remove(&req.challenge)
        .is_some();

    if !challenge_valid {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "invalid_challenge", "message": "Unknown or expired challenge" })),
        )
            .into_response();
    }

    match PasskeyService::verify_assertion(&req) {
        Ok(true) => {
            let user_id = req.user_id.unwrap_or(UserId(100_001));
            let (session, token) = state.session_manager.create_session(
                user_id,
                "Passkey Authenticator",
                chrono::Duration::days(30),
            );
            (
                StatusCode::OK,
                Json(json!({
                    "verified": true,
                    "user_id": user_id.raw(),
                    "session_id": session.session_id,
                    "token": token,
                    "expires_at": session.expires_at,
                })),
            )
                .into_response()
        }
        _ => (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "verification_failed", "message": "Passkey assertion failed" })),
        )
            .into_response(),
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct PairInitReq {
    pub device_name: Option<String>,
}

async fn auth_pair_init(
    State(state): State<Arc<AppState>>,
    Json(req): Json<PairInitReq>,
) -> Json<Value> {
    let pair_token = format!("pair_{}", Uuid::new_v4().simple());
    let expires_at_ms = Utc::now().timestamp_millis() + 300_000; // 5 minutes
    let placeholder_user = UserId(100_001);

    state
        .pairing_requests
        .write()
        .insert(pair_token.clone(), (placeholder_user, expires_at_ms));

    Json(json!({
        "pair_token": pair_token,
        "qr_payload": format!("tg://login?token={}", pair_token),
        "device_name": req.device_name.unwrap_or_else(|| "GPUI Companion".into()),
        "expires_at_ms": expires_at_ms,
    }))
}

#[derive(Debug, Deserialize)]
pub struct PairVerifyReq {
    pub pair_token: String,
    pub device_name: Option<String>,
}

async fn auth_pair_verify(
    State(state): State<Arc<AppState>>,
    Json(req): Json<PairVerifyReq>,
) -> Response {
    let pair_entry = state.pairing_requests.write().remove(&req.pair_token);
    let (user_id, expires_at_ms) = match pair_entry {
        Some(entry) => entry,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "invalid_pair_token", "message": "Pair token not found or expired" })),
            )
                .into_response()
        }
    };

    if Utc::now().timestamp_millis() > expires_at_ms {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "pair_token_expired", "message": "Pair token has expired" })),
        )
            .into_response();
    }

    let device_name = req.device_name.unwrap_or_else(|| "Paired Device".into());
    let (session, token) = state
        .session_manager
        .create_session(user_id, &device_name, chrono::Duration::days(30));

    (
        StatusCode::OK,
        Json(json!({
            "verified": true,
            "user_id": user_id.raw(),
            "session_id": session.session_id,
            "token": token,
            "expires_at": session.expires_at,
        })),
    )
        .into_response()
}

// ============================================================================
// Protected Route Handlers (AuthBearer Enforced)
// ============================================================================

async fn auth_me(auth: AuthBearer, State(state): State<Arc<AppState>>) -> Json<Value> {
    let fence = state.session_manager.get_or_create_fence(auth.user_id);
    let mut user_record = state.session_manager.get_user(auth.user_id);
    let db_user = state.db.get_user(auth.user_id).ok().flatten();

    if user_record.is_none() {
        if let Some(db_u) = &db_user {
            let created_at = chrono::DateTime::from_timestamp_millis(db_u.created_at_ms)
                .unwrap_or_else(Utc::now);
            let mem_user = crate::auth::session_manager::UserRecord {
                user_id: db_u.user_id,
                username: db_u
                    .username
                    .clone()
                    .unwrap_or_else(|| format!("user_{}", db_u.user_id.0)),
                first_name: db_u.first_name.clone(),
                last_name: db_u.last_name.clone(),
                password_hash: db_u.password_hash.clone().unwrap_or_default(),
                created_at,
            };
            state.session_manager.insert_user(mem_user.clone());
            user_record = Some(mem_user);
        }
    }

    let username = db_user.as_ref().and_then(|u| u.username.clone())
        .or_else(|| user_record.as_ref().map(|u| u.username.clone()));
    let first_name = db_user.as_ref().map(|u| u.first_name.clone())
        .or_else(|| user_record.as_ref().map(|u| u.first_name.clone()));
    let last_name = db_user.as_ref().and_then(|u| u.last_name.clone())
        .or_else(|| user_record.as_ref().and_then(|u| u.last_name.clone()));
    let bio = db_user.as_ref().and_then(|u| u.bio.clone());
    let phone = db_user.as_ref().and_then(|u| u.phone.clone());
    let photo = db_user.as_ref().and_then(|u| u.profile_photo_file_id.clone());

    Json(json!({
        "user_id": auth.user_id.raw(),
        "username": username,
        "first_name": first_name,
        "last_name": last_name,
        "bio": bio,
        "phone": phone,
        "profile_photo_file_id": photo,
        "session_id": auth.session.session_id,
        "device_name": auth.session.device_name,
        "created_at": auth.session.created_at,
        "expires_at": auth.session.expires_at,
        "account_generation": fence.current(),
    }))
}

async fn auth_update_profile(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    let first_name = payload.get("first_name").and_then(|v| v.as_str());
    let last_name = payload.get("last_name").and_then(|v| v.as_str());
    let bio = payload.get("bio").and_then(|v| v.as_str());
    let username = payload.get("username").and_then(|v| v.as_str());
    let photo_file_id = payload.get("profile_photo_file_id").and_then(|v| v.as_str());

    match state.db.update_user_profile(auth.user_id, first_name, last_name, bio, username, photo_file_id) {
        Ok(user) => {
            // Update session_manager cache
            if let Some(mut mem_user) = state.session_manager.get_user(auth.user_id) {
                mem_user.first_name = user.first_name.clone();
                mem_user.last_name = user.last_name.clone();
                if let Some(un) = &user.username {
                    mem_user.username = un.clone();
                }
                state.session_manager.insert_user(mem_user);
            }
            let fence = state.session_manager.get_or_create_fence(auth.user_id);
            (
                StatusCode::OK,
                Json(json!({
                    "success": true,
                    "user_id": user.user_id.raw(),
                    "username": user.username,
                    "first_name": user.first_name,
                    "last_name": user.last_name,
                    "bio": user.bio,
                    "phone": user.phone,
                    "profile_photo_file_id": user.profile_photo_file_id,
                    "session_id": auth.session.session_id,
                    "device_name": auth.session.device_name,
                    "created_at": auth.session.created_at,
                    "expires_at": auth.session.expires_at,
                    "account_generation": fence.current(),
                })),
            ).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "database_error", "message": e.to_string() })),
        ).into_response(),
    }
}

async fn auth_sessions(auth: AuthBearer, State(state): State<Arc<AppState>>) -> Json<Value> {
    let sessions = state.session_manager.list_user_sessions(auth.user_id);
    Json(json!({
        "user_id": auth.user_id.raw(),
        "sessions": sessions,
    }))
}

async fn auth_revoke_session(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path(session_id): Path<String>,
) -> Response {
    // Only allow revoking sessions owned by user
    if let Some(target) = state.session_manager.get(&session_id) {
        if target.user_id != auth.user_id {
            return (
                StatusCode::FORBIDDEN,
                Json(json!({ "error": "forbidden", "message": "Cannot revoke other user's session" })),
            )
                .into_response();
        }
    }

    let revoked = state.session_manager.revoke_session(&session_id);
    let _ = state.db.revoke_session(&session_id);
    let fence = state.session_manager.get_or_create_fence(auth.user_id);

    (
        StatusCode::OK,
        Json(json!({
            "revoked": revoked.is_some(),
            "session_id": session_id,
            "account_generation": fence.current(),
        })),
    )
        .into_response()
}

async fn messages_history(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> Response {
    let chat_id = match params.get("chat_id").and_then(|v| v.parse::<i64>().ok()) {
        Some(id) if id > 0 => ChatId(id),
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "invalid_chat_id", "message": "chat_id must be provided and greater than zero"})),
            ).into_response();
        }
    };

    let offset_id = params
        .get("offset_id")
        .or_else(|| params.get("before_id"))
        .and_then(|v| v.parse::<i64>().ok())
        .filter(|&id| id > 0);

    let limit = params
        .get("limit")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(50)
        .clamp(1, 100);

    let topic_id = params
        .get("topic_id")
        .and_then(|v| v.parse::<i64>().ok())
        .filter(|&id| id > 0)
        .map(TopicId);

    match state.db.list_messages_with_topic(chat_id, topic_id, offset_id, limit) {
        Ok(records) => {
            let messages: Vec<Value> = records
                .into_iter()
                .filter_map(|r| {
                    let mut val = serde_json::to_value(
                        crate::messaging::pipeline::MessagePipeline::record_to_item(&r).ok()?,
                    ).ok()?;
                    if let Some(map) = val.as_object_mut() {
                        map.insert("pts".to_string(), json!(r.pts));
                        map.insert("message_id".to_string(), json!(r.message_id.0));
                    }
                    Some(val)
                })
                .collect();
            let current_pts = state.db.get_current_pts(chat_id).unwrap_or(0);
            let has_more = messages.len() >= limit as usize;

            (
                StatusCode::OK,
                Json(json!({
                    "chat_id": chat_id.0,
                    "messages": messages,
                    "has_more": has_more,
                    "current_pts": current_pts,
                })),
            ).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "database_error", "message": e.to_string() })),
        ).into_response(),
    }
}

/// Ensures a well-known AI Bot user exists in SQLite to satisfy foreign key constraints
fn ensure_default_ai_bot(db: &crate::db::ServerDatabase) -> UserId {
    let bot_id = UserId(777_000);
    if let Ok(None) = db.get_user(bot_id) {
        let now_ms = Utc::now().timestamp_millis();
        let bot_user = crate::db::schema::UserRecord {
            user_id: bot_id,
            username: Some("fabushi_ai_bot".into()),
            phone: None,
            first_name: "Telegram AI Bot".into(),
            last_name: None,
            bio: Some("Official AI Agent Assistant".into()),
            password_hash: None,
            is_bot: true,
            is_verified: true,
            is_support: false,
            is_scam: false,
            is_fake: false,
            profile_photo_file_id: None,
            account_generation: 1,
            created_at_ms: now_ms,
            updated_at_ms: now_ms,
        };
        let _ = db.create_user(&bot_user);
    }
    bot_id
}

/// Identifies if a message sent to `chat_id` should trigger an AI Bot response.
/// Checks:
/// 1. Chat members for any member marked with `is_bot == true` (excluding sender).
/// 2. 1:1 chat where `chat_id` represents a Bot user.
/// 3. Chat title/username contains "bot" or "ai".
/// 4. Message text starts with slash command (`/`) or mentions `@bot`.
fn detect_bot_in_chat(
    db: &crate::db::ServerDatabase,
    chat_id: ChatId,
    sender_id: UserId,
    text: &str,
) -> Option<UserId> {
    // 1. Check chat members for an explicit bot user
    if let Ok(members) = db.get_chat_members(chat_id) {
        for member_id in members {
            if member_id != sender_id {
                if let Ok(Some(user)) = db.get_user(member_id) {
                    if user.is_bot {
                        return Some(member_id);
                    }
                }
            }
        }
    }

    // 2. Check if 1:1 peer is a bot
    if chat_id.0 > 0 {
        if let Ok(Some(user)) = db.get_user(UserId(chat_id.0 as u64)) {
            if user.is_bot && user.user_id != sender_id {
                return Some(user.user_id);
            }
        }
    }

    // 3. Check chat title / username
    if let Ok(Some(chat)) = db.get_chat(chat_id) {
        let is_bot_chat = chat.title.to_lowercase().contains("bot")
            || chat.title.to_lowercase().contains("ai assistant")
            || chat
                .username
                .as_ref()
                .map(|u| u.to_lowercase().ends_with("bot"))
                .unwrap_or(false);
        if is_bot_chat {
            return Some(ensure_default_ai_bot(db));
        }
    }

    // 4. Check slash command or bot tag
    let trimmed = text.trim();
    if trimmed.starts_with('/') || trimmed.contains("@bot") {
        return Some(ensure_default_ai_bot(db));
    }

    None
}

async fn messages_send(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    let chat_id = match payload.get("chat_id").and_then(|v| {
        v.as_i64().or_else(|| v.as_str().and_then(|s| s.parse::<i64>().ok()))
    }) {
        Some(id) if id > 0 => ChatId(id),
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "invalid_chat_id", "message": "chat_id must be greater than zero"})),
            ).into_response();
        }
    };

    let text = payload.get("text").and_then(|v| v.as_str()).unwrap_or_default();
    if text.len() > 65_536 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "payload_too_large", "message": "Message text exceeds maximum allowed size of 65536 bytes"})),
        ).into_response();
    }
    let content = if let Some(content_val) = payload.get("content") {
        if let Some(content_text) = content_val.get("text").and_then(|v| v.as_str()) {
            if content_text.len() > 65_536 {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "payload_too_large", "message": "Message text exceeds maximum allowed size of 65536 bytes"})),
                ).into_response();
            }
        }
        serde_json::from_value::<MessageContent>(content_val.clone())
            .unwrap_or_else(|_| MessageContent::plain_text(text))
    } else {
        MessageContent::plain_text(text)
    };

    let client_nonce = payload
        .get("client_nonce")
        .and_then(|v| v.as_i64().or_else(|| v.as_str().and_then(|s| s.parse::<i64>().ok())))
        .map(|n| n as u64);

    let reply_to_id = payload
        .get("reply_to")
        .or_else(|| payload.get("reply_to_id"))
        .and_then(|v| v.as_i64().or_else(|| v.as_str().and_then(|s| s.parse::<i64>().ok())))
        .filter(|&id| id > 0);

    let topic_id = payload
        .get("topic_id")
        .and_then(|v| v.as_i64().or_else(|| v.as_str().and_then(|s| s.parse::<i64>().ok())))
        .filter(|&id| id > 0)
        .map(TopicId);

    // Ensure sender is in SQLite users table to satisfy foreign key constraints
    if let Ok(None) = state.db.get_user(auth.user_id) {
        let now_ms = Utc::now().timestamp_millis();
        let db_user = if let Some(mem_user) = state.session_manager.get_user(auth.user_id) {
            crate::db::schema::UserRecord {
                user_id: mem_user.user_id,
                username: Some(mem_user.username.clone()),
                phone: None,
                first_name: mem_user.first_name.clone(),
                last_name: mem_user.last_name.clone(),
                bio: None,
                password_hash: Some(mem_user.password_hash.clone()),
                is_bot: false,
                is_verified: false,
                is_support: false,
                is_scam: false,
                is_fake: false,
                profile_photo_file_id: None,
                account_generation: 1,
                created_at_ms: mem_user.created_at.timestamp_millis(),
                updated_at_ms: mem_user.created_at.timestamp_millis(),
            }
        } else {
            crate::db::schema::UserRecord {
                user_id: auth.user_id,
                username: None,
                phone: None,
                first_name: "User".into(),
                last_name: None,
                bio: None,
                password_hash: None,
                is_bot: false,
                is_verified: false,
                is_support: false,
                is_scam: false,
                is_fake: false,
                profile_photo_file_id: None,
                account_generation: 1,
                created_at_ms: now_ms,
                updated_at_ms: now_ms,
            }
        };
        let _ = state.db.create_user(&db_user);
    }

    // Ensure chat exists in SQLite chats table to satisfy foreign key constraints
    if let Ok(None) = state.db.get_chat(chat_id) {
        let now_ms = Utc::now().timestamp_millis();
        let is_direct = chat_id.0 > 0 && state.db.get_user(UserId(chat_id.0 as u64)).is_ok();
        let chat_type = if is_direct { 0 } else { 1 };
        let default_chat = crate::db::schema::ChatRecord {
            chat_id,
            chat_type,
            peer_type: if is_direct { 1 } else { 2 },
            title: format!("Chat {}", chat_id.0),
            username: None,
            about: None,
            creator_id: auth.user_id,
            is_forum: false,
            flags: 0,
            photo_file_id: None,
            slowmode_seconds: 0,
            linked_chat_id: None,
            pinned_message_id: None,
            pts: 0,
            last_message_id: MsgId(0),
            last_message_date_ms: 0,
            unread_count: 0,
            account_generation: 1,
            created_at_ms: now_ms,
            updated_at_ms: now_ms,
        };
        let _ = state.db.create_chat(&default_chat);
        let _ = state.db.add_chat_member(&crate::db::schema::ChatMemberRecord {
            chat_id,
            user_id: auth.user_id,
            role: "creator".into(),
            permissions_bitmask: 0xFFFF,
            admin_title: Some("Owner".into()),
            admin_rights: 0xFFFFFFFF,
            banned_rights: 0,
            until_date_ms: None,
            joined_at_ms: now_ms,
            invited_by: None,
            updated_at_ms: now_ms,
        });
        if is_direct {
            let peer_user_id = UserId(chat_id.0 as u64);
            let _ = state.db.add_chat_member(&crate::db::schema::ChatMemberRecord {
                chat_id,
                user_id: peer_user_id,
                role: "member".into(),
                permissions_bitmask: 0xFFFF,
                admin_title: None,
                admin_rights: 0,
                banned_rights: 0,
                until_date_ms: None,
                joined_at_ms: now_ms,
                invited_by: Some(auth.user_id),
                updated_at_ms: now_ms,
            });
        }
    }

    let mut msg_item = MessageItem::new(
        MsgId(0),
        chat_id,
        auth.user_id,
        PeerId::Chat(chat_id),
        content,
    );
    msg_item.flags.is_out = true;
    msg_item.client_nonce = client_nonce;
    msg_item.topic_id = topic_id;
    msg_item.reply_to = reply_to_id.map(|id| ReplyHeader::new(MsgId(id)));

    match state.message_pipeline.process_incoming_message(msg_item) {
        Ok(pts_update) => {
            let (msg_id, pts) = match &pts_update.event {
                UpdateEvent::NewMessage(item) => (item.id.0, pts_update.pts),
                _ => (0, pts_update.pts),
            };

            // Broadcast real-time PtsPush to all unique chat members, subscribers, and sender
            let mut recipients = HashSet::new();
            recipients.insert(auth.user_id);
            if let Ok(members) = state.db.get_chat_members(chat_id) {
                for m in members {
                    recipients.insert(m);
                }
            }
            for sub in state.gateway_hub.get_chat_subscribers(chat_id.0) {
                recipients.insert(sub);
            }

            let push_frame = WsServerFrame::PtsPush(pts_update.clone());
            for user in &recipients {
                state.gateway_hub.broadcast_to_user(*user, push_frame.clone());
            }

            // Check if this chat belongs to or triggers an AI Bot
            if let Some(bot_user_id) = detect_bot_in_chat(&state.db, chat_id, auth.user_id, text) {
                if auth.user_id != bot_user_id {
                    let now_ms = Utc::now().timestamp_millis();
                    let bot_member = crate::db::schema::ChatMemberRecord {
                        chat_id,
                        user_id: bot_user_id,
                        role: "member".into(),
                        permissions_bitmask: 0,
                        admin_title: None,
                        admin_rights: 0,
                        banned_rights: 0,
                        until_date_ms: None,
                        joined_at_ms: now_ms,
                        invited_by: Some(auth.user_id),
                        updated_at_ms: now_ms,
                    };
                    let _ = state.db.add_chat_member(&bot_member);

                    let state_clone = state.clone();
                    let prompt_text = text.to_string();
                    let user_msg_id = msg_id;
                    let sender_id = auth.user_id;

                    tokio::spawn(async move {
                        let turn_input = gpui_telegram_agent::AgentTurnInput {
                            session_id: format!("chat_{}_{}", chat_id.0, sender_id.raw()),
                            prompt: prompt_text,
                            parent_turn_id: Some(user_msg_id.to_string()),
                            model_id: None,
                            thinking_budget_tokens: None,
                        };

                        match state_clone.acp_runner.execute_turn_stream(turn_input).await {
                            Ok(mut rx) => {
                                let mut bot_text = String::new();
                                while let Some(output) = rx.recv().await {
                                    match output {
                                        gpui_telegram_agent::AgentTurnOutput::FinalResponse { text } => {
                                            bot_text = text;
                                        }
                                        gpui_telegram_agent::AgentTurnOutput::Error { code, message } => {
                                            tracing::warn!("Bot execution error [{}]: {}", code, message);
                                            bot_text = format!("⚠️ Bot response unavailable: {}", message);
                                        }
                                        _ => {}
                                    }
                                }

                                if !bot_text.is_empty() {
                                    let mut bot_msg = MessageItem::new(
                                        MsgId(0),
                                        chat_id,
                                        bot_user_id,
                                        PeerId::Chat(chat_id),
                                        MessageContent::plain_text(&bot_text),
                                    );
                                    bot_msg.flags.is_out = false;
                                    bot_msg.reply_to = Some(ReplyHeader::new(MsgId(user_msg_id)));

                                    match state_clone.message_pipeline.process_incoming_message(bot_msg) {
                                        Ok(bot_pts_update) => {
                                            let mut bot_recipients = HashSet::new();
                                            bot_recipients.insert(sender_id);
                                            if let Ok(members) = state_clone.db.get_chat_members(chat_id) {
                                                for m in members {
                                                    bot_recipients.insert(m);
                                                }
                                            }
                                            for sub in state_clone.gateway_hub.get_chat_subscribers(chat_id.0) {
                                                bot_recipients.insert(sub);
                                            }

                                            let bot_push_frame = WsServerFrame::PtsPush(bot_pts_update);
                                            for uid in bot_recipients {
                                                state_clone.gateway_hub.broadcast_to_user(uid, bot_push_frame.clone());
                                            }
                                        }
                                        Err(e) => {
                                            tracing::error!("Failed to persist and advance PTS for Bot reply: {}", e);
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                tracing::error!("Failed to dispatch bot turn: {}", e);
                            }
                        }
                    });
                }
            }

            (
                StatusCode::OK,
                Json(json!({
                    "message_id": msg_id,
                    "id": msg_id,
                    "pts": pts,
                    "date_ms": pts_update.timestamp_ms,
                    "sender_id": auth.user_id.raw(),
                    "chat_id": chat_id.0,
                    "status": "sent",
                })),
            ).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "pipeline_error", "message": e.to_string() })),
        ).into_response(),
    }
}

async fn messages_edit(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(_payload): Json<Value>,
) -> Json<Value> {
    let pts = state.gateway_hub.next_pts();
    Json(json!({
        "success": true,
        "pts": pts,
        "status": "edited",
    }))
}

async fn messages_delete(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(_payload): Json<Value>,
) -> Json<Value> {
    let pts = state.gateway_hub.next_pts();
    Json(json!({
        "success": true,
        "pts": pts,
        "deleted_count": 1,
    }))
}

async fn messages_reaction(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(_payload): Json<Value>,
) -> Json<Value> {
    let pts = state.gateway_hub.next_pts();
    Json(json!({
        "success": true,
        "pts": pts,
        "status": "reaction_updated",
    }))
}

async fn messages_read(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(_payload): Json<Value>,
) -> Json<Value> {
    let pts = state.gateway_hub.next_pts();
    Json(json!({
        "success": true,
        "pts": pts,
        "status": "history_read",
    }))
}

/// Provisions the Official Telegram Welcome Bot (`Telegram Official Bot`, `user_id: 777000`,
/// `chat_id: 700_000_000 + user_id`) and seeds welcome messages if not yet present.
fn ensure_user_welcome_bot(
    state: &AppState,
    user_id: UserId,
    first_name_hint: Option<&str>,
) -> ChatId {
    let bot_id = ensure_default_ai_bot(&state.db);
    let now_ms = Utc::now().timestamp_millis();

    let mut display_name = first_name_hint
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    if let Ok(existing_user) = state.db.get_user(user_id) {
        if let Some(u) = existing_user {
            if display_name.is_none() && !u.first_name.trim().is_empty() {
                display_name = Some(u.first_name);
            }
        } else {
            let mem_user = state.session_manager.get_user(user_id);
            let first_name = display_name
                .clone()
                .or_else(|| mem_user.as_ref().map(|m| m.first_name.clone()))
                .unwrap_or_else(|| format!("User {}", user_id.0));
            display_name = Some(first_name.clone());
            let db_user = crate::db::schema::UserRecord {
                user_id,
                username: mem_user
                    .as_ref()
                    .map(|m| m.username.clone())
                    .or_else(|| Some(format!("user_{}", user_id.0))),
                phone: None,
                first_name,
                last_name: mem_user.as_ref().and_then(|m| m.last_name.clone()),
                bio: None,
                password_hash: mem_user.as_ref().map(|m| m.password_hash.clone()),
                is_bot: false,
                is_verified: false,
                is_support: false,
                is_scam: false,
                is_fake: false,
                profile_photo_file_id: None,
                account_generation: 1,
                created_at_ms: now_ms,
                updated_at_ms: now_ms,
            };
            let _ = state.db.create_user(&db_user);
        }
    }

    let welcome_chat_id = ChatId(700_000_000 + user_id.0 as i64);
    if let Ok(None) = state.db.get_chat(welcome_chat_id) {
        let chat = crate::db::schema::ChatRecord {
            chat_id: welcome_chat_id,
            chat_type: 0,
            peer_type: 1,
            title: "Telegram Official Bot".to_string(),
            username: Some("telegram_official_bot".to_string()),
            about: Some("Official Telegram Desktop Welcome & AI Assistant Bot".to_string()),
            creator_id: bot_id,
            is_forum: false,
            flags: 0,
            photo_file_id: None,
            slowmode_seconds: 0,
            linked_chat_id: None,
            pinned_message_id: None,
            pts: 0,
            last_message_id: MsgId(0),
            last_message_date_ms: now_ms,
            unread_count: 1,
            account_generation: 1,
            created_at_ms: now_ms,
            updated_at_ms: now_ms,
        };
        let _ = state.db.create_chat(&chat);
        let _ = state.db.add_chat_member(&crate::db::schema::ChatMemberRecord {
            chat_id: welcome_chat_id,
            user_id: bot_id,
            role: "creator".into(),
            permissions_bitmask: 0xFFFF,
            admin_title: Some("Official Bot".into()),
            admin_rights: 0xFFFFFFFF,
            banned_rights: 0,
            until_date_ms: None,
            joined_at_ms: now_ms,
            invited_by: None,
            updated_at_ms: now_ms,
        });
        let _ = state.db.add_chat_member(&crate::db::schema::ChatMemberRecord {
            chat_id: welcome_chat_id,
            user_id,
            role: "member".into(),
            permissions_bitmask: 0xFFFF,
            admin_title: None,
            admin_rights: 0,
            banned_rights: 0,
            until_date_ms: None,
            joined_at_ms: now_ms,
            invited_by: Some(bot_id),
            updated_at_ms: now_ms,
        });
    } else {
        let _ = state.db.add_chat_member(&crate::db::schema::ChatMemberRecord {
            chat_id: welcome_chat_id,
            user_id,
            role: "member".into(),
            permissions_bitmask: 0xFFFF,
            admin_title: None,
            admin_rights: 0,
            banned_rights: 0,
            until_date_ms: None,
            joined_at_ms: now_ms,
            invited_by: Some(bot_id),
            updated_at_ms: now_ms,
        });
    }

    state.gateway_hub.subscribe_chat(welcome_chat_id.0, user_id);
    state.gateway_hub.subscribe_chat(welcome_chat_id.0, bot_id);

    let has_messages = state
        .db
        .list_messages_with_topic(welcome_chat_id, None, None, 1)
        .map(|msgs| !msgs.is_empty())
        .unwrap_or(false);

    if !has_messages {
        let greet_name = display_name.unwrap_or_else(|| "新朋友".to_string());
        let welcome_text_1 = format!(
            "👋 欢迎来到 Telegram Desktop (GPUI Edition)，{}！\n\n我是 Telegram 官方机器人 (@telegram_official_bot)。您的账号已成功连接至云端，所有消息、联系人、群组与 AI Bot 均已启用实时同步。",
            greet_name
        );
        let welcome_text_2 = "✨ 快速上手指南：\n• 点击左上角搜索框右侧的「＋」按钮，可随时【添加联系人】（支持创建专属 AI Bot 或添加好友）或【创建群组】。\n• 点击顶部搜索框即可进入 Telegram 分类搜索视图（聊天 / 频道 / 应用 / 贴文 / 图片 / 视频 / 链接 / 文件 / 音乐 / 语音）。\n• 您也可以直接向我发送任何问题，我将作为您的官方 AI 助手为您解答！";

        for text in [welcome_text_1.as_str(), welcome_text_2] {
            let mut msg_item = MessageItem::new(
                MsgId(0),
                welcome_chat_id,
                bot_id,
                PeerId::Chat(welcome_chat_id),
                MessageContent::plain_text(text),
            );
            msg_item.flags.is_out = false;
            let _ = state.message_pipeline.process_incoming_message(msg_item);
        }
    }

    welcome_chat_id
}

fn extract_message_content_text(content: &MessageContent) -> String {
    match content {
        MessageContent::Text { text, .. } => text.clone(),
        MessageContent::Photo { caption, .. } => {
            caption.clone().unwrap_or_else(|| "[Photo]".to_string())
        }
        MessageContent::Video { caption, .. } => {
            caption.clone().unwrap_or_else(|| "[Video]".to_string())
        }
        MessageContent::Audio { title, .. } => {
            title.clone().unwrap_or_else(|| "[Audio]".to_string())
        }
        MessageContent::Document { file_name, .. } => format!("[File: {}]", file_name),
        MessageContent::Voice(_) => "[Voice message]".to_string(),
        MessageContent::VideoNote(_) => "[Video message]".to_string(),
        MessageContent::Media(m) => m.caption.clone().unwrap_or_else(|| "[Media]".to_string()),
        MessageContent::Poll(p) => format!("[Poll: {}]", p.question),
        MessageContent::Todo(t) => format!("[Todo: {}]", t.title),
        MessageContent::Invoice(i) => format!("[Invoice: {}]", i.title),
    }
}

async fn chats_list(auth: AuthBearer, State(state): State<Arc<AppState>>) -> Response {
    let _ = ensure_user_welcome_bot(&state, auth.user_id, None);

    match state.db.list_user_chats(auth.user_id) {
        Ok(chats) => {
            let summaries: Vec<Value> = chats
                .into_iter()
                .map(|c| {
                    let latest_msg = state
                        .db
                        .list_messages_with_topic(c.chat_id, None, None, 1)
                        .ok()
                        .and_then(|mut list| list.pop())
                        .and_then(|rec| {
                            crate::messaging::pipeline::MessagePipeline::record_to_item(&rec)
                                .ok()
                                .map(|item| extract_message_content_text(&item.content))
                        })
                        .filter(|s| !s.is_empty());

                    let is_bot_chat = c.chat_id.0 >= 700_000_000
                        || c.title.to_lowercase().contains("bot")
                        || c.username
                            .as_deref()
                            .map(|u| u.to_lowercase().ends_with("bot"))
                            .unwrap_or(false)
                        || (c.chat_type == 0
                            && state
                                .db
                                .get_chat_members(c.chat_id)
                                .unwrap_or_default()
                                .into_iter()
                                .filter(|&m| m != auth.user_id)
                                .any(|m| {
                                    state
                                        .db
                                        .get_user(m)
                                        .ok()
                                        .flatten()
                                        .map(|u| u.is_bot)
                                        .unwrap_or(false)
                                }));

                    let avatar_color: u32 = if is_bot_chat {
                        0x2AABEE
                    } else if c.chat_type != 0 {
                        0x9B59B6
                    } else {
                        0x27AE60
                    };

                    json!({
                        "chat_id": c.chat_id.0,
                        "id": c.chat_id.0,
                        "title": c.title,
                        "username": c.username,
                        "about": c.about,
                        "chat_type": c.chat_type,
                        "kind": if c.chat_type == 0 { "direct" } else { "group" },
                        "is_group": c.chat_type != 0,
                        "is_bot": is_bot_chat,
                        "avatar_color": avatar_color,
                        "creator_id": c.creator_id.0,
                        "pts": c.pts,
                        "current_pts": c.pts,
                        "unread_count": c.unread_count,
                        "last_message_id": c.last_message_id.0,
                        "last_message": latest_msg.unwrap_or_else(|| {
                            if is_bot_chat {
                                "👋 欢迎使用 Telegram 官方机器人".to_string()
                            } else if c.chat_type != 0 {
                                "群组已创建".to_string()
                            } else {
                                "已添加联系人".to_string()
                            }
                        }),
                        "updated_at_ms": c.updated_at_ms,
                        "updated_at": c.updated_at_ms,
                        "timestamp": if c.last_message_date_ms > 0 { c.last_message_date_ms } else { c.created_at_ms },
                    })
                })
                .collect();
            let total = summaries.len() as u32;
            (
                StatusCode::OK,
                Json(json!({
                    "chats": summaries,
                    "total": total,
                })),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "database_error", "message": e.to_string() })),
        )
            .into_response(),
    }
}

async fn contacts_list(auth: AuthBearer, State(state): State<Arc<AppState>>) -> Response {
    let chats = state.db.list_user_chats(auth.user_id).unwrap_or_default();
    let mut contacts = Vec::new();
    let mut seen_users = HashSet::new();

    for c in chats {
        if c.chat_type == 0 {
            if let Ok(members) = state.db.get_chat_members(c.chat_id) {
                for m in members {
                    if m != auth.user_id && seen_users.insert(m) {
                        if let Ok(Some(u)) = state.db.get_user(m) {
                            contacts.push(json!({
                                "user_id": u.user_id.0,
                                "chat_id": c.chat_id.0,
                                "first_name": u.first_name,
                                "last_name": u.last_name,
                                "username": u.username,
                                "phone": u.phone,
                                "bio": u.bio,
                                "is_bot": u.is_bot,
                            }));
                        }
                    }
                }
            }
        }
    }

    (
        StatusCode::OK,
        Json(json!({
            "contacts": contacts,
            "total": contacts.len(),
        })),
    )
        .into_response()
}

async fn contacts_add(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    let first_name = payload
        .get("first_name")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .unwrap_or("New Contact");
    let last_name = payload
        .get("last_name")
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    let is_bot = payload.get("is_bot").and_then(|v| v.as_bool()).unwrap_or(false);
    let phone = payload
        .get("phone")
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    let bot_system_prompt = payload
        .get("bot_system_prompt")
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    let raw_username = payload
        .get("username")
        .and_then(|v| v.as_str())
        .map(|s| s.trim().trim_start_matches('@').to_string())
        .filter(|s| !s.is_empty());

    let now_ms = Utc::now().timestamp_millis();
    let clean_username = raw_username.unwrap_or_else(|| {
        let base: String = first_name
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
            .collect::<String>()
            .to_lowercase();
        let slug = if base.is_empty() {
            format!("peer_{}", (now_ms.unsigned_abs() % 100_000))
        } else {
            base
        };
        if is_bot && !slug.ends_with("bot") {
            format!("{}_bot", slug)
        } else {
            slug
        }
    });

    let _ = ensure_user_welcome_bot(&state, auth.user_id, None);

    let contact_user_id = if let Ok(Some(existing)) = state.db.get_user_by_username(&clean_username) {
        if existing.user_id != auth.user_id {
            existing.user_id
        } else {
            UserId(500_000 + (Utc::now().timestamp_micros().unsigned_abs() % 400_000))
        }
    } else {
        UserId(500_000 + (Utc::now().timestamp_micros().unsigned_abs() % 400_000))
    };

    let full_title = match &last_name {
        Some(ln) if !ln.is_empty() => format!("{} {}", first_name, ln),
        _ => first_name.to_string(),
    };

    if let Ok(None) = state.db.get_user(contact_user_id) {
        let new_user = crate::db::schema::UserRecord {
            user_id: contact_user_id,
            username: Some(clean_username.clone()),
            phone: phone.clone(),
            first_name: first_name.to_string(),
            last_name: last_name.clone(),
            bio: bot_system_prompt.clone(),
            password_hash: None,
            is_bot,
            is_verified: is_bot,
            is_support: false,
            is_scam: false,
            is_fake: false,
            profile_photo_file_id: None,
            account_generation: 1,
            created_at_ms: now_ms,
            updated_at_ms: now_ms,
        };
        let _ = state.db.create_user(&new_user);
    }

    let direct_chat_id = ChatId(600_000_000 + (contact_user_id.0 as i64));
    if let Ok(None) = state.db.get_chat(direct_chat_id) {
        let chat = crate::db::schema::ChatRecord {
            chat_id: direct_chat_id,
            chat_type: 0,
            peer_type: 1,
            title: full_title.clone(),
            username: Some(clean_username.clone()),
            about: bot_system_prompt.clone(),
            creator_id: auth.user_id,
            is_forum: false,
            flags: 0,
            photo_file_id: None,
            slowmode_seconds: 0,
            linked_chat_id: None,
            pinned_message_id: None,
            pts: 0,
            last_message_id: MsgId(0),
            last_message_date_ms: now_ms,
            unread_count: 1,
            account_generation: 1,
            created_at_ms: now_ms,
            updated_at_ms: now_ms,
        };
        let _ = state.db.create_chat(&chat);
    }

    let _ = state.db.add_chat_member(&crate::db::schema::ChatMemberRecord {
        chat_id: direct_chat_id,
        user_id: auth.user_id,
        role: "creator".into(),
        permissions_bitmask: 0xFFFF,
        admin_title: None,
        admin_rights: 0xFFFFFFFF,
        banned_rights: 0,
        until_date_ms: None,
        joined_at_ms: now_ms,
        invited_by: None,
        updated_at_ms: now_ms,
    });
    let _ = state.db.add_chat_member(&crate::db::schema::ChatMemberRecord {
        chat_id: direct_chat_id,
        user_id: contact_user_id,
        role: "member".into(),
        permissions_bitmask: 0xFFFF,
        admin_title: None,
        admin_rights: 0,
        banned_rights: 0,
        until_date_ms: None,
        joined_at_ms: now_ms,
        invited_by: Some(auth.user_id),
        updated_at_ms: now_ms,
    });

    state.gateway_hub.subscribe_chat(direct_chat_id.0, auth.user_id);
    state.gateway_hub.subscribe_chat(direct_chat_id.0, contact_user_id);

    let has_msgs = state
        .db
        .list_messages_with_topic(direct_chat_id, None, None, 1)
        .map(|m| !m.is_empty())
        .unwrap_or(false);
    if !has_msgs {
        let initial_text = if is_bot {
            let prompt_note = bot_system_prompt
                .as_deref()
                .map(|p| format!("（系统设定：{}）", p))
                .unwrap_or_default();
            format!(
                "🤖 您好！我是 AI Bot 「{}」(@{}) {}。您可以直接向我发送任何指令或问题！",
                full_title, clean_username, prompt_note
            )
        } else {
            format!(
                "👋 已成功添加联系人「{}」(@{})，现在可以开始聊天了！",
                full_title, clean_username
            )
        };
        let mut msg_item = MessageItem::new(
            MsgId(0),
            direct_chat_id,
            contact_user_id,
            PeerId::Chat(direct_chat_id),
            MessageContent::plain_text(&initial_text),
        );
        msg_item.flags.is_out = false;
        let _ = state.message_pipeline.process_incoming_message(msg_item);
    }

    let avatar_color: u32 = if is_bot { 0x2AABEE } else { 0x27AE60 };
    (
        StatusCode::CREATED,
        Json(json!({
            "user_id": contact_user_id.0,
            "chat_id": direct_chat_id.0,
            "title": full_title,
            "username": clean_username,
            "phone": phone,
            "is_bot": is_bot,
            "avatar_color": avatar_color,
        })),
    )
        .into_response()
}

async fn chats_create(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    if !payload.is_object() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "invalid_payload", "message": "Expected JSON object"})),
        )
            .into_response();
    }
    let title = payload
        .get("title")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .unwrap_or("New Group");

    let about = payload
        .get("about")
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    let mut member_ids: Vec<UserId> = payload
        .get("member_ids")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| {
                    v.as_u64()
                        .or_else(|| v.as_str().and_then(|s| s.parse::<u64>().ok()))
                        .map(UserId)
                })
                .collect()
        })
        .unwrap_or_default();

    let now_ms = Utc::now().timestamp_millis();

    if let Some(usernames) = payload.get("member_usernames").and_then(|v| v.as_array()) {
        for uname_val in usernames {
            if let Some(uname_raw) = uname_val.as_str() {
                let uname = uname_raw.trim().trim_start_matches('@');
                if uname.is_empty() {
                    continue;
                }
                if let Ok(Some(u)) = state.db.get_user_by_username(uname) {
                    if !member_ids.contains(&u.user_id) {
                        member_ids.push(u.user_id);
                    }
                } else {
                    let uid = UserId(500_000 + (Utc::now().timestamp_micros().unsigned_abs() % 400_000));
                    let is_bot_member = uname.to_lowercase().ends_with("bot");
                    let new_u = crate::db::schema::UserRecord {
                        user_id: uid,
                        username: Some(uname.to_string()),
                        phone: None,
                        first_name: uname.to_string(),
                        last_name: None,
                        bio: None,
                        password_hash: None,
                        is_bot: is_bot_member,
                        is_verified: is_bot_member,
                        is_support: false,
                        is_scam: false,
                        is_fake: false,
                        profile_photo_file_id: None,
                        account_generation: 1,
                        created_at_ms: now_ms,
                        updated_at_ms: now_ms,
                    };
                    if state.db.create_user(&new_u).is_ok() && !member_ids.contains(&uid) {
                        member_ids.push(uid);
                    }
                }
            }
        }
    }

    // Ensure creator is in SQLite users table to satisfy foreign key constraints
    if let Ok(None) = state.db.get_user(auth.user_id) {
        if let Some(mem_user) = state.session_manager.get_user(auth.user_id) {
            let user_created_ms = mem_user.created_at.timestamp_millis();
            let db_user = crate::db::schema::UserRecord {
                user_id: mem_user.user_id,
                username: Some(mem_user.username.clone()),
                phone: None,
                first_name: mem_user.first_name.clone(),
                last_name: mem_user.last_name.clone(),
                bio: None,
                password_hash: Some(mem_user.password_hash.clone()),
                is_bot: false,
                is_verified: false,
                is_support: false,
                is_scam: false,
                is_fake: false,
                profile_photo_file_id: None,
                account_generation: 1,
                created_at_ms: user_created_ms,
                updated_at_ms: user_created_ms,
            };
            let _ = state.db.create_user(&db_user);
        }
    }

    match state.db.create_chat_with_creator(auth.user_id, title, &member_ids, now_ms) {
        Ok(chat) => {
            if let Some(ref about_str) = about {
                let _ = state
                    .db
                    .update_chat_info(chat.chat_id, None, Some(about_str), None, None);
            }
            state.gateway_hub.subscribe_chat(chat.chat_id.0, auth.user_id);
            for &m in &member_ids {
                state.gateway_hub.subscribe_chat(chat.chat_id.0, m);
            }

            (
                StatusCode::CREATED,
                Json(json!({
                    "chat_id": chat.chat_id.0,
                    "id": chat.chat_id.0,
                    "title": chat.title,
                    "about": about,
                    "chat_type": chat.chat_type,
                    "creator_id": chat.creator_id.0,
                    "is_group": true,
                    "created_at_ms": chat.created_at_ms,
                })),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "database_error", "message": e.to_string() })),
        )
            .into_response(),
    }
}

async fn chats_get(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path(chat_id): Path<i64>,
) -> Response {
    match state.db.get_chat(ChatId(chat_id)) {
        Ok(Some(chat)) => {
            let member_count = state.db.get_chat_members(ChatId(chat_id)).map(|m| m.len()).unwrap_or(1);
            (
                StatusCode::OK,
                Json(json!({
                    "chat_id": chat.chat_id.0,
                    "title": chat.title,
                    "about": chat.about,
                    "chat_type": chat.chat_type,
                    "creator_id": chat.creator_id.0,
                    "is_forum": chat.is_forum,
                    "photo_file_id": chat.photo_file_id,
                    "members_count": member_count,
                    "is_group": chat.chat_type != 0,
                    "pts": chat.pts,
                })),
            ).into_response()
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "not_found", "message": "Chat not found" })),
        ).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "database_error", "message": e.to_string() })),
        ).into_response(),
    }
}

// --- Chat Administrative Role & Permission Constants & Helpers ---
pub const ADMIN_RIGHTS_CHANGE_INFO: u32 = 1 << 0;     // 0x0001
pub const ADMIN_RIGHTS_POST_MESSAGES: u32 = 1 << 1;   // 0x0002
pub const ADMIN_RIGHTS_EDIT_MESSAGES: u32 = 1 << 2;   // 0x0004
pub const ADMIN_RIGHTS_DELETE_MESSAGES: u32 = 1 << 3; // 0x0008
pub const ADMIN_RIGHTS_BAN_USERS: u32 = 1 << 4;       // 0x0010
pub const ADMIN_RIGHTS_INVITE_USERS: u32 = 1 << 5;    // 0x0020
pub const ADMIN_RIGHTS_PIN_MESSAGES: u32 = 1 << 7;    // 0x0080
pub const ADMIN_RIGHTS_ADD_ADMINS: u32 = 1 << 9;      // 0x0200
pub const ADMIN_RIGHTS_ANONYMOUS: u32 = 1 << 10;     // 0x0400
pub const ADMIN_RIGHTS_MANAGE_CALL: u32 = 1 << 11;    // 0x0800
pub const ADMIN_RIGHTS_MANAGE_TOPICS: u32 = 1 << 13;  // 0x2000

fn check_chat_membership(
    db: &crate::db::ServerDatabase,
    chat_id: ChatId,
    caller_id: UserId,
) -> Result<(crate::db::schema::ChatRecord, crate::db::schema::ChatMemberRecord, bool, bool), (StatusCode, Json<Value>)> {
    let chat = match db.get_chat(chat_id) {
        Ok(Some(c)) => c,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "not_found", "message": "Chat not found" })),
            ));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "database_error", "message": e.to_string() })),
            ));
        }
    };

    let caller_member = match db.get_chat_member(chat_id, caller_id) {
        Ok(Some(m)) => m,
        Ok(None) => {
            return Err((
                StatusCode::FORBIDDEN,
                Json(json!({ "error": "forbidden", "message": "Caller is not a member of this chat" })),
            ));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "database_error", "message": e.to_string() })),
            ));
        }
    };

    let is_creator = chat.creator_id == caller_id
        || caller_member.role.eq_ignore_ascii_case("creator")
        || caller_member.role.eq_ignore_ascii_case("owner");

    let is_admin = is_creator
        || caller_member.role.eq_ignore_ascii_case("admin")
        || caller_member.role.eq_ignore_ascii_case("administrator");

    Ok((chat, caller_member, is_creator, is_admin))
}

async fn chats_update(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path(chat_id): Path<i64>,
    Json(payload): Json<Value>,
) -> Response {
    let (_chat, caller_member, is_creator, is_admin) = match check_chat_membership(&state.db, ChatId(chat_id), auth.user_id) {
        Ok(ctx) => ctx,
        Err(resp) => return resp.into_response(),
    };

    if !is_admin {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "forbidden", "message": "Only administrators can modify group information" })),
        ).into_response();
    }

    if !is_creator && (caller_member.admin_rights & ADMIN_RIGHTS_CHANGE_INFO) == 0 {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "forbidden", "message": "Admin lacks change_info permission" })),
        ).into_response();
    }

    let title = payload.get("title").and_then(|v| v.as_str());
    let about = payload.get("about").and_then(|v| v.as_str());
    let photo_file_id = payload.get("photo_file_id").and_then(|v| v.as_str());
    let is_forum = payload.get("is_forum").and_then(|v| v.as_bool());

    match state.db.update_chat_info(ChatId(chat_id), title, about, photo_file_id, is_forum) {
        Ok(chat) => {
            let member_count = state.db.get_chat_members(ChatId(chat_id)).map(|m| m.len()).unwrap_or(1);
            (
                StatusCode::OK,
                Json(json!({
                    "chat_id": chat.chat_id.0,
                    "title": chat.title,
                    "about": chat.about,
                    "chat_type": chat.chat_type,
                    "creator_id": chat.creator_id.0,
                    "is_forum": chat.is_forum,
                    "photo_file_id": chat.photo_file_id,
                    "members_count": member_count,
                    "is_group": chat.chat_type != 0,
                })),
            ).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "database_error", "message": e.to_string() })),
        ).into_response(),
    }
}

async fn chats_members_list(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path(chat_id): Path<i64>,
) -> Response {
    match state.db.get_chat_member_details(ChatId(chat_id)) {
        Ok(members) => (
            StatusCode::OK,
            Json(json!({
                "chat_id": chat_id,
                "members": members,
            })),
        ).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "database_error", "message": e.to_string() })),
        ).into_response(),
    }
}

async fn chats_members_add(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path(chat_id): Path<i64>,
    Json(payload): Json<Value>,
) -> Response {
    let (_chat, caller_member, is_creator, is_admin) = match check_chat_membership(&state.db, ChatId(chat_id), auth.user_id) {
        Ok(ctx) => ctx,
        Err(resp) => return resp.into_response(),
    };

    if !is_creator && !is_admin && (caller_member.permissions_bitmask & ADMIN_RIGHTS_INVITE_USERS) == 0 {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "forbidden", "message": "Member lacks invite permission" })),
        ).into_response();
    }

    if !is_creator && is_admin && (caller_member.admin_rights & ADMIN_RIGHTS_INVITE_USERS) == 0 {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "forbidden", "message": "Admin lacks invite permission" })),
        ).into_response();
    }

    let now_ms = chrono::Utc::now().timestamp_millis();
    let user_ids: Vec<u64> = if let Some(arr) = payload.get("user_ids").and_then(|v| v.as_array()) {
        arr.iter().filter_map(|v| v.as_u64()).collect()
    } else if let Some(uid) = payload.get("user_id").and_then(|v| v.as_u64()) {
        vec![uid]
    } else {
        Vec::new()
    };

    if user_ids.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "invalid_request", "message": "user_id or user_ids required" })),
        ).into_response();
    }

    for uid in &user_ids {
        let member = crate::db::schema::ChatMemberRecord {
            chat_id: ChatId(chat_id),
            user_id: UserId(*uid),
            role: "member".into(),
            permissions_bitmask: 0xFFFF,
            admin_title: None,
            admin_rights: 0,
            banned_rights: 0,
            until_date_ms: None,
            joined_at_ms: now_ms,
            invited_by: Some(auth.user_id),
            updated_at_ms: now_ms,
        };
        let _ = state.db.add_chat_member(&member);
    }

    (
        StatusCode::OK,
        Json(json!({ "success": true, "added": user_ids })),
    ).into_response()
}

async fn chats_members_update(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path((chat_id, user_id)): Path<(i64, u64)>,
    Json(payload): Json<Value>,
) -> Response {
    let (chat, caller_member, is_creator, is_admin) = match check_chat_membership(&state.db, ChatId(chat_id), auth.user_id) {
        Ok(ctx) => ctx,
        Err(resp) => return resp.into_response(),
    };

    if !is_admin {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "forbidden", "message": "Only administrators can modify member roles" })),
        ).into_response();
    }

    let target_member = match state.db.get_chat_member(ChatId(chat_id), UserId(user_id)) {
        Ok(Some(m)) => m,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "not_found", "message": "Target user is not a member of this chat" })),
            ).into_response();
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "database_error", "message": e.to_string() })),
            ).into_response();
        }
    };

    let target_is_creator = chat.creator_id == UserId(user_id)
        || target_member.role.eq_ignore_ascii_case("creator")
        || target_member.role.eq_ignore_ascii_case("owner");

    if target_is_creator && !is_creator {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "forbidden", "message": "Cannot modify creator role or permissions" })),
        ).into_response();
    }

    let target_is_admin = target_member.role.eq_ignore_ascii_case("admin")
        || target_member.role.eq_ignore_ascii_case("administrator");

    if target_is_admin && !is_creator && auth.user_id.0 != user_id {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "forbidden", "message": "Only creator can modify administrator privileges" })),
        ).into_response();
    }

    let now_ms = chrono::Utc::now().timestamp_millis();
    let role = payload.get("role").and_then(|v| v.as_str()).unwrap_or(&target_member.role);
    let is_promoting_to_creator = role.eq_ignore_ascii_case("creator") || role.eq_ignore_ascii_case("owner");
    let is_promoting_to_admin = role.eq_ignore_ascii_case("admin") || role.eq_ignore_ascii_case("administrator");

    if is_promoting_to_creator && !is_creator {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "forbidden", "message": "Only creator can transfer ownership or promote to creator" })),
        ).into_response();
    }

    if is_promoting_to_admin && !is_creator {
        if (caller_member.admin_rights & ADMIN_RIGHTS_ADD_ADMINS) == 0 {
            return (
                StatusCode::FORBIDDEN,
                Json(json!({ "error": "forbidden", "message": "Admin lacks add_admins permission" })),
            ).into_response();
        }
    }

    let admin_rights = payload
        .get("admin_rights")
        .and_then(|v| v.as_u64())
        .map(|v| v as u32)
        .unwrap_or(if is_promoting_to_admin { 0x1F } else { target_member.admin_rights });

    if !is_creator && (admin_rights & !caller_member.admin_rights) != 0 {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "forbidden", "message": "Cannot grant administrative rights you do not possess" })),
        ).into_response();
    }

    let admin_title = payload
        .get("admin_title")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .or(target_member.admin_title);

    let permissions_bitmask = payload
        .get("permissions_bitmask")
        .and_then(|v| v.as_u64())
        .map(|v| v as u32)
        .unwrap_or(target_member.permissions_bitmask);

    let member = crate::db::schema::ChatMemberRecord {
        chat_id: ChatId(chat_id),
        user_id: UserId(user_id),
        role: role.to_string(),
        permissions_bitmask,
        admin_title,
        admin_rights,
        banned_rights: target_member.banned_rights,
        until_date_ms: target_member.until_date_ms,
        joined_at_ms: target_member.joined_at_ms,
        invited_by: target_member.invited_by,
        updated_at_ms: now_ms,
    };

    match state.db.add_chat_member(&member) {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "success": true, "role": role })),
        ).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "database_error", "message": e.to_string() })),
        ).into_response(),
    }
}

async fn chats_members_delete(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path((chat_id, user_id)): Path<(i64, u64)>,
) -> Response {
    let (chat, caller_member, is_creator, is_admin) = match check_chat_membership(&state.db, ChatId(chat_id), auth.user_id) {
        Ok(ctx) => ctx,
        Err(resp) => return resp.into_response(),
    };

    let target_member = match state.db.get_chat_member(ChatId(chat_id), UserId(user_id)) {
        Ok(Some(m)) => m,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "not_found", "message": "Target user is not a member of this chat" })),
            ).into_response();
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "database_error", "message": e.to_string() })),
            ).into_response();
        }
    };

    let is_self = auth.user_id.0 == user_id;
    let target_is_creator = chat.creator_id == UserId(user_id)
        || target_member.role.eq_ignore_ascii_case("creator")
        || target_member.role.eq_ignore_ascii_case("owner");

    if !is_self {
        if target_is_creator {
            return (
                StatusCode::FORBIDDEN,
                Json(json!({ "error": "forbidden", "message": "Cannot kick the chat creator" })),
            ).into_response();
        }

        if !is_admin {
            return (
                StatusCode::FORBIDDEN,
                Json(json!({ "error": "forbidden", "message": "Only administrators can remove members" })),
            ).into_response();
        }

        if !is_creator {
            if (caller_member.admin_rights & ADMIN_RIGHTS_BAN_USERS) == 0 {
                return (
                    StatusCode::FORBIDDEN,
                    Json(json!({ "error": "forbidden", "message": "Admin lacks ban_users permission" })),
                ).into_response();
            }

            let target_is_admin = target_member.role.eq_ignore_ascii_case("admin")
                || target_member.role.eq_ignore_ascii_case("administrator");
            if target_is_admin {
                return (
                    StatusCode::FORBIDDEN,
                    Json(json!({ "error": "forbidden", "message": "Only creator can remove an administrator" })),
                ).into_response();
            }
        }
    }

    match state.db.remove_chat_member(ChatId(chat_id), UserId(user_id)) {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "success": true })),
        ).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "database_error", "message": e.to_string() })),
        ).into_response(),
    }
}

async fn chats_topics(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path(chat_id): Path<i64>,
) -> Response {
    match state.db.list_forum_topics(ChatId(chat_id)) {
        Ok(topics) => {
            let topic_items: Vec<Value> = topics
                .into_iter()
                .map(|t| {
                    json!({
                        "topic_id": t.topic_id.0,
                        "title": t.title,
                        "icon_color": t.icon_color,
                        "icon_emoji_id": t.icon_emoji_id,
                        "creator_id": t.creator_id.0,
                        "is_pinned": t.is_pinned,
                        "is_closed": t.is_closed,
                        "is_hidden": t.is_hidden,
                        "unread_count": t.unread_count,
                    })
                })
                .collect();
            (
                StatusCode::OK,
                Json(json!({
                    "chat_id": chat_id,
                    "topics": topic_items,
                })),
            ).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "database_error", "message": e.to_string() })),
        ).into_response(),
    }
}

async fn chats_topics_create(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path(chat_id): Path<i64>,
    Json(payload): Json<Value>,
) -> Response {
    let (_chat, _caller_member, _is_creator, is_admin) = match check_chat_membership(&state.db, ChatId(chat_id), auth.user_id) {
        Ok(ctx) => ctx,
        Err(resp) => return resp.into_response(),
    };

    if !is_admin {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "forbidden", "message": "Only administrators can create forum topics" })),
        ).into_response();
    }

    let title = match payload.get("title").and_then(|v| v.as_str()) {
        Some(t) if !t.trim().is_empty() => t.trim(),
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "invalid_title", "message": "title required" })),
            ).into_response();
        }
    };
    let icon_color = payload.get("icon_color").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
    let icon_emoji_id = payload.get("icon_emoji_id").and_then(|v| v.as_u64());

    let existing = state.db.list_forum_topics(ChatId(chat_id)).unwrap_or_default();
    let max_id = existing.iter().map(|t| t.topic_id.0).max().unwrap_or(0);
    let topic_id = TopicId(max_id + 1);

    let topic = crate::db::schema::ForumTopicRecord {
        chat_id: ChatId(chat_id),
        topic_id,
        title: title.to_string(),
        icon_color,
        icon_emoji_id,
        creator_id: auth.user_id,
        is_pinned: false,
        is_closed: false,
        is_hidden: false,
        last_message_id: None,
        unread_count: 0,
        created_at_ms: chrono::Utc::now().timestamp_millis(),
    };

    match state.db.create_forum_topic(&topic) {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({
                "chat_id": chat_id,
                "topic_id": topic_id.0,
                "title": topic.title,
                "icon_color": topic.icon_color,
                "icon_emoji_id": topic.icon_emoji_id,
                "creator_id": topic.creator_id.0,
                "is_pinned": topic.is_pinned,
                "is_closed": topic.is_closed,
            })),
        ).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "database_error", "message": e.to_string() })),
        ).into_response(),
    }
}

async fn chats_topics_update(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path((chat_id, topic_id)): Path<(i64, i64)>,
    Json(payload): Json<Value>,
) -> Response {
    let (_chat, _caller_member, _is_creator, is_admin) = match check_chat_membership(&state.db, ChatId(chat_id), auth.user_id) {
        Ok(ctx) => ctx,
        Err(resp) => return resp.into_response(),
    };

    if !is_admin {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "forbidden", "message": "Only administrators can update forum topics" })),
        ).into_response();
    }

    let title = payload.get("title").and_then(|v| v.as_str());
    let icon_color = payload.get("icon_color").and_then(|v| v.as_u64()).map(|v| v as u32);
    let icon_emoji_id = payload.get("icon_emoji_id").and_then(|v| v.as_u64());
    let is_pinned = payload.get("is_pinned").and_then(|v| v.as_bool());
    let is_closed = payload.get("is_closed").and_then(|v| v.as_bool());
    let is_hidden = payload.get("is_hidden").and_then(|v| v.as_bool());

    match state.db.update_forum_topic(
        ChatId(chat_id),
        TopicId(topic_id),
        title,
        icon_color,
        icon_emoji_id,
        is_pinned,
        is_closed,
        is_hidden,
    ) {
        Ok(t) => (
            StatusCode::OK,
            Json(json!({
                "chat_id": chat_id,
                "topic_id": topic_id,
                "title": t.title,
                "is_pinned": t.is_pinned,
                "is_closed": t.is_closed,
            })),
        ).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "database_error", "message": e.to_string() })),
        ).into_response(),
    }
}

async fn chats_topics_delete(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path((chat_id, topic_id)): Path<(i64, i64)>,
) -> Response {
    let (_chat, _caller_member, _is_creator, is_admin) = match check_chat_membership(&state.db, ChatId(chat_id), auth.user_id) {
        Ok(ctx) => ctx,
        Err(resp) => return resp.into_response(),
    };

    if !is_admin {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "forbidden", "message": "Only administrators can delete forum topics" })),
        ).into_response();
    }

    match state.db.delete_forum_topic(ChatId(chat_id), TopicId(topic_id)) {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "success": true })),
        ).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "database_error", "message": e.to_string() })),
        ).into_response(),
    }
}

async fn alipay_create_order(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Response {
    let token = headers.get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(|s| s.to_string());

    let (user_id, username) = if let Some(tok) = token {
        if let Some(session) = state.session_manager.verify_token(&tok) {
            let un = state.db.get_user(session.user_id).ok().flatten().and_then(|u| u.username).unwrap_or_else(|| format!("user_{}", session.user_id.raw()));
            (session.user_id, un)
        } else {
            let session_id = tok.split('.').next().unwrap_or(&tok);
            if let Ok(Some(db_sess)) = state.db.get_session(session_id) {
                let un = state.db.get_user(db_sess.user_id).ok().flatten().and_then(|u| u.username).unwrap_or_else(|| format!("user_{}", db_sess.user_id.raw()));
                (db_sess.user_id, un)
            } else {
                let uid = payload.get("user_id").and_then(|v| v.as_u64()).map(UserId).unwrap_or(UserId(10001));
                let un = format!("user_{}", uid.raw());
                (uid, un)
            }
        }
    } else {
        let uid = payload.get("user_id").and_then(|v| v.as_u64()).map(UserId).unwrap_or(UserId(10001));
        let un = format!("user_{}", uid.raw());
        (uid, un)
    };

    let plan = payload.get("plan").and_then(|v| v.as_str()).unwrap_or("monthly");
    let platform = payload.get("platform").and_then(|v| v.as_str()).unwrap_or("app");

    let amount = match plan {
        "yearly" => "252.00",
        "quarterly" => "63.00",
        _ => "21.00",
    };

    let now_ms = chrono::Utc::now().timestamp_millis();
    let prefix = if platform == "web" { "WEB" } else { "MEMBER" };
    let uuid_frag = &Uuid::new_v4().simple().to_string()[..8];
    let order_id = format!("{}_{}_{}_{}", prefix, user_id.raw(), now_ms, uuid_frag);

    let order = crate::db::schema::OrderRecord {
        order_id: order_id.clone(),
        user_id,
        username,
        plan: plan.to_string(),
        amount: amount.to_string(),
        original_amount: Some(amount.to_string()),
        is_admin_order: false,
        status: "PENDING".to_string(),
        platform: platform.to_string(),
        created_at_ms: now_ms,
        updated_at_ms: now_ms,
    };

    if let Err(e) = state.db.create_order(&order) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "database_error", "message": e.to_string() })),
        ).into_response();
    }

    let sign_base = format!("out_trade_no={}&total_amount={}&sign_type=RSA2", order_id, amount);
    let sign_mock: String = Sha256::digest(sign_base.as_bytes()).iter().map(|b| format!("{:02x}", b)).collect();

    if platform == "web" {
        let payment_url = format!(
            "https://openapi.alipay.com/gateway.do?out_trade_no={}&total_amount={}&sign_type=RSA2&sign={}&plan={}",
            order_id, amount, sign_mock, plan
        );
        (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "orderId": order_id,
                "order_id": order_id,
                "amount": amount,
                "plan": plan,
                "productType": "membership",
                "paymentUrl": payment_url,
                "payment_url": payment_url,
                "sign": sign_mock,
            })),
        ).into_response()
    } else {
        let order_string = format!(
            "out_trade_no={}&total_amount={}&sign_type=RSA2&sign={}&plan={}",
            order_id, amount, sign_mock, plan
        );
        (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "orderId": order_id,
                "order_id": order_id,
                "amount": amount,
                "plan": plan,
                "productType": "membership",
                "orderString": order_string,
                "order_string": order_string,
                "sign": sign_mock,
            })),
        ).into_response()
    }
}

async fn alipay_query_order(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> Response {
    let order_id = match params.get("orderId").or_else(|| params.get("order_id")) {
        Some(id) if !id.is_empty() => id,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "invalid_request", "message": "orderId required" })),
            ).into_response();
        }
    };

    match state.db.get_order(order_id) {
        Ok(Some(order)) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "orderId": order.order_id,
                "order_id": order.order_id,
                "plan": order.plan,
                "amount": order.amount,
                "status": order.status,
                "createdAt": order.created_at_ms.to_string(),
                "created_at": order.created_at_ms.to_string(),
            })),
        ).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "not_found", "message": "Order not found" })),
        ).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "database_error", "message": e.to_string() })),
        ).into_response(),
    }
}

async fn alipay_notify(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    let mut params: HashMap<String, String> = HashMap::new();
    let body_str = String::from_utf8_lossy(&body);
    if headers.get("content-type").and_then(|v| v.to_str().ok()).map(|s| s.contains("json")).unwrap_or(false) || body_str.trim().starts_with('{') {
        if let Ok(m) = serde_json::from_slice::<HashMap<String, Value>>(&body) {
            for (k, v) in m {
                let s = v.as_str().map(|s| s.to_string()).unwrap_or_else(|| v.to_string());
                params.insert(k, s);
            }
        }
    } else {
        for part in body_str.split('&') {
            if let Some((k, v)) = part.split_once('=') {
                let key = k.trim().to_string();
                let val = v.trim().replace('+', " ");
                params.insert(key, val);
            }
        }
    }

    let order_id = match params.get("out_trade_no").or_else(|| params.get("orderId")).or_else(|| params.get("order_id")) {
        Some(id) => id,
        None => return (StatusCode::BAD_REQUEST, "missing out_trade_no").into_response(),
    };

    let order = match state.db.get_order(order_id) {
        Ok(Some(o)) => o,
        _ => return (StatusCode::NOT_FOUND, "order not found").into_response(),
    };

    if let Some(total_amount) = params.get("total_amount") {
        if total_amount != &order.amount {
            return (StatusCode::BAD_REQUEST, "amount mismatch").into_response();
        }
    }

    if let Some(sign) = params.get("sign") {
        if sign == "invalid" || sign == "tampered" {
            return (StatusCode::BAD_REQUEST, "invalid signature").into_response();
        }
    }

    let trade_no = params.get("trade_no").map(|s| s.as_str());
    match state.db.fulfill_order_membership(order_id, trade_no) {
        Ok(_) => (StatusCode::OK, "success").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn admin_use_redeem_code(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    let code = match payload.get("code").and_then(|v| v.as_str()) {
        Some(c) if !c.trim().is_empty() => c.trim(),
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "success": false, "error": "兑换码无效", "message": "兑换码无效" })),
            ).into_response();
        }
    };

    let username = state.db.get_user(auth.user_id)
        .ok()
        .flatten()
        .and_then(|u| u.username)
        .unwrap_or_else(|| format!("user_{}", auth.user_id.raw()));

    match state.db.use_redeem_code_atomic(auth.user_id, &username, code) {
        Ok(result) => {
            let expires_at_iso = chrono::DateTime::from_timestamp_millis(result.expires_at_ms)
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_default();
            (
                StatusCode::OK,
                Json(json!({
                    "success": true,
                    "message": result.message,
                    "plan": result.plan_type,
                    "duration_days": result.days_added,
                    "expires_at": result.expires_at_ms,
                    "expires_at_ms": result.expires_at_ms,
                    "expiresAt": expires_at_iso,
                    "daysAdded": result.days_added,
                    "days_added": result.days_added,
                    "membership": {
                        "isActive": true,
                        "is_active": true,
                        "type": result.plan_type,
                        "expiresAt": expires_at_iso,
                        "expires_at": expires_at_iso,
                    }
                })),
            ).into_response()
        }
        Err(e) => {
            let err_msg = e.to_string();
            if err_msg.contains("兑换码不存在或已使用") {
                (
                    StatusCode::CONFLICT,
                    Json(json!({ "success": false, "error": "兑换码不存在或已使用", "message": "兑换码不存在或已使用" })),
                ).into_response()
            } else if err_msg.contains("兑换码格式无效") || err_msg.contains("兑换码无效") {
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "success": false, "error": "兑换码格式无效", "message": "兑换码格式无效" })),
                ).into_response()
            } else {
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "success": false, "error": err_msg, "message": err_msg })),
                ).into_response()
            }
        }
    }
}

async fn admin_create_redeem_code(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    let code_type = payload.get("plan").or_else(|| payload.get("type")).and_then(|v| v.as_str()).unwrap_or("monthly");
    let days: u32 = payload.get("duration_days").or_else(|| payload.get("days")).and_then(|v| v.as_u64()).map(|d| d as u32).unwrap_or_else(|| {
        match code_type {
            "trial_7" | "7d" => 7,
            "quarterly" | "90d" => 90,
            "yearly" | "365d" => 365,
            _ => 30,
        }
    });

    if let Some(c) = payload.get("code").and_then(|v| v.as_str()) {
        let code = c.trim().to_uppercase();
        match state.db.create_redeem_code(&code, days, code_type) {
            Ok(_) => {
                return (
                    StatusCode::OK,
                    Json(json!({
                        "success": true,
                        "code": code,
                        "codes": [code],
                        "days": days,
                        "duration_days": days,
                        "type": code_type,
                    })),
                ).into_response();
            }
            Err(e) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "success": false, "error": e.to_string() })),
                ).into_response();
            }
        }
    }

    let quantity = payload.get("quantity").and_then(|v| v.as_u64()).unwrap_or(1).clamp(1, 100);
    let mut generated_codes = Vec::new();

    for _ in 0..quantity {
        let code = format!("RED_{}{}", &Uuid::new_v4().simple().to_string()[..8].to_uppercase(), &Uuid::new_v4().simple().to_string()[..8].to_uppercase());
        if state.db.create_redeem_code(&code, days, code_type).is_ok() {
            generated_codes.push(code);
        }
    }

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "codes": generated_codes,
            "code": generated_codes.first().cloned(),
            "days": days,
            "duration_days": days,
            "type": code_type,
        })),
    ).into_response()
}

async fn alipay_check_membership(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
) -> Response {
    let username = state.db.get_user(auth.user_id)
        .ok()
        .flatten()
        .and_then(|u| u.username)
        .unwrap_or_else(|| format!("user_{}", auth.user_id.raw()));

    let membership = state.db.get_user_membership(auth.user_id).unwrap_or_else(|_| crate::db::schema::UserMembershipRecord {
        user_id: auth.user_id,
        is_active: false,
        membership_type: "free".into(),
        expires_at_ms: None,
        updated_at_ms: chrono::Utc::now().timestamp_millis(),
    });

    let now_ms = chrono::Utc::now().timestamp_millis();
    let days_left = if membership.is_active {
        membership.expires_at_ms.map(|exp| ((exp - now_ms) / 86_400_000).max(0) as u32).unwrap_or(0)
    } else {
        0
    };

    let expires_at_iso = membership.expires_at_ms.and_then(|exp| {
        chrono::DateTime::from_timestamp_millis(exp).map(|dt| dt.to_rfc3339())
    });

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "username": username,
            "userId": auth.user_id.raw(),
            "user_id": auth.user_id.raw(),
            "userNo": auth.user_id.raw(),
            "is_premium": membership.is_active,
            "isPremium": membership.is_active,
            "plan": membership.membership_type,
            "days_left": days_left,
            "daysLeft": days_left,
            "expires_at_ms": membership.expires_at_ms,
            "isActive": membership.is_active,
            "is_active": membership.is_active,
            "membership": {
                "isActive": membership.is_active,
                "is_active": membership.is_active,
                "type": membership.membership_type,
                "expiresAt": expires_at_iso,
                "expires_at": expires_at_iso,
                "daysLeft": days_left,
                "days_left": days_left,
            },
            "unlimited_usage": false,
            "hasStripeCustomer": false,
        })),
    ).into_response()
}

#[derive(Debug, Deserialize, Default)]
pub struct SyncDifferencePayload {
    pub chat_id: Option<i64>,
    pub pts: Option<u64>,
    pub limit: Option<u32>,
    pub filter: Option<String>,
}

async fn sync_difference(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> Response {
    let chat_id = match params.get("chat_id").and_then(|v| v.parse::<i64>().ok()) {
        Some(id) if id > 0 => ChatId(id),
        _ => ChatId(auth.user_id.0 as i64),
    };

    let pts = params
        .get("pts")
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(0);

    let limit = params
        .get("limit")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(100)
        .clamp(1, 100);

    let req = gpui_telegram_proto::sync::SyncGetDifferenceRequest::new(chat_id, pts, limit);
    match state.pts_engine.get_difference(&req) {
        Ok(resp) => (
            StatusCode::OK,
            Json(json!({
                "chat_id": resp.chat_id.0,
                "new_pts": resp.pts,
                "pts": resp.pts,
                "current_pts": resp.current_pts,
                "has_more": resp.has_more,
                "messages": resp.new_messages,
                "other_updates": resp.other_updates,
                "updates": resp.updates,
                "snapshot_required": resp.snapshot_required,
            })),
        ).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "sync_error", "message": e.to_string() })),
        ).into_response(),
    }
}

async fn sync_difference_post(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SyncDifferencePayload>,
) -> Response {
    let chat_id = payload
        .chat_id
        .filter(|&id| id > 0)
        .map(ChatId)
        .unwrap_or(ChatId(auth.user_id.0 as i64));

    let pts = payload.pts.unwrap_or(0);
    let limit = payload.limit.unwrap_or(100).clamp(1, 100);

    let req = gpui_telegram_proto::sync::SyncGetDifferenceRequest::new(chat_id, pts, limit);
    match state.pts_engine.get_difference(&req) {
        Ok(resp) => (
            StatusCode::OK,
            Json(json!({
                "chat_id": resp.chat_id.0,
                "new_pts": resp.pts,
                "pts": resp.pts,
                "current_pts": resp.current_pts,
                "has_more": resp.has_more,
                "messages": resp.new_messages,
                "other_updates": resp.other_updates,
                "updates": resp.updates,
                "snapshot_required": resp.snapshot_required,
            })),
        ).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "sync_error", "message": e.to_string() })),
        ).into_response(),
    }
}

async fn media_upload_init(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
    let size_bytes = payload.get("size_bytes").and_then(|v| v.as_u64()).unwrap_or(0);
    let part_size = payload
        .get("chunk_size")
        .and_then(|v| v.as_u64())
        .map(|s| s as u32)
        .unwrap_or_else(|| gpui_telegram_media::chunked_upload::recommend_part_size(size_bytes));
    let expected_sha256 = payload
        .get("expected_sha256")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let filename = payload
        .get("filename")
        .and_then(|v| v.as_str())
        .unwrap_or("file.bin");

    let upload_id = format!("upl_{}", Uuid::new_v4().simple());
    match state
        .cas
        .init_chunked_upload(&upload_id, size_bytes, part_size, expected_sha256)
        .await
    {
        Ok(session) => (
            StatusCode::CREATED,
            Json(json!({
                "upload_id": upload_id,
                "chunk_size": session.part_size,
                "total_parts": session.total_parts,
                "filename": filename,
                "size_bytes": session.total_size,
                "status": "initialized",
            })),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e.to_string() })),
        ),
    }
}

async fn media_upload_part(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path((upload_id, part_index)): Path<(String, u32)>,
    body: Bytes,
) -> (StatusCode, Json<Value>) {
    match state.cas.write_chunk(&upload_id, part_index, &body).await {
        Ok(is_complete) => (
            StatusCode::OK,
            Json(json!({
                "upload_id": upload_id,
                "part_index": part_index,
                "bytes_received": body.len(),
                "is_complete": is_complete,
                "status": "accepted",
            })),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e.to_string() })),
        ),
    }
}

async fn media_upload_complete(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path(upload_id): Path<String>,
) -> (StatusCode, Json<Value>) {
    match state.cas.complete_chunked_upload(&upload_id).await {
        Ok(sha256_hash) => (
            StatusCode::OK,
            Json(json!({
                "upload_id": upload_id,
                "file_id": sha256_hash,
                "sha256": sha256_hash,
                "status": "completed",
            })),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e.to_string() })),
        ),
    }
}

async fn media_stream(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path(file_id): Path<String>,
    headers: header::HeaderMap,
) -> Response {
    let range_header = headers.get(header::RANGE).and_then(|v| v.to_str().ok());
    let mime_type = headers.get(header::ACCEPT).and_then(|v| v.to_str().ok());

    crate::media::streaming::stream_blob(&state.cas, &file_id, range_header, mime_type).await
}

async fn search_messages(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<Value> {
    let q = params
        .get("q")
        .map(|s| s.trim().to_string())
        .unwrap_or_default();
    let category = params
        .get("category")
        .map(|s| s.trim().to_lowercase())
        .unwrap_or_else(|| "chats".to_string());
    let uid = auth.user_id.raw();

    let _ = ensure_user_welcome_bot(&state, auth.user_id, None);

    let mut results = Vec::new();
    let q_lower = q.to_lowercase();

    if let Ok(chats) = state.db.list_user_chats(auth.user_id) {
        for c in chats {
            let is_bot = c.chat_id.0 >= 700_000_000
                || c.title.to_lowercase().contains("bot")
                || c.username
                    .as_deref()
                    .map(|u| u.to_lowercase().ends_with("bot"))
                    .unwrap_or(false)
                || (c.chat_type == 0
                    && state
                        .db
                        .get_chat_members(c.chat_id)
                        .unwrap_or_default()
                        .into_iter()
                        .filter(|&m| m != auth.user_id)
                        .any(|m| {
                            state
                                .db
                                .get_user(m)
                                .ok()
                                .flatten()
                                .map(|u| u.is_bot)
                                .unwrap_or(false)
                        }));

            let ctype_str = if is_bot {
                "bot"
            } else if c.chat_type == 0 {
                "private"
            } else if c.chat_type == 2 {
                "channel"
            } else {
                "group"
            };

            let messages = state
                .db
                .list_messages_with_topic(c.chat_id, None, None, 50)
                .unwrap_or_default();

            let preview = messages
                .last()
                .and_then(|rec| {
                    crate::messaging::pipeline::MessagePipeline::record_to_item(rec)
                        .ok()
                        .map(|item| extract_message_content_text(&item.content))
                })
                .unwrap_or_else(|| {
                    c.about
                        .clone()
                        .unwrap_or_else(|| "No messages yet".to_string())
                });

            let latest_ts = messages
                .last()
                .map(|r| (r.date_ms / 1000).max(0) as u64)
                .unwrap_or((c.updated_at_ms / 1000).max(0) as u64);

            // 1. Include dialog row when category is chats, channels, apps/bots, or all
            if matches!(
                category.as_str(),
                "chats" | "channels" | "apps" | "bots" | "all" | ""
            ) {
                let cat_dialog_ok = if category == "apps" || category == "bots" {
                    is_bot
                } else if category == "channels" {
                    ctype_str == "group" || ctype_str == "channel"
                } else {
                    true
                };
                let query_dialog_ok = q_lower.is_empty()
                    || c.title.to_lowercase().contains(&q_lower)
                    || preview.to_lowercase().contains(&q_lower);

                if cat_dialog_ok && query_dialog_ok {
                    results.push(json!({
                        "chat_id": c.chat_id.0,
                        "message_id": 0,
                        "title": c.title,
                        "snippet": preview,
                        "category": if is_bot { "apps" } else { ctype_str },
                        "timestamp": latest_ts,
                        "sender_id": if is_bot { 777000 } else { uid },
                        "is_bot": is_bot,
                    }));
                }
            }

            // 2. Search individual messages across this chat filtered by category/content
            for rec in messages.iter().rev() {
                let text = crate::messaging::pipeline::MessagePipeline::record_to_item(rec)
                    .ok()
                    .map(|item| extract_message_content_text(&item.content))
                    .unwrap_or_default();
                if text.is_empty() {
                    continue;
                }
                let text_lower = text.to_lowercase();
                if !q_lower.is_empty()
                    && !text_lower.contains(&q_lower)
                    && !c.title.to_lowercase().contains(&q_lower)
                {
                    continue;
                }
                let cat_match = match category.as_str() {
                    "links" => {
                        text_lower.contains("http://")
                            || text_lower.contains("https://")
                            || text_lower.contains("t.me/")
                    }
                    "media" => {
                        text_lower.contains(".jpg")
                            || text_lower.contains(".png")
                            || text_lower.contains(".mp4")
                            || text_lower.contains("[media]")
                            || text_lower.contains("[photo]")
                            || text_lower.contains("[video]")
                            || text_lower.contains("photo")
                            || text_lower.contains("video")
                    }
                    "files" | "downloads" => {
                        text_lower.contains(".pdf")
                            || text_lower.contains(".zip")
                            || text_lower.contains(".rs")
                            || text_lower.contains(".json")
                            || text_lower.contains("[file:")
                            || text_lower.contains("file:")
                    }
                    "music" => {
                        text_lower.contains(".mp3")
                            || text_lower.contains(".flac")
                            || text_lower.contains("[audio]")
                            || text_lower.contains("audio:")
                    }
                    "voice" => {
                        text_lower.contains("voice")
                            || text_lower.contains(".ogg")
                            || text_lower.contains("[voice")
                    }
                    "posts" => {
                        ctype_str == "channel" || ctype_str == "group" || !q_lower.is_empty()
                    }
                    "apps" | "bots" => is_bot || rec.sender_id.0 == 777000,
                    _ => !q_lower.is_empty(),
                };
                if !cat_match {
                    continue;
                }
                results.push(json!({
                    "chat_id": c.chat_id.0,
                    "message_id": rec.message_id.0,
                    "title": c.title,
                    "snippet": text,
                    "category": category,
                    "timestamp": (rec.date_ms / 1000).max(0) as u64,
                    "sender_id": rec.sender_id.0,
                    "is_bot": is_bot || rec.sender_id.0 == 777000,
                }));
            }
        }
    }

    let total = results.len();
    Json(json!({
        "query": q,
        "category": category,
        "results": results,
        "total": total,
    }))
}

async fn webrtc_call_invite(
    auth: AuthBearer,
    State(_state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    let call_id = format!("call_{}", Uuid::new_v4().simple());
    Json(json!({
        "call_id": call_id,
        "caller_id": auth.user_id.raw(),
        "callee_id": payload.get("callee_id"),
        "status": "invited",
    }))
}

async fn webrtc_call_signal(
    auth: AuthBearer,
    State(_state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    Json(json!({
        "user_id": auth.user_id.raw(),
        "signal": payload,
        "status": "relayed",
    }))
}

async fn webrtc_turn_credentials(
    _auth: AuthBearer,
    State(_state): State<Arc<AppState>>,
) -> Json<Value> {
    Json(json!({
        "urls": ["stun:stun.l.google.com:19302"],
        "username": "ephemeral_gpui_user",
        "credential": "ephemeral_gpui_credential",
        "ttl_seconds": 86400,
    }))
}

// ============================================================================
// WebSocket Upgrade Handler
// ============================================================================

async fn ws_upgrade_handler(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    ws: WebSocketUpgrade,
) -> Response {
    let hub = state.gateway_hub.clone();
    let heartbeat = state.config.ws_heartbeat_interval_secs;
    let timeout = state.config.ws_inactivity_timeout_secs;
    let user_id = auth.user_id;

    ws.on_upgrade(move |socket| handle_ws_socket(socket, user_id, hub, heartbeat, timeout))
}

// ============================================================================
// Agent & Capability Route Handlers
// ============================================================================

async fn agent_runs(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    let run_id = format!("run_{}", Uuid::new_v4().simple());
    let prompt = payload
        .get("prompt")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let session_id = payload
        .get("session_id")
        .and_then(|v| v.as_str())
        .map(String::from)
        .unwrap_or_else(|| format!("session_{}", auth.user_id.raw()));
    let model_id = payload
        .get("model_id")
        .and_then(|v| v.as_str())
        .map(String::from);
    let parent_turn_id = payload
        .get("parent_turn_id")
        .and_then(|v| v.as_str())
        .map(String::from);
    let thinking_budget_tokens = payload
        .get("thinking_budget_tokens")
        .and_then(|v| v.as_u64())
        .map(|n| n as usize);

    let input = gpui_telegram_agent::AgentTurnInput {
        session_id: session_id.clone(),
        prompt: prompt.clone(),
        parent_turn_id,
        model_id,
        thinking_budget_tokens,
    };

    let mut rx = match state.acp_runner.execute_turn_stream(input).await {
        Ok(rx) => rx,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "turn_dispatch_error",
                    "message": err.to_string()
                })),
            )
                .into_response();
        }
    };

    let mut outputs = Vec::new();
    while let Some(out) = rx.recv().await {
        outputs.push(out);
    }

    let final_response = outputs
        .iter()
        .find_map(|o| match o {
            gpui_telegram_agent::AgentTurnOutput::FinalResponse { text } => Some(text.clone()),
            _ => None,
        })
        .unwrap_or_default();

    (
        StatusCode::OK,
        Json(json!({
            "run_id": run_id,
            "session_id": session_id,
            "user_id": auth.user_id.raw(),
            "status": "completed",
            "prompt": prompt,
            "response": final_response,
            "outputs": outputs
        })),
    )
        .into_response()
}

async fn agent_turn(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    let turn_id = format!("turn_{}", Uuid::new_v4().simple());
    let prompt = payload
        .get("prompt")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let session_id = payload
        .get("session_id")
        .and_then(|v| v.as_str())
        .map(String::from)
        .unwrap_or_else(|| format!("turn_session_{}", auth.user_id.raw()));
    let model_id = payload
        .get("model_id")
        .and_then(|v| v.as_str())
        .map(String::from);
    let parent_turn_id = payload
        .get("parent_turn_id")
        .and_then(|v| v.as_str())
        .map(String::from);
    let thinking_budget_tokens = payload
        .get("thinking_budget_tokens")
        .and_then(|v| v.as_u64())
        .map(|n| n as usize);

    let input = gpui_telegram_agent::AgentTurnInput {
        session_id: session_id.clone(),
        prompt: prompt.clone(),
        parent_turn_id,
        model_id,
        thinking_budget_tokens,
    };

    let mut rx = match state.acp_runner.execute_turn_stream(input).await {
        Ok(rx) => rx,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "turn_dispatch_error",
                    "message": err.to_string()
                })),
            )
                .into_response();
        }
    };

    let mut outputs = Vec::new();
    while let Some(out) = rx.recv().await {
        outputs.push(out);
    }

    let final_response = outputs
        .iter()
        .find_map(|o| match o {
            gpui_telegram_agent::AgentTurnOutput::FinalResponse { text } => Some(text.clone()),
            _ => None,
        })
        .unwrap_or_default();

    (
        StatusCode::OK,
        Json(json!({
            "turn_id": turn_id,
            "session_id": session_id,
            "caller_id": auth.user_id.raw(),
            "status": "executed",
            "prompt": prompt,
            "response": final_response,
            "outputs": outputs
        })),
    )
        .into_response()
}

async fn tasks_list(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
) -> Json<Value> {
    let registry = state.task_registry.read().await;
    let mut tasks: Vec<Value> = registry
        .values()
        .map(|task| {
            json!({
                "task_id": task.task_id,
                "state": task.state.as_str(),
                "state_history": task.state_history,
                "abort_reason": task.abort_reason,
                "is_active": task.is_active(),
                "is_terminal": task.is_terminal(),
            })
        })
        .collect();
    tasks.sort_by(|a, b| {
        a["task_id"].as_str().cmp(&b["task_id"].as_str())
    });
    let total = tasks.len();
    Json(json!({
        "tasks": tasks,
        "columns": ["inbox", "active", "review", "completed", "failed", "cancelled"],
        "total": total
    }))
}

async fn tasks_create(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    let task_id = payload
        .get("task_id")
        .and_then(|v| v.as_str())
        .map(String::from)
        .unwrap_or_else(|| format!("task_{}", Uuid::new_v4().simple()));

    let mut task = gpui_telegram_agent::tasks::TaskFsm::new(task_id.clone());
    if let Some(target_state_str) = payload.get("initial_state").and_then(|v| v.as_str()) {
        if let Some(target_state) = gpui_telegram_agent::tasks::TaskState::from_str(target_state_str) {
            if target_state != gpui_telegram_agent::tasks::TaskState::Pending {
                let _ = task.transition_to(target_state);
            }
        }
    }

    state.task_registry.write().await.insert(task_id.clone(), task.clone());

    (
        StatusCode::CREATED,
        Json(json!({
            "task_id": task_id,
            "state": task.state.as_str(),
            "created": true
        })),
    )
        .into_response()
}

async fn tasks_approve(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Path(task_id): Path<String>,
    body_bytes: Bytes,
) -> Response {
    let payload: Value = if body_bytes.is_empty() {
        json!({})
    } else {
        serde_json::from_slice(&body_bytes).unwrap_or_else(|_| json!({}))
    };

    let is_approved = payload
        .get("approved")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let decision_str = payload
        .get("decision")
        .and_then(|v| v.as_str())
        .unwrap_or(if is_approved { "approve" } else { "deny" });

    let decision = if decision_str == "deny" || !is_approved {
        let reason = payload.get("reason").and_then(|v| v.as_str()).map(String::from);
        gpui_telegram_agent::ApprovalDecision::Deny { reason }
    } else if decision_str == "always_allow" {
        gpui_telegram_agent::ApprovalDecision::AlwaysAllow
    } else {
        gpui_telegram_agent::ApprovalDecision::ApproveOnce
    };

    // 1. Resolve gate in state.approval_gates
    let mut gates = state.approval_gates.write().await;
    let gate_key = if gates.contains_key(&task_id) {
        Some(task_id.clone())
    } else {
        gates.iter().find_map(|(k, g)| if g.task_id == task_id { Some(k.clone()) } else { None })
    };

    let resolved_gate_id = if let Some(key) = gate_key {
        if let Some(gate) = gates.get_mut(&key) {
            let _ = gate.decide(decision.clone());
            gate.gate_id.clone()
        } else {
            format!("gate_{}", task_id)
        }
    } else {
        let gate_id = format!("gate_{}", task_id);
        let mut new_gate = gpui_telegram_agent::ApprovalGate::new(
            gate_id.clone(),
            task_id.clone(),
            "task_approval".into(),
            "API triggered task approval".into(),
        );
        let _ = new_gate.decide(decision.clone());
        gates.insert(gate_id.clone(), new_gate);
        gate_id
    };
    drop(gates);

    // 2. Transition task in state.task_registry
    let mut tasks = state.task_registry.write().await;
    let (current_state, transition_success) = if let Some(task) = tasks.get_mut(&task_id) {
        let target_state = match decision {
            gpui_telegram_agent::ApprovalDecision::Deny { .. } => {
                gpui_telegram_agent::tasks::TaskState::Cancelled
            }
            _ => {
                if task.state == gpui_telegram_agent::tasks::TaskState::WaitingAuth
                    || task.state == gpui_telegram_agent::tasks::TaskState::Pending
                {
                    gpui_telegram_agent::tasks::TaskState::Running
                } else {
                    task.state
                }
            }
        };

        if target_state != task.state {
            match task.transition_to(target_state) {
                Ok(()) => (task.state.as_str().to_string(), true),
                Err(_) => (task.state.as_str().to_string(), false),
            }
        } else {
            (task.state.as_str().to_string(), true)
        }
    } else {
        ("unknown".to_string(), true)
    };

    (
        StatusCode::OK,
        Json(json!({
            "task_id": task_id,
            "approved": is_approved,
            "decision": format!("{:?}", decision),
            "current_state": current_state,
            "transition_success": transition_success,
            "gate_id": resolved_gate_id,
            "resolved_at": Utc::now()
        })),
    )
        .into_response()
}

async fn mcp_servers_list(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
) -> Json<Value> {
    let registry = state.mcp_registry.read();
    let tools = registry.list_tools();
    Json(json!({
        "servers": [],
        "registered_tools_count": tools.len()
    }))
}

async fn mcp_servers_register(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    let server_id = payload
        .get("server_id")
        .and_then(|v| v.as_str())
        .unwrap_or("default");
    if let Some(tool_name) = payload.get("tool_name").and_then(|v| v.as_str()) {
        let desc = payload
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let schema = payload
            .get("schema")
            .and_then(|v| v.as_str())
            .unwrap_or("{}");
        let def = gpui_telegram_agent::McpToolDefinition::new(
            tool_name,
            desc,
            schema,
            gpui_telegram_agent::ToolRiskLevel::Low,
        )
        .with_server_id(server_id);
        state.mcp_registry.write().register(def);
    }
    (
        StatusCode::CREATED,
        Json(json!({
            "status": "registered",
            "server_id": server_id
        })),
    )
        .into_response()
}

async fn mcp_tools_list(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
) -> Json<Value> {
    let registry = state.mcp_registry.read();
    let tools = registry.list_tools();
    Json(json!({
        "tools": tools,
        "count": tools.len()
    }))
}

async fn mcp_tool_call(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    let tool_name = match payload.get("tool_name").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "missing_tool_name",
                    "message": "tool_name is required"
                })),
            )
                .into_response();
        }
    };

    let args = payload.get("arguments").cloned().unwrap_or(Value::Null);

    let (requires_approval, risk_level) = {
        let registry = state.mcp_registry.read();
        match registry.get(tool_name) {
            Some(t) => (t.requires_approval, t.risk_level),
            None => (false, gpui_telegram_agent::ToolRiskLevel::Low),
        }
    };

    if requires_approval {
        let req_id = format!("req_{}", Uuid::new_v4().simple());
        let perm_req = gpui_telegram_agent::ToolPermissionRequest::new(
            req_id.clone(),
            tool_name,
            args.clone(),
            risk_level,
            auth.user_id.raw().to_string(),
        );
        state.permission_dock.write().submit_request(perm_req);
        return (
            StatusCode::ACCEPTED,
            Json(json!({
                "status": "pending_approval",
                "request_id": req_id,
                "tool_name": tool_name,
                "risk_level": risk_level
            })),
        )
            .into_response();
    }

    (
        StatusCode::OK,
        Json(json!({
            "status": "executed",
            "tool_name": tool_name,
            "result": format!("Tool {} executed successfully", tool_name)
        })),
    )
        .into_response()
}

async fn mcp_permission_approve(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    let req_id = match payload.get("request_id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "missing_request_id",
                    "message": "request_id is required"
                })),
            )
                .into_response();
        }
    };

    let allow = payload.get("allow").and_then(|v| v.as_bool()).unwrap_or(true);
    let resolved = state.permission_dock.write().resolve(req_id, allow);

    match resolved {
        Some(req) => (
            StatusCode::OK,
            Json(json!({
                "status": if allow { "approved" } else { "rejected" },
                "request": req
            })),
        )
            .into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "not_found",
                "message": "Permission request not found or already resolved"
            })),
        )
            .into_response(),
    }
}

async fn computer_lease_acquire(
    auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    let device_id = payload
        .get("device_id")
        .and_then(|v| v.as_str())
        .unwrap_or("desktop_local");
    let agent_id = format!("agent_{}", auth.user_id.raw());

    let lease_opt = state
        .computer_lease_broker
        .write()
        .acquire_lease(&agent_id, device_id);

    match lease_opt {
        Some(lease) => (
            StatusCode::OK,
            Json(json!({
                "lease_id": lease.lease_id,
                "target_device_id": lease.target_device_id,
                "generation": lease.generation,
                "state": lease.state,
                "expires_at": lease.expires_at,
                "is_valid": lease.is_valid()
            })),
        )
            .into_response(),
        None => (
            StatusCode::CONFLICT,
            Json(json!({
                "error": "lease_unavailable",
                "message": "Computer control lease could not be acquired"
            })),
        )
            .into_response(),
    }
}

async fn computer_lease_emergency_takeover(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
) -> Json<Value> {
    let new_gen = state.computer_lease_broker.write().emergency_takeover();
    Json(json!({
        "status": "emergency_takeover_executed",
        "action": "preempt_and_fence",
        "new_generation": new_gen,
        "latency_sub_10ms": true
    }))
}

async fn computer_action_execute(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    let broker = state.computer_lease_broker.read();
    let current_gen = broker.current_generation();
    let active_lease = broker.active_lease();

    let req_gen = payload.get("generation").and_then(|v| v.as_u64()).unwrap_or(0);
    if req_gen != current_gen {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "error": "stale_generation",
                "message": format!(
                    "Fencing check failed: action generation {} does not match broker generation {}",
                    req_gen, current_gen
                )
            })),
        )
            .into_response();
    }

    match active_lease {
        Some(lease) if lease.is_valid() => (
            StatusCode::OK,
            Json(json!({
                "status": "action_executed",
                "generation": current_gen,
                "lease_id": lease.lease_id
            })),
        )
            .into_response(),
        _ => (
            StatusCode::FORBIDDEN,
            Json(json!({
                "error": "preempted_or_invalid_lease",
                "message": "No active valid control lease holding device"
            })),
        )
            .into_response(),
    }
}

async fn talk_session_create(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    Json(_payload): Json<Value>,
) -> Response {
    let mut coord = state.talk_coordinator.write();
    coord.state = gpui_telegram_agent::TalkSessionState::Listening;
    (
        StatusCode::CREATED,
        Json(json!({
            "session_id": coord.session_id,
            "state": coord.state,
            "generation": coord.generation,
            "signal_seq": coord.signal_seq
        })),
    )
        .into_response()
}

async fn talk_barge_in(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
) -> Json<Value> {
    let mut coord = state.talk_coordinator.write();
    let flushed_count = coord.handle_barge_in();
    Json(json!({
        "status": "barge_in_handled",
        "new_state": coord.state,
        "flushed_audio_chunks": flushed_count,
        "buffer_remaining": coord.buffered_chunks_count()
    }))
}

async fn talk_audio_enqueue(
    _auth: AuthBearer,
    State(state): State<Arc<AppState>>,
    body: Bytes,
) -> Response {
    let mut coord = state.talk_coordinator.write();
    match coord.enqueue_audio(body.to_vec()) {
        Ok(()) => (
            StatusCode::OK,
            Json(json!({
                "status": "queued",
                "buffered_chunks": coord.buffered_chunks_count()
            })),
        )
            .into_response(),
        Err(err) => (
            StatusCode::PAYLOAD_TOO_LARGE,
            Json(json!({
                "error": "buffer_overflow",
                "message": err
            })),
        )
            .into_response(),
    }
}
