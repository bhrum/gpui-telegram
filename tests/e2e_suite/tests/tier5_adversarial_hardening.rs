//! Tier 5 Adversarial Hardening — WebRTC, Opus, AI Media Tap, VideoViewport,
//! Hermes Auth/Tools, AgenticStore WAL, Multi-Agent Floor/Budget/Tokens, Hybrid Memory

use axum::body::{to_bytes, Body};
use axum::http::{header, Request, StatusCode};
use chrono::Utc;
use ed25519_dalek::SigningKey;
use gpui_telegram_agent::hermes::security::ApprovalMode;
use gpui_telegram_agent::hermes::state_db::CreateApprovalRequest;
use gpui_telegram_agent::hermes::tools_agentic::{KanbanBoard, LocalAgenticStore};
use gpui_telegram_agent::hermes::toolsets::{HermesToolExecutor, PlatformImCaller, ToolsetPreset};
use gpui_telegram_agent::{
    FloorControlQueue, FloorPriority, LoopBudget, ScopedCapabilityToken, DEFAULT_MAX_LOOP_HOPS,
};
use gpui_telegram_core::error::TelegramError;
use gpui_telegram_desktop::components::video_viewport::{encode_rgba_to_gpui_image, VideoViewport};
use gpui_telegram_desktop::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use gpui_telegram_desktop::views::call_overlay::CallOverlay;
use gpui_telegram_proto::semantic_tree::{SemanticAction, SemanticWorkspaceTree};
use gpui_telegram_server::db::schema::UserRecord;
use gpui_telegram_server::db::ServerDatabase;
use gpui_telegram_server::search::hybrid_memory::{HybridMemoryEngine, MemoryDocument, RRF_K};
use gpui_telegram_server::search::schema::{SearchDocument, SearchQueryParams};
use gpui_telegram_server::search::tantivy_indexer::TantivyIndexer;
use gpui_telegram_server::server::AppState;
use gpui_telegram_server::{build_router_with_state, ServerConfig};
use gpui_telegram_webrtc::calls::peer_transport::{
    PeerTransportConfig, TransportMode, WebRtcPeerTransport, WebRtcTransportError,
};
use gpui_telegram_webrtc::media::ai_media_tap::{AiMediaTap, DefaultAiMediaTap};
use gpui_telegram_webrtc::media::opus_codec::{
    CodecError, OpusDepacketizer, OpusFramingCodec, OpusPacketizer, RtpPacket, OPUS_PAYLOAD_TYPE,
    OPUS_SAMPLES_PER_FRAME_MONO, OPUS_SAMPLES_PER_FRAME_STEREO,
};
use serde_json::{json, Value};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use tempfile::TempDir;
use tokio::sync::mpsc;
use tower::ServiceExt;

#[tokio::test]
async fn test_tier5_webrtc_rtp_seq_rollover_drops_and_corrupted_headers() {
    let cfg = PeerTransportConfig {
        mode: TransportMode::MockLoopback,
        audio_sample_rate: 48_000,
        audio_channels: 1,
        ..Default::default()
    };
    let (alice, _bob) = WebRtcPeerTransport::create_loopback_pair(cfg);
    let received_frames = Arc::new(AtomicUsize::new(0));
    let cnt = received_frames.clone();
    alice.set_audio_frame_callback(Arc::new(move |samples, sr, ch| {
        assert_eq!(samples.len(), OPUS_SAMPLES_PER_FRAME_MONO);
        assert_eq!(sr, 48_000);
        assert_eq!(ch, 1);
        cnt.fetch_add(1, Ordering::SeqCst);
    }));

    let pcm = vec![512i16; OPUS_SAMPLES_PER_FRAME_MONO];
    let payload = OpusFramingCodec::encode_frame(&pcm, 1).unwrap();

    // Crossing u16::MAX -> 0 -> 1 and u32::MAX timestamp rollover
    let seqs = [65534u16, 65535u16, 0u16, 1u16];
    let mut ts = u32::MAX - 960;
    for &seq in &seqs {
        let pkt = RtpPacket::new(OPUS_PAYLOAD_TYPE, seq, ts, 0xABCD_1234, payload.clone());
        let wire = pkt.serialize();
        let parsed = RtpPacket::parse(&wire).expect("parse rollover RTP");
        assert_eq!(parsed.sequence_number, seq);
        assert_eq!(parsed.timestamp, ts);
        alice.deliver_incoming_rtp(&parsed).expect("deliver rollover RTP");
        ts = ts.wrapping_add(960);
    }
    assert_eq!(received_frames.load(Ordering::SeqCst), 4);

    // Packet drops & out-of-order delivery must not wedge depacketizer
    for &seq in &[100u16, 102, 105, 104, 108] {
        let pkt = RtpPacket::new(OPUS_PAYLOAD_TYPE, seq, (seq as u32) * 960, 1, payload.clone());
        alice.deliver_incoming_rtp(&pkt).expect("deliver ooo RTP");
    }
    assert_eq!(received_frames.load(Ordering::SeqCst), 9);

    // Corrupted RTP header rejection via RtpPacket::parse
    assert!(matches!(RtpPacket::parse(&[]), Err(CodecError::InvalidRtpPacket(_))));
    assert!(matches!(RtpPacket::parse(&[0x80; 11]), Err(CodecError::InvalidRtpPacket(_))));
    let mut bad_ver = vec![0u8; 12];
    bad_ver[0] = 0x40; // version = 1
    assert!(matches!(RtpPacket::parse(&bad_ver), Err(CodecError::InvalidRtpPacket(_))));
    let mut bad_csrc = vec![0x8F; 16]; // CC=15 needs 72 bytes
    bad_csrc[1] = OPUS_PAYLOAD_TYPE;
    assert!(matches!(RtpPacket::parse(&bad_csrc), Err(CodecError::InvalidRtpPacket(_))));
    let mut bad_ext = vec![0u8; 14];
    bad_ext[0] = 0x90; // V=2, X=1 truncated
    assert!(matches!(RtpPacket::parse(&bad_ext), Err(CodecError::InvalidRtpPacket(_))));
    let mut bad_ext_len = vec![0u8; 16];
    bad_ext_len[0] = 0x90;
    bad_ext_len[14] = 0x00;
    bad_ext_len[15] = 0x10;
    assert!(matches!(RtpPacket::parse(&bad_ext_len), Err(CodecError::InvalidRtpPacket(_))));
    let mut bad_pad_zero = vec![0u8; 13];
    bad_pad_zero[0] = 0xA0;
    bad_pad_zero[12] = 0;
    assert!(matches!(RtpPacket::parse(&bad_pad_zero), Err(CodecError::InvalidRtpPacket(_))));
    let mut bad_pad_over = vec![0u8; 13];
    bad_pad_over[0] = 0xA0;
    bad_pad_over[12] = 5;
    assert!(matches!(RtpPacket::parse(&bad_pad_over), Err(CodecError::InvalidRtpPacket(_))));

    // Wrong payload type (VP8=96) delivered to Opus audio depacketizer
    let wrong_pt = RtpPacket::new(96, 200, 1920, 1, payload);
    assert!(alice.deliver_incoming_rtp(&wrong_pt).is_err());

    // Sending after receiver dropped returns ChannelClosed cleanly
    let (tx_peer, rx_peer) = WebRtcPeerTransport::create_loopback_pair(PeerTransportConfig::default());
    let _ = rx_peer.take_loopback_receivers().await;
    drop(rx_peer);
    let err = tx_peer.send_audio_pcm(&pcm).await.unwrap_err();
    assert!(matches!(err, WebRtcTransportError::ChannelClosed));

    // C2-08: OpusDepacketizer rejects RTP version != 2
    let mut dep = OpusDepacketizer::new(1);
    let mut v0_pkt = RtpPacket::new(
        OPUS_PAYLOAD_TYPE,
        1,
        0,
        1,
        OpusFramingCodec::encode_frame(&pcm, 1).unwrap(),
    );
    v0_pkt.version = 0;
    assert!(dep.depacketize(&v0_pkt).is_err());

    // C2-08: Live WebRTC SDP honors audio_channels = 1
    let mut live_peer = WebRtcPeerTransport::new(PeerTransportConfig {
        mode: TransportMode::Live,
        audio_sample_rate: 48_000,
        audio_channels: 1,
        enable_audio: true,
        enable_video: false,
        ..Default::default()
    })
    .await
    .unwrap();
    let offer_sdp = live_peer.create_offer().await.unwrap();
    assert!(
        offer_sdp.contains("opus/48000/1") || !offer_sdp.contains("opus/48000/2"),
        "Expected mono Opus SDP for audio_channels=1, got:\n{offer_sdp}"
    );
    let _ = live_peer.close().await;
}

#[test]
fn test_tier5_opus_codec_exhaustive_toc_truncation_and_boundaries() {
    let mono_samples = vec![321i16; OPUS_SAMPLES_PER_FRAME_MONO];
    let stereo_samples = vec![-654i16; OPUS_SAMPLES_PER_FRAME_STEREO];
    let mono_encoded = OpusFramingCodec::encode_frame(&mono_samples, 1).unwrap();
    let stereo_encoded = OpusFramingCodec::encode_frame(&stereo_samples, 2).unwrap();

    for toc in 0u8..=u8::MAX {
        let mut m = mono_encoded.clone();
        m[0] = toc;
        let res_m = OpusFramingCodec::decode_frame(&m, 1);
        if toc == 0xA0 {
            assert_eq!(res_m.unwrap(), mono_samples);
        } else {
            assert!(res_m.is_err(), "Mono must reject TOC 0x{toc:02X}");
        }

        let mut s = stereo_encoded.clone();
        s[0] = toc;
        let res_s = OpusFramingCodec::decode_frame(&s, 2);
        if toc == 0xA4 {
            assert_eq!(res_s.unwrap(), stereo_samples);
        } else {
            assert!(res_s.is_err(), "Stereo must reject TOC 0x{toc:02X}");
        }
    }

    assert_eq!(OpusFramingCodec::decode_frame(&[], 1), Err(CodecError::TruncatedPayload));
    assert!(matches!(
        OpusFramingCodec::decode_frame(&[0xA0], 1),
        Err(CodecError::InvalidBufferSize { expected: 1920, got: 0 })
    ));
    assert!(matches!(
        OpusFramingCodec::decode_frame(&mono_encoded[..mono_encoded.len() - 1], 1),
        Err(CodecError::InvalidBufferSize { expected: 1920, got: 1919 })
    ));
    let mut oversized = mono_encoded.clone();
    oversized.push(0);
    assert!(matches!(
        OpusFramingCodec::decode_frame(&oversized, 1),
        Err(CodecError::InvalidBufferSize { expected: 1920, got: 1921 })
    ));

    assert_eq!(OpusFramingCodec::encode_frame(&mono_samples, 0), Err(CodecError::UnsupportedChannels(0)));
    assert_eq!(OpusFramingCodec::encode_frame(&mono_samples, 3), Err(CodecError::UnsupportedChannels(3)));
    assert_eq!(OpusFramingCodec::decode_frame(&mono_encoded, 0), Err(CodecError::UnsupportedChannels(0)));
    assert_eq!(OpusFramingCodec::encode_frame(&[], 1), Err(CodecError::InvalidBufferSize { expected: 960, got: 0 }));
    assert_eq!(OpusFramingCodec::encode_frame(&vec![0; 959], 1), Err(CodecError::InvalidBufferSize { expected: 960, got: 959 }));

    let mut pack = OpusPacketizer::new(2, 0x99);
    assert!(pack.push_pcm(&[1i16; 1919]).is_empty());
    let pkts = pack.push_pcm(&[1i16; 1921]);
    assert_eq!(pkts.len(), 2);
    assert_eq!(pkts[0].timestamp, 0);
    assert_eq!(pkts[1].timestamp, 960);
    pack.reset();
    assert!(pack.push_pcm(&[1i16; 100]).is_empty());
}

#[tokio::test]
async fn test_tier5_ai_media_tap_vad_timing_burst_overflow_and_edges() {
    let (in_tx, mut in_rx) = mpsc::channel::<Vec<i16>>(4);
    let (out_tx, mut out_rx) = mpsc::channel::<Vec<i16>>(4);
    let tap = Arc::new(
        DefaultAiMediaTap::new(48_000, 1, 0.05)
            .with_incoming_sender(in_tx)
            .with_outgoing_sender(out_tx),
    );

    let min_frame = vec![i16::MIN; 960];
    let rms_min = DefaultAiMediaTap::compute_rms(&min_frame);
    assert!(rms_min.is_finite() && (rms_min - 1.0).abs() < 1e-4);
    assert_eq!(DefaultAiMediaTap::compute_rms(&[]), 0.0);

    // Empty frame resets consecutive active frames
    let speech = vec![12_000i16; 960];
    tap.on_incoming_pcm_frame(&speech, 48_000, 1);
    tap.on_incoming_pcm_frame(&speech, 48_000, 1);
    assert!(!tap.trigger_vad_barge_in());
    tap.on_incoming_pcm_frame(&[], 48_000, 1);
    tap.on_incoming_pcm_frame(&speech, 48_000, 1);
    assert!(!tap.trigger_vad_barge_in());
    tap.reset_barge_in();
    while in_rx.try_recv().is_ok() {}

    let tap_clone = tap.clone();
    let waiter = tokio::spawn(async move {
        let start = std::time::Instant::now();
        tap_clone.wait_for_barge_in().await;
        start.elapsed()
    });
    tokio::task::yield_now().await;

    for _ in 0..500 {
        tap.on_incoming_pcm_frame(&speech, 48_000, 1);
        tap.on_outgoing_pcm_frame(&speech, 48_000, 1);
    }
    assert!(tap.trigger_vad_barge_in());
    let wake_latency = tokio::time::timeout(std::time::Duration::from_millis(100), waiter)
        .await
        .expect("barge-in waiter timed out")
        .unwrap();
    assert!(wake_latency.as_millis() < 60, "Wakeup took {:?}", wake_latency);

    let mut drained_in = 0;
    while in_rx.try_recv().is_ok() { drained_in += 1; }
    let mut drained_out = 0;
    while out_rx.try_recv().is_ok() { drained_out += 1; }
    assert_eq!(drained_in, 4);
    assert_eq!(drained_out, 4);
    assert!(tap.dropped_incoming_frames() > 0);
    assert!(tap.dropped_outgoing_frames() > 0);

    // C2-07: Single 960-sample frame at bogus sr=1 must not trigger 1-frame barge-in
    let tap_sr = DefaultAiMediaTap::new(48_000, 1, 0.05);
    tap_sr.on_incoming_pcm_frame(&speech, 1, 1);
    assert!(
        !tap_sr.trigger_vad_barge_in(),
        "Single 960-sample frame at bogus sr=1 must not trigger 1-frame barge-in"
    );
}

#[test]
fn test_tier5_video_viewport_and_call_overlay_malformed_dimensions() {
    assert!(encode_rgba_to_gpui_image(0, 100, &[]).is_none());
    assert!(encode_rgba_to_gpui_image(100, 0, &[]).is_none());
    assert!(encode_rgba_to_gpui_image(u32::MAX, u32::MAX, &[0u8; 16]).is_none());
    assert!(encode_rgba_to_gpui_image(64, 64, &[0u8; 128]).is_none());
    assert!(encode_rgba_to_gpui_image(1000, 1, &vec![0xFF; 1000 * 4]).is_some());
    assert!(encode_rgba_to_gpui_image(1, 1000, &vec![0xFF; 1000 * 4]).is_some());

    let pal = TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x5288C1, 14, 12);
    let mut vp = VideoViewport::new("Peer", pal);
    assert!(matches!(vp.aspect_ratio_mode, gpui::ObjectFit::Contain));
    assert!(!vp.ingest_raw_remote_rgba(0, 0, &[]));
    assert!(!vp.ingest_raw_local_rgba(16, 16, &[1, 2, 3]));
    assert!(!vp.has_active_video());

    // C2-07: Zero-dimension frame clears remote_frame = None & local_frame = None, and NaN bubble coords render safely
    let valid_img = encode_rgba_to_gpui_image(2, 2, &[255u8; 16]).unwrap();
    vp.set_remote_frame(Some(valid_img.clone()), 0, 100);
    assert!(vp.remote_frame.is_none());
    vp.set_local_frame(Some(valid_img), 2, 0);
    assert!(vp.local_frame.is_none());
    vp.set_local_camera_active(true);
    vp.bubble.x = f32::NAN;
    vp.bubble.y = f32::NAN;
    let _ = vp.render_viewport(f32::NAN, f32::INFINITY);

    let mut overlay = CallOverlay::new("".into());
    assert_eq!(overlay.persona_name, "Voice Call");
    overlay.set_duration(0);
    overlay.hangup();
    assert!(overlay.should_close);
    assert!(!overlay.show_rate_call);

    let mut overlay2 = CallOverlay::new("Alice".into());
    overlay2.set_duration(12);
    overlay2.hangup();
    assert!(overlay2.show_rate_call);
    assert!(!overlay2.is_connected);
    overlay2.submit_rating(255);
    assert_eq!(overlay2.call_rating, 5);
}

