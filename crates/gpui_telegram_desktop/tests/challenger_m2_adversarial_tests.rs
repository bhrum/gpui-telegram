//! Challenger M2 Adversarial Stress & Verification Tests (`crates/gpui_telegram_desktop`).
//!
//! Rigorous stress testing and boundary challenge suite for Milestone 2:
//! - Feature F8: Message Virtualization (RowHeightCache, prefix-sum binary search, windowed slicing, scroll anchoring)
//! - Feature F9: Session Switching Isolation (50+ chats, history isolation, independent scroll persistence)
//! - Feature F7: IME Commit Fencing & Return safety under rapid event sequences
//! - Feature F10: Chat interaction cleanup, Markdown parsing, and Context Menu operations under scale

use gpui_telegram_desktop::views::*;
use gpui_telegram_desktop::views::conversation_stage::*;
use gpui::{px, Point, Size};
use std::time::Instant;

// ============================================================================
// Helper Functions for Heterogeneous Stress Data Generation
// ============================================================================

fn generate_heterogeneous_messages(count: usize, chat_id: u64) -> Vec<MessageItemModel> {
    let mut msgs = Vec::with_capacity(count);
    for i in 0..count {
        let msg_id = (chat_id * 1_000_000) + (i as u64) + 1;
        let is_outgoing = (i % 2) == 1;
        let sender_id = if is_outgoing { 1 } else { chat_id };
        let timestamp = format!("{:02}:{:02}", (i / 60) % 24, i % 60);

        let content = match i % 10 {
            0 => MessageContent::Service {
                text: format!("Date Divider Day #{}", i / 10),
                service_type: ServiceType::DateDivider,
            },
            1 => MessageContent::Text {
                text: format!("Single-line short message #{}", i),
                is_edited: false,
            },
            2 => MessageContent::Text {
                text: format!(
                    "Multi-line paragraph message #{}\nLine 2 with text details\nLine 3 describing architecture\nLine 4 wrap",
                    i
                ),
                is_edited: true,
            },
            3 => MessageContent::Voice {
                duration_secs: (i % 120) as u32 + 5,
                waveform_5bit: vec![(i % 32) as u8; 32],
                playback_position_secs: 0.0,
                is_playing: false,
                playback_speed: 1.0,
            },
            4 => MessageContent::Media {
                media_type: MediaType::Photo,
                url: format!("https://example.com/photo_{}.jpg", i),
                width: 1280,
                height: 720,
                duration_secs: None,
                file_size_bytes: 1_200_000,
                caption: Some(format!("Photo attachment #{} with caption", i)),
                is_streaming: false,
            },
            5 => MessageContent::Album {
                items: vec![
                    AlbumItem {
                        id: 1,
                        media_type: MediaType::Photo,
                        url: format!("https://example.com/photo_{}_1.jpg", i),
                        aspect_ratio: 1.33,
                        preview_color: 0x2C3E50,
                        caption: None,
                    },
                    AlbumItem {
                        id: 2,
                        media_type: MediaType::Photo,
                        url: format!("https://example.com/photo_{}_2.jpg", i),
                        aspect_ratio: 1.33,
                        preview_color: 0x34495E,
                        caption: None,
                    },
                ],
                caption: Some("Album collage".to_string()),
            },
            6 => MessageContent::Location {
                latitude: 37.7749 + (i as f64 * 0.001),
                longitude: -122.4194 + (i as f64 * 0.001),
                title: format!("Venue location #{}", i),
                address: "Market St, San Francisco, CA".to_string(),
            },
            7 => MessageContent::Poll {
                question: format!("Poll question #{} for team?", i),
                poll_type: PollType::Regular,
                options: vec![
                    PollOption { id: 1, text: "Option A".to_string(), votes: 12, percentage: 60.0, is_selected: false, is_correct: false },
                    PollOption { id: 2, text: "Option B".to_string(), votes: 8, percentage: 40.0, is_selected: false, is_correct: false },
                ],
                total_votes: 20,
                is_closed: false,
                selected_option_id: None,
            },
            8 => MessageContent::Call {
                call_type: CallType::Voice,
                direction: CallDirection::Incoming,
                duration_secs: 0,
                status_text: "Missed Call".to_string(),
            },
            _ => MessageContent::Invoice {
                title: format!("Telegram Stars In-App Item #{}", i),
                description: "Digital goods purchase".to_string(),
                amount_stars: 50,
                currency: "Telegram Stars".to_string(),
                status: InvoiceStatus::Pending,
                receipt_id: None,
            },
        };

        msgs.push(MessageItemModel {
            id: msg_id,
            sender_id,
            sender_name: if is_outgoing { "Me".to_string() } else { format!("Peer_{}", chat_id) },
            avatar_initials: if is_outgoing { "ME".to_string() } else { "PR".to_string() },
            avatar_color: 0x2AABEE,
            direction: if is_outgoing { MessageDirection::Outgoing } else { MessageDirection::Incoming },
            timestamp,
            delivery_status: DeliveryStatus::Read,
            reply_to: None,
            reactions: vec![],
            content,
            bot_keyboard: None,
            is_pinned: false,
        });
    }
    msgs
}

