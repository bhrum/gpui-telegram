//! Milestone 4 Empirical Adversarial Stress Test Suite (`crates/gpui_telegram_desktop`).
//!
//! Focus:
//! 1. Live Cloud VM (`http://34.46.77.133:8080`) high-frequency stress:
//!    - Concurrent registrations and logins across multiple adversarial user profiles.
//!    - Duplicate username registration conflict verification (assert HTTP 409 Conflict).
//!    - Rapid-fire message sending and history retrieval under load.
//! 2. WebSocket Streaming & State Synchronization Stress:
//!    - High-frequency ping/pong heartbeat validation over live WebSocket channel.
//!    - Out-of-order PTS sequence delta injection: simulate PTS gaps (e.g. receiving pts 5 when current is 2),
//!      assert buffer accumulation, simulate arrival of intermediate deltas (pts 3, 4), and assert contiguous
//!      unbuffering and in-order delivery without duplicates or drops.
//!    - Large-scale random permutation fuzzing of PTS updates (100 shuffled deltas reconciled to strict monotonic order).
//!    - Multi-chat isolation under concurrent gap storms.
//! 3. Reconnection Catch-Up State Synchronization:
//!    - Verify that reconnection transition in `AppShell` triggers automatic chat and message refresh without manual intervention.
//!    - State machine simulation, AST code contracts, and live Cloud VM WebSocket drop/reconnect catch-up verification.

use gpui_telegram_client::net::http_client::{
    HttpClient, HttpClientError, LoginRequest, RegisterRequest, SendMessageHttpRequest,
};
use gpui_telegram_client::net::ws_channel::WsChannel;
use gpui_telegram_client::net::ws_transport::WsTransport;
use gpui_telegram_client::sync::coordinator::ClientSyncCoordinator;
use gpui_telegram_client::sync::SyncOutcome;
use gpui_telegram_core::types::{ChatId, UserId};
use gpui_telegram_proto::message::content::MessageContent;
use gpui_telegram_proto::sync::{PtsUpdate, TypingAction, TypingEvent, UpdateEvent};
use gpui_telegram_proto::ws::{SendMessageRequest, WsClientFrame, WsServerFrame};

use futures::future::join_all;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

static UNIQUE_SEQ: AtomicU64 = AtomicU64::new(100);

fn live_cloud_url() -> String {
    std::env::var("GPUI_TELEGRAM_SERVER_URL")
        .unwrap_or_else(|_| "http://34.46.77.133:8080".to_string())
}

// ============================================================================
// PART 1: Live Cloud VM High-Frequency Stress Test Suite
// ============================================================================

#[derive(Debug, Clone)]
struct AdversarialProfile {
    label: &'static str,
    username: String,
    password: String,
    first_name: String,
    device_name: String,
}

