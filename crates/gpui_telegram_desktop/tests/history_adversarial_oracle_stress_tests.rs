//! Module 06 (`history`) Adversarial Oracle Stress Tests
//!
//! Blind C++ oracle stress tests targeting upstream edge cases from `Telegram/SourceFiles/history/`:
//! 1. Edit Time Bounds & Permissions Oracle (`history_item.cpp:L3367-L3386`)
//! 2. Delete For Everyone & Bot Rules Oracle (`history_item.cpp:L3490-L3524`)
//! 3. Selection Endpoints Math Oracle (`history_message_selection.h:L21-L31`, `L115-L129`)
//! 4. Admin Log Deletion Coalescing Oracle (`history_admin_log_inner.cpp:L89-L103`, `L1440-L1470`)
//! 5. Virtual List Window Preloading Oracle (`history_widget.cpp:L224-L242`, `history_inner_widget.cpp:L140-L145`)

use gpui_telegram_client::history::admin_log::{
    AdminLogDeleteGroup, K_DELETE_GROUP_TIME_WINDOW_SECS,
};
use gpui_telegram_client::history::item::{
    HistoryItem, MessageFlags, PaidPostType, DEFAULT_EDIT_TIME_LIMIT_SECS,
};
use gpui_telegram_client::history::selection::{
    MessageSelection, MessageSelectionFlatEndpoint, MessageSelectionKind,
};
use gpui_telegram_desktop::views::stage::admin_log::HistoryAdminLogInnerAdminLog;
use gpui_telegram_desktop::views::stage::history_inner_widget::{
    HistoryInnerWidget, MessageItemGeometry, SCROLL_DATE_HIDE_DAY_CROSSING_TIMEOUT_MS,
    SCROLL_DATE_HIDE_TIMEOUT_MS, UNLOAD_HEAVY_PARTS_PAGES,
};
use gpui_telegram_desktop::views::stage::history_widget::{
    HistoryWidget, MESSAGES_PER_PAGE, MESSAGES_PER_PAGE_FIRST, PRELOAD_HEIGHTS_COUNT,
};

/// Deterministic Pseudo-Random Number Generator (Xorshift64Star)
struct XorshiftRng {
    state: u64,
}

impl XorshiftRng {
    fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 { 0x853c49e6748fea9b } else { seed },
        }
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }

    fn next_range(&mut self, min: i64, max: i64) -> i64 {
        assert!(max >= min);
        let span = (max - min + 1) as u64;
        min + (self.next_u64() % span) as i64
    }

    fn next_bool(&mut self) -> bool {
        (self.next_u64() & 1) == 1
    }
}

// ==============================================================================
// 1. Edit Time Bounds & Permissions Oracle (history_item.cpp:L3367-L3386)
// ==============================================================================

