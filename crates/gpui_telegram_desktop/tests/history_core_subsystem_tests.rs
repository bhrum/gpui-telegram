//! History Core Widgets & State Management Subsystem Tests (Module 06 Slice 1).
//!
//! Citations:
//! - Mandate #18 Anti-Bluffing upstream line ranges & constants:
//!   - `history.cpp:L87-L91`: kNewBlockEachMessage=50, kSkipCloudDraftsFor=2s, kCountUnreadMessagesLimit=10000
//!   - `history_widget.cpp:L224-L242`: kMessagesPerPageFirst=30, kMessagesPerPage=50, kPreloadHeightsCount=3
//!   - `history_inner_widget.cpp:L140-L145`: kScrollDateHideTimeout=1s, kScrollDateHideOnDayCrossingTimeout=3s, kUnloadHeavyPartsPages=2
//!   - `history_view_top_toast.cpp:L25-L32`: kAnchoredTooltipDuration=4s, CountToastDuration clamp 5s..8s (14 chars/sec)
//!   - `history_view_pull_to_next_channel.cpp:L48-L63`: kResetReachedOn=0.95, kReverseEase=1.5, kReadyDwell=120ms
//!   - `history_location_manager.cpp:L35-L42`: Maps URL with z=16
//!   - `history_streamed_drafts.cpp:L27`: kClearTimeout=30s
//!   - `poll_link_thumbnail.cpp:L39`: radius = size / 4.0

use gpui_telegram_desktop::views::stage::history::{
    History, HistoryClearType, HistoryNewMessageType, COUNT_UNREAD_MESSAGES_LIMIT,
    NEW_BLOCK_EACH_MESSAGE, SKIP_CLOUD_DRAFTS_FOR_SECS,
};
use gpui_telegram_desktop::views::stage::history_widget::{
    HistoryWidget, DISPLAY_EDIT_TIME_WARNING_MS, MESSAGES_PER_PAGE, MESSAGES_PER_PAGE_FIRST,
    PRELOAD_HEIGHTS_COUNT, SAVE_DRAFT_ANYWAY_TIMEOUT_MS, SAVE_DRAFT_TIMEOUT_MS,
};
use gpui_telegram_desktop::views::stage::history_inner_widget::{
    HistoryInnerWidget, MessageItemGeometry, SCROLL_DATE_HIDE_DAY_CROSSING_TIMEOUT_MS,
    SCROLL_DATE_HIDE_TIMEOUT_MS, UNLOAD_HEAVY_PARTS_PAGES,
};
use gpui_telegram_desktop::views::stage::history_inner_widget_accessibility::{
    AccessibleMessageNode, HistoryInnerWidgetAccessibility, MessageSubItem,
};
use gpui_telegram_desktop::views::stage::history_drag_area::{
    DragTargetZone, HistoryDragArea, DRAG_AREA_MINIMAL_HEIGHT,
};
use gpui_telegram_desktop::views::stage::history_location_manager::{
    HistoryLocationManager, LocationClickHandler, LocationPoint,
};
use gpui_telegram_desktop::views::stage::history_translation::{
    HistoryTranslation, HistoryTranslationUpdateFlag,
};
use gpui_telegram_desktop::views::stage::history_unread_things::{
    HistoryUnreadThings, UnreadThingsList,
};
use gpui_telegram_desktop::views::stage::history_view_top_toast::{
    count_toast_duration_ms, HistoryViewTopToast, ANCHORED_TOOLTIP_DURATION_MS,
};
use gpui_telegram_desktop::views::stage::history_view_pull_to_next_channel::{
    HistoryViewPullToNextChannel, PullToNextChannelMode, UnreadChannelCandidate,
    BOUNCE_DURATION_MS, DEFAULT_PULL_THRESHOLD_PX, EXPAND_DURATION_MS, READY_DWELL_MS,
    RELEASE_HIDE_DURATION_MS, RELEASE_SHOW_DURATION_MS, RESET_REACHED_ON, REVERSE_EASE,
};
use gpui_telegram_desktop::views::stage::history_view_swipe_back_session::HistoryViewSwipeBackSession;
use gpui_telegram_desktop::views::stage::history_view_highlight_manager::{
    HistoryViewHighlightManager, SelectedQuote, ACTIVE_FADE_IN_DURATION_MS,
    ACTIVE_FADE_OUT_DURATION_MS, FADE_WRAP_DURATION_MS,
};
use gpui_telegram_desktop::views::stage::history_streamed_drafts::{
    HistoryStreamedDraftKind, HistoryStreamedDrafts, STREAMED_DRAFT_CLEAR_TIMEOUT_MS,
};
use gpui_telegram_desktop::views::stage::history_message_selection::HistoryMessageSelection;
use gpui_telegram_desktop::views::stage::poll_link_box::PollLinkBox;
use gpui_telegram_desktop::views::stage::poll_link_thumbnail::PollLinkThumbnail;
use gpui_telegram_desktop::views::stage::poll_media_upload::PollMediaUpload;

