//! Tier 5 Adversarial Coverage Hardening — Core Protocol, Storage, Gateway, Outbox, CAS, E2EE & Proxy
//! Exercises production structs directly (zero simulators).

use axum::{
    body::{to_bytes, Body},
    http::{header, Request, StatusCode},
};
use gpui_telegram_client::e2ee::keys::{
    b64_encode, random_32, x25519_pub_from_secret, DeviceIdentityKeys,
};
use gpui_telegram_client::e2ee::x3dh::{x3dh_initiator, x3dh_responder};
use gpui_telegram_client::e2ee::{DoubleRatchetSession, E2eeError};
use gpui_telegram_client::outbox::DEFAULT_OUTBOX_MAX_RETRIES;
use gpui_telegram_client::{
    AuthResponse, ClientConfig, ClientProxyConfig, ClientProxyType, HttpClient, OutboxStatus,
    RichSendMessageRequest, TelegramClient, TransferStatus,
};
use gpui_telegram_core::action_bus::export::{
    CliExporter, HermesToolExporter, McpExporter, OpenApiExporter,
};
use gpui_telegram_core::action_bus::{
    ActionContext, ActionRegistry, DeleteMessageAction, GetChatHistoryAction, ListDialogsAction,
    SendMessageAction,
};
use gpui_telegram_core::error::TelegramError;
use gpui_telegram_agent::hermes::toolsets::PlatformImCaller;
use gpui_telegram_core::types::{ChatId, MessageId, UserId};
use gpui_telegram_proto::e2ee::{
    DevicePreKeyBundle, PublishPreKeysRequest, SignedPreKeyDto, UserPreKeyBundlesResponse,
};
use gpui_telegram_proto::message::MessageContent;
use gpui_telegram_proto::ws::{WsClientFrame, WsServerFrame};
use gpui_telegram_server::bus::{CrossNodeEventBus, InProcessEventBus, RedisEventBusStub};
use gpui_telegram_server::db::SqlitePool;
use gpui_telegram_server::media::CasStore;
use gpui_telegram_server::server::AppState;
use gpui_telegram_server::ws::GatewayHub;
use gpui_telegram_server::{build_router_with_state, ServerConfig};
use parking_lot::Mutex;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Barrier};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tower::ServiceExt;

fn make_auth(user_id: u64, username: &str, token: &str) -> AuthResponse {
    AuthResponse {
        user_id,
        username: username.to_string(),
        first_name: Some(username.to_string()),
        token: token.to_string(),
        session_id: format!("sess_{user_id}"),
        created_at: None,
        expires_at: None,
    }
}

fn open_ephemeral_server() -> (tempfile::TempDir, Arc<AppState>, axum::Router) {
    let dir = tempfile::tempdir().unwrap();
    let cfg = ServerConfig {
        database_url: dir.path().join("srv.db").to_string_lossy().into_owned(),
        media_storage_path: dir.path().join("cas").to_string_lossy().into_owned(),
        ..Default::default()
    };
    let state = Arc::new(AppState::try_new(cfg).unwrap());
    let app = build_router_with_state(state.clone());
    (dir, state, app)
}

async fn register_user(app: &axum::Router, username: &str) -> (u64, String) {
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/register")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "username": username,
                        "password": "Password123!",
                        "first_name": username
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);
    let v: Value =
        serde_json::from_slice(&to_bytes(res.into_body(), usize::MAX).await.unwrap()).unwrap();
    (
        v["user_id"].as_u64().unwrap(),
        v["token"].as_str().unwrap().to_string(),
    )
}