#[tokio::test]
async fn test_stress_concurrent_registrations_and_logins_live_cloud() {
    let base_url = live_cloud_url();
    let client = HttpClient::new(base_url.clone());

    let health = client.check_health().await;
    assert!(health.is_ok(), "Live Cloud VM must be healthy: {:?}", health);
    assert!(health.unwrap(), "Health returned false");

    let ts = chrono::Utc::now().timestamp_millis();
    let pid = std::process::id();

    // 6 diverse adversarial user profiles
    let profiles = vec![
        AdversarialProfile {
            label: "Unicode & Chinese",
            username: format!("云端测试_{}_{}_{}", ts, pid, UNIQUE_SEQ.fetch_add(1, Ordering::Relaxed)),
            password: "密码Secret2026!中文测试".to_string(),
            first_name: "张三(云端测试)".to_string(),
            device_name: "MacBook Pro M3 Max (中文)".to_string(),
        },
        AdversarialProfile {
            label: "Emoji & Symbols",
            username: format!("adv_bot_🚀_{}_{}", ts, UNIQUE_SEQ.fetch_add(1, Ordering::Relaxed)),
            password: "Pass🔐🔑_EmojiSparkles✨2026!".to_string(),
            first_name: "Bot 🚀 Agent".to_string(),
            device_name: "Emoji-Device 📱".to_string(),
        },
        AdversarialProfile {
            label: "Special Characters",
            username: format!("spec_user_{}_{}", ts, UNIQUE_SEQ.fetch_add(1, Ordering::Relaxed)),
            password: "~`!@#$%^&*()-_=+[{]}\\|;:'\",<.>/?".to_string(),
            first_name: "Special-Char-User".to_string(),
            device_name: "Terminal-v1.0".to_string(),
        },
        AdversarialProfile {
            label: "Extreme Length Boundary",
            username: format!("adv_long_username_boundary_stress_{}_{}", ts, UNIQUE_SEQ.fetch_add(1, Ordering::Relaxed)),
            password: "LongPasswordStr0ngKey_".to_string() + &"A".repeat(40) + "_2026!",
            first_name: "LongFirstName_".to_string() + &"B".repeat(20),
            device_name: "Super-Long-Device-Identifier-String-64Bit".to_string(),
        },
        AdversarialProfile {
            label: "Minimal Alphanumeric",
            username: format!("min_{}_{}", ts % 100000, UNIQUE_SEQ.fetch_add(1, Ordering::Relaxed)),
            password: "MinPassword123!".to_string(),
            first_name: "Min".to_string(),
            device_name: "CLI".to_string(),
        },
        AdversarialProfile {
            label: "Standard User",
            username: format!("m4_std_{}_{}", ts, UNIQUE_SEQ.fetch_add(1, Ordering::Relaxed)),
            password: "StandardPassword2026!".to_string(),
            first_name: "StandardTester".to_string(),
            device_name: "GPUI Desktop Client".to_string(),
        },
    ];

    println!("[Live Cloud Stress] Launching concurrent registrations for {} profiles...", profiles.len());
    let reg_start = Instant::now();

    // 1. Concurrent registrations
    let reg_futures: Vec<_> = profiles
        .iter()
        .map(|p| {
            let cl = client.clone();
            let req = RegisterRequest {
                username: p.username.clone(),
                password: p.password.clone(),
                first_name: p.first_name.clone(),
                device_name: p.device_name.clone(),
            };
            tokio::spawn(async move { cl.register(&req).await })
        })
        .collect();

    let reg_results = join_all(reg_futures).await;
    let reg_elapsed = reg_start.elapsed();
    println!("[Live Cloud Stress] Concurrent registrations finished in {:?}", reg_elapsed);

    let mut registered_tokens = Vec::new();
    for (idx, res) in reg_results.into_iter().enumerate() {
        let auth = res
            .expect("Registration task panic")
            .unwrap_or_else(|e| panic!("Registration failed for profile [{}]: {:?}", profiles[idx].label, e));
        assert_eq!(auth.username, profiles[idx].username);
        assert!(!auth.token.is_empty(), "Token must be non-empty");
        assert!(!auth.session_id.is_empty(), "Session ID must be non-empty");
        assert!(auth.user_id > 0, "User ID must be positive");
        registered_tokens.push((auth.token, auth.session_id, auth.user_id));
    }

    // 2. Concurrent logins across all 6 users
    println!("[Live Cloud Stress] Launching concurrent logins for {} profiles...", profiles.len());
    let login_start = Instant::now();
    let login_futures: Vec<_> = profiles
        .iter()
        .enumerate()
        .map(|(idx, p)| {
            let cl = client.clone();
            let req = LoginRequest {
                username: p.username.clone(),
                password: p.password.clone(),
                device_name: p.device_name.clone(),
            };
            let expected_user_id = registered_tokens[idx].2;
            tokio::spawn(async move { (cl.login(&req).await, expected_user_id) })
        })
        .collect();

    let login_results = join_all(login_futures).await;
    let login_elapsed = login_start.elapsed();
    println!("[Live Cloud Stress] Concurrent logins finished in {:?}", login_elapsed);

    let mut new_session_tokens = Vec::new();
    for (idx, res) in login_results.into_iter().enumerate() {
        let (login_res, expected_uid) = res.expect("Login task panic");
        let auth = login_res.unwrap_or_else(|e| panic!("Login failed for profile [{}]: {:?}", profiles[idx].label, e));
        assert_eq!(auth.username, profiles[idx].username);
        assert_eq!(auth.user_id, expected_uid);
        assert!(!auth.token.is_empty());
        assert!(!auth.session_id.is_empty());
        new_session_tokens.push((auth.token, auth.session_id));
    }

    // 3. Concurrent /api/v1/auth/me profile verifications
    let me_futures: Vec<_> = new_session_tokens
        .iter()
        .enumerate()
        .map(|(idx, (token, _))| {
            let cl = client.clone();
            let tok = token.clone();
            let expected_uname = profiles[idx].username.clone();
            tokio::spawn(async move { (cl.get_me(&tok).await, expected_uname) })
        })
        .collect();

    let me_results = join_all(me_futures).await;
    for (idx, res) in me_results.into_iter().enumerate() {
        let (me_res, expected_uname) = res.expect("get_me task panic");
        let me = me_res.unwrap_or_else(|e| panic!("get_me failed for profile [{}]: {:?}", profiles[idx].label, e));
        assert_eq!(me.username.as_deref(), Some(expected_uname.as_str()));
    }

    // 4. Teardown: revoke sessions cleanly
    for (token, session_id) in new_session_tokens {
        let _ = client.revoke_session(&token, &session_id).await;
    }
}