// ============================================================================
// 1. History Location Manager Tests
// ============================================================================

#[test]
fn test_history_location_manager_url_and_zoom() {
    let pt = LocationPoint::new(37.774929, -122.419416);
    let handler = LocationClickHandler::new(pt);
    let url = handler.url_str();

    // Verify exact zoom level z=16 and Google Maps coordinate parameters
    assert!(url.contains("z=16"), "URL must enforce z=16 zoom constant");
    assert!(url.contains("37.774929"), "URL must contain formatted latitude");
    assert!(url.contains("-122.419416"), "URL must contain formatted longitude");
    assert_eq!(handler.copy_to_clipboard_text(), url);
    assert_eq!(handler.copy_to_clipboard_context_item_text(), "Copy Link");

    let mut manager = HistoryLocationManager::new();
    manager.set_point(pt);
    manager.set_venue("Golden Gate Park", "San Francisco, CA");
    assert_eq!(manager.current_point(), Some(pt));
    assert!(manager.map_url().unwrap().contains("z=16"));
}

// ============================================================================
// 2. History Translation Tests
// ============================================================================

#[test]
fn test_history_translation_state_and_flags() {
    let mut trans = HistoryTranslation::new(Some("es".to_string()));
    assert_eq!(trans.offered_from(), Some("es"));
    assert!(trans.is_bar_visible());

    let flags = trans.take_pending_flags();
    assert_eq!(flags, vec![HistoryTranslationUpdateFlag::TranslateFrom]);

    trans.translate_to("en");
    assert_eq!(trans.translated_to(), Some("en"));
    let flags2 = trans.take_pending_flags();
    assert_eq!(flags2, vec![HistoryTranslationUpdateFlag::TranslatedTo]);

    // Setting same target should not produce duplicate notification
    trans.translate_to("en");
    assert!(trans.take_pending_flags().is_empty());
}

// ============================================================================
// 3. History Unread Things Tests
// ============================================================================

#[test]
fn test_history_unread_things_arithmetic_and_bounds() {
    let mut unread = HistoryUnreadThings::new();
    assert_eq!(unread.total_unread_count(), 0);
    assert!(!unread.has_any());

    // Insert mentions
    unread.mentions.insert(105);
    unread.mentions.insert(200);
    unread.mentions.insert(95);
    unread.mentions.set_count(3);

    assert_eq!(unread.mentions.loaded_count(), 3);
    assert_eq!(unread.mentions.min_loaded(), 95);
    assert_eq!(unread.mentions.max_loaded(), 200);
    assert!(unread.mentions.contains(105));
    assert!(!unread.mentions.contains(999));

    // Reactions
    unread.reactions.insert(300);
    unread.reactions.set_count(1);

    assert_eq!(unread.total_unread_count(), 4);
    assert!(unread.has_any());
    assert_eq!(unread.earliest_unread_msg_id(), Some(95));

    // Erase
    unread.mentions.erase(95);
    assert_eq!(unread.mentions.min_loaded(), 105);
    assert_eq!(unread.earliest_unread_msg_id(), Some(105));

    unread.mentions.clear();
    assert_eq!(unread.mentions.loaded_count(), 0);
    assert_eq!(unread.mentions.min_loaded(), 0);
}

// ============================================================================
// 4. Top Toast & Reading Speed Clamp Tests
// ============================================================================

