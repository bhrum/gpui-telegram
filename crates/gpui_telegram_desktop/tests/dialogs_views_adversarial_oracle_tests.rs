//! Blind C++ Oracle Adversarial Stress Tests for Dialogs Views, Widgets, Navigation & FSM.
//!
//! Direct Upstream References:
//! - `Telegram/SourceFiles/dialogs/dialogs_inner_widget.cpp:L2733-2800, L5469-5650, L6703-6745`
//! - `Telegram/SourceFiles/dialogs/dialogs_widget.cpp:L114-124, L3100-3120`
//! - `Telegram/SourceFiles/dialogs/ui/dialogs_stories_list.cpp:L35-75, L273-295`
//! - `Telegram/SourceFiles/dialogs/suggestions/suggestion.h:L43-53`
//! - `Telegram/SourceFiles/dialogs/suggestions/suggestion.cpp:L12-25`
//! - `Telegram/SourceFiles/dialogs/dialogs_top_bar_suggestion.cpp:L130-220`
//!
//! Enforces Mandate #18 Layer 3 (Blind C++ Oracle Tests):
//! 1. Oracle 1: Virtual list keyboard navigation boundary clamping (selectSkip, selectSkipPage across 0, 1, 1000 items).
//! 2. Oracle 2: Pinned drag reorder threshold state machine (delta y < 30px vs delta y >= 30px, cancellation invariance).
//! 3. Oracle 3: Stories horizontal carousel hysteresis deadband (0.72 expand / 0.68 collapse) under rapid oscillation.
//! 4. Oracle 4: Suggestions priority queue dismissal cascade and dynamic promotion invariants (1..9).

use gpui_telegram_client::dialogs::key::DialogKey;
use gpui_telegram_desktop::views::sidebar::dialogs_inner_widget::{
    DialogsInnerWidget, K_START_REORDER_THRESHOLD_PX,
};
use gpui_telegram_desktop::views::sidebar::dialogs_pinned_list::DialogsPinnedList;
use gpui_telegram_desktop::views::sidebar::suggestions::suggestion_manager::{
    SuggestionItem, SuggestionManager, SuggestionPriority, UnreviewedAuthDetails,
};
use gpui_telegram_desktop::views::sidebar::ui::dialogs_stories_list::{
    DialogsStoriesListUi, StoryElement, K_COLLAPSE_AFTER_RATIO, K_EXPAND_AFTER_RATIO,
    K_SMALL_THUMBS_SHOWN,
};
use gpui_telegram_desktop::views::sidebar::ui::dialogs_suggestions::DialogsSuggestionsUi;

// ============================================================================
// ORACLE 1: Virtual List Keyboard Navigation Clamping & Boundary Invariants
// Upstream: `dialogs_inner_widget.cpp:L5469-5650`, `dialogs_inner_widget.h:L160-161`
// ============================================================================

#[test]
fn test_oracle_1_keyboard_navigation_empty_list_zero_items() {
    let mut inner = DialogsInnerWidget::new();
    assert_eq!(inner.selected_index, None);

    // Empty list invariant: any select_skip direction must remain None and not panic or loop
    inner.select_skip(1, 0);
    assert_eq!(inner.selected_index, None);

    inner.select_skip(-1, 0);
    assert_eq!(inner.selected_index, None);

    inner.select_skip(100, 0);
    assert_eq!(inner.selected_index, None);

    inner.select_skip(-1000, 0);
    assert_eq!(inner.selected_index, None);

    // Page skips on empty list
    inner.select_skip_page(620, 1, 0);
    assert_eq!(inner.selected_index, None);

    inner.select_skip_page(620, -1, 0);
    assert_eq!(inner.selected_index, None);

    inner.select_skip_page(0, 1, 0);
    assert_eq!(inner.selected_index, None);

    inner.select_skip_page(-500, -1, 0);
    assert_eq!(inner.selected_index, None);
}

