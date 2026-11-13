//! FUS-D002 regression tests for truthful initial and cloud-reset `ConversationStage` state.

use gpui::{px, Point, Size};
use gpui_telegram_desktop::views::conversation_stage::{
    ConversationStage, DeliveryStatus, MessageContent, MessageContextMenuState, MessageDirection,
    MessageItemModel, ReplyReference,
};

#[test]
fn test_stage_new_has_no_selected_chat_or_fabricated_data() {
    let stage = ConversationStage::new();

    assert_eq!(stage.active_chat_id, None, "Initial stage must have no selected chat ID");
    assert_eq!(stage.chat_title, "Select a chat");
    assert!(
        stage.chat_subtitle.is_empty(),
        "Initial stage must not invent an online subtitle: {:?}",
        stage.chat_subtitle
    );
    assert!(
        stage.chat_avatar_initials.is_empty(),
        "Initial stage must not invent peer initials: {:?}",
        stage.chat_avatar_initials
    );
    assert_eq!(stage.pinned_message, None);
    assert!(stage.messages.is_empty());
    assert!(stage.chat_histories.is_empty());
    assert!(stage.chat_pinned_messages.is_empty());
    assert!(stage.chat_scroll_positions.is_empty());
    assert!(!stage.is_inspector_open);
    assert!(!stage.pending_call_request);
    assert_eq!(stage.pending_media_url, None);
    assert_eq!(stage.pending_bot_action, None);
    assert_eq!(stage.pending_reply, None);
    assert_eq!(stage.active_context_menu, None);
    assert_eq!(stage.unread_incoming_count, 0);
    assert!(stage.is_anchored_to_bottom);
    assert_eq!(stage.scroll_handle.offset(), Point::default());
    assert_eq!(stage.simulated_max_offset, None);

    // RowHeightCache must be cleanly rebuilt for empty state
    assert!(!stage.row_height_cache.is_dirty());
    assert!(stage.row_height_cache.is_empty());
    assert_eq!(stage.row_height_cache.total_height, 0.0);
    assert!(stage.row_height_cache.measured_heights.is_empty());
}

#[test]
fn test_stage_cloud_reset_does_not_fabricate_messages_or_connectivity_from_url_or_username() {
    let mut stage = ConversationStage::new();
    let server_url = "https://cloud-gateway.example.org:8443";
    stage.configure_for_cloud_workspace(server_url, Some("alice_operator"));

    assert_eq!(stage.active_chat_id, None);
    assert_eq!(stage.chat_title, "Select a chat");
    assert!(stage.chat_subtitle.is_empty());
    assert!(stage.chat_avatar_initials.is_empty());
    assert_eq!(stage.pinned_message, None);
    assert!(stage.messages.is_empty());
    assert!(stage.chat_histories.is_empty());
    assert!(stage.chat_pinned_messages.is_empty());
    assert!(stage.chat_scroll_positions.is_empty());

    assert!(!stage.chat_title.contains(server_url));
    assert!(!stage.chat_title.contains("alice_operator"));
    assert!(!stage.chat_subtitle.contains("online"));
    assert!(!stage.chat_subtitle.contains("Connected"));
}