#[test]
fn test_top_toast_reading_duration_clamped() {
    // 14 chars/sec reading speed formula clamped [5000ms .. 8000ms]
    assert_eq!(count_toast_duration_ms(0), 5000, "0 chars must clamp to 5000ms min");
    assert_eq!(count_toast_duration_ms(14), 5000, "14 chars = 1000ms -> clamped to 5000ms");
    assert_eq!(count_toast_duration_ms(70), 5000, "70 chars = 5000ms -> 5000ms");
    assert_eq!(count_toast_duration_ms(98), 7000, "98 chars = 7000ms within bounds");
    assert_eq!(count_toast_duration_ms(112), 8000, "112 chars = 8000ms upper clamp");
    assert_eq!(count_toast_duration_ms(500), 8000, "500 chars must clamp to 8000ms max");

    assert_eq!(ANCHORED_TOOLTIP_DURATION_MS, 4000);

    let mut toast_mgr = HistoryViewTopToast::new();
    toast_mgr.show_info("Message copied to clipboard");
    assert!(toast_mgr.active_info().is_some());
    assert_eq!(toast_mgr.active_info().unwrap().duration_ms, 5000);
}

// ============================================================================
// 5. Swipe Back Session Gesture Tests
// ============================================================================

#[test]
fn test_history_view_swipe_back_session_gesture() {
    let mut session = HistoryViewSwipeBackSession::new();
    assert!(!session.is_active());

    // Starting while selecting text must be blocked
    session.set_selecting_text(true);
    assert!(!session.start_swipe(800.0));

    session.set_selecting_text(false);
    assert!(session.start_swipe(800.0));
    assert!(session.is_active());

    // Pull 200px (25% < 35% threshold)
    session.update_translation(200.0);
    assert_eq!(session.translation_x(), 200.0);
    assert_eq!(session.progress(), 0.25);
    assert!(!session.should_pop());

    // Pull 320px (40% >= 35% threshold)
    session.update_translation(320.0);
    assert_eq!(session.progress(), 0.40);
    assert!(session.should_pop());

    // Finish pops
    let popped = session.finish();
    assert!(popped);
    assert!(!session.is_active());
}

// ============================================================================
// 6. Streamed Drafts AI Generation Tests
// ============================================================================

#[test]
fn test_history_streamed_drafts_lifecycle_and_timeout() {
    let mut drafts = HistoryStreamedDrafts::new();
    assert!(!drafts.has_any());
    assert_eq!(STREAMED_DRAFT_CLEAR_TIMEOUT_MS, 30_000);

    drafts.apply(
        1001,
        2002,
        "Thinking about the solution...",
        HistoryStreamedDraftKind::Text,
        true,
        false,
        1000,
    );
    assert!(drafts.has_any());
    assert!(drafts.stoppable_for(1001));

    let draft = drafts.get_draft(1001).unwrap();
    assert_eq!(draft.text, "Thinking about the solution...");

    // Request stop without keep_on_stop removes draft
    drafts.request_stop(1001);
    assert!(!drafts.has_any());

    // Test expiry purge
    drafts.apply(
        1002,
        2002,
        "Chunk 1",
        HistoryStreamedDraftKind::Text,
        false,
        false,
        1000,
    );
    // At timestamp 20,000ms (< 30s elapsed) draft remains
    drafts.purge_expired(20_000);
    assert!(drafts.get_draft(1002).is_some());

    // At timestamp 40,000ms (> 30s elapsed) draft is purged
    drafts.purge_expired(40_000);
    assert!(drafts.get_draft(1002).is_none());
}

// ============================================================================
// 7. Message Selection Endpoint Tests
// ============================================================================

#[test]
fn test_history_message_selection_endpoints() {
    let mut sel = HistoryMessageSelection::new();
    assert!(!sel.is_active);

    sel.toggle_message(42);
    assert!(sel.is_selected(42));
    assert!(!sel.is_selected(43));
    assert!(sel.is_active);

    sel.select_range(10, 25, false);
    assert!(sel.contains_symbol(15));
    assert!(!sel.contains_symbol(5));
    assert!(!sel.contains_symbol(30));

    sel.clear();
    assert!(!sel.is_active);
    assert!(!sel.is_selected(42));
}

// ============================================================================
// 8. Drag Area Dual-Zone Drop Target Tests
// ============================================================================

