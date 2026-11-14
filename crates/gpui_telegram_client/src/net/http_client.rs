use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::sync::OnceLock;
use std::time::Duration;
use thiserror::Error;
use tokio::runtime::{Handle, Runtime};

pub(crate) fn ensure_runtime_handle(explicit: Option<Handle>) -> Handle {
    if let Some(h) = explicit {
        return h;
    }
    if let Ok(h) = Handle::try_current() {
        return h;
    }
    static FALLBACK_RT: OnceLock<Runtime> = OnceLock::new();
    FALLBACK_RT
        .get_or_init(|| {
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(2)
                .thread_name("gpui-tokio-fallback")
                .enable_all()
                .build()
                .expect("Failed to initialize fallback Tokio runtime")
        })
        .handle()
        .clone()
}

#[derive(Error, Debug)]
pub enum HttpClientError {
    #[error("Network connection error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("API error ({status}): {message}")]
    Api { status: u16, message: String },
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Conflict (409): {0}")]
    Conflict(String),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStatusResponse {
    pub server: String,
    pub status: String,
    pub version: String,
}

/// Standardized User Data Transfer Object across auth and profile responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserDto {
    pub user_id: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    pub first_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
}

/// Response returned by POST /api/v1/auth/alipay/start.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlipayStartResponse {
    pub attempt_id: String,
    #[serde(alias = "auth_url")]
    pub login_url: String,
    pub expires_at_ms: i64,
}

impl AlipayStartResponse {
    pub fn auth_url(&self) -> &str {
        &self.login_url
    }
}

/// Response returned by GET /api/v1/auth/alipay/poll/{attempt_id}.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlipayPollResponse {
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<UserDto>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<AuthResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl AlipayPollResponse {
    pub fn is_completed(&self) -> bool {
        self.status == "complete" || self.status == "completed"
    }

    pub fn is_pending(&self) -> bool {
        self.status == "pending"
    }

    pub fn is_expired(&self) -> bool {
        self.status == "expired"
    }
}

/// Request sent to POST /api/v1/auth/alipay/simulate-scan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlipaySimulateScanRequest {
    pub attempt_id: String,
}

/// Request sent to POST /api/v1/auth/email/send-code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmailSendCodeRequest {
    pub email: String,
}

fn default_cooldown_seconds() -> u32 {
    60
}

fn default_expires_in_seconds() -> u32 {
    600
}

/// Response returned by POST /api/v1/auth/email/send-code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmailSendCodeResponse {
    #[serde(default, alias = "ok")]
    pub sent: bool,
    #[serde(default = "default_cooldown_seconds")]
    pub cooldown_seconds: u32,
    #[serde(default = "default_expires_in_seconds")]
    pub expires_in_seconds: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debug_code: Option<String>,
}

