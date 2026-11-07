//! Unit & Subsystem Tests for History Slice 3: History View Element Base & Core Message Layout
//! Upstream reference: `Telegram/SourceFiles/history/view/`
//! Mandate #18 & #19 Verification Suite

#![allow(unused_imports, unused_mut)]

use gpui_telegram_desktop::views::stage::bottom_info::{
    format_views_count, schedule_period_text, BottomInfoData, BottomInfoFlags, BottomInfoView,
};
use gpui_telegram_desktop::views::stage::history_element::{
    should_attach_to_previous, BubblePadding, BubbleRounding, ElementChatMode, ElementContext,
    ElementFlags, ElementMargins, HistoryElementBase, BUBBLE_RADIUS_LARGE, BUBBLE_RADIUS_SMALL,
    K_ATTACH_MESSAGE_TO_PREVIOUS_SECONDS_DELTA, K_MAX_SHOWN_LINE, MSG_MAX_WIDTH, MSG_MIN_WIDTH,
    MSG_PHOTO_SIZE, MSG_PHOTO_SKIP,
};
use gpui_telegram_desktop::views::stage::history_message::{
    MessageBubbleData, MessageBubbleView, MessageDeliveryStatus,
};
use gpui_telegram_desktop::views::stage::history_service::{
    ServiceMessageBubbleView, ServiceMessageData, ServiceMessageKind, MSG_SERVICE_MARGIN_BOTTOM,
    MSG_SERVICE_MARGIN_TOP, MSG_SERVICE_PADDING_BOTTOM, MSG_SERVICE_PADDING_LEFT,
    MSG_SERVICE_PADDING_RIGHT, MSG_SERVICE_PADDING_TOP, WIDE_CHAT_WIDTH,
};
use gpui_telegram_desktop::views::stage::reply_view::{
    ReplyPreviewData, ReplyPreviewView, K_NON_EXPANDED_LINES_LIMIT,
};
use gpui_telegram_desktop::views::stage::rich_quote::{RichQuoteData, RichQuoteView};

// Target View modules
use gpui_telegram_desktop::views::stage::view::history_view_bottom_info::HistoryViewBottomInfoView;
use gpui_telegram_desktop::views::stage::view::history_view_copy_selection::{
    CopySelectionPayload, HistoryViewCopySelectionView, K_MAX_RECOGNITIONS_IN_FLIGHT,
};
use gpui_telegram_desktop::views::stage::view::history_view_corner_buttons::{
    CornerButtonType, HistoryViewCornerButtonsView, K_STASH_ARROW_DURATION_MS,
    K_STASH_BOUNCE_DURATION_MS, K_STASH_DELAY_MS, K_STASH_DURATION_MS,
};
use gpui_telegram_desktop::views::stage::view::history_view_cursor_state::{
    CursorState, HistoryViewCursorStateView, PointState, TextState,
};
use gpui_telegram_desktop::views::stage::view::history_view_drag::{
    HistoryViewDragView, PhotoDragData,
};
use gpui_telegram_desktop::views::stage::view::history_view_draw_to_reply::HistoryViewDrawToReplyView;
use gpui_telegram_desktop::views::stage::view::history_view_element::HistoryViewElementView;
use gpui_telegram_desktop::views::stage::view::history_view_element_overlay::HistoryViewElementOverlayView;
use gpui_telegram_desktop::views::stage::view::history_view_empty_list_bubble::{
    calculate_empty_bubble_position, HistoryViewEmptyListBubbleView,
};
use gpui_telegram_desktop::views::stage::view::history_view_fake_items::HistoryViewFakeItemsView;
use gpui_telegram_desktop::views::stage::view::history_view_keyboard_text_selection::{
    HistoryViewKeyboardTextSelectionView, SelectionKey, TextSelectionRange,
};
use gpui_telegram_desktop::views::stage::view::history_view_message::HistoryViewMessageView;
use gpui_telegram_desktop::views::stage::view::history_view_object::{
    HistoryViewObjectView, LayoutSize,
};
use gpui_telegram_desktop::views::stage::view::history_view_reply::HistoryViewReplyView;
use gpui_telegram_desktop::views::stage::view::history_view_reply_button::{
    HistoryViewReplyButtonView, ReplyButtonState, K_BUTTON_HIDE_DELAY_MS, K_BUTTON_SHOW_DELAY_MS,
    K_TOGGLE_DURATION_MS, REPLY_CORNER_HEIGHT,
};
use gpui_telegram_desktop::views::stage::view::history_view_rich_quote::HistoryViewRichQuoteView;
use gpui_telegram_desktop::views::stage::view::history_view_service_message::HistoryViewServiceMessageView;
use gpui_telegram_desktop::views::stage::view::history_view_summary_header::HistoryViewSummaryHeaderView;
use gpui_telegram_desktop::views::stage::view::history_view_text_helper::HistoryViewTextHelperView;