// ----------------------------------------------------------------------------
// 1. Action Bus: Quad-Export Parity, Approval Gates, Schema & Error Envelopes
// ----------------------------------------------------------------------------
#[tokio::test]
async fn tier5_action_bus_quad_export_and_gate_adversarial_guards() {
    let reg = ActionRegistry::new();
    reg.register_typed(SendMessageAction).unwrap();
    reg.register_typed(DeleteMessageAction).unwrap();
    reg.register_typed(GetChatHistoryAction).unwrap();
    reg.register_typed(ListDialogsAction).unwrap();

    // Quad-Export consistency
    let mcp = McpExporter::export_tools_list(&reg);
    let hermes = HermesToolExporter::export_tools(&reg);
    let cli = CliExporter::export_cli_schema(&reg, "gpui-tg", "1.0");
    let openapi = OpenApiExporter::export_spec(&reg, "GPUI", "1.0", "http://localhost");
    assert_eq!(mcp.tools.len(), 4);
    assert_eq!(hermes.len(), 4);
    assert_eq!(cli.subcommands.len(), 4);
    assert_eq!(openapi["paths"].as_object().unwrap().len(), 4);

    assert!(mcp.tools.iter().find(|t| t.name == "delete_message").unwrap().requires_approval);
    assert!(hermes.iter().find(|t| t.name == "delete_message").unwrap().requires_approval);
    assert!(cli.subcommands.iter().find(|t| t.name == "delete_message").unwrap().requires_approval);
    assert_eq!(
        openapi["paths"]["/api/v1/actions/delete_message"]["post"]["x-requires-approval"].as_bool(),
        Some(true)
    );

    // Unapproved high-risk action returns Forbidden before input execution
    let err = reg
        .execute("delete_message", ActionContext::new().with_approval(false), json!("invalid"))
        .await
        .unwrap_err();
    assert_eq!(err.code(), "forbidden");

    let (_dir, _state, app) = open_ephemeral_server();
    let (_u1, tok1) = register_user(&app, "t5_act_u1").await;
    let (_u2, tok2) = register_user(&app, "t5_act_u2").await;

    // Non-object JSON returns 422 invalid_input
    let res_null = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/actions/send_message")
                .header(header::AUTHORIZATION, format!("Bearer {tok1}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from("null"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res_null.status(), StatusCode::UNPROCESSABLE_ENTITY);

    // High-risk gate issuance + cross-user IDOR + always_allow reject + deny/re-decide 409
    let res_gate = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/actions/delete_message")
                .header(header::AUTHORIZATION, format!("Bearer {tok1}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"chat_id": 1, "message_ids": [1]}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res_gate.status(), StatusCode::FORBIDDEN);
    let g_val: Value =
        serde_json::from_slice(&to_bytes(res_gate.into_body(), usize::MAX).await.unwrap()).unwrap();
    let gate_id = g_val["gate_id"].as_str().unwrap().to_string();

    let res_idor = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/actions/approvals/{gate_id}/approve"))
                .header(header::AUTHORIZATION, format!("Bearer {tok2}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"decision": "approve_once"}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res_idor.status(), StatusCode::FORBIDDEN);

    let res_always = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/actions/approvals/{gate_id}/approve"))
                .header(header::AUTHORIZATION, format!("Bearer {tok1}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"decision": "always_allow"}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res_always.status(), StatusCode::FORBIDDEN);

    let res_deny = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/actions/approvals/{gate_id}/approve"))
                .header(header::AUTHORIZATION, format!("Bearer {tok1}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"decision": "deny", "reason": "denied"}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res_deny.status(), StatusCode::OK);

    let res_redecide = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/actions/approvals/{gate_id}/approve"))
                .header(header::AUTHORIZATION, format!("Bearer {tok1}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"decision": "approve_once"}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res_redecide.status(), StatusCode::CONFLICT);
}

// ----------------------------------------------------------------------------
// 2. SQLite WAL Pool: Panic Rollback Safety & 65-Reader Saturation Wakeup
// ----------------------------------------------------------------------------
#[test]
fn tier5_sqlite_pool_panic_rollback_and_ephemeral_saturation() {
    let pool = SqlitePool::open_in_memory_with_capacity(1).unwrap();
    pool.with_writer(|c| {
        c.execute_batch("CREATE TABLE tx_test (id INT PRIMARY KEY);")
            .map_err(|e| TelegramError::Database(e.to_string()))
    })
    .unwrap();

    // Panic inside with_transaction must roll back cleanly and not poison the writer mutex
    let pool_p = pool.clone();
    let res = std::thread::spawn(move || {
        let _: Result<(), TelegramError> = pool_p.with_transaction(|tx| {
            tx.execute("INSERT INTO tx_test (id) VALUES (1)", []).unwrap();
            panic!("simulated panic inside transaction");
        });
    })
    .join();
    assert!(res.is_err());

    let cnt: i64 = pool
        .with_reader(|c| {
            c.query_row("SELECT COUNT(*) FROM tx_test", [], |r| r.get(0))
                .map_err(|e| TelegramError::Database(e.to_string()))
        })
        .unwrap();
    assert_eq!(cnt, 0);

    pool.with_transaction(|tx| {
        tx.execute("INSERT INTO tx_test (id) VALUES (2)", [])
            .map_err(|e| TelegramError::Database(e.to_string()))?;
        Ok(())
    })
    .unwrap();

    // Saturate 1 pooled + 64 ephemeral readers; 66th waits on Condvar and wakes cleanly
    let barrier = Arc::new(Barrier::new(65));
    let mut handles = Vec::new();
    for _ in 0..65 {
        let p = pool.clone();
        let b = Arc::clone(&barrier);
        handles.push(std::thread::spawn(move || {
            p.with_reader(|c| {
                b.wait();
                std::thread::sleep(Duration::from_millis(80));
                c.query_row("SELECT COUNT(*) FROM tx_test", [], |r| r.get::<_, i64>(0))
                    .map_err(|e| TelegramError::Database(e.to_string()))
            })
            .unwrap()
        }));
    }
    std::thread::sleep(Duration::from_millis(40));
    let extra = pool
        .with_reader(|c| {
            c.query_row("SELECT COUNT(*) FROM tx_test", [], |r| r.get::<_, i64>(0))
                .map_err(|e| TelegramError::Database(e.to_string()))
        })
        .unwrap();
    assert_eq!(extra, 1);
    for h in handles {
        assert_eq!(h.join().unwrap(), 1);
    }
}

// ----------------------------------------------------------------------------
// 3. GatewayHub & CrossNodeEventBus: ChannelLagged Recovery & AuthZ Hardening
// ----------------------------------------------------------------------------
#[tokio::test]
async fn tier5_gateway_hub_bus_lag_recovery_and_session_authz() {
    // Part A: CrossNodeEventBus ChannelLagged recovery
    let broker = Arc::new(InProcessEventBus::with_capacity("broker", 16));
    let bus_a: Arc<dyn CrossNodeEventBus> =
        Arc::new(RedisEventBusStub::with_shared_broker("nA", "redis://x", broker.clone()));
    let bus_b: Arc<dyn CrossNodeEventBus> =
        Arc::new(RedisEventBusStub::with_shared_broker("nB", "redis://x", broker));

    let hub_a = GatewayHub::new();
    hub_a.set_event_bus(bus_a);
    let hub_b = GatewayHub::new();
    hub_b.set_event_bus(bus_b);

    let (tx_b, mut rx_b) = mpsc::unbounded_channel();
    hub_b.register(UserId(77), tx_b);
    tokio::time::sleep(Duration::from_millis(40)).await;

    for i in 0..100u64 {
        hub_a.broadcast_to_user(UserId(77), WsServerFrame::Pong { nonce: i });
    }
    tokio::time::sleep(Duration::from_millis(60)).await;
    hub_a.broadcast_to_user(UserId(77), WsServerFrame::Pong { nonce: 99999 });

    let mut saw_final = false;
    while let Ok(Some(frame)) = tokio::time::timeout(Duration::from_millis(200), rx_b.recv()).await {
        if matches!(frame, WsServerFrame::Pong { nonce: 99999 }) {
            saw_final = true;
            break;
        }
    }
    assert!(saw_final, "GatewayHub bus listener must survive ChannelLagged");

    // Part B: SubscribeChat membership IDOR & disconnect_session isolation (C1-03)
    let (_dir, state, app) = open_ephemeral_server();
    let (alice_id, alice_tok) = register_user(&app, "t5_ws_alice").await;
    let (eve_id, _eve_tok) = register_user(&app, "t5_ws_eve").await;

    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/chats")
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"title": "Private", "chat_type": "group"}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);
    let chat_v: Value =
        serde_json::from_slice(&to_bytes(res.into_body(), usize::MAX).await.unwrap()).unwrap();
    let private_chat_id = chat_v["chat_id"].as_i64().unwrap();

    let (tx_eve, mut rx_eve) = mpsc::unbounded_channel();
    state
        .gateway_hub
        .register_session(UserId(eve_id), Some("eve_sess".into()), tx_eve.clone());
    state.gateway_hub.handle_client_frame(
        UserId(eve_id),
        WsClientFrame::SubscribeChat { chat_id: private_chat_id },
        &tx_eve,
    );
    state
        .gateway_hub
        .broadcast_to_chat(private_chat_id, WsServerFrame::Pong { nonce: 9999 });

    // Eve must receive Error { code: 403 } and NOT the chat Pong frame
    let first = tokio::time::timeout(Duration::from_millis(150), rx_eve.recv())
        .await
        .expect("expected 403 error frame for unauthorized SubscribeChat")
        .expect("channel open");
    assert!(
        matches!(first, WsServerFrame::Error { code: 403, .. }),
        "Expected 403 Error for non-member SubscribeChat, got {first:?}"
    );
    let leaked = tokio::time::timeout(Duration::from_millis(100), rx_eve.recv()).await;
    if let Ok(Some(frame)) = leaked {
        assert!(
            !matches!(frame, WsServerFrame::Pong { nonce: 9999 }),
            "Non-member Eve must NOT receive broadcast_to_chat frames for private chat"
        );
    }

    // Disconnecting an unrelated session_id must NOT kill unscoped (session_id: None) sockets
    let (tx_unscoped, mut rx_unscoped) = mpsc::unbounded_channel();
    state.gateway_hub.register(UserId(alice_id), tx_unscoped);
    state
        .gateway_hub
        .disconnect_session(UserId(alice_id), "sess_unrelated");
    assert!(
        rx_unscoped.try_recv().is_err(),
        "Unscoped socket must not be killed by unrelated session_id revocation"
    );

    // Part C: Kicked/banned member broadcast isolation (C1-W1-01)
    let (bob_id, _bob_tok) = register_user(&app, "t5_ws_bob").await;
    let add_bob = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/chats/{private_chat_id}/members"))
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"user_id": bob_id, "role": "member"}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(add_bob.status(), StatusCode::OK);

    let (tx_bob, mut rx_bob) = mpsc::unbounded_channel();
    state
        .gateway_hub
        .register_session(UserId(bob_id), Some("bob_sess".into()), tx_bob.clone());
    state.gateway_hub.handle_client_frame(
        UserId(bob_id),
        WsClientFrame::SubscribeChat { chat_id: private_chat_id },
        &tx_bob,
    );
    assert!(
        state
            .gateway_hub
            .get_chat_subscribers(private_chat_id)
            .contains(&UserId(bob_id))
    );

    // Kick Bob via DELETE /api/v1/chats/{chat_id}/members/{user_id}
    let kick_bob = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/chats/{private_chat_id}/members/{bob_id}"))
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(kick_bob.status(), StatusCode::OK);
    assert!(
        !state
            .gateway_hub
            .get_chat_subscribers(private_chat_id)
            .contains(&UserId(bob_id)),
        "Kicked member must be unsubscribed from chat_subscribers"
    );

    // Alice posts a secret message after kicking Bob; Bob must NOT receive it
    let post_secret = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/chats/{private_chat_id}/messages"))
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"text": "TOP_SECRET_AFTER_KICK", "client_nonce": 991122}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert!(post_secret.status().is_success());
    state
        .gateway_hub
        .broadcast_to_chat(private_chat_id, WsServerFrame::Pong { nonce: 8888 });
    while let Ok(Some(frame)) = tokio::time::timeout(Duration::from_millis(80), rx_bob.recv()).await {
        assert!(
            !matches!(frame, WsServerFrame::PtsPush(_) | WsServerFrame::Pong { nonce: 8888 }),
            "Kicked member Bob must NOT receive chat broadcasts: {frame:?}"
        );
    }

    // Even if a stale chat_subscribers entry existed for a non-member, broadcast_to_chat_and_members prunes it
    state.gateway_hub.subscribe_chat(private_chat_id, UserId(bob_id));
    state.gateway_hub.broadcast_to_chat_and_members(
        gpui_telegram_core::types::ChatId(private_chat_id),
        None,
        WsServerFrame::Pong { nonce: 7777 },
    );
    assert!(
        tokio::time::timeout(Duration::from_millis(80), rx_bob.recv()).await.is_err(),
        "Stale subscriber for kicked member must be pruned before delivery"
    );
    assert!(
        !state
            .gateway_hub
            .get_chat_subscribers(private_chat_id)
            .contains(&UserId(bob_id)),
        "Stale kicked subscriber must be evicted from chat_subscribers"
    );
}

// ----------------------------------------------------------------------------
// 4. OutboxWorker: Monotonic Nonces, Duplicate Nonce Idempotency
// ----------------------------------------------------------------------------
#[test]
fn tier5_outbox_duplicate_nonce_idempotency_and_missing_token_handling() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let listener = rt.block_on(TcpListener::bind("127.0.0.1:0")).unwrap();
    let addr = listener.local_addr().unwrap();
    let srv_hits = Arc::new(AtomicUsize::new(0));
    let hits_c = Arc::clone(&srv_hits);

    rt.spawn(async move {
        loop {
            let Ok((mut stream, _)) = listener.accept().await else { break };
            let hits = Arc::clone(&hits_c);
            tokio::spawn(async move {
                let mut buf = [0u8; 4096];
                let _ = stream.read(&mut buf).await;
                hits.fetch_add(1, Ordering::SeqCst);
                let body = json!({
                    "message_id": 1001, "id": 1001, "pts": 1, "pts_count": 1,
                    "duplicate": false, "date_ms": 1700000000000i64,
                    "date": "2026-11-17T00:00:00Z", "sender_id": 1,
                    "chat_id": 42, "text": "hi", "client_nonce": 12345, "status": "sent"
                })
                .to_string();
                let resp = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                );
                let _ = stream.write_all(resp.as_bytes()).await;
            });
        }
    });

    let dir = tempfile::tempdir().unwrap();
    let cfg = ClientConfig {
        server_url: format!("http://{addr}"),
        ..Default::default()
    };
    let client = TelegramClient::open_with_dir(cfg.clone(), dir.path()).unwrap();
    client.set_auth_session(make_auth(1, "alice", "tok"));

    let req = RichSendMessageRequest {
        chat_id: 42,
        text: Some("hi".into()),
        content: Some(MessageContent::plain_text("hi")),
        entities: vec![],
        reply_to: None,
        quote_text: None,
        topic_id: None,
        client_nonce: Some(12345),
        forward_from_chat_id: None,
        forward_from_msg_id: None,
        silent: false,
        scheduled_at_ms: None,
        inline_keyboard: None,
        send_as_peer_id: None,
    };

    rt.block_on(client.send_message_durable(req.clone())).unwrap();
    assert_eq!(srv_hits.load(Ordering::SeqCst), 1);
    assert_eq!(
        client.outbox.store().get(12345).unwrap().unwrap().status,
        OutboxStatus::Delivered
    );

    // Second send_message_durable with already-Delivered client_nonce must NOT re-POST (C1-06)
    let dup_res = rt.block_on(client.send_message_durable(req));
    assert!(dup_res.is_err(), "Duplicate Delivered client_nonce must fail conflict validation");
    assert_eq!(
        srv_hits.load(Ordering::SeqCst),
        1,
        "Duplicate Delivered client_nonce must not re-fire HTTP POST"
    );

    // Missing auth token must increment retry_count and be bounded by max_retries (C1-W1-03)
    let dir_no_tok = tempfile::tempdir().unwrap();
    let client_no_tok = TelegramClient::open_with_dir(cfg, dir_no_tok.path()).unwrap();
    let missing_tok_req = RichSendMessageRequest {
        chat_id: 42,
        text: Some("queued without token".into()),
        content: Some(MessageContent::plain_text("queued without token")),
        entities: vec![],
        reply_to: None,
        quote_text: None,
        topic_id: None,
        client_nonce: Some(54321),
        forward_from_chat_id: None,
        forward_from_msg_id: None,
        silent: false,
        scheduled_at_ms: None,
        inline_keyboard: None,
        send_as_peer_id: None,
    };
    let send_no_tok = rt.block_on(client_no_tok.send_message_durable(missing_tok_req));
    assert!(send_no_tok.is_err(), "Missing token must fail direct send and queue in outbox");

    let rec0 = client_no_tok.outbox.store().get(54321).unwrap().unwrap();
    assert_eq!(rec0.status, OutboxStatus::Pending);
    assert_eq!(rec0.retry_count, 0);

    for attempt in 1..=DEFAULT_OUTBOX_MAX_RETRIES {
        client_no_tok
            .outbox
            .store()
            .reset_all_pending_retry_times(0)
            .unwrap();
        let processed = rt.block_on(client_no_tok.outbox.flush_due()).unwrap();
        assert_eq!(processed.len(), 1);
        let rec = client_no_tok.outbox.store().get(54321).unwrap().unwrap();
        if attempt < DEFAULT_OUTBOX_MAX_RETRIES {
            assert_eq!(rec.status, OutboxStatus::Pending);
            assert_eq!(
                rec.retry_count, attempt,
                "Missing token attempt {attempt} must increment retry_count"
            );
            assert_eq!(rec.last_error.as_deref(), Some("Missing auth token"));
        } else {
            assert_eq!(
                rec.status,
                OutboxStatus::Failed,
                "Missing token retries must stop at max_retries"
            );
        }
    }
}

// ----------------------------------------------------------------------------
// 5. Media Chunked Upload & CAS: Truncation DoS, Corrupt Checksum, Completed Resume
// ----------------------------------------------------------------------------
#[test]
fn tier5_media_cas_bounds_checksum_and_completed_resume_idempotency() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let (_srv_dir, _state, app) = open_ephemeral_server();

    // A. Out-of-order, wrong length, incomplete, and SHA-256 mismatch on CasStore
    rt.block_on(async {
        let dir = tempfile::tempdir().unwrap();
        let cas = CasStore::new(dir.path());
        cas.init_chunked_upload_for_user("u1", 1, 12, 4, Some("0".repeat(64)))
            .await
            .unwrap();
        assert!(!cas.write_chunk_for_user("u1", 1, 2, b"CCCC").await.unwrap());
        assert!(matches!(
            cas.complete_chunked_upload_for_user("u1", 1).await,
            Err(TelegramError::Validation(_))
        ));
        assert!(matches!(
            cas.write_chunk_for_user("u1", 1, 0, b"AAA").await,
            Err(TelegramError::Validation(_))
        ));
        assert!(!cas.write_chunk_for_user("u1", 1, 0, b"AAAA").await.unwrap());
        assert!(cas.write_chunk_for_user("u1", 1, 1, b"BBBB").await.unwrap());
        assert!(matches!(
            cas.complete_chunked_upload_for_user("u1", 1).await,
            Err(TelegramError::Validation(_))
        ));
    });

    // B. Oversized chunk_size (> u32::MAX) must NOT truncate silently to 1 (C1-02)
    let (_uid, tok) = rt.block_on(register_user(&app, "t5_media_u1"));
    let res_trunc = rt.block_on(
        app.clone().oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/upload/init")
                .header(header::AUTHORIZATION, format!("Bearer {tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "size_bytes": 100_000u64,
                        "chunk_size": 4_294_967_297u64,
                        "filename": "dos.bin"
                    })
                    .to_string(),
                ))
                .unwrap(),
        ),
    ).unwrap();
    assert_eq!(
        res_trunc.status(),
        StatusCode::BAD_REQUEST,
        "chunk_size > u32::MAX must be rejected with 400 rather than truncating to 1"
    );

    // Total parts exceeding MAX_TOTAL_PARTS must be rejected cleanly
    let res_oom = rt.block_on(
        app.clone().oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/media/upload/init")
                .header(header::AUTHORIZATION, format!("Bearer {tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "size_bytes": 500_000_000u64,
                        "chunk_size": 1u64,
                        "filename": "oom.bin"
                    })
                    .to_string(),
                ))
                .unwrap(),
        ),
    ).unwrap();
    assert_eq!(res_oom.status(), StatusCode::BAD_REQUEST);

    // C. Resuming an already Completed transfer must not re-upload (C1-05)
    let listener = rt.block_on(TcpListener::bind("127.0.0.1:0")).unwrap();
    let addr = listener.local_addr().unwrap();
    let router = app.clone();
    rt.spawn(async move {
        let _ = axum::serve(listener, router).await;
    });

    let client_dir = tempfile::tempdir().unwrap();
    let file_path = client_dir.path().join("payload.bin");
    std::fs::write(&file_path, vec![0xABu8; 64 * 1024]).unwrap();

    let cfg = ClientConfig {
        server_url: format!("http://{addr}"),
        ..Default::default()
    };
    let client = TelegramClient::open_with_dir(cfg, client_dir.path()).unwrap();
    client.set_auth_session(make_auth(1, "t5_media_u1", &tok));

    let r1 = rt
        .block_on(client.upload_file_resumable(&file_path, "application/octet-stream", None, Some("xfer_done")))
        .unwrap();
    let rec1 = client.chunked_transfers.store().get("xfer_done").unwrap().unwrap();
    assert_eq!(rec1.status, TransferStatus::Completed);
    let first_upload_id = rec1.upload_id.unwrap();

    let r2 = rt
        .block_on(client.upload_file_resumable(&file_path, "application/octet-stream", None, Some("xfer_done")))
        .unwrap();
    let second_upload_id = client
        .chunked_transfers
        .store()
        .get("xfer_done")
        .unwrap()
        .unwrap()
        .upload_id
        .unwrap();
    assert_eq!(
        first_upload_id, second_upload_id,
        "Completed transfer must short-circuit rather than re-uploading"
    );
    assert_eq!(r1.file_id, r2.file_id);
}

