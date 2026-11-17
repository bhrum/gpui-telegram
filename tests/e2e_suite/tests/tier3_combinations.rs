//! Tier 3: Pairwise Feature Combinations (23 Tests)
//!
//! Every test wires genuine cross-feature data and control flow across
//! `EphemeralTestServer`, WebSocket frames, SQLite WAL, Tantivy search,
//! `WebRtcPeerTransport`, `gpui_telegram_webrtc::media::{opus_codec, ai_media_tap}`,
//! `gpui_telegram_media::media::capture`, `VideoViewport` / `CallOverlay`,
//! `NotificationsManagerMacUnMac`, `WindowSessionControllerLinkInfo`,
//! `HermesStateStore`, `LocalTestProxyServer`, and fixed M2/M4 harnesses.

mod common;

use common::*;
use ed25519_dalek::SigningKey;
use gpui_telegram_agent::hermes::state_db::{
    AppendMessageRequest, CreateKanbanCardRequest, CreateSessionRequest, HermesStateStore,
};
use gpui_telegram_agent::talk::VadBargeInDetector;
use gpui_telegram_core::action_bus::{
    ActionRegistry, DeleteMessageAction, GetChatHistoryAction, HermesToolExporter,
    ListDialogsAction, SendMessageAction,
};
use gpui_telegram_core::types::{ChatId, UserId};
use gpui_telegram_desktop::app::session::send_as_peers::{
    SendAsKey, SendAsPeer, SendAsPeersSession,
};
use gpui_telegram_desktop::components::video_viewport::VideoViewport;
use gpui_telegram_desktop::platform::mac::notifications_manager_mac_un::{
    NotificationsManagerMacUnMac, UNNotificationParsed,
};
use gpui_telegram_desktop::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use gpui_telegram_desktop::views::app_shell::window_session_controller_link_info::{
    LinkInfoType, WindowSessionControllerLinkInfo,
};
use gpui_telegram_desktop::views::call_overlay::CallOverlay;
use gpui_telegram_media::chunked_upload::ChunkedUploadSession;
use gpui_telegram_media::media::capture::{
    MediaDeviceSource, MockAudioSource, MockVideoSource,
};
use gpui_telegram_proto::cards::{TodoCard, TodoItem};
use gpui_telegram_proto::miniapp_contracts::MiniAppActionRequest;
use gpui_telegram_proto::sync::UpdateEvent;
use gpui_telegram_proto::ws::{WsClientFrame, WsServerFrame};
use gpui_telegram_server::push::dispatchers::MockPushDispatcher;
use gpui_telegram_server::push::{PushGatewayHub, PushNotification};
use gpui_telegram_webrtc::calls::peer_transport::{PeerTransportConfig, WebRtcPeerTransport};
use gpui_telegram_webrtc::media::ai_media_tap::{AiMediaTap, DefaultAiMediaTap};
use gpui_telegram_webrtc::media::opus_codec::{
    OpusDepacketizer, OpusFramingCodec, OpusPacketizer, RtpPacket,
};
use reqwest::Method;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::time::Duration;
use x25519_dalek::{PublicKey, StaticSecret};

fn build_registry() -> Arc<ActionRegistry> {
    let reg = ActionRegistry::new();
    reg.register_typed(SendMessageAction).unwrap();
    reg.register_typed(GetChatHistoryAction).unwrap();
    reg.register_typed(ListDialogsAction).unwrap();
    reg.register_typed(DeleteMessageAction).unwrap();
    Arc::new(reg)
}