// ============================================================================
// 1. History Element Base & Metrics Tests
// ============================================================================

#[test]
fn test_history_element_constants_mandate_18() {
    assert_eq!(K_ATTACH_MESSAGE_TO_PREVIOUS_SECONDS_DELTA, 900);
    assert_eq!(K_MAX_SHOWN_LINE, 1024 * 1024);
    assert_eq!(MSG_MAX_WIDTH, 430.0);
    assert_eq!(MSG_MIN_WIDTH, 160.0);
    assert_eq!(MSG_PHOTO_SIZE, 33.0);
    assert_eq!(MSG_PHOTO_SKIP, 40.0);
    assert_eq!(BUBBLE_RADIUS_LARGE, 16.0);
    assert_eq!(BUBBLE_RADIUS_SMALL, 4.0);
}

#[test]
fn test_element_flags_bitfield_operations() {
    let mut flags = ElementFlags::new();
    assert!(!flags.contains(ElementFlags::SERVICE_MESSAGE));
    assert!(!flags.contains(ElementFlags::ATTACHED_TO_PREVIOUS));

    flags.insert(ElementFlags::SERVICE_MESSAGE);
    flags.insert(ElementFlags::NEEDS_RESIZE);
    assert!(flags.contains(ElementFlags::SERVICE_MESSAGE));
    assert!(flags.contains(ElementFlags::NEEDS_RESIZE));

    flags.remove(ElementFlags::NEEDS_RESIZE);
    assert!(!flags.contains(ElementFlags::NEEDS_RESIZE));

    flags.set(ElementFlags::ATTACHED_TO_PREVIOUS, true);
    assert!(flags.contains(ElementFlags::ATTACHED_TO_PREVIOUS));
    flags.set(ElementFlags::ATTACHED_TO_PREVIOUS, false);
    assert!(!flags.contains(ElementFlags::ATTACHED_TO_PREVIOUS));
}

#[test]
fn test_should_attach_to_previous_logic() {
    let sender1 = 1001u64;
    let sender2 = 1002u64;
    let base_time = 1700000000i64;

    // Same sender within 900s (15 min) -> true
    assert!(should_attach_to_previous(
        Some(sender1),
        sender1,
        Some(base_time),
        base_time + 899
    ));
    assert!(should_attach_to_previous(
        Some(sender1),
        sender1,
        Some(base_time),
        base_time + 900
    ));

    // Same sender > 900s -> false
    assert!(!should_attach_to_previous(
        Some(sender1),
        sender1,
        Some(base_time),
        base_time + 901
    ));

    // Different sender within delta -> false
    assert!(!should_attach_to_previous(
        Some(sender1),
        sender2,
        Some(base_time),
        base_time + 100
    ));

    // None previous sender or date -> false
    assert!(!should_attach_to_previous(
        None,
        sender1,
        Some(base_time),
        base_time + 100
    ));
    assert!(!should_attach_to_previous(
        Some(sender1),
        sender1,
        None,
        base_time + 100
    ));
}