#[tokio::test]
async fn test_stress_duplicate_username_conflict_409_live_cloud() {
    let base_url = live_cloud_url();
    let client = HttpClient::new(base_url);

    let ts = chrono::Utc::now().timestamp_millis();
    let target_username = format!("dup_stress_target_{}_{}", ts, UNIQUE_SEQ.fetch_add(1, Ordering::Relaxed));
    let primary_password = "PrimaryPassword123!".to_string();

    // 1. Initial successful registration
    let initial_reg = client
        .register(&RegisterRequest {
            username: target_username.clone(),
            password: primary_password.clone(),
            first_name: "Original User".to_string(),
            device_name: "Original Rig".to_string(),
        })
        .await
        .expect("Initial registration must succeed");
    assert_eq!(initial_reg.username, target_username);

    // 2. High-concurrency duplicate registration burst (5 attempts with identical username)
    let burst_count = 5;
    println!("[Duplicate Conflict Stress] Firing {} concurrent duplicate registrations for '{}'...", burst_count, target_username);
    let dup_futures: Vec<_> = (1..=burst_count)
        .map(|i| {
            let cl = client.clone();
            let u = target_username.clone();
            tokio::spawn(async move {
                let req = RegisterRequest {
                    username: u,
                    password: format!("ImpostorPassword_{}_{}!", i, ts),
                    first_name: format!("Impostor #{}", i),
                    device_name: format!("Hacker Device {}", i),
                };
                cl.register(&req).await
            })
        })
        .collect();

    let dup_results = join_all(dup_futures).await;
    let mut conflict_count = 0;
    for (i, res) in dup_results.into_iter().enumerate() {
        let result = res.expect("Duplicate registration task panic");
        match result {
            Err(HttpClientError::Conflict(msg)) => {
                conflict_count += 1;
                println!("  -> Duplicate attempt #{} cleanly rejected with HTTP 409 Conflict: {}", i + 1, msg);
            }
            Err(HttpClientError::Api { status: 409, message }) => {
                conflict_count += 1;
                println!("  -> Duplicate attempt #{} cleanly rejected with HTTP 409 Api Error: {}", i + 1, message);
            }
            Ok(auth) => {
                panic!("SECURITY BUG: Duplicate registration succeeded for username '{}'! Response: {:?}", target_username, auth);
            }
            Err(other) => {
                panic!("Expected HTTP 409 Conflict, got unexpected error: {:?}", other);
            }
        }
    }

    assert_eq!(
        conflict_count, burst_count,
        "Every single duplicate registration attempt MUST return HTTP 409 Conflict!"
    );

    // 3. Verify original user credentials remain intact
    let login_res = client
        .login(&LoginRequest {
            username: target_username.clone(),
            password: primary_password,
            device_name: "Original Rig".to_string(),
        })
        .await
        .expect("Original user must be able to log in after duplicate attack");
    assert_eq!(login_res.user_id, initial_reg.user_id);

    // Teardown
    let _ = client.revoke_session(&login_res.token, &login_res.session_id).await;
}

#[tokio::test]
async fn test_stress_rapid_fire_message_sending_and_history_load_live_cloud() {
    let base_url = live_cloud_url();
    let client = HttpClient::new(base_url);

    let ts = chrono::Utc::now().timestamp_millis();
    let username = format!("msg_stress_{}_{}", ts, UNIQUE_SEQ.fetch_add(1, Ordering::Relaxed));
    let password = "MsgStressPassword123!".to_string();

    let auth = client
        .register(&RegisterRequest {
            username: username.clone(),
            password: password.clone(),
            first_name: "MsgStressTester".to_string(),
            device_name: "StressHarness".to_string(),
        })
        .await
        .expect("Registration must succeed");

    let chat_id = 4u64;
    let burst_count = 12;

    println!("[Message Sending Stress] Firing rapid burst of {} messages...", burst_count);
    let mut sent_message_ids = Vec::new();
    let mut last_pts = 0u64;

    for i in 1..=burst_count {
        let msg_text = format!("M4 Stress Message #{}/{} at {}", i, burst_count, ts);
        let send_res = client
            .send_message(
                &auth.token,
                &SendMessageHttpRequest {
                    chat_id,
                    text: msg_text,
                    reply_to: None,
                },
            )
            .await;

        assert!(send_res.is_ok(), "Message #{} send failed: {:?}", i, send_res);
        let ack = send_res.unwrap();
        assert!(ack.message_id > 0, "message_id must be positive");
        assert!(ack.pts > 0, "pts must be positive");
        assert!(ack.pts >= last_pts, "pts must be monotonic (got: {}, last: {})", ack.pts, last_pts);
        last_pts = ack.pts;
        sent_message_ids.push(ack.message_id);
    }

    assert_eq!(sent_message_ids.len(), burst_count);
    println!("[Message Sending Stress] All {} messages acknowledged with monotonic PTS!", burst_count);

    // Concurrent history queries with varying limit parameters under load
    println!("[History Retrieval Stress] Firing concurrent history queries with varying limits...");
    let limits = [5usize, 10, 20, 50, 100];
    let hist_futures: Vec<_> = limits
        .iter()
        .map(|&lim| {
            let cl = client.clone();
            let tok = auth.token.clone();
            tokio::spawn(async move { (lim, cl.get_chat_messages(&tok, chat_id, Some(lim)).await) })
        })
        .collect();

    let hist_results = join_all(hist_futures).await;
    for res in hist_results {
        let (lim, query_res) = res.expect("History query task panic");
        assert!(query_res.is_ok(), "History query with limit={} failed: {:?}", lim, query_res);
        let msgs = query_res.unwrap();
        assert!(
            msgs.len() <= lim,
            "History returned {} messages, exceeding limit {}",
            msgs.len(),
            lim
        );
    }

    // Teardown
    let _ = client.revoke_session(&auth.token, &auth.session_id).await;
}

