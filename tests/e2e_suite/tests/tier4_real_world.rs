//! Tier 4: Real-World Complex Multi-Step User Journeys (12 Workflows)
//!
//! Every workflow boots an isolated `EphemeralTestServer` (live SQLite WAL, CAS,
//! Tantivy, GatewayHub WebSocket, PushGatewayHub, ActionRegistry, Hermes/MiniApp
//! routes) and orchestrates genuine cross-crate interactions end-to-end.

mod common;

use common::*;
use ed25519_dalek::SigningKey;
use gpui_telegram_agent::hermes::state_db::{
    CreateCronJobRequest, CreateKanbanCardRequest, HermesStateStore,
};
use gpui_telegram_agent::talk::VadBargeInDetector;
use gpui_telegram_client::cache::local_store::LocalStore;
use gpui_telegram_core::action_bus::{
    ActionRegistry, CliExporter, DeleteMessageAction, GetChatHistoryAction, HermesToolExporter,
    ListDialogsAction, McpExporter, OpenApiExporter, SendMessageAction,
};
use gpui_telegram_core::types::{ChatId, UserId};
use gpui::ObjectFit;
use gpui_telegram_desktop::app::session::send_as_peers::{
    SendAsKey, SendAsPeer, SendAsPeersSession,
};
use gpui_telegram_desktop::platform::mac::notifications_manager_mac_un::{
    NotificationsManagerMacUnMac, UNNotificationAction, UNNotificationParsed,
};
use gpui_telegram_desktop::views::app_shell::window_session_controller_link_info::{
    LinkInfoType, WindowSessionControllerLinkInfo,
};
use gpui_telegram_desktop::views::call_overlay::CallOverlay;
use gpui_telegram_media::chunked_upload::ChunkedUploadSession;
use gpui_telegram_media::media::capture::{
    MediaDeviceSource, MockAudioSource, MockVideoSource,
};
use gpui_telegram_proto::cards::{InvoiceCard, PriceLine, TodoCard, TodoItem};
use gpui_telegram_proto::contracts::DraftDto;
use gpui_telegram_proto::miniapp_contracts::MiniAppActionRequest;
use gpui_telegram_proto::sync::UpdateEvent;
use gpui_telegram_proto::ws::{WsClientFrame, WsServerFrame};
use gpui_telegram_server::push::dispatchers::MockPushDispatcher;
use gpui_telegram_server::push::{PushGatewayHub, PushNotification};
use gpui_telegram_server::search::{SearchDocument, SearchQueryParams, TantivyIndexer};
use gpui_telegram_webrtc::calls::peer_transport::{
    PeerTransportConfig, TransportMode, VideoFrameData, WebRtcPeerTransport,
};
use gpui_telegram_webrtc::media::ai_media_tap::{AiMediaTap, DefaultAiMediaTap};
use gpui_telegram_webrtc::media::opus_codec::{OpusDepacketizer, OpusPacketizer, RtpPacket};
use reqwest::Method;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::time::Duration;
use x25519_dalek::{PublicKey, StaticSecret};

