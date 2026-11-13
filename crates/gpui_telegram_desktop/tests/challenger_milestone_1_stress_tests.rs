//! Empirical Challenger 1 Adversarial Stress Suite for Milestone 1
//! Targets:
//! 1. ChatSidebar resize clamping [240.0, 360.0] under extreme values
//! 2. CanonicalComposer expansion formula & overflow scrolling threshold
//! 3. BubbleRounding permutation matrix across all 8 combinations of (is_outgoing, attached_prev, attached_next)
//! 4. Attachment delta boundary at exactly 899s, 900s, 901s

use gpui_telegram_desktop::views::*;
use gpui_telegram_desktop::views::stage::cards::models::{
    DeliveryStatus, MessageContent, MessageDirection, MessageItemModel, ServiceType,
};
use gpui_telegram_desktop::views::stage::cards::text_card::{
    BubbleRounding, K_ATTACH_MESSAGE_TO_PREVIOUS_SECONDS_DELTA,
};
use gpui_telegram_desktop::views::conversation_stage::compute_attachment;

// ============================================================================
// 1. ChatSidebar Resize Clamping Empirical Stress Suite
// ============================================================================

#[test]
fn test_adversarial_sidebar_resize_clamping_extremes() {
    assert_eq!(chat_sidebar::DEFAULT_WIDTH, 288.0);
    assert_eq!(chat_sidebar::MIN_WIDTH, 240.0);
    assert_eq!(chat_sidebar::MAX_WIDTH, 360.0);

    let mut sidebar = ChatSidebar::new();
    assert_eq!(sidebar.width, 288.0);

    // 1. Extreme negative values -> must clamp to MIN_WIDTH (240.0)
    let negative_values = [-1e12, -1_000_000.0, -1000.0, -100.0, -1.0, -0.0001, -0.0];
    for &val in &negative_values {
        sidebar.set_width(val);
        assert_eq!(
            sidebar.width, 240.0,
            "Negative width {} failed to clamp to 240.0",
            val
        );
    }

    // 2. Exact zero -> must clamp to MIN_WIDTH (240.0)
    sidebar.set_width(0.0);
    assert_eq!(sidebar.width, 240.0, "Zero width failed to clamp to 240.0");

    // 3. Sub-240 values -> must clamp to MIN_WIDTH (240.0)
    let sub_min_values = [0.001, 50.0, 100.0, 180.0, 239.0, 239.99, 239.9999];
    for &val in &sub_min_values {
        sidebar.set_width(val);
        assert_eq!(
            sidebar.width, 240.0,
            "Sub-240 width {} failed to clamp to 240.0",
            val
        );
    }

    // 4. Exact boundary MIN_WIDTH (240.0)
    sidebar.set_width(240.0);
    assert_eq!(sidebar.width, 240.0);

    // 5. In-range values -> must be retained exactly
    let valid_widths = [240.001, 250.0, 288.0, 300.5, 320.0, 359.99, 360.0];
    for &val in &valid_widths {
        sidebar.set_width(val);
        assert_eq!(
            sidebar.width, val,
            "Valid width {} was unexpectedly modified",
            val
        );
    }

    // 6. Above MAX_WIDTH and 1000+ widths -> must clamp to MAX_WIDTH (360.0)
    let super_max_values = [360.001, 361.0, 400.0, 800.0, 1000.0, 1000.5, 10_000.0, 1e9, f32::MAX];
    for &val in &super_max_values {
        sidebar.set_width(val);
        assert_eq!(
            sidebar.width, 360.0,
            "Excessive width {} failed to clamp to 360.0",
            val
        );
    }

    // 7. Reset width restores default (288.0)
    sidebar.set_width(1000.0);
    assert_eq!(sidebar.width, 360.0);
    sidebar.reset_width();
    assert_eq!(sidebar.width, 288.0, "reset_width() failed to restore 288.0");

    sidebar.set_width(-500.0);
    assert_eq!(sidebar.width, 240.0);
    sidebar.reset_width();
    assert_eq!(sidebar.width, 288.0, "reset_width() failed to restore 288.0");
}

// ============================================================================
// 2. CanonicalComposer Auto-Expansion Formula Empirical Stress Suite
// ============================================================================