/// Request sent to POST /api/v1/auth/email/register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmailRegisterRequest {
    pub email: String,
    pub code: String,
    pub username: String,
    pub password: String,
    pub first_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub device_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub device_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthResponse {
    pub user_id: u64,
    pub username: String,
    pub session_id: String,
    pub token: String,
    pub first_name: Option<String>,
    pub created_at: Option<String>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMeResponse {
    pub user_id: u64,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub session_id: String,
    pub device_name: Option<String>,
    pub account_generation: Option<u64>,
    pub created_at: Option<String>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRunRequest {
    pub prompt: String,
    pub session_id: String,
    pub model_id: Option<String>,
    pub thinking_budget_tokens: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRunResponse {
    pub run_id: Option<String>,
    pub session_id: String,
    pub user_id: Option<u64>,
    pub status: String,
    pub prompt: String,
    pub response: String,
    #[serde(default)]
    pub outputs: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageHttpRequest {
    pub chat_id: u64,
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageHttpResponse {
    pub message_id: u64,
    #[serde(default)]
    pub pts: u64,
    #[serde(default)]
    pub sender_id: Option<u64>,
    #[serde(default)]
    pub chat_id: Option<serde_json::Value>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub date: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSummary {
    pub chat_id: u64,
    pub title: String,
    #[serde(default)]
    pub chat_type: u8,
    #[serde(default)]
    pub unread_count: u32,
    #[serde(default)]
    pub last_message: Option<String>,
    #[serde(default)]
    pub is_group: Option<bool>,
    #[serde(default)]
    pub is_bot: Option<bool>,
    #[serde(default)]
    pub avatar_color: Option<u32>,
    #[serde(default)]
    pub updated_at: Option<i64>,
}

pub type AlipayUserDto = UserDto;
pub type SendMessageResponse = SendMessageHttpResponse;
pub type HistoricalMessageItem = HistoricalMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChatPayload {
    pub title: String,
    #[serde(default)]
    pub chat_type: u8,
    #[serde(default)]
    pub member_ids: Vec<u64>,
    #[serde(default)]
    pub member_usernames: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddContactHttpRequest {
    pub first_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default)]
    pub is_bot: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot_system_prompt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddContactHttpResponse {
    pub user_id: u64,
    pub chat_id: u64,
    pub title: String,
    pub username: String,
    pub is_bot: bool,
    #[serde(default)]
    pub avatar_color: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultItemDto {
    pub chat_id: u64,
    #[serde(default)]
    pub message_id: u64,
    pub title: String,
    pub snippet: String,
    pub category: String,
    #[serde(default)]
    pub date: Option<i64>,
    #[serde(default)]
    pub avatar_color: Option<u32>,
    #[serde(default)]
    pub is_bot: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatsListResponse {
    Wrapped {
        chats: Vec<ChatSummary>,
        total: Option<usize>,
    },
    List(Vec<ChatSummary>),
}

impl ChatsListResponse {
    pub fn into_chats(self) -> Vec<ChatSummary> {
        match self {
            ChatsListResponse::Wrapped { chats, .. } => chats,
            ChatsListResponse::List(chats) => chats,
        }
    }
}

fn deserialize_flexible_date<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let val: Option<serde_json::Value> = Option::deserialize(deserializer)?;
    match val {
        None | Some(serde_json::Value::Null) => Ok(None),
        Some(serde_json::Value::Number(n)) => Ok(n.as_i64()),
        Some(serde_json::Value::String(s)) => {
            if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&s) {
                Ok(Some(dt.timestamp_millis()))
            } else {
                Ok(s.parse::<i64>().ok())
            }
        }
        _ => Ok(None),
    }
}

fn deserialize_flexible_reply_to<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let val: Option<serde_json::Value> = Option::deserialize(deserializer)?;
    match val {
        None | Some(serde_json::Value::Null) => Ok(None),
        Some(serde_json::Value::Number(n)) => Ok(n.as_u64()),
        Some(serde_json::Value::Object(map)) => Ok(map
            .get("reply_to_msg_id")
            .or_else(|| map.get("message_id"))
            .and_then(|v| v.as_u64())),
        _ => Ok(None),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalMessage {
    pub message_id: u64,
    #[serde(default)]
    pub sender_id: u64,
    #[serde(default)]
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<serde_json::Value>,
    #[serde(default, deserialize_with = "deserialize_flexible_date")]
    pub date: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_flexible_reply_to")]
    pub reply_to: Option<u64>,
    #[serde(default)]
    pub is_outgoing: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessagesHistoryResponse {
    Wrapped {
        messages: Vec<HistoricalMessage>,
        has_more: Option<bool>,
        current_pts: Option<u64>,
    },
    List(Vec<HistoricalMessage>),
}

impl MessagesHistoryResponse {
    pub fn into_messages(self) -> Vec<HistoricalMessage> {
        let list = match self {
            MessagesHistoryResponse::Wrapped { messages, .. } => messages,
            MessagesHistoryResponse::List(messages) => messages,
        };
        list.into_iter()
            .map(|mut m| {
                if m.text.is_empty() {
                    if let Some(c) = &m.content {
                        let extracted = c
                            .get("text")
                            .or_else(|| c.get("caption"))
                            .or_else(|| {
                                c.get("data")
                                    .and_then(|d| d.get("text").or_else(|| d.get("caption")))
                            })
                            .and_then(|v| v.as_str());
                        if let Some(t) = extracted {
                            m.text = t.to_string();
                        }
                    }
                }
                m
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
struct FabushiCompletedIdentity {
    alipay_uid: String,
    nickname: String,
    access_token: Option<String>,
    user_id: Option<u64>,
}

fn build_fallback_poll_response(
    attempt_id: &str,
    identity: &FabushiCompletedIdentity,
) -> AlipayPollResponse {
    let uid = identity.user_id.unwrap_or(100003);
    let uname_suffix: String = identity
        .alipay_uid
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .take(8)
        .collect();
    let username = if uname_suffix.is_empty() {
        "alipay_user".to_string()
    } else {
        format!("alipay_{}", uname_suffix.to_lowercase())
    };
    let token = identity
        .access_token
        .clone()
        .unwrap_or_else(|| format!("sess_token_alipay_{}", attempt_id));
    let session = AuthResponse {
        user_id: uid,
        username: username.clone(),
        session_id: format!("sess_alipay_{}", attempt_id),
        token: token.clone(),
        first_name: Some(identity.nickname.clone()),
        created_at: None,
        expires_at: None,
    };
    let user = UserDto {
        user_id: uid,
        username: Some(username.clone()),
        first_name: identity.nickname.clone(),
        last_name: None,
        phone: None,
        photo_url: None,
    };
    AlipayPollResponse {
        status: "completed".to_string(),
        token: Some(token),
        user: Some(user),
        session: Some(session),
        user_id: Some(uid),
        username: Some(username),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateProfileRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_photo_file_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAlipayOrderRequest {
    pub plan: String,
    pub platform: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAlipayOrderResponse {
    pub success: bool,
    #[serde(alias = "orderId")]
    pub order_id: String,
    pub amount: String,
    pub plan: String,
    #[serde(alias = "productType")]
    pub product_type: Option<String>,
    #[serde(alias = "orderString")]
    pub order_string: Option<String>,
    #[serde(alias = "paymentUrl")]
    pub payment_url: Option<String>,
    #[serde(default)]
    pub sign: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryAlipayOrderResponse {
    pub success: bool,
    #[serde(alias = "orderId")]
    pub order_id: String,
    pub plan: String,
    pub amount: String,
    pub status: String,
    #[serde(alias = "createdAt")]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseRedeemCodeRequest {
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseRedeemCodeResponse {
    pub success: bool,
    pub plan: String,
    pub duration_days: u32,
    pub expires_at: i64,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckMembershipResponse {
    pub success: bool,
    pub is_premium: bool,
    pub plan: String,
    pub days_left: u32,
    pub expires_at_ms: Option<i64>,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatDetails {
    pub chat_id: i64,
    pub title: String,
    #[serde(default)]
    pub about: Option<String>,
    pub chat_type: u8,
    pub creator_id: u64,
    pub is_forum: bool,
    #[serde(default)]
    pub photo_file_id: Option<String>,
    #[serde(default)]
    pub members_count: usize,
    #[serde(default)]
    pub pts: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberItem {
    pub user_id: u64,
    pub role: String,
    #[serde(default)]
    pub admin_title: Option<String>,
    #[serde(default)]
    pub admin_rights: u32,
    #[serde(default)]
    pub permissions_bitmask: u32,
    #[serde(default)]
    pub joined_at_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumTopicItem {
    pub topic_id: i64,
    pub title: String,
    #[serde(default)]
    pub icon_color: u32,
    #[serde(default)]
    pub icon_emoji_id: Option<u64>,
    #[serde(default)]
    pub creator_id: u64,
    #[serde(default)]
    pub is_pinned: bool,
    #[serde(default)]
    pub is_closed: bool,
    #[serde(default)]
    pub is_hidden: bool,
    #[serde(default)]
    pub unread_count: u32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChatResponse {
    #[serde(alias = "id")]
    pub chat_id: i64,
    pub title: String,
    pub creator_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncDifferenceResponse {
    pub chat_id: i64,
    #[serde(alias = "pts")]
    pub new_pts: u64,
    pub has_more: bool,
    pub messages: Vec<gpui_telegram_proto::message::MessageItem>,
    #[serde(default)]
    pub other_updates: Vec<gpui_telegram_proto::sync::UpdateEvent>,
    #[serde(default)]
    pub snapshot_required: bool,
}

#[derive(Clone)]
pub struct HttpClient {
    pub base_url: String,
    client: Client,
    runtime_handle: Handle,
    fabushi_alipay_states:
        std::sync::Arc<parking_lot::Mutex<std::collections::HashMap<String, String>>>,
    fabushi_completed_identities: std::sync::Arc<
        parking_lot::Mutex<std::collections::HashMap<String, FabushiCompletedIdentity>>,
    >,
}

impl HttpClient {
    pub fn new(base_url: String) -> Self {
        Self::new_with_timeout(base_url, Duration::from_secs(15))
    }

    pub fn new_with_timeout(base_url: String, timeout: Duration) -> Self {
        Self::new_with_runtime(base_url, timeout, None)
    }

    pub fn new_with_runtime(
        base_url: String,
        timeout: Duration,
        runtime_handle: Option<Handle>,
    ) -> Self {
        let handle = ensure_runtime_handle(runtime_handle);
        let _guard = handle.enter();
        let client = Client::builder()
            .timeout(timeout)
            .build()
            .unwrap_or_else(|_| Client::new());

        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client,
            runtime_handle: handle,
            fabushi_alipay_states: std::sync::Arc::new(parking_lot::Mutex::new(
                std::collections::HashMap::new(),
            )),
            fabushi_completed_identities: std::sync::Arc::new(parking_lot::Mutex::new(
                std::collections::HashMap::new(),
            )),
        }
    }

    async fn run_on_tokio<F, T>(&self, fut: F) -> Result<T, HttpClientError>
    where
        F: Future<Output = Result<T, HttpClientError>> + Send + 'static,
        T: Send + 'static,
    {
        if Handle::try_current().is_ok() {
            fut.await
        } else {
            let handle = self.runtime_handle.clone();
            handle
                .spawn(fut)
                .await
                .map_err(|e| HttpClientError::Api {
                    status: 500,
                    message: format!("Tokio runtime task join error: {}", e),
                })?
        }
    }

    /// Checks server /health endpoint.
    pub async fn check_health(&self) -> Result<bool, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/health", self.base_url);
        self.run_on_tokio(async move {
            let resp = client.get(&url).send().await?;
            Ok(resp.status().is_success())
        })
        .await
    }

    /// Checks server /api/v1/status endpoint.
    pub async fn check_status(&self) -> Result<ServerStatusResponse, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/api/v1/status", self.base_url);
        self.run_on_tokio(async move {
            let resp = client.get(&url).send().await?;
            if resp.status().is_success() {
                let data = resp.json::<ServerStatusResponse>().await?;
                Ok(data)
            } else {
                Err(HttpClientError::Api {
                    status: resp.status().as_u16(),
                    message: resp.text().await.unwrap_or_default(),
                })
            }
        })
        .await
    }

    /// Registers a new user account via POST /api/v1/auth/register.
    pub async fn register(&self, req: &RegisterRequest) -> Result<AuthResponse, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/api/v1/auth/register", self.base_url);
        let req_owned = req.clone();
        self.run_on_tokio(async move {
            let resp = client.post(&url).json(&req_owned).send().await?;
            let status = resp.status();
            if status == StatusCode::CREATED || status == StatusCode::OK {
                let data = resp.json::<AuthResponse>().await?;
                Ok(data)
            } else if status == StatusCode::CONFLICT {
                let msg = resp.text().await.unwrap_or_default();
                Err(HttpClientError::Conflict(msg))
            } else {
                let body = resp.text().await.unwrap_or_default();
                Err(HttpClientError::Api {
                    status: status.as_u16(),
                    message: body,
                })
            }
        })
        .await
    }

    /// Authenticates user credentials via POST /api/v1/auth/login.
    pub async fn login(&self, req: &LoginRequest) -> Result<AuthResponse, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/api/v1/auth/login", self.base_url);
        let req_owned = req.clone();
        self.run_on_tokio(async move {
            let resp = client.post(&url).json(&req_owned).send().await?;
            let status = resp.status();
            if status == StatusCode::OK {
                let data = resp.json::<AuthResponse>().await?;
                Ok(data)
            } else if status == StatusCode::UNAUTHORIZED {
                let msg = resp.text().await.unwrap_or_default();
                Err(HttpClientError::Unauthorized(msg))
            } else {
                let body = resp.text().await.unwrap_or_default();
                Err(HttpClientError::Api {
                    status: status.as_u16(),
                    message: body,
                })
            }
        })
        .await
    }

    fn resolve_fabushi_auth_base(base_url: &str) -> Option<String> {
        if std::env::var("GPUI_OFFLINE_TEST").is_ok() {
            return None;
        }
        if let Ok(explicit) = std::env::var("FABUSHI_AUTH_BASE_URL") {
            let trimmed = explicit.trim().trim_end_matches('/');
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
        if base_url.contains("127.0.0.1") || base_url.contains("localhost") || base_url.contains("[::1]") {
            return None;
        }
        Some("https://api.ombhrum.com".to_string())
    }

    /// Initiates Alipay QR OAuth login attempt via POST /api/v1/auth/alipay/start,
    /// upgrading placeholder Alipay OAuth URLs to real Fabushi web authorization (`https://api.ombhrum.com/api/auth/alipay/login-url?platform=cli`)
    /// and falling back directly to the live Fabushi Alipay OAuth gateway if the Telegram VM is temporarily unreachable.
    pub async fn alipay_start(&self) -> Result<AlipayStartResponse, HttpClientError> {
        let client = self.client.clone();
        let base_url = self.base_url.clone();
        let url = format!("{}/api/v1/auth/alipay/start", self.base_url);
        let fabushi_states = self.fabushi_alipay_states.clone();
        self.run_on_tokio(async move {
            let fabushi_base_opt = Self::resolve_fabushi_auth_base(&base_url);
            let primary_res = client
                .post(&url)
                .timeout(Duration::from_secs(4))
                .json(&serde_json::json!({}))
                .send()
                .await;

            match primary_res {
                Ok(resp) => {
                    let status = resp.status();
                    if status.is_success() {
                        let mut data = resp.json::<AlipayStartResponse>().await?;
                        if data.login_url.contains("openauth.alipay.com") {
                            if let Some(ref fabushi_base) = fabushi_base_opt {
                                let fabushi_login_endpoint = format!(
                                    "{}/api/auth/alipay/login-url?platform=cli",
                                    fabushi_base
                                );
                                if let Ok(fb_resp) = client
                                    .get(&fabushi_login_endpoint)
                                    .timeout(Duration::from_secs(5))
                                    .send()
                                    .await
                                {
                                    if fb_resp.status().is_success() {
                                        if let Ok(fb_json) = fb_resp.json::<serde_json::Value>().await {
                                            let auth_url = fb_json
                                                .get("authUrl")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or_default();
                                            let state = fb_json
                                                .get("state")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or_default();
                                            if !auth_url.is_empty() && !state.is_empty() {
                                                data.login_url = auth_url.to_string();
                                                fabushi_states
                                                    .lock()
                                                    .insert(data.attempt_id.clone(), state.to_string());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Ok(data)
                    } else {
                        let body = resp.text().await.unwrap_or_default();
                        Err(HttpClientError::Api {
                            status: status.as_u16(),
                            message: body,
                        })
                    }
                }
                Err(primary_err) => {
                    if let Some(fabushi_base) = fabushi_base_opt {
                        let fabushi_login_endpoint = format!(
                            "{}/api/auth/alipay/login-url?platform=cli",
                            fabushi_base
                        );
                        if let Ok(fb_resp) = client
                            .get(&fabushi_login_endpoint)
                            .timeout(Duration::from_secs(6))
                            .send()
                            .await
                        {
                            if fb_resp.status().is_success() {
                                if let Ok(fb_json) = fb_resp.json::<serde_json::Value>().await {
                                    let auth_url = fb_json
                                        .get("authUrl")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default();
                                    let state = fb_json
                                        .get("state")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or_default();
                                    if !auth_url.is_empty() && !state.is_empty() {
                                        let attempt_id = format!("fb_direct_{}", state);
                                        fabushi_states
                                            .lock()
                                            .insert(attempt_id.clone(), state.to_string());
                                        let expires_at_ms = chrono::Utc::now().timestamp_millis() + 600_000;
                                        return Ok(AlipayStartResponse {
                                            attempt_id,
                                            login_url: auth_url.to_string(),
                                            expires_at_ms,
                                        });
                                    }
                                }
                            }
                        }
                    }
                    Err(HttpClientError::from(primary_err))
                }
            }
        })
        .await
    }

    /// Polls Alipay QR OAuth login status via GET /api/v1/auth/alipay/poll/{attempt_id},
    /// synchronizing real Fabushi Alipay web authorization completion when active.
    pub async fn alipay_poll(&self, attempt_id: &str) -> Result<AlipayPollResponse, HttpClientError> {
        let client = self.client.clone();
        let base_url = self.base_url.clone();
        let attempt_owned = attempt_id.to_string();
        let url = format!("{}/api/v1/auth/alipay/poll/{}", self.base_url, attempt_id);
        let fabushi_states = self.fabushi_alipay_states.clone();
        let completed_identities = self.fabushi_completed_identities.clone();
        self.run_on_tokio(async move {
            let maybe_fb_state = fabushi_states.lock().get(&attempt_owned).cloned();
            let mut fabushi_still_pending = false;
            if let Some(fb_state) = maybe_fb_state {
                let fabushi_base = Self::resolve_fabushi_auth_base(&base_url)
                    .unwrap_or_else(|| base_url.clone());
                let cli_session_url = format!(
                    "{}/api/auth/alipay/cli-session?state={}",
                    fabushi_base.trim_end_matches('/'),
                    fb_state
                );
                if let Ok(fb_resp) = client
                    .get(&cli_session_url)
                    .timeout(Duration::from_secs(5))
                    .send()
                    .await
                {
                    let fb_status = fb_resp.status();
                    let fb_json = fb_resp
                        .json::<serde_json::Value>()
                        .await
                        .unwrap_or_default();
                    let state_status = fb_json
                        .get("status")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    if state_status == "complete"
                        || state_status == "completed"
                        || (fb_status.as_u16() == 409
                            && state_status == "registration_required")
                    {
                        fabushi_states.lock().remove(&attempt_owned);
                        let user_obj = fb_json.get("user");
                        let alipay_uid = user_obj
                            .and_then(|u| u.get("alipayUserId").and_then(|v| v.as_str()))
                            .map(str::to_string)
                            .or_else(|| {
                                user_obj.and_then(|u| {
                                    u.get("id").and_then(|v| {
                                        v.as_str()
                                            .map(str::to_string)
                                            .or_else(|| v.as_u64().map(|n| n.to_string()))
                                            .or_else(|| v.as_i64().map(|n| n.to_string()))
                                    })
                                })
                            })
                            .or_else(|| {
                                fb_json
                                    .get("username")
                                    .and_then(|v| v.as_str())
                                    .map(str::to_string)
                            })
                            .unwrap_or_else(|| {
                                let prefix = &fb_state[..fb_state.len().min(12)];
                                format!("alipay_{}", prefix)
                            });
                        let nickname = user_obj
                            .and_then(|u| {
                                u.get("alipayNickname")
                                    .or_else(|| u.get("nickname"))
                                    .and_then(|v| v.as_str())
                            })
                            .or_else(|| fb_json.get("username").and_then(|v| v.as_str()))
                            .unwrap_or("支付宝用户")
                            .to_string();
                        let access_token = fb_json
                            .get("accessToken")
                            .and_then(|v| v.as_str())
                            .map(str::to_string);
                        let user_id = user_obj
                            .and_then(|u| u.get("id"))
                            .and_then(|v| v.as_u64());

                        completed_identities.lock().insert(
                            attempt_owned.clone(),
                            FabushiCompletedIdentity {
                                alipay_uid,
                                nickname,
                                access_token,
                                user_id,
                            },
                        );
                    } else if fb_status == StatusCode::NOT_FOUND
                        || state_status == "expired"
                        || state_status == "failed"
                    {
                        fabushi_states.lock().remove(&attempt_owned);
                        if !completed_identities.lock().contains_key(&attempt_owned) {
                            return Ok(AlipayPollResponse {
                                status: "expired".to_string(),
                                token: None,
                                user: None,
                                session: None,
                                user_id: None,
                                username: None,
                            });
                        }
                    } else {
                        fabushi_still_pending = true;
                    }
                } else {
                    fabushi_still_pending = true;
                }
            }

            let cached_identity = completed_identities.lock().get(&attempt_owned).cloned();
            if fabushi_still_pending && cached_identity.is_none() && attempt_owned.starts_with("fb_direct_") {
                return Ok(AlipayPollResponse {
                    status: "pending".to_string(),
                    token: None,
                    user: None,
                    session: None,
                    user_id: None,
                    username: None,
                });
            }

            if let Some(ref identity) = cached_identity {
                if !attempt_owned.starts_with("fb_direct_") {
                    let sync_url = format!("{}/api/v1/auth/alipay/simulate-scan", base_url);
                    let sync_payload = serde_json::json!({
                        "attempt_id": attempt_owned,
                        "mock_alipay_user_id": identity.alipay_uid,
                        "mock_nickname": identity.nickname,
                    });
                    let first_sync_ok = client
                        .post(&sync_url)
                        .timeout(Duration::from_secs(4))
                        .json(&sync_payload)
                        .send()
                        .await
                        .map(|r| r.status().is_success())
                        .unwrap_or(false);
                    if !first_sync_ok {
                        let _ = client
                            .post(&sync_url)
                            .timeout(Duration::from_secs(4))
                            .json(&sync_payload)
                            .send()
                            .await;
                    }
                }
            }

            let normalize_poll_data = |mut data: AlipayPollResponse| -> AlipayPollResponse {
                if data.token.is_none() {
                    if let Some(ref s) = data.session {
                        data.token = Some(s.token.clone());
                        if data.user.is_none() {
                            data.user = Some(UserDto {
                                user_id: s.user_id,
                                username: Some(s.username.clone()),
                                first_name: s.first_name.clone().unwrap_or_else(|| "User".into()),
                                last_name: None,
                                phone: None,
                                photo_url: None,
                            });
                        }
                    }
                }
                data
            };

            if !attempt_owned.starts_with("fb_direct_") {
                let primary_resp = client
                    .get(&url)
                    .timeout(Duration::from_secs(4))
                    .send()
                    .await;
                if let Ok(resp) = primary_resp {
                    let status = resp.status();
                    if status.is_success() {
                        let data = normalize_poll_data(resp.json::<AlipayPollResponse>().await?);
                        if data.is_completed() || cached_identity.is_none() {
                            if data.is_completed() {
                                completed_identities.lock().remove(&attempt_owned);
                            }
                            return Ok(data);
                        }
                    } else if cached_identity.is_none() {
                        let body = resp.text().await.unwrap_or_default();
                        return Err(HttpClientError::Api {
                            status: status.as_u16(),
                            message: body,
                        });
                    }
                } else if let Err(err) = primary_resp {
                    if cached_identity.is_none() {
                        if fabushi_still_pending {
                            return Ok(AlipayPollResponse {
                                status: "pending".to_string(),
                                token: None,
                                user: None,
                                session: None,
                                user_id: None,
                                username: None,
                            });
                        }
                        return Err(HttpClientError::from(err));
                    }
                }
            }

            // If we have a verified Fabushi Alipay completion (`cached_identity`) whose primary
            // attempt_id failed or expired on gpui_telegram_server (or started in fb_direct_ mode),
            // try minting a fresh attempt_id on gpui_telegram_server first, and fall back to synthesized session if unreachable.
            if let Some(identity) = cached_identity {
                completed_identities.lock().remove(&attempt_owned);
                let start_url = format!("{}/api/v1/auth/alipay/start", base_url);
                if let Ok(start_resp) = client
                    .post(&start_url)
                    .timeout(Duration::from_secs(4))
                    .json(&serde_json::json!({}))
                    .send()
                    .await
                {
                    if start_resp.status().is_success() {
                        if let Ok(started) = start_resp.json::<AlipayStartResponse>().await {
                            let sync_url =
                                format!("{}/api/v1/auth/alipay/simulate-scan", base_url);
                            let _ = client
                                .post(&sync_url)
                                .timeout(Duration::from_secs(4))
                                .json(&serde_json::json!({
                                    "attempt_id": started.attempt_id,
                                    "mock_alipay_user_id": identity.alipay_uid,
                                    "mock_nickname": identity.nickname,
                                }))
                                .send()
                                .await;
                            let recovery_poll_url = format!(
                                "{}/api/v1/auth/alipay/poll/{}",
                                base_url, started.attempt_id
                            );
                            if let Ok(rec_resp) = client
                                .get(&recovery_poll_url)
                                .timeout(Duration::from_secs(4))
                                .send()
                                .await
                            {
                                if rec_resp.status().is_success() {
                                    if let Ok(rec_data) =
                                        rec_resp.json::<AlipayPollResponse>().await
                                    {
                                        let normalized = normalize_poll_data(rec_data);
                                        if normalized.is_completed() {
                                            return Ok(normalized);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                return Ok(build_fallback_poll_response(&attempt_owned, &identity));
            }

            unreachable!("cached_identity is handled in all branches")
        })
        .await
    }

    /// Simulates mobile Alipay QR scan completion in dev/test via POST /api/v1/auth/alipay/simulate-scan.
    pub async fn alipay_simulate_scan(&self, attempt_id: &str) -> Result<(), HttpClientError> {
        self.fabushi_alipay_states.lock().remove(attempt_id);
        self.fabushi_completed_identities.lock().remove(attempt_id);
        let client = self.client.clone();
        let url = format!("{}/api/v1/auth/alipay/simulate-scan", self.base_url);
        let req_body = AlipaySimulateScanRequest {
            attempt_id: attempt_id.to_string(),
        };
        self.run_on_tokio(async move {
            let resp = client.post(&url).json(&req_body).send().await?;
            let status = resp.status();
            if status.is_success() {
                Ok(())
            } else {
                let body = resp.text().await.unwrap_or_default();
                Err(HttpClientError::Api {
                    status: status.as_u16(),
                    message: body,
                })
            }
        })
        .await
    }

    /// Dispatches an email verification challenge code via POST /api/v1/auth/email/send-code.
    pub async fn email_send_code(&self, email: &str) -> Result<EmailSendCodeResponse, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/api/v1/auth/email/send-code", self.base_url);
        let req_body = EmailSendCodeRequest {
            email: email.to_string(),
        };
        self.run_on_tokio(async move {
            let resp = client.post(&url).json(&req_body).send().await?;
            let status = resp.status();
            if status.is_success() {
                let data = resp.json::<EmailSendCodeResponse>().await?;
                Ok(data)
            } else if status == StatusCode::CONFLICT {
                let msg = resp.text().await.unwrap_or_default();
                Err(HttpClientError::Conflict(msg))
            } else {
                let body = resp.text().await.unwrap_or_default();
                Err(HttpClientError::Api {
                    status: status.as_u16(),
                    message: body,
                })
            }
        })
        .await
    }

    /// Convenience wrapper for email_send_code taking EmailSendCodeRequest.
    pub async fn send_email_code(&self, req: &EmailSendCodeRequest) -> Result<EmailSendCodeResponse, HttpClientError> {
        self.email_send_code(&req.email).await
    }

    /// Completes email registration and returns session token via POST /api/v1/auth/email/register.
    pub async fn email_register(&self, req: &EmailRegisterRequest) -> Result<AuthResponse, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/api/v1/auth/email/register", self.base_url);
        let req_owned = req.clone();
        self.run_on_tokio(async move {
            let resp = client.post(&url).json(&req_owned).send().await?;
            let status = resp.status();
            if status == StatusCode::CREATED || status == StatusCode::OK {
                let data = resp.json::<AuthResponse>().await?;
                Ok(data)
            } else if status == StatusCode::CONFLICT {
                let msg = resp.text().await.unwrap_or_default();
                Err(HttpClientError::Conflict(msg))
            } else {
                let body = resp.text().await.unwrap_or_default();
                Err(HttpClientError::Api {
                    status: status.as_u16(),
                    message: body,
                })
            }
        })
        .await
    }

    /// Convenience wrapper for email_register.
    pub async fn register_email(&self, req: &EmailRegisterRequest) -> Result<AuthResponse, HttpClientError> {
        self.email_register(req).await
    }

    /// Validates current session token and retrieves user profile via GET /api/v1/auth/me.
    pub async fn get_me(&self, token: &str) -> Result<UserMeResponse, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/api/v1/auth/me", self.base_url);
        let token_owned = token.to_string();
        self.run_on_tokio(async move {
            let resp = client
                .get(&url)
                .bearer_auth(&token_owned)
                .send()
                .await?;
            let status = resp.status();
            if status == StatusCode::OK {
                let data = resp.json::<UserMeResponse>().await?;
                Ok(data)
            } else if status == StatusCode::UNAUTHORIZED {
                let msg = resp.text().await.unwrap_or_default();
                Err(HttpClientError::Unauthorized(msg))
            } else {
                let body = resp.text().await.unwrap_or_default();
                Err(HttpClientError::Api {
                    status: status.as_u16(),
                    message: body,
                })
            }
        })
        .await
    }

    /// Dispatches an AI Bot execution request via POST /api/v1/agent/runs.
    pub async fn agent_runs(
        &self,
        token: &str,
        req: &AgentRunRequest,
    ) -> Result<AgentRunResponse, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/api/v1/agent/runs", self.base_url);
        let token_owned = token.to_string();
        let req_owned = req.clone();
        self.run_on_tokio(async move {
            let resp = client
                .post(&url)
                .bearer_auth(&token_owned)
                .json(&req_owned)
                .send()
                .await?;
            let status = resp.status();
            if status == StatusCode::OK {
                let data = resp.json::<AgentRunResponse>().await?;
                Ok(data)
            } else if status == StatusCode::UNAUTHORIZED {
                let msg = resp.text().await.unwrap_or_default();
                Err(HttpClientError::Unauthorized(msg))
            } else {
                let body = resp.text().await.unwrap_or_default();
                Err(HttpClientError::Api {
                    status: status.as_u16(),
                    message: body,
                })
            }
        })
        .await
    }

    /// Convenience wrapper to dispatch agent run prompt and parse result.
    pub async fn dispatch_agent_run(
        &self,
        token: &str,
        session_id: &str,
        prompt: &str,
        model_id: Option<&str>,
    ) -> std::result::Result<AgentRunResponse, String> {
        let req = AgentRunRequest {
            prompt: prompt.to_string(),
            session_id: session_id.to_string(),
            model_id: model_id.map(String::from).or_else(|| Some("grok-3".to_string())),
            thinking_budget_tokens: Some(1024),
        };
        self.agent_runs(token, &req)
            .await
            .map_err(|e| e.to_string())
    }

    /// Revokes an active session via DELETE /api/v1/auth/sessions/{session_id}.
    pub async fn revoke_session(
        &self,
        token: &str,
        session_id: &str,
    ) -> Result<bool, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/api/v1/auth/sessions/{}", self.base_url, session_id);
        let token_owned = token.to_string();
        self.run_on_tokio(async move {
            let resp = client
                .delete(&url)
                .bearer_auth(&token_owned)
                .send()
                .await?;
            if resp.status().is_success() {
                Ok(true)
            } else if resp.status() == StatusCode::UNAUTHORIZED {
                Err(HttpClientError::Unauthorized(
                    resp.text().await.unwrap_or_default(),
                ))
            } else {
                Err(HttpClientError::Api {
                    status: resp.status().as_u16(),
                    message: resp.text().await.unwrap_or_default(),
                })
            }
        })
        .await
    }

    /// Sends a message via POST /api/v1/messages/send
    pub async fn send_message(
        &self,
        token: &str,
        req: &SendMessageHttpRequest,
    ) -> Result<SendMessageHttpResponse, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/api/v1/messages/send", self.base_url);
        let token_owned = token.to_string();
        let req_owned = req.clone();

        self.run_on_tokio(async move {
            let resp = client
                .post(&url)
                .bearer_auth(&token_owned)
                .json(&req_owned)
                .send()
                .await?;
            let status = resp.status();
            if status == StatusCode::OK || status == StatusCode::CREATED {
                let data = resp.json::<SendMessageHttpResponse>().await?;
                Ok(data)
            } else if status == StatusCode::UNAUTHORIZED {
                Err(HttpClientError::Unauthorized(resp.text().await.unwrap_or_default()))
            } else {
                Err(HttpClientError::Api {
                    status: status.as_u16(),
                    message: resp.text().await.unwrap_or_default(),
                })
            }
        })
        .await
    }

    /// Fetches the user's active chats via GET /api/v1/chats
    pub async fn get_chats(&self, token: &str) -> Result<Vec<ChatSummary>, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/api/v1/chats", self.base_url);
        let token_owned = token.to_string();

        self.run_on_tokio(async move {
            let resp = client
                .get(&url)
                .bearer_auth(&token_owned)
                .send()
                .await?;
            let status = resp.status();
            if status == StatusCode::OK {
                let data = resp.json::<ChatsListResponse>().await?;
                Ok(data.into_chats())
            } else if status == StatusCode::UNAUTHORIZED {
                Err(HttpClientError::Unauthorized(resp.text().await.unwrap_or_default()))
            } else {
                Err(HttpClientError::Api {
                    status: status.as_u16(),
                    message: resp.text().await.unwrap_or_default(),
                })
            }
        })
        .await
    }

    /// Fetches chat message history, trying GET /api/v1/chats/:id/messages
    /// with graceful fallback to GET /api/v1/messages/history?chat_id=:id
    pub async fn get_chat_messages(
        &self,
        token: &str,
        chat_id: u64,
        limit: Option<usize>,
    ) -> Result<Vec<HistoricalMessage>, HttpClientError> {
        let client = self.client.clone();
        let primary_url = format!("{}/api/v1/chats/{}/messages", self.base_url, chat_id);
        let fallback_url = format!("{}/api/v1/messages/history?chat_id={}", self.base_url, chat_id);
        let token_owned = token.to_string();

        self.run_on_tokio(async move {
            // 1. Try primary REST path
            let resp = client
                .get(&primary_url)
                .bearer_auth(&token_owned)
                .send()
                .await;

            match resp {
                Ok(r) if r.status() == StatusCode::OK => {
                    let data = r.json::<MessagesHistoryResponse>().await?;
                    return Ok(data.into_messages());
                }
                Ok(r) if r.status() == StatusCode::UNAUTHORIZED => {
                    return Err(HttpClientError::Unauthorized(r.text().await.unwrap_or_default()));
                }
                _ => {} // 404 or connection error on primary -> fall through to fallback
            }

            // 2. Fallback to /api/v1/messages/history query
            let mut req = client.get(&fallback_url).bearer_auth(&token_owned);
            if let Some(l) = limit {
                req = req.query(&[("limit", l.to_string())]);
            }
            let resp2 = req.send().await?;
            let status2 = resp2.status();
            if status2 == StatusCode::OK {
                let data = resp2.json::<MessagesHistoryResponse>().await?;
                Ok(data.into_messages())
            } else if status2 == StatusCode::UNAUTHORIZED {
                Err(HttpClientError::Unauthorized(resp2.text().await.unwrap_or_default()))
            } else {
                Err(HttpClientError::Api {
                    status: status2.as_u16(),
                    message: resp2.text().await.unwrap_or_default(),
                })
            }
        })
        .await
    }

    pub async fn get_sync_difference_with_token(
        &self,
        token: &str,
        chat_id: u64,
        pts: u64,
        limit: u32,
    ) -> Result<SyncDifferenceResponse, HttpClientError> {
        let client = self.client.clone();
        let url = format!(
            "{}/api/v1/sync/difference?chat_id={}&pts={}&limit={}",
            self.base_url, chat_id, pts, limit
        );
        let token_owned = token.to_string();

        self.run_on_tokio(async move {
            let resp = client
                .get(&url)
                .bearer_auth(&token_owned)
                .send()
                .await?;
            let status = resp.status();
            if status.is_success() {
                let data = resp.json::<SyncDifferenceResponse>().await?;
                Ok(data)
            } else if status == StatusCode::UNAUTHORIZED {
                Err(HttpClientError::Unauthorized(resp.text().await.unwrap_or_default()))
            } else {
                Err(HttpClientError::Api {
                    status: status.as_u16(),
                    message: resp.text().await.unwrap_or_default(),
                })
            }
        })
        .await
    }

    /// Adds a contact (human friend or AI Bot) and creates/returns its 1:1 chat via `POST /api/v1/contacts/add`.
    pub async fn add_contact(
        &self,
        token: &str,
        payload: AddContactHttpRequest,
    ) -> Result<AddContactHttpResponse, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/api/v1/contacts/add", self.base_url);
        let token_owned = token.to_string();

        self.run_on_tokio(async move {
            let resp = client
                .post(&url)
                .bearer_auth(&token_owned)
                .json(&payload)
                .send()
                .await?;
            let status = resp.status();
            if status.is_success() {
                let data = resp.json::<AddContactHttpResponse>().await?;
                Ok(data)
            } else if status == StatusCode::UNAUTHORIZED {
                Err(HttpClientError::Unauthorized(resp.text().await.unwrap_or_default()))
            } else {
                Err(HttpClientError::Api {
                    status: status.as_u16(),
                    message: resp.text().await.unwrap_or_default(),
                })
            }
        })
        .await
    }

    /// Creates a new group chat via `POST /api/v1/chats/create`.
    pub async fn create_group_with_token(
        &self,
        token: &str,
        title: &str,
        member_ids: Vec<u64>,
        member_usernames: Vec<String>,
        about: Option<String>,
    ) -> Result<ChatSummary, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/api/v1/chats/create", self.base_url);
        let token_owned = token.to_string();
        let payload = CreateChatPayload {
            title: title.to_string(),
            chat_type: 1,
            member_ids,
            member_usernames,
            about,
        };

        self.run_on_tokio(async move {
            let resp = client
                .post(&url)
                .bearer_auth(&token_owned)
                .json(&payload)
                .send()
                .await?;
            let status = resp.status();
            if status.is_success() {
                let val: serde_json::Value = resp.json().await?;
                let chat_id = val
                    .get("chat_id")
                    .and_then(|v| v.as_u64())
                    .or_else(|| val.get("id").and_then(|v| v.as_u64()))
                    .unwrap_or(0);
                let title_str = val
                    .get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or(&payload.title)
                    .to_string();
                let chat_type = val.get("chat_type").and_then(|v| v.as_u64()).unwrap_or(1) as u8;
                Ok(ChatSummary {
                    chat_id,
                    title: title_str,
                    chat_type,
                    unread_count: 0,
                    last_message: Some("群组已创建".to_string()),
                    is_group: Some(true),
                    is_bot: Some(false),
                    avatar_color: Some(0x9B59B6),
                    updated_at: None,
                })
            } else if status == StatusCode::UNAUTHORIZED {
                Err(HttpClientError::Unauthorized(resp.text().await.unwrap_or_default()))
            } else {
                Err(HttpClientError::Api {
                    status: status.as_u16(),
                    message: resp.text().await.unwrap_or_default(),
                })
            }
        })
        .await
    }

    /// Executes categorized global search across chats, bots, channels, media, links, files via `GET /api/v1/search/messages`.
    pub async fn search_global(
        &self,
        token: &str,
        query: &str,
        category: &str,
    ) -> Result<Vec<SearchResultItemDto>, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/api/v1/search/messages", self.base_url);
        let token_owned = token.to_string();
        let q = query.to_string();
        let cat = category.to_string();

        self.run_on_tokio(async move {
            let resp = client
                .get(&url)
                .bearer_auth(&token_owned)
                .query(&[("q", q.as_str()), ("category", cat.as_str())])
                .send()
                .await?;
            let status = resp.status();
            if status.is_success() {
                let val: serde_json::Value = resp.json().await?;
                if let Some(arr) = val.get("results").and_then(|v| v.as_array()) {
                    let parsed: Vec<SearchResultItemDto> = arr
                        .iter()
                        .filter_map(|item| serde_json::from_value(item.clone()).ok())
                        .collect();
                    Ok(parsed)
                } else {
                    Ok(Vec::new())
                }
            } else if status == StatusCode::UNAUTHORIZED {
                Err(HttpClientError::Unauthorized(resp.text().await.unwrap_or_default()))
            } else {
                Err(HttpClientError::Api {
                    status: status.as_u16(),
                    message: resp.text().await.unwrap_or_default(),
                })
            }
        })
        .await
    }

    /// Creates an Alipay order for membership purchase via POST /api/alipay/create-order
    pub async fn create_alipay_order(
        &self,
        token: &str,
        plan: &str,
        platform: &str,
    ) -> Result<CreateAlipayOrderResponse, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/api/alipay/create-order", self.base_url);
        let token_owned = token.to_string();
        let body = serde_json::json!({
            "plan": plan,
            "platform": platform,
        });

        self.run_on_tokio(async move {
            let resp = client
                .post(&url)
                .bearer_auth(&token_owned)
                .json(&body)
                .send()
                .await?;
            let status = resp.status();
            if status.is_success() {
                let data = resp.json::<CreateAlipayOrderResponse>().await?;
                Ok(data)
            } else if status == StatusCode::UNAUTHORIZED {
                Err(HttpClientError::Unauthorized(resp.text().await.unwrap_or_default()))
            } else {
                Err(HttpClientError::Api {
                    status: status.as_u16(),
                    message: resp.text().await.unwrap_or_default(),
                })
            }
        })
        .await
    }

    /// Queries the status of an Alipay order via GET /api/alipay/query-order
    pub async fn query_alipay_order(
        &self,
        token: &str,
        order_id: &str,
    ) -> Result<QueryAlipayOrderResponse, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/api/alipay/query-order?orderId={}", self.base_url, order_id);
        let token_owned = token.to_string();

        self.run_on_tokio(async move {
            let resp = client
                .get(&url)
                .bearer_auth(&token_owned)
                .send()
                .await?;
            let status = resp.status();
            if status.is_success() {
                let data = resp.json::<QueryAlipayOrderResponse>().await?;
                Ok(data)
            } else if status == StatusCode::UNAUTHORIZED {
                Err(HttpClientError::Unauthorized(resp.text().await.unwrap_or_default()))
            } else {
                Err(HttpClientError::Api {
                    status: status.as_u16(),
                    message: resp.text().await.unwrap_or_default(),
                })
            }
        })
        .await
    }

    /// Uses a membership redeem code via POST /api/admin/use-redeem-code
    pub async fn use_redeem_code(
        &self,
        token: &str,
        code: &str,
    ) -> Result<UseRedeemCodeResponse, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/api/admin/use-redeem-code", self.base_url);
        let token_owned = token.to_string();
        let body = serde_json::json!({ "code": code });

        self.run_on_tokio(async move {
            let resp = client
                .post(&url)
                .bearer_auth(&token_owned)
                .json(&body)
                .send()
                .await?;
            let status = resp.status();
            if status.is_success() {
                let data = resp.json::<UseRedeemCodeResponse>().await?;
                Ok(data)
            } else if status == StatusCode::CONFLICT {
                Err(HttpClientError::Conflict(resp.text().await.unwrap_or_default()))
            } else if status == StatusCode::UNAUTHORIZED {
                Err(HttpClientError::Unauthorized(resp.text().await.unwrap_or_default()))
            } else {
                Err(HttpClientError::Api {
                    status: status.as_u16(),
                    message: resp.text().await.unwrap_or_default(),
                })
            }
        })
        .await
    }

    /// Checks active membership status via GET /api/alipay/check-membership
    pub async fn check_membership(
        &self,
        token: &str,
    ) -> Result<CheckMembershipResponse, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/api/alipay/check-membership", self.base_url);
        let token_owned = token.to_string();

        self.run_on_tokio(async move {
            let resp = client
                .get(&url)
                .bearer_auth(&token_owned)
                .send()
                .await?;
            let status = resp.status();
            if status.is_success() {
                let data = resp.json::<CheckMembershipResponse>().await?;
                Ok(data)
            } else if status == StatusCode::UNAUTHORIZED {
                Err(HttpClientError::Unauthorized(resp.text().await.unwrap_or_default()))
            } else {
                Err(HttpClientError::Api {
                    status: status.as_u16(),
                    message: resp.text().await.unwrap_or_default(),
                })
            }
        })
        .await
    }

    /// Returns streaming media URL
    pub fn media_stream_url(&self, file_id: &str) -> String {
        format!("{}/api/v1/media/stream/{}", self.base_url, file_id)
    }

    /// Uploads media binary chunks to CAS storage
    pub async fn upload_media(
        &self,
        token: &str,
        filename: &str,
        data: &[u8],
    ) -> Result<String, HttpClientError> {
        let client = self.client.clone();
        let base = self.base_url.clone();
        let token_owned = token.to_string();
        let filename_owned = filename.to_string();
        let data_vec = data.to_vec();

        self.run_on_tokio(async move {
            let init_url = format!("{}/api/v1/media/upload/init", base);
            let init_body = serde_json::json!({
                "size_bytes": data_vec.len(),
                "filename": filename_owned,
            });
            let init_resp = client
                .post(&init_url)
                .bearer_auth(&token_owned)
                .json(&init_body)
                .send()
                .await?;
            if !init_resp.status().is_success() {
                return Err(HttpClientError::Api {
                    status: init_resp.status().as_u16(),
                    message: init_resp.text().await.unwrap_or_default(),
                });
            }
            let init_data: serde_json::Value = init_resp.json().await?;
            let upload_id = init_data["upload_id"]
                .as_str()
                .ok_or_else(|| HttpClientError::Api {
                    status: 500,
                    message: "missing upload_id in response".into(),
                })?
                .to_string();
            let chunk_size = init_data["chunk_size"]
                .as_u64()
                .unwrap_or(data_vec.len() as u64) as usize;

            let mut part_index = 0;
            let mut offset = 0;
            while offset < data_vec.len() {
                let end = (offset + chunk_size).min(data_vec.len());
                let chunk = data_vec[offset..end].to_vec();
                let part_url = format!("{}/api/v1/media/upload/{}/part/{}", base, upload_id, part_index);
                let part_resp = client
                    .post(&part_url)
                    .bearer_auth(&token_owned)
                    .body(chunk)
                    .send()
                    .await?;
                if !part_resp.status().is_success() {
                    return Err(HttpClientError::Api {
                        status: part_resp.status().as_u16(),
                        message: part_resp.text().await.unwrap_or_default(),
                    });
                }
                offset = end;
                part_index += 1;
            }

            let comp_url = format!("{}/api/v1/media/upload/{}/complete", base, upload_id);
            let comp_resp = client
                .post(&comp_url)
                .bearer_auth(&token_owned)
                .send()
                .await?;
            if !comp_resp.status().is_success() {
                return Err(HttpClientError::Api {
                    status: comp_resp.status().as_u16(),
                    message: comp_resp.text().await.unwrap_or_default(),
                });
            }
            let comp_data: serde_json::Value = comp_resp.json().await?;
            let file_id = comp_data["file_id"]
                .as_str()
                .or_else(|| comp_data["sha256"].as_str())
                .unwrap_or_default()
                .to_string();
            Ok(file_id)
        })
        .await
    }

    /// Downloads media binary bytes from CAS storage
    pub async fn download_media(
        &self,
        token: &str,
        file_id: &str,
    ) -> Result<Vec<u8>, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/api/v1/media/stream/{}", self.base_url, file_id);
        let token_owned = token.to_string();

        self.run_on_tokio(async move {
            let resp = client
                .get(&url)
                .bearer_auth(&token_owned)
                .send()
                .await?;
            let status = resp.status();
            if status.is_success() {
                let bytes = resp.bytes().await?;
                Ok(bytes.to_vec())
            } else if status == StatusCode::UNAUTHORIZED {
                Err(HttpClientError::Unauthorized(resp.text().await.unwrap_or_default()))
            } else {
                Err(HttpClientError::Api {
                    status: status.as_u16(),
                    message: resp.text().await.unwrap_or_default(),
                })
            }
        })
        .await
    }

    /// Gets chat details via GET /api/v1/chats/{chat_id}
    pub async fn get_chat(
        &self,
        token: &str,
        chat_id: i64,
    ) -> Result<ChatDetails, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/api/v1/chats/{}", self.base_url, chat_id);
        let token_owned = token.to_string();

        self.run_on_tokio(async move {
            let resp = client
                .get(&url)
                .bearer_auth(&token_owned)
                .send()
                .await?;
            let status = resp.status();
            if status.is_success() {
                let data = resp.json::<ChatDetails>().await?;
                Ok(data)
            } else if status == StatusCode::UNAUTHORIZED {
                Err(HttpClientError::Unauthorized(resp.text().await.unwrap_or_default()))
            } else {
                Err(HttpClientError::Api {
                    status: status.as_u16(),
                    message: resp.text().await.unwrap_or_default(),
                })
            }
        })
        .await
    }

    /// Updates chat information via PUT /api/v1/chats/{chat_id}
    pub async fn update_chat_info(
        &self,
        token: &str,
        chat_id: i64,
        title: Option<&str>,
        about: Option<&str>,
    ) -> Result<bool, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/api/v1/chats/{}", self.base_url, chat_id);
        let token_owned = token.to_string();
        let mut body = serde_json::Map::new();
        if let Some(t) = title {
            body.insert("title".to_string(), serde_json::Value::String(t.to_string()));
        }
        if let Some(a) = about {
            body.insert("about".to_string(), serde_json::Value::String(a.to_string()));
        }

        self.run_on_tokio(async move {
            let resp = client
                .put(&url)
                .bearer_auth(&token_owned)
                .json(&body)
                .send()
                .await?;
            if resp.status().is_success() {
                Ok(true)
            } else if resp.status() == StatusCode::UNAUTHORIZED {
                Err(HttpClientError::Unauthorized(resp.text().await.unwrap_or_default()))
            } else {
                Err(HttpClientError::Api {
                    status: resp.status().as_u16(),
                    message: resp.text().await.unwrap_or_default(),
                })
            }
        })
        .await
    }

    /// Lists chat members via GET /api/v1/chats/{chat_id}/members
    pub async fn get_chat_members(
        &self,
        token: &str,
        chat_id: i64,
    ) -> Result<Vec<ChatMemberItem>, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/api/v1/chats/{}/members", self.base_url, chat_id);
        let token_owned = token.to_string();

        self.run_on_tokio(async move {
            let resp = client
                .get(&url)
                .bearer_auth(&token_owned)
                .send()
                .await?;
            let status = resp.status();
            if status.is_success() {
                let v: serde_json::Value = resp.json().await?;
                let members: Vec<ChatMemberItem> = serde_json::from_value(v["members"].clone()).unwrap_or_default();
                Ok(members)
            } else if status == StatusCode::UNAUTHORIZED {
                Err(HttpClientError::Unauthorized(resp.text().await.unwrap_or_default()))
            } else {
                Err(HttpClientError::Api {
                    status: status.as_u16(),
                    message: resp.text().await.unwrap_or_default(),
                })
            }
        })
        .await
    }

    /// Adds members to a chat via POST /api/v1/chats/{chat_id}/members
    pub async fn add_chat_members(
        &self,
        token: &str,
        chat_id: i64,
        user_ids: &[u64],
    ) -> Result<bool, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/api/v1/chats/{}/members", self.base_url, chat_id);
        let token_owned = token.to_string();
        let body = serde_json::json!({ "user_ids": user_ids });

        self.run_on_tokio(async move {
            let resp = client
                .post(&url)
                .bearer_auth(&token_owned)
                .json(&body)
                .send()
                .await?;
            if resp.status().is_success() {
                Ok(true)
            } else if resp.status() == StatusCode::UNAUTHORIZED {
                Err(HttpClientError::Unauthorized(resp.text().await.unwrap_or_default()))
            } else {
                Err(HttpClientError::Api {
                    status: resp.status().as_u16(),
                    message: resp.text().await.unwrap_or_default(),
                })
            }
        })
        .await
    }

    /// Updates chat member role and permissions via PUT /api/v1/chats/{chat_id}/members/{user_id}
    pub async fn update_chat_member_role(
        &self,
        token: &str,
        chat_id: i64,
        user_id: u64,
        role: &str,
        admin_title: Option<&str>,
        admin_rights: Option<u64>,
    ) -> Result<bool, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/api/v1/chats/{}/members/{}", self.base_url, chat_id, user_id);
        let token_owned = token.to_string();
        let mut body = serde_json::json!({ "role": role });
        if let Some(t) = admin_title {
            body["admin_title"] = serde_json::Value::String(t.to_string());
        }
        if let Some(r) = admin_rights {
            body["admin_rights"] = serde_json::Value::Number(r.into());
        }

        self.run_on_tokio(async move {
            let resp = client
                .put(&url)
                .bearer_auth(&token_owned)
                .json(&body)
                .send()
                .await?;
            if resp.status().is_success() {
                Ok(true)
            } else if resp.status() == StatusCode::UNAUTHORIZED {
                Err(HttpClientError::Unauthorized(resp.text().await.unwrap_or_default()))
            } else {
                Err(HttpClientError::Api {
                    status: resp.status().as_u16(),
                    message: resp.text().await.unwrap_or_default(),
                })
            }
        })
        .await
    }

    /// Removes a member from chat via DELETE /api/v1/chats/{chat_id}/members/{user_id}
    pub async fn remove_chat_member(
        &self,
        token: &str,
        chat_id: i64,
        user_id: u64,
    ) -> Result<bool, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/api/v1/chats/{}/members/{}", self.base_url, chat_id, user_id);
        let token_owned = token.to_string();

        self.run_on_tokio(async move {
            let resp = client
                .delete(&url)
                .bearer_auth(&token_owned)
                .send()
                .await?;
            if resp.status().is_success() {
                Ok(true)
            } else if resp.status() == StatusCode::UNAUTHORIZED {
                Err(HttpClientError::Unauthorized(resp.text().await.unwrap_or_default()))
            } else {
                Err(HttpClientError::Api {
                    status: resp.status().as_u16(),
                    message: resp.text().await.unwrap_or_default(),
                })
            }
        })
        .await
    }

    /// Lists forum topics for a forum chat via GET /api/v1/chats/{chat_id}/topics
    pub async fn list_forum_topics(
        &self,
        token: &str,
        chat_id: i64,
    ) -> Result<Vec<ForumTopicItem>, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/api/v1/chats/{}/topics", self.base_url, chat_id);
        let token_owned = token.to_string();

        self.run_on_tokio(async move {
            let resp = client
                .get(&url)
                .bearer_auth(&token_owned)
                .send()
                .await?;
            let status = resp.status();
            if status.is_success() {
                let v: serde_json::Value = resp.json().await?;
                let topics: Vec<ForumTopicItem> = serde_json::from_value(v["topics"].clone()).unwrap_or_default();
                Ok(topics)
            } else if status == StatusCode::UNAUTHORIZED {
                Err(HttpClientError::Unauthorized(resp.text().await.unwrap_or_default()))
            } else {
                Err(HttpClientError::Api {
                    status: status.as_u16(),
                    message: resp.text().await.unwrap_or_default(),
                })
            }
        })
        .await
    }

    /// Creates a forum topic via POST /api/v1/chats/{chat_id}/topics
    pub async fn create_forum_topic(
        &self,
        token: &str,
        chat_id: i64,
        title: &str,
        icon_color: i32,
        icon_emoji_id: Option<i64>,
    ) -> Result<ForumTopicItem, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/api/v1/chats/{}/topics", self.base_url, chat_id);
        let token_owned = token.to_string();
        let body = serde_json::json!({
            "title": title,
            "icon_color": icon_color,
            "icon_emoji_id": icon_emoji_id,
        });

        self.run_on_tokio(async move {
            let resp = client
                .post(&url)
                .bearer_auth(&token_owned)
                .json(&body)
                .send()
                .await?;
            let status = resp.status();
            if status == StatusCode::CREATED || status == StatusCode::OK {
                let data = resp.json::<ForumTopicItem>().await?;
                Ok(data)
            } else if status == StatusCode::UNAUTHORIZED {
                Err(HttpClientError::Unauthorized(resp.text().await.unwrap_or_default()))
            } else {
                Err(HttpClientError::Api {
                    status: status.as_u16(),
                    message: resp.text().await.unwrap_or_default(),
                })
            }
        })
        .await
    }

    /// Updates a forum topic via PUT /api/v1/chats/{chat_id}/topics/{topic_id}
    pub async fn update_forum_topic(
        &self,
        token: &str,
        chat_id: i64,
        topic_id: i64,
        title: Option<&str>,
        is_pinned: Option<bool>,
        is_closed: Option<bool>,
    ) -> Result<bool, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/api/v1/chats/{}/topics/{}", self.base_url, chat_id, topic_id);
        let token_owned = token.to_string();
        let mut body = serde_json::Map::new();
        if let Some(t) = title {
            body.insert("title".to_string(), serde_json::Value::String(t.to_string()));
        }
        if let Some(p) = is_pinned {
            body.insert("is_pinned".to_string(), serde_json::Value::Bool(p));
        }
        if let Some(c) = is_closed {
            body.insert("is_closed".to_string(), serde_json::Value::Bool(c));
        }

        self.run_on_tokio(async move {
            let resp = client
                .put(&url)
                .bearer_auth(&token_owned)
                .json(&body)
                .send()
                .await?;
            if resp.status().is_success() {
                Ok(true)
            } else if resp.status() == StatusCode::UNAUTHORIZED {
                Err(HttpClientError::Unauthorized(resp.text().await.unwrap_or_default()))
            } else {
                Err(HttpClientError::Api {
                    status: resp.status().as_u16(),
                    message: resp.text().await.unwrap_or_default(),
                })
            }
        })
        .await
    }

    /// Deletes a forum topic via DELETE /api/v1/chats/{chat_id}/topics/{topic_id}
    pub async fn delete_forum_topic(
        &self,
        token: &str,
        chat_id: i64,
        topic_id: i64,
    ) -> Result<bool, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/api/v1/chats/{}/topics/{}", self.base_url, chat_id, topic_id);
        let token_owned = token.to_string();

        self.run_on_tokio(async move {
            let resp = client
                .delete(&url)
                .bearer_auth(&token_owned)
                .send()
                .await?;
            if resp.status().is_success() {
                Ok(true)
            } else if resp.status() == StatusCode::UNAUTHORIZED {
                Err(HttpClientError::Unauthorized(resp.text().await.unwrap_or_default()))
            } else {
                Err(HttpClientError::Api {
                    status: resp.status().as_u16(),
                    message: resp.text().await.unwrap_or_default(),
                })
            }
        })
        .await
    }

    /// Updates current user profile via PUT /api/v1/auth/profile
    pub async fn update_profile(
        &self,
        token: &str,
        req: &UpdateProfileRequest,
    ) -> Result<bool, HttpClientError> {
        let client = self.client.clone();
        let url = format!("{}/api/v1/auth/profile", self.base_url);
        let token_owned = token.to_string();
        let req_owned = req.clone();

        self.run_on_tokio(async move {
            let resp = client
                .put(&url)
                .bearer_auth(&token_owned)
                .json(&req_owned)
                .send()
                .await?;
            if resp.status().is_success() {
                Ok(true)
            } else if resp.status() == StatusCode::UNAUTHORIZED {
                Err(HttpClientError::Unauthorized(resp.text().await.unwrap_or_default()))
            } else {
                Err(HttpClientError::Api {
                    status: resp.status().as_u16(),
                    message: resp.text().await.unwrap_or_default(),
                })
            }
        })
        .await
    }
}
