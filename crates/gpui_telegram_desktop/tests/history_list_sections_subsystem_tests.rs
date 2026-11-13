//! Subsystem Tests for History List Widget, Chat Sections & Navigation Bars
//!
//! Validates Mandate #18 anti-bluffing formulas, virtual scrolling math,
//! pinned message cyclical tracking, scheduling state machine, and translation bar.

use std::collections::HashSet;
use std::time::{Duration, Instant};

use gpui_telegram_desktop::views::stage::chat_section::{
    calculate_keyboard_reserve, is_special_show_at_msg_id, resolve_highlight_id,
    ChatMemento, FullMsgId, HistoryViewChatSectionState, SendPermission,
    K_PSA_ABOUT_PREFIX, K_SCROLL_TO_VOICE_AFTER_SCROLLED_MS,
    K_SHOW_MEMBERS_DROPDOWN_TIMEOUT_MS, SHOW_AND_MAYBE_START_BOT_MSG_ID,
    SHOW_AND_START_BOT_MSG_ID, SHOW_AT_THE_END_MSG_ID, SHOW_FOR_CHOOSE_MESSAGES_MSG_ID,
};
use gpui_telegram_desktop::views::stage::list_widget::{
    hides_dates, supports_read_metrics, HistoryListContext, HistoryViewListWidgetState,
    MessageSelectionManager, StickyDateBadgeState, VirtualListGeometry, VirtualMessageItem,
    K_CLEAR_USERPICS_AFTER, K_PRELOADED_SCREENS_COUNT, K_PRELOADED_SCREENS_COUNT_FULL,
    K_PRELOAD_IF_LESS_THAN_SCREENS, K_SCROLL_DATE_HIDE_ON_DAY_CROSSING_TIMEOUT,
};
use gpui_telegram_desktop::views::stage::pinned_bar::{
    format_pinned_title, project_universal_msg_id, HistoryViewPinnedBarState,
    PinnedSectionMemento, PinnedTracker, K_CHANGE_VIEWER_LIMIT, K_FULL_LOADED,
    K_LOADED_LIMIT, K_NOT_LOADED, K_SOME_LOADED, SERVER_MAX_MSG_ID,
};
use gpui_telegram_desktop::views::stage::scheduled_section::{
    HistoryViewScheduleBoxState, HistoryViewScheduledSectionState, ScheduleBoxType,
    ScheduleRepeatPeriod, ScheduledMessageItem, API_SCHEDULED_UNTIL_ONLINE_TIMESTAMP,
    K_NOTIFY_LAST_FRAME, K_NOTIFY_REST_FRAME, K_NOTIFY_TOOLTIP_DURATION,
    K_VIDEO_PROCESSING_INFO_DURATION,
};
use gpui_telegram_desktop::views::stage::top_bar::{
    has_group_call_menu, resolve_top_bar_title, HistoryViewTopBarWidgetState,
    TopBarAction, TopBarSection, K_EMOJI_INTERACTION_SEEN_DURATION,
    SWITCH_TO_CHOOSE_FROM_QUERY,
};
use gpui_telegram_desktop::views::stage::translate_bar::{
    HistoryViewTranslateBarState, TranslateTracker, K_ENOUGH_FOR_RECOGNITION,
    K_ENOUGH_FOR_TRANSLATION, K_MAX_CHECK_IN_BUNCH, K_REQUEST_COUNT_LIMIT,
    K_REQUEST_LENGTH_LIMIT, K_TOAST_DURATION,
};
use gpui_telegram_desktop::views::stage::view::history_view_context_menu::{
    HistoryMenuAction, HistoryViewContextMenuView, K_MIN_VIEWS_COUNT,
    K_PUBLIC_POST_LINK_TOAST_DURATION, K_RESCHEDULE_LIMIT, K_TAG_NAME_LIMIT,
};
use gpui_telegram_desktop::views::stage::view::history_view_empty_list_bubble::{
    calculate_empty_bubble_position, HistoryViewEmptyListBubbleView,
};
use gpui_telegram_desktop::views::stage::view::history_view_group_call_bar::{
    calculate_avatar_stack_width, HistoryViewGroupCallBarView,
    K_GROUP_CALL_USERPICS_LIMIT,
};
use gpui_telegram_desktop::views::stage::view::history_view_group_members_widget::{
    GroupMemberRow, HistoryViewGroupMembersWidgetView,
};
use gpui_telegram_desktop::views::stage::view::history_view_read_metrics_tracker::{
    HistoryViewReadMetricsTrackerView, K_READ_METRICS_ACTIVITY_TIMEOUT,
    K_READ_METRICS_GRACE_PERIOD, K_READ_METRICS_MAX_TRACKING_DURATION,
    K_READ_METRICS_MIN_REPORT_THRESHOLD,
};
use gpui_telegram_desktop::views::stage::view::history_view_requests_bar::{
    HistoryViewRequestsBarView, PendingJoinUser,
};
use gpui_telegram_desktop::views::stage::view::history_view_subsection_tabs::{
    HistoryViewSubsectionTabsView, SubsectionTabsMode, TopicTabItem,
    K_DEFAULT_SUBSECTION_TABS_LIMIT,
};
use gpui_telegram_desktop::views::stage::view::history_view_top_peers_selector::{
    HistoryViewTopPeersSelectorView, TopPeerCandidate, K_MAX_TOP_PEERS, K_TOP_PEER_SHIFT,
};
use gpui_telegram_desktop::views::stage::view::history_view_welcome_messages_section::{
    HistoryViewWelcomeMessagesSectionView, WelcomeMessageItem,
};