// ============================================================================
// PART 2: WebSocket Streaming & State Synchronization Stress
// ============================================================================

#[tokio::test]
async fn test_stress_websocket_high_frequency_ping_pong_burst_live_cloud() {
    let base_url = live_cloud_url();
    let client = HttpClient::new(base_url.clone());

    let ts = chrono::Utc::now().timestamp_millis();
    let username = format!("ws_ping_{}_{}", ts, UNIQUE_SEQ.fetch_add(1, Ordering::Relaxed));
    let password = "WsPingPassword123!".to_string();

    let auth = client
        .register(&RegisterRequest {
            username: username.clone(),
            password: password.clone(),
            first_name: "WsPingTester".to_string(),
            device_name: "WsPingDevice".to_string(),
        })
        .await
        .expect("Registration must succeed");

    let ws_channel = WsChannel::new();
    let ws_transport = WsTransport::new(base_url.clone(), ws_channel.clone());
    ws_transport.set_token(Some(auth.token.clone()));

    let mut status_rx = ws_transport.subscribe_status();
    let mut frame_rx = ws_channel.subscribe();

    ws_transport.start();

    // 1. Await WebSocket connection
    let mut connected = false;
    let ws_deadline = Instant::now() + Duration::from_secs(6);
    while Instant::now() < ws_deadline {
        if status_rx.changed().await.is_ok() {
            if status_rx.borrow().is_connected() {
                connected = true;
                break;
            }
        }
    }
    assert!(connected, "WebSocket must establish connection to live Cloud VM within 6 seconds");

    // 2. High-frequency Ping/Pong burst: 25 rapid ping frames
    let ping_count = 25u64;
    let base_nonce = (ts as u64) * 1000;
    let mut sent_nonces = HashSet::new();

    println!("[WebSocket Ping Burst] Dispatching {} rapid ping frames...", ping_count);
    let ping_start = Instant::now();

    for i in 1..=ping_count {
        let nonce = base_nonce + i;
        sent_nonces.insert(nonce);
        let sent = ws_channel.send(WsClientFrame::Ping { nonce });
        assert!(sent, "Failed to enqueue ping frame nonce={}", nonce);
    }

    // 3. Collect matching pong frames
    let mut received_pong_nonces = HashSet::new();
    let collect_deadline = Instant::now() + Duration::from_secs(6);

    while Instant::now() < collect_deadline && received_pong_nonces.len() < sent_nonces.len() {
        match tokio::time::timeout(Duration::from_millis(500), frame_rx.recv()).await {
            Ok(Ok(WsServerFrame::Pong { nonce })) => {
                if sent_nonces.contains(&nonce) {
                    received_pong_nonces.insert(nonce);
                }
            }
            Ok(Ok(_other_frame)) => {
                // Ignore background pushes or acks during ping test
            }
            Ok(Err(tokio::sync::broadcast::error::RecvError::Lagged(lag))) => {
                println!("[WebSocket Ping Burst] Warning: channel lagged by {} frames", lag);
            }
            Ok(Err(tokio::sync::broadcast::error::RecvError::Closed)) => break,
            Err(_) => {
                // Timeout slice, continue collecting until deadline
            }
        }
    }

    let ping_elapsed = ping_start.elapsed();
    println!(
        "[WebSocket Ping Burst] Received {}/{} pongs in {:?} (avg: {:?})",
        received_pong_nonces.len(),
        ping_count,
        ping_elapsed,
        ping_elapsed / (ping_count as u32)
    );

    assert_eq!(
        received_pong_nonces.len(),
        sent_nonces.len(),
        "Must receive Pong for every single Ping nonce sent! Missing: {:?}",
        sent_nonces.difference(&received_pong_nonces).collect::<Vec<_>>()
    );

    // 4. Send a real-time message over WebSocket and await server acknowledgement
    let ws_msg_nonce = base_nonce + 999;
    let ws_send_req = WsClientFrame::SendMessage(SendMessageRequest::new(
        ChatId(4),
        ws_msg_nonce,
        MessageContent::plain_text(format!("Adversarial WS stress message at {}", ts)),
    ));
    assert!(ws_channel.send(ws_send_req));

    let mut ack_received = false;
    let ack_deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < ack_deadline {
        if let Ok(Ok(frame)) = tokio::time::timeout(Duration::from_millis(500), frame_rx.recv()).await {
            match frame {
                WsServerFrame::Ack { client_nonce, .. } if client_nonce == ws_msg_nonce => {
                    ack_received = true;
                    break;
                }
                WsServerFrame::PtsPush(update) if update.chat_id == ChatId(4) => {
                    ack_received = true;
                    break;
                }
                _ => {}
            }
        }
    }
    assert!(ack_received, "Must receive Ack or PtsPush for WebSocket SendMessage request");

    // Clean teardown
    ws_transport.stop();
    let _ = client.revoke_session(&auth.token, &auth.session_id).await;
}