#[test]
fn test_oracle_edit_time_bounds_10000_fuzz() {
    let mut rng = XorshiftRng::new(0xDEADBEEF01);
    let base_date = 1_700_000_000i64;
    const TEN_YEARS_SECS: i64 = 10 * 365 * 86400;

    for i in 0..10_000 {
        let msg_id = 10_000 + i as i64;
        let mut item = HistoryItem::new(msg_id, 1001, 2001, base_date, "Test edit message");
        item.flags.insert(MessageFlags::OUTGOING);

        // Randomized delta between -100,000s and +300,000s
        let delta = rng.next_range(-100_000, 300_000);
        let now = base_date + delta;

        // Upstream Oracle: (now - date >= edit_time_limit)
        let is_too_old_expected = delta >= DEFAULT_EDIT_TIME_LIMIT_SECS;
        let is_too_old_actual = item.is_too_old_for_edit(now, DEFAULT_EDIT_TIME_LIMIT_SECS, false);
        assert_eq!(
            is_too_old_actual, is_too_old_expected,
            "Mismatch at iter {} with delta {}: actual={}, expected={}",
            i, delta, is_too_old_actual, is_too_old_expected
        );

        let allows_edit_actual = item.allows_edit(now, DEFAULT_EDIT_TIME_LIMIT_SECS, false);
        let allows_edit_expected = !is_too_old_expected;
        assert_eq!(
            allows_edit_actual, allows_edit_expected,
            "allows_edit mismatch at iter {} with delta {}",
            i, delta
        );

        // Oracle rule: Self chat can edit indefinitely even 10 years in the past
        let ten_years_now = base_date + TEN_YEARS_SECS;
        assert!(
            !item.is_too_old_for_edit(ten_years_now, DEFAULT_EDIT_TIME_LIMIT_SECS, true),
            "Self chat must not be too old for edit even after 10 years"
        );
        assert!(
            item.allows_edit(ten_years_now, DEFAULT_EDIT_TIME_LIMIT_SECS, true),
            "Self chat must allow edit even after 10 years"
        );

        // Oracle rule: Scheduled message can edit indefinitely
        item.is_scheduled = true;
        assert!(
            !item.is_too_old_for_edit(ten_years_now, DEFAULT_EDIT_TIME_LIMIT_SECS, false),
            "Scheduled message must not be too old for edit even after 10 years"
        );
        item.is_scheduled = false;

        // Oracle rule: Rejection of forwarded, via_bot, service, paid, legacy, and editing_media regardless of timestamp
        let fresh_now = base_date + 60; // 1 minute after creation
        assert!(item.allows_edit(fresh_now, DEFAULT_EDIT_TIME_LIMIT_SECS, false));

        // 1. Forwarded
        item.is_forwarded = true;
        assert!(!item.allows_edit(fresh_now, DEFAULT_EDIT_TIME_LIMIT_SECS, false));
        assert!(!item.allows_edit(fresh_now, DEFAULT_EDIT_TIME_LIMIT_SECS, true));
        item.is_forwarded = false;

        // 2. Via Bot
        item.has_via_bot = true;
        assert!(!item.allows_edit(fresh_now, DEFAULT_EDIT_TIME_LIMIT_SECS, false));
        assert!(!item.allows_edit(fresh_now, DEFAULT_EDIT_TIME_LIMIT_SECS, true));
        item.has_via_bot = false;

        // 3. Service
        item.is_service = true;
        assert!(!item.allows_edit(fresh_now, DEFAULT_EDIT_TIME_LIMIT_SECS, false));
        assert!(!item.allows_edit(fresh_now, DEFAULT_EDIT_TIME_LIMIT_SECS, true));
        item.is_service = false;

        // 4. Paid Post
        item.paid_type = PaidPostType::Stars;
        assert!(!item.allows_edit(fresh_now, DEFAULT_EDIT_TIME_LIMIT_SECS, false));
        assert!(!item.allows_edit(fresh_now, DEFAULT_EDIT_TIME_LIMIT_SECS, true));
        item.paid_type = PaidPostType::None;

        // 5. Legacy
        item.flags.insert(MessageFlags::LEGACY);
        assert!(!item.allows_edit(fresh_now, DEFAULT_EDIT_TIME_LIMIT_SECS, false));
        assert!(!item.allows_edit(fresh_now, DEFAULT_EDIT_TIME_LIMIT_SECS, true));
        item.flags.remove(MessageFlags::LEGACY);

        // 6. Currently editing media
        item.is_editing_media = true;
        assert!(!item.allows_edit(fresh_now, DEFAULT_EDIT_TIME_LIMIT_SECS, false));
        assert!(!item.allows_edit(fresh_now, DEFAULT_EDIT_TIME_LIMIT_SECS, true));
        item.is_editing_media = false;
    }
}

// ==============================================================================
// 2. Delete For Everyone & Bot Rules Oracle (history_item.cpp:L3490-L3524)
// ==============================================================================