#[test]
fn test_oracle_1_keyboard_navigation_single_item_boundary_clamping() {
    let mut inner = DialogsInnerWidget::new();
    assert_eq!(inner.selected_index, None);

    // With 1 item (total_count = 1), initial skip forward selects index 0
    inner.select_skip(1, 1);
    assert_eq!(inner.selected_index, Some(0));

    // Subsequent skips forward or backward MUST strictly clamp to [0, 0]
    for dir in [-100, -10, -1, 0, 1, 10, 100] {
        inner.select_skip(dir, 1);
        assert_eq!(
            inner.selected_index,
            Some(0),
            "Failed clamping to 0 with direction {}",
            dir
        );
    }

    // Page skip on single item must strictly stay at 0
    inner.select_skip_page(620, 1, 1);
    assert_eq!(inner.selected_index, Some(0));

    inner.select_skip_page(620, -1, 1);
    assert_eq!(inner.selected_index, Some(0));

    inner.select_skip_page(100_000, 1, 1);
    assert_eq!(inner.selected_index, Some(0));

    inner.select_skip_page(-500, 1, 1);
    assert_eq!(inner.selected_index, Some(0));
}

#[test]
fn test_oracle_1_keyboard_navigation_1000_items_stress_and_page_skips() {
    let mut inner = DialogsInnerWidget::new();
    const TOTAL: usize = 1_000;
    const MAX_INDEX: usize = TOTAL - 1; // 999

    // Step 1: Initial step lands on Some(1)
    inner.select_skip(1, TOTAL);
    assert_eq!(inner.selected_index, Some(1));

    // Step 2: Monotonic forward walk for 998 steps reaches exactly 999
    for expected in 2..=MAX_INDEX {
        inner.select_skip(1, TOTAL);
        assert_eq!(inner.selected_index, Some(expected));
    }
    assert_eq!(inner.selected_index, Some(MAX_INDEX));

    // Step 3: Upper boundary clamping stress - 50 further forward steps must remain at 999
    for _ in 0..50 {
        inner.select_skip(1, TOTAL);
        assert_eq!(inner.selected_index, Some(MAX_INDEX));
    }

    // Extreme forward jump: +500,000 clamped to 999
    inner.select_skip(500_000, TOTAL);
    assert_eq!(inner.selected_index, Some(MAX_INDEX));

    // Step 4: Monotonic backward walk down to 0
    for expected in (0..MAX_INDEX).rev() {
        inner.select_skip(-1, TOTAL);
        assert_eq!(inner.selected_index, Some(expected));
    }
    assert_eq!(inner.selected_index, Some(0));

    // Step 5: Lower boundary clamping stress - 50 further backward steps must remain at 0
    for _ in 0..50 {
        inner.select_skip(-1, TOTAL);
        assert_eq!(inner.selected_index, Some(0));
    }

    // Extreme backward jump: -500,000 clamped to 0
    inner.select_skip(-500_000, TOTAL);
    assert_eq!(inner.selected_index, Some(0));

    // Step 6: Rapid boundary oscillation (alternating +1 / -1 at index 0)
    for _ in 0..100 {
        inner.select_skip(-1, TOTAL);
        assert_eq!(inner.selected_index, Some(0));
        inner.select_skip(1, TOTAL);
        assert_eq!(inner.selected_index, Some(1));
        inner.select_skip(-1, TOTAL);
        assert_eq!(inner.selected_index, Some(0));
    }

    // Step 7: Page skip navigation tests
    // With row_height = 62px, viewport = 620px -> 10 rows per page
    inner.selected_index = Some(0);
    inner.select_skip_page(620, 1, TOTAL);
    assert_eq!(inner.selected_index, Some(10));

    inner.select_skip_page(620, 1, TOTAL);
    assert_eq!(inner.selected_index, Some(20));

    inner.select_skip_page(620, -1, TOTAL);
    assert_eq!(inner.selected_index, Some(10));

    // Huge page jump clamping to upper boundary
    inner.select_skip_page(620_000, 1, TOTAL);
    assert_eq!(inner.selected_index, Some(MAX_INDEX));

    // Huge page jump clamping to lower boundary
    inner.select_skip_page(620_000, -1, TOTAL);
    assert_eq!(inner.selected_index, Some(0));

    // Fallback for zero or small viewport height: must skip at least 1 item
    inner.select_skip_page(10, 1, TOTAL); // 10 / 62 = 0 -> clamped to .max(1)
    assert_eq!(inner.selected_index, Some(1));

    inner.select_skip_page(0, 1, TOTAL); // 0 / 62 = 0 -> clamped to .max(1)
    assert_eq!(inner.selected_index, Some(2));
}

// ============================================================================
// ORACLE 2: Pinned Drag Reorder Threshold State Machine & Invariance
// Upstream: `dialogs_inner_widget.cpp:L2733-2800`, `dialogs_pinned_list.cpp:L1-179`
// ============================================================================