#[test]
fn test_bubble_rounding_incoming_and_outgoing() {
    // Incoming: attachment on left side
    let in_unattached = BubbleRounding::for_message(false, false, false);
    assert_eq!(in_unattached.top_left, 16.0);
    assert_eq!(in_unattached.top_right, 16.0);
    assert_eq!(in_unattached.bottom_left, 16.0);
    assert_eq!(in_unattached.bottom_right, 16.0);

    let in_attached_prev = BubbleRounding::for_message(false, true, false);
    assert_eq!(in_attached_prev.top_left, 4.0);
    assert_eq!(in_attached_prev.top_right, 16.0);
    assert_eq!(in_attached_prev.bottom_left, 16.0);
    assert_eq!(in_attached_prev.bottom_right, 16.0);

    let in_attached_both = BubbleRounding::for_message(false, true, true);
    assert_eq!(in_attached_both.top_left, 4.0);
    assert_eq!(in_attached_both.top_right, 16.0);
    assert_eq!(in_attached_both.bottom_left, 4.0);
    assert_eq!(in_attached_both.bottom_right, 16.0);

    // Outgoing: attachment on right side
    let out_attached_prev = BubbleRounding::for_message(true, true, false);
    assert_eq!(out_attached_prev.top_left, 16.0);
    assert_eq!(out_attached_prev.top_right, 4.0);
    assert_eq!(out_attached_prev.bottom_left, 16.0);
    assert_eq!(out_attached_prev.bottom_right, 16.0);

    let out_attached_both = BubbleRounding::for_message(true, true, true);
    assert_eq!(out_attached_both.top_left, 16.0);
    assert_eq!(out_attached_both.top_right, 4.0);
    assert_eq!(out_attached_both.bottom_left, 16.0);
    assert_eq!(out_attached_both.bottom_right, 4.0);
}

#[test]
fn test_bubble_margins_and_padding_metrics() {
    let unattached_margins = ElementMargins::for_message(false);
    assert_eq!(unattached_margins.left, 16.0);
    assert_eq!(unattached_margins.top, 6.0);
    assert_eq!(unattached_margins.right, 56.0);
    assert_eq!(unattached_margins.bottom, 2.0);

    let attached_margins = ElementMargins::for_message(true);
    assert_eq!(attached_margins.top, 0.0); // Top margin collapses when attached

    let service_margins = ElementMargins::for_service();
    assert_eq!(service_margins.left, 10.0);
    assert_eq!(service_margins.top, 10.0);
    assert_eq!(service_margins.right, 10.0);
    assert_eq!(service_margins.bottom, 2.0);

    let padding = BubblePadding::default();
    assert_eq!(padding.left, 11.0);
    assert_eq!(padding.top, 8.0);
    assert_eq!(padding.right, 11.0);
    assert_eq!(padding.bottom, 8.0);
}

#[test]
fn test_history_element_base_lifecycle() {
    let mut base = HistoryElementBase::new(42, 1001, 1700000000, false);
    assert_eq!(base.item_id, 42);
    assert_eq!(base.sender_id, 1001);
    assert!(!base.is_outgoing);
    assert_eq!(base.context, ElementContext::History);
    assert_eq!(base.chat_mode, ElementChatMode::Default);

    // Initial margins and rounding
    assert_eq!(base.margins.top, 6.0);
    assert_eq!(base.bubble_rounding.top_left, 16.0);

    // Attach to previous
    base.set_attached_to_previous(true);
    assert!(base.flags.contains(ElementFlags::ATTACHED_TO_PREVIOUS));
    assert!(base.flags.contains(ElementFlags::BUBBLE_ATTACHED_TO_PREVIOUS));
    assert_eq!(base.margins.top, 0.0);
    assert_eq!(base.bubble_rounding.top_left, 4.0);

    // Attach to next
    base.set_attached_to_next(true);
    assert!(base.flags.contains(ElementFlags::ATTACHED_TO_NEXT));
    assert_eq!(base.bubble_rounding.bottom_left, 4.0);
}

// ============================================================================
// 2. Message Bubble Layout & Sizing Tests
// ============================================================================