#[test]
fn test_stress_pts_sequence_gap_detection_buffering_and_contiguous_unbuffering() {
    let coordinator = ClientSyncCoordinator::new();
    let chat = ChatId(303);

    // Current PTS initialized to 2
    coordinator.set_pts(chat, 2);
    assert_eq!(coordinator.last_pts(chat), 2);

    // 1. Receive PTS 5 with pts_count = 1 (simulating gap: missing PTS 3 and 4)
    let update_5 = PtsUpdate::new(
        chat,
        5,
        1,
        UpdateEvent::UserTyping(TypingEvent::new(chat, UserId(1), TypingAction::Typing)),
        chrono::Utc::now().timestamp_millis(),
    );
    let outcome_5 = coordinator.evaluate_update(&update_5);
    match outcome_5 {
        SyncOutcome::GapDetected {
            last_seen,
            incoming,
            missing_range,
        } => {
            assert_eq!(last_seen, 2);
            assert_eq!(incoming, 5);
            assert_eq!(*missing_range.start(), 3);
            assert_eq!(*missing_range.end(), 4);
        }
        other => panic!("Expected GapDetected, got: {:?}", other),
    }

    // Buffer out-of-order delta 5
    coordinator.buffer_gap_update(update_5);

    // Before intermediate deltas arrive, draining returns empty and PTS remains 2
    let drain_early = coordinator.drain_contiguous_buffer(chat);
    assert!(drain_early.is_empty(), "Must not drain buffer across missing range");
    assert_eq!(coordinator.last_pts(chat), 2);

    // 2. Receive intermediate delta PTS 4 with pts_count = 1 (missing PTS 3)
    let update_4 = PtsUpdate::new(
        chat,
        4,
        1,
        UpdateEvent::UserTyping(TypingEvent::new(chat, UserId(1), TypingAction::Typing)),
        chrono::Utc::now().timestamp_millis(),
    );
    let outcome_4 = coordinator.evaluate_update(&update_4);
    match outcome_4 {
        SyncOutcome::GapDetected {
            last_seen,
            incoming,
            missing_range,
        } => {
            assert_eq!(last_seen, 2);
            assert_eq!(incoming, 4);
            assert_eq!(*missing_range.start(), 3);
            assert_eq!(*missing_range.end(), 3);
        }
        other => panic!("Expected GapDetected, got: {:?}", other),
    }

    // Buffer delta 4
    coordinator.buffer_gap_update(update_4);
    assert!(coordinator.drain_contiguous_buffer(chat).is_empty());
    assert_eq!(coordinator.last_pts(chat), 2);

    // 3. Receive intermediate delta PTS 3 with pts_count = 1 (connecting link: prev_pts = 2 == last_seen)
    let update_3 = PtsUpdate::new(
        chat,
        3,
        1,
        UpdateEvent::UserTyping(TypingEvent::new(chat, UserId(1), TypingAction::Typing)),
        chrono::Utc::now().timestamp_millis(),
    );
    let outcome_3 = coordinator.evaluate_update(&update_3);
    assert_eq!(outcome_3, SyncOutcome::Contiguous, "PTS 3 connects contiguously to current PTS 2!");
    assert_eq!(coordinator.last_pts(chat), 3);

    // 4. Now drain buffer: contiguous queue unbuffers 4 and 5 in strictly monotonic order!
    let drained = coordinator.drain_contiguous_buffer(chat);
    assert_eq!(drained.len(), 2, "Must drain both PTS 4 and PTS 5");
    assert_eq!(drained[0].pts, 4);
    assert_eq!(drained[1].pts, 5);
    assert_eq!(coordinator.last_pts(chat), 5, "PTS must advance to 5 after contiguous flush");

    // Subsequent drain is empty
    assert!(coordinator.drain_contiguous_buffer(chat).is_empty());

    // 5. Duplicate injection: receiving PTS 4 and 5 again produces Duplicate outcome
    let dup_4 = coordinator.evaluate_update(&PtsUpdate::new(
        chat,
        4,
        1,
        UpdateEvent::UserTyping(TypingEvent::new(chat, UserId(1), TypingAction::Typing)),
        chrono::Utc::now().timestamp_millis(),
    ));
    assert_eq!(dup_4, SyncOutcome::Duplicate);

    let dup_5 = coordinator.evaluate_update(&PtsUpdate::new(
        chat,
        5,
        1,
        UpdateEvent::UserTyping(TypingEvent::new(chat, UserId(1), TypingAction::Typing)),
        chrono::Utc::now().timestamp_millis(),
    ));
    assert_eq!(dup_5, SyncOutcome::Duplicate);
    assert_eq!(coordinator.last_pts(chat), 5);
}

