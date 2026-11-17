//! Tier 1: Single Feature Baseline Coverage (115 Tests across 23 Features)
//!
//! Covers F1..F23 (5 tests per feature = 115 tests), wired directly against
//! genuine workspace crates, `EphemeralTestServer`, WebSocket gateway frames,
//! SQLite, Tantivy, WebRTC `WebRtcPeerTransport`, `gpui_telegram_media::capture`,
//! `VideoViewport`, `NotificationsManagerMacUnMac`, `HermesStateStore`, and fixed
//! M2/M4 contract harnesses.

mod common;

use common::*;
use ed25519_dalek::SigningKey;
use gpui::ObjectFit;
use gpui_telegram_agent::hermes::state_db::{
    AppendMessageRequest, CreateCronJobRequest, CreateKanbanCardRequest, CreateSessionRequest,
    HermesStateStore, KanbanCardPatch,
};
use gpui_telegram_agent::hermes::toolsets::ToolsetId;
use gpui_telegram_agent::talk::VadBargeInDetector;
use gpui_telegram_client::cache::local_store::LocalStore;
use gpui_telegram_core::action_bus::{
    ActionCategory, ActionContext, ActionRegistry, CliExporter, DeleteMessageAction,
    GetChatHistoryAction, HermesToolExporter, ListDialogsAction, McpExporter, OpenApiExporter,
    SendMessageAction,
};
use gpui_telegram_core::types::{ChatId, UserId};
use gpui_telegram_desktop::app::session::send_as_peers::{
    SendAsKey, SendAsPeer, SendAsPeersSession, SendAsType,
};
use gpui_telegram_desktop::components::video_viewport::VideoViewport;
use gpui_telegram_desktop::platform::mac::notifications_manager_mac_un::{
    NotificationsManagerMacUnMac, UNNotificationAction, UNNotificationParsed,
};
use gpui_telegram_desktop::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use gpui_telegram_desktop::views::app_shell::window_session_controller_link_info::{
    LinkInfoType, WindowSessionControllerLinkInfo,
};
use gpui_telegram_desktop::views::boxes::connection_box::{
    proxy_to_tg_link, ConnectionBox, ProxyConfig as DesktopProxyConfig, ProxyItemState,
    ProxyType as DesktopProxyType,
};
use gpui_telegram_desktop::views::call_overlay::CallOverlay;
use gpui_telegram_media::chunked_upload::ChunkedUploadSession;
use gpui_telegram_media::media::capture::{
    DeviceState, MediaDeviceSource, MockAudioSource, MockScreenSource, MockVideoSource,
};
use gpui_telegram_proto::cards::{InvoiceCard, PollCard, PollOption, PriceLine, TodoCard, TodoItem};
use gpui_telegram_proto::contracts::DraftDto;
use gpui_telegram_proto::miniapp_contracts::{
    MiniAppActionRequest, MiniAppManifestDto, MiniAppPermission, MiniAppUiSchema,
};
use gpui_telegram_proto::sync::UpdateEvent;
use gpui_telegram_proto::ws::{WsClientFrame, WsServerFrame};
use gpui_telegram_server::bus::{CrossNodeEventBus, EventPayload, InProcessEventBus, ServerEvent};
use gpui_telegram_server::db::{ServerDatabase, SqlitePool};
use gpui_telegram_server::push::dispatchers::MockPushDispatcher;
use gpui_telegram_server::push::{PushGatewayHub, PushNotification, PushPlatform};
use gpui_telegram_server::search::{SearchDocument, SearchQueryParams, TantivyIndexer};
use gpui_telegram_server::ws::GatewayHub;
use gpui_telegram_webrtc::calls::calls_video_bubble::BubbleCorner;
use gpui_telegram_webrtc::calls::peer_transport::{
    PeerConnectionState, PeerTransportConfig, TransportMode, VideoFrameData, WebRtcPeerTransport,
};
use gpui_telegram_webrtc::media::ai_media_tap::{AiMediaTap, DefaultAiMediaTap};
use gpui_telegram_webrtc::media::opus_codec::{
    OpusDepacketizer, OpusFramingCodec, OpusPacketizer, RtpPacket, OPUS_PAYLOAD_TYPE,
    OPUS_SAMPLES_PER_FRAME_MONO, OPUS_SAMPLES_PER_FRAME_STEREO,
};
use reqwest::Method;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::Duration;
use x25519_dalek::{PublicKey, StaticSecret};

fn build_full_registry() -> Arc<ActionRegistry> {
    let reg = ActionRegistry::new();
    reg.register_typed(SendMessageAction).unwrap();
    reg.register_typed(GetChatHistoryAction).unwrap();
    reg.register_typed(ListDialogsAction).unwrap();
    reg.register_typed(DeleteMessageAction).unwrap();
    Arc::new(reg)
}

fn secret_from_seed(seed: u8) -> StaticSecret {
    StaticSecret::from([seed; 32])
}

// ============================================================================
// F1: Action Bus, Quad-Export & Live HTTP Action Dispatch (5 tests)
// ============================================================================

#[tokio::test]
async fn test_f1_01_registry_and_send_message_action() {
    let registry = build_full_registry();
    assert_eq!(registry.count(), 4);

    let send_action = registry.get("send_message").expect("send_message registered");
    assert_eq!(send_action.category(), ActionCategory::Messaging);

    let ctx = ActionContext::new().with_user(42);
    let out = registry
        .execute(
            "send_message",
            ctx,
            json!({ "chat_id": 1001, "text": "ActionBus dispatch check" }),
        )
        .await
        .expect("send_message execution succeeds");
    assert_eq!(out["chat_id"], 1001);
    assert!(out["message_id"].as_i64().unwrap() > 0);
}

#[test]
fn test_f1_02_quad_export_openapi() {
    let registry = build_full_registry();
    let openapi = OpenApiExporter::export_spec(&registry, "E2E API", "1.0.0", "http://127.0.0.1");
    assert_eq!(openapi["openapi"], "3.1.0");
    let paths = openapi["paths"].as_object().expect("openapi paths object");
    assert!(paths.contains_key("/api/v1/actions/send_message"));
    assert!(paths.contains_key("/api/v1/actions/delete_message"));
    assert!(paths.contains_key("/api/v1/actions/get_chat_history"));
    assert!(paths.contains_key("/api/v1/actions/list_dialogs"));
}

#[test]
fn test_f1_03_quad_export_hermes_mcp_and_cli() {
    let registry = build_full_registry();

    let hermes_tools = HermesToolExporter::export_openai_tools(&registry);
    assert_eq!(hermes_tools.len(), 4);
    assert!(hermes_tools
        .iter()
        .any(|t| t["function"]["name"] == "send_message"));

    let mcp_tools = McpExporter::export_tools_list(&registry).tools;
    assert_eq!(mcp_tools.len(), 4);
    assert!(mcp_tools.iter().any(|t| t.name == "delete_message"));

    let cli_schema = CliExporter::export_cli_schema(&registry, "gpui-tg", "1.0.0");
    assert_eq!(cli_schema.program_name, "gpui-tg");
    assert_eq!(cli_schema.subcommands.len(), 4);
    assert!(cli_schema
        .subcommands
        .iter()
        .any(|c| c.kebab_name == "get-chat-history" || c.name == "get_chat_history"));
}

#[tokio::test]
async fn test_f1_04_mcp_protocol_tools_call_executes_registry() {
    let registry = build_full_registry();
    let mcp = MockMcpClient::new(registry);

    let init = mcp.initialize("gpui-e2e-harness");
    assert_eq!(
        init.result.as_ref().unwrap()["protocolVersion"],
        "2024-11-05"
    );

    let tools_resp = mcp.tools_list();
    let tools = tools_resp.result.unwrap()["tools"]
        .as_array()
        .unwrap()
        .clone();
    assert_eq!(tools.len(), 4);

    let ok_resp = mcp
        .tools_call(
            "send_message",
            json!({ "chat_id": 77, "text": "MCP registry execution" }),
        )
        .await;
    assert!(ok_resp.error.is_none());
    let text = ok_resp.result.unwrap()["content"][0]["text"]
        .as_str()
        .unwrap()
        .to_string();
    let parsed: Value = serde_json::from_str(&text).unwrap();
    assert_eq!(parsed["chat_id"], 77);

    // High/destructive action requires approval=true
    let denied = mcp
        .tools_call_with_approval(
            "delete_message",
            json!({ "chat_id": 77, "message_ids": [1], "for_everyone": true }),
            false,
        )
        .await;
    assert!(
        denied.error.is_some(),
        "delete_message without approval must fail"
    );

    let approved = mcp
        .tools_call_with_approval(
            "delete_message",
            json!({ "chat_id": 77, "message_ids": [1], "for_everyone": true }),
            true,
        )
        .await;
    assert!(approved.error.is_none());
    let app_txt = approved.result.unwrap()["content"][0]["text"]
        .as_str()
        .unwrap()
        .to_string();
    let app_val: Value = serde_json::from_str(&app_txt).unwrap();
    assert_eq!(app_val["deleted_count"], 1);
}

#[tokio::test]
async fn test_f1_05_server_http_action_bus_dispatch() {
    let (mut server, driver, (_alice_id, alice_token), (_bob_id, _bob_token), chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let (st, list_body) = driver
        .json_req(Method::GET, "/api/v1/actions", Some(&alice_token), None)
        .await
        .unwrap();
    assert_eq!(st, 200);
    let actions = list_body["actions"].as_array().expect("actions array");
    assert!(actions.iter().any(|a| a["name"] == "send_message"));

    let (st, schema_body) = driver
        .json_req(
            Method::GET,
            "/api/v1/actions/send_message/schema",
            Some(&alice_token),
            None,
        )
        .await
        .unwrap();
    assert_eq!(st, 200);
    assert!(schema_body.get("input_schema").is_some());

    let (st, exec_body) = driver
        .json_req(
            Method::POST,
            "/api/v1/actions/send_message",
            Some(&alice_token),
            Some(json!({
                "chat_id": chat_id,
                "text": "Live server action_bus send_message"
            })),
        )
        .await
        .unwrap();
    assert_eq!(st, 200, "action exec: {exec_body:?}");

    let (st, hist_body) = driver
        .json_req(
            Method::GET,
            &format!("/api/v1/chats/{chat_id}/messages"),
            Some(&alice_token),
            None,
        )
        .await
        .unwrap();
    assert_eq!(st, 200);
    let msgs = hist_body["messages"].as_array().expect("messages array");
    assert!(msgs
        .iter()
        .any(|m| m.to_string().contains("Live server action_bus send_message")));

    server.stop().await;
}

// ============================================================================
// F2: SQLite Pool, Transactions, WAL & Concurrent Readers (5 tests)
// ============================================================================

#[test]
fn test_f2_01_sqlite_pool_init_and_schema() {
    let tmp = new_test_dir("f2_01");
    let pool = SqlitePool::open(tmp.path().join("pool.db").to_str().unwrap()).expect("open pool");
    pool.with_writer(|conn| {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS e2e_items (id INTEGER PRIMARY KEY, val TEXT NOT NULL)",
            [],
        )
        .map_err(|e| gpui_telegram_core::error::TelegramError::Database(e.to_string()))?;
        Ok(())
    })
    .expect("create table");

    let srv_db =
        ServerDatabase::open(tmp.path().join("srv.db").to_str().unwrap()).expect("open srv db");
    let users_count = srv_db
        .pool()
        .with_reader(|conn| {
            let cnt: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='users'",
                    [],
                    |r| r.get(0),
                )
                .map_err(|e| gpui_telegram_core::error::TelegramError::Database(e.to_string()))?;
            Ok(cnt)
        })
        .expect("query users table");
    assert_eq!(users_count, 1);
}

#[test]
fn test_f2_02_sqlite_crud_roundtrip() {
    let tmp = new_test_dir("f2_02");
    let db_path = tmp.path().join("crud.db");
    let pool = SqlitePool::open(db_path.to_str().unwrap()).expect("open");

    pool.with_writer(|conn| {
        conn.execute_batch(
            "CREATE TABLE kv (k TEXT PRIMARY KEY, v TEXT NOT NULL);
             INSERT INTO kv (k, v) VALUES ('alpha', 'one'), ('beta', 'two');
             UPDATE kv SET v = 'ONE' WHERE k = 'alpha';",
        )
        .map_err(|e| gpui_telegram_core::error::TelegramError::Database(e.to_string()))?;
        Ok(())
    })
    .expect("insert and update");

    let val: String = pool
        .with_reader(|conn| {
            conn.query_row("SELECT v FROM kv WHERE k = 'alpha'", [], |r| r.get(0))
                .map_err(|e| gpui_telegram_core::error::TelegramError::Database(e.to_string()))
        })
        .expect("select updated");
    assert_eq!(val, "ONE");

    pool.with_writer(|conn| {
        conn.execute("DELETE FROM kv WHERE k = 'beta'", [])
            .map_err(|e| gpui_telegram_core::error::TelegramError::Database(e.to_string()))?;
        Ok(())
    })
    .expect("delete");

    let cnt: i64 = pool
        .with_reader(|conn| {
            conn.query_row("SELECT COUNT(*) FROM kv", [], |r| r.get(0))
                .map_err(|e| gpui_telegram_core::error::TelegramError::Database(e.to_string()))
        })
        .expect("count");
    assert_eq!(cnt, 1);
}