// ----------------------------------------------------------------------------
// 6. Production E2EE: MAX_SKIP/Eviction, X3DH Re-init Recovery, SenderKey Rotation
// ----------------------------------------------------------------------------
#[test]
fn tier5_e2ee_ratchet_skip_limits_x3dh_reset_and_sender_key_rotation() {
    // Part A: DoubleRatchet MAX_SKIP (256)
    let alice_id = DeviceIdentityKeys::generate().unwrap();
    let bob_id = DeviceIdentityKeys::generate().unwrap();
    let bob_spk_sec = random_32().unwrap();
    let bob_spk_pub = x25519_pub_from_secret(&bob_spk_sec);
    let spk_sig = bob_id.sign_signed_prekey(&bob_spk_pub);
    let bundle = DevicePreKeyBundle {
        user_id: 2,
        device_id: "b1".into(),
        device_name: None,
        identity_key: b64_encode(&bob_id.x25519_pub),
        identity_signing_key: b64_encode(&bob_id.ed25519_pub),
        signed_prekey: SignedPreKeyDto {
            key_id: 1,
            public_key: b64_encode(&bob_spk_pub),
            signature: b64_encode(&spk_sig),
            created_at_ms: 1,
        },
        one_time_prekey: None,
    };
    let ek_sec = random_32().unwrap();
    let ek_pub = x25519_pub_from_secret(&ek_sec);
    let init = x3dh_initiator(&alice_id.x25519_sec, &ek_sec, &bundle).unwrap();
    let resp = x3dh_responder(&bob_id.x25519_sec, &bob_spk_sec, None, &alice_id.x25519_pub, &ek_pub).unwrap();

    let mut alice = DoubleRatchetSession::init_alice(*init.shared_secret, bob_spk_pub, init.associated_data).unwrap();
    let mut bob = DoubleRatchetSession::init_bob(*resp.shared_secret, bob_spk_sec, resp.associated_data);

    let mut saved = Vec::new();
    for i in 0..258 {
        saved.push(alice.encrypt(format!("m{i}").as_bytes()).unwrap());
    }
    assert!(matches!(
        bob.decrypt(&saved[257].0, &saved[257].1),
        Err(E2eeError::OutOfOrderLimit)
    ));
    assert_eq!(bob.decrypt(&saved[0].0, &saved[0].1).unwrap(), b"m0");

    // Part B & C: Production E2eeManager stale-session recovery & SenderKey rotation (C1-04)
    let rt = tokio::runtime::Runtime::new().unwrap();
    let prekey_store: Arc<Mutex<HashMap<(u64, String), PublishPreKeysRequest>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let srv_store = Arc::clone(&prekey_store);

    let listener = rt.block_on(TcpListener::bind("127.0.0.1:0")).unwrap();
    let addr = listener.local_addr().unwrap();
    rt.spawn(async move {
        loop {
            let Ok((mut stream, _)) = listener.accept().await else { break };
            let store = Arc::clone(&srv_store);
            tokio::spawn(async move {
                let mut buf = Vec::new();
                let mut tmp = [0u8; 4096];
                let mut hend = None;
                while hend.is_none() {
                    let n = stream.read(&mut tmp).await.unwrap_or(0);
                    if n == 0 { break; }
                    buf.extend_from_slice(&tmp[..n]);
                    if let Some(p) = buf.windows(4).position(|w| w == b"\r\n\r\n") { hend = Some(p + 4); }
                }
                let hend = hend.unwrap_or(buf.len());
                let hstr = String::from_utf8_lossy(&buf[..hend]).to_string();
                let clen = hstr.lines().find_map(|l| l.to_ascii_lowercase().strip_prefix("content-length:").and_then(|v| v.trim().parse::<usize>().ok())).unwrap_or(0);
                while buf.len() < hend + clen {
                    let n = stream.read(&mut tmp).await.unwrap_or(0);
                    if n == 0 { break; }
                    buf.extend_from_slice(&tmp[..n]);
                }
                let first = hstr.lines().next().unwrap_or("");
                let mut parts = first.split_whitespace();
                let method = parts.next().unwrap_or("");
                let path = parts.next().unwrap_or("");
                let uid: u64 = hstr.lines().find_map(|l| l.to_ascii_lowercase().strip_prefix("authorization:").and_then(|v| v.trim().strip_prefix("bearer tok_").and_then(|s| s.parse().ok()))).unwrap_or(0);
                let body = &buf[hend..];

                let (status, resp_str) = if method == "POST" && path == "/api/v1/e2ee/prekeys" {
                    let req: PublishPreKeysRequest = serde_json::from_slice(body).unwrap();
                    let count = req.one_time_prekeys.len();
                    let dev = req.device_id.clone();
                    store.lock().insert((uid, dev.clone()), req);
                    (200, json!({"success": true, "user_id": uid, "device_id": dev, "remaining_one_time_prekeys": count, "updated_at_ms": 1700000000000i64}).to_string())
                } else if method == "GET" && path.starts_with("/api/v1/e2ee/devices/") {
                    let t_uid: u64 = path.trim_start_matches("/api/v1/e2ee/devices/").trim_end_matches("/prekeys").parse().unwrap_or(0);
                    let mut g = store.lock();
                    let mut devices = Vec::new();
                    for ((u, dev), req) in g.iter_mut() {
                        if *u == t_uid {
                            let otpk = if req.one_time_prekeys.is_empty() { None } else { Some(req.one_time_prekeys.remove(0)) };
                            devices.push(DevicePreKeyBundle {
                                user_id: t_uid, device_id: dev.clone(), device_name: req.device_name.clone(),
                                identity_key: req.identity_key.clone(), identity_signing_key: req.identity_signing_key.clone(),
                                signed_prekey: req.signed_prekey.clone(), one_time_prekey: otpk,
                            });
                        }
                    }
                    (200, serde_json::to_string(&UserPreKeyBundlesResponse { user_id: t_uid, devices }).unwrap())
                } else {
                    (404, "{}".into())
                };
                let resp = format!("HTTP/1.1 {status} OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{resp_str}", resp_str.len());
                let _ = stream.write_all(resp.as_bytes()).await;
            });
        }
    });

    let cfg = ClientConfig { server_url: format!("http://{addr}"), ..Default::default() };
    let d_a1 = tempfile::tempdir().unwrap();
    let d_a2 = tempfile::tempdir().unwrap();
    let d_b = tempfile::tempdir().unwrap();

    let c_a = TelegramClient::open_with_dir(cfg.clone(), d_a1.path()).unwrap();
    c_a.set_device_id("dev_a1");
    c_a.set_auth_session(make_auth(1, "alice", "tok_1"));

    let c_b = TelegramClient::open_with_dir(cfg.clone(), d_b.path()).unwrap();
    c_b.set_device_id("dev_b1");
    c_b.set_auth_session(make_auth(2, "bob", "tok_2"));

    rt.block_on(c_a.e2ee_init_and_publish_prekeys(5)).unwrap();
    rt.block_on(c_b.e2ee_init_and_publish_prekeys(5)).unwrap();

    let env1 = rt.block_on(c_a.e2ee_encrypt_dm(2, b"hello 1")).unwrap();
    assert_eq!(c_b.e2ee_decrypt_envelope(&env1).unwrap(), b"hello 1");

    // Delayed chain1 message must still decrypt after chain2 distribution
    let (d1, m1) = rt.block_on(c_a.e2ee_encrypt_group(555, &[1, 2], b"chain1 msg1")).unwrap();
    c_b.e2ee_decrypt_envelope(&d1.unwrap()).unwrap();
    assert_eq!(c_b.e2ee_decrypt_envelope(&m1).unwrap(), b"chain1 msg1");
    let (_, m2_delayed) = rt.block_on(c_a.e2ee_encrypt_group(555, &[1, 2], b"chain1 msg2 delayed")).unwrap();
    c_a.e2ee_manager().rotate_group_sender_key(555).unwrap();
    let (d2, m3) = rt.block_on(c_a.e2ee_encrypt_group(555, &[1, 2], b"chain2 msg3")).unwrap();
    c_b.e2ee_decrypt_envelope(&d2.unwrap()).unwrap();
    assert_eq!(c_b.e2ee_decrypt_envelope(&m3).unwrap(), b"chain2 msg3");
    assert_eq!(c_b.e2ee_decrypt_envelope(&m2_delayed).unwrap(), b"chain1 msg2 delayed");

    // Alice resets device state and re-initiates X3DH: Bob must recover seamlessly
    let c_a_fresh = TelegramClient::open_with_dir(cfg.clone(), d_a2.path()).unwrap();
    c_a_fresh.set_device_id("dev_a1");
    c_a_fresh.set_auth_session(make_auth(1, "alice", "tok_1"));
    rt.block_on(c_a_fresh.e2ee_init_and_publish_prekeys(5)).unwrap();

    let env2 = rt.block_on(c_a_fresh.e2ee_encrypt_dm(2, b"hello after reset")).unwrap();
    assert_eq!(c_b.e2ee_decrypt_envelope(&env2).unwrap(), b"hello after reset");

    // Replaying env1 after session recovery must be rejected (even with 0 OTPKs / SPK-only X3DH) (C1-W1-02)
    assert!(
        c_b.e2ee_decrypt_envelope(&env1).is_err(),
        "Replaying old X3DH epoch env1 must be rejected"
    );

    let d_c1 = tempfile::tempdir().unwrap();
    let d_c2 = tempfile::tempdir().unwrap();
    let d_d = tempfile::tempdir().unwrap();
    let c_c = TelegramClient::open_with_dir(cfg.clone(), d_c1.path()).unwrap();
    c_c.set_device_id("dev_c1");
    c_c.set_auth_session(make_auth(10, "carol", "tok_10"));
    let c_d = TelegramClient::open_with_dir(cfg.clone(), d_d.path()).unwrap();
    c_d.set_device_id("dev_d1");
    c_d.set_auth_session(make_auth(20, "dave", "tok_20"));
    rt.block_on(c_c.e2ee_init_and_publish_prekeys(0)).unwrap();
    rt.block_on(c_d.e2ee_init_and_publish_prekeys(0)).unwrap();

    let env_old = rt.block_on(c_c.e2ee_encrypt_dm(20, b"epoch 1 no otpk")).unwrap();
    assert_eq!(c_d.e2ee_decrypt_envelope(&env_old).unwrap(), b"epoch 1 no otpk");

    let c_c_fresh = TelegramClient::open_with_dir(cfg, d_c2.path()).unwrap();
    c_c_fresh.set_device_id("dev_c1");
    c_c_fresh.set_auth_session(make_auth(10, "carol", "tok_10"));
    rt.block_on(c_c_fresh.e2ee_init_and_publish_prekeys(0)).unwrap();
    let env_new = rt.block_on(c_c_fresh.e2ee_encrypt_dm(20, b"epoch 2 no otpk")).unwrap();
    assert_eq!(c_d.e2ee_decrypt_envelope(&env_new).unwrap(), b"epoch 2 no otpk");

    assert!(
        matches!(c_d.e2ee_decrypt_envelope(&env_old), Err(E2eeError::ReplayDetected)),
        "Replayed X3DH handshake from prior epoch must be rejected with ReplayDetected"
    );
    let env_followup = rt.block_on(c_c_fresh.e2ee_encrypt_dm(20, b"epoch 2 msg 2")).unwrap();
    assert_eq!(
        c_d.e2ee_decrypt_envelope(&env_followup).unwrap(),
        b"epoch 2 msg 2",
        "Active epoch 2 session must NOT be rolled back by rejected replay"
    );
}