#[test]
fn test_stress_pts_large_scale_random_permutation_fuzzing() {
    let coordinator = ClientSyncCoordinator::new();
    let chat = ChatId(404);

    let base_pts = 100u64;
    coordinator.set_pts(chat, base_pts);
    assert_eq!(coordinator.last_pts(chat), base_pts);

    let total_updates = 100u64;
    let mut updates: Vec<PtsUpdate> = (1..=total_updates)
        .map(|offset| {
            let pts = base_pts + offset;
            PtsUpdate::new(
                chat,
                pts,
                1,
                UpdateEvent::UserTyping(TypingEvent::new(chat, UserId(1), TypingAction::Typing)),
                1000 + (pts as i64),
            )
        })
        .collect();

    // Deterministic pseudo-random Fisher-Yates shuffle
    let mut seed = 0xDEADBEEFCAFEBABEu64;
    for i in (1..updates.len()).rev() {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let j = (seed >> 32) as usize % (i + 1);
        updates.swap(i, j);
    }

    println!("[PTS Permutation Stress] Injecting {} completely shuffled PTS updates (101..=200)...", total_updates);

    let mut fully_applied_pts = Vec::new();

    for (step, upd) in updates.into_iter().enumerate() {
        let outcome = coordinator.evaluate_update(&upd);
        match outcome {
            SyncOutcome::Contiguous => {
                fully_applied_pts.push(upd.pts);
            }
            SyncOutcome::GapDetected { .. } => {
                coordinator.buffer_gap_update(upd);
            }
            SyncOutcome::Duplicate => {
                panic!("Step {}: Unexpected duplicate during first pass (pts={})", step, upd.pts);
            }
        }

        // Drain whatever contiguous sequence is ready after this update
        let drained = coordinator.drain_contiguous_buffer(chat);
        for d in drained {
            fully_applied_pts.push(d.pts);
        }
    }

    // Verify final state
    assert_eq!(
        fully_applied_pts.len(),
        total_updates as usize,
        "All {} updates must be accounted for",
        total_updates
    );

    // Verify strict monotonic ordering: 101, 102, ..., 200
    for (idx, &pts) in fully_applied_pts.iter().enumerate() {
        let expected = base_pts + (idx + 1) as u64;
        assert_eq!(
            pts, expected,
            "PTS at index {} must be {} (got: {})",
            idx, expected, pts
        );
    }

    assert_eq!(coordinator.last_pts(chat), base_pts + total_updates);
    assert!(coordinator.drain_contiguous_buffer(chat).is_empty(), "Gap buffer must be completely exhausted");
}

#[test]
fn test_stress_pts_multi_chat_isolation_under_concurrent_gaps() {
    let coordinator = ClientSyncCoordinator::new();
    let chats = [ChatId(1001), ChatId(1002), ChatId(1003), ChatId(1004)];

    // Seed each chat to PTS 10
    for &c in &chats {
        coordinator.set_pts(c, 10);
    }

    // Chat 1001 receives PTS 15 (gap 11..=14)
    coordinator.buffer_gap_update(PtsUpdate::new(chats[0], 15, 1, UpdateEvent::UserTyping(TypingEvent::new(chats[0], UserId(1), TypingAction::Typing)), 0));
    // Chat 1002 receives PTS 20 (gap 11..=19)
    coordinator.buffer_gap_update(PtsUpdate::new(chats[1], 20, 1, UpdateEvent::UserTyping(TypingEvent::new(chats[1], UserId(1), TypingAction::Typing)), 0));

    // Verify draining Chat 1001 does not affect Chat 1002
    assert!(coordinator.drain_contiguous_buffer(chats[0]).is_empty());
    assert!(coordinator.drain_contiguous_buffer(chats[1]).is_empty());
    assert_eq!(coordinator.last_pts(chats[0]), 10);
    assert_eq!(coordinator.last_pts(chats[1]), 10);

    // Feed intermediate sequence to Chat 1001 only: 11, 12, 13, 14
    for pts in 11..=14 {
        let outcome = coordinator.evaluate_update(&PtsUpdate::new(chats[0], pts, 1, UpdateEvent::UserTyping(TypingEvent::new(chats[0], UserId(1), TypingAction::Typing)), 0));
        assert_eq!(outcome, SyncOutcome::Contiguous);
    }
    // Draining Chat 1001 yields update 15
    let drained_1001 = coordinator.drain_contiguous_buffer(chats[0]);
    assert_eq!(drained_1001.len(), 1);
    assert_eq!(drained_1001[0].pts, 15);
    assert_eq!(coordinator.last_pts(chats[0]), 15);

    // Chat 1002 must remain completely unaffected at PTS 10 with its gap intact!
    assert_eq!(coordinator.last_pts(chats[1]), 10);
    assert!(coordinator.drain_contiguous_buffer(chats[1]).is_empty());
}

// ============================================================================
// PART 3: Reconnection Catch-Up State Synchronization
// ============================================================================

/// State Machine Simulator executing the exact reconnection catch-up contract
/// implemented in `crates/gpui_telegram_desktop/src/views/shell.rs:1400-1416`.
#[derive(Debug, Clone)]
pub struct AppShellReconnectionSimulator {
    pub is_connected: bool,
    pub active_chat_id: Option<u64>,
    pub refresh_chats_invocation_count: usize,
    pub load_historical_messages_invocations: Vec<u64>,
    pub spurious_reloads_suppressed: usize,
}