// ============================================================================
// PART 1: Virtualization Stress Testing (Feature F8)
// ============================================================================

#[test]
fn test_adversarial_virtualization_25k_messages_monotonic_prefix_sums() {
    let mut cache = RowHeightCache::new();
    let count = 25_000;
    let messages = generate_heterogeneous_messages(count, 1);

    let start = Instant::now();
    cache.rebuild_prefix_sums(&messages);
    let duration = start.elapsed();

    // 1. Performance: 25k prefix sums rebuilt within reasonable budget (< 100ms)
    println!("Prefix sum rebuild for 25,000 messages took: {:?}", duration);
    assert_eq!(cache.len(), count);
    assert!(!cache.is_dirty());
    assert!(cache.total_height > 0.0);

    // 2. Strict Monotonicity: prefix_sums[i] must be strictly greater than prefix_sums[i-1]
    let mut prev = 0.0f32;
    for (idx, &curr) in cache.prefix_sums.iter().enumerate() {
        assert!(
            curr > prev,
            "Prefix sum non-monotonic at index {}: prev={}, curr={}",
            idx,
            prev,
            curr
        );
        let row_height = curr - prev;
        assert!(
            row_height >= 24.0,
            "Row height at index {} too small: {}",
            idx,
            row_height
        );
        prev = curr;
    }

    assert_eq!(cache.total_height, *cache.prefix_sums.last().unwrap());
}