#[test]
fn test_adversarial_composer_expansion_formula_and_clamping() {
    assert_eq!(canonical_composer::MIN_COMPOSER_HEIGHT, 44.0);
    assert_eq!(canonical_composer::MAX_COMPOSER_HEIGHT, 192.0);
    assert_eq!(canonical_composer::LINE_HEIGHT, 20.0);
    assert_eq!(canonical_composer::ACTION_BTN_WIDTH, 44.0);
    assert_eq!(canonical_composer::ACTION_BTN_HEIGHT, 46.0);

    let mut composer = CanonicalComposer::new();

    // 1. Single line: empty text -> 44.0px
    composer.set_text("");
    assert_eq!(composer.calculate_line_count(), 1);
    assert_eq!(f32::from(composer.calculate_content_height()), 44.0);
    assert!(!composer.is_overflow_scroll_active());

    // Single line: text without newlines -> 44.0px
    composer.set_text("Single line message without newlines");
    assert_eq!(composer.calculate_line_count(), 1);
    assert_eq!(f32::from(composer.calculate_content_height()), 44.0);
    assert!(!composer.is_overflow_scroll_active());

    // 2. 2 lines: exactly 1 newline separating 2 lines -> 44.0 + 1 * 20.0 = 64.0px
    composer.set_text("Line 1\nLine 2");
    assert_eq!(composer.calculate_line_count(), 2);
    assert_eq!(f32::from(composer.calculate_content_height()), 64.0);
    assert!(!composer.is_overflow_scroll_active());

    // 3. Multi-line progression up to 8 lines
    for n in 1..=8 {
        let lines: Vec<String> = (1..=n).map(|i| format!("Line {}", i)).collect();
        composer.set_text(lines.join("\n"));
        assert_eq!(composer.calculate_line_count(), n);
        let expected_height = 44.0 + ((n - 1) as f32) * 20.0;
        assert_eq!(
            f32::from(composer.calculate_content_height()),
            expected_height,
            "Height mismatch at {} lines",
            n
        );
        assert!(
            !composer.is_overflow_scroll_active(),
            "Scroll should NOT be active at {} lines",
            n
        );
    }

    // Exactly 8 lines -> 184.0px (44.0 + 7 * 20.0 = 184.0)
    let eight_lines: Vec<String> = (1..=8).map(|i| format!("Row {}", i)).collect();
    composer.set_text(eight_lines.join("\n"));
    assert_eq!(composer.calculate_line_count(), 8);
    assert_eq!(f32::from(composer.calculate_content_height()), 184.0);
    assert!(!composer.is_overflow_scroll_active());

    // 4. 9 lines -> 44.0 + 8 * 20.0 = 204.0px -> must clamp strictly to 192.0px
    let nine_lines: Vec<String> = (1..=9).map(|i| format!("Row {}", i)).collect();
    composer.set_text(nine_lines.join("\n"));
    assert_eq!(composer.calculate_line_count(), 9);
    assert_eq!(f32::from(composer.calculate_content_height()), 192.0);
    assert!(composer.is_overflow_scroll_active(), "Scroll MUST be active at 9 lines");

    // 5. Extreme 100 lines -> must clamp strictly to 192.0px
    let hundred_lines: Vec<String> = (1..=100).map(|i| format!("Row {}", i)).collect();
    composer.set_text(hundred_lines.join("\n"));
    assert_eq!(composer.calculate_line_count(), 100);
    assert_eq!(f32::from(composer.calculate_content_height()), 192.0);
    assert!(composer.is_overflow_scroll_active());

    // 6. Extreme 10,000 lines -> must clamp strictly to 192.0px without overflow/crash
    let huge_lines: Vec<String> = (1..=10_000).map(|i| format!("{}", i)).collect();
    composer.set_text(huge_lines.join("\n"));
    assert_eq!(composer.calculate_line_count(), 10_000);
    assert_eq!(f32::from(composer.calculate_content_height()), 192.0);
    assert!(composer.is_overflow_scroll_active());

    // 7. Wrapping stress: single line of 96 characters without newlines
    // char_count / 48 = 2 wrapped -> total lines = 3 -> 44 + 2 * 20 = 84px
    let wrapped_text = "A".repeat(96);
    composer.set_text(wrapped_text);
    assert_eq!(composer.calculate_line_count(), 3);
    assert_eq!(f32::from(composer.calculate_content_height()), 84.0);
}

// ============================================================================
// 3. BubbleRounding Permutation Matrix (All 8 Combinations)
// ============================================================================