#[test]
fn test_virtual_list_geometry_scrolling_math() {
    // 1. Verify upstream anonymous-namespace constants from history_view_list_widget.cpp:L112-L118
    assert_eq!(K_PRELOADED_SCREENS_COUNT, 4);
    assert_eq!(K_PRELOAD_IF_LESS_THAN_SCREENS, 2);
    assert_eq!(K_PRELOADED_SCREENS_COUNT_FULL, 9);
    assert_eq!(K_CLEAR_USERPICS_AFTER, 50);
    assert_eq!(K_SCROLL_DATE_HIDE_ON_DAY_CROSSING_TIMEOUT, Duration::from_millis(3000));

    // 2. Viewport height = 600px, content = 10,000px, scrolled to 2000px
    let geo = VirtualListGeometry::new(2000.0, 600.0, 10000.0);
    assert_eq!(geo.viewport_top, 2000.0);
    assert_eq!(geo.viewport_bottom(), 2600.0);

    // Preloaded bounds: 4 screens above (4 * 600 = 2400 -> clamped to 0) and 4 screens below (2600 + 2400 = 5000)
    assert_eq!(geo.preloaded_top(), 0.0);
    assert_eq!(geo.preloaded_bottom(), 5000.0);

    // Generate 100 items each 100px high
    let mut items = Vec::new();
    for i in 0..100 {
        items.push(VirtualMessageItem {
            id: i as i64 + 1,
            top: i as f32 * 100.0,
            height: 100.0,
            timestamp: 1700000000 + i as i64 * 60,
            is_unread: false,
            is_selected: false,
        });
    }

    // Visible range for [2000.0, 2600.0) -> items with tops in [2000, 2600) -> indices 20..26
    let (v_start, v_end) = geo.calculate_visible_range(&items);
    assert_eq!(v_start, 20);
    assert_eq!(v_end, 26);

    // Preloaded range for [0.0, 5000.0) -> indices 0..50
    let (p_start, p_end) = geo.calculate_preloaded_range(&items);
    assert_eq!(p_start, 0);
    assert_eq!(p_end, 50);

    // Test prefetch triggers (2 screens = 1200px threshold)
    let near_top_geo = VirtualListGeometry::new(800.0, 600.0, 10000.0);
    assert!(near_top_geo.should_preload_top());
    assert!(!near_top_geo.should_preload_bottom());

    let near_bottom_geo = VirtualListGeometry::new(9000.0, 600.0, 10000.0);
    assert!(!near_bottom_geo.should_preload_top());
    assert!(near_bottom_geo.should_preload_bottom());
}

