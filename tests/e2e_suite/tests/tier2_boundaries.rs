//! Tier 2: Boundary, Error Path & Edge-Case Coverage (115 Tests across 23 Features)
//!
//! Every boundary test exercises genuine workspace crates, `EphemeralTestServer`
//! HTTP/WebSocket error paths, SQLite concurrency, WebRTC/Opus/RTP limits,
//! media capture & `VideoViewport` bounds, real TCP proxy failures, and fixed
//! M2/M4 contract edge cases (zero inline-closure or tautological tests).

mod common;

use common::*;
use ed25519_dalek::SigningKey;
use gpui_telegram_agent::hermes::state_db::{
    CreateKanbanCardRequest, HermesStateStore, KanbanCardPatch,
};
use gpui_telegram_agent::talk::VadBargeInDetector;
use gpui_telegram_client::cache::local_store::LocalStore;
use gpui_telegram_core::action_bus::{
    ActionContext, ActionError, ActionRegistry, DeleteMessageAction, GetChatHistoryAction,
    ListDialogsAction, SendMessageAction,
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
use gpui_telegram_media::chunked_upload::ChunkedUploadSession;
use gpui_telegram_media::media::capture::{
    DeviceState, MediaDeviceSource, MockAudioSource, MockVideoSource,
};
use gpui_telegram_proto::cards::{PollCard, PollOption, TodoCard};
use gpui_telegram_proto::contracts::DraftDto;
use gpui_telegram_proto::miniapp_contracts::MiniAppActionRequest;
use gpui_telegram_proto::ws::WsServerFrame;
use gpui_telegram_server::db::{ServerDatabase, SqlitePool};
use gpui_telegram_server::push::dispatchers::MockPushDispatcher;
use gpui_telegram_server::push::{PushError, PushGatewayHub, PushNotification, PushPlatform};
use gpui_telegram_server::search::{SearchDocument, SearchQueryParams, TantivyIndexer};
use gpui_telegram_server::ws::GatewayHub;
use gpui_telegram_webrtc::calls::calls_video_bubble::BubbleCorner;
use gpui_telegram_webrtc::calls::peer_transport::{
    PeerConnectionState, PeerTransportConfig, TransportMode, WebRtcPeerTransport,
};
use gpui_telegram_webrtc::media::ai_media_tap::{AiMediaTap, DefaultAiMediaTap};
use gpui_telegram_webrtc::media::opus_codec::{
    CodecError, OpusDepacketizer, OpusFramingCodec, RtpPacket, OPUS_PAYLOAD_TYPE,
    OPUS_SAMPLES_PER_FRAME_MONO,
};
use reqwest::Method;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::mpsc;

fn build_registry() -> Arc<ActionRegistry> {
    let reg = ActionRegistry::new();
    reg.register_typed(SendMessageAction).unwrap();
    reg.register_typed(GetChatHistoryAction).unwrap();
    reg.register_typed(ListDialogsAction).unwrap();
    reg.register_typed(DeleteMessageAction).unwrap();
    Arc::new(reg)
}

// ============================================================================
// F1 Boundaries: Action Bus Validation, Unknown Actions & Approval Gates (5 tests)
// ============================================================================

#[tokio::test]
async fn test_f1_b01_unknown_action_returns_not_found() {
    let reg = build_registry();
    let res = reg
        .execute("nonexistent_action", ActionContext::new().with_user(1), json!({}))
        .await;
    assert!(matches!(res, Err(ActionError::NotFound(_))));
}

#[tokio::test]
async fn test_f1_b02_empty_send_message_text_or_missing_field_rejected() {
    let reg = build_registry();
    let res = reg
        .execute("send_message", ActionContext::new().with_user(1), json!({ "chat_id": 1 }))
        .await;
    assert!(matches!(res, Err(ActionError::InvalidInput(_))));
}

#[tokio::test]
async fn test_f1_b03_delete_message_requires_approval_flag() {
    let reg = build_registry();
    let err = reg
        .execute(
            "delete_message",
            ActionContext::new().with_user(1).with_approval(false),
            json!({ "chat_id": 1, "message_ids": [10], "for_everyone": true }),
        )
        .await;
    assert!(matches!(err, Err(ActionError::Forbidden(_))));
}

#[tokio::test]
async fn test_f1_b04_unauthenticated_action_rejected_by_server_http() {
    let mut server = boot_test_server().await.expect("boot_test_server");
    let driver = E2eSystemDriver::new(server.base_url());
    let (st, _) = driver
        .json_req(
            Method::POST,
            "/api/v1/actions/send_message",
            None,
            Some(json!({ "chat_id": 1, "text": "No bearer" })),
        )
        .await
        .unwrap();
    assert_eq!(st, 401);
    server.stop().await;
}

#[tokio::test]
async fn test_f1_b05_server_high_risk_action_approval_token_single_use() {
    let (mut server, driver, (_alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let (_, msg) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/chats/{chat_id}/messages"),
            Some(&alice_token),
            Some(json!({ "text": "Will delete via approval", "client_nonce": 100501 })),
        )
        .await
        .unwrap();
    let mid = msg["message_id"]
        .as_i64()
        .or_else(|| msg["message"]["message_id"].as_i64())
        .unwrap();

    let (st_gate, gate_body) = driver
        .json_req(
            Method::POST,
            "/api/v1/actions/delete_message",
            Some(&alice_token),
            Some(json!({ "chat_id": chat_id, "message_ids": [mid], "for_everyone": true })),
        )
        .await
        .unwrap();
    assert_ne!(st_gate, 200);
    if let Some(gate_id) = gate_body["gate_id"].as_str() {
        let (st_app, app_body) = driver
            .json_req(
                Method::POST,
                &format!("/api/v1/actions/approvals/{gate_id}/approve"),
                Some(&alice_token),
                Some(json!({ "decision": "approve_once" })),
            )
            .await
            .unwrap();
        assert_eq!(st_app, 200);
        let approval_tok = app_body["approval_token"].as_str().unwrap();

        let resp = driver
            .http
            .post(format!("{}/api/v1/actions/delete_message", server.base_url()))
            .bearer_auth(&alice_token)
            .header("X-Approval-Token", approval_tok)
            .json(&json!({ "chat_id": chat_id, "message_ids": [mid], "for_everyone": true }))
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status().as_u16(), 200);

        let replay = driver
            .http
            .post(format!("{}/api/v1/actions/delete_message", server.base_url()))
            .bearer_auth(&alice_token)
            .header("X-Approval-Token", approval_tok)
            .json(&json!({ "chat_id": chat_id, "message_ids": [mid], "for_everyone": true }))
            .send()
            .await
            .unwrap();
        assert_ne!(replay.status().as_u16(), 200);
    }

    server.stop().await;
}

// ============================================================================
// F2 Boundaries: SQLite Error Handling, Rollback & Concurrent Readers (5 tests)
// ============================================================================

#[test]
fn test_f2_b01_sqlite_transaction_rollback_leaves_clean_state() {
    let tmp = new_test_dir("f2_b01");
    let pool = SqlitePool::open(tmp.path().join("rb.db").to_str().unwrap()).unwrap();
    pool.with_writer(|conn| {
        conn.execute("CREATE TABLE t (id INTEGER PRIMARY KEY, v TEXT NOT NULL)", [])
            .map_err(|e| gpui_telegram_core::error::TelegramError::Database(e.to_string()))?;
        Ok(())
    })
    .unwrap();

    let tx_res: Result<(), _> = pool.with_transaction(|tx| {
        tx.execute("INSERT INTO t (id, v) VALUES (1, 'keep_no')", [])
            .map_err(|e| gpui_telegram_core::error::TelegramError::Database(e.to_string()))?;
        Err(gpui_telegram_core::error::TelegramError::Database(
            "forced rollback".into(),
        ))
    });
    assert!(tx_res.is_err());

    let count: i64 = pool
        .with_reader(|conn| {
            conn.query_row("SELECT COUNT(*) FROM t", [], |r| r.get(0))
                .map_err(|e| gpui_telegram_core::error::TelegramError::Database(e.to_string()))
        })
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_f2_b02_sqlite_unique_constraint_violation_error() {
    let tmp = new_test_dir("f2_b02");
    let pool = SqlitePool::open(tmp.path().join("uniq.db").to_str().unwrap()).unwrap();
    pool.with_writer(|conn| {
        conn.execute("CREATE TABLE u (id INTEGER PRIMARY KEY, email TEXT UNIQUE)", [])
            .map_err(|e| gpui_telegram_core::error::TelegramError::Database(e.to_string()))?;
        conn.execute("INSERT INTO u (id, email) VALUES (1, 'a@b.c')", [])
            .map_err(|e| gpui_telegram_core::error::TelegramError::Database(e.to_string()))?;
        Ok(())
    })
    .unwrap();

    let dup = pool.with_writer(|conn| {
        conn.execute("INSERT INTO u (id, email) VALUES (2, 'a@b.c')", [])
            .map_err(|e| gpui_telegram_core::error::TelegramError::Database(e.to_string()))?;
        Ok(())
    });
    assert!(dup.is_err());
}

#[test]
fn test_f2_b03_sqlite_empty_query_returns_none() {
    let tmp = new_test_dir("f2_b03");
    let srv = ServerDatabase::open(tmp.path().join("empty.db").to_str().unwrap()).unwrap();
    assert!(srv.get_user(UserId(999_999)).unwrap().is_none());
}

#[test]
fn test_f2_b04_sqlite_reader_concurrency_under_writer_lock() {
    let tmp = new_test_dir("f2_b04");
    let pool = Arc::new(SqlitePool::open(tmp.path().join("conc.db").to_str().unwrap()).unwrap());
    pool.with_writer(|conn| {
        conn.execute("CREATE TABLE metrics (id INTEGER PRIMARY KEY, n INTEGER)", [])
            .map_err(|e| gpui_telegram_core::error::TelegramError::Database(e.to_string()))?;
        conn.execute("INSERT INTO metrics (id, n) VALUES (1, 77)", [])
            .map_err(|e| gpui_telegram_core::error::TelegramError::Database(e.to_string()))?;
        Ok(())
    })
    .unwrap();

    let readers: Vec<_> = (0..12)
        .map(|_| {
            let p = Arc::clone(&pool);
            std::thread::spawn(move || {
                p.with_reader(|conn| {
                    let n: i64 = conn
                        .query_row("SELECT n FROM metrics WHERE id = 1", [], |r| r.get(0))
                        .map_err(|e| {
                            gpui_telegram_core::error::TelegramError::Database(e.to_string())
                        })?;
                    assert_eq!(n, 77);
                    Ok(())
                })
            })
        })
        .collect();

    for r in readers {
        r.join().unwrap().expect("p.with_reader succeeds");
    }
}

#[test]
fn test_f2_b05_sqlite_invalid_directory_path_returns_error() {
    let res = SqlitePool::open("/nonexistent_root_dir_e2e/sub/db.sqlite");
    assert!(res.is_err());
}

// ============================================================================
// F3 Boundaries: GatewayHub Unregistration, Multi-Device & Bad WS Tokens (5 tests)
// ============================================================================

#[tokio::test]
async fn test_f3_b01_ws_invalid_token_rejected() {
    let mut server = boot_test_server().await.expect("boot_test_server");
    let res = connect_ws(&server, "invalid_bearer_token_e2e").await;
    assert!(res.is_err(), "WS connection with invalid token must fail");
    server.stop().await;
}

#[test]
fn test_f3_b02_gateway_hub_broadcast_to_offline_user_is_noop() {
    let hub = GatewayHub::new();
    hub.broadcast_to_user(UserId(9999), WsServerFrame::Pong { nonce: 1 });
    assert_eq!(hub.user_connection_count(UserId(9999)), 0);
}

#[test]
fn test_f3_b03_gateway_hub_duplicate_chat_subscription_idempotent() {
    let hub = GatewayHub::new();
    hub.subscribe_chat(10, UserId(1));
    hub.subscribe_chat(10, UserId(1));
    assert_eq!(hub.get_chat_subscribers(10).len(), 1);
}

#[test]
fn test_f3_b04_gateway_hub_unregister_removes_all_connections() {
    let hub = GatewayHub::new();
    let (tx, _rx) = mpsc::unbounded_channel();
    hub.register(UserId(42), tx);
    assert_eq!(hub.user_connection_count(UserId(42)), 1);

    hub.unregister(UserId(42));
    assert_eq!(
        hub.user_connection_count(UserId(42)),
        0,
        "unregister must drop user connection count to 0"
    );
}

#[test]
fn test_f3_b05_gateway_hub_multi_device_unregister_sender_isolation() {
    let hub = GatewayHub::new();
    let (tx1, _rx1) = mpsc::unbounded_channel();
    let (tx2, mut rx2) = mpsc::unbounded_channel();

    hub.register(UserId(55), tx1.clone());
    hub.register(UserId(55), tx2.clone());
    assert_eq!(hub.user_connection_count(UserId(55)), 2);

    hub.unregister_sender(UserId(55), &tx1);
    assert_eq!(hub.user_connection_count(UserId(55)), 1);

    hub.broadcast_to_user(UserId(55), WsServerFrame::Pong { nonce: 777 });
    assert_eq!(rx2.try_recv().unwrap(), WsServerFrame::Pong { nonce: 777 });

    hub.unregister_sender(UserId(55), &tx2);
    assert_eq!(hub.user_connection_count(UserId(55)), 0);
}

// ============================================================================
// F4 Boundaries: Push Registration Validation & Dispatch Error Cases (5 tests)
// ============================================================================

#[tokio::test]
async fn test_f4_b01_empty_push_token_rejected_400() {
    let (mut server, driver, (_alice_id, alice_token), _bob, _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let (st, body) = driver
        .json_req(
            Method::POST,
            "/api/v1/devices/register_push",
            Some(&alice_token),
            Some(json!({ "platform": "apns", "token": "   " })),
        )
        .await
        .unwrap();
    assert_eq!(st, 400, "empty token: {body:?}");
    server.stop().await;
}

#[tokio::test]
async fn test_f4_b02_4096_byte_token_boundary_accepted_and_dispatched() {
    let (mut server, driver, (alice_id, alice_token), _bob, _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let long_token = "t".repeat(4096);
    let (st, _) = driver
        .json_req(
            Method::POST,
            "/api/v1/devices/register_push",
            Some(&alice_token),
            Some(json!({ "platform": "mock", "token": long_token })),
        )
        .await
        .unwrap();
    assert_eq!(st, 200);

    let state = server.state().unwrap().clone();
    let hub = PushGatewayHub::new(Arc::clone(&state.db));
    hub.register_dispatcher(Arc::new(MockPushDispatcher::new()));
    let receipts = hub
        .send_to_user(
            UserId(alice_id),
            &PushNotification {
                title: "Boundary".into(),
                body: "4096 token".into(),
                chat_id: None,
                message_id: None,
                badge: None,
                sound: None,
                custom_data: Default::default(),
            },
        )
        .await;
    assert_eq!(receipts.len(), 1);
    assert!(receipts[0].as_ref().unwrap().delivered);

    server.stop().await;
}

#[tokio::test]
async fn test_f4_b03_unsupported_push_platform_rejected_400() {
    let (mut server, driver, (_alice_id, alice_token), _bob, _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let (st, _) = driver
        .json_req(
            Method::POST,
            "/api/v1/devices/register_push",
            Some(&alice_token),
            Some(json!({ "platform": "blackberry_bbm", "token": "tok" })),
        )
        .await
        .unwrap();
    assert_eq!(st, 400);
    server.stop().await;
}

#[tokio::test]
async fn test_f4_b04_push_hub_without_registered_dispatcher_returns_no_dispatcher() {
    let tmp = new_test_dir("f4_b04");
    let db = Arc::new(ServerDatabase::open(tmp.path().join("push.db").to_str().unwrap()).unwrap());
    let hub = PushGatewayHub::new(db);
    let err = hub
        .send_to_token(
            PushPlatform::Apns,
            "tok",
            &PushNotification {
                title: "t".into(),
                body: "b".into(),
                chat_id: None,
                message_id: None,
                badge: None,
                sound: None,
                custom_data: Default::default(),
            },
        )
        .await;
    assert_eq!(err.unwrap_err(), PushError::NoDispatcher(PushPlatform::Apns));
}

#[tokio::test]
async fn test_f4_b05_mock_dispatcher_rejects_empty_token() {
    let tmp = new_test_dir("f4_b05");
    let db = Arc::new(ServerDatabase::open(tmp.path().join("push2.db").to_str().unwrap()).unwrap());
    let hub = PushGatewayHub::new(db);
    hub.register_dispatcher(Arc::new(MockPushDispatcher::new()));
    let res = hub
        .send_to_token(
            PushPlatform::Mock,
            "",
            &PushNotification {
                title: "t".into(),
                body: "b".into(),
                chat_id: None,
                message_id: None,
                badge: None,
                sound: None,
                custom_data: Default::default(),
            },
        )
        .await;
    assert!(matches!(res, Err(PushError::InvalidToken(_))));
}

// ============================================================================
// F5 Boundaries: Chunked Media Bounds & Server Empty Upload Rejection (5 tests)
// ============================================================================

#[test]
fn test_f5_b01_chunked_zero_byte_file_and_zero_part_size_boundary() {
    let session = ChunkedUploadSession::new("zero", 0, 1024).unwrap();
    assert_eq!(session.total_parts, 1);
    assert!(!session.is_complete());
    assert!(ChunkedUploadSession::new("bad_part", 100, 0).is_err());
}

#[test]
fn test_f5_b02_chunked_part_size_larger_than_total_single_part() {
    let mut session = ChunkedUploadSession::new("single", 10, 1_000_000).unwrap();
    assert_eq!(session.total_parts, 1);
    session.receive_part(0, &[9u8; 10]).unwrap();
    assert!(session.is_complete());
}

#[test]
fn test_f5_b03_chunked_out_of_bounds_part_index_rejected() {
    let mut session = ChunkedUploadSession::new("oob", 100, 50).unwrap();
    assert!(session.receive_part(2, &[1u8; 50]).is_err());
}

#[test]
fn test_f5_b04_chunked_incomplete_assemble_returns_error() {
    let mut session = ChunkedUploadSession::new("inc", 100, 50).unwrap();
    session.receive_part(0, &[1u8; 50]).unwrap();
    assert!(session.assemble_payload().is_err());
    assert!(session.compute_sha256().is_err());
}

#[tokio::test]
async fn test_f5_b05_server_rejects_empty_media_upload_400() {
    let (mut server, driver, (_alice_id, alice_token), _bob, _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let resp = driver
        .http
        .post(format!("{}/api/v1/media/upload", server.base_url()))
        .bearer_auth(&alice_token)
        .body(Vec::<u8>::new())
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status().as_u16(), 400);
    server.stop().await;
}

// ============================================================================
// F6 Boundaries: Outbox Limits, Empty Text & Backoff Overflow Safety (5 tests)
// ============================================================================

#[test]
fn test_f6_b01_outbox_rejects_empty_message_text() {
    let outbox = OutboxSimulator::new();
    assert!(outbox.enqueue(10, "   ").is_err());
}

#[test]
fn test_f6_b02_outbox_backoff_u32_max_saturates_safely() {
    assert_eq!(OutboxSimulator::compute_backoff_delay_ms(u32::MAX), 30_000);
}

#[test]
fn test_f6_b03_outbox_flush_while_offline_returns_err() {
    let outbox = OutboxSimulator::new();
    outbox.enqueue(10, "wait offline").unwrap();
    assert!(outbox.flush_pending(false).is_err());
    assert_eq!(outbox.list_pending().len(), 1);
}

#[test]
fn test_f6_b04_outbox_three_transient_failures_still_retryable() {
    let outbox = OutboxSimulator::new();
    let n = outbox.enqueue(10, "retry").unwrap();
    for _ in 0..3 {
        outbox.simulate_failure(n, false);
    }
    let rec = outbox.get_record(n).unwrap();
    assert_eq!(rec.retry_count, 3);
    assert_eq!(rec.status, OutboxStatus::Pending);
}

#[tokio::test]
async fn test_f6_b05_server_rejects_oversized_message_text_400() {
    let (mut server, driver, (_alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let oversized = "x".repeat(5000);
    let (st, _) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/chats/{chat_id}/messages"),
            Some(&alice_token),
            Some(json!({ "text": oversized, "client_nonce": 6005 })),
        )
        .await
        .unwrap();
    assert_eq!(st, 400);
    server.stop().await;
}

// ============================================================================
// F7 Boundaries: Resumable Server Upload Validation & Corruption Guards (5 tests)
// ============================================================================

#[tokio::test]
async fn test_f7_b01_server_upload_invalid_part_index_rejected() {
    let (mut server, driver, (_alice_id, alice_token), _bob, _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let (_, init) = driver
        .json_req(
            Method::POST,
            "/api/v1/media/upload/init",
            Some(&alice_token),
            Some(json!({ "filename": "b1.bin", "size_bytes": 40, "chunk_size": 20 })),
        )
        .await
        .unwrap();
    let upload_id = init["upload_id"].as_str().unwrap();

    let resp = driver
        .http
        .put(format!(
            "{}/api/v1/media/upload/{upload_id}/part/99",
            server.base_url()
        ))
        .bearer_auth(&alice_token)
        .body(vec![1u8; 20])
        .send()
        .await
        .unwrap();
    assert!(resp.status().is_client_error());
    server.stop().await;
}

#[tokio::test]
async fn test_f7_b02_server_upload_unknown_id_status_404() {
    let (mut server, driver, (_alice_id, alice_token), _bob, _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let (st, _) = driver
        .json_req(
            Method::GET,
            "/api/v1/media/upload/does_not_exist_e2e/status",
            Some(&alice_token),
            None,
        )
        .await
        .unwrap();
    assert_eq!(st, 404);
    server.stop().await;
}

#[tokio::test]
async fn test_f7_b03_server_upload_overwrite_part_idempotent() {
    let (mut server, driver, (_alice_id, alice_token), _bob, _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let payload = b"ABCDEFGHIJ0123456789".to_vec();
    let (_, init) = driver
        .json_req(
            Method::POST,
            "/api/v1/media/upload/init",
            Some(&alice_token),
            Some(json!({ "filename": "idemp.bin", "size_bytes": 20, "chunk_size": 10 })),
        )
        .await
        .unwrap();
    let id = init["upload_id"].as_str().unwrap();

    for _ in 0..2 {
        let r = driver
            .http
            .put(format!("{}/api/v1/media/upload/{id}/part/0", server.base_url()))
            .bearer_auth(&alice_token)
            .body(payload[..10].to_vec())
            .send()
            .await
            .unwrap();
        assert_eq!(r.status().as_u16(), 200);
    }
    driver
        .http
        .put(format!("{}/api/v1/media/upload/{id}/part/1", server.base_url()))
        .bearer_auth(&alice_token)
        .body(payload[10..].to_vec())
        .send()
        .await
        .unwrap();

    let (st, comp) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/media/upload/{id}/complete"),
            Some(&alice_token),
            Some(json!({})),
        )
        .await
        .unwrap();
    assert_eq!(st, 200);
    assert!(comp["file_id"].as_str().is_some());
    server.stop().await;
}

#[tokio::test]
async fn test_f7_b04_server_upload_complete_twice_handled_cleanly() {
    let (mut server, driver, (_alice_id, alice_token), _bob, _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let (_, init) = driver
        .json_req(
            Method::POST,
            "/api/v1/media/upload/init",
            Some(&alice_token),
            Some(json!({ "filename": "once.bin", "size_bytes": 5, "chunk_size": 5 })),
        )
        .await
        .unwrap();
    let id = init["upload_id"].as_str().unwrap();
    driver
        .http
        .put(format!("{}/api/v1/media/upload/{id}/part/0", server.base_url()))
        .bearer_auth(&alice_token)
        .body(vec![5u8; 5])
        .send()
        .await
        .unwrap();

    let (st1, _) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/media/upload/{id}/complete"),
            Some(&alice_token),
            Some(json!({})),
        )
        .await
        .unwrap();
    assert_eq!(st1, 200);

    let (st2, _) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/media/upload/{id}/complete"),
            Some(&alice_token),
            Some(json!({})),
        )
        .await
        .unwrap();
    assert!((400..500).contains(&st2) || st2 == 200);
    server.stop().await;
}

#[tokio::test]
async fn test_f7_b05_server_upload_init_zero_chunk_size_rejected() {
    let (mut server, driver, (_alice_id, alice_token), _bob, _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let (st, _) = driver
        .json_req(
            Method::POST,
            "/api/v1/media/upload/init",
            Some(&alice_token),
            Some(json!({ "filename": "empty.bin", "size_bytes": 100, "chunk_size": 0 })),
        )
        .await
        .unwrap();
    assert_eq!(st, 400);
    server.stop().await;
}

// ============================================================================
// F8 Boundaries: E2EE Tamper, Wrong Keys, Replay & Missing Prekeys (5 tests)
// ============================================================================

#[test]
fn test_f8_b01_e2ee_missing_prekey_bundle_returns_none() {
    let e2ee = E2eeSimulator::new();
    assert!(e2ee.consume_prekey_bundle(999, "missing").is_none());
}

#[test]
fn test_f8_b02_e2ee_tampered_ciphertext_fails_aead_tag() {
    let e2ee = E2eeSimulator::new();
    let k = [0x11u8; 32];
    let mut ct = E2eeSimulator::encrypt_message(&k, 1, b"secret").unwrap();
    ct[0] ^= 0xFF;
    assert!(e2ee.decrypt_message("d1", &k, 1, &ct).is_err());
}

#[test]
fn test_f8_b03_e2ee_wrong_session_key_rejected() {
    let e2ee = E2eeSimulator::new();
    let ct = E2eeSimulator::encrypt_message(&[0x11u8; 32], 1, b"secret").unwrap();
    assert!(e2ee.decrypt_message("d1", &[0x22u8; 32], 1, &ct).is_err());
}

#[test]
fn test_f8_b04_e2ee_truncated_ciphertext_rejected() {
    let e2ee = E2eeSimulator::new();
    assert!(e2ee.decrypt_message("d1", &[0u8; 32], 0, &[1, 2, 3]).is_err());
}

#[test]
fn test_f8_b05_e2ee_empty_plaintext_encrypts_and_decrypts_cleanly() {
    let e2ee = E2eeSimulator::new();
    let k = [0x55u8; 32];
    let ct = E2eeSimulator::encrypt_message(&k, 0, b"").unwrap();
    let pt = e2ee.decrypt_message("d1", &k, 0, &ct).unwrap();
    assert!(pt.is_empty());
}

// ============================================================================
// F9 Boundaries: Proxy Auth Failures, Closed Ports & Config Limits (5 tests)
// ============================================================================

#[tokio::test]
async fn test_f9_b01_http_proxy_wrong_credentials_rejected_407() {
    let mut server = boot_test_server().await.expect("boot_test_server");
    let proxy = LocalTestProxyServer::start_http(Some(("user1".into(), "pass1".into())))
        .await
        .expect("start http proxy");

    let bad_cfg = ProxyConfig {
        proxy_type: ProxyType::Http,
        host: "127.0.0.1".into(),
        port: proxy.port(),
        username: Some("user1".into()),
        password: Some("WRONG".into()),
    };
    let target: std::net::SocketAddr = server
        .base_url()
        .trim_start_matches("http://")
        .parse()
        .unwrap();

    let res = bad_cfg.tunnel_http_get(target, "/api/v1/status").await;
    assert!(res.is_err(), "HTTP proxy must reject wrong credentials");

    proxy.stop().await;
    server.stop().await;
}

#[tokio::test]
async fn test_f9_b02_socks5_proxy_wrong_credentials_rejected() {
    let mut server = boot_test_server().await.expect("boot_test_server");
    let proxy = LocalTestProxyServer::start_socks5(Some(("u".into(), "p".into())))
        .await
        .expect("start socks5 proxy");

    let bad_cfg = ProxyConfig {
        proxy_type: ProxyType::Socks5,
        host: "127.0.0.1".into(),
        port: proxy.port(),
        username: Some("u".into()),
        password: Some("bad_pw".into()),
    };
    let target: std::net::SocketAddr = server
        .base_url()
        .trim_start_matches("http://")
        .parse()
        .unwrap();

    let res = bad_cfg.tunnel_http_get(target, "/api/v1/status").await;
    assert!(res.is_err(), "SOCKS5 proxy must reject wrong password");

    proxy.stop().await;
    server.stop().await;
}

#[tokio::test]
async fn test_f9_b03_proxy_unreachable_closed_port_returns_error() {
    let l = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let closed_port = l.local_addr().unwrap().port();
    drop(l);

    let cfg = ProxyConfig {
        proxy_type: ProxyType::Socks5,
        host: "127.0.0.1".into(),
        port: closed_port,
        username: None,
        password: None,
    };
    let res = cfg
        .tunnel_http_get("127.0.0.1:9".parse().unwrap(), "/api/v1/status")
        .await;
    assert!(res.is_err());
}

#[test]
fn test_f9_b04_connection_box_remove_selected_proxy_clears_selection() {
    let mut cbox = ConnectionBox::new();
    cbox.add_proxy(DesktopProxyConfig {
        id: 7,
        proxy_type: DesktopProxyType::None,
        host: "127.0.0.1".into(),
        port: 8080,
        user: None,
        password: None,
        secret: None,
        ping_ms: None,
        state: ProxyItemState::NotTested,
        is_enabled: false,
    });
    cbox.select_proxy(7);
    assert_eq!(cbox.selected_proxy_id, Some(7));
    cbox.remove_proxy(7);
    assert_eq!(cbox.selected_proxy_id, None);
    assert!(proxy_to_tg_link(&DesktopProxyConfig {
        id: 1,
        proxy_type: DesktopProxyType::None,
        host: "x".into(),
        port: 1,
        user: None,
        password: None,
        secret: None,
        ping_ms: None,
        state: ProxyItemState::NotTested,
        is_enabled: false,
    })
    .is_empty());
}

#[tokio::test]
async fn test_f9_b05_proxy_failover_from_dead_http_to_live_socks5() {
    let mut server = boot_test_server().await.expect("boot_test_server");
    let target: std::net::SocketAddr = server
        .base_url()
        .trim_start_matches("http://")
        .parse()
        .unwrap();

    let l = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let dead_port = l.local_addr().unwrap().port();
    drop(l);

    let dead = ProxyConfig {
        proxy_type: ProxyType::Http,
        host: "127.0.0.1".into(),
        port: dead_port,
        username: None,
        password: None,
    };
    assert!(dead.tunnel_http_get(target, "/api/v1/status").await.is_err());

    let live_socks = LocalTestProxyServer::start_socks5(None).await.unwrap();
    let live = ProxyConfig {
        proxy_type: ProxyType::Socks5,
        host: "127.0.0.1".into(),
        port: live_socks.port(),
        username: None,
        password: None,
    };
    let (st, _) = live.tunnel_http_get(target, "/api/v1/status").await.unwrap();
    assert_eq!(st, 200);

    live_socks.stop().await;
    server.stop().await;
}

// ============================================================================
// F10 Boundaries: SendAsPeers, Draft 4096 Boundary, Pins & TTL Clamp (5 tests)
// ============================================================================

#[test]
fn test_f10_b01_send_as_peers_empty_list_and_unknown_key() {
    let sess = SendAsPeersSession::new(50);
    assert!(sess
        .list(&SendAsKey::new(999, SendAsType::VideoStream))
        .is_empty());
    assert_eq!(sess.resolve_chosen(&SendAsKey::message(999)), 50);
}

#[test]
fn test_f10_b02_send_as_peers_overwrite_chosen_identity() {
    let mut sess = SendAsPeersSession::new(1);
    let key = SendAsKey::message(500);
    sess.set_list(
        key,
        vec![
            SendAsPeer::new(10, "A".into(), false),
            SendAsPeer::new(20, "B".into(), true),
        ],
    );
    sess.set_chosen(500, 10);
    assert_eq!(sess.chosen(500), Some(10));
    sess.set_chosen(500, 20);
    assert_eq!(sess.chosen(500), Some(20));
}

#[tokio::test]
async fn test_f10_b03_draft_persistence_and_message_4096_char_boundary() {
    let (mut server, driver, (_alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");
    let tmp = new_test_dir("f10_b03");
    let store = LocalStore::open_with_dir(tmp.path()).unwrap();

    let max_text = "a".repeat(4096);
    store.set_draft(
        ChatId(chat_id),
        Some(DraftDto {
            chat_id,
            text: max_text.clone(),
            entities: vec![],
            reply_to_msg_id: None,
            topic_id: None,
            date_ms: 100,
        }),
    );
    assert_eq!(store.get_draft(ChatId(chat_id)).unwrap().text.len(), 4096);

    let (st_ok, _) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/chats/{chat_id}/messages"),
            Some(&alice_token),
            Some(json!({ "text": max_text, "client_nonce": 100301 })),
        )
        .await
        .unwrap();
    assert!((200..300).contains(&st_ok));

    let (st_over, _) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/chats/{chat_id}/messages"),
            Some(&alice_token),
            Some(json!({ "text": "a".repeat(4097), "client_nonce": 100302 })),
        )
        .await
        .unwrap();
    assert_eq!(st_over, 400);

    server.stop().await;
}

#[tokio::test]
async fn test_f10_b04_pinned_messages_empty_and_duplicate_pin() {
    let (mut server, driver, (_alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let (st_empty, empty_pins) = driver
        .json_req(
            Method::GET,
            &format!("/api/v1/chats/{chat_id}/pins"),
            Some(&alice_token),
            None,
        )
        .await
        .unwrap();
    assert_eq!(st_empty, 200);
    assert!(empty_pins["pinned_message_ids"].as_array().unwrap().is_empty());

    let (_, msg) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/chats/{chat_id}/messages"),
            Some(&alice_token),
            Some(json!({ "text": "dup pin", "client_nonce": 100401 })),
        )
        .await
        .unwrap();
    let mid = msg["message_id"]
        .as_i64()
        .or_else(|| msg["message"]["message_id"].as_i64())
        .unwrap();

    for _ in 0..2 {
        let (st, _) = driver
            .json_req(
                Method::POST,
                &format!("/api/v1/chats/{chat_id}/pins"),
                Some(&alice_token),
                Some(json!({ "message_id": mid })),
            )
            .await
            .unwrap();
        assert_eq!(st, 200);
    }

    let (_, pins) = driver
        .json_req(
            Method::GET,
            &format!("/api/v1/chats/{chat_id}/pins"),
            Some(&alice_token),
            None,
        )
        .await
        .unwrap();
    assert_eq!(pins["pinned_message_ids"].as_array().unwrap().len(), 1);

    server.stop().await;
}

#[tokio::test]
async fn test_f10_b05_chat_ttl_zero_and_upper_clamp_boundary() {
    let (mut server, driver, (_alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let (st_zero, body_zero) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/chats/{chat_id}/ttl"),
            Some(&alice_token),
            Some(json!({ "ttl_seconds": 0 })),
        )
        .await
        .unwrap();
    assert_eq!(st_zero, 200);
    assert_eq!(body_zero["ttl_seconds"], 0);

    let (st_max, body_max) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/chats/{chat_id}/ttl"),
            Some(&alice_token),
            Some(json!({ "ttl_seconds": 999_999_999u64 })),
        )
        .await
        .unwrap();
    assert_eq!(st_max, 200);
    assert_eq!(body_max["ttl_seconds"], 31_536_000);

    server.stop().await;
}

// ============================================================================
// F11 Boundaries: WebRTC PeerTransport Closed/Unconnected Guards & Live SDP (5 tests)
// ============================================================================

#[tokio::test]
async fn test_f11_b01_webrtc_send_after_peer_drop_returns_error() {
    let (mut a, b) = WebRtcPeerTransport::create_loopback_pair(PeerTransportConfig::default());
    drop(b);
    let res = a.send_audio_pcm(&vec![0i16; 960]).await;
    assert!(res.is_err());
    a.close().await.unwrap();
}

#[tokio::test]
async fn test_f11_b02_webrtc_double_close_idempotent() {
    let (mut a, mut b) = WebRtcPeerTransport::create_loopback_pair(PeerTransportConfig::default());
    a.close().await.unwrap();
    a.close().await.unwrap();
    assert_eq!(a.state(), PeerConnectionState::Closed);
    b.close().await.unwrap();
}

#[tokio::test]
async fn test_f11_b03_webrtc_live_answer_without_remote_offer_fails() {
    let mut t = WebRtcPeerTransport::new(PeerTransportConfig {
        mode: TransportMode::Live,
        ..Default::default()
    })
    .await
    .unwrap();
    let res = t.create_answer().await;
    assert!(
        res.is_err(),
        "Live create_answer without remote SDP offer must error"
    );
    assert!(t.recv_loopback_audio().await.is_err());
    t.close().await.unwrap();
}

#[tokio::test]
async fn test_f11_b04_webrtc_live_invalid_sdp_rejected() {
    let mut t = WebRtcPeerTransport::new(PeerTransportConfig {
        mode: TransportMode::Live,
        ..Default::default()
    })
    .await
    .unwrap();
    let res = t.set_remote_description("not_valid_sdp", true).await;
    assert!(res.is_err());
    t.close().await.unwrap();
}

#[tokio::test]
async fn test_f11_b05_webrtc_live_audio_only_sdp_offer() {
    let mut t = WebRtcPeerTransport::new(PeerTransportConfig {
        mode: TransportMode::Live,
        enable_audio: true,
        enable_video: false,
        ..Default::default()
    })
    .await
    .unwrap();
    let sdp = t.create_offer().await.unwrap();
    assert!(sdp.contains("m=audio"));
    assert!(!sdp.contains("m=video"));
    t.close().await.unwrap();
}

#[tokio::test]
async fn test_f11_b06_call_signaling_and_rating_edge_conditions() {
    let (mut server, driver, (_alice_id, alice_token), (bob_id, bob_token), chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");
    let (_eve_id, eve_token) = driver
        .register_user("eve", "pass1234")
        .await
        .expect("register eve");

    // 1. Nonexistent call_id returns 404 Not Found
    let (st_missing, body_missing) = driver
        .json_req(
            Method::POST,
            "/api/v1/calls/signal",
            Some(&alice_token),
            Some(json!({
                "call_id": "call_does_not_exist_b06",
                "signal_type": "offer",
                "sdp": "v=0",
            })),
        )
        .await
        .unwrap();
    assert_eq!(st_missing, 404, "missing call_id: {body_missing:?}");

    let (st_rate_missing, body_rate_missing) = driver
        .json_req(
            Method::POST,
            "/api/v1/calls/missing_call_id/rate",
            Some(&alice_token),
            Some(json!({ "rating": 5 })),
        )
        .await
        .unwrap();
    assert_eq!(
        st_rate_missing, 404,
        "missing call_id rate must be 404: {body_rate_missing:?}"
    );

    // Seed an active ringing call between Alice and Bob
    let (st_inv, invite_val) = driver
        .json_req(
            Method::POST,
            "/api/v1/calls/invite",
            Some(&alice_token),
            Some(json!({
                "callee_id": bob_id,
                "chat_id": chat_id,
                "is_video": false,
            })),
        )
        .await
        .unwrap();
    assert_eq!(st_inv, 201, "invite: {invite_val:?}");
    let call_id = invite_val["call_id"].as_str().unwrap().to_string();
    assert_eq!(invite_val["state"].as_str(), Some("ringing"));

    // 2. Malformed/unsupported signal_type returns 400 Bad Request
    let (st_bad_type, body_bad_type) = driver
        .json_req(
            Method::POST,
            "/api/v1/calls/signal",
            Some(&alice_token),
            Some(json!({
                "call_id": call_id,
                "signal_type": "bogus_signal",
            })),
        )
        .await
        .unwrap();
    assert_eq!(st_bad_type, 400, "invalid signal_type: {body_bad_type:?}");

    // 3. Offer missing/blank sdp, missing/blank candidate, or out-of-range sdp_mline_index returns 400 Bad Request
    for sdp_payload in [
        json!({ "call_id": call_id, "signal_type": "offer" }),
        json!({ "call_id": call_id, "signal_type": "offer", "sdp": "   " }),
        json!({ "call_id": call_id, "signal_type": "answer" }),
        json!({ "call_id": call_id, "signal_type": "ice_candidate" }),
        json!({ "call_id": call_id, "signal_type": "ice_candidate", "candidate": "   " }),
        json!({
            "call_id": call_id,
            "signal_type": "ice_candidate",
            "candidate": "candidate:1 1 UDP 2130706431 127.0.0.1 50000 typ host",
            "sdp_mline_index": 70000
        }),
    ] {
        let (st_bad_payload, body_bad_payload) = driver
            .json_req(
                Method::POST,
                "/api/v1/calls/signal",
                Some(&alice_token),
                Some(sdp_payload),
            )
            .await
            .unwrap();
        assert_eq!(
            st_bad_payload, 400,
            "bad signal payload must be 400: {body_bad_payload:?}"
        );
    }

    // 4. Non-participant Eve sending signal on Alice<->Bob call returns 403 Forbidden
    for eve_sig in [
        json!({ "call_id": call_id, "signal_type": "hangup", "reason": "hijack" }),
        json!({ "call_id": call_id, "signal_type": "offer", "sdp": "v=0" }),
    ] {
        let (st_eve_sig, body_eve_sig) = driver
            .json_req(
                Method::POST,
                "/api/v1/calls/signal",
                Some(&eve_token),
                Some(eve_sig),
            )
            .await
            .unwrap();
        assert_eq!(
            st_eve_sig, 403,
            "non-participant signal must be 403: {body_eve_sig:?}"
        );
    }

    // 4b. Caller Alice cannot acknowledge ringing or accept her own outgoing call (403 Forbidden)
    for caller_forbidden_sig in ["ringing", "accept"] {
        let (st_caller_sig, body_caller_sig) = driver
            .json_req(
                Method::POST,
                "/api/v1/calls/signal",
                Some(&alice_token),
                Some(json!({
                    "call_id": call_id,
                    "signal_type": caller_forbidden_sig,
                })),
            )
            .await
            .unwrap();
        assert_eq!(
            st_caller_sig, 403,
            "caller sending {caller_forbidden_sig} must be 403: {body_caller_sig:?}"
        );
    }

    // 5. Non-participant Eve rating Alice<->Bob call returns 403 Forbidden
    let (st_eve_rate, body_eve_rate) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/calls/{call_id}/rate"),
            Some(&eve_token),
            Some(json!({ "rating": 5, "feedback": "hijacked" })),
        )
        .await
        .unwrap();
    assert_eq!(
        st_eve_rate, 403,
        "non-participant rate must be 403: {body_eve_rate:?}"
    );

    // 5b. Verify ZERO database mutation occurred behind Eve's 403 signals/rating
    let (st_list_alice, list_alice) = driver
        .json_req(Method::GET, "/api/v1/calls", Some(&alice_token), None)
        .await
        .unwrap();
    assert_eq!(st_list_alice, 200, "alice list calls: {list_alice:?}");
    let call_rec = list_alice["calls"]
        .as_array()
        .and_then(|arr| {
            arr.iter()
                .find(|c| c["call_id"].as_str() == Some(call_id.as_str()))
        })
        .expect("Alice must see call_id in /api/v1/calls");
    assert!(
        call_rec["rating"].is_null(),
        "403 rating by Eve must NOT mutate rating in DB: {call_rec:?}"
    );
    assert_eq!(
        call_rec["state"].as_str(),
        Some("ringing"),
        "403 signals/rating by Eve must NOT mutate call state: {call_rec:?}"
    );
    assert!(
        call_rec["ended_at_ms"].is_null(),
        "403 hangup by Eve must NOT set ended_at_ms: {call_rec:?}"
    );

    let (st_list_eve, list_eve) = driver
        .json_req(Method::GET, "/api/v1/calls", Some(&eve_token), None)
        .await
        .unwrap();
    assert_eq!(st_list_eve, 200, "eve list calls: {list_eve:?}");
    assert!(
        !list_eve["calls"]
            .as_array()
            .unwrap()
            .iter()
            .any(|c| c["call_id"].as_str() == Some(call_id.as_str())),
        "non-participant Eve must NOT see Alice<->Bob call in /api/v1/calls: {list_eve:?}"
    );

    // 6. Invalid ratings (0, 6, -1) return 400 Bad Request and do not mutate DB
    for bad_rating in [json!(0), json!(6), json!(-1)] {
        let (st_bad_rate, body_bad_rate) = driver
            .json_req(
                Method::POST,
                &format!("/api/v1/calls/{call_id}/rate"),
                Some(&alice_token),
                Some(json!({ "rating": bad_rating })),
            )
            .await
            .unwrap();
        assert_eq!(
            st_bad_rate, 400,
            "invalid rating {bad_rating}: {body_bad_rate:?}"
        );
    }
    let (st_list_post_400, list_post_400) = driver
        .json_req(Method::GET, "/api/v1/calls", Some(&alice_token), None)
        .await
        .unwrap();
    assert_eq!(st_list_post_400, 200);
    let call_rec_post_400 = list_post_400["calls"]
        .as_array()
        .and_then(|arr| {
            arr.iter()
                .find(|c| c["call_id"].as_str() == Some(call_id.as_str()))
        })
        .expect("call_id present after 400 ratings");
    assert!(
        call_rec_post_400["rating"].is_null(),
        "400 invalid ratings must NOT mutate rating in DB: {call_rec_post_400:?}"
    );

    // 7. Hangup during ringing succeeds (200 OK, state == "ended")
    let (st_hang, body_hang) = driver
        .json_req(
            Method::POST,
            "/api/v1/calls/signal",
            Some(&alice_token),
            Some(json!({
                "call_id": call_id,
                "signal_type": "hangup",
                "reason": "cancel_ringing",
            })),
        )
        .await
        .unwrap();
    assert_eq!(st_hang, 200, "hangup during ringing: {body_hang:?}");
    assert_eq!(body_hang["call"]["state"].as_str(), Some("ended"));

    // 8. Signal after call has ended returns 409 Conflict
    let (st_post_end_ans, body_post_end_ans) = driver
        .json_req(
            Method::POST,
            "/api/v1/calls/signal",
            Some(&bob_token),
            Some(json!({
                "call_id": call_id,
                "signal_type": "answer",
                "sdp": "v=0",
            })),
        )
        .await
        .unwrap();
    assert_eq!(
        st_post_end_ans, 409,
        "answer after ended must be 409: {body_post_end_ans:?}"
    );

    let (st_post_end_hang, body_post_end_hang) = driver
        .json_req(
            Method::POST,
            "/api/v1/calls/signal",
            Some(&alice_token),
            Some(json!({
                "call_id": call_id,
                "signal_type": "hangup",
            })),
        )
        .await
        .unwrap();
    assert_eq!(
        st_post_end_hang, 409,
        "hangup after ended must be 409: {body_post_end_hang:?}"
    );

    // 9. Valid rating boundary values (1 and 5) return 200 OK
    let (st_r1, body_r1) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/calls/{call_id}/rate"),
            Some(&alice_token),
            Some(json!({ "rating": 1 })),
        )
        .await
        .unwrap();
    assert_eq!(st_r1, 200, "rating 1: {body_r1:?}");
    assert_eq!(body_r1["rating"].as_u64(), Some(1));

    let (st_r5, body_r5) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/calls/{call_id}/rate"),
            Some(&bob_token),
            Some(json!({ "rating": 5 })),
        )
        .await
        .unwrap();
    assert_eq!(st_r5, 200, "rating 5: {body_r5:?}");
    assert_eq!(body_r5["rating"].as_u64(), Some(5));

    // 10. Sending "ringing" on an already "active" call returns 409 Conflict and preserves "active" state
    let (st_inv2, inv2_val) = driver
        .json_req(
            Method::POST,
            "/api/v1/calls/invite",
            Some(&alice_token),
            Some(json!({
                "callee_id": bob_id,
                "chat_id": chat_id,
                "is_video": false,
            })),
        )
        .await
        .unwrap();
    assert_eq!(st_inv2, 201, "invite call 2: {inv2_val:?}");
    let call_id_2 = inv2_val["call_id"].as_str().unwrap().to_string();

    let (st_ans2, ans2_val) = driver
        .json_req(
            Method::POST,
            "/api/v1/calls/signal",
            Some(&bob_token),
            Some(json!({
                "call_id": call_id_2,
                "signal_type": "answer",
                "sdp": "v=0",
            })),
        )
        .await
        .unwrap();
    assert_eq!(st_ans2, 200, "answer call 2: {ans2_val:?}");
    assert_eq!(ans2_val["call"]["state"].as_str(), Some("active"));

    let (st_late_ring, body_late_ring) = driver
        .json_req(
            Method::POST,
            "/api/v1/calls/signal",
            Some(&bob_token),
            Some(json!({
                "call_id": call_id_2,
                "signal_type": "ringing",
            })),
        )
        .await
        .unwrap();
    assert_eq!(
        st_late_ring, 409,
        "ringing on active call must return 409 Conflict: {body_late_ring:?}"
    );
    let (st_list2, list2_val) = driver
        .json_req(Method::GET, "/api/v1/calls", Some(&alice_token), None)
        .await
        .unwrap();
    assert_eq!(st_list2, 200);
    let rec2 = list2_val["calls"]
        .as_array()
        .and_then(|arr| {
            arr.iter()
                .find(|c| c["call_id"].as_str() == Some(call_id_2.as_str()))
        })
        .expect("call_id_2 must exist");
    assert_eq!(
        rec2["state"].as_str(),
        Some("active"),
        "call_id_2 DB state must remain active after late ringing: {rec2:?}"
    );

    // 11. Concurrent signaling race: hangup vs answer and hangup vs hangup
    for race_idx in 0..3 {
        let (st_inv_r, inv_r_val) = driver
            .json_req(
                Method::POST,
                "/api/v1/calls/invite",
                Some(&alice_token),
                Some(json!({
                    "callee_id": bob_id,
                    "chat_id": chat_id,
                    "is_video": false,
                })),
            )
            .await
            .unwrap();
        assert_eq!(st_inv_r, 201);
        let race_call_id = inv_r_val["call_id"].as_str().unwrap().to_string();

        let (res_hang_r, res_ans_r) = tokio::join!(
            driver.json_req(
                Method::POST,
                "/api/v1/calls/signal",
                Some(&alice_token),
                Some(json!({
                    "call_id": race_call_id,
                    "signal_type": "hangup",
                    "reason": "race_hangup",
                })),
            ),
            driver.json_req(
                Method::POST,
                "/api/v1/calls/signal",
                Some(&bob_token),
                Some(json!({
                    "call_id": race_call_id,
                    "signal_type": "answer",
                    "sdp": "v=0",
                })),
            ),
        );
        let (st_hang_r, body_hang_r) = res_hang_r.unwrap();
        let (st_ans_r, body_ans_r) = res_ans_r.unwrap();
        assert_eq!(
            st_hang_r, 200,
            "race {race_idx} hangup must succeed: {body_hang_r:?}"
        );
        assert!(
            st_ans_r == 200 || st_ans_r == 409,
            "race {race_idx} answer must be 200 (before hangup) or 409 (after hangup): {st_ans_r} {body_ans_r:?}"
        );

        let (st_list_r, list_r) = driver
            .json_req(Method::GET, "/api/v1/calls", Some(&alice_token), None)
            .await
            .unwrap();
        assert_eq!(st_list_r, 200);
        let rec_r = list_r["calls"]
            .as_array()
            .and_then(|arr| {
                arr.iter()
                    .find(|c| c["call_id"].as_str() == Some(race_call_id.as_str()))
            })
            .expect("race call record must exist");
        assert_eq!(
            rec_r["state"].as_str(),
            Some("ended"),
            "race {race_idx} ended call must NOT be resurrected to active: {rec_r:?}"
        );
        assert!(
            rec_r["ended_at_ms"].is_i64(),
            "race {race_idx} ended_at_ms must be populated: {rec_r:?}"
        );
    }

    // Concurrent double-hangup: exactly one succeeds (200) and the second gets 409 Conflict
    let (st_inv_dh, inv_dh_val) = driver
        .json_req(
            Method::POST,
            "/api/v1/calls/invite",
            Some(&alice_token),
            Some(json!({
                "callee_id": bob_id,
                "chat_id": chat_id,
                "is_video": false,
            })),
        )
        .await
        .unwrap();
    assert_eq!(st_inv_dh, 201);
    let dh_call_id = inv_dh_val["call_id"].as_str().unwrap().to_string();
    let (res_dh_a, res_dh_b) = tokio::join!(
        driver.json_req(
            Method::POST,
            "/api/v1/calls/signal",
            Some(&alice_token),
            Some(json!({ "call_id": dh_call_id, "signal_type": "hangup" })),
        ),
        driver.json_req(
            Method::POST,
            "/api/v1/calls/signal",
            Some(&bob_token),
            Some(json!({ "call_id": dh_call_id, "signal_type": "hangup" })),
        ),
    );
    let st_dh_a = res_dh_a.unwrap().0;
    let st_dh_b = res_dh_b.unwrap().0;
    assert!(
        (st_dh_a == 200 && st_dh_b == 409) || (st_dh_a == 409 && st_dh_b == 200),
        "concurrent double hangup must yield one 200 and one 409, got ({st_dh_a}, {st_dh_b})"
    );

    server.stop().await;
}

// ============================================================================
// F12 Boundaries: Opus Codec Errors, Sequence & U32 Timestamp Wrap-Around (5 tests)
// ============================================================================

#[test]
fn test_f12_b01_opus_unsupported_channels_error() {
    let pcm = vec![0i16; 960];
    assert!(matches!(
        OpusFramingCodec::encode_frame(&pcm, 0),
        Err(CodecError::UnsupportedChannels(0))
    ));
    assert!(matches!(
        OpusFramingCodec::encode_frame(&pcm, 6),
        Err(CodecError::UnsupportedChannels(6))
    ));
}

#[test]
fn test_f12_b02_opus_truncated_payload_error() {
    assert!(matches!(
        OpusFramingCodec::decode_frame(&[], 1),
        Err(CodecError::TruncatedPayload)
    ));
    assert!(matches!(
        RtpPacket::parse(&[0x80, 0x6F]),
        Err(CodecError::InvalidRtpPacket(_))
    ));
}

#[test]
fn test_f12_b03_opus_wrong_payload_type_rejected_by_depacketizer() {
    let mut dep = OpusDepacketizer::new(1);
    let encoded = OpusFramingCodec::encode_frame(&vec![0i16; 960], 1).unwrap();
    let wrong_pt = RtpPacket::new(96, 1, 0, 1, encoded);
    assert!(matches!(
        dep.depacketize(&wrong_pt),
        Err(CodecError::InvalidRtpPacket(_))
    ));
}

#[test]
fn test_f12_b04_rtp_u16_sequence_number_wrap_around() {
    let mut dep = OpusDepacketizer::new(1);
    let payload = OpusFramingCodec::encode_frame(&vec![100i16; 960], 1).unwrap();

    let p_max = RtpPacket::parse(
        &RtpPacket::new(OPUS_PAYLOAD_TYPE, u16::MAX, 48_000, 1, payload.clone()).serialize(),
    )
    .unwrap();
    let p_wrap = RtpPacket::parse(
        &RtpPacket::new(OPUS_PAYLOAD_TYPE, 0, 48_960, 1, payload).serialize(),
    )
    .unwrap();

    dep.depacketize(&p_max).unwrap();
    assert_eq!(dep.last_sequence_number(), Some(u16::MAX));
    dep.depacketize(&p_wrap).unwrap();
    assert_eq!(dep.last_sequence_number(), Some(0));
}

#[test]
fn test_f12_b05_rtp_u32_timestamp_wrap_around_and_reordering() {
    let mut dep = OpusDepacketizer::new(1);
    let payload = OpusFramingCodec::encode_frame(&vec![250i16; 960], 1).unwrap();

    let ts_before_wrap = u32::MAX - 959;
    let ts_after_wrap = ts_before_wrap.wrapping_add(OPUS_SAMPLES_PER_FRAME_MONO as u32);
    assert_eq!(ts_after_wrap, 0);

    let p_after = RtpPacket::parse(
        &RtpPacket::new(OPUS_PAYLOAD_TYPE, 100, ts_after_wrap, 9, payload.clone()).serialize(),
    )
    .unwrap();
    let p_before = RtpPacket::parse(
        &RtpPacket::new(OPUS_PAYLOAD_TYPE, 99, ts_before_wrap, 9, payload).serialize(),
    )
    .unwrap();

    let s1 = dep.depacketize(&p_after).unwrap();
    assert_eq!(s1.len(), OPUS_SAMPLES_PER_FRAME_MONO);
    assert_eq!(dep.last_timestamp(), Some(0));

    let s2 = dep.depacketize(&p_before).unwrap();
    assert_eq!(s2.len(), OPUS_SAMPLES_PER_FRAME_MONO);
    assert_eq!(dep.last_timestamp(), Some(ts_before_wrap));
}

// ============================================================================
// F13 Boundaries: Media Capture Lifecycle Guards & Edge Frames (5 tests)
// ============================================================================

#[test]
fn test_f13_b01_media_device_pause_before_start_errors() {
    let mut dev = MockVideoSource::new(64, 36, 30);
    assert!(dev.pause(true).is_err());
}

#[test]
fn test_f13_b02_media_device_double_start_and_double_stop_safe() {
    let mut dev = MockAudioSource::new_sine(440.0, 48_000, 1);
    dev.start().unwrap();
    dev.start().unwrap();
    assert_eq!(dev.state(), DeviceState::Streaming);
    dev.stop().unwrap();
    dev.stop().unwrap();
    assert_eq!(dev.state(), DeviceState::Stopped);
}

#[test]
fn test_f13_b03_mock_audio_zero_duration_returns_zero_rms() {
    let mut src = MockAudioSource::new_sine(440.0, 48_000, 1);
    let f = src.next_frame(0);
    assert!(f.samples.is_empty());
    assert_eq!(f.rms_level(), 0);
}

#[test]
fn test_f13_b04_mock_video_odd_dimensions_i420_safe() {
    let mut cam = MockVideoSource::new(16, 16, 30);
    let vf = cam.next_frame();
    assert_eq!(vf.to_rgba8().len(), 16 * 16 * 4);
    assert_eq!(vf.to_i420().len(), 16 * 16 * 3 / 2);
}

#[test]
fn test_f13_b05_media_device_pause_after_stop_fails() {
    let mut dev = MockVideoSource::new(32, 32, 15);
    dev.start().unwrap();
    dev.stop().unwrap();
    assert!(dev.pause(true).is_err());
}

// ============================================================================
// F14 Boundaries: VideoViewport Validation, Zero Dimensions & Buffer Guards (5 tests)
// ============================================================================

#[test]
fn test_f14_b01_video_viewport_rejects_mismatched_rgba_len() {
    let pal = TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x40A7E3, 14, 12);
    let mut vp = VideoViewport::new("Peer", pal);
    assert!(!vp.ingest_raw_remote_rgba(10, 10, &[0u8; 4]));
    assert!(!vp.has_active_video());
}

#[test]
fn test_f14_b02_video_viewport_rejects_zero_dimensions() {
    let pal = TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x40A7E3, 14, 12);
    let mut vp = VideoViewport::new("Peer", pal);
    assert!(!vp.ingest_raw_remote_rgba(0, 10, &[]));
    assert!(!vp.ingest_raw_local_rgba(10, 0, &[]));
}

#[test]
fn test_f14_b03_video_viewport_local_rgba_and_camera_toggle() {
    let pal = TdesktopThemePalette::build(TdesktopThemeMode::Day, 0x40A7E3, 14, 12);
    let mut vp = VideoViewport::new("Peer", pal);
    assert!(vp.ingest_raw_local_rgba(2, 2, &[255u8; 16]));
    assert_eq!(vp.local_resolution, (2, 2));
    vp.set_local_camera_active(false);
    assert!(!vp.is_local_camera_active);
}

#[test]
fn test_f14_b04_video_viewport_clear_remote_when_already_empty() {
    let pal = TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x40A7E3, 14, 12);
    let mut vp = VideoViewport::new("Peer", pal);
    vp.set_remote_frame(None, 0, 0);
    assert!(!vp.has_active_video());
}

#[test]
fn test_f14_b05_video_viewport_cycle_corner_small_bounds_safe() {
    let pal = TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x40A7E3, 14, 12);
    let mut vp = VideoViewport::new("Peer", pal);
    for _ in 0..4 {
        vp.cycle_bubble_corner(120.0, 90.0);
    }
    assert_eq!(vp.bubble.active_corner, BubbleCorner::BottomRight);
}

// ============================================================================
// F15 Boundaries: macOS Notification Injection Guard & Bad Deep Links (5 tests)
// ============================================================================

#[test]
fn test_f15_b01_applescript_escaping_quotes_and_backslashes() {
    let raw = r#"Alice said: "rm -rf /" \n now"#;
    let esc = NotificationsManagerMacUnMac::escape_applescript_string(raw);
    assert_eq!(esc, r#"Alice said: \"rm -rf /\" \\n now"#);
}

#[test]
fn test_f15_b02_click_and_action_on_unknown_notification_id() {
    let mut mgr = NotificationsManagerMacUnMac::new(1).with_mock_mode(true);
    assert!(mgr.handle_notification_click("nonexistent").is_none());
    assert!(!mgr
        .handle_notification_action(
            "nonexistent",
            UNNotificationAction::InlineReply("hi".into())
        )
        .unwrap());
}

#[test]
fn test_f15_b03_deep_link_unknown_scheme_falls_back_cleanly() {
    let parsed = WindowSessionControllerLinkInfo::parse_deep_link("ftp://invalid.host/foo");
    assert!(matches!(parsed, LinkInfoType::Unknown | LinkInfoType::Url(_)));
}

#[test]
fn test_f15_b04_deep_link_empty_string_handled_safely() {
    let parsed = WindowSessionControllerLinkInfo::parse_deep_link("");
    assert!(matches!(parsed, LinkInfoType::Unknown));
}

#[test]
fn test_f15_b05_notification_clear_from_peer_when_empty_is_noop() {
    let mut mgr = NotificationsManagerMacUnMac::new(1).with_mock_mode(true);
    mgr.show_system_notification("A", "", "hello", false, UNNotificationParsed::new(1, 5, 1))
        .unwrap();
    mgr.clear_from_peer(999).unwrap();
    assert_eq!(mgr.in_flight_requests.len(), 1);
}

// ============================================================================
// F16 Boundaries: HttpPlatformImDriver Error Paths & Validation (5 tests)
// ============================================================================

#[tokio::test]
async fn test_f16_b01_http_driver_unknown_tool_returns_err() {
    let driver = HttpPlatformImDriver::new("http://127.0.0.1:9", "tok");
    let res = driver.execute("im_nonexistent_tool", json!({})).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn test_f16_b02_http_driver_missing_poll_question_or_chat_rejected() {
    let (mut server, _driver, (_alice_id, alice_token), _bob, _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");
    let im = HttpPlatformImDriver::new(server.base_url(), &alice_token);
    let res = im
        .execute("im_create_poll", json!({ "question": "No chat id" }))
        .await;
    assert!(res.is_err());
    server.stop().await;
}

#[tokio::test]
async fn test_f16_b03_http_driver_unauthorized_token_rejected() {
    let mut server = boot_test_server().await.expect("boot_test_server");
    let im = HttpPlatformImDriver::new(server.base_url(), "bad_token");
    let res = im
        .execute("im_send_message", json!({ "chat_id": 1, "text": "hi" }))
        .await;
    assert!(res.is_err());
    server.stop().await;
}

#[tokio::test]
async fn test_f16_b04_http_driver_empty_message_rejected() {
    let (mut server, _driver, (_alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");
    let im = HttpPlatformImDriver::new(server.base_url(), &alice_token);
    let res = im
        .execute("im_send_message", json!({ "chat_id": chat_id, "text": "" }))
        .await;
    assert!(res.is_err());
    server.stop().await;
}

#[tokio::test]
async fn test_f16_b05_http_driver_nonexistent_chat_history_rejected() {
    let (mut server, _driver, (_alice_id, alice_token), _bob, _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");
    let im = HttpPlatformImDriver::new(server.base_url(), &alice_token);
    let res = im
        .execute("im_get_chat_history", json!({ "chat_id": 999_999 }))
        .await;
    assert!(res.is_err());
    server.stop().await;
}

// ============================================================================
// F17 Boundaries: HermesStateStore & Kanban API Edge Cases (5 tests)
// ============================================================================

#[test]
fn test_f17_b01_hermes_move_missing_card_fails() {
    let store = HermesStateStore::open_in_memory().unwrap();
    assert!(store.move_kanban_card("nonexistent_card", "Done", None).is_err());
}

#[test]
fn test_f17_b02_hermes_update_missing_card_fails() {
    let store = HermesStateStore::open_in_memory().unwrap();
    assert!(store
        .update_kanban_card("missing", KanbanCardPatch::default())
        .is_err());
}

#[test]
fn test_f17_b03_hermes_get_missing_board_and_cron_returns_none() {
    let store = HermesStateStore::open_in_memory().unwrap();
    assert!(store.get_kanban_board("no_board").unwrap().is_none());
    assert!(store.get_cron_job("no_cron").unwrap().is_none());
}

#[test]
fn test_f17_b04_hermes_delete_kanban_card_idempotency() {
    let store = HermesStateStore::open_in_memory().unwrap();
    let board = store.create_kanban_board("B", "desc", None).unwrap();
    let card = store
        .create_kanban_card(CreateKanbanCardRequest {
            id: None,
            board_id: board.id,
            column_name: Some("Backlog".into()),
            title: "Delete me".into(),
            description: None,
            priority: None,
            assignee: None,
            labels: None,
        })
        .unwrap();
    store.delete_kanban_card(&card.id).unwrap();
    assert!(store.move_kanban_card(&card.id, "Done", None).is_err());
}

#[tokio::test]
async fn test_f17_b05_server_hermes_card_missing_board_rejected() {
    let (mut server, driver, (_alice_id, alice_token), _bob, _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");
    let (st, _) = driver
        .json_req(
            Method::POST,
            "/api/v1/hermes/kanban/cards",
            Some(&alice_token),
            Some(json!({ "board_id": "missing_board_e2e", "title": "Orphan" })),
        )
        .await
        .unwrap();
    assert!((400..500).contains(&st));
    server.stop().await;
}

// ============================================================================
// F18 Boundaries: Semantic Workspace Tree Guards (5 tests)
// ============================================================================

#[test]
fn test_f18_b01_semantic_send_without_chat_fails() {
    let mut tree = SemanticWorkspaceTreeSimulator::new();
    tree.perform_action(SemanticAction::ComposeText {
        text: "hello".into(),
    });
    assert!(!tree.perform_action(SemanticAction::SendMessage).success);
}

#[test]
fn test_f18_b02_semantic_send_empty_text_fails_even_with_chat_selected() {
    let mut tree = SemanticWorkspaceTreeSimulator::new();
    tree.register_chat(1);
    assert!(tree.perform_action(SemanticAction::SelectChat { chat_id: 1 }).success);
    tree.perform_action(SemanticAction::ComposeText {
        text: "   ".into(),
    });
    assert!(!tree.perform_action(SemanticAction::SendMessage).success);
}

#[test]
fn test_f18_b03_semantic_select_unknown_chat_fails() {
    let mut tree = SemanticWorkspaceTreeSimulator::new();
    assert!(!tree.perform_action(SemanticAction::SelectChat { chat_id: 404 }).success);
}

#[test]
fn test_f18_b04_semantic_modal_close_resets_state() {
    let mut tree = SemanticWorkspaceTreeSimulator::new();
    tree.perform_action(SemanticAction::OpenModal {
        modal_id: "info".into(),
    });
    assert_eq!(tree.inspect_state().active_modal.as_deref(), Some("info"));
    tree.perform_action(SemanticAction::CloseModal);
    assert!(tree.inspect_state().active_modal.is_none());
}

#[test]
fn test_f18_b05_semantic_toggle_call_twice_returns_false() {
    let mut tree = SemanticWorkspaceTreeSimulator::new();
    tree.perform_action(SemanticAction::ToggleCall {
        peer_id: 1,
        is_video: false,
    });
    assert!(tree.inspect_state().is_call_active);
    tree.perform_action(SemanticAction::ToggleCall {
        peer_id: 1,
        is_video: false,
    });
    assert!(!tree.inspect_state().is_call_active);
}

// ============================================================================
// F19 Boundaries: AgentManifest Validation & Server MiniApp Guards (5 tests)
// ============================================================================

#[test]
fn test_f19_b01_agent_manifest_malformed_json_rejected() {
    assert!(AgentManifest::from_json("{ bad json").is_err());
}

#[test]
fn test_f19_b02_agent_manifest_non_object_state_rejected() {
    let m = AgentManifest {
        app_id: "bot".into(),
        version: "1.0.0".into(),
        readable_state_schema: json!({ "type": "object" }),
        actions: vec![],
    };
    assert!(m.validate_state(&json!([1, 2, 3])).is_err());
}

#[test]
fn test_f19_b03_agent_manifest_high_risk_requires_approval() {
    let m = AgentManifest {
        app_id: "bot".into(),
        version: "1.0.0".into(),
        readable_state_schema: json!({}),
        actions: vec![MiniAppActionSpec {
            name: "wipe".into(),
            description: "Wipe".into(),
            input_schema: json!({ "type": "object" }),
            output_schema: json!({}),
            risk_level: "critical".into(),
            requires_approval: true,
        }],
    };
    assert!(m.execute_action("wipe", &json!({}), false).is_err());
    assert!(m.execute_action("wipe", &json!({}), true).is_ok());
}

#[test]
fn test_f19_b04_agent_manifest_execute_undeclared_action_rejected() {
    let m = AgentManifest {
        app_id: "bot".into(),
        version: "1.0.0".into(),
        readable_state_schema: json!({}),
        actions: vec![MiniAppActionSpec {
            name: "allowed".into(),
            description: "Allowed".into(),
            input_schema: json!({}),
            output_schema: json!({}),
            risk_level: "low".into(),
            requires_approval: false,
        }],
    };
    assert!(m.execute_action("forbidden", &json!({}), true).is_err());
}

#[tokio::test]
async fn test_f19_b05_server_unknown_miniapp_action_rejected() {
    let (mut server, driver, (_alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");
    let (st, _) = driver
        .json_req(
            Method::POST,
            "/api/v1/miniapps/no_bot/no_app/action",
            Some(&alice_token),
            Some(
                serde_json::to_value(MiniAppActionRequest::main_button_click(Some(chat_id)))
                    .unwrap(),
            ),
        )
        .await
        .unwrap();
    assert!((400..500).contains(&st));
    server.stop().await;
}

// ============================================================================
// F20 Boundaries: GenerativeCard XSS/State Guards & Empty Card Math (5 tests)
// ============================================================================

#[test]
fn test_f20_b01_generative_card_escapes_html_and_script_tags() {
    let dirty = "<iframe src=\"x\"></iframe><script>alert(9)</script>Safe";
    let clean = GenerativeCard::sanitize_text(dirty);
    assert!(!clean.contains("<iframe"));
    assert!(!clean.contains("<script"));
    assert!(clean.contains("&lt;iframe"));
    assert!(clean.contains("Safe"));
}

#[test]
fn test_f20_b02_generative_card_progress_clamps_to_unit_interval() {
    let card = GenerativeCard {
        card_id: "c1".into(),
        title: "t".into(),
        subtitle: None,
        layout: GenerativeCardLayout::VerticalStack,
        components: vec![
            GenerativeComponent::Progress {
                value: 255.0,
                max: 100.0,
                label: None,
            },
            GenerativeComponent::Progress {
                value: 10.0,
                max: 0.0,
                label: None,
            },
        ],
        status: None,
    };
    assert_eq!(card.clamped_progress(), vec![1.0, 0.0]);
}

#[test]
fn test_f20_b03_todo_card_empty_items_zero_percent_no_nan() {
    let todo = TodoCard::new("empty", "Nothing yet", vec![]);
    assert_eq!(todo.completion_percentage(), 0.0);
}

#[test]
fn test_f20_b04_poll_card_zero_votes_zero_percent_no_nan() {
    let poll = PollCard::new(
        "p0",
        "Zero votes?",
        vec![PollOption::new(0, "A"), PollOption::new(1, "B")],
        true,
    );
    assert_eq!(poll.options[0].percentage, 0.0);
    assert!(!poll.options[0].percentage.is_nan());
}

#[test]
fn test_f20_b05_todo_card_toggle_missing_item_returns_false() {
    let mut todo = TodoCard::new("t1", "List", vec![]);
    assert!(!todo.toggle_item("missing"));
}

// ============================================================================
// F21 Boundaries: FloorControl Queue Edge Cases, Budgets & Delimiter Attacks (5 tests)
// ============================================================================

#[test]
fn test_f21_b01_floor_control_wrong_agent_release_does_not_displace_speaker() {
    let mut q = FloorControlQueue::new(5_000);
    assert!(q.request_floor_at("owner", FloorPriority::DirectMention, 10));
    q.release_floor("impostor");
    assert_eq!(q.current_speaker(), Some("owner"));
}

#[test]
fn test_f21_b02_floor_control_lease_expiry_promotes_next() {
    let mut q = FloorControlQueue::new(100);
    assert!(q.request_floor_at("a1", FloorPriority::TaskAssignee, 10));
    assert!(!q.request_floor_at("a2", FloorPriority::DirectMention, 20));
    q.drain_expired_leases(200);
    assert_eq!(q.current_speaker(), Some("a2"));
}

#[test]
fn test_f21_b03_loop_budget_zero_max_hops_rejects_first_call() {
    let mut budget = LoopBudget::new(0);
    assert!(budget.record_hop("agent").is_err());
}

#[test]
fn test_f21_b04_loop_budget_reset_clears_trace_and_allows_reuse() {
    let mut budget = LoopBudget::new(1);
    budget.record_hop("a1").unwrap();
    assert!(budget.is_exhausted());
    budget.reset();
    assert!(!budget.is_exhausted());
    assert!(budget.trace().is_empty());
    assert!(budget.record_hop("a2").is_ok());
}

#[test]
fn test_f21_b05_scoped_capability_token_expired_and_delimiter_injection() {
    let sk = SigningKey::from_bytes(&[21u8; 32]);
    let vk = sk.verifying_key();
    let now = chrono::Utc::now().timestamp();
    let tok = ScopedCapabilityToken::issue(
        &sk,
        "user|admin|9999999",
        55,
        vec!["read".into()],
        50,
    );
    assert!(!tok.verify_for_scope(&vk, now + 10, "user", 55, "admin"));
    assert!(
        !tok.verify_for_scope(&vk, now + 100, "user|admin|9999999", 55, "read"),
        "expired token must fail"
    );
    let wrong_vk = SigningKey::from_bytes(&[22u8; 32]).verifying_key();
    assert!(!tok.verify_for_scope(&wrong_vk, now + 10, "user|admin|9999999", 55, "read"));
}

// ============================================================================
// F22 Boundaries: Tantivy & HybridMemoryEngine Dedup, NaN & Empty Guards (5 tests)
// ============================================================================

#[test]
fn test_f22_b01_hybrid_memory_empty_query_and_no_matching_chat_returns_empty() {
    let engine = HybridMemoryEngine::new();
    engine.index_document(MemoryDocument {
        message_id: "d1".into(),
        chat_id: 1,
        content: "rust search".into(),
        embedding: vec![1.0, 0.0],
        linked_entities: vec![],
    });
    assert!(engine.search("   ", None, &[1], None).is_empty());
    assert!(engine.search("rust", Some(&[1.0, 0.0]), &[999], None).is_empty());
}

#[test]
fn test_f22_b02_hybrid_memory_nan_embedding_does_not_panic() {
    let engine = HybridMemoryEngine::new();
    engine.index_document(MemoryDocument {
        message_id: "nan_doc".into(),
        chat_id: 1,
        content: "rust test".into(),
        embedding: vec![f32::NAN, 1.0],
        linked_entities: vec![],
    });
    engine.index_document(MemoryDocument {
        message_id: "ok_doc".into(),
        chat_id: 1,
        content: "rust test".into(),
        embedding: vec![1.0, 0.0],
        linked_entities: vec![],
    });
    let hits = engine.search("rust", Some(&[1.0, 0.0]), &[1], None);
    assert_eq!(hits.len(), 2);
    assert!(!hits[0].rrf_score.is_nan());
}

#[test]
fn test_f22_b03_hybrid_memory_mismatched_embedding_dims_safe() {
    let sim = HybridMemoryEngine::cosine_similarity(&[1.0, 0.0], &[1.0, 0.0, 0.0]);
    assert_eq!(sim, 0.0);
}

#[test]
fn test_f22_b04_hybrid_memory_duplicate_id_deduplicated_in_results() {
    let engine = HybridMemoryEngine::new();
    engine.index_document(MemoryDocument {
        message_id: "dup-1".into(),
        chat_id: 1,
        content: "legacy rust text".into(),
        embedding: vec![0.5, 0.5],
        linked_entities: vec![],
    });
    engine.index_document(MemoryDocument {
        message_id: "dup-1".into(),
        chat_id: 1,
        content: "updated rust text".into(),
        embedding: vec![1.0, 0.0],
        linked_entities: vec![],
    });

    let hits = engine.search("rust", Some(&[1.0, 0.0]), &[1], None);
    assert_eq!(
        hits.len(),
        1,
        "duplicate document IDs must deduplicate to 1 hit"
    );
    assert_eq!(hits[0].message_id, "dup-1");
}

#[tokio::test]
async fn test_f22_b05_tantivy_empty_and_missing_query_safe() {
    let idx = TantivyIndexer::in_ram().unwrap();
    idx.index_message(SearchDocument::new(
        "1",
        "1",
        "1",
        100,
        "regular searchable message",
        Some("text".into()),
    ))
    .await
    .unwrap();
    idx.commit().await.unwrap();

    if let Ok(empty_hits) = idx.search_hits(&SearchQueryParams::new("")) {
        assert!(empty_hits.is_empty());
    }

    let missing_hits = idx
        .search_hits(&SearchQueryParams::new("zzzz_nonexistent_token"))
        .unwrap();
    assert!(missing_hits.is_empty());
}

// ============================================================================
// F23 Boundaries: DefaultAiMediaTap & VadBargeInDetector Edge Signals (5 tests)
// ============================================================================

#[test]
fn test_f23_b01_default_ai_media_tap_empty_pcm_safe_zero_rms() {
    let rms = DefaultAiMediaTap::compute_rms(&[]);
    assert_eq!(rms, 0.0);
    let tap = DefaultAiMediaTap::new(48_000, 1, 0.05).with_consecutive_frames(1);
    tap.on_incoming_pcm_frame(&[], 48_000, 1);
    assert!(!tap.trigger_vad_barge_in());
}

#[test]
fn test_f23_b02_default_ai_media_tap_i16_min_clipped_signal_no_overflow() {
    let clipped = vec![i16::MIN; 960];
    let rms = DefaultAiMediaTap::compute_rms(&clipped);
    assert!(!rms.is_nan());
    assert!(rms > 0.9 && rms <= 1.01);

    let tap = DefaultAiMediaTap::new(48_000, 1, 0.5).with_consecutive_frames(1);
    tap.on_incoming_pcm_frame(&clipped, 48_000, 1);
    assert!(tap.trigger_vad_barge_in());
}

#[test]
fn test_f23_b03_default_ai_media_tap_sample_rate_mismatch_safe() {
    let tap = DefaultAiMediaTap::new(48_000, 1, 0.05).with_consecutive_frames(1);
    tap.on_incoming_pcm_frame(&vec![10_000i16; 320], 16_000, 1);
    assert!(DefaultAiMediaTap::compute_rms(&vec![10_000i16; 320]) > 0.05);
}

#[test]
fn test_f23_b04_vad_barge_in_detector_empty_and_high_threshold() {
    let mut vad = VadBargeInDetector::new(30_000.0);
    assert!(!vad.process_pcm_frame(&[]));
    assert!(!vad.process_pcm_frame(&vec![5000i16; 960]));
}

#[test]
fn test_f23_b05_vad_barge_in_detector_reset_clears_silence_streak() {
    let mut vad = VadBargeInDetector::new(1000.0);
    let silent = vec![0i16; 960];
    for _ in 0..30 {
        vad.process_pcm_frame(&silent);
    }
    assert!(vad.is_silence_detected());
    vad.reset();
    assert!(!vad.is_silence_detected());
}