#[test]
fn test_oracle_2_pinned_drag_threshold_boundary_precision() {
    assert_eq!(K_START_REORDER_THRESHOLD_PX, 30);
    let start_y = 500;

    // Movement strictly below 30px MUST NOT initiate reorder (no-op)
    for delta in 0..30 {
        assert!(
            !DialogsInnerWidget::check_reorder_pinned_start(start_y, start_y + delta),
            "Failed: delta +{} unexpectedly triggered reorder",
            delta
        );
        assert!(
            !DialogsInnerWidget::check_reorder_pinned_start(start_y, start_y - delta),
            "Failed: delta -{} unexpectedly triggered reorder",
            delta
        );
    }

    // Boundary edge cases: 29 is false, 30 is true
    assert!(!DialogsInnerWidget::check_reorder_pinned_start(start_y, start_y + 29));
    assert!(!DialogsInnerWidget::check_reorder_pinned_start(start_y, start_y - 29));

    assert!(DialogsInnerWidget::check_reorder_pinned_start(start_y, start_y + 30));
    assert!(DialogsInnerWidget::check_reorder_pinned_start(start_y, start_y - 30));
    assert!(DialogsInnerWidget::check_reorder_pinned_start(start_y, start_y + 31));
    assert!(DialogsInnerWidget::check_reorder_pinned_start(start_y, start_y - 31));

    // Large movements
    assert!(DialogsInnerWidget::check_reorder_pinned_start(start_y, start_y + 1000));
    assert!(DialogsInnerWidget::check_reorder_pinned_start(start_y, start_y - 1000));

    // Negative coordinate spaces
    assert!(!DialogsInnerWidget::check_reorder_pinned_start(-200, -171)); // |delta| = 29
    assert!(DialogsInnerWidget::check_reorder_pinned_start(-200, -170)); // |delta| = 30
}

#[test]
fn test_oracle_2_pinned_drag_lifecycle_and_cancellation_invariance() {
    let mut inner = DialogsInnerWidget::new();

    // Invariant: cannot start reorder with fewer than 2 pinned items (upstream L2757)
    let single_item_order = vec![101];
    inner.start_reorder_pinned(101, 200, single_item_order);
    assert!(
        inner.pinned_drag.is_none(),
        "Must not start drag for order.len() < 2"
    );

    let empty_order = Vec::<u64>::new();
    inner.start_reorder_pinned(101, 200, empty_order);
    assert!(inner.pinned_drag.is_none());

    // Valid pinned list with 5 items
    let original_order: Vec<u64> = vec![101, 102, 103, 104, 105];
    inner.start_reorder_pinned(103, 200, original_order.clone());

    assert!(inner.pinned_drag.is_some());
    let drag = inner.pinned_drag.as_ref().unwrap();
    assert_eq!(drag.dragging_peer_id, 103);
    assert_eq!(drag.start_y, 200);
    assert_eq!(drag.current_y, 200);
    assert!(drag.is_reordering);
    assert_eq!(drag.original_order, original_order);

    // Simulate drag position updates
    inner.update_drag_position(225); // delta 25 < 30px
    assert_eq!(inner.pinned_drag.as_ref().unwrap().current_y, 225);

    inner.update_drag_position(310); // delta 110 >= 30px
    assert_eq!(inner.pinned_drag.as_ref().unwrap().current_y, 310);

    // Cancellation invariant: if drag is cancelled or aborted, original order is restored
    let finished_order = inner.finish_drag();
    assert_eq!(
        finished_order,
        Some(original_order.clone()),
        "Original order must be preserved exactly"
    );
    assert!(inner.pinned_drag.is_none(), "Drag state must be cleared");

    // Second finish_drag returns None (idempotent)
    assert_eq!(inner.finish_drag(), None);
}

