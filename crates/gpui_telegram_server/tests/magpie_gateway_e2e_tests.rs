use axum::{
    body::Body,
    extract::State as AxumState,
    http::{Method, Request, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use gpui_telegram_client::net::{
    HermesClient, HttpClient, MagpieGroupsRequest, MagpieImportRequest, MagpieMaskingConfigDto,
    MagpieMaskingPreviewRequest, MagpieProbeRequest, MagpieRoutingGroupDto,
    MagpieRoutingGroupMemberDto, MagpieSaveProviderRequest,
};
use gpui_telegram_server::{
    build_router_with_state, config::ServerConfig, server::AppState,
};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tempfile::TempDir;
use tower::ServiceExt;

fn build_temp_server_config(dir: &std::path::Path) -> ServerConfig {
    let db_path = dir.join("magpie_gateway_e2e.db");
    let media_dir = dir.join("media_cas");
    std::fs::create_dir_all(&media_dir).expect("create media dir");
    let mut cfg = ServerConfig::default();
    cfg.database_url = format!("sqlite://{}", db_path.display());
    cfg.media_storage_path = media_dir.to_string_lossy().to_string();
    cfg.host = "127.0.0.1".to_string();
    cfg.port = 0;
    cfg
}

fn boot_test_server_in(dir: &std::path::Path) -> (Arc<AppState>, Router) {
    let cfg = build_temp_server_config(dir);
    let state = Arc::new(AppState::try_new(cfg).expect("boot AppState"));
    let router = build_router_with_state(state.clone());
    (state, router)
}

fn boot_test_server() -> (TempDir, Arc<AppState>, Router) {
    let dir = tempfile::Builder::new()
        .prefix("magpie_e2e_")
        .tempdir_in(std::env::temp_dir())
        .expect("create tempdir");
    let (state, router) = boot_test_server_in(dir.path());
    (dir, state, router)
}

fn isolate_empty_providers(state: &AppState, empty_home: &std::path::Path) {
    let providers = state.hermes.providers();
    for p in providers.list_providers() {
        if p.id != "local_hermes" {
            providers.remove_provider(&p.id);
        }
    }
    providers.refresh_local_subscriptions_in_home(empty_home, Some(empty_home));
    providers.set_allow_local_fallback(false);
    state.hermes.persist_providers_state();
    state.fabushi_bots.sync_all_providers();
}

async fn http_raw(
    router: &Router,
    method: &str,
    uri: &str,
    bearer: Option<&str>,
    body: Option<Value>,
) -> (StatusCode, String, Vec<u8>) {
    let mut builder = Request::builder()
        .method(Method::from_bytes(method.as_bytes()).expect("valid method"))
        .uri(uri);
    if let Some(tok) = bearer {
        builder = builder.header("authorization", format!("Bearer {tok}"));
    }
    let req_body = match body {
        Some(val) => {
            builder = builder.header("content-type", "application/json");
            Body::from(val.to_string())
        }
        None => Body::empty(),
    };
    let req = builder.body(req_body).expect("build request");
    let resp = router.clone().oneshot(req).await.expect("oneshot request");
    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .expect("read body")
        .to_vec();
    (status, content_type, bytes)
}

async fn http_json(
    router: &Router,
    method: &str,
    uri: &str,
    bearer: Option<&str>,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let (status, _ct, bytes) = http_raw(router, method, uri, bearer, body).await;
    let val = serde_json::from_slice::<Value>(&bytes).unwrap_or_else(|_| {
        json!({ "_raw": String::from_utf8_lossy(&bytes).to_string() })
    });
    (status, val)
}

async fn register_user(router: &Router, username: &str) -> (u64, String) {
    let (st, val) = http_json(
        router,
        "POST",
        "/api/v1/auth/register",
        None,
        Some(json!({
            "username": username,
            "password": "StrongPassword#2026",
            "first_name": username,
            "device_name": "magpie_e2e_test",
            "auto_welcome": true
        })),
    )
    .await;
    assert_eq!(st, StatusCode::CREATED, "register {username}: {val}");
    let uid = val["user_id"].as_u64().expect("user_id");
    let tok = val["token"].as_str().expect("token").to_string();
    (uid, tok)
}

#[derive(Default)]
struct MockUpstreamState {
    captured_bodies: Mutex<Vec<Value>>,
    captured_paths: Mutex<Vec<String>>,
}

async fn spawn_mock_upstreams() -> (String, String, Arc<MockUpstreamState>) {
    let shared = Arc::new(MockUpstreamState::default());

    let s1 = shared.clone();
    let openai_router = Router::new()
        .route(
            "/v1/chat/completions",
            post({
                let st = s1.clone();
                move |Json(body): Json<Value>| {
                    let st = st.clone();
                    async move {
                        st.captured_paths
                            .lock()
                            .unwrap()
                            .push("/v1/chat/completions".to_string());
                        st.captured_bodies.lock().unwrap().push(body.clone());
                        let last_user = body["messages"]
                            .as_array()
                            .and_then(|arr| {
                                arr.iter()
                                    .rev()
                                    .find(|m| m["role"].as_str() == Some("user"))
                            })
                            .and_then(|m| m["content"].as_str())
                            .unwrap_or("empty")
                            .to_string();
                        let reply = if body["stream"].as_bool() == Some(true) {
                            let chunk = json!({
                                "id": "chatcmpl-mock",
                                "object": "chat.completion.chunk",
                                "model": body["model"].as_str().unwrap_or("deepseek-chat"),
                                "choices": [{
                                    "index": 0,
                                    "delta": { "content": format!("MOCK_OPENAI_REPLY: {last_user}") },
                                    "finish_reason": "stop"
                                }]
                            });
                            return (
                                StatusCode::OK,
                                [("content-type", "text/event-stream")],
                                format!("data: {chunk}\n\ndata: [DONE]\n\n"),
                            )
                                .into_response();
                        } else {
                            format!("MOCK_OPENAI_REPLY: {last_user}")
                        };
                        (
                            StatusCode::OK,
                            Json(json!({
                                "id": "chatcmpl-mock",
                                "object": "chat.completion",
                                "model": body["model"].as_str().unwrap_or("deepseek-chat"),
                                "choices": [{
                                    "index": 0,
                                    "message": { "role": "assistant", "content": reply },
                                    "finish_reason": "stop"
                                }],
                                "usage": { "prompt_tokens": 12, "completion_tokens": 6, "total_tokens": 18 }
                            })),
                        )
                            .into_response()
                    }
                }
            }),
        )
        .route(
            "/v1/models",
            get(|| async {
                (
                    StatusCode::OK,
                    Json(json!({ "object": "list", "data": [{ "id": "deepseek-chat" }] })),
                )
            }),
        );

    let openai_listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind mock openai");
    let openai_port = openai_listener.local_addr().unwrap().port();
    tokio::spawn(async move {
        let _ = axum::serve(openai_listener, openai_router).await;
    });

    let s2 = shared.clone();
    let gemini_router = Router::new().route(
        "/v1beta/models/{model_action}",
        post(|AxumState(st): AxumState<Arc<MockUpstreamState>>, axum::extract::Path(model_action): axum::extract::Path<String>, Json(body): Json<Value>| async move {
            st.captured_paths
                .lock()
                .unwrap()
                .push(format!("/v1beta/models/{model_action}"));
            st.captured_bodies.lock().unwrap().push(body);
            if model_action.contains("streamGenerateContent") {
                let evt = json!({
                    "candidates": [{
                        "content": { "role": "model", "parts": [{ "text": "MOCK_GEMINI_STREAM" }] },
                        "finishReason": "STOP"
                    }],
                    "usageMetadata": { "promptTokenCount": 5, "candidatesTokenCount": 3, "totalTokenCount": 8 }
                });
                (
                    StatusCode::OK,
                    [("content-type", "text/event-stream")],
                    format!("data: {evt}\n\n"),
                )
                    .into_response()
            } else {
                (
                    StatusCode::OK,
                    Json(json!({
                        "candidates": [{
                            "content": { "role": "model", "parts": [{ "text": "MOCK_GEMINI_OK" }] },
                            "finishReason": "STOP"
                        }],
                        "usageMetadata": { "promptTokenCount": 5, "candidatesTokenCount": 3, "totalTokenCount": 8 }
                    })),
                )
                    .into_response()
            }
        }),
    )
    .with_state(s2);

    let gemini_listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind mock gemini");
    let gemini_port = gemini_listener.local_addr().unwrap().port();
    tokio::spawn(async move {
        let _ = axum::serve(gemini_listener, gemini_router).await;
    });

    (
        format!("http://127.0.0.1:{openai_port}/v1"),
        format!("http://127.0.0.1:{gemini_port}/v1beta"),
        shared,
    )
}

#[tokio::test]
async fn test_e2e_magpie_state_presets_no_subscription_provider_error_and_bot_button() {
    let (dir, state, router) = boot_test_server();
    isolate_empty_providers(&state, dir.path());

    let (_uid, tok) = register_user(&router, "magpie_no_sub_user").await;

    // 1. Verify /api/v1/hermes/status and /api/v1/hermes/magpie/state report needs_provider_selection=true
    let (st_status, status_val) =
        http_json(&router, "GET", "/api/v1/hermes/status", Some(&tok), None).await;
    assert_eq!(st_status, StatusCode::OK, "hermes/status: {status_val}");
    assert_eq!(status_val["needs_provider_selection"].as_bool(), Some(true));
    assert_eq!(status_val["has_configured_provider"].as_bool(), Some(false));
    assert_eq!(status_val["has_active_subscription"].as_bool(), Some(false));

    let (st_magpie, magpie_val) =
        http_json(&router, "GET", "/api/v1/hermes/magpie/state", Some(&tok), None).await;
    assert_eq!(st_magpie, StatusCode::OK, "magpie/state: {magpie_val}");
    assert_eq!(magpie_val["needs_provider_selection"].as_bool(), Some(true));
    let presets = magpie_val["presets"].as_array().expect("presets array");
    assert!(
        presets.len() >= 40,
        "expected >= 40 Magpie presets, got {}",
        presets.len()
    );
    for pid in ["deepseek", "google", "anthropic", "openai", "ollama", "remote-magpie"] {
        assert!(
            presets.iter().any(|p| p["id"].as_str() == Some(pid)),
            "missing preset {pid}"
        );
    }

    // 2. Direct Hermes turn returns 503 provider_not_configured with guidance + open_ai_providers action
    let (st_turn, turn_val) = http_json(
        &router,
        "POST",
        "/api/v1/hermes/turns",
        Some(&tok),
        Some(json!({ "prompt": "Hello Hermes" })),
    )
    .await;
    assert_eq!(
        st_turn,
        StatusCode::SERVICE_UNAVAILABLE,
        "expected 503 when no provider/subscription configured: {turn_val}"
    );
    assert_eq!(turn_val["error"].as_str(), Some("provider_not_configured"));
    assert_eq!(turn_val["needs_provider_selection"].as_bool(), Some(true));
    assert_eq!(turn_val["action"].as_str(), Some("hermes:open_ai_providers"));

    // 3. Companion AI Bot reply appends inline button [⚙️ 选择 AI 供应商 (Magpie)] (hermes:open_ai_providers)
    let (st_bot, bot_val) = http_json(
        &router,
        "POST",
        "/api/v1/bots",
        Some(&tok),
        Some(json!({
            "display_name": "Magpie Helper",
            "username": "magpie_helper_bot",
            "description": "Test companion bot",
            "ai_persona": "Test bot"
        })),
    )
    .await;
    assert_eq!(st_bot, StatusCode::CREATED, "create bot: {bot_val}");
    let cid = bot_val["companion_chat_id"].as_i64().expect("companion_chat_id");

    let (st_send, send_val) = http_json(
        &router,
        "POST",
        &format!("/api/v1/chats/{cid}/messages"),
        Some(&tok),
        Some(json!({ "text": "Hi bot, explain quantum tunneling" })),
    )
    .await;
    assert!(st_send.is_success(), "send message to bot: {send_val}");

    let mut found_button = false;
    let mut found_error_msg = false;
    for _ in 0..40 {
        tokio::time::sleep(Duration::from_millis(50)).await;
        let (st_h, hist) = http_json(
            &router,
            "GET",
            &format!("/api/v1/chats/{cid}/messages?limit=20"),
            Some(&tok),
            None,
        )
        .await;
        assert_eq!(st_h, StatusCode::OK);
        if let Some(msgs) = hist["messages"].as_array() {
            for m in msgs {
                let text = m["text"].as_str().unwrap_or("");
                if text.contains("PROVIDER_ERROR") {
                    found_error_msg = true;
                }
                if let Some(kb) = m["inline_keyboard"].as_array() {
                    for row in kb {
                        if let Some(btns) = row.as_array() {
                            for b in btns {
                                if b["callback_data"].as_str() == Some("hermes:open_ai_providers") {
                                    assert!(b["text"].as_str().unwrap_or("").contains("选择 AI 供应商"));
                                    found_button = true;
                                }
                            }
                        }
                    }
                }
            }
        }
        if found_button && found_error_msg {
            break;
        }
    }
    assert!(found_error_msg, "expected PROVIDER_ERROR bot message in DM");
    assert!(
        found_button,
        "expected inline button `hermes:open_ai_providers` attached to PROVIDER_ERROR message"
    );

    // 4. Clicking callback returns structured toast
    let (st_cb, cb_val) = http_json(
        &router,
        "POST",
        &format!("/api/v1/chats/{cid}/callback"),
        Some(&tok),
        Some(json!({ "message_id": 1, "callback_data": "hermes:open_ai_providers" })),
    )
    .await;
    assert_eq!(st_cb, StatusCode::OK, "callback: {cb_val}");
    assert_eq!(cb_val["ok"].as_bool(), Some(true));

    // 5. Mini App ask_ai still safely falls back to Dharma pure-Rust reply when unconfigured
    let (st_ai, ai_val) = http_json(
        &router,
        "POST",
        "/api/v1/fabushi-miniapp/ai-chat",
        Some(&tok),
        Some(json!({ "message": "请问如何修持金刚经？" })),
    )
    .await;
    assert_eq!(st_ai, StatusCode::OK, "miniapp ai-chat fallback: {ai_val}");
    assert!(ai_val["reply"].as_str().unwrap_or("").contains("金刚经"));
}

#[tokio::test]
async fn test_e2e_magpie_preset_persistence_probe_delete_and_miniapp_ai_chat() {
    let dir = tempfile::Builder::new()
        .prefix("magpie_persist_e2e_")
        .tempdir_in(std::env::temp_dir())
        .expect("create tempdir");
    let (openai_url, gemini_url, mock_state) = spawn_mock_upstreams().await;

    let tok: String;
    {
        let (state, router) = boot_test_server_in(dir.path());
        isolate_empty_providers(&state, dir.path());
        let (_uid, t) = register_user(&router, "magpie_persist_user").await;
        tok = t;

        // 1. Apply DeepSeek preset with custom mock base_url
        let (st_save, save_val) = http_json(
            &router,
            "POST",
            "/api/v1/hermes/magpie/providers",
            Some(&tok),
            Some(json!({
                "preset_id": "deepseek",
                "api_key": "sk-deepseek-secret-token-9999",
                "base_url": openai_url,
                "default_model": "deepseek-chat",
                "activate": true,
                "allow_local_fallback": false
            })),
        )
        .await;
        assert_eq!(st_save, StatusCode::OK, "save deepseek preset: {save_val}");
        assert_eq!(save_val["provider"]["provider_id"].as_str(), Some("deepseek"));
        assert_eq!(save_val["provider"]["has_api_key"].as_bool(), Some(true));
        let serialized_resp = save_val.to_string();
        assert!(
            !serialized_resp.contains("sk-deepseek-secret-token-9999"),
            "raw API key must never be returned in HTTP JSON"
        );
        assert_eq!(
            save_val["state"]["needs_provider_selection"].as_bool(),
            Some(false)
        );
        assert_eq!(
            save_val["state"]["active_provider"].as_str(),
            Some("deepseek")
        );

        // 2. Probe DeepSeek provider live
        let (st_probe, probe_val) = http_json(
            &router,
            "POST",
            "/api/v1/hermes/magpie/providers/deepseek/probe",
            Some(&tok),
            None,
        )
        .await;
        assert_eq!(st_probe, StatusCode::OK, "probe deepseek: {probe_val}");
        assert_eq!(probe_val["ok"].as_bool(), Some(true), "probe ok: {probe_val}");
        assert!(probe_val["reply_preview"]
            .as_str()
            .unwrap_or("")
            .contains("MOCK_OPENAI_REPLY"));

        // 3. Apply Google Gemini preset and probe Rosetta Gemini wire execution
        let (st_gem, gem_val) = http_json(
            &router,
            "POST",
            "/api/v1/hermes/magpie/providers",
            Some(&tok),
            Some(json!({
                "preset_id": "google",
                "api_key": "AIzaGeminiTestKey1234",
                "base_url": gemini_url,
                "default_model": "gemini-2.5-flash",
                "activate": false
            })),
        )
        .await;
        assert_eq!(st_gem, StatusCode::OK, "save gemini: {gem_val}");

        let (st_gp, gp_val) = http_json(
            &router,
            "POST",
            "/api/v1/hermes/magpie/providers/google/probe",
            Some(&tok),
            None,
        )
        .await;
        assert_eq!(st_gp, StatusCode::OK, "probe gemini: {gp_val}");
        assert_eq!(gp_val["ok"].as_bool(), Some(true), "gemini probe ok: {gp_val}");
        assert!(gp_val["reply_preview"]
            .as_str()
            .unwrap_or("")
            .contains("MOCK_GEMINI"));
    }

    // 4. Restart server on same SQLite DB -> verify Magpie provider state survived!
    {
        let (_state2, router2) = boot_test_server_in(dir.path());
        let (st_state2, state2_val) =
            http_json(&router2, "GET", "/api/v1/hermes/magpie/state", Some(&tok), None).await;
        assert_eq!(st_state2, StatusCode::OK, "reloaded state: {state2_val}");
        assert_eq!(
            state2_val["active_provider"].as_str(),
            Some("deepseek"),
            "active_provider must persist across restart: {state2_val}"
        );
        assert_eq!(
            state2_val["needs_provider_selection"].as_bool(),
            Some(false)
        );
        assert!(state2_val["providers"]
            .as_array()
            .is_some_and(|arr| arr.iter().any(|p| p["provider_id"].as_str() == Some("google"))));

        // 5. Mini App ask_ai uses active Magpie/Hermes provider when configured
        let (st_ai, ai_val) = http_json(
            &router2,
            "POST",
            "/api/v1/fabushi-miniapp/ai-chat",
            Some(&tok),
            Some(json!({ "message": "What is compassion?" })),
        )
        .await;
        assert_eq!(st_ai, StatusCode::OK, "miniapp ai-chat live: {ai_val}");
        assert!(
            ai_val["reply"]
                .as_str()
                .unwrap_or("")
                .contains("MOCK_OPENAI_REPLY"),
            "expected live Hermes/Magpie reply in ai-chat: {ai_val}"
        );

        // 6. Delete provider google; local_hermes stays protected
        let (st_del_lh, _) = http_json(
            &router2,
            "DELETE",
            "/api/v1/hermes/magpie/providers/local_hermes",
            Some(&tok),
            None,
        )
        .await;
        assert_eq!(st_del_lh, StatusCode::BAD_REQUEST);

        let (st_del_g, del_g_val) = http_json(
            &router2,
            "DELETE",
            "/api/v1/hermes/magpie/providers/google",
            Some(&tok),
            None,
        )
        .await;
        assert_eq!(st_del_g, StatusCode::OK, "delete google: {del_g_val}");
        assert!(!del_g_val["state"]["providers"]
            .as_array()
            .unwrap()
            .iter()
            .any(|p| p["provider_id"].as_str() == Some("google")));
    }

    assert!(!mock_state.captured_paths.lock().unwrap().is_empty());
}

#[tokio::test]
async fn test_e2e_magpie_groups_masking_import_and_builtin_v1_gateway_protocols() {
    let (dir, state, router) = boot_test_server();
    isolate_empty_providers(&state, dir.path());
    let (openai_url, _gemini_url, mock_state) = spawn_mock_upstreams().await;
    let (_uid, tok) = register_user(&router, "magpie_gateway_user").await;

    // 1. Register deepseek preset + keyless ollama preset pointed at mock upstream
    let (st1, _) = http_json(
        &router,
        "POST",
        "/api/v1/hermes/magpie/providers",
        Some(&tok),
        Some(json!({
            "preset_id": "deepseek",
            "api_key": "sk-ds-gateway-1234",
            "base_url": openai_url,
            "default_model": "deepseek-chat",
            "activate": true,
            "allow_local_fallback": false
        })),
    )
    .await;
    assert_eq!(st1, StatusCode::OK);

    // 2. Masking preview + live masking enforcement
    let (st_prev, prev_val) = http_json(
        &router,
        "POST",
        "/api/v1/hermes/magpie/masking/preview",
        Some(&tok),
        Some(json!({
            "text": "My key is sk-proj-secret1234567890abcd and email is alice@acme.io"
        })),
    )
    .await;
    assert_eq!(st_prev, StatusCode::OK, "masking preview: {prev_val}");
    assert!(prev_val["hit_count"].as_u64().unwrap_or(0) >= 2);
    let preview_masked = prev_val["masked_text"].as_str().unwrap_or("");
    assert!(!preview_masked.contains("sk-proj-secret1234567890abcd"));
    assert!(!preview_masked.contains("alice@acme.io"));

    let (st_mask_put, _) = http_json(
        &router,
        "PUT",
        "/api/v1/hermes/magpie/masking",
        Some(&tok),
        Some(json!({
            "enabled": true,
            "mask_secrets": true,
            "mask_personal": true,
            "custom_words": ["TOPSECRET_TOKEN"],
            "allow_list": [],
            "default_action": "mask",
            "kind_actions": {}
        })),
    )
    .await;
    assert_eq!(st_mask_put, StatusCode::OK);

    // 3. Create Routing Group `fast-pool`
    let (st_grp, grp_val) = http_json(
        &router,
        "POST",
        "/api/v1/hermes/magpie/groups",
        Some(&tok),
        Some(json!({
            "routing_strategy": "smart",
            "group": {
                "id": "fast-pool",
                "name": "Fast Pool",
                "strategy": "rotate",
                "members": [{ "target": "deepseek/deepseek-chat", "weight": 1, "enabled": true }]
            }
        })),
    )
    .await;
    assert_eq!(st_grp, StatusCode::OK, "upsert group: {grp_val}");

    // 4. Gateway /api/v1/magpie/v1/models lists `group/fast-pool`
    let (st_mods, mods_val) =
        http_json(&router, "GET", "/api/v1/magpie/v1/models", None, None).await;
    assert_eq!(st_mods, StatusCode::OK);
    assert!(mods_val["data"]
        .as_array()
        .is_some_and(|arr| arr.iter().any(|m| m["id"].as_str() == Some("group/fast-pool"))));

    // 5. Gateway /api/v1/magpie/v1/chat/completions via group/fast-pool masks outbound & unmasks reply
    let (st_chat, chat_val) = http_json(
        &router,
        "POST",
        "/api/v1/magpie/v1/chat/completions",
        None,
        Some(json!({
            "model": "group/fast-pool",
            "messages": [{ "role": "user", "content": "Echo TOPSECRET_TOKEN and sk-livekey1234567890" }]
        })),
    )
    .await;
    assert_eq!(st_chat, StatusCode::OK, "gateway chat/completions: {chat_val}");
    let out_txt = chat_val["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("");
    assert!(
        out_txt.contains("TOPSECRET_TOKEN") && out_txt.contains("sk-livekey1234567890"),
        "expected unmasked values returned to caller: {out_txt}"
    );
    {
        let captured = mock_state.captured_bodies.lock().unwrap();
        let last_sent = captured.last().expect("mock captured request").to_string();
        assert!(
            !last_sent.contains("TOPSECRET_TOKEN") && !last_sent.contains("sk-livekey1234567890"),
            "raw secrets must NEVER reach upstream when masking enabled: {last_sent}"
        );
    }

    // 6. Gateway /api/v1/magpie/v1/responses + /messages (JSON + SSE streaming)
    let (st_resp, resp_val) = http_json(
        &router,
        "POST",
        "/api/v1/magpie/v1/responses",
        None,
        Some(json!({ "model": "deepseek-chat", "input": "Responses check" })),
    )
    .await;
    assert_eq!(st_resp, StatusCode::OK, "gateway responses: {resp_val}");
    assert!(resp_val["output"][0]["content"][0]["text"]
        .as_str()
        .unwrap_or("")
        .contains("MOCK_OPENAI_REPLY"));

    let (st_anth, anth_val) = http_json(
        &router,
        "POST",
        "/api/v1/magpie/v1/messages",
        None,
        Some(json!({
            "model": "deepseek-chat",
            "max_tokens": 64,
            "messages": [{ "role": "user", "content": "Anthropic check" }]
        })),
    )
    .await;
    assert_eq!(st_anth, StatusCode::OK, "gateway anthropic messages: {anth_val}");
    assert!(anth_val["content"][0]["text"]
        .as_str()
        .unwrap_or("")
        .contains("MOCK_OPENAI_REPLY"));

    let (st_sse, ct_sse, bytes_sse) = http_raw(
        &router,
        "POST",
        "/api/v1/magpie/v1/chat/completions",
        None,
        Some(json!({
            "model": "deepseek-chat",
            "stream": true,
            "messages": [{ "role": "user", "content": "Stream check" }]
        })),
    )
    .await;
    assert_eq!(st_sse, StatusCode::OK);
    assert!(ct_sse.contains("text/event-stream"));
    let sse_str = String::from_utf8_lossy(&bytes_sse);
    assert!(sse_str.contains("MOCK_OPENAI_REPLY") && sse_str.contains("[DONE]"));

    // 7. Import via magpie:// deep link and .env snippet
    let (st_imp1, imp1_val) = http_json(
        &router,
        "POST",
        "/api/v1/hermes/magpie/import",
        Some(&tok),
        Some(json!({
            "raw": "magpie://import?preset=zhipu&key=sk-zhipu-imported&model=glm-4.5",
            "activate": false
        })),
    )
    .await;
    assert_eq!(st_imp1, StatusCode::OK, "import deeplink: {imp1_val}");
    assert_eq!(imp1_val["imported_count"].as_u64(), Some(1));

    let (st_imp2, imp2_val) = http_json(
        &router,
        "POST",
        "/api/v1/hermes/magpie/import",
        Some(&tok),
        Some(json!({
            "raw": "MOONSHOT_API_KEY=sk-moonshot-env-import\nGROQ_API_KEY=gsk_groq_import\n",
            "activate": false
        })),
    )
    .await;
    assert_eq!(st_imp2, StatusCode::OK, "import dotenv: {imp2_val}");
    assert!(imp2_val["imported_count"].as_u64().unwrap_or(0) >= 2);

    // 8. Delete group cleanly
    let (st_del_grp, del_grp_val) = http_json(
        &router,
        "DELETE",
        "/api/v1/hermes/magpie/groups/fast-pool",
        Some(&tok),
        None,
    )
    .await;
    assert_eq!(st_del_grp, StatusCode::OK, "delete group: {del_grp_val}");
    assert!(del_grp_val["routing_groups"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn test_e2e_hermes_client_magpie_roundtrip_over_http() {
    let (dir, state, router) = boot_test_server();
    isolate_empty_providers(&state, dir.path());
    let (openai_url, _gemini_url, _mock_state) = spawn_mock_upstreams().await;
    let (_uid, tok) = register_user(&router, "magpie_client_rt").await;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind ephemeral port");
    let addr = listener.local_addr().unwrap();
    let base_url = format!("http://127.0.0.1:{}", addr.port());
    let handle = tokio::spawn(async move {
        let _ = axum::serve(listener, router).await;
    });

    let client = HermesClient::new(HttpClient::new(base_url));

    let initial_state = client.magpie_state(&tok).await.expect("magpie_state");
    assert!(initial_state.needs_provider_selection);
    assert!(initial_state.presets.len() >= 40);

    let saved = client
        .magpie_save_provider(
            &tok,
            &MagpieSaveProviderRequest {
                preset_id: Some("deepseek".to_string()),
                api_key: Some("sk-client-e2e".to_string()),
                base_url: Some(openai_url),
                default_model: Some("deepseek-chat".to_string()),
                activate: true,
                allow_local_fallback: Some(false),
                ..Default::default()
            },
        )
        .await
        .expect("magpie_save_provider");
    assert!(saved.ok);
    assert!(!saved.state.needs_provider_selection);

    let probe = client
        .magpie_probe_provider(
            &tok,
            "deepseek",
            Some(&MagpieProbeRequest {
                model: Some("deepseek-chat".to_string()),
                prompt: Some("Ping".to_string()),
            }),
        )
        .await
        .expect("magpie_probe_provider");
    assert!(probe.ok, "probe failed: {:?}", probe.error);
    assert!(probe
        .reply_preview
        .as_deref()
        .unwrap_or("")
        .contains("MOCK_OPENAI_REPLY"));

    let scan = client
        .magpie_scan_subscriptions(&tok)
        .await
        .expect("magpie_scan_subscriptions");
    assert!(scan.ok);

    let grp_res = client
        .magpie_upsert_group(
            &tok,
            &MagpieGroupsRequest {
                group: Some(MagpieRoutingGroupDto {
                    id: "client-pool".to_string(),
                    name: "Client Pool".to_string(),
                    strategy: "smart".to_string(),
                    members: vec![MagpieRoutingGroupMemberDto {
                        target: "deepseek/deepseek-chat".to_string(),
                        weight: 1,
                        enabled: true,
                        ..Default::default()
                    }],
                    ..Default::default()
                }),
                routing_strategy: Some("smart".to_string()),
            },
        )
        .await
        .expect("magpie_upsert_group");
    assert!(grp_res.ok);
    assert_eq!(grp_res.routing_groups.len(), 1);

    let mask_res = client
        .magpie_update_masking(
            &tok,
            &MagpieMaskingConfigDto {
                enabled: true,
                ..Default::default()
            },
        )
        .await
        .expect("magpie_update_masking");
    assert!(mask_res.ok);

    let prev = client
        .magpie_preview_masking(
            &tok,
            &MagpieMaskingPreviewRequest {
                text: "Secret sk-abcdef1234567890xyz".to_string(),
                config: None,
            },
        )
        .await
        .expect("magpie_preview_masking");
    assert!(prev.hit_count >= 1);

    let imp = client
        .magpie_import(
            &tok,
            &MagpieImportRequest {
                raw: "OPENROUTER_API_KEY=sk-or-client-123".to_string(),
                activate: false,
            },
        )
        .await
        .expect("magpie_import");
    assert_eq!(imp.imported_count, 1);

    let del_grp = client
        .magpie_delete_group(&tok, "client-pool")
        .await
        .expect("magpie_delete_group");
    assert!(del_grp.routing_groups.is_empty());

    let del_prov = client
        .magpie_delete_provider(&tok, "openrouter")
        .await
        .expect("magpie_delete_provider");
    assert!(del_prov.ok);

    handle.abort();
}