// ----------------------------------------------------------------------------
// 7. Proxy Tunneling: Fail-Closed Validation, 407/SOCKS5 Auth Fail, DNS Leak
// ----------------------------------------------------------------------------
#[tokio::test]
async fn tier5_proxy_fail_closed_auth_failures_and_socks5_dns_leak_avoidance() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let direct_hits = Arc::new(AtomicUsize::new(0));
    let hits_srv = Arc::clone(&direct_hits);

    tokio::spawn(async move {
        loop {
            let Ok((mut stream, _)) = listener.accept().await else { break };
            let hits = Arc::clone(&hits_srv);
            tokio::spawn(async move {
                hits.fetch_add(1, Ordering::SeqCst);
                let mut buf = [0u8; 1024];
                let _ = stream.read(&mut buf).await;
                let resp = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 11\r\nConnection: close\r\n\r\n{\"ok\":true}";
                let _ = stream.write_all(resp.as_bytes()).await;
            });
        }
    });

    // Invalid proxy with fallback_to_direct = false MUST fail closed, never connect directly! (C1-01)
    let bad_proxy = ClientProxyConfig {
        proxy_type: ClientProxyType::Socks5,
        host: "".into(),
        port: 0,
        username: None,
        password: Some("secret_without_user".into()),
        fallback_to_direct: false,
    };
    let client = HttpClient::new_with_proxy(
        format!("http://{addr}"),
        Duration::from_secs(2),
        Some(tokio::runtime::Handle::current()),
        Some(bad_proxy),
    );
    assert!(
        client.check_health().await.is_err(),
        "Invalid proxy with fallback_to_direct=false must fail closed without leaking direct traffic"
    );
    assert_eq!(direct_hits.load(Ordering::SeqCst), 0);

    // HTTP 407 & SOCKS5 ATYP=0x03 DNS leak avoidance
    let http_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let http_addr = http_listener.local_addr().unwrap();
    tokio::spawn(async move {
        loop {
            let Ok((mut s, _)) = http_listener.accept().await else { break };
            tokio::spawn(async move {
                let mut buf = [0u8; 1024];
                let _ = s.read(&mut buf).await;
                let _ = s.write_all(b"HTTP/1.1 407 Proxy Authentication Required\r\nContent-Length: 0\r\n\r\n").await;
            });
        }
    });
    let http_proxy = ClientProxyConfig::http("127.0.0.1", http_addr.port()).with_auth("bad", "creds");
    let res_407 = http_proxy.probe_health("internal.local", 443, Duration::from_secs(2)).await;
    assert!(res_407.unwrap_err().to_string().contains("407"));

    let saw_atyp_domain = Arc::new(AtomicBool::new(false));
    let atyp_flag = Arc::clone(&saw_atyp_domain);
    let socks_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let socks_addr = socks_listener.local_addr().unwrap();
    tokio::spawn(async move {
        loop {
            let Ok((mut s, _)) = socks_listener.accept().await else { break };
            let flag = Arc::clone(&atyp_flag);
            tokio::spawn(async move {
                let mut hdr = [0u8; 2];
                if s.read_exact(&mut hdr).await.is_err() { return; }
                let mut methods = vec![0u8; hdr[1] as usize];
                let _ = s.read_exact(&mut methods).await;
                let _ = s.write_all(&[0x05, 0x02]).await;
                let mut v = [0u8; 2];
                let _ = s.read_exact(&mut v).await;
                let mut u = vec![0u8; v[1] as usize];
                let _ = s.read_exact(&mut u).await;
                let mut plen = [0u8; 1];
                let _ = s.read_exact(&mut plen).await;
                let mut p = vec![0u8; plen[0] as usize];
                let _ = s.read_exact(&mut p).await;
                if u != b"alice" || p != b"good" {
                    let _ = s.write_all(&[0x01, 0x01]).await;
                    return;
                }
                let _ = s.write_all(&[0x01, 0x00]).await;
                let mut req = [0u8; 4];
                if s.read_exact(&mut req).await.is_err() { return; }
                if req[3] == 0x03 {
                    flag.store(true, Ordering::SeqCst);
                }
            });
        }
    });

    let bad_socks = ClientProxyConfig::socks5("127.0.0.1", socks_addr.port()).with_auth("alice", "wrong");
    assert!(bad_socks.probe_health("nonexistent.invalid", 443, Duration::from_secs(2)).await.is_err());

    let good_socks = ClientProxyConfig::socks5("127.0.0.1", socks_addr.port()).with_auth("alice", "good");
    assert!(good_socks.probe_health("unresolvable.onion.local", 443, Duration::from_secs(2)).await.is_err());
    assert!(saw_atyp_domain.load(Ordering::SeqCst));

    // CR/LF & control character injection validation in proxy config & HTTP CONNECT tunnel
    assert!(
        ClientProxyConfig::http("127.0.0.1\r\nEvil: 1", http_addr.port())
            .validate()
            .is_err(),
        "Proxy host containing CR/LF must fail validation"
    );
    assert!(
        ClientProxyConfig::http("127.0.0.1", http_addr.port())
            .with_auth("u\r\n", "p")
            .validate()
            .is_err(),
        "Proxy username containing CR/LF must fail validation"
    );
    assert!(
        http_proxy
            .probe_health("example.com:80 HTTP/1.1\r\nX-Injected: true\r\n\r\n", 443, Duration::from_secs(1))
            .await
            .is_err(),
        "HTTP CONNECT tunnel must reject CR/LF in target_host"
    );
}