#[test]
fn test_message_bubble_data_layout_clamping() {
    let mut msg = MessageBubbleData::new(
        1,
        101,
        "Alice".to_string(),
        "Short text".to_string(),
        1700000000,
        false,
    );

    // Huge screen: content width should clamp to MSG_MAX_WIDTH (430px)
    let (w1, h1) = msg.calculate_layout(1200.0);
    assert_eq!(w1, MSG_MAX_WIDTH);
    assert!(h1 > 0.0);

    // Small screen: content width should clamp to MSG_MIN_WIDTH (160px)
    let (w2, _h2) = msg.calculate_layout(150.0);
    assert_eq!(w2, MSG_MIN_WIDTH);
}

#[test]
fn test_message_bubble_reply_expands_height() {
    let mut msg_without_reply = MessageBubbleData::new(
        2,
        102,
        "Bob".to_string(),
        "Hello there".to_string(),
        1700000000,
        true,
    );
    let (_w1, h1) = msg_without_reply.calculate_layout(600.0);

    let mut msg_with_reply = msg_without_reply.clone();
    msg_with_reply.reply = Some(ReplyPreviewData::new(
        1,
        "Alice",
        "Original message snippet",
    ));
    let (_w2, h2) = msg_with_reply.calculate_layout(600.0);

    // Reply preview adds 40px (36px base + 4px margin)
    assert!(h2 > h1);
    assert_eq!(h2 - h1, 40.0);
}

#[test]
fn test_message_delivery_status_transitions() {
    let mut msg_view = HistoryViewMessageView::with_message(
        3,
        103,
        "Charlie",
        "Testing delivery status",
        1700000000,
        true,
    );
    assert_eq!(msg_view.bubble.delivery_status, MessageDeliveryStatus::Sent);

    msg_view.set_delivery_status(MessageDeliveryStatus::Pending);
    assert_eq!(
        msg_view.bubble.delivery_status,
        MessageDeliveryStatus::Pending
    );

    msg_view.set_delivery_status(MessageDeliveryStatus::Read);
    assert_eq!(msg_view.bubble.delivery_status, MessageDeliveryStatus::Read);
}

// ============================================================================
// 3. Service Message Pill Tests
// ============================================================================

#[test]
fn test_service_message_constants_and_geometry() {
    assert_eq!(WIDE_CHAT_WIDTH, 542.0);
    assert_eq!(MSG_SERVICE_PADDING_LEFT, 12.0);
    assert_eq!(MSG_SERVICE_PADDING_TOP, 3.0);
    assert_eq!(MSG_SERVICE_PADDING_RIGHT, 12.0);
    assert_eq!(MSG_SERVICE_PADDING_BOTTOM, 4.0);
    assert_eq!(MSG_SERVICE_MARGIN_TOP, 10.0);
    assert_eq!(MSG_SERVICE_MARGIN_BOTTOM, 2.0);
}

#[test]
fn test_service_message_kinds_and_formatting() {
    let date_svc = ServiceMessageData::new_date(1700000000, "November 7");
    assert_eq!(date_svc.display_text(), "November 7");

    let action_svc = ServiceMessageData::new_action(10, 1700000000, "Alice joined group via link");
    assert_eq!(action_svc.display_text(), "Alice joined group via link");

    let pin_svc = ServiceMessageData::new_pin(11, 1700000000, "Bob", "Roadmap 2026");
    assert_eq!(pin_svc.display_text(), "Bob pinned \"Roadmap 2026\"");

    let call_svc = ServiceMessageData {
        item_id: 12,
        date: 1700000000,
        kind: ServiceMessageKind::CallEnded {
            duration: "5m 23s".to_string(),
        },
    };
    assert_eq!(call_svc.display_text(), "Call ended (5m 23s)");

    let channel_svc = ServiceMessageData {
        item_id: 13,
        date: 1700000000,
        kind: ServiceMessageKind::ChannelCreated("Dev Updates".to_string()),
    };
    assert_eq!(channel_svc.display_text(), "Channel \"Dev Updates\" created");
}

#[test]
fn test_history_view_service_message_wrapper() {
    let svc_view = HistoryViewServiceMessageView::with_date(1700000000, "Today");
    assert_eq!(svc_view.wide_chat_width(), 542.0);
    assert_eq!(svc_view.service.display_text(), "Today");
}