#[test]
fn test_oracle_delete_for_everyone_and_bot_rules() {
    let mut rng = XorshiftRng::new(0xBADC0FFEE2);
    let base_date = 1_700_000_000i64;

    const REVOKE_LIMIT_GROUP: i64 = 172_800; // 48h
    const REVOKE_LIMIT_PRIVATE: i64 = 86_400; // 24h

    for _ in 0..2_000 {
        let msg_id = 200 + rng.next_range(1, 100_000);
        let mut item = HistoryItem::new(msg_id, 1001, 2001, base_date, "Delete for everyone test");
        item.flags.insert(MessageFlags::OUTGOING);

        // Randomized delta fuzzing for private chat (REVOKE_LIMIT_PRIVATE = 86400)
        let rand_delta_private = rng.next_range(0, 200_000);
        let rand_now_private = base_date + rand_delta_private;
        let expected_private = rand_delta_private < REVOKE_LIMIT_PRIVATE;
        assert_eq!(
            item.can_delete_for_everyone(
                rand_now_private,
                false,
                true,
                false,
                false,
                REVOKE_LIMIT_GROUP,
                REVOKE_LIMIT_PRIVATE,
                true,
            ),
            expected_private
        );

        // Test boundary conditions for User Chat (uses revoke_private_time_limit)
        let exact_valid_private = base_date + REVOKE_LIMIT_PRIVATE - 1;
        let exact_expired_private = base_date + REVOKE_LIMIT_PRIVATE;
        assert!(item.can_delete_for_everyone(
            exact_valid_private,
            false,
            true,
            false,
            false,
            REVOKE_LIMIT_GROUP,
            REVOKE_LIMIT_PRIVATE,
            true,
        ));
        assert!(!item.can_delete_for_everyone(
            exact_expired_private,
            false,
            true,
            false,
            false,
            REVOKE_LIMIT_GROUP,
            REVOKE_LIMIT_PRIVATE,
            true,
        ));

        // Test boundary conditions for Group Chat (uses revoke_time_limit)
        let exact_valid_group = base_date + REVOKE_LIMIT_GROUP - 1;
        let exact_expired_group = base_date + REVOKE_LIMIT_GROUP;
        assert!(item.can_delete_for_everyone(
            exact_valid_group,
            false,
            false,
            false,
            false,
            REVOKE_LIMIT_GROUP,
            REVOKE_LIMIT_PRIVATE,
            true,
        ));
        assert!(!item.can_delete_for_everyone(
            exact_expired_group,
            false,
            false,
            false,
            false,
            REVOKE_LIMIT_GROUP,
            REVOKE_LIMIT_PRIVATE,
            true,
        ));

        // Strict bot rejection: Bots receive updates and cannot be revoked
        let recent_time = base_date + 60;
        assert!(!item.can_delete_for_everyone(
            recent_time,
            false,
            true,
            false,
            true, // is_bot = true
            REVOKE_LIMIT_GROUP,
            REVOKE_LIMIT_PRIVATE,
            true,
        ));

        // Self chat rejection: Deleted locally without revoke dialog
        assert!(!item.can_delete_for_everyone(
            recent_time,
            true, // is_self = true
            true,
            false,
            false,
            REVOKE_LIMIT_GROUP,
            REVOKE_LIMIT_PRIVATE,
            true,
        ));

        // Channel rejection: Channel messages delete directly via admin rights
        assert!(!item.can_delete_for_everyone(
            recent_time,
            false,
            false,
            true, // is_channel = true
            false,
            REVOKE_LIMIT_GROUP,
            REVOKE_LIMIT_PRIVATE,
            true,
        ));

        // Incoming message in private user chat: allowed only if revoke_private_inbox == true
        item.flags.remove(MessageFlags::OUTGOING); // Incoming
        assert!(item.can_delete_for_everyone(
            recent_time,
            false,
            true,
            false,
            false,
            REVOKE_LIMIT_GROUP,
            REVOKE_LIMIT_PRIVATE,
            true, // revoke_private_inbox enabled
        ));
        assert!(!item.can_delete_for_everyone(
            recent_time,
            false,
            true,
            false,
            false,
            REVOKE_LIMIT_GROUP,
            REVOKE_LIMIT_PRIVATE,
            false, // revoke_private_inbox disabled
        ));
    }
}

// ==============================================================================
// 3. Selection Endpoints Math Oracle (history_message_selection.h:L21-L31, L115-L129)
// ==============================================================================