// ----------------------------------------------------------------------------
// 8. C1-W2-01: Membership Normalization, Kick/Ban/Leave Isolation & Chat AuthZ
// ----------------------------------------------------------------------------
#[tokio::test]
async fn tier5_membership_normalization_kick_ban_leave_and_chat_authz() {
    let (_dir, state, app) = open_ephemeral_server();
    let (alice_id, alice_tok) = register_user(&app, "t5_mem_alice").await;
    let (bob_id, bob_tok) = register_user(&app, "t5_mem_bob").await;
    let (dave_id, dave_tok) = register_user(&app, "t5_mem_dave").await;
    let (eve_id, eve_tok) = register_user(&app, "t5_mem_eve").await;

    // Alice creates a private group with Bob and Dave
    let create_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/chats")
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "title": "Vault Group",
                        "chat_type": 2,
                        "is_forum": true,
                        "member_ids": [bob_id, dave_id]
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert!(create_res.status().is_success());
    let chat_json: Value =
        serde_json::from_slice(&to_bytes(create_res.into_body(), usize::MAX).await.unwrap())
            .unwrap();
    let chat_id = chat_json["chat_id"].as_i64().unwrap();

    // Seed a secret message, topic, poll, and invite code
    let msg_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/chats/{chat_id}/messages"))
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({ "text": "VAULT_SECRET_NEEDLE_77", "client_nonce": 1001 }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(msg_res.status(), StatusCode::CREATED);
    let msg_json: Value =
        serde_json::from_slice(&to_bytes(msg_res.into_body(), usize::MAX).await.unwrap()).unwrap();
    let msg_id = msg_json["message_id"].as_i64().unwrap();

    let poll_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/chats/{chat_id}/polls"))
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({ "question": "Secret Poll?", "options": ["Yes", "No"] }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(poll_res.status(), StatusCode::CREATED);
    let poll_json: Value =
        serde_json::from_slice(&to_bytes(poll_res.into_body(), usize::MAX).await.unwrap()).unwrap();
    let poll_id = poll_json["poll"]["poll_id"].as_i64().unwrap();

    let inv_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/chats/{chat_id}/invites"))
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({ "title": "Main Invite" }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(inv_res.status(), StatusCode::CREATED);
    let inv_json: Value =
        serde_json::from_slice(&to_bytes(inv_res.into_body(), usize::MAX).await.unwrap()).unwrap();
    let invite_code = inv_json["code"].as_str().unwrap().to_string();

    // 1. Invalid & non-string roles return 400 Bad Request
    for bad_payload in [json!({"role": "superadmin"}), json!({"role": 123})] {
        let r = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri(format!("/api/v1/chats/{chat_id}/members/{bob_id}"))
                    .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(bad_payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(r.status(), StatusCode::BAD_REQUEST, "Expected 400 for {bad_payload}");
    }

    // 2. Non-member Eve IDOR checks on newly guarded endpoints
    for (method, uri) in [
        ("GET", format!("/api/v1/chats/{chat_id}/members")),
        ("GET", format!("/api/v1/chats/{chat_id}/topics")),
        ("GET", format!("/api/v1/sync/difference?chat_id={chat_id}&pts=0")),
    ] {
        let r = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(method)
                    .uri(&uri)
                    .header(header::AUTHORIZATION, format!("Bearer {eve_tok}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(r.status(), StatusCode::FORBIDDEN, "Eve must get 403 on {method} {uri}");
    }
    let eve_vote = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/polls/{poll_id}/vote"))
                .header(header::AUTHORIZATION, format!("Bearer {eve_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"option_ids": [0]}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(eve_vote.status(), StatusCode::FORBIDDEN);

    // 3. Demoted admin loses admin_rights (cannot keep admin_rights != 0)
    let promo = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/chats/{chat_id}/members/{bob_id}"))
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"role": "Admin"}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(promo.status(), StatusCode::OK);
    let demote = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/chats/{chat_id}/members/{bob_id}"))
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"role": "member"}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(demote.status(), StatusCode::OK);
    let bob_rec = state
        .db
        .get_chat_member(ChatId(chat_id), UserId(bob_id))
        .unwrap()
        .unwrap();
    assert_eq!(bob_rec.role, "member");
    assert_eq!(bob_rec.admin_rights, 0, "Demoted admin must have admin_rights reset to 0");
    let bridge = state.hermes.platform_im_bridge().unwrap();
    assert!(
        bridge
            .execute_im_tool(
                "pin_message",
                &json!({"chat_id": chat_id, "message_id": msg_id}),
                PlatformImCaller {
                    user_id: Some(bob_id),
                    chat_id: Some(chat_id),
                    ..Default::default()
                },
            )
            .await
            .is_err(),
        "Demoted member Bob must not retain admin pin_message rights in Hermes"
    );

    // 4. Mixed-case role normalization & full-surface revocation ("  KICKED  ", "BANNED", "Left", and direct DB "KiCkEd")
    let (tx_bob, mut rx_bob) = mpsc::unbounded_channel();
    state
        .gateway_hub
        .register_session(UserId(bob_id), Some("bob_w3_sess".into()), tx_bob.clone());

    for (raw_role, expected_norm) in [
        ("  KICKED  ", "kicked"),
        ("Left", "left"),
        ("BANNED", "banned"),
    ] {
        // Subscribe Bob first to verify immediate unsubscribe on PUT
        state.gateway_hub.subscribe_chat(chat_id, UserId(bob_id));
        let put_res = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri(format!("/api/v1/chats/{chat_id}/members/{bob_id}"))
                    .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(json!({"role": raw_role}).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(put_res.status(), StatusCode::OK);
        let put_json: Value =
            serde_json::from_slice(&to_bytes(put_res.into_body(), usize::MAX).await.unwrap())
                .unwrap();
        assert_eq!(put_json["role"].as_str(), Some(expected_norm));
        assert_eq!(
            state
                .db
                .get_chat_member(ChatId(chat_id), UserId(bob_id))
                .unwrap()
                .unwrap()
                .role,
            expected_norm
        );
        assert!(
            !state
                .gateway_hub
                .get_chat_subscribers(chat_id)
                .contains(&UserId(bob_id)),
            "Bob must be unsubscribed immediately on PUT role={raw_role}"
        );
    }

    // Inject raw mixed-case "KiCkEd" directly in DB to also prove read-side case-insensitivity
    state
        .db
        .pool()
        .with_conn(|conn| {
            conn.execute(
                "UPDATE chat_members SET role = 'KiCkEd' WHERE chat_id = ?1 AND user_id = ?2",
                rusqlite::params![chat_id, bob_id as i64],
            )
            .map_err(|e| TelegramError::Database(e.to_string()))?;
            Ok(())
        })
        .unwrap();

    // Verify full-surface revocation against Bob
    let bob_get_chat = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/chats/{chat_id}"))
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bob_get_chat.status(), StatusCode::NOT_FOUND);

    for (method, uri) in [
        ("GET", format!("/api/v1/chats/{chat_id}/messages")),
        ("GET", format!("/api/v1/chats/{chat_id}/members")),
        ("GET", format!("/api/v1/chats/{chat_id}/topics")),
        ("GET", format!("/api/v1/sync/difference?chat_id={chat_id}&pts=0")),
        ("DELETE", format!("/api/v1/chats/{chat_id}/members/{bob_id}")),
    ] {
        let r = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(method)
                    .uri(&uri)
                    .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(r.status(), StatusCode::FORBIDDEN, "Inactive Bob must get 403 on {method} {uri}");
    }

    let bob_post_msg = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/chats/{chat_id}/messages"))
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"text": "hack", "client_nonce": 5551}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bob_post_msg.status(), StatusCode::FORBIDDEN);

    let bob_vote = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/polls/{poll_id}/vote"))
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"option_ids": [0]}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bob_vote.status(), StatusCode::FORBIDDEN);

    let bob_sync_post = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sync/difference")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"chat_id": chat_id, "pts": 0}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bob_sync_post.status(), StatusCode::FORBIDDEN);

    let bob_chats = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/chats")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bob_chats.status(), StatusCode::OK);
    let bob_chats_body = String::from_utf8_lossy(
        &to_bytes(bob_chats.into_body(), usize::MAX).await.unwrap(),
    )
    .to_string();
    assert!(!bob_chats_body.contains("Vault Group"), "Kicked Bob must not see chat in GET /chats");

    let bob_search = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/search?q=VAULT_SECRET_NEEDLE_77")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bob_search.status(), StatusCode::OK);
    let bob_search_json: Value = serde_json::from_slice(
        &to_bytes(bob_search.into_body(), usize::MAX).await.unwrap(),
    )
    .unwrap();
    assert_eq!(bob_search_json["total"].as_u64(), Some(0));
    assert!(bob_search_json["hits"].as_array().unwrap().is_empty());
    assert!(bob_search_json["results"].as_array().unwrap().is_empty());

    // Action Bus checks
    for (act_uri, body) in [
        ("/api/v1/actions/send_message", json!({"chat_id": chat_id, "text": "x"})),
        ("/api/v1/actions/get_chat_history", json!({"chat_id": chat_id})),
    ] {
        let r = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(act_uri)
                    .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(r.status(), StatusCode::FORBIDDEN, "Action Bus {act_uri} must reject inactive Bob");
    }

    // Hermes PlatformImBridge checks
    for (tool, args) in [
        ("send_message", json!({"chat_id": chat_id, "text": "x"})),
        ("list_members", json!({"chat_id": chat_id})),
        ("vote_poll", json!({"poll_id": poll_id, "option_index": 0})),
    ] {
        assert!(
            bridge
                .execute_im_tool(
                    tool,
                    &args,
                    PlatformImCaller {
                        user_id: Some(bob_id),
                        ..Default::default()
                    },
                )
                .await
                .is_err(),
            "PlatformIm {tool} must reject inactive Bob"
        );
    }
    let search_res = bridge
        .execute_im_tool(
            "search_messages",
            &json!({"query": "VAULT_SECRET_NEEDLE_77"}),
            PlatformImCaller {
                user_id: Some(bob_id),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(search_res["count"].as_u64(), Some(0));

    // WS SubscribeChat + broadcast_to_chat_and_members
    state.gateway_hub.handle_client_frame(
        UserId(bob_id),
        WsClientFrame::SubscribeChat { chat_id },
        &tx_bob,
    );
    let sub_err = tokio::time::timeout(Duration::from_millis(80), rx_bob.recv())
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(sub_err, WsServerFrame::Error { code: 403, .. }));
    state.gateway_hub.broadcast_to_chat_and_members(
        ChatId(chat_id),
        Some(UserId(bob_id)),
        WsServerFrame::Pong { nonce: 4444 },
    );
    assert!(
        tokio::time::timeout(Duration::from_millis(80), rx_bob.recv())
            .await
            .is_err(),
        "Inactive Bob must receive 0 WS frames"
    );

    // 4b. WS VotePoll authorization bound to poll's real chat_id (C1-W3-01)
    let eve_chat_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/chats")
                .header(header::AUTHORIZATION, format!("Bearer {eve_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"title": "Eve Own Chat", "chat_type": 2}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(eve_chat_res.status(), StatusCode::CREATED);
    let eve_chat_id = serde_json::from_slice::<Value>(
        &to_bytes(eve_chat_res.into_body(), usize::MAX).await.unwrap(),
    )
    .unwrap()["chat_id"]
        .as_i64()
        .unwrap();

    let bob_own_chat_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/chats")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"title": "Bob Own Chat", "chat_type": 2}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bob_own_chat_res.status(), StatusCode::CREATED);
    let bob_own_chat_id = serde_json::from_slice::<Value>(
        &to_bytes(bob_own_chat_res.into_body(), usize::MAX).await.unwrap(),
    )
    .unwrap()["chat_id"]
        .as_i64()
        .unwrap();

    let (tx_eve, mut rx_eve) = mpsc::unbounded_channel();
    state
        .gateway_hub
        .register_session(UserId(eve_id), Some("eve_w4_sess".into()), tx_eve.clone());
    state.gateway_hub.handle_client_frame(
        UserId(eve_id),
        WsClientFrame::VotePoll {
            chat_id: ChatId(eve_chat_id),
            message_id: MessageId(0),
            poll_id: poll_id as u64,
            option_ids: vec![0],
        },
        &tx_eve,
    );
    let eve_poll_err = tokio::time::timeout(Duration::from_millis(80), rx_eve.recv())
        .await
        .unwrap()
        .unwrap();
    assert!(
        matches!(eve_poll_err, WsServerFrame::Error { code: 403, .. }),
        "Spoofed chat_id WS VotePoll from Eve must fail with 403: {eve_poll_err:?}"
    );

    state.gateway_hub.handle_client_frame(
        UserId(bob_id),
        WsClientFrame::VotePoll {
            chat_id: ChatId(bob_own_chat_id),
            message_id: MessageId(0),
            poll_id: poll_id as u64,
            option_ids: vec![0],
        },
        &tx_bob,
    );
    let bob_poll_err = tokio::time::timeout(Duration::from_millis(80), rx_bob.recv())
        .await
        .unwrap()
        .unwrap();
    assert!(
        matches!(bob_poll_err, WsServerFrame::Error { code: 403, .. }),
        "Spoofed chat_id WS VotePoll from kicked Bob must fail with 403: {bob_poll_err:?}"
    );

    // Kicked Bob passing real chat_id must also fail with 403
    state.gateway_hub.handle_client_frame(
        UserId(bob_id),
        WsClientFrame::VotePoll {
            chat_id: ChatId(chat_id),
            message_id: MessageId(0),
            poll_id: poll_id as u64,
            option_ids: vec![0],
        },
        &tx_bob,
    );
    let bob_real_cid_err = tokio::time::timeout(Duration::from_millis(80), rx_bob.recv())
        .await
        .unwrap()
        .unwrap();
    assert!(
        matches!(bob_real_cid_err, WsServerFrame::Error { code: 403, .. }),
        "Kicked Bob WS VotePoll on target chat must fail with 403: {bob_real_cid_err:?}"
    );
    let (_, poll_after_ws) = state
        .db
        .get_poll_for_user(poll_id, UserId(alice_id))
        .unwrap()
        .unwrap();
    assert_eq!(poll_after_ws.total_voters, 0, "Rejected WS VotePoll must leave total_voters == 0");

    // 4c. WS CallSignal participant enforcement: non-participant Eve cannot hangup Alice->Dave call
    let call_inv_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/calls/invite")
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({ "callee_id": dave_id, "chat_id": chat_id, "is_video": false })
                        .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(call_inv_res.status(), StatusCode::CREATED);
    let call_id = serde_json::from_slice::<Value>(
        &to_bytes(call_inv_res.into_body(), usize::MAX).await.unwrap(),
    )
    .unwrap()["call_id"]
        .as_str()
        .unwrap()
        .to_string();

    state.gateway_hub.handle_client_frame(
        UserId(eve_id),
        WsClientFrame::CallSignal {
            call_id: call_id.clone(),
            signal_type: "hangup".into(),
            target_user_id: Some(dave_id),
            sdp: None,
            candidate: None,
            sdp_mid: None,
            sdp_mline_index: None,
        },
        &tx_eve,
    );
    let eve_call_err = tokio::time::timeout(Duration::from_millis(80), rx_eve.recv())
        .await
        .unwrap()
        .unwrap();
    assert!(
        matches!(eve_call_err, WsServerFrame::Error { code: 403, .. }),
        "Non-participant Eve WS CallSignal must fail with 403: {eve_call_err:?}"
    );
    assert_ne!(
        state.db.get_call(&call_id).unwrap().unwrap().state,
        "ended",
        "Non-participant WS CallSignal must not end Alice-Dave call"
    );

    // 5. Banned Bob cannot rejoin via invite link
    let ban_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/chats/{chat_id}/members/{bob_id}"))
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"role": "BANNED"}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(ban_res.status(), StatusCode::OK);
    let join_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/chats/join")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"code": invite_code}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(join_res.status(), StatusCode::FORBIDDEN);
    assert!(
        !state
            .gateway_hub
            .get_chat_subscribers(chat_id)
            .contains(&UserId(bob_id))
    );

    // 6. Eager unsubscribe on POST /api/v1/chats/{id}/leave (Dave)
    let (tx_dave, _rx_dave) = mpsc::unbounded_channel();
    state
        .gateway_hub
        .register_session(UserId(dave_id), Some("dave_w3_sess".into()), tx_dave.clone());
    state.gateway_hub.handle_client_frame(
        UserId(dave_id),
        WsClientFrame::SubscribeChat { chat_id },
        &tx_dave,
    );
    assert!(
        state
            .gateway_hub
            .get_chat_subscribers(chat_id)
            .contains(&UserId(dave_id))
    );
    let leave_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/chats/{chat_id}/leave"))
                .header(header::AUTHORIZATION, format!("Bearer {dave_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(leave_res.status(), StatusCode::OK);
    assert!(
        !state
            .gateway_hub
            .get_chat_subscribers(chat_id)
            .contains(&UserId(dave_id)),
        "Dave must be unsubscribed immediately after POST /leave before any broadcast"
    );
    let dave_hist = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/chats/{chat_id}/messages"))
                .header(header::AUTHORIZATION, format!("Bearer {dave_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(dave_hist.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn tier5_ws_sync_restricted_and_dm_block_authz_parity() {
    use futures::{SinkExt, StreamExt};
    use gpui_telegram_proto::sync::{
        SyncGetDifferenceRequest, TypingAction, TypingEvent,
    };
    use gpui_telegram_proto::ws::SendMessageRequest;
    use gpui_telegram_server::db::schema::ChatMemberRecord;
    use tokio_tungstenite::tungstenite::Message as WsMsg;

    let (_dir, state, app) = open_ephemeral_server();
    let (alice_id, alice_tok) = register_user(&app, "t5_w5_alice").await;
    let (bob_id, bob_tok) = register_user(&app, "t5_w5_bob").await;
    let (dave_id, dave_tok) = register_user(&app, "t5_w5_dave").await;
    let (malloy_id, malloy_tok) = register_user(&app, "t5_w5_malloy").await;
    let (eve_id, eve_tok) = register_user(&app, "t5_w5_eve").await;
    let _ = (&dave_tok, &malloy_tok);

    // 1. Alice creates a private group with Bob, Dave, Malloy (Eve is never a member)
    let create_grp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/chats")
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "title": "W5 WS Authz Group",
                        "chat_type": 2,
                        "member_ids": [bob_id, dave_id, malloy_id]
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(create_grp.status(), StatusCode::CREATED);
    let chat_id = serde_json::from_slice::<Value>(
        &to_bytes(create_grp.into_body(), usize::MAX).await.unwrap(),
    )
    .unwrap()["chat_id"]
        .as_i64()
        .unwrap();

    let post_secret = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/chats/{chat_id}/messages"))
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "text": "TOP_SECRET_WS_SYNC_NEEDLE",
                        "client_nonce": 9001
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(post_secret.status(), StatusCode::CREATED);

    // Kick Bob (with raw mixed-case DB role normalization check)
    let kick_bob = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/chats/{chat_id}/members/{bob_id}"))
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({ "role": "  KiCkEd " }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(kick_bob.status(), StatusCode::OK);

    let (tx_alice, mut rx_alice) = mpsc::unbounded_channel();
    let (tx_bob, mut rx_bob) = mpsc::unbounded_channel();
    let (tx_dave, mut rx_dave) = mpsc::unbounded_channel();
    let (tx_malloy, mut rx_malloy) = mpsc::unbounded_channel();
    let (tx_eve, mut rx_eve) = mpsc::unbounded_channel();
    state
        .gateway_hub
        .register_session(UserId(alice_id), Some("w5_alice".into()), tx_alice.clone());
    state
        .gateway_hub
        .register_session(UserId(bob_id), Some("w5_bob".into()), tx_bob.clone());
    state
        .gateway_hub
        .register_session(UserId(dave_id), Some("w5_dave".into()), tx_dave.clone());
    state
        .gateway_hub
        .register_session(UserId(malloy_id), Some("w5_malloy".into()), tx_malloy.clone());
    state
        .gateway_hub
        .register_session(UserId(eve_id), Some("w5_eve".into()), tx_eve.clone());

    // 1a. WsClientFrame::Sync authz (C1-W4-01)
    state.gateway_hub.handle_client_frame(
        UserId(eve_id),
        WsClientFrame::Sync(SyncGetDifferenceRequest::new(ChatId(chat_id), 0, 100)),
        &tx_eve,
    );
    let eve_sync_frame = tokio::time::timeout(Duration::from_millis(80), rx_eve.recv())
        .await
        .unwrap()
        .unwrap();
    assert!(
        matches!(eve_sync_frame, WsServerFrame::Error { code: 403, .. }),
        "Non-member Eve Sync must fail with 403, got: {eve_sync_frame:?}"
    );
    assert!(
        tokio::time::timeout(Duration::from_millis(40), rx_eve.recv())
            .await
            .is_err(),
        "Eve must receive 0 SyncDiff frames"
    );

    state.gateway_hub.handle_client_frame(
        UserId(bob_id),
        WsClientFrame::Sync(SyncGetDifferenceRequest::new(ChatId(chat_id), 0, 100)),
        &tx_bob,
    );
    let bob_sync_frame = tokio::time::timeout(Duration::from_millis(80), rx_bob.recv())
        .await
        .unwrap()
        .unwrap();
    assert!(
        matches!(bob_sync_frame, WsServerFrame::Error { code: 403, .. }),
        "Kicked Bob Sync must fail with 403, got: {bob_sync_frame:?}"
    );

    for missing_cid in [999_999i64, 0, -1] {
        state.gateway_hub.handle_client_frame(
            UserId(dave_id),
            WsClientFrame::Sync(SyncGetDifferenceRequest::new(ChatId(missing_cid), 0, 100)),
            &tx_dave,
        );
        let missing_frame = tokio::time::timeout(Duration::from_millis(80), rx_dave.recv())
            .await
            .unwrap()
            .unwrap();
        assert!(
            matches!(missing_frame, WsServerFrame::Error { code: 404, .. }),
            "Nonexistent chat {missing_cid} Sync must fail with 404, got: {missing_frame:?}"
        );
    }

    state.gateway_hub.handle_client_frame(
        UserId(dave_id),
        WsClientFrame::Sync(SyncGetDifferenceRequest::new(ChatId(chat_id), 0, 100)),
        &tx_dave,
    );
    let dave_sync_frame = tokio::time::timeout(Duration::from_millis(80), rx_dave.recv())
        .await
        .unwrap()
        .unwrap();
    let dave_sync_json = serde_json::to_string(&dave_sync_frame).unwrap();
    assert!(
        matches!(dave_sync_frame, WsServerFrame::SyncDiff(_))
            && dave_sync_json.contains("TOP_SECRET_WS_SYNC_NEEDLE"),
        "Active member Dave Sync must succeed with message needle, got: {dave_sync_frame:?}"
    );

    // 2. WsClientFrame::SendMessage & SendTyping restricted mute (C1-W4-02)
    let now_ms = chrono::Utc::now().timestamp_millis();
    state
        .db
        .add_chat_member(&ChatMemberRecord {
            chat_id: ChatId(chat_id),
            user_id: UserId(malloy_id),
            role: " ReStRiCtEd ".to_string(),
            permissions_bitmask: 0,
            admin_title: None,
            admin_rights: 0,
            banned_rights: 1,
            until_date_ms: None,
            joined_at_ms: now_ms,
            invited_by: Some(UserId(alice_id)),
            updated_at_ms: now_ms,
        })
        .unwrap();

    // Restricted Malloy can still Sync (HTTP read parity)
    state.gateway_hub.handle_client_frame(
        UserId(malloy_id),
        WsClientFrame::Sync(SyncGetDifferenceRequest::new(ChatId(chat_id), 0, 100)),
        &tx_malloy,
    );
    let malloy_sync = tokio::time::timeout(Duration::from_millis(80), rx_malloy.recv())
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(malloy_sync, WsServerFrame::SyncDiff(_)));

    while rx_alice.try_recv().is_ok() {}
    while rx_dave.try_recv().is_ok() {}
    let pts_before = state.db.get_current_pts(ChatId(chat_id)).unwrap();

    state.gateway_hub.handle_client_frame(
        UserId(malloy_id),
        WsClientFrame::SendMessage(SendMessageRequest::new(
            ChatId(chat_id),
            888_777,
            MessageContent::plain_text("RESTRICTED_MALLOY_WS_BYPASS"),
        )),
        &tx_malloy,
    );
    let malloy_send_err = tokio::time::timeout(Duration::from_millis(80), rx_malloy.recv())
        .await
        .unwrap()
        .unwrap();
    assert!(
        matches!(malloy_send_err, WsServerFrame::Error { code: 403, .. }),
        "Restricted Malloy WS SendMessage must fail with 403, got: {malloy_send_err:?}"
    );
    assert_eq!(
        state.db.get_current_pts(ChatId(chat_id)).unwrap(),
        pts_before,
        "Rejected restricted WS SendMessage must not advance pts"
    );
    assert!(tokio::time::timeout(Duration::from_millis(40), rx_alice.recv()).await.is_err());
    assert!(tokio::time::timeout(Duration::from_millis(40), rx_dave.recv()).await.is_err());

    state.gateway_hub.handle_client_frame(
        UserId(malloy_id),
        WsClientFrame::SendTyping(TypingEvent::new(
            ChatId(chat_id),
            UserId(malloy_id),
            TypingAction::Typing,
        )),
        &tx_malloy,
    );
    assert!(
        tokio::time::timeout(Duration::from_millis(40), rx_alice.recv())
            .await
            .is_err(),
        "Restricted Malloy WS SendTyping must not reach Alice"
    );

    // Sibling parity while Malloy is restricted: hermes_im_bridge + HTTP polls_create
    let bridge = state.hermes.platform_im_bridge().unwrap();
    assert!(
        bridge
            .execute_im_tool(
                "send_message",
                &json!({ "chat_id": chat_id, "text": "RESTRICTED_HERMES_BYPASS" }),
                PlatformImCaller {
                    user_id: Some(malloy_id),
                    chat_id: Some(chat_id),
                    ..Default::default()
                },
            )
            .await
            .is_err(),
        "PlatformIm send_message must reject restricted Malloy"
    );
    assert!(
        bridge
            .execute_im_tool(
                "create_poll",
                &json!({ "chat_id": chat_id, "question": "Bypass?", "options": ["Y", "N"] }),
                PlatformImCaller {
                    user_id: Some(malloy_id),
                    chat_id: Some(chat_id),
                    ..Default::default()
                },
            )
            .await
            .is_err(),
        "PlatformIm create_poll must reject restricted Malloy"
    );
    let malloy_poll_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/chats/{chat_id}/polls"))
                .header(header::AUTHORIZATION, format!("Bearer {malloy_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({ "question": "Restricted Poll?", "options": ["A", "B"] }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(malloy_poll_res.status(), StatusCode::FORBIDDEN);

    // 2b. Live TCP/WS Upgrade authz smoke check (handle_ws_socket_with_session_weak)
    {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let srv_app = app.clone();
        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
        let srv_task = tokio::spawn(async move {
            let _ = axum::serve(listener, srv_app)
                .with_graceful_shutdown(async {
                    let _ = shutdown_rx.await;
                })
                .await;
        });

        for bad_url in [
            format!("ws://{addr}/api/v1/ws"),
            format!("ws://{addr}/api/v1/ws?token=invalid_ws_token_xyz"),
        ] {
            let err = tokio_tungstenite::connect_async(&bad_url).await.unwrap_err();
            match err {
                tokio_tungstenite::tungstenite::Error::Http(resp) => {
                    assert_eq!(resp.status().as_u16(), 401, "Expected 401 on {bad_url}");
                }
                other => panic!("Expected HTTP 401 on {bad_url}, got {other:?}"),
            }
        }

        let (mut eve_live_ws, _) =
            tokio_tungstenite::connect_async(format!("ws://{addr}/api/v1/ws?token={eve_tok}"))
                .await
                .unwrap();

        let ping_txt = serde_json::to_string(&WsClientFrame::Ping { nonce: 77 }).unwrap();
        eve_live_ws.send(WsMsg::Text(ping_txt.into())).await.unwrap();
        let pong_msg = tokio::time::timeout(Duration::from_secs(2), eve_live_ws.next())
            .await
            .unwrap()
            .unwrap()
            .unwrap();
        let pong_frame: WsServerFrame =
            serde_json::from_str(pong_msg.to_text().unwrap()).unwrap();
        assert!(matches!(pong_frame, WsServerFrame::Pong { nonce: 77 }));

        eve_live_ws
            .send(WsMsg::Text("{\"type\":\"NotARealFrame\"}".into()))
            .await
            .unwrap();
        let bad_msg = tokio::time::timeout(Duration::from_secs(2), eve_live_ws.next())
            .await
            .unwrap()
            .unwrap()
            .unwrap();
        let bad_frame: WsServerFrame =
            serde_json::from_str(bad_msg.to_text().unwrap()).unwrap();
        assert!(matches!(bad_frame, WsServerFrame::Error { code: 400, .. }));

        let eve_sync_txt = serde_json::to_string(&WsClientFrame::Sync(
            SyncGetDifferenceRequest::new(ChatId(chat_id), 0, 50),
        ))
        .unwrap();
        eve_live_ws
            .send(WsMsg::Text(eve_sync_txt.into()))
            .await
            .unwrap();
        let eve_live_resp = tokio::time::timeout(Duration::from_secs(2), eve_live_ws.next())
            .await
            .unwrap()
            .unwrap()
            .unwrap();
        let eve_live_frame: WsServerFrame =
            serde_json::from_str(eve_live_resp.to_text().unwrap()).unwrap();
        assert!(
            matches!(eve_live_frame, WsServerFrame::Error { code: 403, .. }),
            "Live WS Sync from non-member Eve must fail with 403: {eve_live_frame:?}"
        );

        let (mut malloy_live_ws, _) = tokio_tungstenite::connect_async(format!(
            "ws://{addr}/api/v1/ws?token={malloy_tok}"
        ))
        .await
        .unwrap();
        let malloy_send_txt = serde_json::to_string(&WsClientFrame::SendMessage(
            SendMessageRequest::new(
                ChatId(chat_id),
                888_778,
                MessageContent::plain_text("LIVE_WS_RESTRICTED_BYPASS"),
            ),
        ))
        .unwrap();
        malloy_live_ws
            .send(WsMsg::Text(malloy_send_txt.into()))
            .await
            .unwrap();
        let malloy_live_resp =
            tokio::time::timeout(Duration::from_secs(2), malloy_live_ws.next())
                .await
                .unwrap()
                .unwrap()
                .unwrap();
        let malloy_live_frame: WsServerFrame =
            serde_json::from_str(malloy_live_resp.to_text().unwrap()).unwrap();
        assert!(
            matches!(malloy_live_frame, WsServerFrame::Error { code: 403, .. }),
            "Live WS SendMessage from restricted Malloy must fail with 403: {malloy_live_frame:?}"
        );

        drop(eve_live_ws);
        drop(malloy_live_ws);
        let _ = shutdown_tx.send(());
        let _ = tokio::time::timeout(Duration::from_secs(2), srv_task).await;
    }

    // Non-muted restricted Malloy (banned_rights = 2, bit 0 clear) CAN send
    state
        .db
        .add_chat_member(&ChatMemberRecord {
            chat_id: ChatId(chat_id),
            user_id: UserId(malloy_id),
            role: "restricted".to_string(),
            permissions_bitmask: 0,
            admin_title: None,
            admin_rights: 0,
            banned_rights: 2,
            until_date_ms: None,
            joined_at_ms: now_ms,
            invited_by: Some(UserId(alice_id)),
            updated_at_ms: now_ms,
        })
        .unwrap();
    state.gateway_hub.handle_client_frame(
        UserId(malloy_id),
        WsClientFrame::SendMessage(SendMessageRequest::new(
            ChatId(chat_id),
            888_779,
            MessageContent::plain_text("RESTRICTED_NON_MUTED_OK"),
        )),
        &tx_malloy,
    );
    let malloy_ok = tokio::time::timeout(Duration::from_millis(80), rx_malloy.recv())
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(malloy_ok, WsServerFrame::Ack { .. }));

    // 3. Bidirectional DM block (chat_type 0 | 4) & Group (chat_type 2) isolation
    let dm_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/chats")
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({ "peer_user_id": bob_id, "chat_type": 0 }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(dm_res.status(), StatusCode::CREATED);
    let dm_cid = serde_json::from_slice::<Value>(
        &to_bytes(dm_res.into_body(), usize::MAX).await.unwrap(),
    )
    .unwrap()["chat_id"]
        .as_i64()
        .unwrap();

    let bot_rec = state
        .db
        .create_registered_bot(
            alice_id as i64,
            "t5_w5_dm_bot",
            "W5 DM Bot",
            "Bot",
            "Bot",
            "Bot",
            "",
            "",
            "[]",
            false,
            false,
        )
        .unwrap();
    let bot_uid = bot_rec.bot_user_id as u64;
    let (tx_bot, mut rx_bot) = mpsc::unbounded_channel();
    state
        .gateway_hub
        .register_session(UserId(bot_uid), Some("w5_bot".into()), tx_bot.clone());

    let bot_dm_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/chats")
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({ "peer_user_id": bot_uid, "chat_type": 4 }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bot_dm_res.status(), StatusCode::CREATED);
    let bot_dm_cid = serde_json::from_slice::<Value>(
        &to_bytes(bot_dm_res.into_body(), usize::MAX).await.unwrap(),
    )
    .unwrap()["chat_id"]
        .as_i64()
        .unwrap();

    // Shared group (chat_type 2) with Alice and Bob must remain unaffected by DM block
    let grp2_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/chats")
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "title": "W5 Group Unblocked",
                        "chat_type": 2,
                        "member_ids": [bob_id]
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(grp2_res.status(), StatusCode::CREATED);
    let grp2_cid = serde_json::from_slice::<Value>(
        &to_bytes(grp2_res.into_body(), usize::MAX).await.unwrap(),
    )
    .unwrap()["chat_id"]
        .as_i64()
        .unwrap();

    // Alice blocks Bob and Bot
    for target in [bob_id, bot_uid] {
        let blk = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/api/v1/users/{target}/block"))
                    .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(blk.status(), StatusCode::OK);
    }

    while rx_alice.try_recv().is_ok() {}
    while rx_bob.try_recv().is_ok() {}
    while rx_bot.try_recv().is_ok() {}

    let dm_pts_before = state.db.get_current_pts(ChatId(dm_cid)).unwrap();
    let bot_dm_pts_before = state.db.get_current_pts(ChatId(bot_dm_cid)).unwrap();

    // Both Bob (blocked) and Alice (blocker) rejected on DM (0) and Bot DM (4)
    for (uid, cid, nonce) in [
        (bob_id, dm_cid, 9101u64),
        (alice_id, dm_cid, 9102u64),
        (bot_uid, bot_dm_cid, 9103u64),
        (alice_id, bot_dm_cid, 9104u64),
    ] {
        let (tx, rx) = if uid == bob_id {
            (&tx_bob, &mut rx_bob)
        } else if uid == bot_uid {
            (&tx_bot, &mut rx_bot)
        } else {
            (&tx_alice, &mut rx_alice)
        };
        state.gateway_hub.handle_client_frame(
            UserId(uid),
            WsClientFrame::SendMessage(SendMessageRequest::new(
                ChatId(cid),
                nonce,
                MessageContent::plain_text("BLOCKED_DM_ATTEMPT"),
            )),
            tx,
        );
        let err_frame = tokio::time::timeout(Duration::from_millis(80), rx.recv())
            .await
            .unwrap()
            .unwrap();
        assert!(
            matches!(
                err_frame,
                WsServerFrame::Error {
                    code: 403,
                    details: Some(ref d),
                    ..
                } if d == "user_is_blocked"
            ),
            "Blocked DM SendMessage for uid={uid} cid={cid} must fail with 403 user_is_blocked, got: {err_frame:?}"
        );
    }
    assert_eq!(state.db.get_current_pts(ChatId(dm_cid)).unwrap(), dm_pts_before);
    assert_eq!(
        state.db.get_current_pts(ChatId(bot_dm_cid)).unwrap(),
        bot_dm_pts_before
    );
    assert!(tokio::time::timeout(Duration::from_millis(40), rx_alice.recv()).await.is_err());
    assert!(tokio::time::timeout(Duration::from_millis(40), rx_bob.recv()).await.is_err());

    // Blocked Bob SendTyping in DM dropped silently
    state.gateway_hub.handle_client_frame(
        UserId(bob_id),
        WsClientFrame::SendTyping(TypingEvent::new(
            ChatId(dm_cid),
            UserId(bob_id),
            TypingAction::Typing,
        )),
        &tx_bob,
    );
    assert!(tokio::time::timeout(Duration::from_millis(40), rx_alice.recv()).await.is_err());

    // Sibling parity in blocked DM: hermes_im_bridge & HTTP polls_create
    for uid in [bob_id, alice_id] {
        assert!(
            bridge
                .execute_im_tool(
                    "send_message",
                    &json!({ "chat_id": dm_cid, "text": "blocked_im_msg" }),
                    PlatformImCaller {
                        user_id: Some(uid),
                        ..Default::default()
                    },
                )
                .await
                .is_err()
        );
        assert!(
            bridge
                .execute_im_tool(
                    "create_poll",
                    &json!({ "chat_id": dm_cid, "question": "Blocked?", "options": ["Y", "N"] }),
                    PlatformImCaller {
                        user_id: Some(uid),
                        ..Default::default()
                    },
                )
                .await
                .is_err()
        );
    }
    for tok in [&bob_tok, &alice_tok] {
        let p_res = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/api/v1/chats/{dm_cid}/polls"))
                    .header(header::AUTHORIZATION, format!("Bearer {tok}"))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        json!({ "question": "Blocked?", "options": ["Y", "N"] }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(p_res.status(), StatusCode::FORBIDDEN);
        let p_json: Value =
            serde_json::from_slice(&to_bytes(p_res.into_body(), usize::MAX).await.unwrap())
                .unwrap();
        assert_eq!(p_json["error"].as_str(), Some("user_is_blocked"));
    }

    // Group chat (chat_type == 2) works normally while Bob is DM-blocked
    state.gateway_hub.handle_client_frame(
        UserId(bob_id),
        WsClientFrame::SendMessage(SendMessageRequest::new(
            ChatId(grp2_cid),
            9201,
            MessageContent::plain_text("GROUP_ALLOWED_WHILE_DM_BLOCKED"),
        )),
        &tx_bob,
    );
    let grp_ack = tokio::time::timeout(Duration::from_millis(80), rx_bob.recv())
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(grp_ack, WsServerFrame::Ack { .. }));

    let grp_poll_ok = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/chats/{grp2_cid}/polls"))
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({ "question": "Group Poll?", "options": ["1", "2"] }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(grp_poll_ok.status(), StatusCode::CREATED);

    // 4. Call invite block parity (Wave 6: HTTP POST /api/v1/calls/invite + WS CallSignal invite)
    while rx_alice.try_recv().is_ok() {}
    while rx_bob.try_recv().is_ok() {}
    for (caller_tok, callee_id) in [(&bob_tok, alice_id), (&alice_tok, bob_id)] {
        let inv_blk = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/calls/invite")
                    .header(header::AUTHORIZATION, format!("Bearer {caller_tok}"))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        json!({ "callee_id": callee_id, "is_video": false }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            inv_blk.status(),
            StatusCode::FORBIDDEN,
            "Blocked HTTP calls/invite must return 403"
        );
        let inv_json: Value =
            serde_json::from_slice(&to_bytes(inv_blk.into_body(), usize::MAX).await.unwrap())
                .unwrap();
        assert_eq!(inv_json["error"].as_str(), Some("user_is_blocked"));
    }

    state.gateway_hub.handle_client_frame(
        UserId(bob_id),
        WsClientFrame::CallSignal {
            call_id: "ws_blocked_call_1".into(),
            signal_type: "invite".into(),
            target_user_id: Some(alice_id),
            sdp: None,
            candidate: None,
            sdp_mid: None,
            sdp_mline_index: None,
        },
        &tx_bob,
    );
    let ws_call_err = tokio::time::timeout(Duration::from_millis(80), rx_bob.recv())
        .await
        .unwrap()
        .unwrap();
    assert!(
        matches!(
            ws_call_err,
            WsServerFrame::Error {
                code: 403,
                details: Some(ref d),
                ..
            } if d == "user_is_blocked"
        ),
        "Blocked WS CallSignal invite must return 403 user_is_blocked, got: {ws_call_err:?}"
    );
    assert!(
        tokio::time::timeout(Duration::from_millis(40), rx_alice.recv())
            .await
            .is_err(),
        "Alice must receive 0 CallUpdated frames from blocked Bob"
    );

    // 5. Unblock Bob -> DM messaging & call invite succeed immediately
    let unblk = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/users/{bob_id}/block"))
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(unblk.status(), StatusCode::OK);

    state.gateway_hub.handle_client_frame(
        UserId(bob_id),
        WsClientFrame::SendMessage(SendMessageRequest::new(
            ChatId(dm_cid),
            9301,
            MessageContent::plain_text("DM_OK_AFTER_UNBLOCK"),
        )),
        &tx_bob,
    );
    let dm_ack = tokio::time::timeout(Duration::from_millis(80), rx_bob.recv())
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(dm_ack, WsServerFrame::Ack { .. }));
    let alice_push = tokio::time::timeout(Duration::from_millis(80), rx_alice.recv())
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(alice_push, WsServerFrame::PtsPush(_)));

    let inv_ok = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/calls/invite")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({ "callee_id": alice_id, "is_video": false }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(inv_ok.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn tier5_pins_delete_revoke_and_ghost_cleanup() {
    let (_dir, state, app) = open_ephemeral_server();
    let (_alice_id, alice_tok) = register_user(&app, "pins_alice").await;
    let (bob_id, bob_tok) = register_user(&app, "pins_bob").await;
    let (_eve_id, eve_tok) = register_user(&app, "pins_eve").await;

    // 1. Alice creates a group chat with Bob
    let chat_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/chats")
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "title": "Pins Delete Cleanup Group",
                        "chat_type": 2,
                        "member_ids": [bob_id]
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(chat_res.status(), StatusCode::CREATED);
    let cid = serde_json::from_slice::<Value>(
        &to_bytes(chat_res.into_body(), usize::MAX).await.unwrap(),
    )
    .unwrap()["chat_id"]
        .as_i64()
        .unwrap();

    // 2. Post m1, m2, m3 and pin all three
    let mut mids = Vec::new();
    for (idx, text) in [
        "KEEP_PIN_1",
        "DELETED_PIN_REVOKED_SECRET_999",
        "DELETED_PIN_LOCAL_SECRET_888",
    ]
    .into_iter()
    .enumerate()
    {
        let send_res = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/api/v1/chats/{cid}/messages"))
                    .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        json!({ "text": text, "client_nonce": 8100 + idx as i64 }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(send_res.status(), StatusCode::CREATED);
        let mid = serde_json::from_slice::<Value>(
            &to_bytes(send_res.into_body(), usize::MAX).await.unwrap(),
        )
        .unwrap()["message_id"]
            .as_i64()
            .unwrap();
        let pin_res = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/api/v1/chats/{cid}/pins"))
                    .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(json!({ "message_id": mid }).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(pin_res.status(), StatusCode::OK);
        mids.push(mid);
    }
    let (m1, m2, m3) = (mids[0], mids[1], mids[2]);

    // 3. Outsider Eve gets 403; member Bob gets [m3, m2, m1] and chats/{cid}.pinned_message_id == m3
    let eve_pins = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/chats/{cid}/pins"))
                .header(header::AUTHORIZATION, format!("Bearer {eve_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(eve_pins.status(), StatusCode::FORBIDDEN);

    let bob_pins_1 = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/chats/{cid}/pins"))
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bob_pins_1.status(), StatusCode::OK);
    let bob_pins_1_json: Value = serde_json::from_slice(
        &to_bytes(bob_pins_1.into_body(), usize::MAX).await.unwrap(),
    )
    .unwrap();
    assert_eq!(bob_pins_1_json["pinned_message_ids"], json!([m3, m2, m1]));

    let chat_1 = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/chats/{cid}"))
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(chat_1.status(), StatusCode::OK);
    let chat_1_json: Value = serde_json::from_slice(
        &to_bytes(chat_1.into_body(), usize::MAX).await.unwrap(),
    )
    .unwrap();
    assert_eq!(chat_1_json["pinned_message_id"].as_i64(), Some(m3));

    // 4. Revoked delete (?revoke=true) on m2 removes pin and plaintext leak
    let del_m2 = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/chats/{cid}/messages/{m2}?revoke=true"))
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(del_m2.status(), StatusCode::OK);

    let bob_pins_2 = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/chats/{cid}/pins"))
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bob_pins_2.status(), StatusCode::OK);
    let bob_pins_2_bytes = to_bytes(bob_pins_2.into_body(), usize::MAX).await.unwrap();
    let bob_pins_2_txt = String::from_utf8_lossy(&bob_pins_2_bytes).to_string();
    assert!(!bob_pins_2_txt.contains("DELETED_PIN_REVOKED_SECRET_999"));
    let bob_pins_2_json: Value = serde_json::from_slice(&bob_pins_2_bytes).unwrap();
    assert_eq!(bob_pins_2_json["pinned_message_ids"], json!([m3, m1]));
    assert_eq!(bob_pins_2_json["pins"].as_array().unwrap().len(), 2);
    assert_eq!(state.db.list_pinned_messages(ChatId(cid)).unwrap(), vec![m3, m1]);

    // 5. Non-revoked delete (?revoke=false) on latest pin m3 removes pin and updates pinned_message_id -> m1
    let del_m3 = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/chats/{cid}/messages/{m3}?revoke=false"))
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(del_m3.status(), StatusCode::OK);

    let bob_pins_3 = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/chats/{cid}/pins"))
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bob_pins_3.status(), StatusCode::OK);
    let bob_pins_3_bytes = to_bytes(bob_pins_3.into_body(), usize::MAX).await.unwrap();
    let bob_pins_3_txt = String::from_utf8_lossy(&bob_pins_3_bytes).to_string();
    assert!(!bob_pins_3_txt.contains("DELETED_PIN_LOCAL_SECRET_888"));
    let bob_pins_3_json: Value = serde_json::from_slice(&bob_pins_3_bytes).unwrap();
    assert_eq!(bob_pins_3_json["pinned_message_ids"], json!([m1]));

    let chat_2 = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/chats/{cid}"))
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(chat_2.status(), StatusCode::OK);
    let chat_2_json: Value = serde_json::from_slice(
        &to_bytes(chat_2.into_body(), usize::MAX).await.unwrap(),
    )
    .unwrap();
    assert_eq!(chat_2_json["pinned_message_id"].as_i64(), Some(m1));

    // 6. Delete final remaining pin m1 via POST /api/v1/messages/delete -> pinned_message_id -> null
    let del_m1 = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/messages/delete")
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({ "chat_id": cid, "message_ids": [m1], "revoke": true }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(del_m1.status(), StatusCode::OK);

    let bob_pins_4 = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/chats/{cid}/pins"))
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bob_pins_4.status(), StatusCode::OK);
    let bob_pins_4_json: Value = serde_json::from_slice(
        &to_bytes(bob_pins_4.into_body(), usize::MAX).await.unwrap(),
    )
    .unwrap();
    assert_eq!(bob_pins_4_json["pinned_message_ids"], json!([]));
    assert_eq!(bob_pins_4_json["pins"], json!([]));

    let chat_3 = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/chats/{cid}"))
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(chat_3.status(), StatusCode::OK);
    let chat_3_json: Value = serde_json::from_slice(
        &to_bytes(chat_3.into_body(), usize::MAX).await.unwrap(),
    )
    .unwrap();
    assert!(chat_3_json["pinned_message_id"].is_null());

    // 7. Re-pinning deleted message m2 is rejected
    let repin_m2 = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/chats/{cid}/pins"))
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({ "message_id": m2 }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert!(
        matches!(repin_m2.status(), StatusCode::NOT_FOUND | StatusCode::BAD_REQUEST),
        "Re-pinning deleted message must fail with 404/400, got {}",
        repin_m2.status()
    );

    // 8. Defense-in-depth ghost pin DB check: synthetic pinned_messages row pointing to soft-deleted m4
    let send_m4 = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/chats/{cid}/messages"))
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({ "text": "GHOST_DB_PIN_SECRET", "client_nonce": 8199 }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(send_m4.status(), StatusCode::CREATED);
    let m4 = serde_json::from_slice::<Value>(
        &to_bytes(send_m4.into_body(), usize::MAX).await.unwrap(),
    )
    .unwrap()["message_id"]
        .as_i64()
        .unwrap();
    let pin_m4 = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/chats/{cid}/pins"))
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({ "message_id": m4 }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(pin_m4.status(), StatusCode::OK);

    state
        .db
        .pool()
        .with_conn(|conn| {
            conn.execute(
                "UPDATE messages SET deleted_at_ms = 999 WHERE chat_id = ?1 AND message_id = ?2",
                rusqlite::params![cid, m4],
            )
            .map_err(|e| TelegramError::Database(e.to_string()))?;
            Ok(())
        })
        .unwrap();

    let bob_pins_ghost = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/chats/{cid}/pins"))
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bob_pins_ghost.status(), StatusCode::OK);
    let bob_pins_ghost_bytes = to_bytes(bob_pins_ghost.into_body(), usize::MAX)
        .await
        .unwrap();
    let bob_pins_ghost_txt = String::from_utf8_lossy(&bob_pins_ghost_bytes).to_string();
    assert!(!bob_pins_ghost_txt.contains("GHOST_DB_PIN_SECRET"));
    let bob_pins_ghost_json: Value = serde_json::from_slice(&bob_pins_ghost_bytes).unwrap();
    assert_eq!(bob_pins_ghost_json["pinned_message_ids"], json!([]));
    assert!(state.db.list_pinned_messages(ChatId(cid)).unwrap().is_empty());
}