// ============================================================================
// 4. Bottom Info Layout & Formatting Tests
// ============================================================================

#[test]
fn test_bottom_info_flags_and_time_format() {
    let mut flags = BottomInfoFlags::new();
    flags.insert(BottomInfoFlags::OUT_LAYOUT);
    flags.insert(BottomInfoFlags::EDITED);
    assert!(flags.contains(BottomInfoFlags::OUT_LAYOUT));
    assert!(flags.contains(BottomInfoFlags::EDITED));
    assert!(!flags.contains(BottomInfoFlags::SILENT));

    // Timestamp 1700000000: 1700000000 % 86400 = 80000 -> 80000 / 3600 = 22, 80000 % 3600 = 800 -> 800 / 60 = 13 -> "22:13"
    let info = BottomInfoData {
        date: 1700000000,
        edited_date: Some(1700000100),
        author: Some("Bob".to_string()),
        flags,
        views: Some(1250),
        replies: Some(3),
        forwards_count: Some(1),
    };
    assert_eq!(info.format_time_string(), "22:13");
}

#[test]
fn test_schedule_period_text_mapping() {
    assert_eq!(schedule_period_text(30), "minutely");
    assert_eq!(schedule_period_text(60), "minutely");
    assert_eq!(schedule_period_text(120), "5-minutely");
    assert_eq!(schedule_period_text(300), "5-minutely");
    assert_eq!(schedule_period_text(3600), "daily");
    assert_eq!(schedule_period_text(86400), "daily");
    assert_eq!(schedule_period_text(3 * 86400), "weekly");
    assert_eq!(schedule_period_text(10 * 86400), "biweekly");
    assert_eq!(schedule_period_text(25 * 86400), "monthly");
    assert_eq!(schedule_period_text(60 * 86400), "every 3 months");
    assert_eq!(schedule_period_text(120 * 86400), "every 6 months");
    assert_eq!(schedule_period_text(400 * 86400), "yearly");
}

#[test]
fn test_views_counter_formatting() {
    assert_eq!(format_views_count(42), "42");
    assert_eq!(format_views_count(999), "999");
    assert_eq!(format_views_count(1000), "1K");
    assert_eq!(format_views_count(1200), "1.2K");
    assert_eq!(format_views_count(10500), "10.5K");
    assert_eq!(format_views_count(1000000), "1M");
    assert_eq!(format_views_count(2500000), "2.5M");
}

// ============================================================================
// 5. Rich Quote & Reply Preview Tests
// ============================================================================

#[test]
fn test_rich_quote_builder() {
    let quote = RichQuoteData::new("The only way to do great work is to love what you do.")
        .with_author("Steve Jobs");
    assert_eq!(quote.author_name.as_deref(), Some("Steve Jobs"));
    assert_eq!(
        quote.text,
        "The only way to do great work is to love what you do."
    );

    let view = HistoryViewRichQuoteView::with_quote("Quoted line").with_author("Author");
    assert_eq!(
        view.quote.as_ref().unwrap().author_name.as_deref(),
        Some("Author")
    );
}

#[test]
fn test_reply_preview_data_and_thumbnail() {
    let mut reply = ReplyPreviewData::new(101, "Dave", "Look at this snapshot")
        .with_thumbnail("thumb://local/photo_101.jpg");
    assert!(reply.has_thumbnail);
    assert_eq!(
        reply.thumbnail_url.as_deref(),
        Some("thumb://local/photo_101.jpg")
    );
    assert_eq!(reply.calculate_height(400.0), 40.0);
    assert_eq!(K_NON_EXPANDED_LINES_LIMIT, 5);

    let mut reply_view = HistoryViewReplyView::with_reply(101, "Dave", "Snippet text");
    assert!(!reply_view.reply.is_expanded);
    reply_view.toggle_expanded();
    assert!(reply_view.reply.is_expanded);
}

// ============================================================================
// 6. Corner Buttons & Stash Timing Tests
// ============================================================================