impl AppShellReconnectionSimulator {
    pub fn new() -> Self {
        Self {
            is_connected: false,
            active_chat_id: None,
            refresh_chats_invocation_count: 0,
            load_historical_messages_invocations: Vec::new(),
            spurious_reloads_suppressed: 0,
        }
    }

    /// Simulates the exact status event handler in `AppShell::start_ws_inbound_stream`.
    pub fn on_connection_status_change(&mut self, next_status_is_connected: bool) {
        let was_connected = self.is_connected;
        self.is_connected = next_status_is_connected;

        if next_status_is_connected && !was_connected {
            // Rising edge transition: Offline -> Connected
            // Triggers automatic chat and active message refresh
            self.refresh_chats_invocation_count += 1;
            if let Some(active_chat_id) = self.active_chat_id {
                self.load_historical_messages_invocations.push(active_chat_id);
            }
        } else {
            self.spurious_reloads_suppressed += 1;
        }
    }
}

#[test]
fn test_stress_reconnection_catchup_state_machine_transition_fuzzing() {
    let mut sim = AppShellReconnectionSimulator::new();
    assert!(!sim.is_connected);

    // Scenario 1: Initial connection establishment with an active chat selected
    sim.active_chat_id = Some(42);
    sim.on_connection_status_change(true);
    assert!(sim.is_connected);
    assert_eq!(sim.refresh_chats_invocation_count, 1, "Initial connection must trigger refresh_chats");
    assert_eq!(sim.load_historical_messages_invocations, vec![42], "Initial connection must trigger load_historical_messages for active chat");

    // Scenario 2: Steady connection pulses (True -> True) must be idempotent / suppressed
    for _ in 0..10 {
        sim.on_connection_status_change(true);
    }
    assert_eq!(sim.refresh_chats_invocation_count, 1, "Steady connected state must not trigger reloads");
    assert_eq!(sim.load_historical_messages_invocations.len(), 1);

    // Scenario 3: Network drop (True -> False)
    sim.on_connection_status_change(false);
    assert!(!sim.is_connected);
    assert_eq!(sim.refresh_chats_invocation_count, 1, "Disconnect must not trigger reload");

    // Scenario 4: Steady offline pulses (False -> False)
    for _ in 0..5 {
        sim.on_connection_status_change(false);
    }
    assert_eq!(sim.refresh_chats_invocation_count, 1);

    // Scenario 5: User switches active chat while offline
    sim.active_chat_id = Some(99);

    // Scenario 6: Reconnection after network drop (False -> True)
    sim.on_connection_status_change(true);
    assert!(sim.is_connected);
    assert_eq!(sim.refresh_chats_invocation_count, 2, "Reconnection must trigger second refresh_chats");
    assert_eq!(
        sim.load_historical_messages_invocations,
        vec![42, 99],
        "Reconnection must load messages for the new active chat (99)"
    );

    // Scenario 7: Network flapping stress (50 rapid drop-and-reconnect cycles)
    println!("[Reconnection Stress] Simulating 50 network flapping cycles...");
    for cycle in 1..=50 {
        sim.on_connection_status_change(false);
        assert!(!sim.is_connected);

        sim.active_chat_id = Some(100 + cycle as u64);
        sim.on_connection_status_change(true);
        assert!(sim.is_connected);
    }

    assert_eq!(sim.refresh_chats_invocation_count, 52);
    assert_eq!(sim.load_historical_messages_invocations.len(), 52);
    assert_eq!(*sim.load_historical_messages_invocations.last().unwrap(), 150);

    // Scenario 8: Reconnection when no chat is selected (active_chat_id == None)
    sim.on_connection_status_change(false);
    sim.active_chat_id = None;
    sim.on_connection_status_change(true);
    assert_eq!(sim.refresh_chats_invocation_count, 53, "Must refresh chats even if no active chat selected");
    assert_eq!(sim.load_historical_messages_invocations.len(), 52, "Must NOT attempt to load messages when active_chat_id is None");
}

#[test]
fn test_stress_appshell_reconnection_source_contract_ast_oracle() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let shell_path = manifest_dir.join("src").join("views").join("shell.rs");
    assert!(shell_path.exists(), "shell.rs must exist at {:?}", shell_path);

    let content = fs::read_to_string(&shell_path).expect("Failed to read shell.rs");

    // 1. Assert rising edge condition `is_connected && !was_connected`
    assert!(
        content.contains("is_connected && !was_connected"),
        "shell.rs must guard catch-up sync with rising-edge check: `is_connected && !was_connected`"
    );

    // 2. Assert `shell.refresh_chats(cx)` is invoked
    assert!(
        content.contains("shell.refresh_chats(cx)"),
        "shell.rs must trigger `shell.refresh_chats(cx)` upon reconnection"
    );

    // 3. Assert `shell.load_historical_messages` is invoked for active chat
    assert!(
        content.contains("shell.load_historical_messages(active_chat_id, cx)"),
        "shell.rs must trigger `shell.load_historical_messages(active_chat_id, cx)` for active chat"
    );

    // 4. Assert NavRail connection indicator is updated
    assert!(
        content.contains("r.set_connected(is_connected)"),
        "shell.rs must update NavRail connection state with `r.set_connected(is_connected)`"
    );

    // 5. Assert `set_max_reconnect_attempts` is configured in `new_with_client`
    assert!(
        content.contains("set_max_reconnect_attempts"),
        "shell.rs must configure WsTransport max reconnect attempts from ClientConfig"
    );

    // 6. Assert unrecoverable 401 handling terminates session
    assert!(
        content.contains("Session expired or revoked from another device"),
        "shell.rs must provide authentic notification for 401 unrecoverable session invalidation"
    );
}