#[test]
fn test_oracle_selection_endpoints_math_10000_fuzz() {
    let mut rng = XorshiftRng::new(0xCAFEBABE03);
    let sample_text = "The quick brown fox jumps over the lazy dog. 1234567890! Special \u{1F600} emoji text for range testing.";

    for _ in 0..10_000 {
        let anchor_sym = (rng.next_u64() % 65530) as u16;
        let anchor_after = rng.next_bool();
        let focus_sym = (rng.next_u64() % 65530) as u16;
        let focus_after = rng.next_bool();

        let anchor = MessageSelectionFlatEndpoint::new(anchor_sym, anchor_after);
        let focus = MessageSelectionFlatEndpoint::new(focus_sym, focus_after);

        let anchor_offset = anchor.offset();
        let focus_offset = focus.offset();

        // Upstream Oracle verification: MessageSelection::from_endpoints
        let sel = MessageSelection::from_endpoints(anchor, focus);
        let sel_inverted = MessageSelection::from_endpoints(focus, anchor);

        if anchor_offset == focus_offset {
            // Invariant: identical offsets yield empty selection
            assert!(sel.is_empty(), "Equal offsets must yield empty selection");
            assert_eq!(sel.kind, MessageSelectionKind::None);
            assert_eq!(sel_inverted.kind, MessageSelectionKind::None);
        } else {
            // Invariant: normalized output always satisfies from < to
            assert_eq!(sel.kind, MessageSelectionKind::Flat);
            assert_eq!(sel_inverted.kind, MessageSelectionKind::Flat);

            let expected_from = anchor_offset.min(focus_offset) as usize;
            let expected_to = anchor_offset.max(focus_offset) as usize;

            assert_eq!(sel.flat.from, expected_from);
            assert_eq!(sel.flat.to, expected_to);
            assert!(sel.flat.from < sel.flat.to, "Normalized invariant: from < to");
            assert_eq!(sel.flat.len(), expected_to - expected_from);

            // Inversion symmetry invariant: from_endpoints(A, B).flat == from_endpoints(B, A).flat
            assert_eq!(sel.flat, sel_inverted.flat);

            // Contains oracle check
            let test_pt_inside = MessageSelectionFlatEndpoint::new(expected_from as u16, false);
            assert!(sel.contains(&test_pt_inside));

            let test_pt_at_to = MessageSelectionFlatEndpoint::new(expected_to as u16, false);
            assert!(!sel.contains(&test_pt_at_to)); // upper bound is exclusive: [from, to)

            if expected_from > 0 {
                let test_pt_before = MessageSelectionFlatEndpoint::new((expected_from - 1) as u16, false);
                assert!(!sel.contains(&test_pt_before));
            }
        }

        // Fuzz slice extraction: must never panic regardless of bounds
        let extracted = sel.extract_slice(sample_text);
        if !sel.is_empty() && sel.flat.from < sample_text.len() {
            let clamped_to = sel.flat.to.min(sample_text.len());
            if sel.flat.from < clamped_to {
                assert_eq!(extracted.len(), clamped_to - sel.flat.from);
            } else {
                assert!(extracted.is_empty());
            }
        } else {
            assert!(extracted.is_empty());
        }
    }
}

// ==============================================================================
// 4. Admin Log Deletion Coalescing Oracle (history_admin_log_inner.cpp:L89-L103)
// ==============================================================================

#[test]
fn test_oracle_admin_log_deletion_coalescing_1000_events() {
    let mut rng = XorshiftRng::new(0xF00DBEEF04);
    let mut inner = HistoryAdminLogInnerAdminLog::new(1001);

    // Independent Oracle Simulator
    struct OracleDeleteGroup {
        admin_id: i64,
        first_event_date: i64,
        message_ids: Vec<i64>,
    }

    let mut oracle_groups: Vec<OracleDeleteGroup> = Vec::new();
    let mut current_time = 1_700_000_000i64;

    for event_id in 1..=1_000u64 {
        let admin_id = rng.next_range(1, 3); // 3 distinct admins
        let time_step = rng.next_range(0, 10); // 0 to 10 seconds step
        current_time += time_step;
        let msg_id = 50_000 + event_id as i64;

        // Upstream Oracle: history_admin_log_inner.cpp:L1446-L1461
        // sameAdmin && withinTimeWindow (std::abs(thisDate - groupFirstDate) <= kDeleteGroupTimeWindowSeconds)
        let mut coalesced = false;
        if let Some(last_group) = oracle_groups.last_mut() {
            if last_group.admin_id == admin_id
                && (current_time - last_group.first_event_date).abs() <= K_DELETE_GROUP_TIME_WINDOW_SECS
            {
                last_group.message_ids.push(msg_id);
                coalesced = true;
            }
        }
        if !coalesced {
            oracle_groups.push(OracleDeleteGroup {
                admin_id,
                first_event_date: current_time,
                message_ids: vec![msg_id],
            });
        }

        // Feed to implementation
        inner.record_message_deleted(event_id, admin_id, current_time, msg_id);
    }

    // Assert that the implementation groups EXACTLY match the oracle groups
    assert_eq!(
        inner.delete_groups.len(),
        oracle_groups.len(),
        "Delete group count mismatch with upstream C++ coalescing oracle"
    );

    for (idx, (actual, expected)) in inner.delete_groups.iter().zip(oracle_groups.iter()).enumerate() {
        assert_eq!(actual.admin_id, expected.admin_id, "Group #{} admin mismatch", idx);
        assert_eq!(actual.first_event_date, expected.first_event_date, "Group #{} first date mismatch", idx);
        assert_eq!(actual.message_ids, expected.message_ids, "Group #{} message IDs mismatch", idx);
        assert_eq!(actual.count(), expected.message_ids.len(), "Group #{} count mismatch", idx);
    }

    // Precise boundary test: exactly 5s vs 6s
    let mut test_group = AdminLogDeleteGroup::new(9999, 42, 100, 1);
    assert!(test_group.try_coalesce(42, 105, 2), "delta=5 must coalesce");
    assert!(!test_group.try_coalesce(42, 106, 3), "delta=6 must NOT coalesce");
    assert!(!test_group.try_coalesce(43, 104, 4), "different admin must NOT coalesce");
}