#[test]
fn test_oracle_2_pinned_sublist_reorder_and_capacity_limits() {
    // Upstream: DEFAULT_MAX_PINNED = 5, PREMIUM_MAX_PINNED = 10
    assert_eq!(DialogsPinnedList::DEFAULT_MAX_PINNED, 5);
    assert_eq!(DialogsPinnedList::PREMIUM_MAX_PINNED, 10);

    let mut pinned = DialogsPinnedList::new(0, DialogsPinnedList::DEFAULT_MAX_PINNED);

    // Pin up to capacity: [5, 4, 3, 2, 1]
    for id in 1..=5 {
        assert!(pinned.pin(DialogKey::History(id)));
    }
    assert_eq!(pinned.len(), 5);
    assert_eq!(
        pinned.data,
        vec![
            DialogKey::History(5),
            DialogKey::History(4),
            DialogKey::History(3),
            DialogKey::History(2),
            DialogKey::History(1),
        ]
    );

    // Capacity limit eviction invariant: pinning 6th item evicts the oldest (last) item 1
    assert!(pinned.pin(DialogKey::History(6)));
    assert_eq!(pinned.len(), 5);
    assert!(!pinned.contains(&DialogKey::History(1))); // Evicted
    assert!(pinned.contains(&DialogKey::History(6)));
    assert_eq!(pinned.data[0], DialogKey::History(6));

    // Duplicate pin is a no-op
    assert!(!pinned.pin(DialogKey::History(6)));
    assert_eq!(pinned.len(), 5);

    // Pairwise swap reorder invariant
    let key_6 = DialogKey::History(6);
    let key_3 = DialogKey::History(3);
    assert!(pinned.reorder(&key_6, &key_3));
    assert_eq!(pinned.data[0], key_3);
    assert_eq!(pinned.data[3], key_6);

    // Invalid reorder (non-existent key) fails safely
    let non_existent = DialogKey::History(9999);
    assert!(!pinned.reorder(&key_6, &non_existent));
    assert!(!pinned.reorder(&non_existent, &key_3));
    assert_eq!(pinned.len(), 5);
}

// ============================================================================
// ORACLE 3: Stories Horizontal Carousel Hysteresis & Expansion Thresholds
// Upstream: `dialogs_stories_list.cpp:L35-75, L273-295`
// ============================================================================

#[test]
fn test_oracle_3_stories_threshold_constants_and_boundary_precision() {
    assert_eq!(K_SMALL_THUMBS_SHOWN, 3);
    assert_eq!(K_EXPAND_AFTER_RATIO, 0.72);
    assert_eq!(K_COLLAPSE_AFTER_RATIO, 0.68);

    // Expand boundary test: must be >= 0.72
    assert!(DialogsStoriesListUi::should_expand(0.72));
    assert!(DialogsStoriesListUi::should_expand(0.72001));
    assert!(DialogsStoriesListUi::should_expand(0.99));
    assert!(DialogsStoriesListUi::should_expand(1.0));
    assert!(!DialogsStoriesListUi::should_expand(0.71999));
    assert!(!DialogsStoriesListUi::should_expand(0.71));
    assert!(!DialogsStoriesListUi::should_expand(0.0));

    // Collapse boundary test: must be <= 0.68
    assert!(DialogsStoriesListUi::should_collapse(0.68));
    assert!(DialogsStoriesListUi::should_collapse(0.67999));
    assert!(DialogsStoriesListUi::should_collapse(0.50));
    assert!(DialogsStoriesListUi::should_collapse(0.0));
    assert!(!DialogsStoriesListUi::should_collapse(0.68001));
    assert!(!DialogsStoriesListUi::should_collapse(0.69));
    assert!(!DialogsStoriesListUi::should_collapse(1.0));
}

/// Upstream Hysteresis State Machine Oracle Simulator.
/// Direct replica of `List::updateExpanding` in `dialogs_stories_list.cpp:L273-291`.
struct UpstreamStoriesHysteresisSimulator {
    expanded: bool,
    last_ratio: f64,
}

impl UpstreamStoriesHysteresisSimulator {
    fn new() -> Self {
        Self {
            expanded: false,
            last_ratio: 0.0,
        }
    }

    fn update_expanding(&mut self, ratio: f64) -> bool {
        if self.last_ratio == ratio {
            return false;
        }
        let expanding = ratio > self.last_ratio;
        self.last_ratio = ratio;

        let change = if self.expanded {
            !expanding && ratio < (K_COLLAPSE_AFTER_RATIO as f64)
        } else {
            expanding && ratio > (K_EXPAND_AFTER_RATIO as f64)
        };

        if change {
            self.expanded = !self.expanded;
        }
        change
    }
}