#[test]
fn test_history_drag_area_zones_and_minimal_height() {
    assert_eq!(DRAG_AREA_MINIMAL_HEIGHT, 120.0);
    assert_eq!(HistoryDragArea::minimal_height(), 120.0);

    let mut area = HistoryDragArea::new();
    assert!(!area.is_active());

    area.drag_enter(true); // Image detected -> default to photo
    assert!(area.is_active());
    assert_eq!(area.hovered_zone(), Some(DragTargetZone::Photo));

    // Hover in bottom half (y=400 in 600px container) -> Document
    area.drag_move(400.0, 600.0);
    assert_eq!(area.hovered_zone(), Some(DragTargetZone::Document));

    // Hover in top half (y=100 in 600px container) -> Photo
    area.drag_move(100.0, 600.0);
    assert_eq!(area.hovered_zone(), Some(DragTargetZone::Photo));

    let (zone, files) = area.drop_files(vec!["photo.png".to_string()], false);
    assert_eq!(zone, DragTargetZone::Photo);
    assert_eq!(files, vec!["photo.png"]);
    assert!(!area.is_active());
}

// ============================================================================
// 9. Highlight Manager Jump Animation Tests
// ============================================================================

#[test]
fn test_history_view_highlight_manager_animation_cycle() {
    assert_eq!(ACTIVE_FADE_IN_DURATION_MS, 200);
    assert_eq!(FADE_WRAP_DURATION_MS, 300);
    assert_eq!(ACTIVE_FADE_OUT_DURATION_MS, 600);

    let mut mgr = HistoryViewHighlightManager::new();
    assert!(!mgr.is_animating());

    mgr.highlight(
        SelectedQuote {
            message_id: 88,
            text_range: Some((0, 15)),
            ..Default::default()
        },
        1000,
    );
    assert!(mgr.is_animating());
    assert_eq!(mgr.current_message_id(), Some(88));

    // In mid fade-in (100ms in) -> opacity ~0.5
    let (id, state) = mgr.paint_state(1100).unwrap();
    assert_eq!(id, 88);
    assert!((state.opacity - 0.5).abs() < 0.05);

    // In wrap dwell (350ms in -> t=1350ms) -> opacity 1.0, collapsion 1.0
    let (_, state) = mgr.paint_state(1350).unwrap();
    assert_eq!(state.opacity, 1.0);
    assert_eq!(state.collapsion, 1.0);

    // In fade-out (700ms in -> t=1700ms, elapsed=700 > 500)
    let (_, state) = mgr.paint_state(1700).unwrap();
    assert!(state.opacity < 1.0 && state.opacity > 0.0);

    // After full cycle (1000 + 200 + 300 + 600 = 2100ms) -> finishes
    let finished = mgr.paint_state(2200);
    assert!(finished.is_none());
    assert!(!mgr.is_animating());
}

// ============================================================================
// 10. Pull-To-Next-Channel 4-Tier Discovery Tests
// ============================================================================