#[test]
fn test_virtual_list_selection_and_scroll_to_message() {
    let mut list = HistoryViewListWidgetState::new(HistoryListContext::History, 800.0);
    let mut items = Vec::new();
    for i in 0..50 {
        items.push(VirtualMessageItem {
            id: 100 + i as i64,
            top: i as f32 * 80.0,
            height: 80.0,
            timestamp: 1700000000,
            is_unread: false,
            is_selected: false,
        });
    }
    list.set_items(items);
    assert_eq!(list.geometry.content_height, 4000.0);

    // Scroll to message 120 (top = 20 * 80 = 1600.0, height = 80.0)
    // Centered: 1600.0 - (800.0 - 80.0) / 2 = 1600.0 - 360.0 = 1240.0
    let ok = list.scroll_to_message(120);
    assert!(ok);
    assert_eq!(list.geometry.viewport_top, 1240.0);
    assert_eq!(list.scroll_to_item_requested, Some(120));

    // Standalone selection manager test
    let mut standalone_sel = MessageSelectionManager::default();
    assert!(!standalone_sel.is_selected(42));
    standalone_sel.toggle_selection(42);
    assert!(standalone_sel.is_selected(42));

    // Selection toggle & range
    assert!(!list.selection.is_selection_mode);
    assert!(list.selection.toggle_selection(105));
    assert!(list.selection.is_selection_mode);
    assert_eq!(list.selection.count(), 1);

    // Range select 105 to 108
    list.selection.select_range(&list.items, 105, 108);
    assert_eq!(list.selection.count(), 4);
    assert!(list.selection.selected_ids.contains(&105));
    assert!(list.selection.selected_ids.contains(&106));
    assert!(list.selection.selected_ids.contains(&107));
    assert!(list.selection.selected_ids.contains(&108));

    list.selection.clear();
    assert_eq!(list.selection.count(), 0);
    assert!(!list.selection.is_selection_mode);

    // Context properties
    assert!(!hides_dates(HistoryListContext::History));
    assert!(hides_dates(HistoryListContext::ShortcutMessages));
    assert!(hides_dates(HistoryListContext::WelcomeMessages));

    assert!(supports_read_metrics(HistoryListContext::History));
    assert!(supports_read_metrics(HistoryListContext::Replies));
    assert!(!supports_read_metrics(HistoryListContext::Pinned));
}

#[test]
fn test_sticky_date_badge_day_crossing_timeout() {
    let mut badge = StickyDateBadgeState::new();
    assert!(!badge.is_visible);

    let t0 = Instant::now();
    badge.update_date("November 7".to_string(), t0);
    assert!(badge.is_visible);
    assert_eq!(badge.date_text, "November 7");

    // After 2 seconds: still visible
    let t1 = t0 + Duration::from_millis(2000);
    badge.check_timeout(t1);
    assert!(badge.is_visible);

    // After 3.5 seconds: hidden
    let t2 = t0 + Duration::from_millis(3500);
    badge.check_timeout(t2);
    assert!(!badge.is_visible);
}

#[test]
fn test_chat_section_memento_and_special_msg_ids() {
    assert_eq!(K_SHOW_MEMBERS_DROPDOWN_TIMEOUT_MS, 300);
    assert_eq!(K_SCROLL_TO_VOICE_AFTER_SCROLLED_MS, Duration::from_millis(1000));
    assert_eq!(K_PSA_ABOUT_PREFIX, "cloud_lng_about_psa_");

    // Special IDs
    assert!(is_special_show_at_msg_id(SHOW_AT_THE_END_MSG_ID));
    assert!(is_special_show_at_msg_id(SHOW_AND_START_BOT_MSG_ID));
    assert!(is_special_show_at_msg_id(SHOW_AND_MAYBE_START_BOT_MSG_ID));
    assert!(is_special_show_at_msg_id(SHOW_FOR_CHOOSE_MESSAGES_MSG_ID));
    assert!(!is_special_show_at_msg_id(12345));

    // ChatMemento initialization
    let m_special = ChatMemento::new(1001, None, SHOW_FOR_CHOOSE_MESSAGES_MSG_ID);
    assert_eq!(m_special.highlight_id, 0); // Special ID mapped to 0
    assert!(m_special.activate_choose_for_report);

    let m_bot = ChatMemento::new(1001, None, SHOW_AND_START_BOT_MSG_ID);
    assert!(m_bot.send_bot_start);

    // Highlight resolution with migrated peer ID
    let regular = resolve_highlight_id(2001, Some(3001), 50);
    assert_eq!(regular, FullMsgId { peer_id: 2001, msg_id: 50 });

    let migrated = resolve_highlight_id(2001, Some(3001), -777);
    assert_eq!(migrated, FullMsgId { peer_id: 3001, msg_id: 777 });

    // Keyboard reserve calculation
    // keyboard_height = 200, max_field_height = 300 -> half_max = 150 -> reserve = 150
    assert_eq!(calculate_keyboard_reserve(200.0, 300.0), 150.0);
    // keyboard_height = 100, max_field_height = 300 -> reserve = 100
    assert_eq!(calculate_keyboard_reserve(100.0, 300.0), 100.0);

    // Section state permissions
    let mut state = HistoryViewChatSectionState::new(500, Some(10));
    assert!(state.can_send(SendPermission::Anything));
    assert!(state.can_send(SendPermission::Files));

    state.is_read_only = true;
    assert!(!state.can_send(SendPermission::Anything));
    assert!(!state.can_send(SendPermission::Files));
}