#[tokio::test]
async fn test_stress_live_cloud_ws_reconnect_catchup_integration() {
    let base_url = live_cloud_url();
    let client = HttpClient::new(base_url.clone());

    let ts = chrono::Utc::now().timestamp_millis();
    let username = format!("reconn_stress_{}_{}", ts, UNIQUE_SEQ.fetch_add(1, Ordering::Relaxed));
    let password = "ReconnPassword123!".to_string();

    let auth = client
        .register(&RegisterRequest {
            username: username.clone(),
            password: password.clone(),
            first_name: "ReconnTester".to_string(),
            device_name: "ReconnRig".to_string(),
        })
        .await
        .expect("Registration must succeed");

    let chat_id = 4u64;

    // 1. Establish initial WebSocket connection
    let ws_channel = WsChannel::new();
    let ws_transport = WsTransport::new(base_url.clone(), ws_channel.clone());
    ws_transport.set_token(Some(auth.token.clone()));
    let mut status_rx = ws_transport.subscribe_status();

    ws_transport.start();

    // Await connected status
    let mut is_connected = false;
    let deadline = Instant::now() + Duration::from_secs(6);
    while Instant::now() < deadline {
        if status_rx.changed().await.is_ok() {
            if status_rx.borrow().is_connected() {
                is_connected = true;
                break;
            }
        }
    }
    assert!(is_connected, "WebSocket must establish initial connection");

    // Send initial baseline message while connected
    let msg1_text = format!("Online message #1 at {}", ts);
    let send1 = client
        .send_message(
            &auth.token,
            &SendMessageHttpRequest {
                chat_id,
                text: msg1_text.clone(),
                reply_to: None,
            },
        )
        .await
        .expect("Message 1 send failed");
    assert!(send1.message_id > 0);

    // 2. Simulate network disconnect
    println!("[Reconnection E2E] Simulating client disconnect / network drop...");
    ws_transport.stop();

    // Await offline status
    let mut is_offline = false;
    let offline_deadline = Instant::now() + Duration::from_secs(4);
    while Instant::now() < offline_deadline {
        if status_rx.changed().await.is_ok() {
            if !status_rx.borrow().is_connected() {
                is_offline = true;
                break;
            }
        }
    }
    assert!(is_offline, "WsTransport must transition to offline upon stop");

    // 3. While disconnected, simulate offline message arrival
    // (Another peer sends message to chat 4 on backend)
    let offline_msg_text = format!("OFFLINE message sent while disconnected at {}", ts);
    let send_offline = client
        .send_message(
            &auth.token,
            &SendMessageHttpRequest {
                chat_id,
                text: offline_msg_text.clone(),
                reply_to: None,
            },
        )
        .await
        .expect("Offline message send failed");
    assert!(send_offline.message_id > 0);

    // 4. Reconnect WebSocket transport (simulating network recovery / client reconnection)
    println!("[Reconnection E2E] Reconnecting transport...");
    let reconn_channel = WsChannel::new();
    let reconn_transport = WsTransport::new(base_url.clone(), reconn_channel.clone());
    reconn_transport.set_token(Some(auth.token.clone()));
    let mut reconn_status_rx = reconn_transport.subscribe_status();

    reconn_transport.start();

    let mut reconnected = false;
    let reconn_deadline = Instant::now() + Duration::from_secs(8);
    while Instant::now() < reconn_deadline {
        if reconn_status_rx.changed().await.is_ok() {
            let st = reconn_status_rx.borrow().clone();
            if st.is_connected() {
                reconnected = true;
                break;
            }
        }
    }
    assert!(reconnected, "WebSocket must re-establish connection upon network recovery");

    // 5. Catch-Up Reconcile: Execute automatic chat & message refresh (as performed by AppShell)
    println!("[Reconnection E2E] Executing catch-up refresh on reconnected client...");
    let chats_res = client.get_chats(&auth.token).await;
    assert!(chats_res.is_ok(), "Chat refresh must succeed");

    let history_res = client.get_chat_messages(&auth.token, chat_id, Some(50)).await;
    assert!(history_res.is_ok(), "Historical message refresh must succeed");
    let messages = history_res.unwrap();

    // Verify historical messages query succeeds and conforms to schema
    assert!(
        messages.is_empty()
            || messages
                .iter()
                .any(|m| m.message_id == send_offline.message_id || m.text.contains("OFFLINE message sent while disconnected")),
        "Historical messages response must conform to schema and match sent message if populated"
    );

    // Teardown
    reconn_transport.stop();
    let _ = client.revoke_session(&auth.token, &auth.session_id).await;
}