#[test]
fn test_history_view_pull_to_next_channel_4tier_discovery() {
    let channels = vec![
        UnreadChannelCandidate {
            id: 1,
            title: "Archived News".to_string(),
            unread_count: 5,
            is_broadcast: true,
            is_archived: true,
            filter_id: None,
        },
        UnreadChannelCandidate {
            id: 2,
            title: "Work Channel".to_string(),
            unread_count: 2,
            is_broadcast: true,
            is_archived: false,
            filter_id: Some(10), // Filter #10 (Work)
        },
        UnreadChannelCandidate {
            id: 3,
            title: "Tech Channel".to_string(),
            unread_count: 8,
            is_broadcast: true,
            is_archived: false,
            filter_id: Some(20), // Filter #20 (Tech)
        },
        UnreadChannelCandidate {
            id: 4,
            title: "General News".to_string(),
            unread_count: 3,
            is_broadcast: true,
            is_archived: false,
            filter_id: None, // Main list
        },
    ];

    // Case 1: Active filter is 10 -> Tier 1 finds Channel #2
    let next = HistoryViewPullToNextChannel::find_next_unread_channel(99, Some(10), &channels);
    assert_eq!(next.unwrap().id, 2);

    // Case 2: Active filter is 10, but current is 2 -> Tier 2 finds Channel #3 (other filter)
    let next = HistoryViewPullToNextChannel::find_next_unread_channel(2, Some(10), &channels);
    assert_eq!(next.unwrap().id, 3);

    // Case 3: No active filter, current is 99 -> Tier 3 finds Channel #4 (main list)
    let next = HistoryViewPullToNextChannel::find_next_unread_channel(99, None, &channels);
    assert_eq!(next.unwrap().id, 4);

    // Gesture state machine & dwell
    let mut pull = HistoryViewPullToNextChannel::new();
    pull.set_next_channel(Some(channels[1].clone()));

    assert_eq!(DEFAULT_PULL_THRESHOLD_PX, 128.0);
    assert_eq!(RESET_REACHED_ON, 0.95);
    assert_eq!(REVERSE_EASE, 1.5);
    assert_eq!(READY_DWELL_MS, 120);

    // Pull 60px (< 128px) -> Pulling
    pull.update_pull(60.0, 1000);
    assert_eq!(pull.mode(), PullToNextChannelMode::Pulling);

    // Pull past threshold (140px > 128px) -> Ready
    pull.update_pull(80.0, 1000);
    assert_eq!(pull.mode(), PullToNextChannelMode::Ready);

    // Release before 120ms dwell -> Bounces
    let target = pull.release_pull(1050); // only 50ms elapsed
    assert_eq!(target, None);
    assert_eq!(pull.mode(), PullToNextChannelMode::Bouncing);

    // Pull again and hold past 120ms dwell -> Releases and returns target channel id
    pull.set_next_channel(Some(channels[1].clone()));
    pull.update_pull(140.0, 2000);
    let target = pull.release_pull(2150); // 150ms elapsed >= 120ms
    assert_eq!(target, Some(2));
    assert_eq!(pull.mode(), PullToNextChannelMode::Released);
}

// ============================================================================
// 11. Accessibility Semantic Node Tests
// ============================================================================

#[test]
fn test_history_accessibility_subitems_and_announcement() {
    let mut node = AccessibleMessageNode::new(77, "Alice", "10:45 AM", "Hello team!", false);
    node.add_subitem(MessageSubItem::Delivery, "Delivered");
    node.add_subitem(MessageSubItem::Reactions, "3 thumbs up");

    let announcement = node.build_accessible_announcement();
    assert!(announcement.contains("Message from Alice"));
    assert!(announcement.contains("10:45 AM"));
    assert!(announcement.contains("Hello team!"));
    assert!(announcement.contains("Reactions: 3 thumbs up"));

    let mut a11y = HistoryInnerWidgetAccessibility::new();
    a11y.set_nodes(vec![node]);
    assert_eq!(a11y.node_count(), 1);

    let focused = a11y.focus_next_message().unwrap();
    assert_eq!(focused.message_id, 77);
}

// ============================================================================
// 12. Poll Link Box, Thumbnail & Media Upload Tests
// ============================================================================

#[test]
fn test_poll_helpers_validation_and_upload() {
    // Poll Link Box
    let mut box_prompt = PollLinkBox::new("   ");
    assert!(box_prompt.submit().is_err(), "Empty URL must error");
    assert!(box_prompt.has_error);

    box_prompt.set_input("ftp://invalid-protocol.com");
    assert!(box_prompt.submit().is_err(), "Invalid protocol must error");

    box_prompt.set_input("  https://t.me/example_poll  ");
    let result = box_prompt.submit();
    assert_eq!(result.unwrap(), "https://t.me/example_poll");
    assert!(!box_prompt.is_open());

    // Poll Thumbnail radius math (size / 4.0)
    let thumb = PollLinkThumbnail::new(60.0);
    assert_eq!(thumb.size(), 60.0);
    assert_eq!(thumb.corner_radius(), 15.0);

    // Poll Media Upload
    let mut upload_mgr = PollMediaUpload::new();
    let id = upload_mgr.start_upload("chart.png", 204_800);
    assert!(upload_mgr.is_uploading());

    upload_mgr.update_progress(0.65);
    assert_eq!(upload_mgr.active_state().unwrap().progress, 0.65);

    upload_mgr.complete_upload(889900, 5000);
    assert!(!upload_mgr.is_uploading());
    assert_eq!(upload_mgr.active_state().unwrap().upload_token, Some(889900));
}