#[test]
fn test_top_bar_title_resolution_and_group_call_menu() {
    assert_eq!(K_EMOJI_INTERACTION_SEEN_DURATION, Duration::from_millis(3000));
    assert_eq!(SWITCH_TO_CHOOSE_FROM_QUERY, "from:");

    // Title resolution
    assert_eq!(resolve_top_bar_title("Alice", false, false, false), "Alice");
    assert_eq!(resolve_top_bar_title("Saved Messages", true, true, false), "My Notes");
    assert_eq!(resolve_top_bar_title("Hidden User", true, false, true), "Messages from Hidden Users");

    // Group call menu eligibility: !is_user && !has_active_call && can_manage
    assert!(has_group_call_menu(false, false, true));
    assert!(!has_group_call_menu(true, false, true)); // user chat cannot have group call
    assert!(!has_group_call_menu(false, true, true)); // already in active call
    assert!(!has_group_call_menu(false, false, false)); // no admin permissions

    // Top bar state
    let mut bar = HistoryViewTopBarWidgetState::new(100, "Rust Developers".to_string(), false);
    bar.can_manage_group_call = true;
    assert!(bar.can_show_group_call_button());

    assert_eq!(bar.section, TopBarSection::Normal);
    bar.set_selection_mode(3);
    assert_eq!(bar.section, TopBarSection::Selection);
    assert_eq!(bar.selected_count, 3);

    bar.set_selection_mode(0);
    assert_eq!(bar.section, TopBarSection::Normal);

    assert!(!bar.info_drawer_open);
    assert!(bar.toggle_info_drawer());
    assert!(bar.info_drawer_open);
}

#[test]
fn test_pinned_bar_and_tracker_cycling() {
    assert_eq!(K_FULL_LOADED, 2);
    assert_eq!(K_SOME_LOADED, 1);
    assert_eq!(K_NOT_LOADED, 0);
    assert_eq!(K_LOADED_LIMIT, 5);
    assert_eq!(K_CHANGE_VIEWER_LIMIT, 2);
    assert_eq!(SERVER_MAX_MSG_ID, 0x3FFFFFFF);

    // Title formatting parity:
    // 1 item
    assert_eq!(format_pinned_title(0, 1), "Pinned Message");
    // 2 items: index 0 -> Previous, index 1 -> Pinned
    assert_eq!(format_pinned_title(0, 2), "Previous Pinned Message");
    assert_eq!(format_pinned_title(1, 2), "Pinned Message");
    // 4 items: index 0 -> #1, index 1 -> #2, index 3 -> Pinned
    assert_eq!(format_pinned_title(0, 4), "Pinned Message #1");
    assert_eq!(format_pinned_title(1, 4), "Pinned Message #2");
    assert_eq!(format_pinned_title(2, 4), "Pinned Message #3");
    assert_eq!(format_pinned_title(3, 4), "Pinned Message");

    // Universal message ID projection
    assert_eq!(project_universal_msg_id(true, 500), 500); // channel: unchanged
    assert_eq!(project_universal_msg_id(false, SERVER_MAX_MSG_ID + 50), 50); // basic group: offset by boundary

    // Cyclical pinned tracker
    let mut tracker = PinnedTracker::new(true);
    tracker.set_pinned_ids(vec![101, 102, 103]);
    assert_eq!(tracker.current_msg_id(), Some(101));

    // Next click cycles forward: 101 -> 102 -> 103 -> 101
    assert_eq!(tracker.next_pinned_id(), Some(102));
    assert_eq!(tracker.next_pinned_id(), Some(103));
    assert_eq!(tracker.next_pinned_id(), Some(101));

    // Pinned bar widget state
    let mut bar = HistoryViewPinnedBarState::new(true);
    bar.tracker.set_pinned_ids(vec![201, 202]);
    bar.update_content("Important announcement".to_string(), true, false);
    assert_eq!(bar.title, "Previous Pinned Message");

    let clicked = bar.on_bar_clicked();
    assert_eq!(clicked, Some(202));
    assert_eq!(bar.title, "Pinned Message");
}