#[test]
fn test_adversarial_bubble_rounding_permutation_matrix_for_message() {
    // Test BubbleRounding::for_message against strict 16px/4px specifications
    // Permutation Matrix: 2 * 2 * 2 = 8 combinations
    struct ExpectedRounding {
        is_outgoing: bool,
        attached_prev: bool,
        attached_next: bool,
        top_left: f32,
        top_right: f32,
        bottom_left: f32,
        bottom_right: f32,
        has_tail: bool,
    }

    let matrix = [
        // 1. Incoming Isolated
        ExpectedRounding {
            is_outgoing: false,
            attached_prev: false,
            attached_next: false,
            top_left: 16.0,
            top_right: 16.0,
            bottom_left: 16.0,
            bottom_right: 16.0,
            has_tail: true,
        },
        // 2. Incoming Terminal (attached to previous)
        ExpectedRounding {
            is_outgoing: false,
            attached_prev: true,
            attached_next: false,
            top_left: 4.0,
            top_right: 16.0,
            bottom_left: 16.0,
            bottom_right: 16.0,
            has_tail: true,
        },
        // 3. Incoming Initial (attached to next)
        ExpectedRounding {
            is_outgoing: false,
            attached_prev: false,
            attached_next: true,
            top_left: 16.0,
            top_right: 16.0,
            bottom_left: 4.0,
            bottom_right: 16.0,
            has_tail: false,
        },
        // 4. Incoming Middle (attached to both)
        ExpectedRounding {
            is_outgoing: false,
            attached_prev: true,
            attached_next: true,
            top_left: 4.0,
            top_right: 16.0,
            bottom_left: 4.0,
            bottom_right: 16.0,
            has_tail: false,
        },
        // 5. Outgoing Isolated
        ExpectedRounding {
            is_outgoing: true,
            attached_prev: false,
            attached_next: false,
            top_left: 16.0,
            top_right: 16.0,
            bottom_left: 16.0,
            bottom_right: 16.0,
            has_tail: true,
        },
        // 6. Outgoing Terminal (attached to previous)
        ExpectedRounding {
            is_outgoing: true,
            attached_prev: true,
            attached_next: false,
            top_left: 16.0,
            top_right: 4.0,
            bottom_left: 16.0,
            bottom_right: 16.0,
            has_tail: true,
        },
        // 7. Outgoing Initial (attached to next)
        ExpectedRounding {
            is_outgoing: true,
            attached_prev: false,
            attached_next: true,
            top_left: 16.0,
            top_right: 16.0,
            bottom_left: 16.0,
            bottom_right: 4.0,
            has_tail: false,
        },
        // 8. Outgoing Middle (attached to both)
        ExpectedRounding {
            is_outgoing: true,
            attached_prev: true,
            attached_next: true,
            top_left: 16.0,
            top_right: 4.0,
            bottom_left: 16.0,
            bottom_right: 4.0,
            has_tail: false,
        },
    ];

    for exp in &matrix {
        let r = BubbleRounding::for_message(exp.is_outgoing, exp.attached_prev, exp.attached_next);
        assert_eq!(r.top_left, exp.top_left, "TL mismatch for ({}, {}, {})", exp.is_outgoing, exp.attached_prev, exp.attached_next);
        assert_eq!(r.top_right, exp.top_right, "TR mismatch for ({}, {}, {})", exp.is_outgoing, exp.attached_prev, exp.attached_next);
        assert_eq!(r.bottom_left, exp.bottom_left, "BL mismatch for ({}, {}, {})", exp.is_outgoing, exp.attached_prev, exp.attached_next);
        assert_eq!(r.bottom_right, exp.bottom_right, "BR mismatch for ({}, {}, {})", exp.is_outgoing, exp.attached_prev, exp.attached_next);
        assert_eq!(r.has_tail, exp.has_tail, "Tail mismatch for ({}, {}, {})", exp.is_outgoing, exp.attached_prev, exp.attached_next);
    }
}