#[test]
fn test_corner_buttons_stash_timing_mandate_18() {
    assert_eq!(K_STASH_DELAY_MS, 150);
    assert_eq!(K_STASH_ARROW_DURATION_MS, 106);
    assert_eq!(K_STASH_BOUNCE_DURATION_MS, 173);
    assert_eq!(K_STASH_DURATION_MS, 429);

    let mut buttons = HistoryViewCornerButtonsView::new();
    assert_eq!(buttons.stash_duration_ms(), 429);

    buttons.update_visibility(CornerButtonType::Down, true);
    assert!(buttons.down_shown);

    buttons.update_visibility(CornerButtonType::Reactions, true);
    assert!(buttons.reactions_shown);

    buttons.update_visibility(CornerButtonType::Stash, true);
    assert!(buttons.stash_shown);
}

#[test]
fn test_corner_buttons_reply_returns_stack() {
    let mut buttons = HistoryViewCornerButtonsView::new();
    assert!(buttons.reply_returns.is_empty());

    buttons.push_reply_return(100);
    buttons.push_reply_return(200);
    buttons.push_reply_return(300);

    assert_eq!(buttons.pop_reply_return(), Some(300));
    assert_eq!(buttons.pop_reply_return(), Some(200));

    buttons.clear_reply_returns();
    assert_eq!(buttons.pop_reply_return(), None);
}

// ============================================================================
// 7. Text Selection & Cursor State Tests
// ============================================================================

#[test]
fn test_text_selection_range_math() {
    let range = TextSelectionRange::new(5, 12);
    assert_eq!(range.start(), 5);
    assert_eq!(range.end(), 12);
    assert_eq!(range.len(), 7);
    assert!(!range.is_empty());

    let inverted = TextSelectionRange::new(12, 5);
    assert_eq!(inverted.start(), 5);
    assert_eq!(inverted.end(), 12);
    assert_eq!(inverted.len(), 7);

    let empty = TextSelectionRange::new(10, 10);
    assert!(empty.is_empty());
    assert_eq!(empty.len(), 0);
}

#[test]
fn test_keyboard_text_selection_extension() {
    let text = "Hello telegram world";
    let mut selector = HistoryViewKeyboardTextSelectionView::new();

    // Extend Right by char
    let r1 = selector
        .extend(SelectionKey::Right, false, text, text.len())
        .unwrap();
    assert_eq!(r1.anchor, 0);
    assert_eq!(r1.focus, 1);

    // Extend Home & End
    let r_end = selector
        .extend(SelectionKey::End, false, text, text.len())
        .unwrap();
    assert_eq!(r_end.focus, text.len());

    let r_home = selector
        .extend(SelectionKey::Home, false, text, text.len())
        .unwrap();
    assert_eq!(r_home.focus, 0);

    // Extend Right by word: "Hello"
    let r_word = selector
        .extend(SelectionKey::Right, true, text, text.len())
        .unwrap();
    assert_eq!(r_word.focus, 5); // index after "Hello"
}

#[test]
fn test_copy_selection_limits_mandate_18() {
    assert_eq!(K_MAX_RECOGNITIONS_IN_FLIGHT, 2);

    let copy_view = HistoryViewCopySelectionView::with_text("Copied snippet");
    assert_eq!(copy_view.payload.text, "Copied snippet");
    assert_eq!(copy_view.max_recognitions_in_flight(), 2);
}

#[test]
fn test_cursor_and_point_state() {
    let mut cursor = HistoryViewCursorStateView::with_item(42);
    assert_eq!(cursor.point_state, PointState::Inside);
    assert_eq!(cursor.text_state.item_id, 42);

    cursor.text_state.set_position(15, true);
    assert_eq!(cursor.text_state.symbol, 15);
    assert!(cursor.text_state.after_symbol);

    cursor.text_state.add_offset(5);
    assert_eq!(cursor.text_state.symbol, 20);
}

// ============================================================================
// 8. Reply Button & Empty List Bubble Tests
// ============================================================================