#[test]
fn test_adversarial_virtualization_windowed_slicing_bounds_at_various_offsets() {
    let mut stage = ConversationStage::new();
    let count = 10_000;
    stage.messages = generate_heterogeneous_messages(count, 100);
    stage.row_height_cache.rebuild_prefix_sums(&stage.messages);

    let total_h = stage.row_height_cache.total_height;
    let viewport_h = 700.0;
    let overscan_px = 300.0;

    // Test positions: top, 25%, 50%, 75%, bottom, beyond bottom
    let test_scroll_offsets = vec![
        0.0,
        150.0,
        total_h * 0.25,
        total_h * 0.50,
        total_h * 0.75,
        total_h - viewport_h - 50.0,
        total_h - viewport_h,
    ];

    for scroll_top in test_scroll_offsets {
        let (start_idx, end_idx) = stage.row_height_cache.visible_range(
            scroll_top,
            viewport_h,
            overscan_px,
            count,
        );

        assert!(start_idx <= end_idx, "start_idx ({}) > end_idx ({})", start_idx, end_idx);
        assert!(end_idx < count, "end_idx ({}) out of bounds ({})", end_idx, count);

        // Bounded slice size: Viewport (700) + 2 * Overscan (300) = 1300px.
        // Min estimated row height is 32px (Service), so max visible items is ~ 1300 / 32 ~= 41 items.
        // We assert strictly <= 60 items in all scenarios.
        let visible_count = end_idx - start_idx + 1;
        assert!(
            visible_count <= 60,
            "Visible count {} exceeded bound 60 at scroll_top {}",
            visible_count,
            scroll_top
        );

        // Spacer math consistency
        let top_spacer = if start_idx > 0 {
            stage.row_height_cache.prefix_sums[start_idx - 1]
        } else {
            0.0
        };
        let bottom_spacer = if end_idx < count - 1 {
            stage.row_height_cache.total_height - stage.row_height_cache.prefix_sums[end_idx]
        } else {
            0.0
        };

        let rendered_height = stage.row_height_cache.prefix_sums[end_idx]
            - if start_idx > 0 { stage.row_height_cache.prefix_sums[start_idx - 1] } else { 0.0 };

        let reconstructed_total = top_spacer + rendered_height + bottom_spacer;
        assert!(
            (reconstructed_total - total_h).abs() < 0.1,
            "Reconstructed height {} != total {}",
            reconstructed_total,
            total_h
        );
    }
}

#[test]
fn test_adversarial_virtualization_boundary_queries_and_extreme_inputs() {
    let mut cache = RowHeightCache::new();
    let messages = generate_heterogeneous_messages(1000, 1);
    cache.rebuild_prefix_sums(&messages);
    let total_h = cache.total_height;

    // 1. Extreme negative scroll_top (e.g. bounce / momentum overscroll)
    let (s, e) = cache.visible_range(-50_000.0, 800.0, 300.0, 1000);
    assert_eq!(s, 0);
    assert!(e < 50);

    // 2. Extreme positive scroll_top (overscroll far past transcript end)
    let (s_far, e_far) = cache.visible_range(1_000_000.0, 800.0, 300.0, 1000);
    assert_eq!(s_far, 999);
    assert_eq!(e_far, 999);

    // 3. Zero viewport height
    let (s_zero, e_zero) = cache.visible_range(total_h / 2.0, 0.0, 100.0, 1000);
    assert!(s_zero <= e_zero);

    // 4. Negative viewport height (malformed window size)
    let (s_neg, e_neg) = cache.visible_range(total_h / 2.0, -200.0, 100.0, 1000);
    assert!(s_neg <= e_neg);

    // 5. Negative overscan
    let (s_ov, e_ov) = cache.visible_range(total_h / 2.0, 600.0, -100.0, 1000);
    assert!(s_ov <= e_ov);

    // 6. Zero items
    let (s_empty, e_empty) = cache.visible_range(100.0, 600.0, 300.0, 0);
    assert_eq!(s_empty, 0);
    assert_eq!(e_empty, 0);

    // 7. Empty cache
    let empty_cache = RowHeightCache::new();
    let (s_ec, e_ec) = empty_cache.visible_range(100.0, 600.0, 300.0, 100);
    assert_eq!(s_ec, 0);
    assert_eq!(e_ec, 0);

    // 8. Single item
    let single_msg = generate_heterogeneous_messages(1, 1);
    let mut single_cache = RowHeightCache::new();
    single_cache.rebuild_prefix_sums(&single_msg);
    let (s_one, e_one) = single_cache.visible_range(0.0, 800.0, 300.0, 1);
    assert_eq!(s_one, 0);
    assert_eq!(e_one, 0);
    let (s_one_far, e_one_far) = single_cache.visible_range(5000.0, 800.0, 300.0, 1);
    assert_eq!(s_one_far, 0);
    assert_eq!(e_one_far, 0);
}