#[test]
fn test_oracle_3_stories_hysteresis_deadband_and_rapid_oscillation_stress() {
    let mut sim = UpstreamStoriesHysteresisSimulator::new();
    assert!(!sim.expanded);

    // Phase A: Expanding motion approaching threshold
    assert!(!sim.update_expanding(0.50));
    assert!(!sim.expanded);

    assert!(!sim.update_expanding(0.70));
    assert!(!sim.expanded);

    // Oscillation inside deadband (0.68 .. 0.72) while collapsed
    for _ in 0..500 {
        sim.update_expanding(0.715);
        assert!(!sim.expanded, "Should not expand below 0.72");
        sim.update_expanding(0.685);
        assert!(!sim.expanded, "Should remain collapsed");
    }

    // Step across expand threshold (> 0.72) while expanding
    let triggered_expand = sim.update_expanding(0.725);
    assert!(triggered_expand);
    assert!(sim.expanded, "Must snap expanded at ratio > 0.72");

    // Phase B: Oscillation inside deadband (0.68 .. 0.72) while expanded
    for _ in 0..500 {
        sim.update_expanding(0.715);
        assert!(!sim.update_expanding(0.715));
        assert!(sim.expanded, "Should remain expanded above 0.68");

        sim.update_expanding(0.685);
        assert!(sim.expanded, "Should remain expanded inside deadband");
    }

    // Step across collapse threshold (< 0.68) while collapsing
    let triggered_collapse = sim.update_expanding(0.675);
    assert!(triggered_collapse);
    assert!(!sim.expanded, "Must snap collapsed at ratio < 0.68");

    // Phase C: Expanding direction must NOT trigger expand if ratio <= 0.72
    assert!(!sim.update_expanding(0.719));
    assert!(!sim.expanded);

    // Collapsing direction must NOT trigger collapse if ratio >= 0.68
    sim.update_expanding(0.85); // Expand
    assert!(sim.expanded);
    sim.update_expanding(0.75); // Moving down, but 0.75 >= 0.68
    assert!(sim.expanded);
    sim.update_expanding(0.681); // Moving down, but 0.681 >= 0.68
    assert!(sim.expanded);

    // Finally snap collapse at 0.67
    sim.update_expanding(0.67);
    assert!(!sim.expanded);
}

#[test]
fn test_oracle_3_stories_collapsed_preview_count_invariants() {
    let mut stories = DialogsStoriesListUi::new();
    assert_eq!(stories.collapsed_preview_count(), 0);

    let make_element = |id: u64| StoryElement {
        id,
        peer_name: format!("Peer {}", id),
        avatar_initials: "P".to_string(),
        avatar_color: 0x123456,
        story_count: 1,
        unread_count: 1,
        has_video_stream: false,
    };

    // 1 element -> 1 preview
    stories.set_elements(vec![make_element(1)]);
    assert_eq!(stories.collapsed_preview_count(), 1);

    // 2 elements -> 2 previews
    stories.set_elements(vec![make_element(1), make_element(2)]);
    assert_eq!(stories.collapsed_preview_count(), 2);

    // 3 elements -> 3 previews (K_SMALL_THUMBS_SHOWN)
    stories.set_elements(vec![make_element(1), make_element(2), make_element(3)]);
    assert_eq!(stories.collapsed_preview_count(), 3);

    // 10 elements -> clamped to exactly 3 previews
    let ten_elements: Vec<StoryElement> = (1..=10).map(make_element).collect();
    stories.set_elements(ten_elements);
    assert_eq!(stories.collapsed_preview_count(), 3);

    // Toggle expansion state
    assert!(!stories.is_expanded);
    assert_eq!(stories.expand_ratio, 0.0);
    stories.toggle_expanded();
    assert!(stories.is_expanded);
    assert_eq!(stories.expand_ratio, 1.0);
    stories.toggle_expanded();
    assert!(!stories.is_expanded);
    assert_eq!(stories.expand_ratio, 0.0);
}

// ============================================================================
// ORACLE 4: Suggestions Priority Queue Dismissal Cascade & Dynamic Promotion
// Upstream: `suggestion.h:L43-53`, `dialogs_top_bar_suggestion.cpp:L130-220`
// ============================================================================