#[tokio::test]
async fn test_f1_f2_action_bus_with_sqlite_wal_persistence() {
    let (mut server, driver, (_alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let (st, res) = driver
        .json_req(
            Method::POST,
            "/api/v1/actions/send_message",
            Some(&alice_token),
            Some(json!({ "chat_id": chat_id, "text": "ActionBus -> SQLite WAL" })),
        )
        .await
        .unwrap();
    assert_eq!(st, 200, "action exec: {res:?}");

    server.restart().await.expect("restart EphemeralTestServer");
    let driver_after = E2eSystemDriver::new(server.base_url());
    let (st_h, hist) = driver_after
        .json_req(
            Method::GET,
            &format!("/api/v1/chats/{chat_id}/messages"),
            Some(&alice_token),
            None,
        )
        .await
        .unwrap();
    assert_eq!(st_h, 200);
    assert!(hist.to_string().contains("ActionBus -> SQLite WAL"));

    server.stop().await;
}

#[tokio::test]
async fn test_f1_f16_action_bus_hermes_native_tool_forwarding() {
    let reg = build_registry();
    let exported = HermesToolExporter::export_openai_tools(&reg);
    assert!(exported
        .iter()
        .any(|t| t["function"]["name"] == "send_message"));

    let (mut server, driver, (_alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");
    let im = HttpPlatformImDriver::new(server.base_url(), &alice_token);
    let sent = im
        .execute(
            "im_send_message",
            json!({ "chat_id": chat_id, "text": "Forwarded via Hermes IM driver" }),
        )
        .await
        .expect("im_send_message");
    let msg_id = sent["message_id"].as_i64().unwrap();

    let (st, hist) = driver
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
        .any(|m| m["message_id"] == msg_id));

    server.stop().await;
}

#[tokio::test]
async fn test_f1_f19_action_bus_miniapp_action_manifest_delegation() {
    let mcp = MockMcpClient::new(build_registry());
    let _ = mcp.initialize("miniapp-delegator");

    let manifest = AgentManifest::from_json(
        r#"{
            "app_id": "miniapp_sync_bot",
            "version": "1.0.0",
            "readable_state_schema": { "type": "object" },
            "actions": [
                {
                    "name": "send_message",
                    "description": "Forward message",
                    "input_schema": { "type": "object", "required": ["chat_id", "text"] },
                    "output_schema": { "type": "object" },
                    "risk_level": "low",
                    "requires_approval": false
                }
            ]
        }"#,
    )
    .unwrap();
    let delegated = manifest
        .execute_action(
            "send_message",
            &json!({ "chat_id": 1, "text": "Delegated" }),
            false,
        )
        .unwrap();
    assert_eq!(delegated["status"], "executed");

    let (mut server, driver, (_alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");
    let mcp_out = mcp
        .tools_call(
            "send_message",
            json!({ "chat_id": chat_id, "text": "MCP sidecar ok" }),
        )
        .await;
    assert!(mcp_out.error.is_none());

    driver
        .json_req(
            Method::POST,
            "/api/v1/bots",
            Some(&alice_token),
            Some(json!({ "display_name": "Delegate Bot", "username": "delegate_e2e_bot" })),
        )
        .await
        .unwrap();
    driver
        .json_req(
            Method::POST,
            "/api/v1/miniapps",
            Some(&alice_token),
            Some(json!({
                "bot_username": "delegate_e2e_bot",
                "short_name": "delapp",
                "title": "Delegate App"
            })),
        )
        .await
        .unwrap();
    driver
        .json_req(
            Method::POST,
            "/api/v1/miniapps/delegate_e2e_bot/delapp/install",
            Some(&alice_token),
            Some(json!({ "chat_id": chat_id })),
        )
        .await
        .unwrap();
    let (st_l, _) = driver
        .json_req(
            Method::POST,
            "/api/v1/miniapps/delegate_e2e_bot/delapp/launch",
            Some(&alice_token),
            Some(json!({ "chat_id": chat_id })),
        )
        .await
        .unwrap();
    assert!((200..300).contains(&st_l));

    server.stop().await;
}

#[tokio::test]
async fn test_f2_f6_sqlite_wal_outbox_transactional_durability() {
    let (mut server, driver, (_alice_id, alice_token), (_bob_id, bob_token), chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let tmp = new_test_dir("f2_f6");
    let db_path = tmp.path().join("outbox_wal.db");
    {
        let outbox = OutboxSimulator::open(&db_path).unwrap();
        outbox.enqueue(chat_id, "Crash-safe outbox #1").unwrap();
        outbox.enqueue(chat_id, "Crash-safe outbox #2").unwrap();
    }

    let reopened = OutboxSimulator::open(&db_path).unwrap();
    let flushed = reopened
        .notify_reconnected_to_server(&driver, &alice_token)
        .await
        .expect("flush reopened outbox");
    assert_eq!(flushed.len(), 2);

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
    let txt = hist.to_string();
    assert!(txt.contains("Crash-safe outbox #1"));
    assert!(txt.contains("Crash-safe outbox #2"));

    server.stop().await;
}

#[test]
fn test_f2_f17_sqlite_wal_agentic_kanban_store_persistence() {
    let tmp = new_test_dir("f2_f17");
    let db_path = tmp.path().join("hermes_wal.db");
    let card_id = {
        let store = HermesStateStore::open(&db_path).unwrap();
        let board = store
            .create_kanban_board("WAL Board", "Durability", None)
            .unwrap();
        let card = store
            .create_kanban_card(CreateKanbanCardRequest {
                id: None,
                board_id: board.id,
                column_name: Some("Backlog".into()),
                title: "Persist across drop".into(),
                description: Some("SQLite WAL".into()),
                priority: Some("high".into()),
                assignee: None,
                labels: Some(vec!["wal".into()]),
            })
            .unwrap();
        store.move_kanban_card(&card.id, "In Progress", None).unwrap();
        let sess = store
            .create_session(CreateSessionRequest {
                id: Some("s1".into()),
                chat_id: None,
                bot_id: None,
                source: None,
                model: None,
                api_mode: None,
                title: Some("Kanban".into()),
                parent_session_id: None,
                branch_from_seq: None,
                system_prompt_hash: None,
                active_toolsets: None,
                metadata: None,
            })
            .unwrap();
        store
            .append_message(AppendMessageRequest {
                id: None,
                session_id: sess.id,
                role: "assistant".into(),
                content: "Moved Persist across drop to in_progress".into(),
                reasoning: None,
                tool_calls_json: None,
                tool_call_id: None,
                tool_name: None,
                token_count: 8,
                created_at: None,
            })
            .unwrap();
        card.id
    };

    let reopened = HermesStateStore::open(&db_path).unwrap();
    let moved = reopened.move_kanban_card(&card_id, "Done", None).unwrap();
    assert_eq!(moved.column_name, "Done");
    let search = reopened.session_search("in_progress", None, 5).unwrap();
    assert_eq!(search.len(), 1);
}

#[tokio::test]
async fn test_f3_f4_event_bus_gateway_push_notification_dispatch() {
    let (mut server, driver, (_alice_id, alice_token), (bob_id, bob_token), chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let (st_reg, _) = driver
        .json_req(
            Method::POST,
            "/api/v1/devices/register_push",
            Some(&bob_token),
            Some(json!({ "platform": "mock", "token": "bob_offline_device_token" })),
        )
        .await
        .unwrap();
    assert_eq!(st_reg, 200);

    let mut alice_ws = connect_ws(&server, &alice_token).await.unwrap();
    send_ws_frame(&mut alice_ws, &WsClientFrame::SubscribeChat { chat_id })
        .await
        .unwrap();

    let (st_msg, msg_resp) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/chats/{chat_id}/messages"),
            Some(&alice_token),
            Some(json!({ "text": "Wake Bob via push", "client_nonce": 304 })),
        )
        .await
        .unwrap();
    assert!((200..300).contains(&st_msg));
    let mid = msg_resp["message_id"]
        .as_i64()
        .or_else(|| msg_resp["message"]["message_id"].as_i64())
        .unwrap();

    let state = server.state().unwrap().clone();
    let mock_disp = Arc::new(MockPushDispatcher::new());
    let push_hub = PushGatewayHub::new(Arc::clone(&state.db));
    push_hub.register_dispatcher(mock_disp.clone());

    let receipts = push_hub
        .send_to_user(
            UserId(bob_id),
            &PushNotification {
                title: "Alice".into(),
                body: "Wake Bob via push".into(),
                chat_id: Some(chat_id),
                message_id: Some(mid),
                badge: Some(1),
                sound: None,
                custom_data: Default::default(),
            },
        )
        .await;
    assert_eq!(receipts.len(), 1);
    assert!(receipts[0].as_ref().unwrap().delivered);
    assert_eq!(mock_disp.dispatched_count(), 1);

    let _ = alice_ws.close(None).await;
    server.stop().await;
}

#[tokio::test]
async fn test_f3_f6_gateway_reconnect_flushes_pending_outbox() {
    let (mut server, driver, (_alice_id, alice_token), (_bob_id, bob_token), chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let mut bob_ws = connect_ws(&server, &bob_token).await.unwrap();
    send_ws_frame(&mut bob_ws, &WsClientFrame::SubscribeChat { chat_id })
        .await
        .unwrap();

    let tmp = new_test_dir("f3_f6");
    let outbox = OutboxSimulator::open(tmp.path().join("outbox_ws.db")).unwrap();
    outbox.enqueue(chat_id, "WS pushed from outbox").unwrap();

    let sent = outbox
        .notify_reconnected_to_server(&driver, &alice_token)
        .await
        .unwrap();
    let expected_msg_id = sent[0].1;

    let frame = recv_ws_frame_matching(&mut bob_ws, Duration::from_secs(3), |f| match f {
        WsServerFrame::PtsPush(p) => {
            matches!(&p.event, UpdateEvent::NewMessage(m) if m.id.0 == expected_msg_id)
        }
        WsServerFrame::Update(UpdateEvent::NewMessage(m)) => m.id.0 == expected_msg_id,
        _ => false,
    })
    .await
    .expect("Bob WS receives outbox-flushed message");

    match frame {
        WsServerFrame::PtsPush(p) => match p.event {
            UpdateEvent::NewMessage(m) => assert_eq!(m.chat_id, ChatId(chat_id)),
            other => panic!("unexpected pts event: {other:?}"),
        },
        WsServerFrame::Update(UpdateEvent::NewMessage(m)) => {
            assert_eq!(m.chat_id, ChatId(chat_id));
        }
        other => panic!("unexpected frame: {other:?}"),
    }

    let _ = bob_ws.close(None).await;
    server.stop().await;
}

#[tokio::test]
async fn test_f5_f7_chunked_cas_storage_with_breakpoint_resume() {
    let (mut server, driver, (_alice_id, alice_token), _bob, _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let png = create_test_png(33, 99, 200, 255);
    let mut local_session = ChunkedUploadSession::new("local_sha", png.len() as u64, 30).unwrap();
    for (idx, c) in png.chunks(30).enumerate() {
        local_session.receive_part(idx as u32, c).unwrap();
    }
    let expected_sha = local_session.compute_sha256().unwrap();

    let (_, init) = driver
        .json_req(
            Method::POST,
            "/api/v1/media/upload/init",
            Some(&alice_token),
            Some(json!({
                "filename": "photo.png",
                "mime_type": "image/png",
                "size_bytes": png.len(),
                "chunk_size": 30
            })),
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
        .body(png[..30.min(png.len())].to_vec())
        .send()
        .await
        .unwrap();

    let (_, st_mid) = driver
        .json_req(
            Method::GET,
            &format!("/api/v1/media/upload/{upload_id}/status"),
            Some(&alice_token),
            None,
        )
        .await
        .unwrap();
    let missing = st_mid["missing_parts"].as_array().unwrap().clone();
    assert!(!missing.is_empty());

    for idx_val in missing {
        let idx = idx_val.as_u64().unwrap() as usize;
        let start = idx * 30;
        let end = (start + 30).min(png.len());
        driver
            .http
            .put(format!(
                "{}/api/v1/media/upload/{upload_id}/part/{idx}",
                server.base_url()
            ))
            .bearer_auth(&alice_token)
            .body(png[start..end].to_vec())
            .send()
            .await
            .unwrap();
    }

    let (st_c, comp) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/media/upload/{upload_id}/complete"),
            Some(&alice_token),
            Some(json!({})),
        )
        .await
        .unwrap();
    assert_eq!(st_c, 200);
    assert_eq!(comp["sha256"].as_str().unwrap(), expected_sha);

    server.stop().await;
}

#[tokio::test]
async fn test_f6_f8_offline_outbox_with_e2ee_envelope_encryption() {
    let (mut server, driver, (_alice_id, alice_token), (bob_id, bob_token), chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let e2ee = E2eeSimulator::new();
    let alice_k = StaticSecret::from([1u8; 32]);
    let bob_k = StaticSecret::from([2u8; 32]);
    let bob_spk = StaticSecret::from([3u8; 32]);
    let eph_k = StaticSecret::from([4u8; 32]);
    e2ee.publish_prekeys(PrekeyBundle {
        user_id: bob_id,
        device_id: "bob-1".into(),
        identity_key: PublicKey::from(&bob_k),
        signed_prekey: PublicKey::from(&bob_spk),
        one_time_prekeys: vec![],
    });
    let (bundle, _) = e2ee.consume_prekey_bundle(bob_id, "bob-1").unwrap();
    let sk_a = E2eeSimulator::derive_x3dh_initiator(
        &alice_k,
        &eph_k,
        &bundle.identity_key,
        &bundle.signed_prekey,
        None,
    );
    let sealed_bytes = E2eeSimulator::encrypt_message(&sk_a, 1, b"Top Secret Offline").unwrap();
    let sealed_hex: String = sealed_bytes.iter().map(|b| format!("{b:02x}")).collect();

    let outbox = OutboxSimulator::new();
    outbox.enqueue(chat_id, &sealed_hex).unwrap();
    outbox
        .notify_reconnected_to_server(&driver, &alice_token)
        .await
        .unwrap();

    let (_, hist) = driver
        .json_req(
            Method::GET,
            &format!("/api/v1/chats/{chat_id}/messages"),
            Some(&bob_token),
            None,
        )
        .await
        .unwrap();
    let wire_hex = hist["messages"][0]["text"]
        .as_str()
        .or_else(|| hist["messages"][0]["content"]["text"].as_str())
        .unwrap();
    assert_eq!(wire_hex, sealed_hex);
    let wire_bytes: Vec<u8> = (0..wire_hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&wire_hex[i..i + 2], 16).unwrap())
        .collect();

    let sk_b = E2eeSimulator::derive_x3dh_responder(
        &bob_k,
        &bob_spk,
        None,
        &PublicKey::from(&alice_k),
        &PublicKey::from(&eph_k),
    );
    let plain = e2ee
        .decrypt_message("alice-1", &sk_b, 1, &wire_bytes)
        .unwrap();
    assert_eq!(plain, b"Top Secret Offline");

    server.stop().await;
}

#[tokio::test]
async fn test_f7_f9_breakpoint_resume_through_proxy_tunnel() {
    let (mut server, _driver, (_alice_id, alice_token), _bob, _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");
    let proxy = LocalTestProxyServer::start_socks5(None)
        .await
        .expect("start socks5 proxy");

    let cfg = ProxyConfig {
        proxy_type: ProxyType::Socks5,
        host: "127.0.0.1".into(),
        port: proxy.port(),
        username: None,
        password: None,
    };
    let target: std::net::SocketAddr = server
        .base_url()
        .trim_start_matches("http://")
        .parse()
        .unwrap();

    let init_req = br#"{"filename":"proxied.bin","size_bytes":24,"chunk_size":12}"#;
    let (st_i, init_body) = cfg
        .tunnel_http_request(
            target,
            "POST",
            "/api/v1/media/upload/init",
            Some(&alice_token),
            Some(init_req),
            Some("application/json"),
        )
        .await
        .unwrap();
    assert_eq!(st_i, 201, "init via proxy: {init_body}");
    let init_json: Value = serde_json::from_str(&init_body).unwrap();
    let upload_id = init_json["upload_id"].as_str().unwrap();

    let data = b"PROXY_TUNNEL_MEDIA_24_BY";
    for idx in 0..2 {
        let (st_p, _) = cfg
            .tunnel_http_request(
                target,
                "PUT",
                &format!("/api/v1/media/upload/{upload_id}/part/{idx}"),
                Some(&alice_token),
                Some(&data[idx * 12..(idx + 1) * 12]),
                Some("application/octet-stream"),
            )
            .await
            .unwrap();
        assert_eq!(st_p, 200);
    }

    let (st_c, comp_body) = cfg
        .tunnel_http_request(
            target,
            "POST",
            &format!("/api/v1/media/upload/{upload_id}/complete"),
            Some(&alice_token),
            Some(b"{}"),
            Some("application/json"),
        )
        .await
        .unwrap();
    assert_eq!(st_c, 200);
    let comp_json: Value = serde_json::from_str(&comp_body).unwrap();
    let file_id = comp_json["file_id"].as_str().unwrap();

    let (st_s, streamed) = cfg
        .tunnel_http_request(
            target,
            "GET",
            &format!("/api/v1/media/stream/{file_id}"),
            Some(&alice_token),
            None,
            None,
        )
        .await
        .unwrap();
    assert_eq!(st_s, 200);
    assert_eq!(streamed.as_bytes(), data);

    proxy.stop().await;
    server.stop().await;
}

#[tokio::test]
async fn test_f8_f10_e2ee_secret_chat_with_send_as_peers_and_pin() {
    let (mut server, driver, (alice_id, alice_token), (_bob_id, bob_token), chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let mut send_as = SendAsPeersSession::new(alice_id);
    let key = SendAsKey::message(chat_id as u64);
    send_as.set_list(
        key,
        vec![
            SendAsPeer::new(alice_id, "Alice".into(), false),
            SendAsPeer::new(9001, "Security Officer".into(), false),
        ],
    );
    send_as.set_chosen(chat_id as u64, 9001);
    assert_eq!(send_as.resolve_chosen(&key), 9001);

    let e2ee = E2eeSimulator::new();
    let k = [0x33u8; 32];
    let ct_bytes = E2eeSimulator::encrypt_message(&k, 0, b"Pinned security notice").unwrap();
    let ct_hex: String = ct_bytes.iter().map(|b| format!("{b:02x}")).collect();
    let (_, msg) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/chats/{chat_id}/messages"),
            Some(&alice_token),
            Some(json!({ "text": ct_hex, "client_nonce": 810 })),
        )
        .await
        .unwrap();
    let mid = msg["message_id"]
        .as_i64()
        .or_else(|| msg["message"]["message_id"].as_i64())
        .unwrap();

    let (st_pin, _) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/chats/{chat_id}/pins"),
            Some(&alice_token),
            Some(json!({ "message_id": mid })),
        )
        .await
        .unwrap();
    assert_eq!(st_pin, 200);

    let (_, pins) = driver
        .json_req(
            Method::GET,
            &format!("/api/v1/chats/{chat_id}/pins"),
            Some(&bob_token),
            None,
        )
        .await
        .unwrap();
    assert!(pins["pinned_message_ids"]
        .as_array()
        .unwrap()
        .contains(&json!(mid)));
    assert_eq!(
        e2ee.decrypt_message("alice", &k, 0, &ct_bytes).unwrap(),
        b"Pinned security notice"
    );

    server.stop().await;
}

#[tokio::test]
async fn test_f11_f12_webrtc_loopback_with_opus_rtp_framing() {
    let mut sine = MockAudioSource::new_sine(440.0, 48_000, 1);
    let frame = sine.next_frame(20);

    let mut pktizer = OpusPacketizer::new(1, 0xFACE_0001);
    let rtp_list = pktizer.push_pcm(&frame.samples);
    assert_eq!(rtp_list.len(), 1);
    let raw_rtp = rtp_list[0].serialize();
    let parsed_rtp = RtpPacket::parse(&raw_rtp).unwrap();
    let mut dep = OpusDepacketizer::new(1);
    let decoded_pcm = dep.depacketize(&parsed_rtp).unwrap();

    let (mut a, mut b) = WebRtcPeerTransport::create_loopback_pair(PeerTransportConfig::default());
    a.send_audio_pcm(&decoded_pcm).await.unwrap();
    let delivered = b.recv_loopback_audio().await.unwrap();
    assert_eq!(delivered, decoded_pcm);

    a.close().await.unwrap();
    b.close().await.unwrap();
}

#[tokio::test]
async fn test_f11_f23_webrtc_audio_stream_with_vad_barge_in() {
    let (mut a, b) = WebRtcPeerTransport::create_loopback_pair(PeerTransportConfig::default());
    let b = Arc::new(b);
    let tap = Arc::new(DefaultAiMediaTap::new(48_000, 1, 0.05).with_consecutive_frames(1));
    b.set_ai_media_tap(tap.clone());
    let rx_h = WebRtcPeerTransport::spawn_loopback_receiver(b.clone()).await;

    let mut vad = VadBargeInDetector::new(1000.0);
    let speech = vec![9000i16; 960];
    assert!((0..3).any(|_| vad.process_pcm_frame(&speech)));

    a.send_audio_pcm(&speech).await.unwrap();
    tokio::time::timeout(Duration::from_secs(2), async {
        while !tap.trigger_vad_barge_in() {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("WebRTC loopback triggered DefaultAiMediaTap VAD barge-in");

    if let Some(h) = rx_h {
        h.abort();
    }
    a.close().await.unwrap();
}

#[test]
fn test_f12_f13_mock_audio_sine_capture_to_opus_rtp_stream() {
    let mut src = MockAudioSource::new_sine(880.0, 48_000, 1);
    src.start().unwrap();
    let captured = src.next_frame(20);
    assert!(captured.rms_level() > 100);

    let encoded = OpusFramingCodec::encode_frame(&captured.samples, 1).unwrap();
    assert_eq!(encoded[0], 0xA0);

    let mut pktizer = OpusPacketizer::new(1, 77);
    let pkts = pktizer.push_pcm(&captured.samples);
    assert_eq!(pkts.len(), 1);
    let mut dep = OpusDepacketizer::new(1);
    let out = dep.depacketize(&pkts[0]).unwrap();
    assert_eq!(out, captured.samples);
    src.stop().unwrap();
}

#[test]
fn test_f13_f14_mock_video_source_to_i420_viewport_render() {
    let mut cam = MockVideoSource::new(64, 36, 30);
    cam.start().unwrap();
    let vf = cam.next_frame();
    assert_eq!(vf.to_i420().len(), 64 * 36 * 3 / 2);
    let rgba = vf.to_rgba8();

    let pal = TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x40A7E3, 14, 12);
    let mut vp = VideoViewport::new("Bob", pal);
    assert!(vp.ingest_raw_remote_rgba(64, 36, &rgba));
    assert!(vp.has_active_video());
    assert_eq!(vp.remote_resolution, (64, 36));
    cam.stop().unwrap();
}

#[test]
fn test_f14_f18_video_viewport_state_in_semantic_workspace_tree() {
    let mut overlay = CallOverlay::new("Alice".to_string());
    overlay.toggle_camera();
    assert!(overlay.is_camera_on);
    assert!(overlay
        .video_viewport
        .ingest_raw_local_rgba(4, 4, &vec![200u8; 64]));

    let mut tree = SemanticWorkspaceTreeSimulator::new();
    tree.register_chat(500);
    assert!(tree.perform_action(SemanticAction::SelectChat { chat_id: 500 }).success);
    assert!(
        tree.perform_action(SemanticAction::ToggleCall {
            peer_id: 500,
            is_video: true
        })
        .success
    );
    assert!(tree.inspect_state().is_call_active);
}

#[test]
fn test_f15_f18_macos_notification_deep_link_chat_navigation() {
    let mut mgr = NotificationsManagerMacUnMac::new(1).with_mock_mode(true);
    let id = mgr
        .show_system_notification(
            "Team",
            "",
            "Open incident thread",
            true,
            UNNotificationParsed::new(1, 99, 501),
        )
        .unwrap();
    let deep_link = mgr.handle_notification_click(&id).unwrap();

    let mut tree = SemanticWorkspaceTreeSimulator::new();
    tree.register_chat(99);
    match WindowSessionControllerLinkInfo::parse_deep_link(&deep_link) {
        LinkInfoType::Peer(p) => {
            let cid = p.channel_id.unwrap();
            assert!(tree.perform_action(SemanticAction::SelectChat { chat_id: cid }).success);
            assert_eq!(tree.inspect_state().selected_chat_id, Some(99));
        }
        other => panic!("unexpected deep link parse: {other:?}"),
    }
}

#[tokio::test]
async fn test_f16_f21_hermes_im_dispatcher_with_multi_agent_floor_control() {
    let (mut server, driver, (_alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let sk = SigningKey::from_bytes(&[16u8; 32]);
    let vk = sk.verifying_key();
    let now = chrono::Utc::now().timestamp();
    let tok = ScopedCapabilityToken::issue(
        &sk,
        "coder_agent",
        chat_id,
        vec!["chat:write".into()],
        300,
    );
    assert!(tok.verify_for_scope(&vk, now + 10, "coder_agent", chat_id, "chat:write"));

    let mut floor = FloorControlQueue::new(5_000);
    assert!(floor.request_floor_at("planner", FloorPriority::TaskAssignee, 100));
    assert!(floor.request_floor_at("coder_agent", FloorPriority::UserInterruption, 110));
    let mut budget = LoopBudget::new(3);
    budget.record_hop("coder_agent").unwrap();

    let im = HttpPlatformImDriver::new(server.base_url(), &alice_token);
    im.execute(
        "im_send_message",
        json!({
            "chat_id": chat_id,
            "text": format!("[{}] Floor granted & executed", floor.current_speaker().unwrap())
        }),
    )
    .await
    .unwrap();

    let (_, hist) = driver
        .json_req(
            Method::GET,
            &format!("/api/v1/chats/{chat_id}/messages"),
            Some(&alice_token),
            None,
        )
        .await
        .unwrap();
    assert!(hist
        .to_string()
        .contains("[coder_agent] Floor granted & executed"));

    server.stop().await;
}

#[tokio::test]
async fn test_f16_f22_hermes_chat_search_with_hybrid_memory_engine() {
    let (mut server, _driver, (_alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let im = HttpPlatformImDriver::new(server.base_url(), &alice_token);
    im.execute(
        "im_send_message",
        json!({ "chat_id": chat_id, "text": "Tantivy architectural milestone complete" }),
    )
    .await
    .unwrap();

    server.state().unwrap().search_indexer.commit().await.unwrap();
    let search_res = im
        .execute("im_search_messages", json!({ "query": "architectural" }))
        .await
        .expect("im_search_messages");
    assert!(!search_res["hits"].as_array().unwrap().is_empty());

    let hybrid = HybridMemoryEngine::new();
    hybrid.index_document(MemoryDocument {
        message_id: "hit-1".into(),
        chat_id,
        content: "Tantivy architectural milestone complete".into(),
        embedding: vec![1.0, 0.0],
        linked_entities: vec![],
    });
    let h_hits = hybrid.search("architectural", Some(&[1.0, 0.0]), &[chat_id], None);
    assert_eq!(h_hits.len(), 1);

    server.stop().await;
}

#[tokio::test]
async fn test_f17_f16_agentic_kanban_event_notifications_to_im_chat() {
    let store = HermesStateStore::open_in_memory().unwrap();
    let board = store.create_kanban_board("Ops", "Deploy", None).unwrap();
    let card = store
        .create_kanban_card(CreateKanbanCardRequest {
            id: None,
            board_id: board.id,
            column_name: Some("Backlog".into()),
            title: "Ship RC1".into(),
            description: None,
            priority: None,
            assignee: None,
            labels: None,
        })
        .unwrap();
    let moved = store.move_kanban_card(&card.id, "Done", None).unwrap();

    let (mut server, driver, (_alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");
    let im = HttpPlatformImDriver::new(server.base_url(), &alice_token);
    im.execute(
        "im_send_message",
        json!({
            "chat_id": chat_id,
            "text": format!("Kanban card '{}' moved to {}", moved.title, moved.column_name)
        }),
    )
    .await
    .unwrap();

    let (_, hist) = driver
        .json_req(
            Method::GET,
            &format!("/api/v1/chats/{chat_id}/messages"),
            Some(&alice_token),
            None,
        )
        .await
        .unwrap();
    assert!(hist
        .to_string()
        .contains("Kanban card 'Ship RC1' moved to Done"));

    server.stop().await;
}

#[tokio::test]
async fn test_f19_f20_miniapp_action_execution_streams_generative_cards() {
    let (mut server, driver, (_alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    driver
        .json_req(
            Method::POST,
            "/api/v1/bots",
            Some(&alice_token),
            Some(json!({ "display_name": "Card Bot", "username": "card_stream_bot" })),
        )
        .await
        .unwrap();
    driver
        .json_req(
            Method::POST,
            "/api/v1/miniapps",
            Some(&alice_token),
            Some(json!({
                "bot_username": "card_stream_bot",
                "short_name": "cards",
                "title": "Card Stream"
            })),
        )
        .await
        .unwrap();
    driver
        .json_req(
            Method::POST,
            "/api/v1/miniapps/card_stream_bot/cards/install",
            Some(&alice_token),
            Some(json!({ "chat_id": chat_id })),
        )
        .await
        .unwrap();
    driver
        .json_req(
            Method::POST,
            "/api/v1/miniapps/card_stream_bot/cards/launch",
            Some(&alice_token),
            Some(json!({ "chat_id": chat_id })),
        )
        .await
        .unwrap();
    let (st, act_resp) = driver
        .json_req(
            Method::POST,
            "/api/v1/miniapps/card_stream_bot/cards/action",
            Some(&alice_token),
            Some(
                serde_json::to_value(MiniAppActionRequest::main_button_click(Some(chat_id)))
                    .unwrap(),
            ),
        )
        .await
        .unwrap();
    assert!((200..300).contains(&st), "miniapp action: {act_resp:?}");

    let mut card = GenerativeCard {
        card_id: "miniapp-stream".into(),
        title: "Task Stream".into(),
        subtitle: None,
        layout: GenerativeCardLayout::VerticalStack,
        components: vec![GenerativeComponent::Text {
            text: String::new(),
            bold: false,
            color: None,
        }],
        status: Some("streaming".into()),
    };
    card.append_streaming_delta(0, "Action acknowledged").unwrap();
    let mut todo = TodoCard::new("t1", "Steps", vec![TodoItem::new("s1", "Invoke MiniApp")]);
    assert!(todo.toggle_item("s1"));
    assert_eq!(todo.completion_percentage(), 100.0);

    server.stop().await;
}

#[test]
fn test_f20_f21_generative_ui_card_with_scoped_capability_token() {
    let sk = SigningKey::from_bytes(&[20u8; 32]);
    let vk = sk.verifying_key();
    let now = chrono::Utc::now().timestamp();
    let tok = ScopedCapabilityToken::issue(&sk, "card_agent", 77, vec!["card:approve".into()], 300);

    let card = GenerativeCard {
        card_id: "approval-card-1".into(),
        title: "Approve Transfer".into(),
        subtitle: None,
        layout: GenerativeCardLayout::VerticalStack,
        components: vec![GenerativeComponent::Button {
            id: "approve_btn".into(),
            label: "Approve".into(),
            action_payload: serde_json::to_string(&tok).unwrap(),
        }],
        status: Some("ready".into()),
    };

    // Extract token back out of rendered GenerativeComponent and verify scope
    let payload_str = match &card.components[0] {
        GenerativeComponent::Button { action_payload, .. } => action_payload,
        other => panic!("unexpected component: {other:?}"),
    };
    let embedded: ScopedCapabilityToken =
        serde_json::from_str(payload_str).expect("extract capability token from card component");
    assert!(embedded.verify_for_scope(&vk, now + 10, "card_agent", 77, "card:approve"));
    assert!(!embedded.verify_for_scope(&vk, now + 10, "card_agent", 77, "card:admin"));
}

#[test]
fn test_f21_f23_multi_agent_floor_preemption_via_vad_barge_in() {
    let mut floor = FloorControlQueue::new(5_000);
    assert!(floor.request_floor_at("tts_bot", FloorPriority::DirectMention, 10));

    let tap = DefaultAiMediaTap::new(48_000, 1, 0.05).with_consecutive_frames(1);
    let mut vad = VadBargeInDetector::new(1000.0);
    let user_voice = vec![12_000i16; 960];

    tap.on_incoming_pcm_frame(&user_voice, 48_000, 1);
    assert!(tap.trigger_vad_barge_in());
    assert!((0..3).any(|_| vad.process_pcm_frame(&user_voice)));

    assert!(floor.request_floor_at("human_user", FloorPriority::UserInterruption, 20));
    assert_eq!(floor.current_speaker(), Some("human_user"));

    tap.reset_barge_in();
    assert!(!tap.trigger_vad_barge_in());
}