#[test]
fn test_stage_cloud_reset_clears_all_account_bleed_from_populated_model() {
    let mut stage = ConversationStage::new();

    // Populate Account 1 state across multiple chats, caches, scroll positions, and pending actions
    stage.set_chat(10, "Account 1 Peer", "online", "A1", 0x2AABEE);
    stage.add_message(MessageItemModel {
        id: 1001,
        sender_id: 10,
        sender_name: "Account 1 Peer".to_string(),
        avatar_initials: "A1".to_string(),
        avatar_color: 0x2AABEE,
        direction: MessageDirection::Incoming,
        timestamp: "10:15".to_string(),
        delivery_status: DeliveryStatus::Read,
        reply_to: None,
        reactions: vec![],
        content: MessageContent::Text {
            text: "Confidential message in account 1".to_string(),
            is_edited: false,
        },
        bot_keyboard: None,
        is_pinned: true,
    });
    stage.set_simulated_scroll_offset(
        Point::new(px(0.0), px(-640.0)),
        Size::new(px(0.0), px(2400.0)),
    );
    stage.row_height_cache.measured_heights.insert(1001, 88.0);
    stage.row_height_cache.rebuild_prefix_sums(&stage.messages);

    // Switch to another chat in Account 1 so chat 10 is stored in histories/pins/scroll maps
    stage.set_chat(20, "Second Peer", "last seen recently", "SP", 0x9B59B6);
    stage.add_outgoing_text_message("Outgoing message in chat 20");
    stage.is_inspector_open = true;
    stage.pending_call_request = true;
    stage.pending_media_url = Some(("https://example.com/private.mp4".to_string(), true));
    stage.pending_bot_action = Some("Remote Computer".to_string());
    stage.pending_reply = Some(ReplyReference {
        message_id: 1001,
        sender_name: "Account 1 Peer".to_string(),
        snippet: "Confidential".to_string(),
    });
    stage.active_context_menu = Some(MessageContextMenuState {
        message_id: 1001,
        position: Point::new(px(120.0), px(180.0)),
        is_outgoing: false,
        is_pinned: true,
        text_snippet: Some("Confidential".to_string()),
    });
    stage.unread_incoming_count = 7;

    // Reset for Account 2 cloud workspace
    stage.configure_for_cloud_workspace("https://server.example.com", Some("account_two"));

    assert_eq!(stage.active_chat_id, None);
    assert_eq!(stage.chat_title, "Select a chat");
    assert!(stage.chat_subtitle.is_empty());
    assert!(stage.chat_avatar_initials.is_empty());
    assert_eq!(stage.pinned_message, None);
    assert!(stage.messages.is_empty());
    assert!(stage.chat_histories.is_empty());
    assert!(stage.chat_pinned_messages.is_empty());
    assert!(stage.chat_scroll_positions.is_empty());
    assert!(!stage.is_inspector_open);
    assert!(!stage.pending_call_request);
    assert_eq!(stage.pending_media_url, None);
    assert_eq!(stage.pending_bot_action, None);
    assert_eq!(stage.pending_reply, None);
    assert_eq!(stage.active_context_menu, None);
    assert_eq!(stage.unread_incoming_count, 0);
    assert_eq!(stage.scroll_handle.offset(), Point::default());
    assert_eq!(stage.simulated_max_offset, None);
    assert!(stage.is_anchored_to_bottom);
    assert!(stage.row_height_cache.measured_heights.is_empty());
    assert!(stage.row_height_cache.prefix_sums.is_empty());
    assert_eq!(stage.row_height_cache.total_height, 0.0);
    assert!(!stage.row_height_cache.is_dirty());

    // Selecting chat ID 10 in the new account must not resurrect Account 1 history, pin, or scroll state
    stage.set_chat(10, "New Account Peer", "", "NA", 0x1ABC9C);
    assert_eq!(stage.active_chat_id, Some(10));
    assert!(stage.messages.is_empty());
    assert_eq!(stage.pinned_message, None);
    assert_eq!(stage.scroll_handle.offset(), Point::default());
    assert!(stage.is_anchored_to_bottom);
}

#[test]
fn test_stage_authoritative_selection_and_scroll_preservation() {
    let mut stage = ConversationStage::new();
    assert_eq!(stage.active_chat_id, None);

    // First selection from None must not insert phantom history entries
    stage.set_chat(101, "Engineering", "8 members", "EN", 0x2AABEE);
    assert_eq!(stage.active_chat_id, Some(101));
    assert!(stage.chat_histories.is_empty());

    stage.add_incoming_message(501, "First authoritative message");
    stage.add_outgoing_text_message("Second authoritative message");
    assert_eq!(stage.message_count(), 2);

    // Simulate user scrolling up in chat 101
    stage.set_simulated_scroll_offset(
        Point::new(px(0.0), px(-300.0)),
        Size::new(px(0.0), px(1200.0)),
    );
    assert!(!stage.is_at_bottom());

    // Incoming message while scrolled up increments unread count without stealing scroll
    stage.add_incoming_message(501, "Third message while reading history");
    assert_eq!(stage.unread_incoming_count, 1);
    assert_eq!(stage.scroll_handle.offset().y, px(-300.0));

    // Pending context menu & reply on chat 101 should clear when switching to chat 202
    stage.active_context_menu = Some(MessageContextMenuState {
        message_id: 1,
        position: Point::new(px(20.0), px(40.0)),
        is_outgoing: false,
        is_pinned: false,
        text_snippet: Some("First".to_string()),
    });
    stage.pending_reply = Some(ReplyReference {
        message_id: 1,
        sender_name: "Peer".to_string(),
        snippet: "First".to_string(),
    });

    stage.set_chat(202, "Design", "4 members", "DS", 0x9B59B6);
    assert_eq!(stage.active_chat_id, Some(202));
    assert!(stage.messages.is_empty());
    assert_eq!(stage.active_context_menu, None);
    assert_eq!(stage.pending_reply, None);
    assert!(stage.is_anchored_to_bottom);

    // Switching back to chat 101 restores its 3 messages and saved scroll offset
    stage.set_chat(101, "Engineering", "8 members", "EN", 0x2AABEE);
    assert_eq!(stage.active_chat_id, Some(101));
    assert_eq!(stage.message_count(), 3);
    assert_eq!(stage.scroll_handle.offset().y, px(-300.0));
    assert!(!stage.is_anchored_to_bottom);
}

#[test]
fn test_stage_new_demo_explicit_fixture_remains_isolated_from_production_new() {
    let prod_stage = ConversationStage::new();
    assert_eq!(prod_stage.active_chat_id, None);
    assert!(prod_stage.messages.is_empty());

    let demo_stage = ConversationStage::new_demo();
    assert_eq!(demo_stage.active_chat_id, Some(1));
    assert!(demo_stage.message_count() >= 11);
}