#[test]
fn test_f2_03_sqlite_wal_checkpoint_and_reopen() {
    let tmp = new_test_dir("f2_03");
    let db_path = tmp.path().join("wal.db");
    {
        let pool = SqlitePool::open(db_path.to_str().unwrap()).expect("open");
        let mode: String = pool
            .with_reader(|conn| {
                conn.query_row("PRAGMA journal_mode", [], |r| r.get(0))
                    .map_err(|e| gpui_telegram_core::error::TelegramError::Database(e.to_string()))
            })
            .expect("journal mode");
        assert_eq!(mode.to_ascii_lowercase(), "wal");

        pool.with_writer(|conn| {
            conn.execute("CREATE TABLE log (id INTEGER PRIMARY KEY, msg TEXT)", [])
                .map_err(|e| gpui_telegram_core::error::TelegramError::Database(e.to_string()))?;
            Ok(())
        })
        .expect("create table");

        pool.with_transaction(|tx| {
            tx.execute("INSERT INTO log (msg) VALUES ('committed')", [])
                .map_err(|e| gpui_telegram_core::error::TelegramError::Database(e.to_string()))?;
            Ok(())
        })
        .expect("tx commit");
    }

    let reopened = SqlitePool::open(db_path.to_str().unwrap()).expect("reopen");
    let msg: String = reopened
        .with_reader(|conn| {
            conn.query_row("SELECT msg FROM log WHERE id = 1", [], |r| r.get(0))
                .map_err(|e| gpui_telegram_core::error::TelegramError::Database(e.to_string()))
        })
        .expect("read reopened");
    assert_eq!(msg, "committed");
}

#[test]
fn test_f2_04_concurrent_readers_16_threads() {
    let tmp = new_test_dir("f2_04");
    let db_path = tmp.path().join("concurrent.db");
    let pool = Arc::new(SqlitePool::open(db_path.to_str().unwrap()).expect("open pool"));
    pool.with_writer(|conn| {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS shared_cfg (id INTEGER PRIMARY KEY, val TEXT NOT NULL)",
            [],
        )
        .map_err(|e| gpui_telegram_core::error::TelegramError::Database(e.to_string()))?;
        conn.execute(
            "INSERT INTO shared_cfg (id, val) VALUES (1, 'wal_reader_ok')",
            [],
        )
        .map_err(|e| gpui_telegram_core::error::TelegramError::Database(e.to_string()))?;
        Ok(())
    })
    .expect("seed shared_cfg");

    let handles: Vec<_> = (0..16)
        .map(|_| {
            let p = Arc::clone(&pool);
            std::thread::spawn(move || {
                p.with_reader(|conn| {
                    let s: String = conn
                        .query_row("SELECT val FROM shared_cfg WHERE id = 1", [], |r| r.get(0))
                        .map_err(|e| {
                            gpui_telegram_core::error::TelegramError::Database(e.to_string())
                        })?;
                    assert_eq!(s, "wal_reader_ok");
                    Ok(())
                })
            })
        })
        .collect();

    for h in handles {
        h.join()
            .expect("reader thread did not panic")
            .expect("with_reader query succeeded");
    }
}

#[tokio::test]
async fn test_f2_05_server_database_restart_persistence() {
    let (mut server, driver, (_alice_id, alice_token), (_bob_id, _bob_token), chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let (st, _) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/chats/{chat_id}/messages"),
            Some(&alice_token),
            Some(json!({ "text": "Survives server restart", "client_nonce": 205 })),
        )
        .await
        .unwrap();
    assert!((200..300).contains(&st));

    server.restart().await.expect("restart EphemeralTestServer");
    let driver_after = E2eSystemDriver::new(server.base_url());
    let (st, hist) = driver_after
        .json_req(
            Method::GET,
            &format!("/api/v1/chats/{chat_id}/messages"),
            Some(&alice_token),
            None,
        )
        .await
        .unwrap();
    assert_eq!(st, 200);
    assert!(hist["messages"]
        .as_array()
        .unwrap()
        .iter()
        .any(|m| m.to_string().contains("Survives server restart")));

    server.stop().await;
}

// ============================================================================
// F3: WebSocket GatewayHub, EventBus & Presence Broadcasts (5 tests)
// ============================================================================

#[test]
fn test_f3_01_gateway_hub_register_and_broadcast() {
    let hub = GatewayHub::new();
    let (tx, mut rx) = mpsc::unbounded_channel();
    hub.register(UserId(10), tx);
    assert_eq!(hub.user_connection_count(UserId(10)), 1);

    let pts = hub.next_pts();
    assert!(pts >= 1);

    hub.broadcast_to_user(UserId(10), WsServerFrame::Pong { nonce: 88 });
    let got = rx.try_recv().expect("frame received on registered channel");
    assert_eq!(got, WsServerFrame::Pong { nonce: 88 });
}

#[test]
fn test_f3_02_gateway_hub_chat_subscription() {
    let hub = GatewayHub::new();
    hub.subscribe_chat(500, UserId(1));
    hub.subscribe_chat(500, UserId(2));
    let subs = hub.get_chat_subscribers(500);
    assert_eq!(subs.len(), 2);

    hub.unsubscribe_chat(500, UserId(1));
    let subs_after = hub.get_chat_subscribers(500);
    assert_eq!(subs_after.len(), 1);
    assert!(subs_after.contains(&UserId(2)));
}

#[tokio::test]
async fn test_f3_03_inprocess_cross_node_event_bus() {
    let bus = InProcessEventBus::new("node-alpha");
    assert_eq!(bus.node_id(), "node-alpha");
    let mut sub = bus.subscribe("chat:*").await.expect("subscribe");

    let evt = ServerEvent::new(
        "chat:99",
        "node-alpha",
        EventPayload::NewMessage {
            chat_id: 99,
            message_id: 7,
            sender_id: 1,
            text: "Cross-node bus ping".into(),
            pts: 12,
        },
    );
    bus.publish("chat:99", &evt).await.expect("publish");

    let received = tokio::time::timeout(Duration::from_secs(2), sub.recv())
        .await
        .expect("timely recv")
        .expect("event payload");
    assert_eq!(received.event_id, evt.event_id);
    match received.payload {
        EventPayload::NewMessage { text, pts, .. } => {
            assert_eq!(text, "Cross-node bus ping");
            assert_eq!(pts, 12);
        }
        other => panic!("unexpected payload: {other:?}"),
    }
}