/// Workflow 1: Multi-agent collaborative chat with floor arbitration, loop budget,
/// capability token verification, and real WebSocket delivery to Bob.
#[tokio::test]
async fn test_workflow_01_multi_agent_collaborative_chat_with_floor_and_loop_budget() {
    let (mut server, driver, (_alice_id, alice_token), (_bob_id, bob_token), chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let mut bob_ws = connect_ws(&server, &bob_token).await.unwrap();
    send_ws_frame(&mut bob_ws, &WsClientFrame::SubscribeChat { chat_id })
        .await
        .unwrap();

    let sk = SigningKey::from_bytes(&[1u8; 32]);
    let vk = sk.verifying_key();
    let now = chrono::Utc::now().timestamp();
    let planner_tok = ScopedCapabilityToken::issue(
        &sk,
        "planner",
        chat_id,
        vec!["chat:write".into()],
        600,
    );
    let coder_tok = ScopedCapabilityToken::issue(
        &sk,
        "coder",
        chat_id,
        vec!["chat:write".into()],
        600,
    );
    assert!(planner_tok.verify_for_scope(&vk, now + 10, "planner", chat_id, "chat:write"));
    assert!(coder_tok.verify_for_scope(&vk, now + 10, "coder", chat_id, "chat:write"));

    let mut floor = FloorControlQueue::new(5_000);
    let mut budget = LoopBudget::new(5);
    let im = HttpPlatformImDriver::new(server.base_url(), &alice_token);

    assert!(floor.request_floor_at("planner", FloorPriority::TaskAssignee, 100));
    budget.record_hop("planner").unwrap();
    let m1 = im
        .execute(
            "im_send_message",
            json!({ "chat_id": chat_id, "text": "[planner] Architecture v1 ready" }),
        )
        .await
        .unwrap();
    let mid1 = m1["message_id"].as_i64().unwrap();

    assert!(floor.request_floor_at("coder", FloorPriority::UserInterruption, 110));
    budget.record_hop("coder").unwrap();
    let m2 = im
        .execute(
            "im_send_message",
            json!({ "chat_id": chat_id, "text": "[coder] Patch applied & tested" }),
        )
        .await
        .unwrap();
    let mid2 = m2["message_id"].as_i64().unwrap();

    for expected_id in [mid1, mid2] {
        let frame = recv_ws_frame_matching(&mut bob_ws, Duration::from_secs(3), |f| match f {
            WsServerFrame::PtsPush(p) => {
                matches!(&p.event, UpdateEvent::NewMessage(m) if m.id.0 == expected_id)
            }
            WsServerFrame::Update(UpdateEvent::NewMessage(m)) => m.id.0 == expected_id,
            _ => false,
        })
        .await
        .expect("Bob WS receives collaborative agent update");
        assert!(matches!(
            frame,
            WsServerFrame::PtsPush(_) | WsServerFrame::Update(UpdateEvent::NewMessage(_))
        ));
    }

    let (_, hist) = driver
        .json_req(
            Method::GET,
            &format!("/api/v1/chats/{chat_id}/messages"),
            Some(&bob_token),
            None,
        )
        .await
        .unwrap();
    assert!(hist.to_string().contains("[coder] Patch applied & tested"));

    let _ = bob_ws.close(None).await;
    server.stop().await;
}

/// Workflow 2: Large chunked upload interrupted mid-flight, server crashes and restarts
/// from disk CAS staging, client queries missing chunks, resumes, completes, and streams
/// back byte-identical SHA-256 payload.
#[tokio::test]
async fn test_workflow_02_large_file_chunked_transfer_interruption_and_resume() {
    let (mut server, driver, (_alice_id, alice_token), (_bob_id, _bob_token), _chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let payload: Vec<u8> = (0..180_000u32).map(|i| ((i * 13) % 251) as u8).collect();
    let chunk_size = 32_768usize;
    let mut local_session =
        ChunkedUploadSession::new("wf02_sha", payload.len() as u64, chunk_size as u32).unwrap();
    for (idx, c) in payload.chunks(chunk_size).enumerate() {
        local_session.receive_part(idx as u32, c).unwrap();
    }
    let expected_sha = local_session.compute_sha256().unwrap();

    let (st_i, init) = driver
        .json_req(
            Method::POST,
            "/api/v1/media/upload/init",
            Some(&alice_token),
            Some(json!({
                "filename": "archive_180k.bin",
                "mime_type": "application/octet-stream",
                "size_bytes": payload.len(),
                "chunk_size": chunk_size
            })),
        )
        .await
        .unwrap();
    assert_eq!(st_i, 201);
    let upload_id = init["upload_id"].as_str().unwrap().to_string();

    for idx in 0..2 {
        let resp = driver
            .http
            .put(format!(
                "{}/api/v1/media/upload/{upload_id}/part/{idx}",
                server.base_url()
            ))
            .bearer_auth(&alice_token)
            .body(payload[idx * chunk_size..(idx + 1) * chunk_size].to_vec())
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status().as_u16(), 200);
    }

    server.restart().await.expect("restart server mid-upload");
    let driver2 = E2eSystemDriver::new(server.base_url());

    let (st_status, status_body) = driver2
        .json_req(
            Method::GET,
            &format!("/api/v1/media/upload/{upload_id}/status"),
            Some(&alice_token),
            None,
        )
        .await
        .unwrap();
    assert_eq!(st_status, 200);
    let missing = status_body["missing_parts"].as_array().unwrap().clone();
    assert_eq!(
        missing.len(),
        4,
        "parts 2,3,4,5 should still be missing after restart"
    );

    for idx_val in missing {
        let idx = idx_val.as_u64().unwrap() as usize;
        let start = idx * chunk_size;
        let end = (start + chunk_size).min(payload.len());
        let resp = driver2
            .http
            .put(format!(
                "{}/api/v1/media/upload/{upload_id}/part/{idx}",
                server.base_url()
            ))
            .bearer_auth(&alice_token)
            .body(payload[start..end].to_vec())
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status().as_u16(), 200);
    }

    let (st_c, comp) = driver2
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
    let file_id = comp["file_id"].as_str().unwrap();

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
    assert_eq!(streamed.as_ref(), payload.as_slice());

    server.stop().await;
}