#[test]
fn test_adversarial_bubble_rounding_compute_divergence_oracle() {
    // Empirical examination of BubbleRounding::compute()
    // Finding: BubbleRounding::compute sets bottom corner to 2.0 (rather than 16.0)
    // when attached_next == false to accommodate the tail notch.
    let isolated_out = BubbleRounding::compute(true, false, false);
    assert_eq!(isolated_out.top_left, 16.0);
    assert_eq!(isolated_out.top_right, 16.0);
    assert_eq!(isolated_out.bottom_left, 16.0);
    // Notice: bottom_right is 2.0 in compute(), whereas for_message() produces 16.0
    assert_eq!(isolated_out.bottom_right, 2.0, "compute() applies 2.0px tail corner radius");
    assert!(isolated_out.has_tail);

    let isolated_in = BubbleRounding::compute(false, false, false);
    assert_eq!(isolated_in.top_left, 16.0);
    assert_eq!(isolated_in.top_right, 16.0);
    // Notice: bottom_left is 2.0 in compute(), whereas for_message() produces 16.0
    assert_eq!(isolated_in.bottom_left, 2.0, "compute() applies 2.0px tail corner radius");
    assert_eq!(isolated_in.bottom_right, 16.0);
    assert!(isolated_in.has_tail);

    // When attached_next == true, both compute() and for_message() yield 4.0px
    let middle_out = BubbleRounding::compute(true, true, true);
    assert_eq!(middle_out.top_right, 4.0);
    assert_eq!(middle_out.bottom_right, 4.0);
    assert!(!middle_out.has_tail);

    let middle_in = BubbleRounding::compute(false, true, true);
    assert_eq!(middle_in.top_left, 4.0);
    assert_eq!(middle_in.bottom_left, 4.0);
    assert!(!middle_in.has_tail);
}

// ============================================================================
// 4. Attachment Delta Boundary Empirical Stress Suite (899s, 900s, 901s)
// ============================================================================

fn make_msg(id: u64, sender_id: u64, timestamp: &str, is_service: bool) -> MessageItemModel {
    MessageItemModel {
        id,
        sender_id,
        sender_name: "Tester".to_string(),
        avatar_initials: "T".to_string(),
        avatar_color: 0x2AABEE,
        direction: MessageDirection::Incoming,
        timestamp: timestamp.to_string(),
        delivery_status: DeliveryStatus::Read,
        reply_to: None,
        reactions: vec![],
        content: if is_service {
            MessageContent::Service {
                text: "Pinned a message".to_string(),
                service_type: ServiceType::PinnedMessage,
            }
        } else {
            MessageContent::Text {
                text: format!("Message {}", id),
                is_edited: false,
            }
        },
        bot_keyboard: None,
        is_pinned: false,
    }
}

#[test]
fn test_adversarial_attachment_delta_exact_boundaries() {
    assert_eq!(K_ATTACH_MESSAGE_TO_PREVIOUS_SECONDS_DELTA, 900);

    // Case 1: Exactly 899 seconds delta (Unix timestamp format) -> MUST BE ATTACHED
    let msgs_899 = vec![
        make_msg(1, 42, "1000", false),
        make_msg(2, 42, "1899", false),
    ];
    let (p0_899, n0_899) = compute_attachment(&msgs_899, 0);
    let (p1_899, n1_899) = compute_attachment(&msgs_899, 1);
    assert!(!p0_899, "First message has no previous");
    assert!(n0_899, "Message 0 MUST attach to next at delta = 899s");
    assert!(p1_899, "Message 1 MUST attach to previous at delta = 899s");
    assert!(!n1_899, "Message 1 has no next");

    // Case 2: Exactly 900 seconds delta (boundary equality) -> MUST BE ATTACHED
    let msgs_900 = vec![
        make_msg(1, 42, "1000", false),
        make_msg(2, 42, "1900", false),
    ];
    let (p0_900, n0_900) = compute_attachment(&msgs_900, 0);
    let (p1_900, n1_900) = compute_attachment(&msgs_900, 1);
    assert!(!p0_900);
    assert!(n0_900, "Message 0 MUST attach to next at boundary delta = 900s");
    assert!(p1_900, "Message 1 MUST attach to previous at boundary delta = 900s");
    assert!(!n1_900);

    // Case 3: Exactly 901 seconds delta (one second over boundary) -> MUST BE UNATTACHED
    let msgs_901 = vec![
        make_msg(1, 42, "1000", false),
        make_msg(2, 42, "1901", false),
    ];
    let (p0_901, n0_901) = compute_attachment(&msgs_901, 0);
    let (p1_901, n1_901) = compute_attachment(&msgs_901, 1);
    assert!(!p0_901);
    assert!(!n0_901, "Message 0 MUST NOT attach to next at delta = 901s (> 900s)");
    assert!(!p1_901, "Message 1 MUST NOT attach to previous at delta = 901s (> 900s)");
    assert!(!n1_901);
}