#[tokio::test]
async fn test_f3_04_ws_live_auth_and_ping_pong() {
    let (mut server, _driver, (_alice_id, alice_token), (_bob_id, _bob_token), _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let mut ws = connect_ws(&server, &alice_token)
        .await
        .expect("connect Alice WS");
    send_ws_frame(&mut ws, &WsClientFrame::Ping { nonce: 4242 })
        .await
        .expect("send Ping");
    let frame = recv_ws_frame_matching(&mut ws, Duration::from_secs(3), |f| {
        matches!(f, WsServerFrame::Pong { nonce: 4242 })
    })
    .await
    .expect("receive Pong");
    assert_eq!(frame, WsServerFrame::Pong { nonce: 4242 });

    let _ = ws.close(None).await;
    server.stop().await;
}

#[tokio::test]
async fn test_f3_05_presence_broadcast_via_hub_and_db() {
    let (mut server, _driver, (alice_id, _alice_token), (bob_id, _bob_token), _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let state = server.state().expect("server AppState").clone();
    let (tx_bob, mut rx_bob) = mpsc::unbounded_channel();
    state.gateway_hub.register(UserId(bob_id), tx_bob);

    state.gateway_hub.broadcast_user_status(UserId(alice_id), true);
    let online_frame = tokio::time::timeout(Duration::from_secs(2), rx_bob.recv())
        .await
        .expect("timeout waiting for Online UserStatusChanged")
        .expect("channel open");
    match online_frame {
        WsServerFrame::Update(UpdateEvent::UserStatusChanged { user_id, status }) => {
            assert_eq!(user_id, UserId(alice_id));
            assert!(matches!(
                status,
                gpui_telegram_core::types::UserStatus::Online { .. }
            ));
        }
        other => panic!("expected UserStatusChanged Online, got {other:?}"),
    }

    state.gateway_hub.broadcast_user_status(UserId(alice_id), false);
    let offline_frame = tokio::time::timeout(Duration::from_secs(2), rx_bob.recv())
        .await
        .expect("timeout waiting for Offline UserStatusChanged")
        .expect("channel open");
    match offline_frame {
        WsServerFrame::Update(UpdateEvent::UserStatusChanged { user_id, status }) => {
            assert_eq!(user_id, UserId(alice_id));
            assert!(matches!(
                status,
                gpui_telegram_core::types::UserStatus::Offline { .. }
            ));
        }
        other => panic!("expected UserStatusChanged Offline, got {other:?}"),
    }

    server.stop().await;
}

// ============================================================================
// F4: Multi-Platform Push Registration & Dispatch (5 tests)
// ============================================================================

#[tokio::test]
async fn test_f4_01_register_apns_push_via_server() {
    let (mut server, driver, (alice_id, alice_token), _bob, _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let (st, body) = driver
        .json_req(
            Method::POST,
            "/api/v1/devices/register_push",
            Some(&alice_token),
            Some(json!({
                "platform": "apns",
                "token": "apns_dev_token_e2e_01"
            })),
        )
        .await
        .unwrap();
    assert_eq!(st, 200, "register_push: {body:?}");

    let tokens = server
        .state()
        .unwrap()
        .db
        .get_user_push_tokens(UserId(alice_id))
        .expect("query tokens");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].platform, "apns");
    assert_eq!(tokens[0].token, "apns_dev_token_e2e_01");

    server.stop().await;
}

#[tokio::test]
async fn test_f4_02_register_fcm_and_webpush_multi_platform() {
    let (mut server, driver, (alice_id, alice_token), _bob, _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let (st1, _) = driver
        .json_req(
            Method::POST,
            "/api/v1/devices/register_push",
            Some(&alice_token),
            Some(json!({ "platform": "fcm", "token": "fcm_token_e2e_02" })),
        )
        .await
        .unwrap();
    assert_eq!(st1, 200);

    let (st2, _) = driver
        .json_req(
            Method::POST,
            "/api/v1/devices/register_push",
            Some(&alice_token),
            Some(json!({
                "platform": "webpush",
                "token": "webpush_endpoint_02",
                "endpoint": "https://push.example/sub/02",
                "p256dh": "BKeyExample",
                "auth_secret": "AuthExample"
            })),
        )
        .await
        .unwrap();
    assert_eq!(st2, 200);

    let tokens = server
        .state()
        .unwrap()
        .db
        .get_user_push_tokens(UserId(alice_id))
        .unwrap();
    assert_eq!(tokens.len(), 2);

    server.stop().await;
}

#[tokio::test]
async fn test_f4_03_register_push_idempotent_upsert() {
    let (mut server, driver, (alice_id, alice_token), _bob, _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    for _ in 0..2 {
        let (st, _) = driver
            .json_req(
                Method::POST,
                "/api/v1/devices/register_push",
                Some(&alice_token),
                Some(json!({ "platform": "mock", "token": "same_token_e2e_03" })),
            )
            .await
            .unwrap();
        assert_eq!(st, 200);
    }

    let tokens = server
        .state()
        .unwrap()
        .db
        .get_user_push_tokens(UserId(alice_id))
        .unwrap();
    assert_eq!(tokens.len(), 1, "re-registering same token must upsert");

    server.stop().await;
}

#[tokio::test]
async fn test_f4_04_unregister_push_token() {
    let (mut server, driver, (alice_id, alice_token), _bob, _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let (st, _) = driver
        .json_req(
            Method::POST,
            "/api/v1/devices/register_push",
            Some(&alice_token),
            Some(json!({ "platform": "mock", "token": "removable_token_04" })),
        )
        .await
        .unwrap();
    assert_eq!(st, 200);

    let (st_del, _) = driver
        .json_req(
            Method::DELETE,
            "/api/v1/devices/push",
            Some(&alice_token),
            Some(json!({ "token": "removable_token_04" })),
        )
        .await
        .unwrap();
    assert_eq!(st_del, 200);

    let tokens = server
        .state()
        .unwrap()
        .db
        .get_user_push_tokens(UserId(alice_id))
        .unwrap();
    assert!(tokens.is_empty());

    server.stop().await;
}

#[tokio::test]
async fn test_f4_05_push_gateway_hub_dispatches_to_mock_dispatcher() {
    let (mut server, driver, (alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let (st, _) = driver
        .json_req(
            Method::POST,
            "/api/v1/devices/register_push",
            Some(&alice_token),
            Some(json!({ "platform": "mock", "token": "mock_push_token_05" })),
        )
        .await
        .unwrap();
    assert_eq!(st, 200);

    let state = server.state().unwrap().clone();
    let mock_disp = Arc::new(MockPushDispatcher::new());
    let hub = PushGatewayHub::new(Arc::clone(&state.db));
    hub.register_dispatcher(mock_disp.clone());

    let notif = PushNotification {
        title: "New Message".into(),
        body: "Hello from Bob".into(),
        chat_id: Some(chat_id),
        message_id: Some(101),
        badge: Some(1),
        sound: Some("default".into()),
        custom_data: Default::default(),
    };
    let receipts = hub.send_to_user(UserId(alice_id), &notif).await;
    assert_eq!(receipts.len(), 1);
    let receipt = receipts[0].as_ref().expect("push dispatch ok");
    assert!(receipt.delivered);
    assert_eq!(receipt.platform, PushPlatform::Mock);
    assert_eq!(receipt.token, "mock_push_token_05");
    assert_eq!(mock_disp.dispatched_count(), 1);

    server.stop().await;
}

// ============================================================================
// F5: Chunked Media Upload & Server CAS Integrity (5 tests)
// ============================================================================

#[test]
fn test_f5_01_chunked_session_reassembly_and_sha256() {
    let payload: Vec<u8> = (0..10_000u32).map(|i| (i % 251) as u8).collect();
    let mut session = ChunkedUploadSession::new("up_01", payload.len() as u64, 4096).unwrap();
    assert_eq!(session.total_parts, 3);

    for (idx, chunk) in payload.chunks(4096).enumerate() {
        session.receive_part(idx as u32, chunk).expect("receive");
    }
    assert!(session.is_complete());
    let assembled = session.assemble_payload().expect("assemble");
    assert_eq!(assembled, payload);
    let digest = session.compute_sha256().expect("sha256");
    assert_eq!(digest.len(), 64);
}

#[test]
fn test_f5_02_chunked_out_of_order_and_duplicate_idempotency() {
    let payload = b"0123456789ABCDEF0123456789abcdef".to_vec();
    let mut session = ChunkedUploadSession::new("up_02", payload.len() as u64, 8).unwrap();
    assert_eq!(session.total_parts, 4);

    session.receive_part(3, &payload[24..32]).unwrap();
    session.receive_part(1, &payload[8..16]).unwrap();
    session.receive_part(1, &payload[8..16]).unwrap();
    assert_eq!(session.missing_parts(), vec![0, 2]);

    session.receive_part(0, &payload[0..8]).unwrap();
    session.receive_part(2, &payload[16..24]).unwrap();
    assert_eq!(session.assemble_payload().unwrap(), payload);
}

#[tokio::test]
async fn test_f5_03_server_http_chunked_upload_and_cas_stream() {
    let (mut server, driver, (_alice_id, alice_token), _bob, _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let png_bytes = create_test_png(12, 180, 240, 255);
    let chunk_size = 32usize;
    let (st, init_body) = driver
        .json_req(
            Method::POST,
            "/api/v1/media/upload/init",
            Some(&alice_token),
            Some(json!({
                "filename": "e2e_f5.png",
                "mime_type": "image/png",
                "size_bytes": png_bytes.len(),
                "chunk_size": chunk_size
            })),
        )
        .await
        .unwrap();
    assert_eq!(st, 201, "init: {init_body:?}");
    let upload_id = init_body["upload_id"].as_str().unwrap().to_string();

    for (idx, chunk) in png_bytes.chunks(chunk_size).enumerate() {
        let resp = driver
            .http
            .put(format!(
                "{}/api/v1/media/upload/{upload_id}/part/{idx}",
                server.base_url()
            ))
            .bearer_auth(&alice_token)
            .body(chunk.to_vec())
            .send()
            .await
            .expect("put chunk");
        assert_eq!(resp.status().as_u16(), 200);
    }

    let (st_done, done_body) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/media/upload/{upload_id}/complete"),
            Some(&alice_token),
            Some(json!({})),
        )
        .await
        .unwrap();
    assert_eq!(st_done, 200, "complete: {done_body:?}");
    let file_id = done_body["file_id"].as_str().unwrap();

    let downloaded = driver
        .http
        .get(format!("{}/api/v1/media/stream/{file_id}", server.base_url()))
        .bearer_auth(&alice_token)
        .send()
        .await
        .expect("stream GET")
        .bytes()
        .await
        .unwrap();
    assert_eq!(downloaded.as_ref(), png_bytes.as_slice());

    server.stop().await;
}

#[tokio::test]
async fn test_f5_04_server_upload_status_reports_missing_parts() {
    let (mut server, driver, (_alice_id, alice_token), _bob, _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let (st, init_body) = driver
        .json_req(
            Method::POST,
            "/api/v1/media/upload/init",
            Some(&alice_token),
            Some(json!({
                "filename": "parts.bin",
                "size_bytes": 100,
                "chunk_size": 40
            })),
        )
        .await
        .unwrap();
    assert_eq!(st, 201);
    let upload_id = init_body["upload_id"].as_str().unwrap();

    let resp = driver
        .http
        .put(format!(
            "{}/api/v1/media/upload/{upload_id}/part/0",
            server.base_url()
        ))
        .bearer_auth(&alice_token)
        .body(vec![7u8; 40])
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status().as_u16(), 200);

    let (st_status, status_body) = driver
        .json_req(
            Method::GET,
            &format!("/api/v1/media/upload/{upload_id}/status"),
            Some(&alice_token),
            None,
        )
        .await
        .unwrap();
    assert_eq!(st_status, 200);
    assert_eq!(status_body["is_complete"], false);
    assert_eq!(status_body["missing_parts"], json!([1, 2]));

    server.stop().await;
}

#[test]
fn test_f5_05_sha256_integrity_detects_single_byte_tamper() {
    let data = vec![0xAB; 512];
    let mut s1 = ChunkedUploadSession::new("orig", 512, 256).unwrap();
    s1.receive_part(0, &data[..256]).unwrap();
    s1.receive_part(1, &data[256..]).unwrap();
    let h1 = s1.compute_sha256().unwrap();

    let mut tampered = data.clone();
    tampered[300] ^= 0x01;
    let mut s2 = ChunkedUploadSession::new("tamp", 512, 256).unwrap();
    s2.receive_part(0, &tampered[..256]).unwrap();
    s2.receive_part(1, &tampered[256..]).unwrap();
    let h2 = s2.compute_sha256().unwrap();

    assert_ne!(h1, h2);
}

// ============================================================================
// F6: SQLite Outbox Queue, Backoff & Server Replay Dedup (5 tests)
// ============================================================================

#[test]
fn test_f6_01_outbox_sqlite_persistence_across_reopen() {
    let tmp = new_test_dir("f6_01");
    let db_path = tmp.path().join("outbox.db");
    let nonce = {
        let outbox = OutboxSimulator::open(&db_path).expect("open outbox");
        outbox.enqueue(10, "persisted offline msg").expect("enqueue")
    };
    let reopened = OutboxSimulator::open(&db_path).expect("reopen outbox");
    let rec = reopened.get_record(nonce).expect("record survives reopen");
    assert_eq!(rec.text, "persisted offline msg");
    assert_eq!(rec.status, OutboxStatus::Pending);
}

#[test]
fn test_f6_02_outbox_backoff_schedule_exponential_cap() {
    assert_eq!(OutboxSimulator::compute_backoff_delay_ms(0), 500);
    assert_eq!(OutboxSimulator::compute_backoff_delay_ms(1), 1000);
    assert_eq!(OutboxSimulator::compute_backoff_delay_ms(2), 2000);
    assert_eq!(OutboxSimulator::compute_backoff_delay_ms(10), 30_000);
}

#[test]
fn test_f6_03_outbox_transient_retry_and_permanent_fail() {
    let tmp = new_test_dir("f6_03");
    let outbox = OutboxSimulator::open(tmp.path().join("outbox_retry.db")).unwrap();
    let n1 = outbox.enqueue(10, "transient").unwrap();
    let n2 = outbox.enqueue(10, "permanent").unwrap();

    outbox.simulate_failure(n1, false);
    outbox.simulate_failure(n2, true);
    assert_eq!(outbox.get_record(n1).unwrap().status, OutboxStatus::Pending);
    assert_eq!(outbox.get_record(n2).unwrap().status, OutboxStatus::Failed);
}

#[tokio::test]
async fn test_f6_04_outbox_flush_replays_to_live_server() {
    let (mut server, driver, (_alice_id, alice_token), (_bob_id, bob_token), chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let tmp = new_test_dir("f6_04");
    let outbox = OutboxSimulator::open(tmp.path().join("outbox_live.db")).unwrap();
    let n1 = outbox.enqueue(chat_id, "Offline queued #1").unwrap();
    let n2 = outbox.enqueue(chat_id, "Offline queued #2").unwrap();

    let sent = outbox
        .notify_reconnected_to_server(&driver, &alice_token)
        .await
        .expect("flush to live server");
    assert_eq!(sent.len(), 2);
    assert_eq!(outbox.get_record(n1).unwrap().status, OutboxStatus::Sent);
    assert_eq!(outbox.get_record(n2).unwrap().status, OutboxStatus::Sent);

    let (st, hist) = driver
        .json_req(
            Method::GET,
            &format!("/api/v1/chats/{chat_id}/messages"),
            Some(&bob_token),
            None,
        )
        .await
        .unwrap();
    assert_eq!(st, 200);
    let body_str = hist.to_string();
    assert!(body_str.contains("Offline queued #1"));
    assert!(body_str.contains("Offline queued #2"));

    server.stop().await;
}

#[tokio::test]
async fn test_f6_05_server_client_nonce_idempotency() {
    let (mut server, driver, (_alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let payload = json!({ "text": "Idempotent nonce message", "client_nonce": 987654321u64 });
    let (st1, r1) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/chats/{chat_id}/messages"),
            Some(&alice_token),
            Some(payload.clone()),
        )
        .await
        .unwrap();
    assert!((200..300).contains(&st1));

    let (st2, r2) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/chats/{chat_id}/messages"),
            Some(&alice_token),
            Some(payload),
        )
        .await
        .unwrap();
    assert!((200..300).contains(&st2));
    assert_eq!(
        r1["message_id"], r2["message_id"],
        "duplicate client_nonce must return existing message_id"
    );

    server.stop().await;
}

// ============================================================================
// F7: Resumable Media Transfer via Live Server Chunks & Range/Stream (5 tests)
// ============================================================================

#[tokio::test]
async fn test_f7_01_server_resumable_upload_interrupt_and_resume() {
    let (mut server, driver, (_alice_id, alice_token), _bob, _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let data: Vec<u8> = (0..150u8).collect();
    let (st, init_body) = driver
        .json_req(
            Method::POST,
            "/api/v1/media/upload/init",
            Some(&alice_token),
            Some(json!({
                "filename": "resume.bin",
                "size_bytes": data.len(),
                "chunk_size": 50
            })),
        )
        .await
        .unwrap();
    assert_eq!(st, 201);
    let upload_id = init_body["upload_id"].as_str().unwrap().to_string();

    for idx in [0usize, 2usize] {
        let resp = driver
            .http
            .put(format!(
                "{}/api/v1/media/upload/{upload_id}/part/{idx}",
                server.base_url()
            ))
            .bearer_auth(&alice_token)
            .body(data[idx * 50..(idx + 1) * 50].to_vec())
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status().as_u16(), 200);
    }

    let (_, status) = driver
        .json_req(
            Method::GET,
            &format!("/api/v1/media/upload/{upload_id}/status"),
            Some(&alice_token),
            None,
        )
        .await
        .unwrap();
    assert_eq!(status["missing_parts"], json!([1]));

    let resp = driver
        .http
        .put(format!(
            "{}/api/v1/media/upload/{upload_id}/part/1",
            server.base_url()
        ))
        .bearer_auth(&alice_token)
        .body(data[50..100].to_vec())
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status().as_u16(), 200);

    let (st_comp, comp) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/media/upload/{upload_id}/complete"),
            Some(&alice_token),
            Some(json!({})),
        )
        .await
        .unwrap();
    assert_eq!(st_comp, 200);
    let file_id = comp["file_id"].as_str().unwrap();

    let got = driver
        .http
        .get(format!("{}/api/v1/media/stream/{file_id}", server.base_url()))
        .bearer_auth(&alice_token)
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    assert_eq!(got.as_ref(), data.as_slice());

    server.stop().await;
}

#[tokio::test]
async fn test_f7_02_server_incomplete_upload_complete_rejected() {
    let (mut server, driver, (_alice_id, alice_token), _bob, _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let (st, init_body) = driver
        .json_req(
            Method::POST,
            "/api/v1/media/upload/init",
            Some(&alice_token),
            Some(json!({
                "filename": "incomplete.bin",
                "size_bytes": 100,
                "chunk_size": 50
            })),
        )
        .await
        .unwrap();
    assert_eq!(st, 201);
    let upload_id = init_body["upload_id"].as_str().unwrap();

    let (st_comp, _) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/media/upload/{upload_id}/complete"),
            Some(&alice_token),
            Some(json!({})),
        )
        .await
        .unwrap();
    assert!((400..500).contains(&st_comp), "incomplete upload must be rejected: {st_comp}");

    server.stop().await;
}

#[tokio::test]
async fn test_f7_03_server_stream_unknown_file_returns_404() {
    let (mut server, driver, (_alice_id, alice_token), _bob, _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let resp = driver
        .http
        .get(format!(
            "{}/api/v1/media/stream/nonexistent_file_e2e",
            server.base_url()
        ))
        .bearer_auth(&alice_token)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status().as_u16(), 404);

    server.stop().await;
}

#[tokio::test]
async fn test_f7_04_server_upload_cross_user_isolation() {
    let (mut server, driver, (_alice_id, alice_token), (_bob_id, bob_token), _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let (_, init_body) = driver
        .json_req(
            Method::POST,
            "/api/v1/media/upload/init",
            Some(&alice_token),
            Some(json!({ "filename": "alice_secret.bin", "size_bytes": 20, "chunk_size": 20 })),
        )
        .await
        .unwrap();
    let upload_id = init_body["upload_id"].as_str().unwrap();

    let (st_bob, _) = driver
        .json_req(
            Method::GET,
            &format!("/api/v1/media/upload/{upload_id}/status"),
            Some(&bob_token),
            None,
        )
        .await
        .unwrap();
    assert!(
        st_bob == 403 || st_bob == 404,
        "other user must not inspect Alice's upload status: got {st_bob}"
    );

    server.stop().await;
}

#[tokio::test]
async fn test_f7_05_server_upload_persists_across_restart() {
    let (mut server, driver, (_alice_id, alice_token), _bob, _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let bytes = b"Restart resilient CAS media payload".to_vec();
    let (_, init) = driver
        .json_req(
            Method::POST,
            "/api/v1/media/upload/init",
            Some(&alice_token),
            Some(json!({ "filename": "cas.txt", "size_bytes": bytes.len(), "chunk_size": 64 })),
        )
        .await
        .unwrap();
    let upload_id = init["upload_id"].as_str().unwrap();

    driver
        .http
        .put(format!(
            "{}/api/v1/media/upload/{upload_id}/part/0",
            server.base_url()
        ))
        .bearer_auth(&alice_token)
        .body(bytes.clone())
        .send()
        .await
        .unwrap();

    let (_, comp) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/media/upload/{upload_id}/complete"),
            Some(&alice_token),
            Some(json!({})),
        )
        .await
        .unwrap();
    let file_id = comp["file_id"].as_str().unwrap().to_string();

    server.restart().await.expect("restart");
    let driver2 = E2eSystemDriver::new(server.base_url());
    let streamed = driver2
        .http
        .get(format!("{}/api/v1/media/stream/{file_id}", server.base_url()))
        .bearer_auth(&alice_token)
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    assert_eq!(streamed.as_ref(), bytes.as_slice());

    server.stop().await;
}

// ============================================================================
// F8: E2EE X3DH Key Exchange & Server Ciphertext Delivery (5 tests)
// ============================================================================

#[test]
fn test_f8_01_x3dh_4dh_shared_secret_agreement() {
    let e2ee = E2eeSimulator::new();
    let alice_id = secret_from_seed(1);
    let alice_pub = PublicKey::from(&alice_id);
    let bob_id = secret_from_seed(2);
    let bob_spk = secret_from_seed(3);
    let bob_opk = secret_from_seed(4);
    let eph = secret_from_seed(5);
    let eph_pub = PublicKey::from(&eph);

    e2ee.publish_prekeys(PrekeyBundle {
        user_id: 2,
        device_id: "bob_dev1".into(),
        identity_key: PublicKey::from(&bob_id),
        signed_prekey: PublicKey::from(&bob_spk),
        one_time_prekeys: vec![PublicKey::from(&bob_opk)],
    });

    let (bundle, consumed_opk) = e2ee.consume_prekey_bundle(2, "bob_dev1").unwrap();
    assert_eq!(consumed_opk, Some(PublicKey::from(&bob_opk)));

    let sk_alice = E2eeSimulator::derive_x3dh_initiator(
        &alice_id,
        &eph,
        &bundle.identity_key,
        &bundle.signed_prekey,
        consumed_opk.as_ref(),
    );
    let sk_bob = E2eeSimulator::derive_x3dh_responder(
        &bob_id,
        &bob_spk,
        Some(&bob_opk),
        &alice_pub,
        &eph_pub,
    );
    assert_eq!(sk_alice, sk_bob);
}

#[test]
fn test_f8_02_one_time_prekey_consumed_single_use() {
    let e2ee = E2eeSimulator::new();
    let bob_id = secret_from_seed(10);
    let bob_spk = secret_from_seed(11);
    let opk = secret_from_seed(12);
    e2ee.publish_prekeys(PrekeyBundle {
        user_id: 20,
        device_id: "d1".into(),
        identity_key: PublicKey::from(&bob_id),
        signed_prekey: PublicKey::from(&bob_spk),
        one_time_prekeys: vec![PublicKey::from(&opk)],
    });

    let (_, first_opk) = e2ee.consume_prekey_bundle(20, "d1").unwrap();
    assert!(first_opk.is_some());
    let (_, second_opk) = e2ee.consume_prekey_bundle(20, "d1").unwrap();
    assert!(second_opk.is_none(), "OTPK must only be dispensed once");
}

#[test]
fn test_f8_03_encrypt_decrypt_roundtrip_and_replay_protection() {
    let e2ee = E2eeSimulator::new();
    let alice_id = secret_from_seed(21);
    let alice_pub = PublicKey::from(&alice_id);
    let bob_id = secret_from_seed(22);
    let bob_spk = secret_from_seed(23);
    let eph = secret_from_seed(24);
    let eph_pub = PublicKey::from(&eph);

    let sk_a = E2eeSimulator::derive_x3dh_initiator(
        &alice_id,
        &eph,
        &PublicKey::from(&bob_id),
        &PublicKey::from(&bob_spk),
        None,
    );
    let sk_b =
        E2eeSimulator::derive_x3dh_responder(&bob_id, &bob_spk, None, &alice_pub, &eph_pub);

    let ct = E2eeSimulator::encrypt_message(&sk_a, 1, b"Zero-knowledge secret").unwrap();
    let pt = e2ee.decrypt_message("alice-1", &sk_b, 1, &ct).unwrap();
    assert_eq!(pt, b"Zero-knowledge secret");
    assert!(
        e2ee.decrypt_message("alice-1", &sk_b, 1, &ct).is_err(),
        "replay sequence must be rejected"
    );
}

#[tokio::test]
async fn test_f8_04_x3dh_ciphertext_transported_via_live_server() {
    let (mut server, driver, (_alice_id, alice_token), (bob_id, bob_token), chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let e2ee = E2eeSimulator::new();
    let alice_k = secret_from_seed(31);
    let bob_k = secret_from_seed(32);
    let bob_spk = secret_from_seed(33);
    let eph_k = secret_from_seed(34);
    e2ee.publish_prekeys(PrekeyBundle {
        user_id: bob_id,
        device_id: "bob_mac".into(),
        identity_key: PublicKey::from(&bob_k),
        signed_prekey: PublicKey::from(&bob_spk),
        one_time_prekeys: vec![],
    });

    let (bundle, _) = e2ee.consume_prekey_bundle(bob_id, "bob_mac").unwrap();
    let sk_alice = E2eeSimulator::derive_x3dh_initiator(
        &alice_k,
        &eph_k,
        &bundle.identity_key,
        &bundle.signed_prekey,
        None,
    );
    let ciphertext = E2eeSimulator::encrypt_message(&sk_alice, 0, b"E2EE over wire").unwrap();
    let encoded: String = ciphertext.iter().map(|b| format!("{b:02x}")).collect();

    let (st, _) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/chats/{chat_id}/messages"),
            Some(&alice_token),
            Some(json!({ "text": format!("E2EE:{encoded}"), "client_nonce": 804 })),
        )
        .await
        .unwrap();
    assert!((200..300).contains(&st));

    let (st_h, hist) = driver
        .json_req(
            Method::GET,
            &format!("/api/v1/chats/{chat_id}/messages"),
            Some(&bob_token),
            None,
        )
        .await
        .unwrap();
    assert_eq!(st_h, 200);
    let msg_text = hist["messages"][0]["text"].as_str().unwrap();
    let hex_ct = msg_text.strip_prefix("E2EE:").unwrap();
    assert!(!msg_text.contains("E2EE over wire"));
    let wire_bytes: Vec<u8> = (0..hex_ct.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex_ct[i..i + 2], 16).unwrap())
        .collect();

    let sk_bob = E2eeSimulator::derive_x3dh_responder(
        &bob_k,
        &bob_spk,
        None,
        &PublicKey::from(&alice_k),
        &PublicKey::from(&eph_k),
    );
    let decrypted = e2ee
        .decrypt_message("alice-wire", &sk_bob, 0, &wire_bytes)
        .unwrap();
    assert_eq!(decrypted, b"E2EE over wire");

    server.stop().await;
}

#[test]
fn test_f8_05_x3dh_ratchet_step_counter_rejects_mismatch() {
    let e2ee = E2eeSimulator::new();
    let k = [0x44u8; 32];
    let ct = E2eeSimulator::encrypt_message(&k, 5, b"step 5").unwrap();
    assert!(e2ee.decrypt_message("d", &k, 4, &ct).is_err());
    assert_eq!(e2ee.decrypt_message("d", &k, 5, &ct).unwrap(), b"step 5");
}

// ============================================================================
// F9: ConnectionBox, Proxy Deep-Links & Local TCP HTTP/SOCKS5 Proxies (5 tests)
// ============================================================================

#[test]
fn test_f9_01_connection_box_proxy_list_and_selection() {
    let mut cbox = ConnectionBox::new();
    let p1 = DesktopProxyConfig {
        id: 1,
        proxy_type: DesktopProxyType::Socks5,
        host: "127.0.0.1".into(),
        port: 1080,
        user: Some("alice".into()),
        password: Some("s3cr3t".into()),
        secret: None,
        ping_ms: Some(12),
        state: ProxyItemState::Online,
        is_enabled: false,
    };
    cbox.add_proxy(p1.clone());
    cbox.select_proxy(1);
    assert_eq!(cbox.selected_proxy_id, Some(1));
    assert!(cbox.proxies[0].is_enabled);

    let link = proxy_to_tg_link(&p1);
    assert!(link.starts_with("tg://socks?server=127.0.0.1&port=1080"));
    assert!(link.contains("user=alice"));
}

#[tokio::test]
async fn test_f9_02_http_connect_proxy_tunnels_to_ephemeral_server() {
    let mut server = boot_test_server().await.expect("boot_test_server");
    let proxy = LocalTestProxyServer::start_http(None)
        .await
        .expect("start http proxy");

    let cfg = ProxyConfig {
        proxy_type: ProxyType::Http,
        host: "127.0.0.1".into(),
        port: proxy.port(),
        username: None,
        password: None,
    };
    let target: std::net::SocketAddr = server
        .base_url()
        .trim_start_matches("http://")
        .parse()
        .expect("parse addr");

    let (status, body) = cfg
        .tunnel_http_get(target, "/api/v1/status")
        .await
        .expect("HTTP CONNECT tunnel to /api/v1/status");
    assert_eq!(status, 200);
    assert!(body.contains("ok") || body.contains("healthy"));

    proxy.stop().await;
    server.stop().await;
}

#[tokio::test]
async fn test_f9_03_socks5_proxy_with_auth_tunnels_http_post() {
    let (mut server, _driver, (_alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");
    let proxy = LocalTestProxyServer::start_socks5(Some(("e2e_user".into(), "e2e_pass".into())))
        .await
        .expect("start socks5 proxy");

    let cfg = ProxyConfig {
        proxy_type: ProxyType::Socks5,
        host: "127.0.0.1".into(),
        port: proxy.port(),
        username: Some("e2e_user".into()),
        password: Some("e2e_pass".into()),
    };
    let target: std::net::SocketAddr = server
        .base_url()
        .trim_start_matches("http://")
        .parse()
        .unwrap();

    let payload = br#"{"text":"Sent through SOCKS5 tunnel","client_nonce":903}"#;
    let (status, resp) = cfg
        .tunnel_http_request(
            target,
            "POST",
            &format!("/api/v1/chats/{chat_id}/messages"),
            Some(&alice_token),
            Some(payload),
            Some("application/json"),
        )
        .await
        .expect("SOCKS5 tunnel POST");
    assert!((200..300).contains(&status), "resp: {resp}");
    assert!(resp.contains("Sent through SOCKS5 tunnel"));

    proxy.stop().await;
    server.stop().await;
}

#[test]
fn test_f9_04_proxy_config_validation_rules() {
    let valid_http = ProxyConfig {
        proxy_type: ProxyType::Http,
        host: "proxy.example.org".into(),
        port: 8080,
        username: None,
        password: None,
    };
    assert!(valid_http.validate().is_ok());

    let empty_host = ProxyConfig {
        host: "  ".into(),
        ..valid_http
    };
    assert!(empty_host.validate().is_err());

    let zero_port = ProxyConfig {
        proxy_type: ProxyType::Socks5,
        host: "127.0.0.1".into(),
        port: 0,
        username: None,
        password: None,
    };
    assert!(zero_port.validate().is_err());
}

#[tokio::test]
async fn test_f9_05_direct_connection_reaches_server_status() {
    let mut server = boot_test_server().await.expect("boot_test_server");
    let driver = E2eSystemDriver::new(server.base_url());
    let (st, body) = driver
        .json_req(Method::GET, "/api/v1/status", None, None)
        .await
        .unwrap();
    assert_eq!(st, 200);
    assert!(body["status"] == "healthy" || body["status"] == "ok");
    server.stop().await;
}

// ============================================================================
// F10: Send-As Peers, Folders, Drafts (LocalStore+Server) & Pins (5 tests)
// ============================================================================

#[test]
fn test_f10_01_send_as_peers_session_selection() {
    let mut sess = SendAsPeersSession::new(100);
    let key = SendAsKey::message(900);
    sess.set_list(
        key,
        vec![
            SendAsPeer::new(100, "Personal".into(), false),
            SendAsPeer::new(999, "Channel Identity".into(), false),
        ],
    );
    sess.set_chosen(900, 999);
    assert_eq!(sess.chosen(900), Some(999));
    assert_eq!(sess.resolve_chosen(&key), 999);
    assert_eq!(
        sess.list(&SendAsKey::new(900, SendAsType::Message)).len(),
        2
    );
}

#[test]
fn test_f10_02_send_as_peers_fallback_to_user() {
    let sess = SendAsPeersSession::new(777);
    assert_eq!(sess.resolve_chosen(&SendAsKey::message(12345)), 777);
}

#[tokio::test]
async fn test_f10_03_server_folders_crud_and_chat_assignment() {
    let (mut server, driver, (_alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let (st, created) = driver
        .json_req(
            Method::POST,
            "/api/v1/folders",
            Some(&alice_token),
            Some(json!({ "title": "Work", "included_chat_ids": [chat_id] })),
        )
        .await
        .unwrap();
    assert!((200..300).contains(&st), "create folder: {created:?}");

    let (st_list, list) = driver
        .json_req(Method::GET, "/api/v1/folders", Some(&alice_token), None)
        .await
        .unwrap();
    assert_eq!(st_list, 200);
    assert!(list["folders"]
        .as_array()
        .unwrap()
        .iter()
        .any(|f| f["title"] == "Work"));

    server.stop().await;
}

#[tokio::test]
async fn test_f10_04_draft_persistence_local_store_and_server() {
    let (mut server, driver, (_alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");
    let tmp = new_test_dir("f10_04");
    let store = LocalStore::open_with_dir(tmp.path()).expect("open LocalStore");

    let draft = DraftDto {
        chat_id,
        text: "Unsent local + server draft".into(),
        entities: vec![],
        reply_to_msg_id: None,
        topic_id: None,
        date_ms: 1_700_000_000_000,
    };
    store.set_draft(ChatId(chat_id), Some(draft));
    assert_eq!(
        store.get_draft(ChatId(chat_id)).unwrap().text,
        "Unsent local + server draft"
    );

    let (st_put, _) = driver
        .json_req(
            Method::PUT,
            &format!("/api/v1/chats/{chat_id}/draft"),
            Some(&alice_token),
            Some(json!({ "text": "Unsent local + server draft" })),
        )
        .await
        .unwrap();
    assert_eq!(st_put, 200);

    let (st_get, body) = driver
        .json_req(
            Method::GET,
            &format!("/api/v1/chats/{chat_id}/draft"),
            Some(&alice_token),
            None,
        )
        .await
        .unwrap();
    assert_eq!(st_get, 200);
    assert_eq!(body["draft"]["text"], "Unsent local + server draft");

    let (st_del, _) = driver
        .json_req(
            Method::DELETE,
            &format!("/api/v1/chats/{chat_id}/draft"),
            Some(&alice_token),
            None,
        )
        .await
        .unwrap();
    assert_eq!(st_del, 200);
    store.set_draft(ChatId(chat_id), None);
    assert!(store.get_draft(ChatId(chat_id)).is_none());

    server.stop().await;
}

#[tokio::test]
async fn test_f10_05_server_pin_and_unpin_message() {
    let (mut server, driver, (_alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let (_, msg_resp) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/chats/{chat_id}/messages"),
            Some(&alice_token),
            Some(json!({ "text": "Pin me please", "client_nonce": 1005 })),
        )
        .await
        .unwrap();
    let msg_id = msg_resp["message_id"]
        .as_i64()
        .or_else(|| msg_resp["message"]["message_id"].as_i64())
        .unwrap();

    let (st_pin, _) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/chats/{chat_id}/pins"),
            Some(&alice_token),
            Some(json!({ "message_id": msg_id })),
        )
        .await
        .unwrap();
    assert_eq!(st_pin, 200);

    let (st_list, pins) = driver
        .json_req(
            Method::GET,
            &format!("/api/v1/chats/{chat_id}/pins"),
            Some(&alice_token),
            None,
        )
        .await
        .unwrap();
    assert_eq!(st_list, 200);
    assert!(pins["pinned_message_ids"]
        .as_array()
        .unwrap()
        .contains(&json!(msg_id)));

    let (st_unpin, _) = driver
        .json_req(
            Method::DELETE,
            &format!("/api/v1/chats/{chat_id}/pins/{msg_id}"),
            Some(&alice_token),
            None,
        )
        .await
        .unwrap();
    assert_eq!(st_unpin, 200);

    server.stop().await;
}

// ============================================================================
// F11: Genuine WebRTC PeerTransport (Loopback + Live SDP Offer/Answer) (5 tests)
// ============================================================================

#[tokio::test]
async fn test_f11_01_webrtc_loopback_pair_connected_state() {
    let (a, b) = WebRtcPeerTransport::create_loopback_pair(PeerTransportConfig::default());
    assert_eq!(a.state(), PeerConnectionState::Connected);
    assert_eq!(b.state(), PeerConnectionState::Connected);
}

#[tokio::test]
async fn test_f11_02_webrtc_loopback_audio_pcm_delivery() {
    let (a, b) = WebRtcPeerTransport::create_loopback_pair(PeerTransportConfig::default());
    let pcm: Vec<i16> = (0..OPUS_SAMPLES_PER_FRAME_MONO)
        .map(|i| ((i * 37) % 5000) as i16)
        .collect();
    a.send_audio_pcm(&pcm).await.expect("send audio pcm");
    let recv = b.recv_loopback_audio().await.expect("recv audio");
    assert_eq!(recv, pcm);
}

#[tokio::test]
async fn test_f11_03_webrtc_loopback_video_frame_delivery() {
    let (a, b) = WebRtcPeerTransport::create_loopback_pair(PeerTransportConfig::default());
    let frame = VideoFrameData {
        width: 320,
        height: 180,
        timestamp_ms: 1234,
        data: vec![0x7F; 320 * 180 * 4],
    };
    a.send_video_frame(&frame).await.expect("send video");
    let recv = b.recv_loopback_video().await.expect("recv video");
    assert_eq!(recv.width, 320);
    assert_eq!(recv.height, 180);
    assert_eq!(recv.timestamp_ms, 1234);
    assert_eq!(recv.data.len(), 320 * 180 * 4);
}

#[tokio::test]
async fn test_f11_04_webrtc_live_mode_sdp_offer_answer_handshake() {
    let cfg = PeerTransportConfig {
        mode: TransportMode::Live,
        ..Default::default()
    };
    let mut caller = WebRtcPeerTransport::new(cfg.clone())
        .await
        .expect("init caller RTCPeerConnection");
    let mut callee = WebRtcPeerTransport::new(cfg)
        .await
        .expect("init callee RTCPeerConnection");

    let offer = caller.create_offer().await.expect("create SDP offer");
    assert!(offer.contains("v=0"));
    assert!(offer.contains("m=audio") || offer.contains("opus"));

    callee
        .set_remote_description(&offer, true)
        .await
        .expect("callee set remote offer");
    let answer = callee.create_answer().await.expect("create SDP answer");
    assert!(answer.contains("v=0"));
    caller
        .set_remote_description(&answer, false)
        .await
        .expect("caller set remote answer");

    caller.close().await.unwrap();
    callee.close().await.unwrap();
}

#[tokio::test]
async fn test_f11_05_webrtc_hangup_transitions_to_closed() {
    let (mut a, mut b) =
        WebRtcPeerTransport::create_loopback_pair(PeerTransportConfig::default());
    a.close().await.expect("close a");
    b.close().await.expect("close b");
    assert_eq!(a.state(), PeerConnectionState::Closed);
    assert_eq!(b.state(), PeerConnectionState::Closed);
}

// ============================================================================
// F12: Genuine Opus Codec Framing & RTP Packetizer/Depacketizer (5 tests)
// ============================================================================

#[test]
fn test_f12_01_opus_mono_encode_decode_toc_0xa0() {
    let pcm: Vec<i16> = (0..OPUS_SAMPLES_PER_FRAME_MONO)
        .map(|i| (i as i16).wrapping_mul(11))
        .collect();
    let encoded = OpusFramingCodec::encode_frame(&pcm, 1).expect("encode mono");
    assert_eq!(encoded[0], 0xA0, "mono TOC byte must be 0xA0");
    let decoded = OpusFramingCodec::decode_frame(&encoded, 1).expect("decode mono");
    assert_eq!(decoded, pcm);
}

#[test]
fn test_f12_02_opus_stereo_encode_decode_toc_0xa4() {
    let pcm = vec![420i16; OPUS_SAMPLES_PER_FRAME_STEREO];
    let encoded = OpusFramingCodec::encode_frame(&pcm, 2).expect("encode stereo");
    assert_eq!(encoded[0], 0xA4, "stereo TOC byte must be 0xA4");
    let decoded = OpusFramingCodec::decode_frame(&encoded, 2).expect("decode stereo");
    assert_eq!(decoded, pcm);
}

#[test]
fn test_f12_03_rtp_packet_serialize_parse_roundtrip() {
    let pkt = RtpPacket::new(
        OPUS_PAYLOAD_TYPE,
        501,
        48_000,
        0xCAFE_BABE,
        vec![1, 2, 3, 4],
    );
    let wire = pkt.serialize();
    let parsed = RtpPacket::parse(&wire).expect("parse RTP");
    assert_eq!(parsed.payload_type, OPUS_PAYLOAD_TYPE);
    assert_eq!(parsed.sequence_number, 501);
    assert_eq!(parsed.timestamp, 48_000);
    assert_eq!(parsed.ssrc, 0xCAFE_BABE);
    assert_eq!(parsed.payload, vec![1, 2, 3, 4]);
}

#[test]
fn test_f12_04_opus_packetizer_depacketizer_streaming() {
    let mut pktizer = OpusPacketizer::new(1, 0x1122_3344);
    let mut depkt = OpusDepacketizer::new(1);
    let pcm: Vec<i16> = vec![900i16; OPUS_SAMPLES_PER_FRAME_MONO * 2];

    let packets = pktizer.push_pcm(&pcm);
    assert_eq!(packets.len(), 2);
    assert_eq!(packets[1].sequence_number, packets[0].sequence_number + 1);

    let out0 = depkt.depacketize(&packets[0]).expect("depkt 0");
    let out1 = depkt.depacketize(&packets[1]).expect("depkt 1");
    assert_eq!(out0.len(), OPUS_SAMPLES_PER_FRAME_MONO);
    assert_eq!(out1.len(), OPUS_SAMPLES_PER_FRAME_MONO);
    assert_eq!(
        depkt.last_sequence_number(),
        Some(packets[1].sequence_number)
    );
}

#[test]
fn test_f12_05_opus_rejects_wrong_frame_length() {
    let bad_pcm = vec![0i16; 123];
    assert!(OpusFramingCodec::encode_frame(&bad_pcm, 1).is_err());
    assert!(OpusFramingCodec::encode_frame(&bad_pcm, 3).is_err());
}

// ============================================================================
// F13: Genuine Media Capture Sources & Device Lifecycle (5 tests)
// ============================================================================

#[test]
fn test_f13_01_mock_audio_sine_vs_silence_rms() {
    let mut sine = MockAudioSource::new_sine(440.0, 48_000, 1);
    let sine_frame = sine.next_frame(20);
    assert_eq!(sine_frame.samples.len(), 960);
    assert!(sine_frame.rms_level() > 100);

    let mut silence = MockAudioSource::new_silence(48_000, 1);
    let sil_frame = silence.next_frame(20);
    assert_eq!(sil_frame.rms_level(), 0);
}

#[test]
fn test_f13_02_mock_video_rgba8_and_i420_conversion() {
    let mut cam = MockVideoSource::new(64, 36, 30);
    let vf = cam.next_frame();
    let rgba = vf.to_rgba8();
    assert_eq!(rgba.len(), 64 * 36 * 4);
    let i420 = vf.to_i420();
    assert_eq!(i420.len(), 64 * 36 * 3 / 2);
}

#[test]
fn test_f13_03_mock_screen_capture_with_cursor_pos() {
    let mut screen = MockScreenSource::new(128, 72, 15);
    let sf = screen.next_frame();
    assert_eq!(sf.video_frame.width, 128);
    assert_eq!(sf.video_frame.height, 72);
    assert!(sf.cursor_pos.is_some());
}

#[test]
fn test_f13_04_media_device_source_lifecycle_transitions() {
    let mut dev = MockVideoSource::new(64, 36, 30);
    assert_eq!(dev.state(), DeviceState::Uninitialized);

    dev.start().expect("start transitions to Streaming");
    assert_eq!(dev.state(), DeviceState::Streaming);

    dev.pause(true).expect("pause");
    assert_eq!(dev.state(), DeviceState::Paused);

    dev.pause(false).expect("resume");
    assert_eq!(dev.state(), DeviceState::Streaming);

    dev.stop().expect("stop");
    assert_eq!(dev.state(), DeviceState::Stopped);
}

#[test]
fn test_f13_05_media_device_invalid_pause_before_start() {
    let mut dev = MockAudioSource::new_sine(440.0, 48_000, 1);
    assert!(
        dev.pause(true).is_err(),
        "pausing an Uninitialized device must fail"
    );
}

// ============================================================================
// F14: GPUI VideoViewport & CallOverlay Controls (5 tests)
// ============================================================================

#[test]
fn test_f14_01_video_viewport_ingests_raw_remote_rgba() {
    let palette = TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x40A7E3, 14, 12);
    let mut vp = VideoViewport::new("Bob", palette);
    assert!(!vp.has_active_video());

    let ok = vp.ingest_raw_remote_rgba(4, 4, &vec![200u8; 4 * 4 * 4]);
    assert!(ok);
    assert!(vp.has_active_video());
    assert_eq!(vp.remote_resolution, (4, 4));
}

#[test]
fn test_f14_02_video_viewport_clear_remote_frame() {
    let palette = TdesktopThemePalette::build(TdesktopThemeMode::Day, 0x40A7E3, 14, 12);
    let mut vp = VideoViewport::new("Alice", palette);
    assert!(vp.ingest_raw_remote_rgba(2, 2, &vec![128u8; 16]));
    vp.set_remote_frame(None, 0, 0);
    assert!(!vp.has_active_video());
    assert_eq!(vp.remote_resolution, (0, 0));
}

#[test]
fn test_f14_03_video_viewport_aspect_ratio_modes() {
    let palette = TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x40A7E3, 14, 12);
    let mut vp = VideoViewport::new("Peer", palette);
    assert!(matches!(vp.aspect_ratio_mode, ObjectFit::Contain));

    vp.aspect_ratio_mode = ObjectFit::Cover;
    assert!(matches!(vp.aspect_ratio_mode, ObjectFit::Cover));
}

#[test]
fn test_f14_04_video_viewport_pip_corner_cycling() {
    let palette = TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x40A7E3, 14, 12);
    let mut vp = VideoViewport::new("Peer", palette);
    assert_eq!(vp.bubble.active_corner, BubbleCorner::BottomRight);
    vp.cycle_bubble_corner(1280.0, 720.0);
    assert_ne!(vp.bubble.active_corner, BubbleCorner::BottomRight);
}

#[test]
fn test_f14_05_call_overlay_toggles_camera_screen_and_mute() {
    let mut overlay = CallOverlay::new("Peer Bob".to_string());
    assert!(!overlay.is_camera_on);
    overlay.toggle_camera();
    assert!(overlay.is_camera_on);
    overlay.toggle_screen_share();
    assert!(overlay.is_screen_sharing);
    overlay.toggle_mute();
    assert!(overlay.is_mic_muted);
}

// ============================================================================
// F15: macOS NotificationsManager & Deep-Link Parser (5 tests)
// ============================================================================

#[test]
fn test_f15_01_mac_notification_dispatch_and_history() {
    let mut mgr = NotificationsManagerMacUnMac::new(1).with_mock_mode(true);
    let parsed = UNNotificationParsed::new(1, 500, 1001);
    mgr.show_system_notification(
        "Alice",
        "Work Chat",
        "Please review PR #42",
        true,
        parsed,
    )
    .unwrap();
    assert_eq!(mgr.dispatched_history.len(), 1);
    assert_eq!(mgr.dispatched_history[0].title, "Alice");
    assert_eq!(mgr.dispatched_history[0].message, "Please review PR #42");
}

#[test]
fn test_f15_02_mac_notification_click_and_reply_action() {
    let mut mgr = NotificationsManagerMacUnMac::new(1).with_mock_mode(true);
    let id1 = mgr
        .show_system_notification(
            "Bob",
            "",
            "Lunch?",
            false,
            UNNotificationParsed::new(1, 77, 888),
        )
        .unwrap();
    let ok = mgr
        .handle_notification_action(&id1, UNNotificationAction::InlineReply("On my way!".into()))
        .unwrap();
    assert!(ok);
    assert_eq!(mgr.replied_messages.len(), 1);
    assert_eq!(mgr.replied_messages[0].1, "On my way!");

    let id2 = mgr
        .show_system_notification(
            "Bob",
            "",
            "Follow-up",
            false,
            UNNotificationParsed::new(1, 77, 889),
        )
        .unwrap();
    let routed = mgr.handle_notification_click(&id2);
    assert_eq!(routed.as_deref(), Some("tg://chat?id=77"));
}

#[test]
fn test_f15_03_mac_notification_clear_from_peer_and_escape() {
    let mut mgr = NotificationsManagerMacUnMac::new(1).with_mock_mode(true);
    mgr.show_system_notification("P1", "", "m1", false, UNNotificationParsed::new(1, 10, 1))
        .unwrap();
    mgr.show_system_notification("P2", "", "m2", false, UNNotificationParsed::new(1, 20, 2))
        .unwrap();
    assert_eq!(mgr.in_flight_requests.len(), 2);

    mgr.clear_from_peer(10).unwrap();
    assert_eq!(mgr.in_flight_requests.len(), 1);
    assert_eq!(
        mgr.in_flight_requests.values().next().unwrap().parsed.peer_id,
        20
    );

    let escaped = NotificationsManagerMacUnMac::escape_applescript_string(r#"Say "hi"\n"#);
    assert!(escaped.contains("\\\"hi\\\""));
}

#[test]
fn test_f15_04_deep_link_resolve_username_and_chat() {
    match WindowSessionControllerLinkInfo::parse_deep_link("tg://resolve?domain=gpui_bot") {
        LinkInfoType::Peer(info) => {
            assert_eq!(info.username_or_id.as_deref(), Some("gpui_bot"));
        }
        other => panic!("expected Peer deep link, got {other:?}"),
    }

    match WindowSessionControllerLinkInfo::parse_deep_link("tg://chat?id=4242") {
        LinkInfoType::Peer(info) => assert_eq!(info.channel_id, Some(4242)),
        other => panic!("expected Peer chat link, got {other:?}"),
    }
}

#[test]
fn test_f15_05_deep_link_openmessage_and_invite() {
    match WindowSessionControllerLinkInfo::parse_deep_link(
        "tg://openmessage?chat_id=99&id=555",
    ) {
        LinkInfoType::Peer(info) => {
            assert_eq!(info.channel_id, Some(99));
            assert_eq!(info.message_id, Some(555));
        }
        other => panic!("expected openmessage link, got {other:?}"),
    }

    match WindowSessionControllerLinkInfo::parse_deep_link("tg://join?invite=AbCdEf123") {
        LinkInfoType::Peer(info) => {
            assert_eq!(info.chat_link_slug.as_deref(), Some("AbCdEf123"));
        }
        other => panic!("expected invite link, got {other:?}"),
    }
}

// ============================================================================
// F16: HttpPlatformImDriver Live HTTP Dispatch + Hermes ToolsetId Registry (5 tests)
// ============================================================================

#[tokio::test]
async fn test_f16_01_http_driver_send_and_history() {
    let (mut server, _driver, (_alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");
    let im = HttpPlatformImDriver::new(server.base_url(), &alice_token);

    let sent = im
        .execute(
            "im_send_message",
            json!({ "chat_id": chat_id, "text": "HttpPlatformImDriver message" }),
        )
        .await
        .expect("im_send_message");
    assert!(sent["message_id"].as_i64().unwrap() > 0);

    let hist = im
        .execute("im_get_chat_history", json!({ "chat_id": chat_id }))
        .await
        .expect("im_get_chat_history");
    assert!(hist["messages"]
        .as_array()
        .unwrap()
        .iter()
        .any(|m| m.to_string().contains("HttpPlatformImDriver message")));

    server.stop().await;
}

#[tokio::test]
async fn test_f16_02_http_driver_edit_and_delete_message() {
    let (mut server, _driver, (_alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");
    let im = HttpPlatformImDriver::new(server.base_url(), &alice_token);

    let sent = im
        .execute(
            "im_send_message",
            json!({ "chat_id": chat_id, "text": "Before edit" }),
        )
        .await
        .unwrap();
    let mid = sent["message_id"].as_i64().unwrap();

    let edited = im
        .execute(
            "im_edit_message",
            json!({ "chat_id": chat_id, "message_id": mid, "text": "After edit" }),
        )
        .await
        .expect("im_edit_message");
    assert!(edited.is_object());

    let del = im
        .execute(
            "im_delete_message",
            json!({ "chat_id": chat_id, "message_id": mid }),
        )
        .await
        .expect("im_delete_message");
    assert!(del.is_object());

    server.stop().await;
}

#[tokio::test]
async fn test_f16_03_http_driver_reactions_and_pins() {
    let (mut server, _driver, (_alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");
    let im = HttpPlatformImDriver::new(server.base_url(), &alice_token);

    let sent = im
        .execute(
            "im_send_message",
            json!({ "chat_id": chat_id, "text": "React & Pin" }),
        )
        .await
        .unwrap();
    let mid = sent["message_id"].as_i64().unwrap();

    let react = im
        .execute(
            "im_set_reaction",
            json!({ "chat_id": chat_id, "message_id": mid, "emoji": "🚀", "add": true }),
        )
        .await
        .expect("im_set_reaction");
    assert!(react.is_object());

    let pin = im
        .execute(
            "im_pin_message",
            json!({ "chat_id": chat_id, "message_id": mid }),
        )
        .await
        .expect("im_pin_message");
    assert!(pin.is_object());

    server.stop().await;
}

#[tokio::test]
async fn test_f16_04_http_driver_poll_and_vote() {
    let (mut server, _driver, (_alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");
    let im = HttpPlatformImDriver::new(server.base_url(), &alice_token);

    let poll = im
        .execute(
            "im_create_poll",
            json!({
                "chat_id": chat_id,
                "question": "Release blocker?",
                "options": ["None", "Need fix"]
            }),
        )
        .await
        .expect("im_create_poll");
    let poll_id = poll["poll"]["poll_id"]
        .as_u64()
        .or_else(|| poll["poll_id"].as_u64())
        .expect("poll_id");

    let voted = im
        .execute(
            "im_vote_poll",
            json!({ "poll_id": poll_id, "option_ids": [0] }),
        )
        .await
        .expect("im_vote_poll");
    assert!(voted.is_object());

    server.stop().await;
}

#[test]
fn test_f16_05_hermes_toolsets_enum_complete() {
    let all = ToolsetId::all();
    assert_eq!(all.len(), 29);
    assert!(all.contains(&ToolsetId::Kanban));
    assert!(all.contains(&ToolsetId::SessionSearch));
    assert!(all.contains(&ToolsetId::Mcp));
    assert!(all.contains(&ToolsetId::PlatformIm));
}

// ============================================================================
// F17: HermesStateStore (Kanban, Cron, Session Search, KV) & Server API (5 tests)
// ============================================================================

#[test]
fn test_f17_01_hermes_kanban_board_and_card_move() {
    let store = HermesStateStore::open_in_memory().expect("open HermesStateStore");
    let board = store
        .create_kanban_board("Sprint 1", "E2E board", None)
        .expect("create board");
    let card = store
        .create_kanban_card(CreateKanbanCardRequest {
            id: None,
            board_id: board.id.clone(),
            column_name: Some("Backlog".into()),
            title: "Verify WebRTC".into(),
            description: Some("Wire live SDP".into()),
            priority: Some("high".into()),
            assignee: Some("worker_2".into()),
            labels: Some(vec!["webrtc".into()]),
        })
        .expect("create card");

    let moved = store
        .move_kanban_card(&card.id, "Done", None)
        .expect("move card");
    assert_eq!(moved.column_name, "Done");

    let patched = store
        .update_kanban_card(
            &card.id,
            KanbanCardPatch {
                title: Some("Verify WebRTC (Done)".into()),
                ..Default::default()
            },
        )
        .expect("update card");
    assert_eq!(patched.title, "Verify WebRTC (Done)");
}

#[test]
fn test_f17_02_hermes_cron_job_persistence_across_reopen() {
    let tmp = new_test_dir("f17_02");
    let db_path = tmp.path().join("hermes_cron.db");
    let job_id = {
        let store = HermesStateStore::open(&db_path).expect("open store");
        let job = store
            .create_cron_job(CreateCronJobRequest {
                id: None,
                name: "nightly_cleanup".into(),
                schedule: "0 3 * * *".into(),
                timezone: None,
                target_chat_id: None,
                prompt: "Run ledger check".into(),
                model: None,
                skills: None,
                enabled: Some(true),
                deliver_to: None,
                origin_chat_id: None,
                next_run_at: None,
            })
            .expect("create cron");
        job.id
    };

    let reopened = HermesStateStore::open(&db_path).expect("reopen store");
    let got = reopened.get_cron_job(&job_id).expect("query cron").unwrap();
    assert_eq!(got.name, "nightly_cleanup");
    assert_eq!(got.schedule, "0 3 * * *");
}

#[test]
fn test_f17_03_hermes_session_append_and_search() {
    let store = HermesStateStore::open_in_memory().expect("open store");
    let sess = store
        .create_session(CreateSessionRequest {
            id: Some("e2e-hermes-sess".into()),
            chat_id: None,
            bot_id: None,
            source: None,
            model: None,
            api_mode: None,
            title: Some("Engineering".into()),
            parent_session_id: None,
            branch_from_seq: None,
            system_prompt_hash: None,
            active_toolsets: None,
            metadata: None,
        })
        .expect("create session");
    store
        .append_message(AppendMessageRequest {
            id: None,
            session_id: sess.id.clone(),
            role: "user".into(),
            content: "Investigate Tantivy indexer commit".into(),
            reasoning: None,
            tool_calls_json: None,
            tool_call_id: None,
            tool_name: None,
            token_count: 10,
            created_at: None,
        })
        .expect("append user");
    store
        .append_message(AppendMessageRequest {
            id: None,
            session_id: sess.id.clone(),
            role: "assistant".into(),
            content: "Committed search segment".into(),
            reasoning: None,
            tool_calls_json: None,
            tool_call_id: None,
            tool_name: None,
            token_count: 8,
            created_at: None,
        })
        .expect("append assistant");

    let hits = store
        .session_search("Tantivy", None, 10)
        .expect("session_search");
    assert!(!hits.is_empty());
    assert!(hits.iter().any(|h| h.snippet.contains("Tantivy")));
}

#[test]
fn test_f17_04_hermes_kv_store_roundtrip() {
    let store = HermesStateStore::open_in_memory().expect("open store");
    store
        .kv_set("config", "model_tier", "reasoning-pro")
        .unwrap();
    assert_eq!(
        store.kv_get("config", "model_tier").unwrap().as_deref(),
        Some("reasoning-pro")
    );
}

#[tokio::test]
async fn test_f17_05_server_http_hermes_kanban_and_cron_endpoints() {
    let (mut server, driver, (_alice_id, alice_token), _bob, _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let (st_b, board_body) = driver
        .json_req(
            Method::POST,
            "/api/v1/hermes/kanban/boards",
            Some(&alice_token),
            Some(json!({ "name": "Server Board", "title": "Server Board", "description": "Live API" })),
        )
        .await
        .unwrap();
    assert!((200..300).contains(&st_b), "board create: {st_b} {board_body:?}");
    let board_id = board_body["board_id"]
        .as_str()
        .or_else(|| board_body["board"]["id"].as_str())
        .or_else(|| board_body["id"].as_str())
        .unwrap()
        .to_string();

    let (st_c, card_body) = driver
        .json_req(
            Method::POST,
            "/api/v1/hermes/kanban/cards",
            Some(&alice_token),
            Some(json!({
                "board_id": board_id,
                "title": "API Card",
                "column_name": "Backlog"
            })),
        )
        .await
        .unwrap();
    assert!((200..300).contains(&st_c), "card create: {st_c} {card_body:?}");

    let (st_cron, cron_body) = driver
        .json_req(
            Method::POST,
            "/api/v1/hermes/cron",
            Some(&alice_token),
            Some(json!({
                "name": "server_cron",
                "schedule": "*/15 * * * *",
                "prompt": "Health check"
            })),
        )
        .await
        .unwrap();
    assert!((200..300).contains(&st_cron), "cron create: {st_cron} {cron_body:?}");

    server.stop().await;
}

// ============================================================================
// F18: Semantic Workspace Tree Inspection & Action Execution (5 tests)
// ============================================================================

#[test]
fn test_f18_01_semantic_tree_select_chat_and_send_message() {
    let mut tree = SemanticWorkspaceTreeSimulator::new();
    tree.register_chat(101);
    let r1 = tree.perform_action(SemanticAction::SelectChat { chat_id: 101 });
    assert!(r1.success);
    let r2 = tree.perform_action(SemanticAction::ComposeText {
        text: "Hello workspace".into(),
    });
    assert!(r2.success);
    let r3 = tree.perform_action(SemanticAction::SendMessage);
    assert!(r3.success);

    let snap = tree.inspect_state();
    assert_eq!(snap.selected_chat_id, Some(101));
    assert!(snap.composer_text.is_empty());
}

#[test]
fn test_f18_02_semantic_tree_requires_selected_chat_before_send() {
    let mut tree = SemanticWorkspaceTreeSimulator::new();
    tree.perform_action(SemanticAction::ComposeText {
        text: "Orphan msg".into(),
    });
    let receipt = tree.perform_action(SemanticAction::SendMessage);
    assert!(!receipt.success, "SendMessage without SelectChat must error");
}

#[test]
fn test_f18_03_semantic_tree_modal_open_and_close() {
    let mut tree = SemanticWorkspaceTreeSimulator::new();
    let r1 = tree.perform_action(SemanticAction::OpenModal {
        modal_id: "settings".into(),
    });
    assert!(r1.success);
    assert_eq!(tree.inspect_state().active_modal.as_deref(), Some("settings"));

    let r2 = tree.perform_action(SemanticAction::CloseModal);
    assert!(r2.success);
    assert!(tree.inspect_state().active_modal.is_none());
}

#[test]
fn test_f18_04_semantic_tree_select_unregistered_chat_fails() {
    let mut tree = SemanticWorkspaceTreeSimulator::new();
    let receipt = tree.perform_action(SemanticAction::SelectChat { chat_id: 99999 });
    assert!(!receipt.success);
}

#[test]
fn test_f18_05_semantic_tree_trigger_call_state() {
    let mut tree = SemanticWorkspaceTreeSimulator::new();
    let r = tree.perform_action(SemanticAction::ToggleCall {
        peer_id: 888,
        is_video: true,
    });
    assert!(r.success);
    assert!(tree.inspect_state().is_call_active);
}

// ============================================================================
// F19: Mini Apps, BotFather & Agent Manifest Contracts (5 tests)
// ============================================================================

#[test]
fn test_f19_01_miniapp_manifest_dto_and_action_request() {
    let dto = MiniAppManifestDto {
        app_id: "calc_app".into(),
        bot_username: "calc_bot".into(),
        short_name: "calc".into(),
        title: "Calculator".into(),
        description: "Quick calculator".into(),
        category: "utilities".into(),
        icon_emoji: "🧮".into(),
        web_url: "https://apps.example/calc".into(),
        version: "1.0.0".into(),
        peer_types: vec!["user".into(), "group".into()],
        requested_permissions: vec![MiniAppPermission::ReadConversation],
        ui_schema: MiniAppUiSchema::default(),
    };
    let roundtrip: MiniAppManifestDto =
        serde_json::from_str(&serde_json::to_string(&dto).unwrap()).unwrap();
    assert_eq!(roundtrip.short_name, "calc");

    let act = MiniAppActionRequest::widget_event("expr", json!("6*7"), Some(100));
    assert_eq!(act.widget_id.as_deref(), Some("expr"));
}

#[test]
fn test_f19_02_agent_manifest_from_json_validates_bounds() {
    let raw = r#"{
        "app_id": "triage_agent",
        "version": "1.2.0",
        "readable_state_schema": { "type": "object" },
        "actions": [
            {
                "name": "classify_bug",
                "description": "Classify bug severity",
                "input_schema": { "type": "object", "required": ["title"] },
                "output_schema": { "type": "object" },
                "risk_level": "low",
                "requires_approval": false
            }
        ]
    }"#;
    let m = AgentManifest::from_json(raw).expect("parse AgentManifest");
    m.validate_state(&json!({ "open_count": 3 })).unwrap();
    assert!(m
        .validate_action_call("classify_bug", &json!({ "title": "Crash" }))
        .is_ok());
    assert!(m.validate_action_call("drop_database", &json!({})).is_err());
}

#[test]
fn test_f19_03_agent_manifest_execute_action_real_execution() {
    let m = AgentManifest {
        app_id: "ops_bot".into(),
        version: "1.0.0".into(),
        readable_state_schema: json!({ "type": "object" }),
        actions: vec![MiniAppActionSpec {
            name: "deploy_check".into(),
            description: "Check staging".into(),
            input_schema: json!({ "type": "object", "required": ["env"] }),
            output_schema: json!({ "type": "object" }),
            risk_level: "low".into(),
            requires_approval: false,
        }],
    };
    let out = m
        .execute_action("deploy_check", &json!({ "env": "staging" }), false)
        .expect("execute_action");
    assert_eq!(out["status"], "executed");
    assert_eq!(out["app_id"], "ops_bot");
    assert_eq!(out["params"]["env"], "staging");
}

#[tokio::test]
async fn test_f19_04_server_create_bot_and_miniapp() {
    let (mut server, driver, (_alice_id, alice_token), _bob, _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let (st_bot, bot_body) = driver
        .json_req(
            Method::POST,
            "/api/v1/bots",
            Some(&alice_token),
            Some(json!({ "display_name": "E2E Helper", "username": "e2e_helper_bot" })),
        )
        .await
        .unwrap();
    assert!((200..300).contains(&st_bot), "create bot: {bot_body:?}");

    let (st_app, app_body) = driver
        .json_req(
            Method::POST,
            "/api/v1/miniapps",
            Some(&alice_token),
            Some(json!({
                "bot_username": "e2e_helper_bot",
                "short_name": "planner",
                "title": "Planner MiniApp"
            })),
        )
        .await
        .unwrap();
    assert!((200..300).contains(&st_app), "create miniapp: {app_body:?}");

    server.stop().await;
}

#[tokio::test]
async fn test_f19_05_server_miniapp_install_launch_and_action() {
    let (mut server, driver, (_alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let (st_b, _) = driver
        .json_req(
            Method::POST,
            "/api/v1/bots",
            Some(&alice_token),
            Some(json!({ "display_name": "Poll Bot", "username": "poll_e2e_bot" })),
        )
        .await
        .unwrap();
    assert!((200..300).contains(&st_b));

    let (st_m, _) = driver
        .json_req(
            Method::POST,
            "/api/v1/miniapps",
            Some(&alice_token),
            Some(json!({
                "bot_username": "poll_e2e_bot",
                "short_name": "tracker",
                "title": "Task Tracker"
            })),
        )
        .await
        .unwrap();
    assert!((200..300).contains(&st_m));

    let (st_i, _) = driver
        .json_req(
            Method::POST,
            "/api/v1/miniapps/poll_e2e_bot/tracker/install",
            Some(&alice_token),
            Some(json!({ "chat_id": chat_id })),
        )
        .await
        .unwrap();
    assert!((200..300).contains(&st_i));

    let (st_l, launch_body) = driver
        .json_req(
            Method::POST,
            "/api/v1/miniapps/poll_e2e_bot/tracker/launch",
            Some(&alice_token),
            Some(json!({ "chat_id": chat_id })),
        )
        .await
        .unwrap();
    assert!((200..300).contains(&st_l), "launch: {launch_body:?}");

    let (st_a, act_body) = driver
        .json_req(
            Method::POST,
            "/api/v1/miniapps/poll_e2e_bot/tracker/action",
            Some(&alice_token),
            Some(
                serde_json::to_value(MiniAppActionRequest::main_button_click(Some(chat_id)))
                    .unwrap(),
            ),
        )
        .await
        .unwrap();
    assert!((200..300).contains(&st_a), "action: {act_body:?}");

    server.stop().await;
}

// ============================================================================
// F20: Structured Cards & Generative Card Rendering Safety (5 tests)
// ============================================================================

#[test]
fn test_f20_01_todo_card_toggle_and_percentage() {
    let mut todo = TodoCard::new(
        "list-1",
        "Ship Checklist",
        vec![
            TodoItem::new("i1", "Compile cleanly"),
            TodoItem::new("i2", "Run all 265 E2E tests"),
        ],
    );
    assert_eq!(todo.completion_percentage(), 0.0);
    assert!(todo.toggle_item("i1"));
    assert_eq!(todo.completed_count, 1);
    assert_eq!(todo.completion_percentage(), 50.0);
}

#[test]
fn test_f20_02_poll_card_percentage_calculation() {
    let mut opt0 = PollOption::new(0, "Yes");
    opt0.vote_count = 3;
    let mut opt1 = PollOption::new(1, "No");
    opt1.vote_count = 1;
    let card = PollCard::new("p1", "Deploy?", vec![opt0, opt1], true);
    assert_eq!(card.total_votes, 4);
    assert!((card.options[0].percentage - 75.0).abs() < 0.01);
}

#[test]
fn test_f20_03_invoice_card_prices_and_paid_transition() {
    let mut inv = InvoiceCard::new("inv-1", "Pro Plan", "Annual", "USD", 0).with_prices(vec![
        PriceLine::new("Base", 1000),
        PriceLine::new("Support", 500),
    ]);
    assert_eq!(inv.total_from_prices(), 1500);
    inv.mark_paid();
    assert!(inv.is_paid);
}

#[test]
fn test_f20_04_generative_card_xss_sanitization() {
    let clean = GenerativeCard::sanitize_text(
        "<script>alert(1)</script>Hello <a onclick=\"evil()\">world</a>",
    );
    assert!(!clean.contains("<script"));
    assert!(clean.contains("&lt;script&gt;"));
    assert!(clean.contains("Hello"));
}

#[test]
fn test_f20_05_generative_card_streaming_delta_and_clamp() {
    let mut card = GenerativeCard {
        card_id: "gc-1".into(),
        title: "Build Status".into(),
        subtitle: None,
        layout: GenerativeCardLayout::VerticalStack,
        components: vec![
            GenerativeComponent::Text {
                text: String::new(),
                bold: false,
                color: None,
            },
            GenerativeComponent::Progress {
                value: 150.0,
                max: 100.0,
                label: Some("Done".into()),
            },
        ],
        status: Some("streaming".into()),
    };
    assert_eq!(card.clamped_progress(), vec![1.0]);
    card.append_streaming_delta(0, "Step 1 ok; ").unwrap();
    card.append_streaming_delta(0, "<script>bad()</script>Step 2 ok")
        .unwrap();
    match &card.components[0] {
        GenerativeComponent::Text { text, .. } => {
            assert!(text.contains("Step 1 ok"));
            assert!(text.contains("Step 2 ok"));
            assert!(!text.contains("<script"));
        }
        other => panic!("unexpected component: {other:?}"),
    }
}

// ============================================================================
// F21: Multi-Agent Floor Control, Loop Budget & Scoped Capabilities (5 tests)
// ============================================================================

#[test]
fn test_f21_01_floor_control_preemption_and_release() {
    let mut q = FloorControlQueue::new(5_000);
    assert!(q.request_floor_at("agent_low", FloorPriority::GeneralContribution, 10));
    assert!(!q.request_floor_at("agent_mid", FloorPriority::DirectMention, 20));
    assert!(q.request_floor_at("agent_user", FloorPriority::UserInterruption, 30));
    assert_eq!(q.current_speaker(), Some("agent_user"));

    // Release user interruption -> highest queued priority (DirectMention) promoted next
    q.release_floor("agent_user");
    assert_eq!(q.current_speaker(), Some("agent_mid"));
}

#[test]
fn test_f21_02_loop_budget_enforcement() {
    let mut budget = LoopBudget::new(3);
    assert_eq!(budget.record_hop("planner").unwrap(), 1);
    assert_eq!(budget.record_hop("coder").unwrap(), 2);
    assert_eq!(budget.record_hop("reviewer").unwrap(), 3);
    assert!(budget.is_exhausted());
    assert!(budget.record_hop("extra").is_err());
}

#[test]
fn test_f21_03_scoped_capability_token_signature_and_scope() {
    let sk = SigningKey::from_bytes(&[7u8; 32]);
    let vk = sk.verifying_key();
    let now = chrono::Utc::now().timestamp();
    let tok = ScopedCapabilityToken::issue(&sk, "agent-1", 42, vec!["chat:write".into()], 3_600);
    assert!(tok.verify(&vk, now + 10));
    assert!(tok.verify_for_scope(&vk, now + 10, "agent-1", 42, "chat:write"));
    assert!(!tok.verify_for_scope(&vk, now + 10, "agent-1", 42, "chat:admin"));
    assert!(!tok.verify(&vk, now + 5_000));
}

#[test]
fn test_f21_04_scoped_token_rejects_delimiter_injection() {
    let sk = SigningKey::from_bytes(&[8u8; 32]);
    let vk = sk.verifying_key();
    let now = chrono::Utc::now().timestamp();
    let tok = ScopedCapabilityToken::issue(
        &sk,
        "attacker|chat:admin|9999999",
        10,
        vec!["chat:read".into()],
        60,
    );
    assert!(!tok.verify_for_scope(&vk, now + 5, "attacker", 10, "chat:admin"));
    assert!(tok.verify_for_scope(
        &vk,
        now + 5,
        "attacker|chat:admin|9999999",
        10,
        "chat:read"
    ));
}

#[test]
fn test_f21_05_scoped_token_tamper_fails_ed25519() {
    let sk = SigningKey::from_bytes(&[9u8; 32]);
    let vk = sk.verifying_key();
    let now = chrono::Utc::now().timestamp();
    let mut tok = ScopedCapabilityToken::issue(&sk, "a1", 5, vec!["read".into()], 100);
    tok.expires_at += 999_999;
    assert!(!tok.verify(&vk, now + 50));
}

// ============================================================================
// F22: Genuine Tantivy Full-Text Indexer & HybridMemoryEngine (5 tests)
// ============================================================================

#[tokio::test]
async fn test_f22_01_tantivy_in_ram_index_and_search() {
    let idx = TantivyIndexer::in_ram().expect("TantivyIndexer::in_ram");
    idx.index_message(SearchDocument::new(
        "101",
        "55",
        "1",
        1_700_000_000,
        "Quantum cryptography handshake verified",
        Some("text".into()),
    ))
    .await
    .unwrap();
    idx.commit().await.unwrap();

    let hits = idx
        .search_hits(&SearchQueryParams::new("cryptography"))
        .expect("search_hits");
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].message_id, "101");
}

#[tokio::test]
async fn test_f22_02_tantivy_chat_filter_and_disk_reopen() {
    let tmp = new_test_dir("f22_02");
    let idx_dir = tmp.path().join("tantivy_idx");
    {
        let idx = TantivyIndexer::open_or_create(&idx_dir).expect("open_or_create");
        idx.index_message(SearchDocument::new(
            "1",
            "10",
            "1",
            100,
            "alpha release",
            Some("text".into()),
        ))
        .await
        .unwrap();
        idx.index_message(SearchDocument::new(
            "2",
            "20",
            "1",
            101,
            "alpha branch",
            Some("text".into()),
        ))
        .await
        .unwrap();
        idx.commit().await.unwrap();
    }

    let reopened = TantivyIndexer::open_or_create(&idx_dir).expect("reopen tantivy");
    let hits = reopened
        .search_hits(&SearchQueryParams::new("alpha").with_chat_id("20"))
        .unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].message_id, "2");
}

#[test]
fn test_f22_03_hybrid_memory_multi_word_scoring_and_graph_boost() {
    let engine = HybridMemoryEngine::new();
    engine.index_document(MemoryDocument {
        message_id: "m1".into(),
        chat_id: 1,
        content: "rust async tokio runtime".into(),
        embedding: vec![1.0, 0.0, 0.0],
        linked_entities: vec![],
    });
    engine.index_document(MemoryDocument {
        message_id: "m2".into(),
        chat_id: 1,
        content: "rust async tokio architecture design".into(),
        embedding: vec![1.0, 0.0, 0.0],
        linked_entities: vec!["tokio".into()],
    });

    let hits = engine.search("rust tokio", Some(&[1.0, 0.0, 0.0]), &[1], Some("tokio"));
    assert_eq!(hits.len(), 2);
    assert_eq!(hits[0].message_id, "m2", "graph-linked doc must boost higher");
}

#[test]
fn test_f22_04_hybrid_memory_chat_acl_filters_results() {
    let engine = HybridMemoryEngine::new();
    engine.index_document(MemoryDocument {
        message_id: "m1".into(),
        chat_id: 10,
        content: "hello world".into(),
        embedding: vec![1.0, 0.0],
        linked_entities: vec![],
    });
    assert!(engine
        .search("hello", Some(&[1.0, 0.0]), &[99], None)
        .is_empty());
}

#[test]
fn test_f22_05_hybrid_memory_cosine_similarity_zero_norm_safe() {
    let sim = HybridMemoryEngine::cosine_similarity(&[0.0, 0.0], &[1.0, 0.0]);
    assert_eq!(sim, 0.0);
    assert!(!sim.is_nan());
}

// ============================================================================
// F23: Genuine DefaultAiMediaTap & VadBargeInDetector (5 tests)
// ============================================================================

#[test]
fn test_f23_01_default_ai_media_tap_incoming_pcm_and_barge_in() {
    let tap = DefaultAiMediaTap::new(48_000, 1, 0.05).with_consecutive_frames(1);
    assert!(!tap.trigger_vad_barge_in());

    let speech = vec![12_000i16; 960];
    tap.on_incoming_pcm_frame(&speech, 48_000, 1);
    assert!(tap.trigger_vad_barge_in(), "loud speech triggers VAD barge-in");

    tap.reset_barge_in();
    assert!(!tap.trigger_vad_barge_in());
}

#[test]
fn test_f23_02_default_ai_media_tap_silence_does_not_trigger() {
    let tap = DefaultAiMediaTap::new(48_000, 1, 0.05).with_consecutive_frames(1);
    let silence = vec![5i16; 960];
    tap.on_incoming_pcm_frame(&silence, 48_000, 1);
    assert!(!tap.trigger_vad_barge_in());
}

#[test]
fn test_f23_03_default_ai_media_tap_outgoing_pcm_and_rms() {
    let tap = DefaultAiMediaTap::new(48_000, 1, 0.05).with_consecutive_frames(1);
    let tone = vec![16_384i16; 960];
    let rms = DefaultAiMediaTap::compute_rms(&tone);
    assert!((rms - 0.5).abs() < 0.02);

    tap.on_outgoing_pcm_frame(&tone, 48_000, 1);
    assert!(
        !tap.trigger_vad_barge_in(),
        "outgoing TTS audio must not trigger barge-in"
    );
}

#[test]
fn test_f23_04_vad_barge_in_detector_threshold_and_silence_streak() {
    let mut vad = VadBargeInDetector::new(1000.0);
    let loud = vec![5000i16; 960];
    assert!(!vad.process_pcm_frame(&loud));
    assert!(!vad.process_pcm_frame(&loud));
    assert!(vad.process_pcm_frame(&loud), "3rd consecutive loud frame triggers VAD barge-in");
    assert!(!vad.is_silence_detected());

    let silent = vec![0i16; 960];
    for _ in 0..25 {
        vad.process_pcm_frame(&silent);
    }
    assert!(vad.is_silence_detected());
}

#[tokio::test]
async fn test_f23_05_webrtc_transport_feeds_ai_media_tap() {
    let (mut a, b) = WebRtcPeerTransport::create_loopback_pair(PeerTransportConfig::default());
    let b = Arc::new(b);
    let tap = Arc::new(DefaultAiMediaTap::new(48_000, 1, 0.05).with_consecutive_frames(1));
    b.set_ai_media_tap(tap.clone());
    let rx_handle = WebRtcPeerTransport::spawn_loopback_receiver(b.clone()).await;

    let loud_frame = vec![10_000i16; 960];
    a.send_audio_pcm(&loud_frame).await.expect("send audio pcm");

    tokio::time::timeout(Duration::from_secs(2), async {
        while !tap.trigger_vad_barge_in() {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("AI media tap triggered by live WebRtcPeerTransport audio");

    if let Some(h) = rx_handle {
        h.abort();
    }
    a.close().await.unwrap();
}