#[test]
fn test_scheduled_section_and_schedule_box() {
    assert_eq!(K_VIDEO_PROCESSING_INFO_DURATION, Duration::from_millis(4000));
    assert_eq!(K_NOTIFY_REST_FRAME, 40);
    assert_eq!(K_NOTIFY_LAST_FRAME, 80);
    assert_eq!(K_NOTIFY_TOOLTIP_DURATION, Duration::from_millis(5000));
    assert_eq!(API_SCHEDULED_UNTIL_ONLINE_TIMESTAMP, 0x7FFFFFFF);

    // ScheduleBox: Until Online only valid for ScheduledToUser
    let mut box_group = HistoryViewScheduleBoxState::new(ScheduleBoxType::SilentOnly, true);
    assert!(!box_group.set_until_online());

    let mut box_user = HistoryViewScheduleBoxState::new(ScheduleBoxType::ScheduledToUser, false);
    assert!(box_user.set_until_online());
    assert_eq!(box_user.scheduled_timestamp, API_SCHEDULED_UNTIL_ONLINE_TIMESTAMP);

    // Repeat period requires premium
    let mut free_box = HistoryViewScheduleBoxState::new(ScheduleBoxType::ScheduledToUser, false);
    assert!(!free_box.set_repeat_period(ScheduleRepeatPeriod::Daily));

    let mut prem_box = HistoryViewScheduleBoxState::new(ScheduleBoxType::ScheduledToUser, true);
    assert!(prem_box.set_repeat_period(ScheduleRepeatPeriod::Daily));
    assert_eq!(prem_box.repeat, ScheduleRepeatPeriod::Daily);

    // Scheduled section operations
    let mut section = HistoryViewScheduledSectionState::new(100, None);
    section.items.push(ScheduledMessageItem {
        id: 1,
        text: "Hello future".to_string(),
        scheduled_at: 1750000000,
        is_until_online: false,
        is_silent: false,
        repeat: ScheduleRepeatPeriod::None,
        has_video: false,
    });
    section.items.push(ScheduledMessageItem {
        id: 2,
        text: "Online greeting".to_string(),
        scheduled_at: API_SCHEDULED_UNTIL_ONLINE_TIMESTAMP,
        is_until_online: true,
        is_silent: true,
        repeat: ScheduleRepeatPeriod::None,
        has_video: true,
    });

    assert_eq!(section.items.len(), 2);
    let sent = section.send_now(1);
    assert!(sent.is_some());
    assert_eq!(sent.unwrap().id, 1);
    assert_eq!(section.items.len(), 1);

    assert!(section.reschedule(2, 1760000000));
    assert_eq!(section.items[0].scheduled_at, 1760000000);
    assert!(!section.items[0].is_until_online);
}

#[test]
fn test_translate_bar_and_tracker_language_detection() {
    assert_eq!(K_ENOUGH_FOR_RECOGNITION, 10);
    assert_eq!(K_ENOUGH_FOR_TRANSLATION, 6);
    assert_eq!(K_MAX_CHECK_IN_BUNCH, 100);
    assert_eq!(K_REQUEST_LENGTH_LIMIT, 24576);
    assert_eq!(K_REQUEST_COUNT_LIMIT, 20);
    assert_eq!(K_TOAST_DURATION, Duration::from_millis(4000));

    let mut tracker = TranslateTracker::new(500, false);
    assert!(!tracker.feed_text("Hi")); // 2 chars < 10
    assert!(tracker.feed_text("This is an English message")); // > 10 chars -> triggers recognition
    tracker.set_detected_language("en".to_string());

    let mut skip = HashSet::new();
    skip.insert("es".to_string());

    // Target = "zh", detected = "en" -> offer translation
    assert!(tracker.should_offer_translation("zh", &skip, false));
    // Target = "en", detected = "en" -> same language, don't offer
    assert!(!tracker.should_offer_translation("en", &skip, false));
    // Peer disabled translation -> don't offer
    assert!(!tracker.should_offer_translation("zh", &skip, true));

    // If "en" is added to skip list -> don't offer
    skip.insert("en".to_string());
    assert!(!tracker.should_offer_translation("zh", &skip, false));

    // Translate bar lifecycle
    let mut bar = HistoryViewTranslateBarState::new("zh".to_string());
    skip.remove("en");
    tracker.set_detected_language("en".to_string());
    bar.update_from_tracker(&tracker, false);
    assert!(bar.is_visible);
    assert_eq!(bar.from_lang.as_deref(), Some("en"));

    bar.trigger_translation();
    assert!(bar.is_translating);
    bar.complete_translation();
    assert!(!bar.is_translating);
    assert!(bar.is_translated);
    assert!(bar.show_toast);

    bar.revert_to_original();
    assert!(!bar.is_translated);
}