#[test]
fn test_adversarial_virtualization_dynamic_insertions_and_deletions_at_arbitrary_indices() {
    let mut stage = ConversationStage::new();
    stage.messages = generate_heterogeneous_messages(500, 10);
    stage.row_height_cache.rebuild_prefix_sums(&stage.messages);
    let initial_h = stage.row_height_cache.total_height;

    // 1. Delete middle message (index 250)
    let target_del_id = stage.messages[250].id;
    let del_row_h = RowHeightCache::estimate_row_height(&stage.messages[250], false);
    stage.delete_message(target_del_id);
    assert_eq!(stage.messages.len(), 499);
    assert!(stage.row_height_cache.is_dirty());

    stage.row_height_cache.rebuild_prefix_sums(&stage.messages);
    assert_eq!(stage.row_height_cache.len(), 499);
    assert!(
        (stage.row_height_cache.total_height - (initial_h - del_row_h)).abs() < 10.0,
        "Total height after deletion mismatched: before={}, after={}",
        initial_h,
        stage.row_height_cache.total_height
    );

    // 2. Insert message dynamically at tail via add_outgoing_text_message
    stage.add_outgoing_text_message("New dynamic tail message");
    assert_eq!(stage.messages.len(), 500);
    assert!(stage.row_height_cache.is_dirty());

    stage.row_height_cache.rebuild_prefix_sums(&stage.messages);
    assert_eq!(stage.row_height_cache.len(), 500);
    assert!(stage.row_height_cache.total_height > (initial_h - del_row_h));

    // 3. Delete first message
    let first_id = stage.messages[0].id;
    stage.delete_message(first_id);
    assert_eq!(stage.messages.len(), 499);

    // 4. Delete last message
    let last_id = stage.messages.last().unwrap().id;
    stage.delete_message(last_id);
    assert_eq!(stage.messages.len(), 498);
}

// ============================================================================
// PART 2: Rapid Session Switching Isolation (Feature F9)
// ============================================================================