#[test]
fn test_reply_button_timing_and_scale_mandate_18() {
    assert_eq!(K_TOGGLE_DURATION_MS, 120);
    assert_eq!(K_BUTTON_SHOW_DELAY_MS, 0);
    assert_eq!(K_BUTTON_HIDE_DELAY_MS, 300);
    assert_eq!(REPLY_CORNER_HEIGHT, 28.0);

    let mut button = HistoryViewReplyButtonView::with_item(55);
    assert_eq!(button.scale, 1.0);

    button.apply_state(ReplyButtonState::Hidden);
    assert_eq!(button.scale, 0.0);
    assert_eq!(button.opacity, 0.0);

    button.apply_state(ReplyButtonState::Active);
    assert_eq!(button.scale, 1.0);
}

#[test]
fn test_empty_list_bubble_position_calculation() {
    // Upstream: bubble is centered horizontally and placed 1/3 from top
    let parent_w = 800.0;
    let parent_h = 600.0;
    let bubble_w = 280.0;
    let bubble_h = 80.0;

    let (x, y) = calculate_empty_bubble_position(parent_w, parent_h, bubble_w, bubble_h);
    assert_eq!(x, (800.0 - 280.0) / 2.0); // 260.0
    assert_eq!(y, (600.0 - 80.0) / 3.0); // 173.33333

    let bubble = HistoryViewEmptyListBubbleView::new("No messages here yet".to_string());
    let (cx, cy) = bubble.compute_position(800.0, 600.0);
    assert_eq!(cx, 260.0);
    assert!((cy - 173.33333).abs() < 0.01);
}

// ============================================================================
// 9. Object Layout & Additional Views Tests
// ============================================================================

#[test]
fn test_history_view_object_dimensions_and_resize() {
    let mut obj = HistoryViewObjectView::new();
    assert!(!obj.is_initialized);

    obj.init_dimensions(LayoutSize::new(400, 250));
    assert!(obj.is_initialized);
    assert_eq!(obj.optimal_size(), LayoutSize::new(400, 250));

    let new_h = obj.resize_get_height(350, 220);
    assert_eq!(new_h, 220);
    assert_eq!(obj.current_size(), LayoutSize::new(350, 220));

    obj.adjust_height(10);
    assert_eq!(obj.height, 230);
}

#[test]
fn test_history_view_element_overlay_lifecycle() {
    let mut overlay = HistoryViewElementOverlayView::new();
    assert!(!overlay.is_active);

    overlay.show(99, 150);
    assert!(overlay.is_active);
    assert_eq!(overlay.item_id, 99);
    assert_eq!(overlay.top_offset, 150);

    overlay.trigger_submit();
    assert!(overlay.submit_requested);

    overlay.handle_click_outside();
    assert!(!overlay.is_active);
    assert!(overlay.close_requested);
}

#[test]
fn test_fake_items_and_summary_header() {
    let mut fake_gen = HistoryViewFakeItemsView::new();
    let item = fake_gen.generate_item(1001, None, "Simulated message", 1700000000);
    assert_eq!(item.id, 1);
    assert_eq!(item.text, "Simulated message");

    let mut summary = HistoryViewSummaryHeaderView::with_summary("Topic Discussion", "TL;DR summary text");
    assert!(!summary.data.is_expanded);
    summary.toggle_expanded();
    assert!(summary.data.is_expanded);
}

#[test]
fn test_draw_to_reply_and_text_helper_and_drag() {
    let mut draw = HistoryViewDrawToReplyView::new();
    assert!(!draw.state.editor_open);
    draw.open_editor(Some(501), None);
    assert!(draw.state.editor_open);
    assert!(draw.state.has_image);
    draw.close_editor();
    assert!(!draw.state.editor_open);

    let mut helper = HistoryViewTextHelperView::new();
    assert!(!helper.options.spoilers_revealed);
    helper.reveal_spoilers();
    assert!(helper.options.spoilers_revealed);
    helper.toggle_blockquote();
    assert!(helper.options.blockquote_expanded);

    let drag = HistoryViewDragView::with_photo(888, 10240);
    assert!(drag.drag_data.as_ref().unwrap().is_ready);
    assert_eq!(drag.drag_data.as_ref().unwrap().byte_count, 10240);
}