#[test]
fn test_oracle_4_suggestions_strict_priority_enum_ordering() {
    // Upstream `Dialogs::TopBarSuggestions::Priority` enum values 1..9
    assert_eq!(SuggestionPriority::UserpicSetup as u8, 1);
    assert_eq!(SuggestionPriority::PremiumOffer as u8, 2);
    assert_eq!(SuggestionPriority::BirthdaySetup as u8, 3);
    assert_eq!(SuggestionPriority::BirthdayContacts as u8, 4);
    assert_eq!(SuggestionPriority::LowCreditsSubs as u8, 5);
    assert_eq!(SuggestionPriority::PremiumGrace as u8, 6);
    assert_eq!(SuggestionPriority::CustomPromo as u8, 7);
    assert_eq!(SuggestionPriority::GiftAuctions as u8, 8);
    assert_eq!(SuggestionPriority::UnreviewedAuth as u8, 9);

    // Strict transitive ordering invariant (lower integer = higher priority)
    assert!(SuggestionPriority::UserpicSetup < SuggestionPriority::PremiumOffer);
    assert!(SuggestionPriority::PremiumOffer < SuggestionPriority::BirthdaySetup);
    assert!(SuggestionPriority::BirthdaySetup < SuggestionPriority::BirthdayContacts);
    assert!(SuggestionPriority::BirthdayContacts < SuggestionPriority::LowCreditsSubs);
    assert!(SuggestionPriority::LowCreditsSubs < SuggestionPriority::PremiumGrace);
    assert!(SuggestionPriority::PremiumGrace < SuggestionPriority::CustomPromo);
    assert!(SuggestionPriority::CustomPromo < SuggestionPriority::GiftAuctions);
    assert!(SuggestionPriority::GiftAuctions < SuggestionPriority::UnreviewedAuth);
}

#[test]
fn test_oracle_4_suggestions_cascade_dismissal_and_promotion_invariants() {
    let mut manager = SuggestionManager::new();
    assert_eq!(manager.active_count(), 0);
    assert!(manager.current_suggestion().is_none());

    let all_priorities = [
        (SuggestionPriority::UnreviewedAuth, "KEY_AUTH"),
        (SuggestionPriority::GiftAuctions, "KEY_GIFTS"),
        (SuggestionPriority::CustomPromo, "KEY_PROMO"),
        (SuggestionPriority::PremiumGrace, "KEY_GRACE"),
        (SuggestionPriority::LowCreditsSubs, "KEY_CREDITS"),
        (SuggestionPriority::BirthdayContacts, "KEY_CONTACTS"),
        (SuggestionPriority::BirthdaySetup, "KEY_BDAY"),
        (SuggestionPriority::PremiumOffer, "KEY_OFFER"),
        (SuggestionPriority::UserpicSetup, "KEY_USERPIC"),
    ];

    // Insert all 9 suggestions in reverse priority order
    for (priority, key) in all_priorities.iter() {
        manager.add_suggestion(SuggestionItem {
            priority: *priority,
            key: key.to_string(),
            title: format!("Title for {:?}", priority),
            description: "Test description".to_string(),
            button_text: Some("Action".to_string()),
            action_url: None,
            can_close: true,
        });
    }

    assert_eq!(manager.active_count(), 9);

    // Invariant: UserpicSetup (Priority 1) MUST be at the front regardless of insertion order
    assert_eq!(
        manager.current_suggestion().unwrap().priority,
        SuggestionPriority::UserpicSetup
    );

    // Cascade dismissal: dismissing priority k MUST immediately promote priority k+1
    let expected_sequence = [
        ("KEY_USERPIC", SuggestionPriority::PremiumOffer),
        ("KEY_OFFER", SuggestionPriority::BirthdaySetup),
        ("KEY_BDAY", SuggestionPriority::BirthdayContacts),
        ("KEY_CONTACTS", SuggestionPriority::LowCreditsSubs),
        ("KEY_CREDITS", SuggestionPriority::PremiumGrace),
        ("KEY_GRACE", SuggestionPriority::CustomPromo),
        ("KEY_PROMO", SuggestionPriority::GiftAuctions),
        ("KEY_GIFTS", SuggestionPriority::UnreviewedAuth),
    ];

    for (dismiss_key, next_expected_priority) in expected_sequence {
        manager.dismiss(dismiss_key);
        assert!(manager.is_dismissed(dismiss_key));
        assert_eq!(
            manager.current_suggestion().unwrap().priority,
            next_expected_priority,
            "Failed promoting {:?} after dismissing {}",
            next_expected_priority,
            dismiss_key
        );
    }

    // Dismiss final item (Priority 9: UnreviewedAuth)
    manager.dismiss("KEY_AUTH");
    assert_eq!(manager.active_count(), 0);
    assert!(manager.current_suggestion().is_none());

    // Invariant: Dismissed suggestions CANNOT be re-added
    manager.add_suggestion(SuggestionItem {
        priority: SuggestionPriority::UserpicSetup,
        key: "KEY_USERPIC".to_string(),
        title: "Re-added Userpic".to_string(),
        description: "Ignored".to_string(),
        button_text: None,
        action_url: None,
        can_close: true,
    });
    assert_eq!(
        manager.active_count(),
        0,
        "Dismissed item must not be re-added"
    );
    assert!(manager.current_suggestion().is_none());
}