#[tokio::test]
async fn test_tier5_hermes_platform_im_permission_boundaries_and_schema() {
    let dir = TempDir::new().unwrap();
    let db = Arc::new(ServerDatabase::open(dir.path().join("hermes_im.db")).unwrap());
    let mut cfg = ServerConfig::default();
    cfg.media_storage_path = dir.path().join("media").display().to_string();
    let state = Arc::new(AppState::with_db(cfg, db.clone()));

    let (alice, _, alice_raw_tok) = state.session_manager.register("alice_t5", "Pass123456!", "Alice", None, "CLI").unwrap();
    let (bob, _, bob_raw_tok) = state.session_manager.register("bob_t5", "Pass123456!", "Bob", None, "CLI").unwrap();
    let (eve, _, _) = state.session_manager.register("eve_t5", "Pass123456!", "Eve", None, "CLI").unwrap();
    let now_ms = Utc::now().timestamp_millis();
    for (uid, uname, fname) in [
        (alice.user_id, "alice_t5", "Alice"),
        (bob.user_id, "bob_t5", "Bob"),
        (eve.user_id, "eve_t5", "Eve"),
    ] {
        db.create_user(&UserRecord {
            user_id: uid,
            username: Some(uname.into()),
            phone: None,
            first_name: fname.into(),
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
        }).unwrap();
    }

    let ctx_alice = state.hermes.build_tool_context_with_user("sess_alice", None, None, Some(&alice.user_id.0.to_string()), 1, Some(ToolsetPreset::Full), 0, Arc::new(AtomicBool::new(false)));
    let ctx_bob = state.hermes.build_tool_context_with_user("sess_bob", None, None, Some(&bob.user_id.0.to_string()), 1, Some(ToolsetPreset::Full), 0, Arc::new(AtomicBool::new(false)));
    let ctx_eve = state.hermes.build_tool_context_with_user("sess_eve", None, None, Some(&eve.user_id.0.to_string()), 1, Some(ToolsetPreset::Full), 0, Arc::new(AtomicBool::new(false)));
    let ctx_unauth = state.hermes.build_tool_context_with_user("sess_anon", None, None, None, 1, Some(ToolsetPreset::Full), 0, Arc::new(AtomicBool::new(false)));

    let r_chat = HermesToolExecutor::execute("create_channel", json!({ "title": "Alice Ops", "chat_type": "supergroup", "is_forum": true, "member_ids": [bob.user_id.0] }), &ctx_alice).await;
    assert!(r_chat.success);
    let chat_id = serde_json::from_str::<Value>(&r_chat.output).unwrap()["chat_id"].as_i64().unwrap();
    let r_msg = HermesToolExecutor::execute("send_message", json!({ "chat_id": chat_id, "text": "Alice post" }), &ctx_alice).await;
    let msg_id = serde_json::from_str::<Value>(&r_msg.output).unwrap()["message_id"].as_i64().unwrap();
    let r_topic = HermesToolExecutor::execute("manage_topic", json!({ "chat_id": chat_id, "action": "create", "title": "Topic 1" }), &ctx_alice).await;
    let topic_id = serde_json::from_str::<Value>(&r_topic.output).unwrap()["topic"]["topic_id"].as_i64().unwrap();

    // Unauthenticated & non-member denials
    assert!(!HermesToolExecutor::execute("send_message", json!({ "chat_id": chat_id, "text": "spoof" }), &ctx_unauth).await.success);
    for (tool, args) in [
        ("send_message", json!({ "chat_id": chat_id, "text": "intrude" })),
        ("edit_message", json!({ "chat_id": chat_id, "message_id": msg_id, "text": "hacked" })),
        ("delete_message", json!({ "chat_id": chat_id, "message_id": msg_id })),
        ("pin_message", json!({ "chat_id": chat_id, "message_id": msg_id })),
        ("add_reaction", json!({ "chat_id": chat_id, "message_id": msg_id, "emoji": "🔥" })),
        ("list_members", json!({ "chat_id": chat_id })),
        ("kick_member", json!({ "chat_id": chat_id, "user_id": bob.user_id.0 })),
        ("manage_topic", json!({ "chat_id": chat_id, "action": "list" })),
        ("create_poll", json!({ "chat_id": chat_id, "question": "Q?", "options": ["A", "B"] })),
    ] {
        assert!(!HermesToolExecutor::execute(tool, args, &ctx_eve).await.success, "Outsider Eve must be denied {tool}");
    }

    // Non-admin member Bob denials
    assert!(!HermesToolExecutor::execute("edit_message", json!({ "chat_id": chat_id, "message_id": msg_id, "text": "hijack" }), &ctx_bob).await.success);
    assert!(!HermesToolExecutor::execute("delete_message", json!({ "chat_id": chat_id, "message_id": msg_id }), &ctx_bob).await.success);
    assert!(!HermesToolExecutor::execute("pin_message", json!({ "chat_id": chat_id, "message_id": msg_id }), &ctx_bob).await.success);
    assert!(!HermesToolExecutor::execute("kick_member", json!({ "chat_id": chat_id, "user_id": alice.user_id.0 }), &ctx_bob).await.success);
    assert!(!HermesToolExecutor::execute("manage_topic", json!({ "chat_id": chat_id, "action": "update", "topic_id": topic_id, "title": "Hijacked" }), &ctx_bob).await.success);
    assert!(!HermesToolExecutor::execute("manage_topic", json!({ "chat_id": chat_id, "action": "delete", "topic_id": topic_id }), &ctx_bob).await.success);

    // Invalid payload schemas (C2-03)
    for (tool, bad_args) in [
        ("send_message", json!("not_an_object")),
        ("send_message", json!({ "chat_id": chat_id, "text": "   " })),
        ("send_message", json!({ "chat_id": "bad_type", "text": "hi" })),
        ("send_message", json!({ "chat_id": chat_id, "generative_card": { "bad": 1 } })),
        ("edit_message", json!({ "chat_id": chat_id, "message_id": -5, "text": "hi" })),
        ("delete_message", json!({ "chat_id": chat_id })),
        ("manage_topic", json!({ "chat_id": chat_id, "action": "nuke" })),
        ("vote_poll", json!({ "poll_id": "non_numeric" })),
    ] {
        assert!(!HermesToolExecutor::execute(tool, bad_args, &ctx_alice).await.success, "Malformed payload for {tool} must fail");
    }

    // Disabled toolset enforcement
    let ctx_minimal = state.hermes.build_tool_context_with_user("sess_min", None, None, Some(&alice.user_id.0.to_string()), 1, Some(ToolsetPreset::Minimal), 0, Arc::new(AtomicBool::new(false)));
    assert!(!HermesToolExecutor::execute("send_message", json!({ "chat_id": chat_id, "text": "no" }), &ctx_minimal).await.success);

    // C2-03: ApprovalMode::Deny blocks High-risk PlatformIm tools
    let ctx_deny = ctx_alice.clone().with_approval_mode(ApprovalMode::Deny);
    assert!(!HermesToolExecutor::execute("kick_member", json!({ "chat_id": chat_id, "user_id": bob.user_id.0 }), &ctx_deny).await.success);
    assert!(!HermesToolExecutor::execute("delete_message", json!({ "chat_id": chat_id, "message_id": msg_id }), &ctx_deny).await.success);

    // C2-03: Cross-chat pinning enforced on write, read, and miniapp PlatformIm tools
    let r_chat2 = HermesToolExecutor::execute("create_channel", json!({ "title": "Alice Secret Vault", "chat_type": "group" }), &ctx_alice).await;
    assert!(r_chat2.success);
    let chat_id_2 = serde_json::from_str::<Value>(&r_chat2.output).unwrap()["chat_id"].as_i64().unwrap();
    assert!(HermesToolExecutor::execute("send_message", json!({ "chat_id": chat_id_2, "text": "nuclear_launch_code_xyz" }), &ctx_alice).await.success);

    let ctx_pinned_c1 = state.hermes.build_tool_context_with_user("sess_pinned_c1", Some(chat_id), None, Some(&alice.user_id.0.to_string()), 1, Some(ToolsetPreset::Full), 0, Arc::new(AtomicBool::new(false)));
    for (tool, cross_args) in [
        ("send_message", json!({ "chat_id": chat_id_2, "text": "exfil" })),
        ("search_messages", json!({ "chat_id": chat_id_2, "query": "nuclear_launch_code_xyz" })),
        ("list_members", json!({ "chat_id": chat_id_2 })),
        ("launch_miniapp", json!({ "chat_id": chat_id_2, "bot_username": "test_bot", "short_name": "test_app" })),
        ("invoke_miniapp_action", json!({ "chat_id": chat_id_2, "bot_username": "test_bot", "short_name": "test_app", "action_type": "ping" })),
    ] {
        let r = HermesToolExecutor::execute(tool, cross_args, &ctx_pinned_c1).await;
        assert!(!r.success, "Pinned chat context must reject cross-chat '{tool}'");
        assert!(
            r.error.as_deref().unwrap_or("").contains("Cross-chat target mismatch"),
            "Expected Cross-chat target mismatch for '{tool}', got: {:?}",
            r.error
        );
    }
    assert!(HermesToolExecutor::execute("list_members", json!({ "chat_id": chat_id }), &ctx_pinned_c1).await.success);
    assert!(HermesToolExecutor::execute("search_messages", json!({ "query": "Alice" }), &ctx_pinned_c1).await.success);

    // Explorer 3: vote_poll cross-chat pinning (explicit chat_id_2, omitted chat_id, bad schema, direct bridge)
    let r_poll_c1 = HermesToolExecutor::execute("create_poll", json!({ "chat_id": chat_id, "question": "C1 Poll?", "options": ["A", "B"] }), &ctx_alice).await;
    assert!(r_poll_c1.success);
    let poll_c1 = serde_json::from_str::<Value>(&r_poll_c1.output).unwrap()["poll"]["poll_id"].as_i64().unwrap();
    let r_poll_c2 = HermesToolExecutor::execute("create_poll", json!({ "chat_id": chat_id_2, "question": "C2 Poll?", "options": ["X", "Y"] }), &ctx_alice).await;
    assert!(r_poll_c2.success);
    let poll_c2 = serde_json::from_str::<Value>(&r_poll_c2.output).unwrap()["poll"]["poll_id"].as_i64().unwrap();

    let vote_cross_explicit = HermesToolExecutor::execute("vote_poll", json!({ "chat_id": chat_id_2, "poll_id": poll_c2, "option_index": 0 }), &ctx_pinned_c1).await;
    assert!(!vote_cross_explicit.success);
    assert!(vote_cross_explicit.error.as_deref().unwrap_or("").contains("Cross-chat target mismatch"));
    let vote_cross_omitted = HermesToolExecutor::execute("vote_poll", json!({ "poll_id": poll_c2, "option_index": 0 }), &ctx_pinned_c1).await;
    assert!(!vote_cross_omitted.success);
    assert!(vote_cross_omitted.error.as_deref().unwrap_or("").contains("Cross-chat target mismatch"));
    let vote_bad_schema = HermesToolExecutor::execute("vote_poll", json!({ "chat_id": "bad_str", "poll_id": poll_c2, "option_index": 0 }), &ctx_pinned_c1).await;
    assert!(!vote_bad_schema.success);
    let bridge = state.hermes.platform_im_bridge().unwrap();
    let bridge_vote_err = bridge
        .execute_im_tool(
            "vote_poll",
            &json!({ "poll_id": poll_c2, "option_index": 0 }),
            PlatformImCaller { user_id: Some(alice.user_id.0), chat_id: Some(chat_id), ..Default::default() },
        )
        .await
        .unwrap_err();
    assert!(bridge_vote_err.contains("Cross-chat target mismatch"), "got: {bridge_vote_err}");
    assert!(HermesToolExecutor::execute("vote_poll", json!({ "poll_id": poll_c1, "option_index": 0 }), &ctx_pinned_c1).await.success);

    // Direct ServerPlatformImBridge search_messages pinning must not leak chat_id_2
    let bridge_search = bridge
        .execute_im_tool(
            "search_messages",
            &json!({ "query": "nuclear_launch_code_xyz" }),
            PlatformImCaller { user_id: Some(alice.user_id.0), chat_id: Some(chat_id), ..Default::default() },
        )
        .await
        .unwrap();
    assert_eq!(bridge_search["count"].as_u64(), Some(0));

    // Explorer 3: Seed real Bot + Published Pomodoro MiniApp and test invoke_miniapp_action cross-chat query_id hijack
    let bot_rec = db
        .create_registered_bot(
            alice.user_id.0 as i64,
            "t5_pomodoro_bot",
            "Pomodoro Bot",
            "Focus timer",
            "About",
            "Helpful timer",
            "Open",
            "",
            "[]",
            false,
            true,
        )
        .unwrap();
    let pomodoro_schema = gpui_telegram_server::db::miniapp_store::default_pomodoro_ui_schema().to_string();
    db.create_or_upsert_mini_app_with_options(
        bot_rec.bot_user_id,
        "t5_pomodoro_bot",
        "timer",
        "Focus Timer",
        "Pomodoro",
        "productivity",
        "⏱️",
        None,
        &pomodoro_schema,
        true,
        true,
        false,
        "",
        "",
        None,
        false,
    )
    .unwrap();
    let launch_c2 = HermesToolExecutor::execute("launch_miniapp", json!({ "bot_username": "t5_pomodoro_bot", "short_name": "timer", "chat_id": chat_id_2 }), &ctx_alice).await;
    assert!(launch_c2.success, "launch_c2 failed: {:?}", launch_c2.error);
    let query_id_c2 = serde_json::from_str::<Value>(&launch_c2.output).unwrap()["query_id"].as_str().unwrap().to_string();
    let launch_global = HermesToolExecutor::execute("launch_miniapp", json!({ "bot_username": "t5_pomodoro_bot", "short_name": "timer" }), &ctx_alice).await;
    assert!(launch_global.success);
    let query_id_global = serde_json::from_str::<Value>(&launch_global.output).unwrap()["query_id"].as_str().unwrap().to_string();

    let hijack_c2 = HermesToolExecutor::execute(
        "invoke_miniapp_action",
        json!({ "bot_username": "t5_pomodoro_bot", "short_name": "timer", "action_type": "widget:completed_sessions", "value": 99, "query_id": query_id_c2 }),
        &ctx_pinned_c1,
    )
    .await;
    assert!(!hijack_c2.success);
    assert!(hijack_c2.error.as_deref().unwrap_or("").contains("Cross-chat target mismatch"), "got: {:?}", hijack_c2.error);
    assert_eq!(db.get_mini_app_session(&query_id_c2).unwrap().unwrap().state_json, "{}");

    let hijack_global = HermesToolExecutor::execute(
        "invoke_miniapp_action",
        json!({ "bot_username": "t5_pomodoro_bot", "short_name": "timer", "action_type": "widget:completed_sessions", "value": 99, "query_id": query_id_global }),
        &ctx_pinned_c1,
    )
    .await;
    assert!(!hijack_global.success);
    assert_eq!(db.get_mini_app_session(&query_id_global).unwrap().unwrap().state_json, "{}");

    let hijack_unpinned_mismatch = HermesToolExecutor::execute(
        "invoke_miniapp_action",
        json!({ "chat_id": chat_id, "bot_username": "t5_pomodoro_bot", "short_name": "timer", "action_type": "widget:completed_sessions", "value": 99, "query_id": query_id_c2 }),
        &ctx_alice,
    )
    .await;
    assert!(!hijack_unpinned_mismatch.success);
    assert_eq!(db.get_mini_app_session(&query_id_c2).unwrap().unwrap().state_json, "{}");

    let launch_c1 = HermesToolExecutor::execute("launch_miniapp", json!({ "bot_username": "t5_pomodoro_bot", "short_name": "timer" }), &ctx_pinned_c1).await;
    assert!(launch_c1.success);
    let query_id_c1 = serde_json::from_str::<Value>(&launch_c1.output).unwrap()["query_id"].as_str().unwrap().to_string();
    let ok_c1_action = HermesToolExecutor::execute(
        "invoke_miniapp_action",
        json!({ "bot_username": "t5_pomodoro_bot", "short_name": "timer", "action_type": "widget:completed_sessions", "value": 5, "query_id": query_id_c1 }),
        &ctx_pinned_c1,
    )
    .await;
    assert!(ok_c1_action.success, "Positive c1 miniapp action failed: {:?}", ok_c1_action.error);

    // C2-01: Unauthenticated POST /v1/chat/completions returns 401
    let app = build_router_with_state(state.clone());
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/chat/completions")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"messages":[{"role":"user","content":"hi"}]}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    // C2-02: Non-admin Bob cannot mutate global state.hermes toolsets via PUT /api/v1/hermes/toolsets
    let resp2 = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/api/v1/hermes/toolsets")
                .header(header::AUTHORIZATION, format!("Bearer {bob_raw_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"preset":"minimal"}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp2.status(), StatusCode::FORBIDDEN);

    // C2-04: Bob cannot hijack or read Alice's private session (with or without yolo_mode)
    let _alice_turn = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/hermes/turns")
                .header(header::AUTHORIZATION, format!("Bearer {alice_raw_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"prompt":"my private secret is 42","session_id":"alice_priv_sess"}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let resp_turn_idor = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/hermes/turns")
                .header(header::AUTHORIZATION, format!("Bearer {bob_raw_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"prompt":"hijack_no_yolo","session_id":"alice_priv_sess","yolo_mode":false}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp_turn_idor.status(), StatusCode::FORBIDDEN);
    let resp_yolo_idor = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/hermes/turns")
                .header(header::AUTHORIZATION, format!("Bearer {bob_raw_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"prompt":"hijack","session_id":"alice_priv_sess","yolo_mode":true}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp_yolo_idor.status(), StatusCode::FORBIDDEN);
    let resp_get_idor = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/hermes/sessions/alice_priv_sess")
                .header(header::AUTHORIZATION, format!("Bearer {bob_raw_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp_get_idor.status(), StatusCode::FORBIDDEN);
    let resp_branch_idor = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/hermes/sessions/alice_priv_sess/branch")
                .header(header::AUTHORIZATION, format!("Bearer {bob_raw_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"branch_name":"stolen"}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp_branch_idor.status(), StatusCode::FORBIDDEN);
    let resp_alice_get = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/hermes/sessions/alice_priv_sess")
                .header(header::AUTHORIZATION, format!("Bearer {alice_raw_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp_alice_get.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_tier5_agentic_store_wal_concurrency_and_rollback_atomicity() {
    let dir = TempDir::new().unwrap();
    let db = Arc::new(ServerDatabase::open(dir.path().join("agentic_wal_t5.db")).unwrap());

    let mut initial = LocalAgenticStore::new();
    initial.boards.insert("b_init".into(), KanbanBoard {
        id: "b_init".into(),
        name: "Initial".into(),
        description: "keep".into(),
        columns: vec!["Todo".into()],
        created_at: Utc::now(),
    });
    db.converge_agentic_store("scope_tx", &initial).unwrap();

    let rollback_res: gpui_telegram_core::error::Result<()> = db.pool().with_transaction(|tx| {
        tx.execute("INSERT INTO hermes_kanban_boards (id, scope_key, name, description, columns_json, created_at, updated_at) VALUES ('b_bad', 'scope_tx', 'Bad', '', '[]', 1, 1)", []).unwrap();
        tx.execute("DELETE FROM hermes_kanban_boards WHERE scope_key = 'scope_tx' AND id = 'b_init'", []).unwrap();
        Err(TelegramError::Internal("simulated mid-transaction abort".into()))
    });
    assert!(rollback_res.is_err());

    let after_rollback = db.load_agentic_store("scope_tx").unwrap();
    assert!(after_rollback.boards.contains_key("b_init"));
    assert!(!after_rollback.boards.contains_key("b_bad"));

    let mut handles = Vec::new();
    for w in 0..4usize {
        let db_w = db.clone();
        handles.push(tokio::spawn(async move {
            for iter in 0..25usize {
                let mut store = LocalAgenticStore::new();
                let bid = format!("b_{w}_{iter}");
                store.boards.insert(bid.clone(), KanbanBoard {
                    id: bid,
                    name: format!("Board {w}-{iter}"),
                    description: "concurrent".into(),
                    columns: vec!["Todo".into(), "Done".into()],
                    created_at: Utc::now(),
                });
                db_w.converge_agentic_store(&format!("scope_c_{w}"), &store).unwrap();
            }
        }));
    }
    for r in 0..4usize {
        let db_r = db.clone();
        handles.push(tokio::spawn(async move {
            for _ in 0..50usize {
                let loaded = db_r.load_agentic_store(&format!("scope_c_{r}")).unwrap();
                assert!(loaded.boards.len() <= 1);
            }
        }));
    }
    for h in handles { h.await.unwrap(); }
}

#[test]
fn test_tier5_semantic_action_invalid_types_and_state_hash_tampering() {
    for bad_json in [
        r#"{"action":"DropDatabase","params":{}}"#,
        r#"{"action":"SelectChat","params":{"chat_id":"not_u64"}}"#,
        r#"{"action":"ComposeText","params":{}}"#,
        r#"{"action":"OpenModal","params":{"modal_id":123}}"#,
        r#"{"action":"ToggleCall","params":{"peer_id":1}}"#,
    ] {
        assert!(serde_json::from_str::<SemanticAction>(bad_json).is_err());
    }

    let mut tree = SemanticWorkspaceTree {
        timestamp_ms: 100,
        window_layout: "Normal".into(),
        selected_chat_id: Some(1),
        composer_text: "draft".into(),
        active_modal: None,
        is_call_active: false,
        window: Default::default(),
        nav_rail: Default::default(),
        sidebar: Default::default(),
        stage: Default::default(),
        composer: Default::default(),
        inspector: Default::default(),
        active_modal_node: None,
        state_hash: String::new(),
    };
    tree.recompute_state_hash();
    let baseline = tree.state_hash.clone();
    tree.timestamp_ms = 999999;
    assert_eq!(tree.compute_full_state_hash(), baseline);
    tree.window.width_px = 1440.0;
    assert_ne!(tree.compute_full_state_hash(), baseline);
    tree.window.width_px = 0.0;
    tree.nav_rail.connected = true;
    assert_ne!(tree.compute_full_state_hash(), baseline);
}

#[test]
fn test_tier5_multi_agent_floor_loop_budget_and_capability_token_forgery() {
    let mut q = FloorControlQueue::new(1_000);
    assert!(q.request_floor_at("a1", FloorPriority::DirectMention, 100));
    assert!(!q.request_floor_at("a2", FloorPriority::TaskAssignee, 200));
    q.release_floor("a2"); // non-holder release is no-op
    assert_eq!(q.current_speaker(), Some("a1"));
    q.drain_expired_leases(1099);
    assert_eq!(q.current_speaker(), Some("a1"));
    q.drain_expired_leases(1100);
    assert_eq!(q.current_speaker(), Some("a2"));

    // C2-06: release_floor & release_floor_at preserve simulated clock domain, and cancel_request removes queued waiters
    let mut q2 = FloorControlQueue::new(500);
    assert!(q2.request_floor_at("a1", FloorPriority::DirectMention, 100));
    assert!(!q2.request_floor_at("a2", FloorPriority::TaskAssignee, 200));
    assert!(!q2.request_floor_at("a3", FloorPriority::GeneralContribution, 250));
    assert_eq!(q2.queued_count(), 2);
    assert!(q2.cancel_request("a3"));
    assert!(!q2.cancel_request("a3"));
    assert_eq!(q2.queued_count(), 1);
    q2.release_floor("a1");
    assert_eq!(q2.current_speaker(), Some("a2"));
    q2.drain_expired_leases(749);
    assert_eq!(q2.current_speaker(), Some("a2"));
    q2.drain_expired_leases(750);
    assert_eq!(q2.current_speaker(), None);
    assert!(q2.request_floor_at("b1", FloorPriority::DirectMention, 1000));
    assert!(!q2.request_floor_at("b2", FloorPriority::TaskAssignee, 1100));
    q2.release_floor_at("b1", 1200);
    assert_eq!(q2.current_speaker(), Some("b2"));
    q2.drain_expired_leases(1699);
    assert_eq!(q2.current_speaker(), Some("b2"));
    q2.drain_expired_leases(1700);
    assert_eq!(q2.current_speaker(), None);

    let mut budget = LoopBudget::default();
    assert_eq!(budget.max_hops(), DEFAULT_MAX_LOOP_HOPS);
    for hop in 1..=5 {
        assert_eq!(budget.record_hop(&format!("agent_{hop}")).unwrap(), hop);
    }
    assert!(budget.is_exhausted());
    let err = budget.record_hop("agent_6").unwrap_err();
    assert_eq!(err.current, 6);
    assert_eq!(err.max, 5);
    assert_eq!(err.trace.len(), 5);
    budget.reset();
    assert!(!budget.is_exhausted());
    assert_eq!(budget.record_hop("agent_after_reset").unwrap(), 1);

    let mut zero_budget = LoopBudget::new(0);
    assert!(zero_budget.is_exhausted());
    assert!(zero_budget.record_hop("a").is_err());

    // C2-07: saturating_add(1) prevents u32::MAX overflow panic
    let mut max_b: LoopBudget = serde_json::from_str(
        r#"{"max_hops":4294967295,"current_hop":4294967295,"causal_trace":[]}"#,
    )
    .unwrap();
    assert!(max_b.record_hop("safe").is_err());

    let sk = SigningKey::from_bytes(&[7u8; 32]);
    let vk = sk.verifying_key();
    let wrong_vk = SigningKey::from_bytes(&[8u8; 32]).verifying_key();
    let tok = ScopedCapabilityToken::issue(&sk, "agent_alpha", 500, vec!["chat:write".into(), "agent:handoff".into()], 60);
    let exp = tok.expires_at;
    assert!(tok.verify_for_scope(&vk, exp, "agent_alpha", 500, "chat:write"));
    assert!(!tok.verify_for_scope(&vk, exp + 1, "agent_alpha", 500, "chat:write"));
    assert!(!tok.verify(&wrong_vk, exp - 1));
    assert!(!tok.verify_for_scope(&vk, exp - 1, "agent_beta", 500, "chat:write"));
    assert!(!tok.verify_for_scope(&vk, exp - 1, "agent_alpha", 501, "chat:write"));
    assert!(!tok.verify_for_scope(&vk, exp - 1, "agent_alpha", 500, "chat:admin"));

    let mut forged_sig = tok.clone();
    let first = if forged_sig.signature_hex.starts_with('a') { 'b' } else { 'a' };
    forged_sig.signature_hex.replace_range(0..1, &first.to_string());
    assert!(!forged_sig.verify(&vk, exp - 1));

    for bad_hex in ["", "abc", &"zz".repeat(64), &"00".repeat(63)] {
        let mut t = tok.clone();
        t.signature_hex = bad_hex.to_string();
        assert!(!t.verify(&vk, exp - 1));
    }
    let mut t_agent = tok.clone(); t_agent.agent_id = "agent_evil".into(); assert!(!t_agent.verify(&vk, exp - 1));
    let mut t_chat = tok.clone(); t_chat.chat_id = 999; assert!(!t_chat.verify(&vk, exp - 1));
    let mut t_act = tok.clone(); t_act.allowed_actions.push("*".into()); assert!(!t_act.verify(&vk, exp - 1));
    let mut t_exp = tok.clone(); t_exp.expires_at += 9999; assert!(!t_exp.verify(&vk, exp - 1));
    let expired_tok = ScopedCapabilityToken::issue(&sk, "a", 1, vec!["*".into()], -10);
    assert!(!expired_tok.verify(&vk, Utc::now().timestamp()));
}

#[tokio::test]
async fn test_tier5_hybrid_memory_acl_bypass_empty_query_ties_and_rrf() {
    let db = Arc::new(ServerDatabase::open_in_memory().unwrap());
    let idx = Arc::new(TantivyIndexer::in_ram().unwrap());
    idx.index_message(SearchDocument::new("1", "10", "1", 1000, "public alpha secret", None)).await.unwrap();
    idx.index_message(SearchDocument::new("2", "20", "2", 1000, "classified nuclear secret", None)).await.unwrap();
    idx.commit().await.unwrap();

    for inj in ["chat_id:20", "chat_id:20 OR classified", "+chat_id:20 +nuclear", "secret AND chat_id:20"] {
        let hits = idx.search_hits_for_chats(inj, &[10], 50).unwrap();
        assert!(hits.iter().all(|h| h.chat_id == "10"), "ACL leaked on '{inj}': {hits:?}");
    }

    // C2-05: Empty/whitespace chat_id filter bypass is forbidden
    assert!(idx.search_hits(&SearchQueryParams::new("classified").with_chat_id("")).is_err());
    assert!(idx.search_hits(&SearchQueryParams::new("classified").with_chat_id("   ")).is_err());

    let engine = HybridMemoryEngine::with_db_and_tantivy(db, Arc::downgrade(&idx));
    engine.index_message_or_file("10:1", 10, "message", "public alpha secret", vec![]);
    engine.index_message_or_file("20:2", 20, "message", "classified nuclear secret", vec![]);
    assert!(engine.search("classified", None, &[], None).is_empty());

    for q in ["", "   ", "!@#$%"] {
        assert!(engine.search(q, None, &[10], None).is_empty());
    }
    for q in ["[unclosed", "AND OR"] {
        let hits = engine.search(q, None, &[10], None);
        assert!(hits.iter().all(|h| h.chat_id == 10));
    }

    assert_eq!(HybridMemoryEngine::cosine_similarity(&[f32::NAN; 64], &[1.0; 64]), 0.0);
    assert_eq!(HybridMemoryEngine::cosine_similarity(&[f32::INFINITY; 64], &[1.0; 64]), 0.0);
    assert_eq!(HybridMemoryEngine::cosine_similarity(&[0.0; 64], &[1.0; 64]), 0.0);
    assert_eq!(HybridMemoryEngine::cosine_similarity(&[1.0; 32], &[1.0; 64]), 0.0);

    let tied_engine = HybridMemoryEngine::new();
    let emb = vec![1.0f32; 64];
    tied_engine.index_document(MemoryDocument { message_id: "tie_1".into(), chat_id: 1, content: "doc one".into(), embedding: emb.clone(), linked_entities: vec![] });
    tied_engine.index_document(MemoryDocument { message_id: "tie_2".into(), chat_id: 1, content: "doc two".into(), embedding: emb.clone(), linked_entities: vec![] });
    let tied_hits = tied_engine.search("", Some(&emb), &[1], None);
    assert_eq!(tied_hits.len(), 2);
    assert!(tied_hits[0].rrf_score >= tied_hits[1].rrf_score);

    assert_eq!(HybridMemoryEngine::compute_rrf_score(&[None, None, None], RRF_K), 0.0);
    let max_rrf = HybridMemoryEngine::compute_rrf_score(&[Some(1), Some(1), Some(1)], RRF_K);
    assert!((max_rrf - (3.0 / 61.0)).abs() < 1e-6);

    // C2-05: Tantivy-only hits included in RRF results even when self.docs is non-empty
    idx.index_message(SearchDocument::new("999", "10", "1", 2000, "unique_tantivy_only_needle", None)).await.unwrap();
    idx.commit().await.unwrap();
    assert_eq!(engine.search_with_limit("unique_tantivy_only_needle", None, &[10], None, 10).len(), 1);
}

#[tokio::test]
async fn test_tier5_hermes_acp_command_openai_and_sibling_isolation() {
    let dir = TempDir::new().unwrap();
    let db = Arc::new(ServerDatabase::open(dir.path().join("hermes_w3_iso.db")).unwrap());
    let mut cfg = ServerConfig::default();
    cfg.media_storage_path = dir.path().join("media").display().to_string();
    let state = Arc::new(AppState::with_db(cfg, db.clone()));

    let (alice, _, alice_tok) = state.session_manager.register("alice_w3", "Pass123456!", "Alice", None, "CLI").unwrap();
    let (bob, _, bob_tok) = state.session_manager.register("bob_w3", "Pass123456!", "Bob", None, "CLI").unwrap();
    let now_ms = Utc::now().timestamp_millis();
    for (uid, uname, fname) in [(alice.user_id, "alice_w3", "Alice"), (bob.user_id, "bob_w3", "Bob")] {
        db.create_user(&UserRecord {
            user_id: uid,
            username: Some(uname.into()),
            phone: None,
            first_name: fname.into(),
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
        }).unwrap();
    }

    let app = build_router_with_state(state.clone());

    // Setup: Alice creates private chat_vault, private Hermes session, approval, talk turn, cron job, and bot
    let create_vault = app.clone().oneshot(
        Request::builder().method("POST").uri("/api/v1/chats")
            .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({"title":"Alice Vault","chat_type":2}).to_string())).unwrap()
    ).await.unwrap();
    assert!(create_vault.status().is_success());
    let vault_id = serde_json::from_slice::<Value>(&to_bytes(create_vault.into_body(), usize::MAX).await.unwrap()).unwrap()["chat_id"].as_i64().unwrap();
    let post_vault = app.clone().oneshot(
        Request::builder().method("POST").uri(format!("/api/v1/chats/{vault_id}/messages"))
            .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({"text":"ALICE_VAULT_SECRET_CODE_999","client_nonce":7771}).to_string())).unwrap()
    ).await.unwrap();
    assert_eq!(post_vault.status(), StatusCode::CREATED);

    let alice_turn_res = app.clone().oneshot(
        Request::builder().method("POST").uri("/api/v1/hermes/turns")
            .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({"prompt":"my private secret is 42","session_id":"alice_priv_sess"}).to_string())).unwrap()
    ).await.unwrap();
    assert_eq!(alice_turn_res.status(), StatusCode::OK);
    let alice_turn_json: Value = serde_json::from_slice(&to_bytes(alice_turn_res.into_body(), usize::MAX).await.unwrap()).unwrap();
    let alice_turn_id = alice_turn_json["turn_id"].as_str().unwrap().to_string();

    state.hermes.store().enqueue_approval(CreateApprovalRequest {
        id: Some("apr_alice".into()),
        session_id: "alice_priv_sess".into(),
        command: "rm -rf /tmp/alice_secret_path".into(),
        risk_level: "high".into(),
        pattern_name: "rm_rf".into(),
    }).unwrap();

    let alice_talk_res = app.clone().oneshot(
        Request::builder().method("POST").uri("/api/v1/hermes/talk/turn")
            .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({"transcript":"ALICE_TALK_SECRET","session_id":"alice_talk_1"}).to_string())).unwrap()
    ).await.unwrap();
    assert_eq!(alice_talk_res.status(), StatusCode::OK);

    let alice_cron_res = app.clone().oneshot(
        Request::builder().method("POST").uri("/api/v1/hermes/cron")
            .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({"name":"alice_vault_cron","schedule":"@every 60s","prompt":"vault digest","chat_id":vault_id}).to_string())).unwrap()
    ).await.unwrap();
    assert_eq!(alice_cron_res.status(), StatusCode::CREATED);
    let alice_cron_id = serde_json::from_slice::<Value>(&to_bytes(alice_cron_res.into_body(), usize::MAX).await.unwrap()).unwrap()["job_id"].as_str().unwrap().to_string();

    let alice_bot = db.create_registered_bot(
        alice.user_id.0 as i64, "alice_priv_bot", "Alice Bot", "Desc", "About", "Persona", "Open", "", "[]", false, false
    ).unwrap();

    // 1. ACP JSON-RPC isolation (POST /api/v1/hermes/acp)
    let acp_impersonate = app.clone().oneshot(
        Request::builder().method("POST").uri("/api/v1/hermes/acp")
            .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({
                "jsonrpc": "2.0", "id": 1, "method": "tools/call",
                "params": {
                    "name": "search_messages",
                    "user_id": alice.user_id.0.to_string(),
                    "arguments": { "chat_id": vault_id, "query": "ALICE_VAULT_SECRET_CODE_999" }
                }
            }).to_string())).unwrap()
    ).await.unwrap();
    assert_eq!(acp_impersonate.status(), StatusCode::FORBIDDEN);
    let acp_imp_body = String::from_utf8_lossy(&to_bytes(acp_impersonate.into_body(), usize::MAX).await.unwrap()).to_string();
    assert!(!acp_imp_body.contains("ALICE_VAULT_SECRET_CODE_999"));
    assert_eq!(serde_json::from_str::<Value>(&acp_imp_body).unwrap()["error"]["code"].as_i64(), Some(-32003));

    let acp_sess_search = app.clone().oneshot(
        Request::builder().method("POST").uri("/api/v1/hermes/acp")
            .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({
                "jsonrpc": "2.0", "id": 2, "method": "tools/call",
                "params": { "name": "session_search", "arguments": { "query": "private secret is 42" } }
            }).to_string())).unwrap()
    ).await.unwrap();
    assert_eq!(acp_sess_search.status(), StatusCode::OK);
    let acp_ss_json: Value = serde_json::from_slice(&to_bytes(acp_sess_search.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(acp_ss_json["result"]["structured"]["count"].as_u64(), Some(0));
    assert!(acp_ss_json["result"]["structured"]["hits"].as_array().unwrap().is_empty());

    let acp_list = app.clone().oneshot(
        Request::builder().method("POST").uri("/api/v1/hermes/acp")
            .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({"jsonrpc":"2.0","id":3,"method":"session/list","params":{}}).to_string())).unwrap()
    ).await.unwrap();
    let acp_list_body = String::from_utf8_lossy(&to_bytes(acp_list.into_body(), usize::MAX).await.unwrap()).to_string();
    assert!(!acp_list_body.contains("alice_priv_sess"));

    for acp_payload in [
        json!({"jsonrpc":"2.0","id":4,"method":"command/execute","params":{"session_id":"alice_priv_sess","command":"/history"}}),
        json!({"jsonrpc":"2.0","id":5,"method":"session/prompt","params":{"session_id":"alice_priv_sess","prompt":"hijack"}}),
        json!({"jsonrpc":"2.0","id":6,"method":"session/prompt","params":{"chat_id":vault_id,"prompt":"hijack"}}),
        json!({"jsonrpc":"2.0","id":7,"method":"session/cancel","params":{"session_id":"alice_priv_sess"}}),
    ] {
        let r = app.clone().oneshot(
            Request::builder().method("POST").uri("/api/v1/hermes/acp")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(acp_payload.to_string())).unwrap()
        ).await.unwrap();
        assert_eq!(r.status(), StatusCode::FORBIDDEN, "Expected 403 for ACP {acp_payload}");
        let body_str = String::from_utf8_lossy(&to_bytes(r.into_body(), usize::MAX).await.unwrap()).to_string();
        assert!(!body_str.contains("private secret is 42"));
    }

    // 2. POST /api/v1/hermes/command isolation
    for cmd in ["/history", "/export sharegpt", "/clear", "/undo", "/yolo on"] {
        let r = app.clone().oneshot(
            Request::builder().method("POST").uri("/api/v1/hermes/command")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"session_id":"alice_priv_sess","command":cmd}).to_string())).unwrap()
        ).await.unwrap();
        assert_eq!(r.status(), StatusCode::FORBIDDEN, "Expected 403 on /command {cmd}");
    }
    assert_eq!(state.hermes.security().read().unwrap().approvals.get_session_mode("alice_priv_sess"), ApprovalMode::Prompt);
    assert!(!state.hermes.store().get_messages("alice_priv_sess", None).unwrap().is_empty());

    for global_cmd in ["/sessions", "/search private secret"] {
        let r = app.clone().oneshot(
            Request::builder().method("POST").uri("/api/v1/hermes/command")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"command":global_cmd}).to_string())).unwrap()
        ).await.unwrap();
        assert_eq!(r.status(), StatusCode::OK);
        let body = String::from_utf8_lossy(&to_bytes(r.into_body(), usize::MAX).await.unwrap()).to_string();
        assert!(!body.contains("alice_priv_sess"), "{global_cmd} leaked alice_priv_sess: {body}");
        assert!(!body.contains("private secret is 42"), "{global_cmd} leaked secret: {body}");
    }
    let r_checkout = app.clone().oneshot(
        Request::builder().method("POST").uri("/api/v1/hermes/command")
            .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({"command":"/checkout alice_priv_sess"}).to_string())).unwrap()
    ).await.unwrap();
    let checkout_json: Value = serde_json::from_slice(&to_bytes(r_checkout.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(checkout_json["data"]["is_error"].as_bool(), Some(true));

    // 3. POST /v1/chat/completions session hijack isolation
    let oai_idor = app.clone().oneshot(
        Request::builder().method("POST").uri("/v1/chat/completions")
            .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({"session_id":"alice_priv_sess","messages":[{"role":"user","content":"BOB_HIJACK_MARKER"}]}).to_string())).unwrap()
    ).await.unwrap();
    assert_eq!(oai_idor.status(), StatusCode::FORBIDDEN);
    let alice_msgs = state.hermes.store().get_messages("alice_priv_sess", None).unwrap();
    assert!(!alice_msgs.iter().any(|m| m.content.contains("BOB_HIJACK_MARKER")));

    // 4. Sibling endpoints isolation
    let ev_idor = app.clone().oneshot(
        Request::builder().method("GET").uri(format!("/api/v1/hermes/turns/{alice_turn_id}/events"))
            .header(header::AUTHORIZATION, format!("Bearer {bob_tok}")).body(Body::empty()).unwrap()
    ).await.unwrap();
    assert_eq!(ev_idor.status(), StatusCode::FORBIDDEN);

    let rb_idor = app.clone().oneshot(
        Request::builder().method("POST").uri("/api/v1/hermes/rollback")
            .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({"session_id":"alice_priv_sess"}).to_string())).unwrap()
    ).await.unwrap();
    assert_eq!(rb_idor.status(), StatusCode::FORBIDDEN);

    let talk_idor = app.clone().oneshot(
        Request::builder().method("POST").uri("/api/v1/hermes/talk/turn")
            .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({"transcript":"steal","session_id":"alice_talk_1"}).to_string())).unwrap()
    ).await.unwrap();
    assert_eq!(talk_idor.status(), StatusCode::FORBIDDEN);

    let bob_talk_new = app.clone().oneshot(
        Request::builder().method("POST").uri("/api/v1/hermes/talk/turn")
            .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({"transcript":"bob own talk"}).to_string())).unwrap()
    ).await.unwrap();
    assert_eq!(bob_talk_new.status(), StatusCode::OK);
    let bob_talk_sid = serde_json::from_slice::<Value>(&to_bytes(bob_talk_new.into_body(), usize::MAX).await.unwrap()).unwrap()["session_id"].as_str().unwrap().to_string();
    assert!(bob_talk_sid.starts_with(&format!("talk_{}_", bob.user_id.0)));

    let apr_list_bob = app.clone().oneshot(
        Request::builder().method("GET").uri("/api/v1/hermes/approvals")
            .header(header::AUTHORIZATION, format!("Bearer {bob_tok}")).body(Body::empty()).unwrap()
    ).await.unwrap();
    let apr_list_body = String::from_utf8_lossy(&to_bytes(apr_list_bob.into_body(), usize::MAX).await.unwrap()).to_string();
    assert!(!apr_list_body.contains("apr_alice"));

    let apr_decide_bob = app.clone().oneshot(
        Request::builder().method("POST").uri("/api/v1/hermes/approvals/apr_alice/decide")
            .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({"decision":"yolo"}).to_string())).unwrap()
    ).await.unwrap();
    assert_eq!(apr_decide_bob.status(), StatusCode::FORBIDDEN);

    let cron_list_bob = app.clone().oneshot(
        Request::builder().method("GET").uri("/api/v1/hermes/cron")
            .header(header::AUTHORIZATION, format!("Bearer {bob_tok}")).body(Body::empty()).unwrap()
    ).await.unwrap();
    let cron_list_body = String::from_utf8_lossy(&to_bytes(cron_list_bob.into_body(), usize::MAX).await.unwrap()).to_string();
    assert!(!cron_list_body.contains(&alice_cron_id));

    for (m, u, b) in [
        ("PATCH", format!("/api/v1/hermes/cron/{alice_cron_id}"), json!({"enabled":false}).to_string()),
        ("POST", format!("/api/v1/hermes/cron/{alice_cron_id}/run"), "{}".to_string()),
        ("DELETE", format!("/api/v1/hermes/cron/{alice_cron_id}"), String::new()),
    ] {
        let mut req_b = Request::builder().method(m).uri(&u).header(header::AUTHORIZATION, format!("Bearer {bob_tok}"));
        if !b.is_empty() {
            req_b = req_b.header(header::CONTENT_TYPE, "application/json");
        }
        let r = app.clone().oneshot(req_b.body(Body::from(b)).unwrap()).await.unwrap();
        assert_eq!(r.status(), StatusCode::FORBIDDEN, "Bob must get 403 on {m} {u}");
    }

    let soul_idor = app.clone().oneshot(
        Request::builder().method("PUT").uri("/api/v1/hermes/soul")
            .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({"bot_id": alice_bot.bot_user_id, "persona_name":"Hijacked"}).to_string())).unwrap()
    ).await.unwrap();
    assert_eq!(soul_idor.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_tier5_bot_takeover_and_acp_cron_isolation() {
    let dir = TempDir::new().unwrap();
    let db = Arc::new(ServerDatabase::open(dir.path().join("hermes_w4_bot_cron.db")).unwrap());
    let mut cfg = ServerConfig::default();
    cfg.media_storage_path = dir.path().join("media").display().to_string();
    let state = Arc::new(AppState::with_db(cfg, db.clone()));

    let (alice, _, alice_tok) = state.session_manager.register("alice_w4", "Pass123456!", "Alice", None, "CLI").unwrap();
    let (bob, _, bob_tok) = state.session_manager.register("bob_w4", "Pass123456!", "Bob", None, "CLI").unwrap();
    let now_ms = Utc::now().timestamp_millis();
    for (uid, uname, fname) in [(alice.user_id, "alice_w4", "Alice"), (bob.user_id, "bob_w4", "Bob")] {
        db.create_user(&UserRecord {
            user_id: uid,
            username: Some(uname.into()),
            phone: None,
            first_name: fname.into(),
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
        }).unwrap();
    }

    let app = build_router_with_state(state.clone());

    // Alice creates a private vault chat and a Fabushi bot with secret memory & soul
    let create_vault = app.clone().oneshot(
        Request::builder().method("POST").uri("/api/v1/chats")
            .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({"title":"Alice Vault W4","chat_type":2}).to_string())).unwrap()
    ).await.unwrap();
    assert!(create_vault.status().is_success());
    let vault_id = serde_json::from_slice::<Value>(&to_bytes(create_vault.into_body(), usize::MAX).await.unwrap()).unwrap()["chat_id"].as_i64().unwrap();

    let create_bot = app.clone().oneshot(
        Request::builder().method("POST").uri("/api/v1/fabushi-bots")
            .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({
                "display_name": "Alice Secret Bot",
                "username": "alice_secret_w4_bot",
                "description": "Private bot"
            }).to_string())).unwrap()
    ).await.unwrap();
    assert_eq!(create_bot.status(), StatusCode::CREATED);
    let alice_bot_id = serde_json::from_slice::<Value>(&to_bytes(create_bot.into_body(), usize::MAX).await.unwrap()).unwrap()["bot"]["bot_user_id"].as_u64().unwrap();

    let set_mem = app.clone().oneshot(
        Request::builder().method("PUT").uri(format!("/api/v1/hermes/memory?bot_id={alice_bot_id}"))
            .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({"memory_markdown":"ALICE_BOT_PRIVATE_MEMORY_XYZ"}).to_string())).unwrap()
    ).await.unwrap();
    assert_eq!(set_mem.status(), StatusCode::OK);

    let set_soul = app.clone().oneshot(
        Request::builder().method("PUT").uri("/api/v1/hermes/soul")
            .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({"bot_id":alice_bot_id,"persona_name":"AliceSoul","content":"# AliceSoul\n## Persona\nALICE_BOT_PRIVATE_SOUL"}).to_string())).unwrap()
    ).await.unwrap();
    assert_eq!(set_soul.status(), StatusCode::OK);

    // Bob creates his own private chat (where Alice's bot is NOT a member)
    let create_bob_chat = app.clone().oneshot(
        Request::builder().method("POST").uri("/api/v1/chats")
            .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({"title":"Bob Private Chat","chat_type":2}).to_string())).unwrap()
    ).await.unwrap();
    assert!(create_bob_chat.status().is_success());
    let bob_chat_id = serde_json::from_slice::<Value>(&to_bytes(create_bob_chat.into_body(), usize::MAX).await.unwrap()).unwrap()["chat_id"].as_i64().unwrap();

    // 1. Bob using bot_id=alice_bot_id + chat_id=bob_chat_id MUST receive 403 Forbidden across all surfaces
    for cmd in ["/memory view", "/soul set BOB_SOUL_CMD"] {
        let r = app.clone().oneshot(
            Request::builder().method("POST").uri("/api/v1/hermes/command")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"bot_id": alice_bot_id, "chat_id": bob_chat_id, "command": cmd}).to_string())).unwrap()
        ).await.unwrap();
        assert_eq!(r.status(), StatusCode::FORBIDDEN, "Expected 403 for /command {cmd}");
        let body = String::from_utf8_lossy(&to_bytes(r.into_body(), usize::MAX).await.unwrap()).to_string();
        assert!(!body.contains("ALICE_BOT_PRIVATE_MEMORY_XYZ"));
    }

    for acp_body in [
        json!({"jsonrpc":"2.0","id":11,"method":"tools/call","params":{"bot_id":alice_bot_id,"chat_id":bob_chat_id,"name":"memory","arguments":{"action":"read","target":"memory"}}}),
        json!({"jsonrpc":"2.0","id":12,"method":"tools/call","params":{"bot_id":alice_bot_id,"chat_id":bob_chat_id,"name":"skill_manage","arguments":{"action":"list"}}}),
        json!({"jsonrpc":"2.0","id":13,"method":"command/execute","params":{"bot_id":alice_bot_id,"chat_id":bob_chat_id,"command":"/memory view"}}),
        json!({"jsonrpc":"2.0","id":14,"method":"command/execute","params":{"bot_id":alice_bot_id,"chat_id":bob_chat_id,"command":"/soul set BOB_SOUL_ACP"}}),
        json!({"jsonrpc":"2.0","id":15,"method":"session/prompt","params":{"bot_id":alice_bot_id,"chat_id":bob_chat_id,"prompt":"/soul set BOB_SOUL_ACP_PROMPT"}}),
    ] {
        let r = app.clone().oneshot(
            Request::builder().method("POST").uri("/api/v1/hermes/acp")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(acp_body.to_string())).unwrap()
        ).await.unwrap();
        assert_eq!(r.status(), StatusCode::FORBIDDEN, "Expected 403 for ACP {acp_body}");
        let bytes = to_bytes(r.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8_lossy(&bytes).to_string();
        assert!(!body_str.contains("ALICE_BOT_PRIVATE_MEMORY_XYZ"));
        let v: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(v["error"]["code"].as_i64(), Some(-32003));
    }

    let turns_r = app.clone().oneshot(
        Request::builder().method("POST").uri("/api/v1/hermes/turns")
            .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({"bot_id":alice_bot_id,"chat_id":bob_chat_id,"prompt":"/soul set BOB_SOUL_TURN"}).to_string())).unwrap()
    ).await.unwrap();
    assert_eq!(turns_r.status(), StatusCode::FORBIDDEN);

    let agent_runs_r = app.clone().oneshot(
        Request::builder().method("POST").uri(format!("/api/v1/chats/{bob_chat_id}/agent-runs"))
            .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({"bot_id":alice_bot_id,"prompt":"/memory add BOB_MEM_AGENT_RUNS"}).to_string())).unwrap()
    ).await.unwrap();
    assert_eq!(agent_runs_r.status(), StatusCode::FORBIDDEN);

    // Even in a shared chat where Alice's bot IS a member, Bob cannot run bot-config commands/tools
    let shared_chat = app.clone().oneshot(
        Request::builder().method("POST").uri("/api/v1/chats")
            .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({"title":"Shared Bot Room","chat_type":1,"member_ids":[bob.user_id.0, alice_bot_id]}).to_string())).unwrap()
    ).await.unwrap();
    assert!(shared_chat.status().is_success());
    let shared_cid = serde_json::from_slice::<Value>(&to_bytes(shared_chat.into_body(), usize::MAX).await.unwrap()).unwrap()["chat_id"].as_i64().unwrap();

    let shared_cmd = app.clone().oneshot(
        Request::builder().method("POST").uri("/api/v1/hermes/command")
            .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({"bot_id": alice_bot_id, "chat_id": shared_cid, "command": "/memory view"}).to_string())).unwrap()
    ).await.unwrap();
    assert_eq!(shared_cmd.status(), StatusCode::FORBIDDEN);

    let shared_acp = app.clone().oneshot(
        Request::builder().method("POST").uri("/api/v1/hermes/acp")
            .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({"jsonrpc":"2.0","id":16,"method":"tools/call","params":{"bot_id":alice_bot_id,"chat_id":shared_cid,"name":"memory","arguments":{"action":"read"}}}).to_string())).unwrap()
    ).await.unwrap();
    assert_eq!(shared_acp.status(), StatusCode::FORBIDDEN);

    // Normal non-config turn in shared chat with added bot still succeeds!
    let shared_ok = app.clone().oneshot(
        Request::builder().method("POST").uri(format!("/api/v1/chats/{shared_cid}/agent-runs"))
            .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({"bot_id":alice_bot_id,"prompt":"Hello bot!"}).to_string())).unwrap()
    ).await.unwrap();
    assert_eq!(shared_ok.status(), StatusCode::OK);

    // Confirm Alice's bot MEMORY.md & SOUL.md were never leaked or mutated
    let alice_mem_check = app.clone().oneshot(
        Request::builder().method("GET").uri(format!("/api/v1/hermes/memory?bot_id={alice_bot_id}"))
            .header(header::AUTHORIZATION, format!("Bearer {alice_tok}")).body(Body::empty()).unwrap()
    ).await.unwrap();
    let alice_mem_body = String::from_utf8_lossy(&to_bytes(alice_mem_check.into_body(), usize::MAX).await.unwrap()).to_string();
    assert!(alice_mem_body.contains("ALICE_BOT_PRIVATE_MEMORY_XYZ"));
    assert!(!alice_mem_body.contains("BOB_MEM_AGENT_RUNS"));

    let alice_soul_check = app.clone().oneshot(
        Request::builder().method("POST").uri("/api/v1/hermes/command")
            .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({"bot_id": alice_bot_id, "command": "/soul view"}).to_string())).unwrap()
    ).await.unwrap();
    let alice_soul_body = String::from_utf8_lossy(&to_bytes(alice_soul_check.into_body(), usize::MAX).await.unwrap()).to_string();
    assert!(alice_soul_body.contains("ALICE_BOT_PRIVATE_SOUL"));
    assert!(!alice_soul_body.contains("BOB_SOUL_"));

    // 2. ACP tools/call cronjob_manage per-user & target_chat_id isolation
    let alice_acp_cron = app.clone().oneshot(
        Request::builder().method("POST").uri("/api/v1/hermes/acp")
            .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({
                "jsonrpc": "2.0", "id": 20, "method": "tools/call",
                "params": {
                    "name": "cronjob_manage",
                    "arguments": {
                        "action": "create",
                        "name": "alice_acp_cron",
                        "schedule": "@every 60s",
                        "prompt": "ALICE_ACP_CRON_SECRET",
                        "target_chat_id": vault_id
                    }
                }
            }).to_string())).unwrap()
    ).await.unwrap();
    assert_eq!(alice_acp_cron.status(), StatusCode::OK);
    let acp_cron_json: Value = serde_json::from_slice(&to_bytes(alice_acp_cron.into_body(), usize::MAX).await.unwrap()).unwrap();
    let acp_cron_id = acp_cron_json["result"]["structured"]["id"].as_str().unwrap().to_string();

    let bob_cron_list = app.clone().oneshot(
        Request::builder().method("POST").uri("/api/v1/hermes/acp")
            .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({
                "jsonrpc": "2.0", "id": 21, "method": "tools/call",
                "params": { "name": "cronjob_manage", "arguments": { "action": "list" } }
            }).to_string())).unwrap()
    ).await.unwrap();
    assert_eq!(bob_cron_list.status(), StatusCode::OK);
    let bob_list_body = String::from_utf8_lossy(&to_bytes(bob_cron_list.into_body(), usize::MAX).await.unwrap()).to_string();
    assert!(!bob_list_body.contains("ALICE_ACP_CRON_SECRET"));
    assert!(!bob_list_body.contains(&acp_cron_id));

    let bob_cron_create = app.clone().oneshot(
        Request::builder().method("POST").uri("/api/v1/hermes/acp")
            .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({
                "jsonrpc": "2.0", "id": 22, "method": "tools/call",
                "params": {
                    "name": "cronjob_manage",
                    "arguments": {
                        "action": "create",
                        "name": "pwn",
                        "schedule": "@every 60s",
                        "prompt": "PWN",
                        "target_chat_id": vault_id
                    }
                }
            }).to_string())).unwrap()
    ).await.unwrap();
    assert_eq!(bob_cron_create.status(), StatusCode::FORBIDDEN);
    let bob_create_json: Value = serde_json::from_slice(&to_bytes(bob_cron_create.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(bob_create_json["error"]["code"].as_i64(), Some(-32003));

    let bob_cron_pause = app.clone().oneshot(
        Request::builder().method("POST").uri("/api/v1/hermes/acp")
            .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({
                "jsonrpc": "2.0", "id": 23, "method": "tools/call",
                "params": { "name": "cronjob_manage", "arguments": { "action": "pause", "id": acp_cron_id } }
            }).to_string())).unwrap()
    ).await.unwrap();
    let bob_pause_json: Value = serde_json::from_slice(&to_bytes(bob_cron_pause.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(bob_pause_json["result"]["success"].as_bool(), Some(false));

    let bob_cron_del = app.clone().oneshot(
        Request::builder().method("POST").uri("/api/v1/hermes/acp")
            .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({
                "jsonrpc": "2.0", "id": 24, "method": "tools/call",
                "params": { "name": "cronjob_manage", "arguments": { "action": "delete", "id": acp_cron_id } }
            }).to_string())).unwrap()
    ).await.unwrap();
    let bob_del_json: Value = serde_json::from_slice(&to_bytes(bob_cron_del.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_ne!(bob_del_json["result"]["structured"]["deleted"].as_bool(), Some(true));

    // Confirm Alice's cron job is still intact and enabled
    let alice_cron_check = app.clone().oneshot(
        Request::builder().method("POST").uri("/api/v1/hermes/acp")
            .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json!({
                "jsonrpc": "2.0", "id": 25, "method": "tools/call",
                "params": { "name": "cronjob_manage", "arguments": { "action": "get", "id": acp_cron_id } }
            }).to_string())).unwrap()
    ).await.unwrap();
    let alice_get_json: Value = serde_json::from_slice(&to_bytes(alice_cron_check.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(alice_get_json["result"]["success"].as_bool(), Some(true));
    assert_eq!(alice_get_json["result"]["structured"]["enabled"].as_bool(), Some(true));
}

#[tokio::test]
async fn test_tier5_wave5_wave6_bot_computer_miniapp_talk_and_isolation() {
    use gpui_telegram_agent::computer::{AgentCloudServerConfigInput, ComputerCloudProvider};
    use gpui_telegram_agent::hermes::memory::MemoryTarget;
    use gpui_telegram_agent::hermes::skills::CreateSkillRequest;
    use gpui_telegram_agent::hermes::HermesEngine;
    use gpui_telegram_agent::SlashCommandParser;
    use std::collections::HashSet;
    use std::sync::atomic::AtomicBool;

    let dir = TempDir::new().unwrap();
    let db = Arc::new(ServerDatabase::open(dir.path().join("wave5_6_hardening.db")).unwrap());
    let mut cfg = ServerConfig::default();
    cfg.media_storage_path = dir.path().join("media").display().to_string();
    let state = Arc::new(AppState::with_db(cfg, db.clone()));
    let app = build_router_with_state(state.clone());

    let reg = |uname: &str| {
        let app_c = app.clone();
        let u = uname.to_string();
        async move {
            let res = app_c
                .oneshot(
                    Request::builder()
                        .method("POST")
                        .uri("/api/v1/auth/register")
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(Body::from(
                            json!({
                                "username": u,
                                "password": "Password123!",
                                "first_name": u
                            })
                            .to_string(),
                        ))
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(res.status(), StatusCode::CREATED);
            let v: Value =
                serde_json::from_slice(&to_bytes(res.into_body(), usize::MAX).await.unwrap())
                    .unwrap();
            (
                v["user_id"].as_u64().unwrap(),
                v["token"].as_str().unwrap().to_string(),
            )
        }
    };

    let (alice_id, alice_tok) = reg("w56_alice").await;
    let (bob_id, bob_tok) = reg("w56_bob").await;

    // Seed Alice private Fabushi bot + secret SOUL.md + secret skill + private vault + shared chat
    let vault_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/chats")
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({ "title": "W56 Alice Vault", "chat_type": 2 }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(vault_res.status(), StatusCode::CREATED);
    let vault_id = serde_json::from_slice::<Value>(
        &to_bytes(vault_res.into_body(), usize::MAX).await.unwrap(),
    )
    .unwrap()["chat_id"]
        .as_i64()
        .unwrap();

    let bob_chat_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/chats")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({ "title": "W56 Bob Private", "chat_type": 2 }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bob_chat_res.status(), StatusCode::CREATED);
    let bob_chat_id = serde_json::from_slice::<Value>(
        &to_bytes(bob_chat_res.into_body(), usize::MAX).await.unwrap(),
    )
    .unwrap()["chat_id"]
        .as_i64()
        .unwrap();

    let create_bot_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/fabushi-bots")
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "display_name": "Alice W56 Bot",
                        "username": "alice_w56_bot",
                        "description": "Private W56 bot"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(create_bot_res.status(), StatusCode::CREATED);
    let create_bot_json: Value = serde_json::from_slice(
        &to_bytes(create_bot_res.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();
    let alice_bot_id = create_bot_json["bot"]["bot_user_id"].as_u64().unwrap();
    let alice_bot_un = create_bot_json["registry"]["username"]
        .as_str()
        .unwrap()
        .to_string();

    let put_soul = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/api/v1/hermes/soul")
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "bot_id": alice_bot_id,
                        "persona_name": "AlicePrivateSoul",
                        "content": "# AlicePrivateSoul\nALICE_W56_PRIVATE_SOUL_SECRET"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(put_soul.status(), StatusCode::OK);

    let create_skill = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/hermes/skills?bot_id={alice_bot_id}"))
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "name": "alice-secret-skill",
                        "description": "Private skill",
                        "content": "ALICE_W56_SECRET_SKILL_BODY"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(create_skill.status(), StatusCode::CREATED);

    let shared_chat_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/chats")
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "title": "W56 Shared Bot Chat",
                        "chat_type": 2,
                        "member_ids": [bob_id, alice_bot_id]
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(shared_chat_res.status(), StatusCode::CREATED);
    let shared_cid = serde_json::from_slice::<Value>(
        &to_bytes(shared_chat_res.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap()["chat_id"]
        .as_i64()
        .unwrap();

    // Seed an unowned non-official bot (owner_user_id = 0)
    let unowned_bot = db
        .create_registered_bot(
            0,
            "unowned_w56_bot",
            "Unowned Bot",
            "Shared utility",
            "Shared utility",
            "",
            "",
            "",
            "[]",
            false,
            false,
        )
        .unwrap();
    let unowned_bot_id = unowned_bot.bot_user_id as u64;

    // ------------------------------------------------------------------------
    // 1. bot_computer_{get,put}, computers/cloud bind_bot_id, ComputerControlService, unowned isolation
    // ------------------------------------------------------------------------
    for target_bid in [alice_bot_id, 777_000u64] {
        let g = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!("/api/v1/bots/{target_bid}/computer"))
                    .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(g.status(), StatusCode::FORBIDDEN);
        let g_body =
            String::from_utf8_lossy(&to_bytes(g.into_body(), usize::MAX).await.unwrap())
                .to_string();
        assert!(!g_body.contains("ALICE_W56_PRIVATE_SOUL_SECRET"));

        let p = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri(format!("/api/v1/bots/{target_bid}/computer"))
                    .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        json!({
                            "persona_prompt": "BOB_HIJACKED_PERSONA",
                            "preferred_model": "bob-evil-model"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(p.status(), StatusCode::FORBIDDEN);
        assert!(db
            .get_bot_computer_profile(bob_id, target_bid)
            .unwrap()
            .is_none());

        let cloud_bind = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/computers/cloud")
                    .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        json!({
                            "display_name": "Bob Evil Node",
                            "host": "127.0.0.1",
                            "port": 22,
                            "username": "root",
                            "auth_method": "agent_http_token",
                            "secret": "tok",
                            "verify_connection": false,
                            "bind_bot_id": target_bid
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(cloud_bind.status(), StatusCode::FORBIDDEN);
        assert!(db
            .get_bot_computer_profile(bob_id, target_bid)
            .unwrap()
            .is_none());

        let svc_bind_err = ComputerCloudProvider::bind_bot_computer(
            state.computers.as_ref(),
            bob_id,
            target_bid,
            None,
        )
        .await
        .unwrap_err();
        assert!(matches!(svc_bind_err, TelegramError::PermissionDenied(_)));

        let svc_cfg_err = ComputerCloudProvider::configure_cloud_server(
            state.computers.as_ref(),
            bob_id,
            Some(target_bid),
            AgentCloudServerConfigInput {
                display_name: "SvcHijack".into(),
                host: "127.0.0.1".into(),
                bind_to_bot: true,
                ..Default::default()
            },
        )
        .await
        .unwrap_err();
        assert!(matches!(svc_cfg_err, TelegramError::PermissionDenied(_)));
    }

    // Unowned bot per-user bot_computer_configs isolation (no global leak/mutation)
    let alice_unowned_put = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/bots/{unowned_bot_id}/computer"))
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "persona_prompt": "ALICE_UNOWNED_PERSONA",
                        "preferred_model": "alice-unowned-model"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(alice_unowned_put.status(), StatusCode::OK);

    let bob_unowned_get = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/bots/{unowned_bot_id}/computer"))
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bob_unowned_get.status(), StatusCode::OK);
    let bob_unowned_body = String::from_utf8_lossy(
        &to_bytes(bob_unowned_get.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .to_string();
    assert!(!bob_unowned_body.contains("ALICE_UNOWNED_PERSONA"));
    assert!(!bob_unowned_body.contains("alice-unowned-model"));
    assert!(db
        .get_bot_by_user_id(unowned_bot_id as i64)
        .unwrap()
        .unwrap()
        .ai_persona
        .is_empty());

    // ------------------------------------------------------------------------
    // 2. fabushi-bots/*, bots_public_profile, miniapps_action (ai_copilot + ad-hoc "app")
    // ------------------------------------------------------------------------
    let fb_list_bob = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/fabushi-bots")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(fb_list_bob.status(), StatusCode::OK);
    let fb_list_body = String::from_utf8_lossy(
        &to_bytes(fb_list_bob.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .to_string();
    assert!(!fb_list_body.contains(&alice_bot_id.to_string()));
    assert!(!fb_list_body.contains("ALICE_W56_PRIVATE_SOUL_SECRET"));

    let fb_get_bob = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/fabushi-bots/{alice_bot_id}"))
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(fb_get_bob.status(), StatusCode::FORBIDDEN);

    for patch_bid in [alice_bot_id, 777_000u64, unowned_bot_id] {
        let fb_patch_bob = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("PATCH")
                    .uri(format!("/api/v1/fabushi-bots/{patch_bid}"))
                    .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        json!({ "display_name": "BOB_HIJACKED_BOT" }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            fb_patch_bob.status(),
            StatusCode::FORBIDDEN,
            "Bob PATCH /fabushi-bots/{patch_bid} must return 403"
        );
    }

    // bots_public_profile redacts ai_persona for non-owners of non-official bots
    let pub_prof_bob = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/bots/profile/{alice_bot_id}"))
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(pub_prof_bob.status(), StatusCode::OK);
    let pub_prof_bob_json: Value = serde_json::from_slice(
        &to_bytes(pub_prof_bob.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();
    assert_eq!(pub_prof_bob_json["bot"]["ai_persona"].as_str(), Some(""));

    let pub_prof_alice = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/bots/profile/{alice_bot_id}"))
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(pub_prof_alice.status(), StatusCode::OK);
    let pub_prof_alice_json: Value = serde_json::from_slice(
        &to_bytes(pub_prof_alice.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();
    assert!(pub_prof_alice_json["bot"]["ai_persona"]
        .as_str()
        .unwrap()
        .contains("ALICE_W56_PRIVATE_SOUL_SECRET"));

    // Wave 6: miniapps_action ai_copilot + ad-hoc "app" on Alice's private bot must be blocked
    // and must NOT leak SOUL or persist an ad-hoc MiniApp row
    let adhoc_launch_bob = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/miniapps/{alice_bot_un}/app/launch"))
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({ "launch_mode": "DirectLink" }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(adhoc_launch_bob.status(), StatusCode::NOT_FOUND);
    assert!(db
        .list_mini_apps_for_bot(alice_bot_id as i64)
        .unwrap()
        .is_empty());

    // Wave 7 (Probe 1d-side): launch with custom_url on Alice's unconfigured private bot
    // must be rejected (403), leak zero ai_persona, and persist zero ADHOC_WEBAPP_CATEGORY rows
    let adhoc_custom_launch_bob = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/miniapps/{alice_bot_un}/app/launch"))
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "launch_mode": "DirectLink",
                        "custom_url": "https://attacker.example/app"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(adhoc_custom_launch_bob.status(), StatusCode::FORBIDDEN);
    let adhoc_custom_body = String::from_utf8_lossy(
        &to_bytes(adhoc_custom_launch_bob.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .to_string();
    assert!(!adhoc_custom_body.contains("ALICE_W56_PRIVATE_SOUL_SECRET"));
    assert!(
        db.list_mini_apps_for_bot(alice_bot_id as i64)
            .unwrap()
            .is_empty(),
        "Non-owner ad-hoc /launch with custom_url must not persist MiniApp rows on unconfigured private bot"
    );

    let adhoc_copilot_bob = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/miniapps/{alice_bot_un}/app/action"))
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "action_type": "ai_copilot",
                        "prompt": "/tools list"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert!(
        matches!(
            adhoc_copilot_bob.status(),
            StatusCode::FORBIDDEN | StatusCode::NOT_FOUND
        ),
        "Bob calling ad-hoc miniapps/{alice_bot_un}/app/action ai_copilot must fail with 403/404, got {}",
        adhoc_copilot_bob.status()
    );
    let adhoc_body = String::from_utf8_lossy(
        &to_bytes(adhoc_copilot_bob.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .to_string();
    assert!(!adhoc_body.contains("ALICE_W56_PRIVATE_SOUL_SECRET"));
    assert!(
        db.list_mini_apps_for_bot(alice_bot_id as i64)
            .unwrap()
            .is_empty(),
        "Rejected ad-hoc miniapps_action must not persist MiniApp rows on Alice's bot"
    );

    // Published MiniApp chat-delivery guard & ai_copilot companion/shared-chat gate
    let create_app = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/miniapps")
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "bot_username": alice_bot_un,
                        "short_name": "w56app",
                        "title": "W56 App",
                        "description": "Test app"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert!(create_app.status().is_success());

    // Wave 7: Launching a published MiniApp as non-owner redacts companion_bot.ai_persona,
    // while owner launch still receives ai_persona
    let pub_launch_bob = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/miniapps/{alice_bot_un}/w56app/launch"))
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({ "launch_mode": "DirectLink" }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(pub_launch_bob.status(), StatusCode::OK);
    let pub_launch_bob_json: Value = serde_json::from_slice(
        &to_bytes(pub_launch_bob.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        pub_launch_bob_json["companion_bot"]["ai_persona"].as_str(),
        Some(""),
        "Non-owner /launch must redact companion_bot.ai_persona"
    );

    let pub_launch_alice = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/miniapps/{alice_bot_un}/w56app/launch"))
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({ "launch_mode": "DirectLink" }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(pub_launch_alice.status(), StatusCode::OK);
    let pub_launch_alice_json: Value = serde_json::from_slice(
        &to_bytes(pub_launch_alice.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();
    assert!(pub_launch_alice_json["companion_bot"]["ai_persona"]
        .as_str()
        .unwrap()
        .contains("ALICE_W56_PRIVATE_SOUL_SECRET"));

    // Non-owner Bob calling ai_copilot on published app WITHOUT installing or sharing a chat -> 403
    let pub_copilot_denied = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/miniapps/{alice_bot_un}/w56app/action"))
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "action_type": "ai_copilot",
                        "prompt": "summarize"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(pub_copilot_denied.status(), StatusCode::FORBIDDEN);

    let app_submit_bob = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/miniapps/{alice_bot_un}/w56app/action"))
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "action_type": "submit",
                        "data": "hi",
                        "chat_id": bob_chat_id
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(app_submit_bob.status(), StatusCode::OK);
    let submit_json: Value = serde_json::from_slice(
        &to_bytes(app_submit_bob.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();
    assert_ne!(submit_json["companion_chat_id"].as_i64(), Some(bob_chat_id));
    assert_ne!(submit_json["companion_chat_id"].as_i64(), Some(vault_id));

    // Wave 8: GET /api/v1/search?category=bots, GET /api/v1/bots/attach-menu, and
    // POST /api/v1/miniapps/{bot}/{short}/install must redact ai_persona for non-owner Bob
    // while retaining full ai_persona for owner Alice.
    let search_bots_bob = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/search?category=bots&q={alice_bot_un}"))
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(search_bots_bob.status(), StatusCode::OK);
    let search_bob_bytes = to_bytes(search_bots_bob.into_body(), usize::MAX)
        .await
        .unwrap();
    let search_bob_text = String::from_utf8_lossy(&search_bob_bytes).to_string();
    assert!(!search_bob_text.contains("ALICE_W56_PRIVATE_SOUL_SECRET"));
    let search_bob_json: Value = serde_json::from_slice(&search_bob_bytes).unwrap();
    let bob_found_bot = search_bob_json["bots"]
        .as_array()
        .unwrap()
        .iter()
        .find(|b| b["username"].as_str() == Some(alice_bot_un.as_str()))
        .expect("Bob search should find alice_bot_un in bots list");
    assert_eq!(
        bob_found_bot["ai_persona"].as_str(),
        Some(""),
        "Non-owner GET /search must redact ai_persona"
    );

    let search_bots_alice = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/search?category=bots&q={alice_bot_un}"))
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(search_bots_alice.status(), StatusCode::OK);
    let search_alice_json: Value = serde_json::from_slice(
        &to_bytes(search_bots_alice.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();
    let alice_found_bot = search_alice_json["bots"]
        .as_array()
        .unwrap()
        .iter()
        .find(|b| b["username"].as_str() == Some(alice_bot_un.as_str()))
        .expect("Alice search should find alice_bot_un in bots list");
    assert!(alice_found_bot["ai_persona"]
        .as_str()
        .unwrap()
        .contains("ALICE_W56_PRIVATE_SOUL_SECRET"));

    let attach_bob = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/bots/attach-menu")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(attach_bob.status(), StatusCode::OK);
    let attach_bob_bytes = to_bytes(attach_bob.into_body(), usize::MAX).await.unwrap();
    let attach_bob_text = String::from_utf8_lossy(&attach_bob_bytes).to_string();
    assert!(!attach_bob_text.contains("ALICE_W56_PRIVATE_SOUL_SECRET"));
    let attach_bob_json: Value = serde_json::from_slice(&attach_bob_bytes).unwrap();
    let bob_attach_bot = attach_bob_json["bots"]
        .as_array()
        .unwrap()
        .iter()
        .find(|b| b["username"].as_str() == Some(alice_bot_un.as_str()))
        .expect("Bob attach-menu should include bot with published app");
    assert_eq!(
        bob_attach_bot["ai_persona"].as_str(),
        Some(""),
        "Non-owner GET /bots/attach-menu must redact ai_persona"
    );

    let install_bob = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/miniapps/{alice_bot_un}/w56app/install"))
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({ "allow_write_access": true }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(install_bob.status(), StatusCode::OK);
    let install_bob_bytes = to_bytes(install_bob.into_body(), usize::MAX).await.unwrap();
    let install_bob_text = String::from_utf8_lossy(&install_bob_bytes).to_string();
    assert!(!install_bob_text.contains("ALICE_W56_PRIVATE_SOUL_SECRET"));
    let install_bob_json: Value = serde_json::from_slice(&install_bob_bytes).unwrap();
    assert_eq!(
        install_bob_json["companion_bot"]["ai_persona"].as_str(),
        Some(""),
        "Non-owner /install must redact companion_bot.ai_persona"
    );

    let install_alice = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/miniapps/{alice_bot_un}/w56app/install"))
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({ "allow_write_access": true }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(install_alice.status(), StatusCode::OK);
    let install_alice_json: Value = serde_json::from_slice(
        &to_bytes(install_alice.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();
    assert!(install_alice_json["companion_bot"]["ai_persona"]
        .as_str()
        .unwrap()
        .contains("ALICE_W56_PRIVATE_SOUL_SECRET"));

    // Wave 8: Non-owner launching an ad-hoc MiniApp on a non-official bot configured with an
    // external menu_button_url and passing custom_url MUST NOT overwrite mini_apps.web_url in SQLite.
    let create_menu_bot = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/bots")
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "username": "alice_menu_w8_bot",
                        "display_name": "Alice Menu W8 Bot",
                        "description": "External menu bot",
                        "about": "External menu bot",
                        "ai_persona": "ALICE_MENU_W8_PRIVATE_SOUL",
                        "menu_button_text": "Open Menu",
                        "menu_button_url": "https://alice-menu.example/start"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(create_menu_bot.status(), StatusCode::CREATED);
    let create_menu_bot_json: Value = serde_json::from_slice(
        &to_bytes(create_menu_bot.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();
    let menu_bot_id = create_menu_bot_json["bot"]["bot_user_id"].as_i64().unwrap();

    for attacker_url in [
        "https://attacker.example/override",
        "https://attacker.example/override2",
    ] {
        let menu_launch_bob = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/miniapps/alice_menu_w8_bot/app/launch")
                    .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        json!({
                            "launch_mode": "DirectLink",
                            "custom_url": attacker_url
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(menu_launch_bob.status(), StatusCode::OK);
        let menu_launch_bob_json: Value = serde_json::from_slice(
            &to_bytes(menu_launch_bob.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(
            menu_launch_bob_json["companion_bot"]["ai_persona"].as_str(),
            Some(""),
            "Non-owner ad-hoc /launch must redact companion_bot.ai_persona"
        );
        let persisted_apps = db.list_mini_apps_for_bot(menu_bot_id).unwrap();
        assert_eq!(persisted_apps.len(), 1);
        assert_eq!(
            persisted_apps[0].web_url, "https://alice-menu.example/start",
            "Non-owner custom_url must never overwrite mini_apps.web_url in SQLite"
        );
    }

    // ------------------------------------------------------------------------
    // 3. /api/v1/hermes/talk/turn, /api/v1/talk/{session,barge_in,audio}, and /tools command gating
    // ------------------------------------------------------------------------
    for cmd_str in [
        "/tools",
        "/tools list",
        "/tools preset minimal",
        "  /Tools@alice_w56_bot enable web",
    ] {
        assert!(
            SlashCommandParser::is_bot_config_command(cmd_str),
            "Expected is_bot_config_command == true for {cmd_str}"
        );
    }

    let sig_seq_before = state.talk_coordinator.read().signal_seq;
    for bot_val in [json!(alice_bot_id), json!(format!("bot_{alice_bot_id}"))] {
        for cid_val in [Value::Null, json!(bob_chat_id)] {
            for txt in ["hello", "/tools list", "/memory view", "/soul set PWN"] {
                let r = app
                    .clone()
                    .oneshot(
                        Request::builder()
                            .method("POST")
                            .uri("/api/v1/hermes/talk/turn")
                            .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                            .header(header::CONTENT_TYPE, "application/json")
                            .body(Body::from(
                                json!({
                                    "bot_id": bot_val,
                                    "chat_id": cid_val,
                                    "transcript": txt
                                })
                                .to_string(),
                            ))
                            .unwrap(),
                    )
                    .await
                    .unwrap();
                assert_eq!(r.status(), StatusCode::FORBIDDEN);
                let body_s = String::from_utf8_lossy(
                    &to_bytes(r.into_body(), usize::MAX).await.unwrap(),
                )
                .to_string();
                assert!(!body_s.contains("ALICE_W56_PRIVATE_SOUL_SECRET"));
            }
        }
    }
    assert_eq!(
        state.talk_coordinator.read().signal_seq,
        sig_seq_before,
        "Rejected talk/turn calls must not mutate talk_coordinator.signal_seq"
    );

    // /tools list & /tools preset minimal in shared_cid blocked across all 5 endpoints
    for tools_cmd in ["/tools list", "/tools preset minimal"] {
        let r_cmd = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/hermes/command")
                    .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        json!({
                            "bot_id": alice_bot_id,
                            "chat_id": shared_cid,
                            "command": tools_cmd
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(r_cmd.status(), StatusCode::FORBIDDEN);

        let r_turns = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/hermes/turns")
                    .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        json!({
                            "bot_id": alice_bot_id,
                            "chat_id": shared_cid,
                            "prompt": tools_cmd
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(r_turns.status(), StatusCode::FORBIDDEN);

        let r_talk = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/hermes/talk/turn")
                    .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        json!({
                            "bot_id": alice_bot_id,
                            "chat_id": shared_cid,
                            "transcript": tools_cmd
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(r_talk.status(), StatusCode::FORBIDDEN);

        for acp_body in [
            json!({
                "jsonrpc": "2.0", "id": 101, "method": "command/execute",
                "params": { "bot_id": alice_bot_id, "chat_id": shared_cid, "command": tools_cmd }
            }),
            json!({
                "jsonrpc": "2.0", "id": 102, "method": "session/prompt",
                "params": { "bot_id": alice_bot_id, "chat_id": shared_cid, "prompt": tools_cmd }
            }),
        ] {
            let r_acp = app
                .clone()
                .oneshot(
                    Request::builder()
                        .method("POST")
                        .uri("/api/v1/hermes/acp")
                        .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(Body::from(acp_body.to_string()))
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(r_acp.status(), StatusCode::FORBIDDEN);
        }
    }

    // Normal talk/turn in shared_cid succeeds; reusing that session_id with mismatched chat_id fails
    let shared_talk_ok = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/hermes/talk/turn")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "bot_id": alice_bot_id,
                        "chat_id": shared_cid,
                        "transcript": "Hello voice bot"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(shared_talk_ok.status(), StatusCode::OK);
    let shared_talk_sid = serde_json::from_slice::<Value>(
        &to_bytes(shared_talk_ok.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap()["session_id"]
        .as_str()
        .unwrap()
        .to_string();

    let talk_mismatch_cid = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/hermes/talk/turn")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "bot_id": alice_bot_id,
                        "chat_id": bob_chat_id,
                        "session_id": shared_talk_sid,
                        "transcript": "Cross-chat reuse"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(talk_mismatch_cid.status(), StatusCode::FORBIDDEN);

    // Alice talk/turn on alice_bot_id persists to FabushiBot's isolated store, NOT state.hermes
    let alice_talk = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/hermes/talk/turn")
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "bot_id": alice_bot_id,
                        "transcript": "Alice bot voice ping"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(alice_talk.status(), StatusCode::OK);
    let alice_talk_sid = serde_json::from_slice::<Value>(
        &to_bytes(alice_talk.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap()["session_id"]
        .as_str()
        .unwrap()
        .to_string();
    let expected_hermes_sid = format!("hermes_{alice_talk_sid}");
    assert!(
        state
            .hermes
            .store()
            .get_session(&expected_hermes_sid)
            .unwrap()
            .is_none(),
        "Fabushi bot talk/turn must NOT write session into host state.hermes"
    );
    assert!(
        state
            .fabushi_bots
            .get_by_user_id(alice_bot_id as i64)
            .unwrap()
            .hermes()
            .store()
            .get_session(&expected_hermes_sid)
            .unwrap()
            .is_some(),
        "Fabushi bot talk/turn must persist in FabushiBot's isolated HermesEngine"
    );

    // Wave 6: TalkCoordinator per-session / per-tenant isolation on /api/v1/talk/{audio,barge_in}
    let alice_audio = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/talk/audio?session_id={alice_talk_sid}"))
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .body(Body::from(vec![1u8, 2, 3, 4]))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(alice_audio.status(), StatusCode::OK);

    // Bob cannot barge_in or enqueue audio into Alice's session_id
    let bob_barge_alice = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/talk/barge_in")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({ "session_id": alice_talk_sid }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bob_barge_alice.status(), StatusCode::FORBIDDEN);

    let bob_audio_alice = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/talk/audio?session_id={alice_talk_sid}"))
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .body(Body::from(vec![9u8, 9, 9]))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bob_audio_alice.status(), StatusCode::FORBIDDEN);

    // POST /api/v1/talk/session mints fresh talk_{bob}_... without overwriting Alice's talk_sessions_db row
    // and Bob's barge_in on his own session does not flush Alice's audio buffer
    let bob_talk_sess = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/talk/session")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bob_talk_sess.status(), StatusCode::CREATED);
    let bob_talk_sess_id = serde_json::from_slice::<Value>(
        &to_bytes(bob_talk_sess.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap()["session_id"]
        .as_str()
        .unwrap()
        .to_string();
    assert_ne!(bob_talk_sess_id, alice_talk_sid);

    let bob_barge_own = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/talk/barge_in")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bob_barge_own.status(), StatusCode::OK);
    let bob_barge_json: Value = serde_json::from_slice(
        &to_bytes(bob_barge_own.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();
    assert_eq!(bob_barge_json["flushed_audio_chunks"].as_u64(), Some(0));
    assert_eq!(
        state
            .talk_sessions
            .read()
            .get(&alice_talk_sid)
            .unwrap()
            .buffered_chunks_count(),
        1,
        "Bob barge_in must not flush Alice's isolated talk session audio buffer"
    );

    let talk_rows = db.load_talk_sessions().unwrap();
    assert_eq!(
        talk_rows
            .iter()
            .find(|(sid, _, _, _, _)| sid == &alice_talk_sid)
            .map(|(_, uid, _, _, _)| *uid),
        Some(alice_id)
    );
    assert_eq!(
        talk_rows
            .iter()
            .find(|(sid, _, _, _, _)| sid == &bob_talk_sess_id)
            .map(|(_, uid, _, _, _)| *uid),
        Some(bob_id)
    );

    // Wave 7 (Probe 4d): Post-hoc hijack of Charlie's fallback talk_user_{charlie_id} session
    // via hermes/talk/turn and talk/{barge_in,audio} must be rejected with 403
    let (charlie_id, charlie_tok) = reg("w56_charlie").await;
    let charlie_fallback_sid = format!("talk_user_{charlie_id}");
    let charlie_audio = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/talk/audio")
                .header(header::AUTHORIZATION, format!("Bearer {charlie_tok}"))
                .body(Body::from(vec![1u8, 2, 3, 4]))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(charlie_audio.status(), StatusCode::OK);
    let charlie_audio_json: Value = serde_json::from_slice(
        &to_bytes(charlie_audio.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        charlie_audio_json["session_id"].as_str(),
        Some(charlie_fallback_sid.as_str())
    );

    // Charlie advances his own talk_user_{charlie_id} session to Speaking via hermes/talk/turn
    let charlie_own_turn = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/hermes/talk/turn")
                .header(header::AUTHORIZATION, format!("Bearer {charlie_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "session_id": charlie_fallback_sid,
                        "transcript": "charlie own turn"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(charlie_own_turn.status(), StatusCode::OK);

    let bob_hijack_charlie_turn = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/hermes/talk/turn")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "session_id": charlie_fallback_sid,
                        "transcript": "hijack charlie"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        bob_hijack_charlie_turn.status(),
        StatusCode::FORBIDDEN,
        "Probe 4d: Bob must not hijack Charlie's talk_user_{{id}} via hermes/talk/turn"
    );

    let bob_barge_charlie = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/talk/barge_in")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({ "session_id": charlie_fallback_sid }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bob_barge_charlie.status(), StatusCode::FORBIDDEN);

    let bob_audio_charlie = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/talk/audio?session_id={charlie_fallback_sid}"))
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .body(Body::from(vec![9u8, 9, 9]))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bob_audio_charlie.status(), StatusCode::FORBIDDEN);

    let charlie_barge_own = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/talk/barge_in")
                .header(header::AUTHORIZATION, format!("Bearer {charlie_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({ "session_id": charlie_fallback_sid }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(charlie_barge_own.status(), StatusCode::OK);
    let charlie_barge_json: Value = serde_json::from_slice(
        &to_bytes(charlie_barge_own.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();
    assert_eq!(charlie_barge_json["flushed_audio_chunks"].as_u64(), Some(1));
    assert_eq!(
        db.get_talk_session_owner(&charlie_fallback_sid).unwrap(),
        Some(charlie_id)
    );

    // Wave 7 (Probe 4f): Pre-claim of talk_user_{dave_id} before Dave creates any talk session
    // must be rejected with 403 and must not allow Bob to flush Dave's audio
    let (dave_id, dave_tok) = reg("w56_dave").await;
    let dave_fallback_sid = format!("talk_user_{dave_id}");

    let bob_preclaim_dave_turn = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/hermes/talk/turn")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "session_id": dave_fallback_sid,
                        "transcript": "preclaim dave"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        bob_preclaim_dave_turn.status(),
        StatusCode::FORBIDDEN,
        "Probe 4f: Bob must not pre-claim Dave's talk_user_{{id}} via hermes/talk/turn"
    );

    let bob_preclaim_barge = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/talk/barge_in")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({ "session_id": dave_fallback_sid }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bob_preclaim_barge.status(), StatusCode::FORBIDDEN);

    let bob_preclaim_audio = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/talk/audio?session_id={dave_fallback_sid}"))
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .body(Body::from(vec![7u8, 7, 7]))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bob_preclaim_audio.status(), StatusCode::FORBIDDEN);

    let dave_audio = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/talk/audio")
                .header(header::AUTHORIZATION, format!("Bearer {dave_tok}"))
                .body(Body::from(vec![5u8, 6, 7, 8]))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(dave_audio.status(), StatusCode::OK);
    let dave_audio_json: Value = serde_json::from_slice(
        &to_bytes(dave_audio.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        dave_audio_json["session_id"].as_str(),
        Some(dave_fallback_sid.as_str())
    );

    let bob_barge_dave_after = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/talk/barge_in")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({ "session_id": dave_fallback_sid }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        bob_barge_dave_after.status(),
        StatusCode::FORBIDDEN,
        "Probe 4f: Bob must not flush Dave's talk_user_{{id}} audio buffer"
    );
    assert_eq!(
        state
            .talk_sessions
            .read()
            .get(&dave_fallback_sid)
            .unwrap()
            .buffered_chunks_count(),
        1,
        "Dave's audio buffer must remain intact after Bob's rejected barge_in"
    );
    assert_eq!(
        db.get_talk_session_owner(&dave_fallback_sid).unwrap(),
        Some(dave_id)
    );

    // Unowned bot on talk/turn requires per-user bot_computer_configs when chat_id is None
    let unowned_talk_denied = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/hermes/talk/turn")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({ "bot_id": unowned_bot_id, "transcript": "hi unowned" }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(unowned_talk_denied.status(), StatusCode::FORBIDDEN);

    let bob_unowned_put = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/bots/{unowned_bot_id}/computer"))
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({ "persona_prompt": "Bob local persona" }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(bob_unowned_put.status(), StatusCode::OK);

    let unowned_talk_ok = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/hermes/talk/turn")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({ "bot_id": unowned_bot_id, "transcript": "hi unowned" }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(unowned_talk_ok.status(), StatusCode::OK);

    // ------------------------------------------------------------------------
    // 4. is_restricted_bot_caller on bound_agent_id == None (standard bot + standalone engine)
    // ------------------------------------------------------------------------
    let std_bot = db
        .create_registered_bot_with_options(
            alice_id as i64,
            "std_w56_bot",
            "Standard Bot",
            "Standard",
            "Standard",
            "",
            "default",
            "",
            "",
            "[]",
            false,
            false,
            true,
        )
        .unwrap();
    let std_bot_id = std_bot.bot_user_id as u64;
    let std_shared_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/chats")
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "title": "W56 Std Bot Room",
                        "chat_type": 2,
                        "member_ids": [bob_id, std_bot_id]
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(std_shared_res.status(), StatusCode::CREATED);
    let std_shared_cid = serde_json::from_slice::<Value>(
        &to_bytes(std_shared_res.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap()["chat_id"]
        .as_i64()
        .unwrap();

    state
        .hermes
        .memory()
        .add(MemoryTarget::Memory, "STD_BOT_HOST_SECRET")
        .unwrap();

    for (tool_name, args) in [
        ("memory", json!({ "action": "read", "target": "memory" })),
        ("skill_manage", json!({ "action": "list" })),
        ("skill_list", json!({})),
        ("skill_view", json!({ "name": "alice-secret-skill" })),
    ] {
        let r_std_acp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/hermes/acp")
                    .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        json!({
                            "jsonrpc": "2.0",
                            "id": 200,
                            "method": "tools/call",
                            "params": {
                                "bot_id": std_bot_id,
                                "chat_id": std_shared_cid,
                                "name": tool_name,
                                "arguments": args
                            }
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            r_std_acp.status(),
            StatusCode::FORBIDDEN,
            "Standard bot ACP tools/call {tool_name} as non-owner Bob must return 403"
        );
    }

    // Direct HermesEngine::in_memory() (bound_agent_id == None) verification
    let standalone_eng = HermesEngine::in_memory().unwrap();
    standalone_eng
        .memory()
        .add(MemoryTarget::Memory, "STANDALONE_MEM_SECRET")
        .unwrap();
    standalone_eng
        .skills()
        .skill_create(CreateSkillRequest {
            name: "standalone-skill".into(),
            description: "Standalone".into(),
            body: "STANDALONE_SKILL_BODY".into(),
            platforms: None,
            toolsets: None,
            tags: None,
            version: None,
            origin: None,
            trusted: Some(true),
        })
        .unwrap();

    let cancel_flag = Arc::new(AtomicBool::new(false));
    let restricted_ctx = standalone_eng.build_tool_context_scoped(
        "s_restricted",
        Some(10),
        Some(&std_bot_id.to_string()),
        Some(&bob_id.to_string()),
        1,
        Some(ToolsetPreset::HermesTelegram),
        0,
        cancel_flag.clone(),
        Some(HashSet::from([10])),
        false,
        false,
    );
    let mem_denied = HermesToolExecutor::execute(
        "memory",
        json!({ "action": "read", "target": "memory" }),
        &restricted_ctx,
    )
    .await;
    assert!(!mem_denied.success);
    assert!(!mem_denied.output.contains("STANDALONE_MEM_SECRET"));

    let skill_denied =
        HermesToolExecutor::execute("skill_list", json!({}), &restricted_ctx).await;
    assert!(!skill_denied.success);
    assert!(!skill_denied.output.contains("standalone-skill"));

    let owner_ctx = standalone_eng.build_tool_context_scoped(
        "s_owner",
        Some(10),
        Some(&std_bot_id.to_string()),
        Some(&alice_id.to_string()),
        1,
        Some(ToolsetPreset::HermesTelegram),
        0,
        cancel_flag.clone(),
        Some(HashSet::from([10])),
        false,
        true,
    );
    let mem_allowed = HermesToolExecutor::execute(
        "memory",
        json!({ "action": "read", "target": "memory" }),
        &owner_ctx,
    )
    .await;
    assert!(mem_allowed.success);
    assert!(mem_allowed.output.contains("STANDALONE_MEM_SECRET"));

    // ------------------------------------------------------------------------
    // 5. GET /soul, GET /skills*, heartbeat (incl. chatless hijack & "bot_" norm), Note 5 chatless cron & PATCH/upsert retarget
    // ------------------------------------------------------------------------
    for uri in [
        format!("/api/v1/hermes/soul?bot_id={alice_bot_id}"),
        format!("/api/v1/hermes/skills?bot_id={alice_bot_id}"),
        format!("/api/v1/hermes/skills/alice-secret-skill?bot_id={alice_bot_id}"),
    ] {
        let r_bob = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(&uri)
                    .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(r_bob.status(), StatusCode::FORBIDDEN, "Bob GET {uri} must be 403");
        let b_str =
            String::from_utf8_lossy(&to_bytes(r_bob.into_body(), usize::MAX).await.unwrap())
                .to_string();
        assert!(!b_str.contains("ALICE_W56_PRIVATE_SOUL_SECRET"));
        assert!(!b_str.contains("ALICE_W56_SECRET_SKILL_BODY"));

        let r_alice = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(&uri)
                    .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(r_alice.status(), StatusCode::OK, "Alice GET {uri} must be 200");
    }

    // Wave 6: Chatless Heartbeat secret leak (GET), existing-bot hijack (POST), & "bot_" normalization
    let hb_alice_chatless = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/hermes/heartbeat")
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "chat_id": Value::Null,
                        "bot_id": format!("bot_{alice_bot_id}"),
                        "enabled": false,
                        "prompt": "ALICE_CHATLESS_HB_SECRET"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(hb_alice_chatless.status(), StatusCode::OK);
    let hb_alice_json: Value = serde_json::from_slice(
        &to_bytes(hb_alice_chatless.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();
    assert_eq!(hb_alice_json["bot_id"].as_u64(), Some(alice_bot_id));

    let hb_get_bob = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/hermes/heartbeat")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(hb_get_bob.status(), StatusCode::FORBIDDEN);
    let hb_get_body = String::from_utf8_lossy(
        &to_bytes(hb_get_bob.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .to_string();
    assert!(!hb_get_body.contains("ALICE_CHATLESS_HB_SECRET"));

    let hb_rebind_bob = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/hermes/heartbeat")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "chat_id": bob_chat_id,
                        "enabled": true,
                        "prompt": "BOB_HIJACKED_HB"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(hb_rebind_bob.status(), StatusCode::FORBIDDEN);

    let hb_alice_verify = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/hermes/heartbeat")
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(hb_alice_verify.status(), StatusCode::OK);
    let hb_verify_json: Value = serde_json::from_slice(
        &to_bytes(hb_alice_verify.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();
    assert_eq!(hb_verify_json["prompt"].as_str(), Some("ALICE_CHATLESS_HB_SECRET"));
    assert_eq!(hb_verify_json["bot_id"].as_u64(), Some(alice_bot_id));
    assert!(hb_verify_json["chat_id"].is_null());

    // Heartbeat with explicit vault_id + explicit bot_id checks
    let hb_alice_vault = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/hermes/heartbeat")
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "chat_id": vault_id,
                        "bot_id": alice_bot_id,
                        "prompt": "ALICE_VAULT_HB"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(hb_alice_vault.status(), StatusCode::OK);

    for bad_hb in [
        json!({ "bot_id": alice_bot_id, "prompt": "PWN" }),
        json!({ "prompt": "PWN" }),
        json!({ "chat_id": Value::Null, "prompt": "PWN" }),
    ] {
        let r_hb = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/hermes/heartbeat")
                    .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(bad_hb.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(r_hb.status(), StatusCode::FORBIDDEN);
    }

    // Note 5: Chatless cron isolation (HTTP + /cron + standalone ToolContextBridge)
    let alice_chatless_cron = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/hermes/cron")
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "name": "alice_chatless_cron",
                        "schedule": "@every 60s",
                        "prompt": "ALICE_CHATLESS_CRON_SECRET"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(alice_chatless_cron.status(), StatusCode::CREATED);
    let chatless_cron_id = serde_json::from_slice::<Value>(
        &to_bytes(alice_chatless_cron.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap()["job_id"]
        .as_str()
        .unwrap()
        .to_string();

    let bob_cron_list = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/hermes/cron")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let bob_cron_list_body = String::from_utf8_lossy(
        &to_bytes(bob_cron_list.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .to_string();
    assert!(!bob_cron_list_body.contains(&chatless_cron_id));
    assert!(!bob_cron_list_body.contains("ALICE_CHATLESS_CRON_SECRET"));

    for (method, uri, body) in [
        (
            "POST",
            "/api/v1/hermes/cron".to_string(),
            json!({
                "job_id": chatless_cron_id,
                "name": "pwn",
                "schedule": "@every 60s",
                "prompt": "PWN"
            })
            .to_string(),
        ),
        (
            "PATCH",
            format!("/api/v1/hermes/cron/{chatless_cron_id}"),
            json!({ "enabled": false }).to_string(),
        ),
        (
            "POST",
            format!("/api/v1/hermes/cron/{chatless_cron_id}/run"),
            "{}".to_string(),
        ),
        (
            "DELETE",
            format!("/api/v1/hermes/cron/{chatless_cron_id}"),
            String::new(),
        ),
    ] {
        let mut rb = Request::builder()
            .method(method)
            .uri(&uri)
            .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"));
        if !body.is_empty() {
            rb = rb.header(header::CONTENT_TYPE, "application/json");
        }
        let res = app.clone().oneshot(rb.body(Body::from(body)).unwrap()).await.unwrap();
        assert_eq!(res.status(), StatusCode::FORBIDDEN);
    }

    for slash_cmd in [
        "/cron list".to_string(),
        format!("/cron pause {chatless_cron_id}"),
        format!("/cron delete {chatless_cron_id}"),
        format!("/cron history {chatless_cron_id}"),
        format!("/cron run {chatless_cron_id}"),
    ] {
        let r_slash = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/hermes/command")
                    .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(json!({ "command": slash_cmd }).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        let body_s = String::from_utf8_lossy(
            &to_bytes(r_slash.into_body(), usize::MAX).await.unwrap(),
        )
        .to_string();
        assert!(!body_s.contains("ALICE_CHATLESS_CRON_SECRET"));
        if slash_cmd == "/cron list" {
            assert!(!body_s.contains(&chatless_cron_id));
        }
    }

    // Wave 6: PATCH /api/v1/hermes/cron/{id} & POST upsert retargeting chat_id without re-sending bot_id
    let shared_cron_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/hermes/cron")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "name": "bob_shared_bot_cron",
                        "schedule": "@every 60s",
                        "prompt": "shared",
                        "chat_id": shared_cid,
                        "bot_id": format!("bot_{alice_bot_id}")
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(shared_cron_create.status(), StatusCode::CREATED);
    let shared_cron_json: Value = serde_json::from_slice(
        &to_bytes(shared_cron_create.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();
    assert_eq!(shared_cron_json["bot_id"].as_u64(), Some(alice_bot_id));
    let shared_cron_id = shared_cron_json["job_id"]
        .as_str()
        .unwrap()
        .to_string();

    for retarget_cid in [json!(bob_chat_id), Value::Null] {
        let patch_retarget = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("PATCH")
                    .uri(format!("/api/v1/hermes/cron/{shared_cron_id}"))
                    .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        json!({ "chat_id": retarget_cid }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            patch_retarget.status(),
            StatusCode::FORBIDDEN,
            "PATCH /cron/{shared_cron_id} retargeting chat_id={retarget_cid} while retaining alice_bot_id must return 403"
        );

        let upsert_retarget = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/hermes/cron")
                    .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        json!({
                            "job_id": shared_cron_id,
                            "name": "bob_retarget_upsert",
                            "schedule": "@every 60s",
                            "chat_id": retarget_cid
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            upsert_retarget.status(),
            StatusCode::FORBIDDEN,
            "POST /cron upsert retargeting chat_id={retarget_cid} while retaining alice_bot_id must return 403"
        );
    }
    assert_eq!(
        state
            .hermes
            .store()
            .get_cron_job(&shared_cron_id)
            .unwrap()
            .unwrap()
            .target_chat_id,
        Some(shared_cid)
    );

    // Standalone HermesEngine::in_memory() (no agentic_backend) chatless cronjob_manage isolation
    let alice_ctx = standalone_eng.build_tool_context_scoped(
        "cron_alice",
        None,
        None,
        Some(&alice_id.to_string()),
        1,
        Some(ToolsetPreset::HermesTelegram),
        0,
        cancel_flag.clone(),
        Some(HashSet::from([vault_id])),
        false,
        true,
    );
    let created_standalone = HermesToolExecutor::execute(
        "cronjob_manage",
        json!({
            "action": "create",
            "name": "standalone_chatless",
            "schedule": "@every 60s",
            "prompt": "STANDALONE_CHATLESS_SECRET"
        }),
        &alice_ctx,
    )
    .await;
    assert!(created_standalone.success);
    let standalone_job_id = created_standalone
        .structured
        .as_ref()
        .and_then(|v| v.get("id"))
        .and_then(|v| v.as_str())
        .unwrap()
        .to_string();

    let bob_ctx = standalone_eng.build_tool_context_scoped(
        "cron_bob",
        None,
        None,
        Some(&bob_id.to_string()),
        1,
        Some(ToolsetPreset::HermesTelegram),
        0,
        cancel_flag,
        Some(HashSet::from([bob_chat_id])),
        false,
        true,
    );
    let bob_list =
        HermesToolExecutor::execute("cronjob_manage", json!({ "action": "list" }), &bob_ctx).await;
    assert!(!bob_list.output.contains(&standalone_job_id));
    assert!(!bob_list.output.contains("STANDALONE_CHATLESS_SECRET"));
    for act in ["get", "pause", "delete", "run_now"] {
        let res = HermesToolExecutor::execute(
            "cronjob_manage",
            json!({ "action": act, "id": standalone_job_id }),
            &bob_ctx,
        )
        .await;
        assert!(!res.success, "Bob standalone cronjob_manage action={act} must fail");
    }
}

#[tokio::test]
async fn test_tier5_wave8_official_bot_install_and_talk_isolation() {
    use gpui_telegram_server::db::miniapp_store::BOTFATHER_USER_ID;

    let dir = TempDir::new().unwrap();
    let db = Arc::new(ServerDatabase::open(dir.path().join("wave8_reg.db")).unwrap());
    let mut cfg = ServerConfig::default();
    cfg.media_storage_path = dir.path().join("media").display().to_string();
    let state = Arc::new(AppState::with_db(cfg, db.clone()));
    let app = build_router_with_state(state.clone());

    let reg = |uname: &str| {
        let app_c = app.clone();
        let u = uname.to_string();
        async move {
            let res = app_c
                .oneshot(
                    Request::builder()
                        .method("POST")
                        .uri("/api/v1/auth/register")
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(Body::from(
                            json!({
                                "username": u,
                                "password": "Password123!",
                                "first_name": u
                            })
                            .to_string(),
                        ))
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(res.status(), StatusCode::CREATED);
            let v: Value =
                serde_json::from_slice(&to_bytes(res.into_body(), usize::MAX).await.unwrap())
                    .unwrap();
            (
                v["user_id"].as_u64().unwrap(),
                v["token"].as_str().unwrap().to_string(),
            )
        }
    };

    let (_alice_id, alice_tok) = reg("w8_alice").await;
    let (bob_id, bob_tok) = reg("w8_bob").await;
    let (eve_id, eve_tok) = reg("w8_eve").await;
    let (dave_id, dave_tok) = reg("w8_dave").await;
    let (admin_id, admin_tok) = reg("w8_admin").await;
    db.pool()
        .with_conn(|conn| {
            conn.execute(
                "UPDATE users SET is_support = 1 WHERE user_id = ?1",
                rusqlite::params![admin_id as i64],
            )
            .map_err(|e| TelegramError::Database(e.to_string()))?;
            Ok(())
        })
        .unwrap();

    // ------------------------------------------------------------------------
    // 1. Official Bots (@BotFather & @fabushi_ai_bot) non-owner custom_url DB non-persistence
    // ------------------------------------------------------------------------
    assert!(db.list_mini_apps_for_bot(BOTFATHER_USER_ID as i64).unwrap().is_empty());
    assert!(db.list_mini_apps_for_bot(777_000).unwrap().is_empty());

    for (bot_slug, bot_uid) in [
        ("BotFather", BOTFATHER_USER_ID as i64),
        ("fabushi_ai_bot", 777_000i64),
    ] {
        let launch_atk = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/api/v1/miniapps/{bot_slug}/app/launch"))
                    .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        json!({
                            "launch_mode": "DirectLink",
                            "custom_url": format!("https://attacker.example/phish-{bot_slug}")
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            launch_atk.status(),
            StatusCode::FORBIDDEN,
            "Non-owner custom_url /launch on unconfigured official bot @{bot_slug} must return 403"
        );
        assert!(
            db.list_mini_apps_for_bot(bot_uid).unwrap().is_empty(),
            "Official bot @{bot_slug} must have zero persisted mini_apps rows"
        );
    }

    let victim_launch_bf = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/miniapps/BotFather/app/launch")
                .header(header::AUTHORIZATION, format!("Bearer {eve_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({ "launch_mode": "DirectLink" }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(victim_launch_bf.status(), StatusCode::NOT_FOUND);

    let html_bf = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/miniapps/BotFather/app")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(html_bf.status(), StatusCode::NOT_FOUND);

    // ------------------------------------------------------------------------
    // 2. miniapps_install AuthZ (403 on unpublished, ad-hoc, and unpublished official 777_010)
    // ------------------------------------------------------------------------
    let create_bot_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/bots")
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "username": "alice_w8_inst_bot",
                        "display_name": "Alice W8 Inst Bot",
                        "description": "Install AuthZ test bot",
                        "about": "Install AuthZ test bot",
                        "ai_persona": "ALICE_W8_INST_SECRET_PERSONA",
                        "menu_button_text": "Open Menu",
                        "menu_button_url": "https://alice-inst.example/menu"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(create_bot_res.status(), StatusCode::CREATED);
    let alice_bot_id = serde_json::from_slice::<Value>(
        &to_bytes(create_bot_res.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap()["bot"]["bot_user_id"]
        .as_i64()
        .unwrap();

    // (a) Materialize ad-hoc MiniApp row (__bot_inline_webapp__) via Bob launching menu URL
    let adhoc_launch = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/miniapps/alice_w8_inst_bot/app/launch")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({ "launch_mode": "DirectLink" }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(adhoc_launch.status(), StatusCode::OK);

    // (b) Unpublished real MiniApp (auto_menu_button=false so main_app_short_name remains unset)
    db.create_or_upsert_mini_app_with_options(
        alice_bot_id,
        "alice_w8_inst_bot",
        "secretapp",
        "Secret Admin App",
        "CONFIDENTIAL_UNPUB_DESC",
        "tools",
        "🔒",
        Some("https://internal.alice.example/secret-admin"),
        "{}",
        false,
        false,
        false,
        "",
        "",
        None,
        false,
    )
    .unwrap();

    // (c) Unpublished legacy official bot (777_010) MiniApp
    let now_ms = chrono::Utc::now().timestamp_millis();
    db.pool()
        .with_conn(|conn| {
            conn.execute(
                "INSERT OR IGNORE INTO users (user_id, username, first_name, is_bot, is_verified, account_generation, created_at_ms, updated_at_ms)
                 VALUES (777010, 'legacy_official_demo_bot', 'Legacy Official', 1, 1, 1, ?1, ?1)",
                rusqlite::params![now_ms],
            )
            .map_err(|e| TelegramError::Database(e.to_string()))?;
            conn.execute(
                "INSERT OR IGNORE INTO bots_registry (bot_user_id, owner_user_id, username, display_name, bot_token, description, about, ai_persona, menu_button_text, menu_button_url, is_official, is_inline, allow_attach_menu, attach_peer_types_json, commands_json, created_at, updated_at)
                 VALUES (777010, 0, 'legacy_official_demo_bot', 'Legacy Official', '777010:AA_TEST', 'Legacy', 'Legacy', 'LEGACY_SECRET', '', '', 1, 0, 0, '[]', '[]', ?1, ?1)",
                rusqlite::params![now_ms],
            )
            .map_err(|e| TelegramError::Database(e.to_string()))?;
            Ok(())
        })
        .unwrap();
    db.create_or_upsert_mini_app(
        777_010,
        "legacy_official_demo_bot",
        "legacyapp",
        "Legacy Official App",
        "LEGACY_UNPUB_DESC",
        "tools",
        "🧪",
        Some("https://legacy.example/internal"),
        "{}",
        false,
        false,
        false,
    )
    .unwrap();

    // (d) Published real MiniApp
    let create_pub = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/miniapps")
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "bot_username": "alice_w8_inst_bot",
                        "short_name": "pubapp",
                        "title": "Public App",
                        "description": "Published app"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert!(create_pub.status().is_success());

    for (bot_un, short_un, check_bot_id) in [
        ("alice_w8_inst_bot", "secretapp", alice_bot_id),
        ("alice_w8_inst_bot", "app", alice_bot_id),
        ("legacy_official_demo_bot", "legacyapp", 777_010i64),
    ] {
        let inst_forbid = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/api/v1/miniapps/{bot_un}/{short_un}/install"))
                    .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(json!({ "allow_write_access": true }).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            inst_forbid.status(),
            StatusCode::FORBIDDEN,
            "Non-owner Bob installing {bot_un}/{short_un} must return 403"
        );
        let body_txt = String::from_utf8_lossy(
            &to_bytes(inst_forbid.into_body(), usize::MAX).await.unwrap(),
        )
        .to_string();
        assert!(!body_txt.contains("CONFIDENTIAL_UNPUB_DESC"));
        assert!(!body_txt.contains("secret-admin"));
        assert!(!body_txt.contains("ALICE_W8_INST_SECRET_PERSONA"));
        assert!(!body_txt.contains("LEGACY_UNPUB_DESC"));
        assert!(
            db.find_companion_chat_id_for_user(check_bot_id, bob_id as i64)
                .unwrap()
                .is_none(),
            "Rejected install must not create companion DM chat"
        );
    }
    assert!(db.list_user_installed_mini_apps(bob_id as i64).unwrap().is_empty());
    assert_eq!(
        db.get_mini_app("alice_w8_inst_bot", "secretapp")
            .unwrap()
            .unwrap()
            .install_count,
        0
    );

    // Also verify non-owner miniapps_action on unpublished legacy official app 777_010 is 403
    let action_legacy = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/miniapps/legacy_official_demo_bot/legacyapp/action")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({ "action_type": "refresh" }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(action_legacy.status(), StatusCode::FORBIDDEN);

    // Owner Alice can install her own unpublished app
    let inst_owner_ok = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/miniapps/alice_w8_inst_bot/secretapp/install")
                .header(header::AUTHORIZATION, format!("Bearer {alice_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({ "allow_write_access": true }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(inst_owner_ok.status(), StatusCode::OK);

    // Non-owner Bob can install published app with redacted ai_persona
    let inst_pub_bob = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/miniapps/alice_w8_inst_bot/pubapp/install")
                .header(header::AUTHORIZATION, format!("Bearer {bob_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({ "allow_write_access": true }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(inst_pub_bob.status(), StatusCode::OK);
    let inst_pub_json: Value = serde_json::from_slice(
        &to_bytes(inst_pub_bob.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();
    assert_eq!(inst_pub_json["companion_bot"]["ai_persona"].as_str(), Some(""));

    // ------------------------------------------------------------------------
    // 3. Talk Session Split-Brain & Admin Reserved Pre-Creation Prevention
    // ------------------------------------------------------------------------
    let eve_reserved_sids = [
        format!("talk_user_{eve_id}"),
        format!("talk_{eve_id}_custom"),
    ];
    for rsid in &eve_reserved_sids {
        for bot_opt in [Value::Null, json!(777_000)] {
            let adm_pre = app
                .clone()
                .oneshot(
                    Request::builder()
                        .method("POST")
                        .uri("/api/v1/hermes/talk/turn")
                        .header(header::AUTHORIZATION, format!("Bearer {admin_tok}"))
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(Body::from(
                            json!({
                                "session_id": rsid,
                                "bot_id": bot_opt,
                                "transcript": "ADMIN_PRECREATE_SECRET_TRANSCRIPT"
                            })
                            .to_string(),
                        ))
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(
                adm_pre.status(),
                StatusCode::FORBIDDEN,
                "Admin pre-creation of {rsid} (bot_id={bot_opt}) must return 403"
            );
            assert!(
                db.get_talk_session_owner(rsid).unwrap().is_none(),
                "Rejected admin pre-creation must not write talk_sessions_db for {rsid}"
            );
            assert!(
                state
                    .hermes
                    .store()
                    .get_session(&format!("hermes_{rsid}"))
                    .unwrap()
                    .is_none(),
                "Rejected admin pre-creation must not write HermesEngine for {rsid}"
            );
        }

        let adm_audio = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/api/v1/talk/audio?session_id={rsid}"))
                    .header(header::AUTHORIZATION, format!("Bearer {admin_tok}"))
                    .body(Body::from(vec![1u8, 2, 3, 4]))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(adm_audio.status(), StatusCode::FORBIDDEN);

        let adm_barge = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/api/v1/talk/barge_in?session_id={rsid}"))
                    .header(header::AUTHORIZATION, format!("Bearer {admin_tok}"))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(adm_barge.status(), StatusCode::FORBIDDEN);

        let adm_turns = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/hermes/turns")
                    .header(header::AUTHORIZATION, format!("Bearer {admin_tok}"))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        json!({
                            "session_id": format!("hermes_{rsid}"),
                            "prompt": "ADMIN_PRECREATE_TURNS"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(adm_turns.status(), StatusCode::FORBIDDEN);
    }

    // Victim Eve flows: /talk/audio -> /talk/barge_in -> /hermes/talk/turn -> GET /hermes/sessions/hermes_{sid}
    let eve_audio = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/talk/audio")
                .header(header::AUTHORIZATION, format!("Bearer {eve_tok}"))
                .body(Body::from(vec![1u8, 2, 3, 4]))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(eve_audio.status(), StatusCode::OK);
    let eve_audio_json: Value = serde_json::from_slice(
        &to_bytes(eve_audio.into_body(), usize::MAX).await.unwrap(),
    )
    .unwrap();
    let eve_sid = eve_audio_json["session_id"].as_str().unwrap().to_string();
    assert_eq!(eve_sid, format!("talk_user_{eve_id}"));

    let eve_barge = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/talk/barge_in")
                .header(header::AUTHORIZATION, format!("Bearer {eve_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(eve_barge.status(), StatusCode::OK);
    let eve_barge_json: Value = serde_json::from_slice(
        &to_bytes(eve_barge.into_body(), usize::MAX).await.unwrap(),
    )
    .unwrap();
    assert_eq!(eve_barge_json["session_id"].as_str(), Some(eve_sid.as_str()));

    let eve_turn = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/hermes/talk/turn")
                .header(header::AUTHORIZATION, format!("Bearer {eve_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "session_id": eve_sid,
                        "transcript": "Eve legitimate voice turn"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(eve_turn.status(), StatusCode::OK);

    let eve_sess_get = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/hermes/sessions/hermes_{eve_sid}"))
                .header(header::AUTHORIZATION, format!("Bearer {eve_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(eve_sess_get.status(), StatusCode::OK);
    let eve_sess_txt = String::from_utf8_lossy(
        &to_bytes(eve_sess_get.into_body(), usize::MAX).await.unwrap(),
    )
    .to_string();
    assert!(!eve_sess_txt.contains("ADMIN_PRECREATE_SECRET_TRANSCRIPT"));

    // Case C2 + Case D: Victim Dave creates talk_user_{dave_id} via /talk/audio only,
    // Admin is blocked (403) from binding it to Alice's private bot,
    // Admin calling hermes/talk/turn without private bot syncs HermesEngine owner_user_id = dave_id,
    // and Dave's subsequent hermes/talk/turn & GET session remain 200 OK.
    let dave_audio = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/talk/audio")
                .header(header::AUTHORIZATION, format!("Bearer {dave_tok}"))
                .body(Body::from(vec![9u8, 8, 7]))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(dave_audio.status(), StatusCode::OK);
    let dave_sid = format!("talk_user_{dave_id}");
    assert_eq!(
        db.get_talk_session_owner(&dave_sid).unwrap(),
        Some(dave_id)
    );

    let adm_private_bot_bind = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/hermes/talk/turn")
                .header(header::AUTHORIZATION, format!("Bearer {admin_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "session_id": dave_sid,
                        "bot_id": alice_bot_id,
                        "transcript": "Admin bind victim session to private bot"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        adm_private_bot_bind.status(),
        StatusCode::FORBIDDEN,
        "Admin must not bind victim session to a private bot victim cannot access"
    );

    let adm_c2_turn = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/hermes/talk/turn")
                .header(header::AUTHORIZATION, format!("Bearer {admin_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "session_id": dave_sid,
                        "transcript": "Admin support turn on existing victim talk session"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(adm_c2_turn.status(), StatusCode::OK);

    let dave_hermes_rec = state
        .hermes
        .store()
        .get_session(&format!("hermes_{dave_sid}"))
        .unwrap()
        .expect("Hermes session must exist after Case C2 turn");
    let dave_meta: Value = serde_json::from_str(&dave_hermes_rec.metadata_json).unwrap();
    assert_eq!(
        dave_meta["owner_user_id"].as_str(),
        Some(dave_id.to_string().as_str()),
        "HermesEngine owner_user_id must stay synced to victim Dave, not Admin"
    );

    let dave_followup_turn = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/hermes/talk/turn")
                .header(header::AUTHORIZATION, format!("Bearer {dave_tok}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "session_id": dave_sid,
                        "transcript": "Dave follow-up turn"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(dave_followup_turn.status(), StatusCode::OK);

    let dave_sess_get = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/hermes/sessions/hermes_{dave_sid}"))
                .header(header::AUTHORIZATION, format!("Bearer {dave_tok}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(dave_sess_get.status(), StatusCode::OK);
}