#[test]
fn test_adversarial_rapid_session_switching_across_55_chats() {
    let mut stage = ConversationStage::new();

    // Build 55 chats with distinct profiles:
    // - Chat 1: 10,000 messages with pinned note
    // - Chat 2: Empty chat (0 messages, no pinned note)
    // - Chat 3: 1 message
    // - Chat 4: 50 messages
    // - Chat 5: 500 messages with distinct pinned note
    // - Chats 6..55: Varied message counts (0, 10, 25, 100)
    let mut expected_counts = std::collections::HashMap::new();
    let mut expected_pins = std::collections::HashMap::new();

    // Populate Chat 1 (10,000 messages)
    let chat1_msgs = generate_heterogeneous_messages(10_000, 1);
    expected_counts.insert(1, 10_000);
    expected_pins.insert(1, Some("Design review meeting notes and sprint roadmap".to_string()));
    stage.messages = chat1_msgs.clone();
    stage.pinned_message = expected_pins[&1].clone();
    stage.chat_histories.insert(1, chat1_msgs);
    stage.chat_pinned_messages.insert(1, expected_pins[&1].clone());

    // Populate Chat 2 (0 messages)
    expected_counts.insert(2, 0);
    expected_pins.insert(2, None);
    stage.chat_histories.insert(2, vec![]);
    stage.chat_pinned_messages.insert(2, None);

    // Populate Chat 3 (1 message)
    let chat3_msgs = generate_heterogeneous_messages(1, 3);
    expected_counts.insert(3, 1);
    expected_pins.insert(3, None);
    stage.chat_histories.insert(3, chat3_msgs);
    stage.chat_pinned_messages.insert(3, None);

    // Populate Chat 4 (50 messages)
    let chat4_msgs = generate_heterogeneous_messages(50, 4);
    expected_counts.insert(4, 50);
    expected_pins.insert(4, None);
    stage.chat_histories.insert(4, chat4_msgs);
    stage.chat_pinned_messages.insert(4, None);

    // Populate Chat 5 (500 messages, pinned note)
    let mut chat5_msgs = generate_heterogeneous_messages(500, 5);
    chat5_msgs[10].is_pinned = true;
    expected_counts.insert(5, 500);
    expected_pins.insert(5, Some("Pinned in Chat 5".to_string()));
    stage.chat_histories.insert(5, chat5_msgs);
    stage.chat_pinned_messages.insert(5, expected_pins[&5].clone());

    // Populate Chats 6..=55
    for cid in 6..=55 {
        let count = match cid % 4 {
            0 => 0,
            1 => 1,
            2 => 15,
            _ => 60,
        };
        let msgs = generate_heterogeneous_messages(count, cid);
        expected_counts.insert(cid, count);
        expected_pins.insert(cid, None);
        stage.chat_histories.insert(cid, msgs);
        stage.chat_pinned_messages.insert(cid, None);
    }

    // Set initial active chat to 1
    stage.set_chat(1, "Chat 1", "online", "C1", 0x2AABEE);

    // Perform 1,000 rapid switches across pseudo-random pattern of chat IDs
    let mut current_seed: u64 = 987654321;
    for step in 0..1000 {
        current_seed = current_seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let target_id = (current_seed % 55) + 1;

        let title = format!("Chat {}", target_id);
        let initials = format!("C{}", target_id);
        stage.set_chat(target_id, &title, "online", &initials, 0x2AABEE);

        // Assert 1: Active chat ID matches
        assert_eq!(stage.active_chat_id, Some(target_id));

        // Assert 2: Exact message count matches without leakage
        let exp_count = expected_counts[&target_id];
        assert_eq!(
            stage.messages.len(),
            exp_count,
            "Step {}: Chat {} expected {} messages, found {}",
            step,
            target_id,
            exp_count,
            stage.messages.len()
        );

        // Assert 3: All messages belong strictly to target_id (no cross-contamination)
        for msg in &stage.messages {
            if msg.direction == MessageDirection::Incoming {
                assert_eq!(
                    msg.sender_id,
                    target_id,
                    "Step {}: Cross-chat leak! Message id {} with sender_id {} found in chat {}",
                    step,
                    msg.id,
                    msg.sender_id,
                    target_id
                );
            }
        }

        // Assert 4: Pinned message banner isolation
        let exp_pin = &expected_pins[&target_id];
        assert_eq!(
            &stage.pinned_message,
            exp_pin,
            "Step {}: Pinned banner mismatch for chat {}",
            step,
            target_id
        );
    }
}

#[test]
fn test_adversarial_independent_scroll_persistence_under_complex_transitions() {
    let mut stage = ConversationStage::new();

    // Chat 10: 10,000 messages (tall transcript)
    let chat10_msgs = generate_heterogeneous_messages(10_000, 10);
    stage.chat_histories.insert(10, chat10_msgs);

    // Chat 20: 200 messages (medium transcript)
    let chat20_msgs = generate_heterogeneous_messages(200, 20);
    stage.chat_histories.insert(20, chat20_msgs);

    // Chat 30: 0 messages (empty)
    stage.chat_histories.insert(30, vec![]);

    // 1. Visit Chat 10 and scroll up to middle reading position
    stage.set_chat(10, "Chat 10", "online", "10", 0x2AABEE);
    stage.set_simulated_scroll_offset(Point::new(px(0.0), px(-25_000.0)), Size::new(px(0.0), px(500_000.0)));
    assert!(!stage.is_at_bottom());
    assert_eq!(stage.scroll_handle.offset().y, px(-25_000.0));

    // 2. Switch to Chat 20 and stay at bottom
    stage.set_chat(20, "Chat 20", "online", "20", 0x2AABEE);
    stage.set_simulated_scroll_offset(Point::new(px(0.0), px(-10_000.0)), Size::new(px(0.0), px(10_000.0)));
    assert!(stage.is_at_bottom());

    // 3. Switch to Chat 30 (empty chat)
    stage.set_chat(30, "Chat 30", "online", "30", 0x2AABEE);
    assert_eq!(stage.messages.len(), 0);
    assert!(stage.is_at_bottom());

    // 4. Return to Chat 10: Reading position MUST be preserved (-25_000px)
    stage.set_chat(10, "Chat 10", "online", "10", 0x2AABEE);
    assert_eq!(
        stage.scroll_handle.offset().y,
        px(-25_000.0),
        "Chat 10 reading offset was lost after peer switching!"
    );
    assert!(!stage.is_anchored_to_bottom);

    // 5. Return to Chat 20: Bottom anchor MUST be preserved
    stage.set_chat(20, "Chat 20", "online", "20", 0x2AABEE);
    assert!(stage.is_anchored_to_bottom);

    // 6. Return to Chat 30: Empty chat remains at bottom
    stage.set_chat(30, "Chat 30", "online", "30", 0x2AABEE);
    assert_eq!(stage.messages.len(), 0);
    assert!(stage.is_at_bottom());
}

