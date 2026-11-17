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