#[test]
fn test_adversarial_attachment_delta_hhmmss_boundaries() {
    // Test HH:MM:SS format
    // Base: 10:00:00 (36000s)
    // 899s: 10:14:59 (36899s)
    // 900s: 10:15:00 (36900s)
    // 901s: 10:15:01 (36901s)
    let msgs = vec![
        make_msg(1, 100, "10:00:00", false),
        make_msg(2, 100, "10:14:59", false), // delta = 899s -> attached to msg 1
        make_msg(3, 100, "10:30:00", false), // delta = 901s from msg 2 -> unattached to msg 2
        make_msg(4, 100, "10:45:00", false), // delta = 900s from msg 3 -> attached to msg 3
        make_msg(5, 100, "11:00:01", false), // delta = 901s from msg 4 -> unattached to msg 4
    ];

    // Msg 1 (10:00:00): attached_next should be true (to Msg 2)
    let (p1, n1) = compute_attachment(&msgs, 0);
    assert!(!p1);
    assert!(n1, "Msg 1 attached_next to Msg 2 (899s) must be true");

    // Msg 2 (10:14:59): attached_prev true (from Msg 1), attached_next false (to Msg 3: 901s)
    let (p2, n2) = compute_attachment(&msgs, 1);
    assert!(p2, "Msg 2 attached_prev from Msg 1 (899s) must be true");
    assert!(!n2, "Msg 2 attached_next to Msg 3 (901s) must be false");

    // Msg 3 (10:30:00): attached_prev false (from Msg 2: 901s), attached_next true (to Msg 4: 900s)
    let (p3, n3) = compute_attachment(&msgs, 2);
    assert!(!p3, "Msg 3 attached_prev from Msg 2 (901s) must be false");
    assert!(n3, "Msg 3 attached_next to Msg 4 (900s) must be true");

    // Msg 4 (10:45:00): attached_prev true (from Msg 3: 900s), attached_next false (to Msg 5: 901s)
    let (p4, n4) = compute_attachment(&msgs, 3);
    assert!(p4, "Msg 4 attached_prev from Msg 3 (900s) must be true");
    assert!(!n4, "Msg 4 attached_next to Msg 5 (901s) must be false");

    // Msg 5 (11:00:01): attached_prev false (from Msg 4: 901s), attached_next false
    let (p5, n5) = compute_attachment(&msgs, 4);
    assert!(!p5, "Msg 5 attached_prev from Msg 4 (901s) must be false");
    assert!(!n5);
}

#[test]
fn test_adversarial_attachment_disruptions() {
    // 1. Different sender: delta = 10s <= 900s -> MUST NOT ATTACH
    let msgs_diff_sender = vec![
        make_msg(1, 10, "10:00:00", false),
        make_msg(2, 20, "10:00:10", false),
    ];
    let (p0, n0) = compute_attachment(&msgs_diff_sender, 0);
    let (p1, n1) = compute_attachment(&msgs_diff_sender, 1);
    assert!(!p0);
    assert!(!n0, "Different sender cannot attach");
    assert!(!p1, "Different sender cannot attach");
    assert!(!n1);

    // 2. Different direction: same sender_id, delta = 10s, but different direction -> MUST NOT ATTACH
    let mut msg_out = make_msg(2, 10, "10:00:10", false);
    msg_out.direction = MessageDirection::Outgoing;
    let msgs_diff_direction = vec![
        make_msg(1, 10, "10:00:00", false), // Incoming
        msg_out,                            // Outgoing
    ];
    let (_, n0_dir) = compute_attachment(&msgs_diff_direction, 0);
    let (p1_dir, _) = compute_attachment(&msgs_diff_direction, 1);
    assert!(!n0_dir, "Different direction cannot attach");
    assert!(!p1_dir, "Different direction cannot attach");

    // 3. Intervening Service message: same sender, delta <= 900s -> MUST NOT ATTACH
    let msgs_service_break = vec![
        make_msg(1, 10, "10:00:00", false),
        make_msg(2, 10, "10:00:05", true), // Service
        make_msg(3, 10, "10:00:10", false),
    ];
    let (_, n0_srv) = compute_attachment(&msgs_service_break, 0);
    let (p1_srv, n1_srv) = compute_attachment(&msgs_service_break, 1);
    let (p2_srv, _) = compute_attachment(&msgs_service_break, 2);
    assert!(!n0_srv, "Cannot attach to next if next is Service");
    assert!(!p1_srv, "Service cannot attach to previous");
    assert!(!n1_srv, "Service cannot attach to next");
    assert!(!p2_srv, "Cannot attach to previous if previous is Service");

    // 4. Out of bounds safety
    let (p_oob, n_oob) = compute_attachment(&msgs_service_break, 999);
    assert!(!p_oob);
    assert!(!n_oob);
}