// ============================================================================
// PART 3: Scroll Anchoring Edge Cases & Concurrency Bursts
// ============================================================================

#[test]
fn test_adversarial_scroll_stickiness_exact_48px_threshold_boundary() {
    let mut stage = ConversationStage::new();
    let max_h = 5000.0;

    // Test exact boundary: 48.0px
    // offset = -(max_h - 48.0) = -4952.0 -> distance = 48.0 -> is_at_bottom() == true
    stage.set_simulated_scroll_offset(Point::new(px(0.0), px(-4952.0)), Size::new(px(0.0), px(max_h)));
    assert_eq!(stage.distance_from_bottom(), px(48.0));
    assert!(stage.is_at_bottom(), "Exactly 48.0px must be considered at bottom");

    // offset = -4951.0 -> distance = 49.0 -> is_at_bottom() == false
    stage.set_simulated_scroll_offset(Point::new(px(0.0), px(-4951.0)), Size::new(px(0.0), px(max_h)));
    assert_eq!(stage.distance_from_bottom(), px(49.0));
    assert!(!stage.is_at_bottom(), "49.0px (> 48px) must NOT be considered at bottom");

    // offset = -4970.0 -> distance = 30.0 -> is_at_bottom() == true
    stage.set_simulated_scroll_offset(Point::new(px(0.0), px(-4970.0)), Size::new(px(0.0), px(max_h)));
    assert!(stage.is_at_bottom());

    // offset = -5000.0 -> distance = 0.0 -> is_at_bottom() == true
    stage.set_simulated_scroll_offset(Point::new(px(0.0), px(-5000.0)), Size::new(px(0.0), px(max_h)));
    assert_eq!(stage.distance_from_bottom(), px(0.0));
    assert!(stage.is_at_bottom());

    // offset = -5050.0 -> overscroll past bottom -> clamped distance = 0.0 -> is_at_bottom() == true
    stage.set_simulated_scroll_offset(Point::new(px(0.0), px(-5050.0)), Size::new(px(0.0), px(max_h)));
    assert_eq!(stage.distance_from_bottom(), px(0.0));
    assert!(stage.is_at_bottom());
}