// ==============================================================================
// 5. Virtual List Window Preloading Oracle (history_widget.cpp:L224-L242)
// ==============================================================================

#[test]
fn test_oracle_virtual_list_preloading_rapid_scrolling() {
    let mut rng = XorshiftRng::new(0x1337C0DE05);
    let mut widget = HistoryWidget::new(42, "Virtual List Test");

    // Oracle constants check
    assert_eq!(PRELOAD_HEIGHTS_COUNT, 3.0);
    assert_eq!(MESSAGES_PER_PAGE_FIRST, 30);
    assert_eq!(MESSAGES_PER_PAGE, 50);

    // Initial first load
    assert_eq!(widget.first_load_messages(100), 30);
    assert!(widget.history_loaded_at_bottom());
    assert!(!widget.history_loaded_at_top());

    // Fuzz 2,000 rapid scrolling positions
    for _ in 0..2_000 {
        let viewport_height = rng.next_range(300, 1200) as f32;
        let total_height = rng.next_range(2_000, 50_000) as f32;
        let max_scroll = (total_height - viewport_height).max(0.0);
        let scroll_top = (rng.next_u64() % ((max_scroll as u64) + 1)) as f32;

        // Upward preload oracle: scroll_top <= viewport_height * PRELOAD_HEIGHTS_COUNT
        let expected_preload_up = scroll_top <= (viewport_height * PRELOAD_HEIGHTS_COUNT);
        assert_eq!(
            widget.should_preload_upward(scroll_top, viewport_height),
            expected_preload_up
        );

        // Downward preload oracle: remaining <= viewport_height * PRELOAD_HEIGHTS_COUNT
        let remaining = total_height - (scroll_top + viewport_height);
        let expected_preload_down = remaining <= (viewport_height * PRELOAD_HEIGHTS_COUNT);
        // widget.is_loaded_at_bottom is currently true, so downward preload should be false
        assert!(!widget.should_preload_downward(scroll_top, viewport_height, total_height));

        // When unconstrained at bottom:
        widget.is_loaded_at_bottom = false;
        assert_eq!(
            widget.should_preload_downward(scroll_top, viewport_height, total_height),
            expected_preload_down
        );
        widget.is_loaded_at_bottom = true;
    }

    // Transcript Virtualization Window Oracle: HistoryInnerWidget
    let mut inner = HistoryInnerWidget::new();
    const NUM_ITEMS: usize = 500;
    let mut items = Vec::with_capacity(NUM_ITEMS);
    let mut expected_total_height = 0.0f32;

    for i in 1..=NUM_ITEMS {
        let height = (rng.next_range(30, 200)) as f32;
        expected_total_height += height;
        items.push(MessageItemGeometry {
            message_id: i as u64,
            top: 0.0,
            height,
            date_timestamp: 1_700_000_000 + (i as u64) * 1800, // half-hour increments
            text_snippet: format!("Transcript message #{}", i),
        });
    }

    inner.set_items(items);
    assert_eq!(inner.total_height(), expected_total_height);

    // Test 1,000 rapid scrolling viewport windows
    for step in 0..1_000 {
        let visible_height = rng.next_range(400, 800) as f32;
        let max_top = (inner.total_height() - visible_height).max(0.0);
        let scroll_top = (rng.next_u64() % ((max_top as u64) + 1)) as f32;
        let now_ms = 1_000_000 + (step as u64) * 500;

        inner.update_scroll(scroll_top, visible_height, now_ms);

        // Virtualization Window Bounds Oracle:
        // buffer = visible_height * UNLOAD_HEAVY_PARTS_PAGES (2.0)
        // window = [scroll_top - buffer, scroll_top + visible_height + buffer]
        let buffer = visible_height * UNLOAD_HEAVY_PARTS_PAGES;
        let window_top = (scroll_top - buffer).max(0.0);
        let window_bottom = scroll_top + visible_height + buffer;

        let visible = inner.visible_items();
        assert!(!visible.is_empty(), "Visible items window must not be empty during scroll");

        for item in &visible {
            assert!(
                item.top + item.height >= window_top && item.top <= window_bottom,
                "Item #{} (top={}, h={}) was rendered outside 2-page buffer [{}, {}]",
                item.message_id, item.top, item.height, window_top, window_bottom
            );
        }

        // First visible item must strictly intersect the visible viewport [scroll_top, scroll_top + visible_height]
        if let Some(first) = inner.first_visible_item() {
            assert!(
                first.top + first.height > scroll_top && first.top < scroll_top + visible_height,
                "First visible item #{} (top={}, h={}) does not intersect viewport [{}, {}]",
                first.message_id, first.top, first.height, scroll_top, scroll_top + visible_height
            );
        }

        // Floating date check
        assert!(inner.floating_date().is_some());
    }

    // Day crossing timeout oracle test (1s vs 3s)
    let single_day_items = vec![
        MessageItemGeometry {
            message_id: 1,
            top: 0.0,
            height: 100.0,
            date_timestamp: 1_700_000_000, // Day X
            text_snippet: "Day X msg 1".to_string(),
        },
        MessageItemGeometry {
            message_id: 2,
            top: 100.0,
            height: 100.0,
            date_timestamp: 1_700_000_100, // Day X
            text_snippet: "Day X msg 2".to_string(),
        },
        MessageItemGeometry {
            message_id: 3,
            top: 200.0,
            height: 100.0,
            date_timestamp: 1_700_086_400, // Day X+1
            text_snippet: "Day X+1 msg 1".to_string(),
        },
        MessageItemGeometry {
            message_id: 4,
            top: 300.0,
            height: 100.0,
            date_timestamp: 1_700_086_500, // Day X+1
            text_snippet: "Day X+1 msg 2".to_string(),
        },
        MessageItemGeometry {
            message_id: 5,
            top: 400.0,
            height: 100.0,
            date_timestamp: 1_700_086_600, // Day X+1
            text_snippet: "Day X+1 msg 3".to_string(),
        },
    ];

    let mut day_inner = HistoryInnerWidget::new();
    day_inner.set_items(single_day_items);

    // Initial scroll within same day
    day_inner.update_scroll(0.0, 150.0, 10_000);
    assert!(day_inner.floating_date().is_some());
    day_inner.check_floating_date_fade(10_000 + SCROLL_DATE_HIDE_TIMEOUT_MS - 100);
    assert!(day_inner.floating_date().is_some(), "Date should be visible before 1000ms timeout");
    day_inner.check_floating_date_fade(10_000 + SCROLL_DATE_HIDE_TIMEOUT_MS + 10);
    assert!(day_inner.floating_date().is_none(), "Date should fade after 1000ms timeout");

    // Scroll into next day (scroll past item 2 at y=200 so item 3 at y=200..300 is first visible)
    day_inner.update_scroll(210.0, 150.0, 20_000);
    assert!(day_inner.floating_date().is_some());
    // Since day changed, timeout is 3000ms (SCROLL_DATE_HIDE_DAY_CROSSING_TIMEOUT_MS)
    day_inner.check_floating_date_fade(20_000 + 2000);
    assert!(
        day_inner.floating_date().is_some(),
        "Date should remain visible at 2000ms during day crossing (3s timeout)"
    );
    day_inner.check_floating_date_fade(20_000 + SCROLL_DATE_HIDE_DAY_CROSSING_TIMEOUT_MS + 10);
    assert!(
        day_inner.floating_date().is_none(),
        "Date should fade after 3000ms day crossing timeout"
    );
}