/// Workflow 3: Live `EphemeralTestServer` call signaling (`/api/v1/webrtc/turn_credentials`,
/// `/api/v1/calls/invite`, `/api/v1/calls/signal`, `/api/v1/calls/{call_id}/rate`, WS
/// `CallUpdated`) + WebRTC SDP offer/answer negotiation + loopback RTP Opus audio & video
/// delivery into `CallOverlay`/`VideoViewport` + `DefaultAiMediaTap` VAD barge-in preemption.
#[tokio::test]
async fn test_workflow_03_realtime_webrtc_call_with_opus_rtp_and_vad_barge_in() {
    let (mut server, driver, (alice_id, alice_token), (bob_id, bob_token), chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot and seed two users");

    let mut alice_ws = connect_ws(&server, &alice_token).await.unwrap();
    let mut bob_ws = connect_ws(&server, &bob_token).await.unwrap();

    let (st_turn, turn_val) = driver
        .json_req(
            Method::GET,
            "/api/v1/webrtc/turn_credentials",
            Some(&alice_token),
            None,
        )
        .await
        .unwrap();
    assert_eq!(st_turn, 200);
    assert!(!turn_val["urls"].as_array().unwrap().is_empty());
    assert!(!turn_val["username"].as_str().unwrap().is_empty());
    assert!(!turn_val["credential"].as_str().unwrap().is_empty());

    let (st_inv, invite_val) = driver
        .json_req(
            Method::POST,
            "/api/v1/calls/invite",
            Some(&alice_token),
            Some(json!({
                "callee_id": bob_id,
                "chat_id": chat_id,
                "is_video": true,
            })),
        )
        .await
        .unwrap();
    assert_eq!(st_inv, 201);
    assert_eq!(invite_val["caller_id"].as_u64().unwrap(), alice_id);
    assert_eq!(invite_val["callee_id"].as_u64().unwrap(), bob_id);
    let call_id = invite_val["call_id"].as_str().unwrap().to_string();

    let ringing_frame = recv_ws_frame_matching(&mut bob_ws, Duration::from_secs(3), |f| {
        matches!(
            f,
            WsServerFrame::Update(UpdateEvent::CallUpdated {
                call_id: cid,
                state,
                ..
            }) if cid == &call_id && state == "ringing"
        )
    })
    .await
    .expect("Bob WS receives CallUpdated ringing");
    assert!(matches!(
        ringing_frame,
        WsServerFrame::Update(UpdateEvent::CallUpdated { .. })
    ));

    let live_cfg = PeerTransportConfig {
        mode: TransportMode::Live,
        ..Default::default()
    };
    let mut caller_live = WebRtcPeerTransport::new(live_cfg.clone()).await.unwrap();
    let mut callee_live = WebRtcPeerTransport::new(live_cfg).await.unwrap();
    let offer = caller_live.create_offer().await.unwrap();
    let (st_off, _) = driver
        .json_req(
            Method::POST,
            "/api/v1/calls/signal",
            Some(&alice_token),
            Some(json!({
                "call_id": call_id,
                "signal_type": "offer",
                "sdp": offer,
            })),
        )
        .await
        .unwrap();
    assert_eq!(st_off, 200);

    let offer_frame = recv_ws_frame_matching(&mut bob_ws, Duration::from_secs(3), |f| {
        matches!(
            f,
            WsServerFrame::Update(UpdateEvent::CallUpdated {
                call_id: cid,
                signal: Some(sig),
                ..
            }) if cid == &call_id && sig.signal_type == "offer"
        )
    })
    .await
    .expect("Bob WS receives CallUpdated offer signal");
    let relayed_offer = match offer_frame {
        WsServerFrame::Update(UpdateEvent::CallUpdated {
            signal: Some(sig), ..
        }) => sig.sdp.expect("relayed offer SDP"),
        _ => unreachable!(),
    };
    assert_eq!(relayed_offer, offer);
    callee_live
        .set_remote_description(&relayed_offer, true)
        .await
        .unwrap();

    let answer = callee_live.create_answer().await.unwrap();
    let (st_ans, ans_val) = driver
        .json_req(
            Method::POST,
            "/api/v1/calls/signal",
            Some(&bob_token),
            Some(json!({
                "call_id": call_id,
                "signal_type": "answer",
                "sdp": answer,
            })),
        )
        .await
        .unwrap();
    assert_eq!(st_ans, 200);
    assert_eq!(ans_val["call"]["state"].as_str(), Some("active"));

    let answer_frame = recv_ws_frame_matching(&mut alice_ws, Duration::from_secs(3), |f| {
        matches!(
            f,
            WsServerFrame::Update(UpdateEvent::CallUpdated {
                call_id: cid,
                state,
                signal: Some(sig),
            }) if cid == &call_id && state == "active" && sig.signal_type == "answer"
        )
    })
    .await
    .expect("Alice WS receives CallUpdated active answer signal");
    let relayed_answer = match answer_frame {
        WsServerFrame::Update(UpdateEvent::CallUpdated {
            signal: Some(sig), ..
        }) => sig.sdp.expect("relayed answer SDP"),
        _ => unreachable!(),
    };
    assert_eq!(relayed_answer, answer);
    caller_live
        .set_remote_description(&relayed_answer, false)
        .await
        .unwrap();

    let mut mic = MockAudioSource::new_sine(440.0, 48_000, 1);
    mic.start().unwrap();
    let audio_frame = mic.next_frame(20);

    let mut pktizer = OpusPacketizer::new(1, 0xABCD_0003);
    let pkts = pktizer.push_pcm(&audio_frame.samples);
    let parsed_rtp = RtpPacket::parse(&pkts[0].serialize()).unwrap();
    let mut depkt = OpusDepacketizer::new(1);
    let pcm = depkt.depacketize(&parsed_rtp).unwrap();

    let (mut tx_peer, rx_peer) =
        WebRtcPeerTransport::create_loopback_pair(PeerTransportConfig::default());
    let rx_peer = Arc::new(rx_peer);
    let tap = Arc::new(DefaultAiMediaTap::new(48_000, 1, 0.05).with_consecutive_frames(1));
    rx_peer.set_ai_media_tap(tap.clone());
    let rx_h = WebRtcPeerTransport::spawn_loopback_receiver(rx_peer.clone()).await;

    let mut floor = FloorControlQueue::new(5_000);
    floor.request_floor_at("assistant_tts", FloorPriority::DirectMention, 10);

    tx_peer.send_audio_pcm(&pcm).await.unwrap();
    tokio::time::timeout(Duration::from_secs(2), async {
        while !tap.trigger_vad_barge_in() {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("VAD barge-in detected on WebRTC audio");

    let mut vad = VadBargeInDetector::new(1000.0);
    assert!((0..3).any(|_| vad.process_pcm_frame(&pcm)));
    assert!(floor.request_floor_at("human_caller", FloorPriority::UserInterruption, 20));
    assert_eq!(floor.current_speaker(), Some("human_caller"));

    let mut cam = MockVideoSource::new(64, 36, 30);
    let vf = cam.next_frame();
    let mut overlay = CallOverlay::new("Bob".to_string());
    overlay.toggle_camera();
    overlay.video_viewport.aspect_ratio_mode = ObjectFit::Cover;
    assert!(overlay
        .video_viewport
        .ingest_raw_remote_rgba(64, 36, &vf.to_rgba8()));
    assert!(overlay.video_viewport.has_active_video());

    let vframe = VideoFrameData {
        width: 64,
        height: 36,
        timestamp_ms: 42,
        data: vf.to_rgba8(),
    };
    tx_peer.send_video_frame(&vframe).await.unwrap();

    let (st_hang, hang_val) = driver
        .json_req(
            Method::POST,
            "/api/v1/calls/signal",
            Some(&alice_token),
            Some(json!({
                "call_id": call_id,
                "signal_type": "hangup",
                "reason": "user_hangup",
            })),
        )
        .await
        .unwrap();
    assert_eq!(st_hang, 200);
    assert_eq!(hang_val["call"]["state"].as_str(), Some("ended"));

    let hangup_frame = recv_ws_frame_matching(&mut bob_ws, Duration::from_secs(3), |f| {
        matches!(
            f,
            WsServerFrame::Update(UpdateEvent::CallUpdated {
                call_id: cid,
                state,
                ..
            }) if cid == &call_id && state == "ended"
        )
    })
    .await
    .expect("Bob WS receives CallUpdated ended");
    assert!(matches!(
        hangup_frame,
        WsServerFrame::Update(UpdateEvent::CallUpdated { .. })
    ));

    let (st_rate, rate_val) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/calls/{call_id}/rate"),
            Some(&alice_token),
            Some(json!({
                "rating": 5,
                "feedback": "crystal clear opus audio",
            })),
        )
        .await
        .unwrap();
    assert_eq!(st_rate, 200);
    assert_eq!(rate_val["rating"].as_u64(), Some(5));

    mic.stop().unwrap();
    if let Some(h) = rx_h {
        h.abort();
    }
    tx_peer.close().await.unwrap();
    caller_live.close().await.unwrap();
    callee_live.close().await.unwrap();
    let _ = alice_ws.close(None).await;
    let _ = bob_ws.close(None).await;
    server.stop().await;
}

/// Workflow 4: Bot + MiniApp registration, installation, launch, action execution on live
/// `EphemeralTestServer`, streaming sanitized `GenerativeCard`, `TodoCard`, and `InvoiceCard`
/// with capability-token approval.
#[tokio::test]
async fn test_workflow_04_miniapp_financial_audit_streaming_generative_ui_cards() {
    let (mut server, driver, (_alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let (st_b, _) = driver
        .json_req(
            Method::POST,
            "/api/v1/bots",
            Some(&alice_token),
            Some(json!({ "display_name": "Audit Bot", "username": "audit_wf04_bot" })),
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
                "bot_username": "audit_wf04_bot",
                "short_name": "fin_audit",
                "title": "Financial Audit"
            })),
        )
        .await
        .unwrap();
    assert!((200..300).contains(&st_m));

    driver
        .json_req(
            Method::POST,
            "/api/v1/miniapps/audit_wf04_bot/fin_audit/install",
            Some(&alice_token),
            Some(json!({ "chat_id": chat_id })),
        )
        .await
        .unwrap();
    driver
        .json_req(
            Method::POST,
            "/api/v1/miniapps/audit_wf04_bot/fin_audit/launch",
            Some(&alice_token),
            Some(json!({ "chat_id": chat_id })),
        )
        .await
        .unwrap();
    let (st_act, _) = driver
        .json_req(
            Method::POST,
            "/api/v1/miniapps/audit_wf04_bot/fin_audit/action",
            Some(&alice_token),
            Some(
                serde_json::to_value(MiniAppActionRequest::widget_event(
                    "run_audit",
                    json!({ "quarter": "Q4" }),
                    Some(chat_id),
                ))
                .unwrap(),
            ),
        )
        .await
        .unwrap();
    assert!((200..300).contains(&st_act));

    let mut inv = InvoiceCard::new("inv-q4", "Q4 Cloud Audit", "Quarterly settlement", "USD", 0)
        .with_prices(vec![
            PriceLine::new("Compute", 12_000),
            PriceLine::new("Storage", 3_000),
        ]);
    assert_eq!(inv.total_from_prices(), 15_000);
    inv.mark_paid();
    assert!(inv.is_paid);

    let mut todo = TodoCard::new(
        "audit-list",
        "Audit Checks",
        vec![TodoItem::new("c1", "Reconcile invoices")],
    );
    todo.toggle_item("c1");

    let sk = SigningKey::from_bytes(&[4u8; 32]);
    let vk = sk.verifying_key();
    let now = chrono::Utc::now().timestamp();
    let cap = ScopedCapabilityToken::issue(
        &sk,
        "audit_wf04_bot",
        chat_id,
        vec!["finance:approve".into()],
        500,
    );
    let mut gcard = GenerativeCard {
        card_id: "gc-wf04".into(),
        title: "Audit Summary".into(),
        subtitle: None,
        layout: GenerativeCardLayout::VerticalStack,
        components: vec![
            GenerativeComponent::Button {
                id: "cap_btn".into(),
                label: "Approve".into(),
                action_payload: serde_json::to_string(&cap).unwrap(),
            },
            GenerativeComponent::Text {
                text: String::new(),
                bold: false,
                color: None,
            },
        ],
        status: Some("streaming".into()),
    };
    gcard
        .append_streaming_delta(1, "<script>xss()</script>Settled $150.00")
        .unwrap();
    if let GenerativeComponent::Text { text, .. } = &gcard.components[1] {
        assert!(!text.contains("<script"));
    }

    if let GenerativeComponent::Button { action_payload, .. } = &gcard.components[0] {
        let extracted: ScopedCapabilityToken = serde_json::from_str(action_payload).unwrap();
        assert!(extracted.verify_for_scope(&vk, now + 10, "audit_wf04_bot", chat_id, "finance:approve"));
    }

    server.stop().await;
}

/// Workflow 5: Multi-device X3DH prekey handshake + offline SQLite outbox + live
/// WebSocket delivery to Bob's active socket + decryption verification.
#[tokio::test]
async fn test_workflow_05_multi_device_e2ee_secret_chat_with_offline_outbox() {
    let (mut server, driver, (_alice_id, alice_token), (bob_id, bob_token), chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let mut bob_ws = connect_ws(&server, &bob_token).await.unwrap();
    send_ws_frame(&mut bob_ws, &WsClientFrame::SubscribeChat { chat_id })
        .await
        .unwrap();

    let e2ee = E2eeSimulator::new();
    let alice_k = StaticSecret::from([51u8; 32]);
    let bob_k = StaticSecret::from([52u8; 32]);
    let bob_spk = StaticSecret::from([53u8; 32]);
    let bob_opk = StaticSecret::from([54u8; 32]);
    let eph_k = StaticSecret::from([55u8; 32]);
    e2ee.publish_prekeys(PrekeyBundle {
        user_id: bob_id,
        device_id: "bob-laptop".into(),
        identity_key: PublicKey::from(&bob_k),
        signed_prekey: PublicKey::from(&bob_spk),
        one_time_prekeys: vec![PublicKey::from(&bob_opk)],
    });

    let (bundle, consumed_opk) = e2ee.consume_prekey_bundle(bob_id, "bob-laptop").unwrap();
    assert!(consumed_opk.is_some());
    let sk_a = E2eeSimulator::derive_x3dh_initiator(
        &alice_k,
        &eph_k,
        &bundle.identity_key,
        &bundle.signed_prekey,
        consumed_opk.as_ref(),
    );
    let ct_bytes =
        E2eeSimulator::encrypt_message(&sk_a, 0, b"Secret multi-device payload").unwrap();
    let ct_hex: String = ct_bytes.iter().map(|b| format!("{b:02x}")).collect();

    let tmp = new_test_dir("wf05");
    let outbox = OutboxSimulator::open(tmp.path().join("wf05_outbox.db")).unwrap();
    outbox.enqueue(chat_id, &ct_hex).unwrap();
    let sent = outbox
        .notify_reconnected_to_server(&driver, &alice_token)
        .await
        .unwrap();
    let msg_id = sent[0].1;

    let ws_frame = recv_ws_frame_matching(&mut bob_ws, Duration::from_secs(3), |f| match f {
        WsServerFrame::PtsPush(p) => {
            matches!(&p.event, UpdateEvent::NewMessage(m) if m.id.0 == msg_id)
        }
        WsServerFrame::Update(UpdateEvent::NewMessage(m)) => m.id.0 == msg_id,
        _ => false,
    })
    .await
    .expect("Bob receives E2EE frame on WS");
    assert!(matches!(
        ws_frame,
        WsServerFrame::PtsPush(_) | WsServerFrame::Update(UpdateEvent::NewMessage(_))
    ));

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
    let wire_bytes: Vec<u8> = (0..wire_hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&wire_hex[i..i + 2], 16).unwrap())
        .collect();
    let sk_b = E2eeSimulator::derive_x3dh_responder(
        &bob_k,
        &bob_spk,
        Some(&bob_opk),
        &PublicKey::from(&alice_k),
        &PublicKey::from(&eph_k),
    );
    assert_eq!(
        e2ee.decrypt_message("alice-laptop", &sk_b, 0, &wire_bytes)
            .unwrap(),
        b"Secret multi-device payload"
    );

    let _ = bob_ws.close(None).await;
    server.stop().await;
}

/// Workflow 6: Capability-gated hybrid search pairing live server Tantivy `/api/v1/search`
/// + disk-backed `TantivyIndexer` + `HybridMemoryEngine`.
#[tokio::test]
async fn test_workflow_06_permission_gated_hybrid_memory_search() {
    let (mut server, driver, (_alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let sk = SigningKey::from_bytes(&[6u8; 32]);
    let vk = sk.verifying_key();
    let now = chrono::Utc::now().timestamp();
    let cap = ScopedCapabilityToken::issue(
        &sk,
        "search_bot",
        chat_id,
        vec!["memory:search".into()],
        600,
    );
    assert!(cap.verify_for_scope(&vk, now + 10, "search_bot", chat_id, "memory:search"));
    assert!(!cap.verify_for_scope(&vk, now + 10, "search_bot", chat_id, "memory:purge"));

    driver
        .json_req(
            Method::POST,
            &format!("/api/v1/chats/{chat_id}/messages"),
            Some(&alice_token),
            Some(json!({ "text": "Zephyr consensus protocol deployed", "client_nonce": 601 })),
        )
        .await
        .unwrap();
    server.state().unwrap().search_indexer.commit().await.unwrap();

    let (st_s, sbody) = driver
        .json_req(
            Method::GET,
            "/api/v1/search?q=Zephyr",
            Some(&alice_token),
            None,
        )
        .await
        .unwrap();
    assert_eq!(st_s, 200);
    assert!(!sbody["hits"].as_array().unwrap().is_empty());

    let tmp = new_test_dir("wf06");
    let local_idx = TantivyIndexer::open_or_create(tmp.path().join("tantivy")).unwrap();
    local_idx
        .index_message(SearchDocument::new(
            "1",
            &chat_id.to_string(),
            "1",
            100,
            "Zephyr consensus protocol deployed",
            Some("text".into()),
        ))
        .await
        .unwrap();
    local_idx.commit().await.unwrap();
    let local_hits = local_idx
        .search_hits(&SearchQueryParams::new("Zephyr").with_chat_id(&chat_id.to_string()))
        .unwrap();
    assert_eq!(local_hits.len(), 1);

    let hybrid = HybridMemoryEngine::new();
    hybrid.index_document(MemoryDocument {
        message_id: "m1".into(),
        chat_id,
        content: "Zephyr consensus protocol deployed".into(),
        embedding: vec![0.9, 0.1],
        linked_entities: vec!["release-notes".into()],
    });
    let h_hits = hybrid.search("Zephyr consensus", Some(&[1.0, 0.0]), &[chat_id], None);
    assert_eq!(h_hits.len(), 1);

    server.stop().await;
}

/// Workflow 7: Dynamic chat management with `SendAsPeersSession`, `LocalStore` + server
/// drafts, folders CRUD, message pinning, and TTL configuration.
#[tokio::test]
async fn test_workflow_07_dynamic_chat_management_with_send_as_peers_folders_and_pin() {
    let (mut server, driver, (alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let mut send_as = SendAsPeersSession::new(alice_id);
    let key = SendAsKey::message(chat_id as u64);
    send_as.set_list(
        key,
        vec![SendAsPeer::new(7001, "Announcements".into(), false)],
    );
    send_as.set_chosen(chat_id as u64, 7001);
    assert_eq!(send_as.resolve_chosen(&key), 7001);

    driver
        .json_req(
            Method::POST,
            "/api/v1/folders",
            Some(&alice_token),
            Some(json!({ "title": "Priority Ops", "included_chat_ids": [chat_id] })),
        )
        .await
        .unwrap();

    let tmp = new_test_dir("wf07");
    let store = LocalStore::open_with_dir(tmp.path()).unwrap();
    store.set_draft(
        ChatId(chat_id),
        Some(DraftDto {
            chat_id,
            text: "WIP runbook".into(),
            entities: vec![],
            reply_to_msg_id: None,
            topic_id: None,
            date_ms: 1000,
        }),
    );
    assert_eq!(store.get_draft(ChatId(chat_id)).unwrap().text, "WIP runbook");
    driver
        .json_req(
            Method::PUT,
            &format!("/api/v1/chats/{chat_id}/draft"),
            Some(&alice_token),
            Some(json!({ "text": "WIP runbook" })),
        )
        .await
        .unwrap();

    let (_, msg) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/chats/{chat_id}/messages"),
            Some(&alice_token),
            Some(json!({ "text": "Published runbook v1", "client_nonce": 707 })),
        )
        .await
        .unwrap();
    let mid = msg["message_id"]
        .as_i64()
        .or_else(|| msg["message"]["message_id"].as_i64())
        .unwrap();
    driver
        .json_req(
            Method::POST,
            &format!("/api/v1/chats/{chat_id}/pins"),
            Some(&alice_token),
            Some(json!({ "message_id": mid })),
        )
        .await
        .unwrap();
    let (st_ttl, ttl_body) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/chats/{chat_id}/ttl"),
            Some(&alice_token),
            Some(json!({ "ttl_seconds": 86_400 })),
        )
        .await
        .unwrap();
    assert_eq!(st_ttl, 200);
    assert_eq!(ttl_body["ttl_seconds"], 86_400);

    server.stop().await;
}

/// Workflow 8: Push token registration -> live message event -> `PushGatewayHub` dispatch
/// -> `NotificationsManagerMacUnMac` banner click -> deep-link navigation to chat/message.
#[tokio::test]
async fn test_workflow_08_push_notification_dispatch_with_deep_link_navigation() {
    let (mut server, driver, (_alice_id, alice_token), (bob_id, bob_token), chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    driver
        .json_req(
            Method::POST,
            "/api/v1/devices/register_push",
            Some(&bob_token),
            Some(json!({ "platform": "mock", "token": "bob_macos_push_wf08" })),
        )
        .await
        .unwrap();

    let (_, msg) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/chats/{chat_id}/messages"),
            Some(&alice_token),
            Some(json!({ "text": "Incident #88 resolved", "client_nonce": 808 })),
        )
        .await
        .unwrap();
    let mid = msg["message_id"]
        .as_i64()
        .or_else(|| msg["message"]["message_id"].as_i64())
        .unwrap();

    let state = server.state().unwrap().clone();
    let mock_disp = Arc::new(MockPushDispatcher::new());
    let hub = PushGatewayHub::new(Arc::clone(&state.db));
    hub.register_dispatcher(mock_disp.clone());

    let receipts = hub
        .send_to_user(
            UserId(bob_id),
            &PushNotification {
                title: "Alice".into(),
                body: "Incident #88 resolved".into(),
                chat_id: Some(chat_id),
                message_id: Some(mid),
                badge: Some(1),
                sound: Some("default".into()),
                custom_data: Default::default(),
            },
        )
        .await;
    assert!(receipts[0].as_ref().unwrap().delivered);

    let (_, payload) = &mock_disp.get_dispatched()[0];
    let mut mac_mgr = NotificationsManagerMacUnMac::new(bob_id).with_mock_mode(true);
    let id1 = mac_mgr
        .show_system_notification(
            &payload.title,
            "",
            &payload.body,
            true,
            UNNotificationParsed::new(bob_id, chat_id as u64, mid),
        )
        .unwrap();
    assert!(mac_mgr
        .handle_notification_action(
            &id1,
            UNNotificationAction::InlineReply("Acknowledged".into()),
        )
        .unwrap());
    assert_eq!(mac_mgr.replied_messages.len(), 1);

    let id2 = mac_mgr
        .show_system_notification(
            &payload.title,
            "",
            &payload.body,
            true,
            UNNotificationParsed::new(bob_id, chat_id as u64, mid),
        )
        .unwrap();
    let link = mac_mgr.handle_notification_click(&id2).unwrap();

    let mut tree = SemanticWorkspaceTreeSimulator::new();
    tree.register_chat(chat_id as u64);
    match WindowSessionControllerLinkInfo::parse_deep_link(&link) {
        LinkInfoType::Peer(info) => {
            assert_eq!(info.channel_id, Some(chat_id as u64));
            assert!(
                tree.perform_action(SemanticAction::SelectChat {
                    chat_id: info.channel_id.unwrap()
                })
                .success
            );
        }
        other => panic!("unexpected deep link: {other:?}"),
    }

    server.stop().await;
}

/// Workflow 9: Quad-export (`OpenApi`, `Hermes`, `MCP`, `CLI`) + live server Action Bus execution
/// and approval gate flow (`approve_once` token single-use execution for `delete_message`).
#[tokio::test]
async fn test_workflow_09_action_bus_quad_export_execution_pipeline() {
    let reg = ActionRegistry::new();
    reg.register_typed(SendMessageAction).unwrap();
    reg.register_typed(GetChatHistoryAction).unwrap();
    reg.register_typed(ListDialogsAction).unwrap();
    reg.register_typed(DeleteMessageAction).unwrap();
    let reg = Arc::new(reg);

    assert_eq!(
        OpenApiExporter::export_spec(&reg, "WF09", "1.0", "http://127.0.0.1")["openapi"],
        "3.1.0"
    );
    assert_eq!(HermesToolExporter::export_openai_tools(&reg).len(), 4);
    assert_eq!(McpExporter::export_tools_list(&reg).tools.len(), 4);
    assert_eq!(
        CliExporter::export_cli_schema(&reg, "gpui-tg", "1.0")
            .subcommands
            .len(),
        4
    );

    let (mut server, driver, (_alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let (st_s, sent) = driver
        .json_req(
            Method::POST,
            "/api/v1/actions/send_message",
            Some(&alice_token),
            Some(json!({ "chat_id": chat_id, "text": "To delete via gate" })),
        )
        .await
        .unwrap();
    assert_eq!(st_s, 200);
    let mid = sent["result"]["message_id"]
        .as_i64()
        .or_else(|| sent["message_id"].as_i64())
        .unwrap();

    let (st_g, gate) = driver
        .json_req(
            Method::POST,
            "/api/v1/actions/delete_message",
            Some(&alice_token),
            Some(json!({ "chat_id": chat_id, "message_ids": [mid], "for_everyone": true })),
        )
        .await
        .unwrap();
    assert_ne!(st_g, 200);
    let gate_id = gate["gate_id"].as_str().unwrap();

    let (_, app) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/actions/approvals/{gate_id}/approve"),
            Some(&alice_token),
            Some(json!({ "decision": "approve_once" })),
        )
        .await
        .unwrap();
    let tok = app["approval_token"].as_str().unwrap();

    let exec_resp = driver
        .http
        .post(format!("{}/api/v1/actions/delete_message", server.base_url()))
        .bearer_auth(&alice_token)
        .header("X-Approval-Token", tok)
        .json(&json!({ "chat_id": chat_id, "message_ids": [mid], "for_everyone": true }))
        .send()
        .await
        .unwrap();
    assert_eq!(exec_resp.status().as_u16(), 200);

    server.stop().await;
}

/// Workflow 10: Agentic Kanban + Cron lifecycle persisted across SQLite WAL `HermesStateStore`
/// AND live server `/api/v1/hermes/kanban/*` restart durability.
#[tokio::test]
async fn test_workflow_10_agentic_kanban_workflow_with_wal_durability() {
    let tmp = new_test_dir("wf10");
    let local_db = tmp.path().join("hermes_wf10.db");
    let card_id = {
        let store = HermesStateStore::open(&local_db).unwrap();
        let board = store.create_kanban_board("Release", "v2.0", None).unwrap();
        let card = store
            .create_kanban_card(CreateKanbanCardRequest {
                id: None,
                board_id: board.id,
                column_name: Some("Backlog".into()),
                title: "Verify E2E".into(),
                description: Some("All 265 tests".into()),
                priority: Some("critical".into()),
                assignee: Some("worker_2".into()),
                labels: Some(vec!["qa".into()]),
            })
            .unwrap();
        store
            .create_cron_job(CreateCronJobRequest {
                id: None,
                name: "nightly_qa".into(),
                schedule: "0 2 * * *".into(),
                timezone: None,
                target_chat_id: None,
                prompt: "Check ledger".into(),
                model: None,
                skills: None,
                enabled: Some(true),
                deliver_to: None,
                origin_chat_id: None,
                next_run_at: None,
            })
            .unwrap();
        store.move_kanban_card(&card.id, "Done", None).unwrap();
        card.id
    };

    let reopened = HermesStateStore::open(&local_db).unwrap();
    let updated = reopened.move_kanban_card(&card_id, "Done", None).unwrap();
    assert_eq!(updated.column_name, "Done");

    let (mut server, driver, (_alice_id, alice_token), _bob, chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let (st_b, b_body) = driver
        .json_req(
            Method::POST,
            "/api/v1/hermes/kanban/boards",
            Some(&alice_token),
            Some(json!({ "name": "Server Release", "title": "Server Release", "description": "WAL board" })),
        )
        .await
        .unwrap();
    assert!((200..300).contains(&st_b));
    let srv_board_id = b_body["board_id"]
        .as_str()
        .or_else(|| b_body["board"]["id"].as_str())
        .or_else(|| b_body["id"].as_str())
        .unwrap()
        .to_string();

    driver
        .json_req(
            Method::POST,
            "/api/v1/hermes/kanban/cards",
            Some(&alice_token),
            Some(json!({
                "board_id": srv_board_id,
                "title": "Restart durability card",
                "column_name": "In Progress"
            })),
        )
        .await
        .unwrap();

    server.restart().await.expect("restart server");
    let driver2 = E2eSystemDriver::new(server.base_url());
    let (st_get, board_after) = driver2
        .json_req(
            Method::GET,
            &format!("/api/v1/hermes/kanban/boards/{srv_board_id}"),
            Some(&alice_token),
            None,
        )
        .await
        .unwrap();
    assert_eq!(st_get, 200);
    assert!(board_after.to_string().contains("Restart durability card"));

    let im = HttpPlatformImDriver::new(server.base_url(), &alice_token);
    im.execute(
        "im_send_message",
        json!({ "chat_id": chat_id, "text": "Workflow 10 Kanban WAL verified" }),
    )
    .await
    .unwrap();

    server.stop().await;
}

/// Workflow 11: Real TCP SOCKS5 proxy server tunneling HTTP message + media chunk upload/stream
/// to `EphemeralTestServer` with live WebSocket verification.
#[tokio::test]
async fn test_workflow_11_secure_proxy_tunneling_for_media_and_websocket() {
    let (mut server, _driver, (_alice_id, alice_token), (_bob_id, bob_token), chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");
    let proxy = LocalTestProxyServer::start_socks5(Some(("alice".into(), "pw_wf11".into())))
        .await
        .unwrap();

    let mut bob_ws = connect_ws(&server, &bob_token).await.unwrap();
    send_ws_frame(&mut bob_ws, &WsClientFrame::SubscribeChat { chat_id })
        .await
        .unwrap();

    let cfg = ProxyConfig {
        proxy_type: ProxyType::Socks5,
        host: "127.0.0.1".into(),
        port: proxy.port(),
        username: Some("alice".into()),
        password: Some("pw_wf11".into()),
    };
    let target: std::net::SocketAddr = server
        .base_url()
        .trim_start_matches("http://")
        .parse()
        .unwrap();

    let png = create_test_png(10, 20, 30, 255);
    let init_payload = json!({
        "filename": "proxied.png",
        "mime_type": "image/png",
        "size_bytes": png.len(),
        "chunk_size": png.len()
    })
    .to_string();
    let (st_i, init_body) = cfg
        .tunnel_http_request(
            target,
            "POST",
            "/api/v1/media/upload/init",
            Some(&alice_token),
            Some(init_payload.as_bytes()),
            Some("application/json"),
        )
        .await
        .unwrap();
    assert_eq!(st_i, 201);
    let upload_id = serde_json::from_str::<Value>(&init_body).unwrap()["upload_id"]
        .as_str()
        .unwrap()
        .to_string();

    let (st_p, _) = cfg
        .tunnel_http_request(
            target,
            "PUT",
            &format!("/api/v1/media/upload/{upload_id}/part/0"),
            Some(&alice_token),
            Some(&png),
            Some("application/octet-stream"),
        )
        .await
        .unwrap();
    assert_eq!(st_p, 200);

    let (st_c, _) = cfg
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

    let msg_body = br#"{"text":"Delivered via authenticated SOCKS5","client_nonce":1111}"#;
    let (st_m, mresp) = cfg
        .tunnel_http_request(
            target,
            "POST",
            &format!("/api/v1/chats/{chat_id}/messages"),
            Some(&alice_token),
            Some(msg_body),
            Some("application/json"),
        )
        .await
        .unwrap();
    assert!((200..300).contains(&st_m));
    let mval = serde_json::from_str::<Value>(&mresp).unwrap();
    let sent_mid = mval["message_id"]
        .as_i64()
        .or_else(|| mval["message"]["message_id"].as_i64())
        .unwrap();

    let frame = recv_ws_frame_matching(&mut bob_ws, Duration::from_secs(3), |f| match f {
        WsServerFrame::PtsPush(p) => {
            matches!(&p.event, UpdateEvent::NewMessage(m) if m.id.0 == sent_mid)
        }
        WsServerFrame::Update(UpdateEvent::NewMessage(m)) => m.id.0 == sent_mid,
        _ => false,
    })
    .await
    .expect("Bob WS receives proxied message");
    assert!(matches!(
        frame,
        WsServerFrame::PtsPush(_) | WsServerFrame::Update(UpdateEvent::NewMessage(_))
    ));

    let _ = bob_ws.close(None).await;
    proxy.stop().await;
    server.stop().await;
}

/// Workflow 12: Headless GPUI `CallOverlay`/`VideoViewport` + `SemanticWorkspaceTreeSimulator`
/// coupled to live `EphemeralTestServer` chat dispatch & WebSocket verification.
#[tokio::test]
async fn test_workflow_12_gpui_headless_semantic_tree_state_mutation_pipeline() {
    let (mut server, driver, (_alice_id, alice_token), (_bob_id, bob_token), chat_id) =
        boot_and_seed_two_users()
            .await
            .expect("boot_and_seed_two_users");

    let mut tree = SemanticWorkspaceTreeSimulator::new();
    tree.register_chat(chat_id as u64);
    assert!(
        tree.perform_action(SemanticAction::SelectChat {
            chat_id: chat_id as u64
        })
        .success
    );
    assert!(
        tree.perform_action(SemanticAction::ComposeText {
            text: "UI tree -> Server".into()
        })
        .success
    );
    let composed = tree.inspect_state().composer_text;
    assert!(tree.perform_action(SemanticAction::SendMessage).success);
    assert!(
        tree.perform_action(SemanticAction::ToggleCall {
            peer_id: chat_id as u64,
            is_video: true
        })
        .success
    );

    let mut overlay = CallOverlay::new("Bob".to_string());
    overlay.toggle_camera();
    overlay.toggle_screen_share();
    assert!(overlay
        .video_viewport
        .ingest_raw_remote_rgba(8, 8, &vec![180u8; 8 * 8 * 4]));
    assert!(overlay.video_viewport.has_active_video());

    let (st, _) = driver
        .json_req(
            Method::POST,
            &format!("/api/v1/chats/{chat_id}/messages"),
            Some(&alice_token),
            Some(json!({
                "text": composed,
                "client_nonce": 1212
            })),
        )
        .await
        .unwrap();
    assert!((200..300).contains(&st));

    let (_, hist) = driver
        .json_req(
            Method::GET,
            &format!("/api/v1/chats/{chat_id}/messages"),
            Some(&bob_token),
            None,
        )
        .await
        .unwrap();
    assert!(hist.to_string().contains("UI tree -> Server"));

    server.stop().await;
}