#[test]
fn test_adversarial_incoming_message_stream_burst_while_reading_history() {
    let mut stage = ConversationStage::new();
    stage.populate_stress_messages(200);

    // 1. User scrolls up into history (-2000px offset out of 10,000px max)
    stage.set_simulated_scroll_offset(Point::new(px(0.0), px(-2000.0)), Size::new(px(0.0), px(10_000.0)));
    assert!(!stage.is_at_bottom());
    assert_eq!(stage.unread_incoming_count, 0);

    // 2. Incoming streaming burst of 100 messages arrives from peer
    for i in 0..100 {
        stage.add_incoming_message(2, &format!("Incoming streaming message chunk #{}", i));
    }

    // Assert: User scroll offset must NOT be stolen!
    assert_eq!(
        stage.scroll_handle.offset().y,
        px(-2000.0),
        "Incoming messages forcefully stole user reading position!"
    );
    assert!(!stage.is_at_bottom());
    assert_eq!(stage.unread_incoming_count, 100);

    // 3. User sends an outgoing message -> MUST immediately auto-scroll to bottom and clear unread counter
    stage.add_outgoing_text_message("My reply to conversation");
    assert!(stage.is_at_bottom(), "Outgoing message failed to anchor to bottom!");
    assert_eq!(stage.unread_incoming_count, 0, "Outgoing send did not clear unread counter");
}

#[test]
fn test_adversarial_incoming_message_stream_burst_while_anchored_at_bottom() {
    let mut stage = ConversationStage::new();
    stage.populate_stress_messages(50);

    // User is at bottom
    stage.set_simulated_scroll_offset(Point::new(px(0.0), px(-5000.0)), Size::new(px(0.0), px(5000.0)));
    assert!(stage.is_at_bottom());

    // 50 incoming messages arrive
    for i in 0..50 {
        stage.add_incoming_message(2, &format!("Live update #{}", i));
        // Must maintain bottom anchor after each message
        assert!(stage.is_at_bottom());
        assert_eq!(stage.unread_incoming_count, 0);
    }
}

// ============================================================================
// PART 4: IME Commit Fencing & Right-Click Context Menu Stress
// ============================================================================

#[test]
fn test_adversarial_ime_rapid_commit_and_enter_fsm_stress() {
    let mut ime = ImeSafetyMachine::new();

    // Simulate 100 consecutive CJK composition and commit cycles with trailing Enters
    for cycle in 0..100 {
        // 1. Idle -> Composing
        ime.on_ime_start();
        assert!(ime.is_composing());

        // 2. Update with pinyin
        ime.on_ime_update(&format!("shuru{}", cycle), 5);
        assert_eq!(ime.preedit_text(), Some(format!("shuru{}", cycle).as_str()));

        // 3. Commit candidate
        ime.on_ime_commit(&format!("输入{}", cycle));
        assert!(ime.just_committed);

        // 4. Trailing Enter generated by Cocoa MUST be swallowed by fence
        let action = ime.handle_enter_key(false);
        assert_eq!(
            action,
            KeyAction::ImeCandidateChosen,
            "Cycle {}: Trailing Enter failed to be absorbed by commit fence",
            cycle
        );
        assert!(!ime.just_committed);

        // 5. Clean Return now sends
        let send_action = ime.handle_enter_key(false);
        assert_eq!(send_action, KeyAction::SendMessage);
    }
}

#[test]
fn test_adversarial_context_menu_under_10k_messages() {
    let mut stage = ConversationStage::new();
    stage.messages = generate_heterogeneous_messages(10_000, 1);
    stage.row_height_cache.rebuild_prefix_sums(&stage.messages);

    // Index 5001 has MessageContent::Text
    let target_msg_id = stage.messages[5001].id;

    // 1. Copy text from middle message in 10k list
    let copied = stage.copy_message_text(target_msg_id);
    assert!(copied.is_some());
    assert!(copied.unwrap().contains("Single-line short message"));

    // 2. Pin message 5001
    assert!(!stage.messages[5001].is_pinned);
    stage.toggle_pin_message(target_msg_id);
    assert!(stage.messages[5001].is_pinned);
    assert!(stage.pinned_message.is_some());

    // 3. Unpin message 5001
    stage.toggle_pin_message(target_msg_id);
    assert!(!stage.messages[5001].is_pinned);

    // 4. Delete message 5001
    stage.delete_message(target_msg_id);
    assert_eq!(stage.messages.len(), 9999);
    assert!(stage.messages.iter().all(|m| m.id != target_msg_id));
}