// ============================================================================
// 13. History Core & Widget Orchestrator Tests
// ============================================================================

#[test]
fn test_history_core_and_widget_constants() {
    assert_eq!(NEW_BLOCK_EACH_MESSAGE, 50);
    assert_eq!(SKIP_CLOUD_DRAFTS_FOR_SECS, 2);
    assert_eq!(COUNT_UNREAD_MESSAGES_LIMIT, 10_000);

    let mut history = History::new(555).with_topic(10);
    assert_eq!(history.peer_id, 555);
    assert_eq!(history.topic_root_id, Some(10));
    assert!(history.is_forum);

    history.add_message(101, "First message", HistoryNewMessageType::Last);
    history.add_message(102, "Second message", HistoryNewMessageType::Unread);
    assert_eq!(history.message_count(), 2);
    assert_eq!(history.unread_count, 1);

    let (min_id, max_id) = history.range_for_difference_request();
    assert_eq!(min_id, 101);
    assert_eq!(max_id, 102);

    history.clear(HistoryClearType::ClearHistory);
    assert!(history.is_empty());
    assert_eq!(history.unread_count, 0);

    // HistoryWidget
    assert_eq!(MESSAGES_PER_PAGE_FIRST, 30);
    assert_eq!(MESSAGES_PER_PAGE, 50);
    assert_eq!(PRELOAD_HEIGHTS_COUNT, 3.0);
    assert_eq!(DISPLAY_EDIT_TIME_WARNING_MS, 300_000);

    let mut widget = HistoryWidget::new(555, "Development Chat");
    widget.set_pinned_message(Some("Sprint v1.0 Launch".to_string()));
    assert_eq!(widget.pinned_text.as_deref(), Some("Sprint v1.0 Launch"));

    assert_eq!(widget.first_load_messages(100), 30);
    assert_eq!(widget.load_messages(100), 50);

    // Preload triggers when remaining distance is within 3 screens (e.g. 1500px for 600px viewport)
    assert!(widget.should_preload_upward(1200.0, 600.0)); // 1200 <= 1800
    assert!(!widget.should_preload_upward(2000.0, 600.0)); // 2000 > 1800

    // Autosave draft debouncers
    assert_eq!(SAVE_DRAFT_TIMEOUT_MS, 1000);
    assert_eq!(SAVE_DRAFT_ANYWAY_TIMEOUT_MS, 5000);
    assert!(widget.should_flush_draft(1500, true));
    widget.record_draft_saved(1500);
    assert!(!widget.should_flush_draft(2000, true)); // only 500ms elapsed
    assert!(widget.should_flush_draft(2600, true)); // 1100ms elapsed >= 1000ms
}

// ============================================================================
// 14. Virtualized Inner Widget Viewport Tests
// ============================================================================

#[test]
fn test_history_inner_widget_virtualization_and_dates() {
    assert_eq!(SCROLL_DATE_HIDE_TIMEOUT_MS, 1000);
    assert_eq!(SCROLL_DATE_HIDE_DAY_CROSSING_TIMEOUT_MS, 3000);
    assert_eq!(UNLOAD_HEAVY_PARTS_PAGES, 2.0);

    let mut inner = HistoryInnerWidget::new();

    let items = (1..=20)
        .map(|i| MessageItemGeometry {
            message_id: i,
            top: 0.0,
            height: 50.0, // 20 * 50 = 1000px total
            date_timestamp: 1_700_000_000 + i * 3600,
            text_snippet: format!("Item #{}", i),
        })
        .collect();

    inner.set_items(items);
    assert_eq!(inner.total_height(), 1000.0);

    // Scroll to y=200 with viewport 300px
    inner.update_scroll(200.0, 300.0, 10_000);
    assert_eq!(inner.scroll_top(), 200.0);
    assert!(inner.floating_date().is_some());

    // First visible item: top + height >= 200. Item 5 has top=200..250
    let first = inner.first_visible_item().unwrap();
    assert_eq!(first.message_id, 5);

    // Fade check before 1000ms: still visible
    inner.check_floating_date_fade(10_500);
    assert!(inner.floating_date().is_some());

    // Fade check after 1000ms: hidden
    inner.check_floating_date_fade(11_500);
    assert!(inner.floating_date().is_none());
}