#[test]
fn test_ancillary_navigation_bars_and_widgets() {
    // 1. Group call avatar overlap calculation
    assert_eq!(K_GROUP_CALL_USERPICS_LIMIT, 3);
    // single = 28.0, shift = 8.0
    // count 1: 28.0
    assert_eq!(calculate_avatar_stack_width(1, 28.0, 8.0), 28.0);
    // count 2: 28.0 + 1 * (28.0 - 8.0) = 48.0
    assert_eq!(calculate_avatar_stack_width(2, 28.0, 8.0), 48.0);
    // count 3: 28.0 + 2 * 20.0 = 68.0
    assert_eq!(calculate_avatar_stack_width(3, 28.0, 8.0), 68.0);
    // count 5: clamped to limit 3 -> 68.0
    assert_eq!(calculate_avatar_stack_width(5, 28.0, 8.0), 68.0);

    // 2. Empty list bubble centered positioning: x = (w - bw)/2, y = (h - bh)/3
    let (bx, by) = calculate_empty_bubble_position(800.0, 600.0, 200.0, 60.0);
    assert_eq!(bx, 300.0); // (800 - 200)/2
    assert_eq!(by, 180.0); // (600 - 60)/3

    // 3. Top peers selector: max 5, self first, bots excluded
    let mut selector = HistoryViewTopPeersSelectorView::new(999);
    let candidates = vec![
        TopPeerCandidate { peer_id: 101, name: "Bob".to_string(), is_user: true, is_bot: false, is_self: false },
        TopPeerCandidate { peer_id: 102, name: "BotFather".to_string(), is_user: true, is_bot: true, is_self: false },
        TopPeerCandidate { peer_id: 103, name: "Charlie".to_string(), is_user: true, is_bot: false, is_self: false },
        TopPeerCandidate { peer_id: 104, name: "David".to_string(), is_user: true, is_bot: false, is_self: false },
        TopPeerCandidate { peer_id: 105, name: "Eve".to_string(), is_user: true, is_bot: false, is_self: false },
        TopPeerCandidate { peer_id: 106, name: "Frank".to_string(), is_user: true, is_bot: false, is_self: false },
    ];
    selector.collect_peers(&candidates);
    assert_eq!(selector.candidates.len(), 5); // capped at K_MAX_TOP_PEERS = 5
    assert_eq!(selector.candidates[0].peer_id, 999); // Self always first
    assert_eq!(selector.candidates[1].peer_id, 101); // Bob
    assert_eq!(selector.candidates[2].peer_id, 103); // Charlie (BotFather skipped!)
    assert_eq!(selector.candidates[3].peer_id, 104); // David
    assert_eq!(selector.candidates[4].peer_id, 105); // Eve

    // 4. Requests bar dismiss
    let mut req_bar = HistoryViewRequestsBarView::new(100, true);
    req_bar.set_pending_users(vec![
        PendingJoinUser { user_id: 1, name: "User1".to_string(), date: 1000 },
        PendingJoinUser { user_id: 2, name: "User2".to_string(), date: 1001 },
    ]);
    assert!(req_bar.is_visible);
    assert_eq!(req_bar.count(), 2);
    let dismissed = req_bar.dismiss_request(1);
    assert_eq!(dismissed.unwrap().user_id, 1);
    assert_eq!(req_bar.count(), 1);
    req_bar.dismiss_request(2);
    assert!(!req_bar.is_visible);

    // 5. Read metrics tracker constants & recording
    let mut metrics = HistoryViewReadMetricsTrackerView::new(777);
    assert_eq!(metrics.grace_period(), K_READ_METRICS_GRACE_PERIOD);
    assert_eq!(metrics.activity_timeout(), K_READ_METRICS_ACTIVITY_TIMEOUT);
    assert_eq!(metrics.max_tracking_duration(), K_READ_METRICS_MAX_TRACKING_DURATION);
    assert_eq!(K_READ_METRICS_MIN_REPORT_THRESHOLD, Duration::from_millis(300));

    // Visible for 200ms < 300ms threshold -> not ready for report
    metrics.record_visibility(10, Duration::from_millis(200));
    assert!(!metrics.is_ready_for_report(10));

    // Visible for additional 150ms -> cumulative 350ms >= 300ms -> ready
    metrics.record_visibility(10, Duration::from_millis(150));
    assert!(metrics.is_ready_for_report(10));
    metrics.mark_reported(10);
    assert!(!metrics.is_ready_for_report(10));

    // 6. Forum subsection tabs
    assert_eq!(K_DEFAULT_SUBSECTION_TABS_LIMIT, 12);
    let mut tabs_view = HistoryViewSubsectionTabsView::new(333, SubsectionTabsMode::Top);
    tabs_view.set_tabs(vec![
        TopicTabItem { topic_id: 1, title: "General".to_string(), icon_id: None, unread_count: 0, is_closed: false },
        TopicTabItem { topic_id: 2, title: "Dev".to_string(), icon_id: Some(10), unread_count: 5, is_closed: false },
    ]);
    assert_eq!(tabs_view.default_limit(), 12);
    assert!(tabs_view.select_topic(2));
    assert_eq!(tabs_view.active_topic_id, Some(2));
    assert!(!tabs_view.select_topic(999));

    // 7. Welcome messages section
    let mut welcome_sec = HistoryViewWelcomeMessagesSectionView::new(444);
    assert!(welcome_sec.is_enabled);
    welcome_sec.set_messages(vec![
        WelcomeMessageItem { id: 1, text: "Welcome to our store!".to_string(), is_away_greeting: false, recipient_filter: "all".to_string() }
    ]);
    assert_eq!(welcome_sec.count(), 1);
    assert!(!welcome_sec.toggle_enabled());

    // 8. Group members inline widget
    let mut members_widget = HistoryViewGroupMembersWidgetView::new(555);
    assert!(!members_widget.is_open);
    members_widget.set_members(vec![
        GroupMemberRow { user_id: 1, name: "Alice".to_string(), is_admin: true, has_stories: true, online_status: "online".to_string() },
        GroupMemberRow { user_id: 2, name: "Bob".to_string(), is_admin: false, has_stories: false, online_status: "offline".to_string() },
    ]);
    assert_eq!(members_widget.count(), 2);
    assert!(members_widget.toggle());
    assert!(members_widget.is_open);

    // 9. Context menu actions & constants
    assert_eq!(K_MIN_VIEWS_COUNT, 10);
    assert_eq!(K_PUBLIC_POST_LINK_TOAST_DURATION, Duration::from_millis(4000));
    assert_eq!(K_RESCHEDULE_LIMIT, 20);
    assert_eq!(K_TAG_NAME_LIMIT, 12);
    let menu = HistoryViewContextMenuView::new(50, 555, true);
    assert!(menu.available_actions.contains(&HistoryMenuAction::Reply));
    assert!(menu.available_actions.contains(&HistoryMenuAction::Edit));
    assert!(menu.available_actions.contains(&HistoryMenuAction::Pin));

    // 10. Empty bubble view & call bar view & pinned memento
    let empty_bubble = HistoryViewEmptyListBubbleView::new("No messages yet".to_string());
    assert_eq!(empty_bubble.text, "No messages yet");
    assert!(empty_bubble.is_visible);

    let call_bar = HistoryViewGroupCallBarView::new(1, 666, "Weekly Standup".to_string());
    assert_eq!(call_bar.peer_id, 666);
    assert_eq!(K_MAX_TOP_PEERS, 5);
    assert_eq!(K_TOP_PEER_SHIFT, 0.15);

    let pinned_memento = PinnedSectionMemento::new(777, None, Some(100));
    assert_eq!(pinned_memento.peer_id, 777);
    assert_eq!(pinned_memento.highlight_msg_id, Some(100));

    let _top_bar_action = TopBarAction::Search;
}