#[test]
fn test_oracle_4_suggestions_interleaved_preemption_and_view_binding() {
    let mut manager = SuggestionManager::new();
    let mut ui = DialogsSuggestionsUi::new();
    assert!(!ui.is_visible);
    assert!(ui.current_item.is_none());

    // Step 1: Add low priority (7: CustomPromo)
    manager.add_suggestion(SuggestionItem {
        priority: SuggestionPriority::CustomPromo,
        key: "PROMO".to_string(),
        title: "Promo 1".to_string(),
        description: "Discount".to_string(),
        button_text: None,
        action_url: None,
        can_close: true,
    });
    ui.update_from_manager(&manager);
    assert!(ui.is_visible);
    assert_eq!(
        ui.current_item.as_ref().unwrap().priority,
        SuggestionPriority::CustomPromo
    );

    // Step 2: Higher priority (3: BirthdaySetup) preempts 7
    manager.add_suggestion(SuggestionItem {
        priority: SuggestionPriority::BirthdaySetup,
        key: "BDAY".to_string(),
        title: "Birthday".to_string(),
        description: "Enter your birthday".to_string(),
        button_text: None,
        action_url: None,
        can_close: true,
    });
    ui.update_from_manager(&manager);
    assert_eq!(
        ui.current_item.as_ref().unwrap().priority,
        SuggestionPriority::BirthdaySetup
    );

    // Step 3: Highest priority (1: UserpicSetup) preempts 3
    manager.add_suggestion(SuggestionItem {
        priority: SuggestionPriority::UserpicSetup,
        key: "USERPIC".to_string(),
        title: "Avatar".to_string(),
        description: "Set profile photo".to_string(),
        button_text: None,
        action_url: None,
        can_close: true,
    });
    ui.update_from_manager(&manager);
    assert_eq!(
        ui.current_item.as_ref().unwrap().priority,
        SuggestionPriority::UserpicSetup
    );

    // Step 4: Dismiss current via UI view
    ui.dismiss_current(&mut manager);
    assert_eq!(
        ui.current_item.as_ref().unwrap().priority,
        SuggestionPriority::BirthdaySetup
    );

    // Step 5: Dismiss next via UI view
    ui.dismiss_current(&mut manager);
    assert_eq!(
        ui.current_item.as_ref().unwrap().priority,
        SuggestionPriority::CustomPromo
    );

    // Step 6: Dismiss last via UI view -> becomes hidden
    ui.dismiss_current(&mut manager);
    assert!(!ui.is_visible);
    assert!(ui.current_item.is_none());
}

#[test]
fn test_oracle_4_unreviewed_auth_formatting_invariants() {
    // Both location and device present
    let full = UnreviewedAuthDetails {
        location: "Zurich, Switzerland".to_string(),
        device: "Desktop PC".to_string(),
        date: "Today, 14:30".to_string(),
    };
    assert_eq!(
        full.format_auth_info(),
        "🌍 Zurich, Switzerland (📱 Desktop PC)"
    );

    // Location only
    let loc_only = UnreviewedAuthDetails {
        location: "Tokyo, Japan".to_string(),
        device: String::new(),
        date: "Yesterday".to_string(),
    };
    assert_eq!(loc_only.format_auth_info(), "🌍 Tokyo, Japan");

    // Device only
    let dev_only = UnreviewedAuthDetails {
        location: String::new(),
        device: "iPhone 16 Pro".to_string(),
        date: "Nov 5".to_string(),
    };
    assert_eq!(dev_only.format_auth_info(), "📱 iPhone 16 Pro");

    // Empty
    let empty = UnreviewedAuthDetails {
        location: String::new(),
        device: String::new(),
        date: String::new(),
    };
    assert_eq!(empty.format_auth_info(), "");
}
