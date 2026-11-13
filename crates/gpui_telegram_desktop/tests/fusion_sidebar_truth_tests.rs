//! FUS-D001 regression tests for truthful `ChatSidebar` initialization,
//! cloud workspace reset, and authoritative dialog selection semantics.

use gpui_telegram_desktop::views::chat_sidebar::{
    ChatSidebar, DialogDeliveryStatus, DialogItemModel, DEFAULT_WIDTH,
};
use gpui_telegram_desktop::views::sidebar::dialogs_widget::SearchInputState;
use gpui_telegram_desktop::views::sidebar::suggestions::suggestion_manager::{
    SuggestionItem, SuggestionManager, SuggestionPriority,
};
use gpui_telegram_desktop::views::sidebar::ui::dialogs_stories_list::StoryElement;
use gpui_telegram_desktop::views::sidebar::ui::top_peers_strip::TopPeerItem;

#[test]
fn test_sidebar_new_and_default_truthful_empty_state() {
    for sidebar in [ChatSidebar::new(), ChatSidebar::default()] {
        assert_eq!(sidebar.width, DEFAULT_WIDTH);
        assert!(!sidebar.is_resizing);
        assert!(
            sidebar.dialogs.is_empty(),
            "Production ChatSidebar::new must not seed fabricated dialogs"
        );
        assert!(
            sidebar.filtered_dialogs().is_empty(),
            "Filtered dialogs must be empty before server data arrives"
        );
        assert_eq!(
            sidebar.active_chat_id, None,
            "Production ChatSidebar::new must not pre-select any chat ID"
        );
        assert!(
            sidebar.selected_dialog().is_none(),
            "selected_dialog() must be None when no chat is selected"
        );
        assert_eq!(
            sidebar.folders,
            vec![("All".to_string(), 0)],
            "Only structural All tab at count 0 may exist before server folder data arrives"
        );
        assert_eq!(sidebar.active_folder, "All");
        assert!(sidebar.search_query.is_empty());
        assert!(!sidebar.search_focused);
        assert!(sidebar.story_clicked_peer.is_none());

        // Authentic submodules must also start without fabricated stories, top peers, or notices
        assert!(sidebar.stories.elements.is_empty());
        assert_eq!(sidebar.stories.collapsed_preview_count(), 0);
        assert!(!sidebar.stories.is_expanded);
        assert!(sidebar.stories.active_story_id.is_none());

        assert!(sidebar.top_peers.entries.is_empty());
        assert!(!sidebar.top_peers.is_expanded);
        assert!(sidebar.top_peers.selected_peer_id.is_none());

        assert!(!sidebar.suggestions.is_visible);
        assert!(sidebar.suggestions.current_item.is_none());
        assert_eq!(sidebar.suggestion_manager.active_count(), 0);
        assert!(sidebar.suggestion_manager.current_suggestion().is_none());

        assert_eq!(sidebar.inner_widget.total_rows, 0);
        assert_eq!(sidebar.inner_widget.selected_index, None);
        assert!(sidebar.inner_widget.pinned_drag.is_none());
        assert_eq!(sidebar.widget.search_state, SearchInputState::Idle);
    }
}

#[test]
fn test_sidebar_cloud_workspace_does_not_fabricate_peers_or_connectivity() {
    let mut sidebar = ChatSidebar::new();
    sidebar.configure_for_cloud_workspace("http://34.170.16.35:8080");

    assert!(
        sidebar.dialogs.is_empty(),
        "configure_for_cloud_workspace must not invent bots, Saved Messages, or server status rows"
    );
    assert_eq!(
        sidebar.active_chat_id, None,
        "configure_for_cloud_workspace must not select fixed chat ID 4 or any other chat"
    );
    assert!(sidebar.selected_dialog().is_none());
    assert_eq!(
        sidebar.folders,
        vec![("All".to_string(), 0)],
        "configure_for_cloud_workspace must not seed Bots/Personal/Work/Unread tabs"
    );
    assert_eq!(sidebar.active_folder, "All");
    assert!(sidebar.stories.elements.is_empty());
    assert!(sidebar.top_peers.entries.is_empty());
    assert!(!sidebar.suggestions.is_visible);
    assert!(sidebar.suggestions.current_item.is_none());
    assert_eq!(sidebar.suggestion_manager.active_count(), 0);
    assert_eq!(sidebar.inner_widget.total_rows, 0);
    assert_eq!(sidebar.inner_widget.selected_index, None);
}

#[test]
fn test_sidebar_cloud_reset_clears_previously_populated_account_state() {
    let mut sidebar = ChatSidebar::new();

    // Populate with previous account dialogs, folders, stories, top peers, suggestions, and UI state
    sidebar.dialogs = vec![
        DialogItemModel {
            id: 77,
            peer_name: "Previous Account Peer".to_string(),
            avatar_initials: "PA".to_string(),
            avatar_color: 0x2AABEE,
            last_message: "Confidential previous account message".to_string(),
            timestamp: "11:20".to_string(),
            unread_count: 4,
            is_muted: false,
            is_pinned: true,
            is_online: true,
            typing_indicator: Some("typing...".to_string()),
            delivery_status: DialogDeliveryStatus::Sent,
            folder: "Work".to_string(),
        },
        DialogItemModel {
            id: 88,
            peer_name: "Second Peer".to_string(),
            avatar_initials: "SP".to_string(),
            avatar_color: 0x9B59B6,
            last_message: "Another message".to_string(),
            timestamp: "10:00".to_string(),
            unread_count: 0,
            is_muted: true,
            is_pinned: true,
            is_online: false,
            typing_indicator: None,
            delivery_status: DialogDeliveryStatus::Read,
            folder: "Personal".to_string(),
        },
    ];
    sidebar.folders = vec![
        ("All".to_string(), 4),
        ("Work".to_string(), 4),
        ("Personal".to_string(), 0),
    ];
    sidebar.select_folder("Work");
    sidebar.select_dialog(77);
    sidebar.set_search_query("confidential");
    sidebar.search_focused = true;
    sidebar.is_resizing = true;
    sidebar.story_clicked_peer = Some("Previous Account Peer".to_string());

    sidebar.stories.set_elements(vec![StoryElement {
        id: 77,
        peer_name: "Previous Account Peer".to_string(),
        avatar_initials: "PA".to_string(),
        avatar_color: 0x2AABEE,
        story_count: 1,
        unread_count: 1,
        has_video_stream: false,
    }]);
    sidebar.stories.toggle_expanded();
    sidebar.stories.active_story_id = Some(77);

    sidebar.top_peers.set_entries(vec![TopPeerItem {
        id: 77,
        name: "PrevPeer".to_string(),
        avatar_initials: "PA".to_string(),
        avatar_color: 0x2AABEE,
        badge: 4,
        is_unread: true,
        is_muted: false,
        is_online: true,
    }]);
    sidebar.top_peers.toggle_expanded();
    sidebar.top_peers.selected_peer_id = Some(77);

    sidebar.suggestion_manager.add_suggestion(SuggestionItem {
        priority: SuggestionPriority::PremiumGrace,
        key: SuggestionManager::KEY_PREMIUM_GRACE.to_string(),
        title: "Premium Subscription Paused".to_string(),
        description: "Update payment method".to_string(),
        button_text: Some("Renew".to_string()),
        action_url: None,
        can_close: true,
    });
    sidebar
        .suggestions
        .update_from_manager(&sidebar.suggestion_manager);

    sidebar.inner_widget.total_rows = 2;
    sidebar
        .inner_widget
        .start_reorder_pinned(77, 10, vec![77, 88]);

    // Trigger cloud workspace reset (e.g. account switch or re-login)
    sidebar.configure_for_cloud_workspace("https://new-session.example.com");

    assert!(!sidebar.is_resizing);
    assert!(sidebar.dialogs.is_empty());
    assert!(sidebar.filtered_dialogs().is_empty());
    assert_eq!(sidebar.active_chat_id, None);
    assert!(sidebar.selected_dialog().is_none());
    assert_eq!(sidebar.folders, vec![("All".to_string(), 0)]);
    assert_eq!(sidebar.active_folder, "All");
    assert!(sidebar.search_query.is_empty());
    assert!(sidebar.widget.search_query.is_empty());
    assert_eq!(sidebar.widget.search_state, SearchInputState::Idle);
    assert!(!sidebar.search_focused);
    assert!(sidebar.story_clicked_peer.is_none());

    assert!(sidebar.stories.elements.is_empty());
    assert!(!sidebar.stories.is_expanded);
    assert!(sidebar.stories.active_story_id.is_none());

    assert!(sidebar.top_peers.entries.is_empty());
    assert!(!sidebar.top_peers.is_expanded);
    assert!(sidebar.top_peers.selected_peer_id.is_none());

    assert!(!sidebar.suggestions.is_visible);
    assert!(sidebar.suggestions.current_item.is_none());
    assert_eq!(sidebar.suggestion_manager.active_count(), 0);
    assert!(sidebar.suggestion_manager.current_suggestion().is_none());

    assert_eq!(sidebar.inner_widget.total_rows, 0);
    assert_eq!(sidebar.inner_widget.selected_index, None);
    assert!(sidebar.inner_widget.pinned_drag.is_none());
}

#[test]
fn test_sidebar_authoritative_dialog_selection_and_nonexistent_id_guard() {
    let mut sidebar = ChatSidebar::new();

    // Selecting any ID when no authoritative dialogs exist must not fabricate a selection
    sidebar.select_dialog(1);
    sidebar.select_dialog(3);
    sidebar.select_dialog(4);
    assert_eq!(sidebar.active_chat_id, None);
    assert!(sidebar.selected_dialog().is_none());
    assert_eq!(sidebar.inner_widget.selected_index, None);

    // Populate authoritative dialogs from server
    sidebar.dialogs.push(DialogItemModel {
        id: 101,
        peer_name: "Backend Engineering".to_string(),
        avatar_initials: "BE".to_string(),
        avatar_color: 0x2AABEE,
        last_message: "Deployment verified".to_string(),
        timestamp: "09:15".to_string(),
        unread_count: 3,
        is_muted: false,
        is_pinned: false,
        is_online: false,
        typing_indicator: None,
        delivery_status: DialogDeliveryStatus::Sent,
        folder: "All".to_string(),
    });
    sidebar.dialogs.push(DialogItemModel {
        id: 202,
        peer_name: "Release Coordination".to_string(),
        avatar_initials: "RC".to_string(),
        avatar_color: 0x1ABC9C,
        last_message: "All checks green".to_string(),
        timestamp: "09:30".to_string(),
        unread_count: 5,
        is_muted: false,
        is_pinned: true,
        is_online: true,
        typing_indicator: None,
        delivery_status: DialogDeliveryStatus::Sending,
        folder: "All".to_string(),
    });
    sidebar.inner_widget.total_rows = sidebar.dialogs.len();

    // Selecting an authoritative dialog updates active_chat_id, inner_widget index, and read state
    sidebar.select_dialog(202);
    assert_eq!(sidebar.active_chat_id, Some(202));
    assert_eq!(sidebar.inner_widget.selected_index, Some(1));
    let selected = sidebar
        .selected_dialog()
        .expect("Dialog 202 must be selected");
    assert_eq!(selected.peer_name, "Release Coordination");
    assert_eq!(selected.unread_count, 0);
    assert_eq!(selected.delivery_status, DialogDeliveryStatus::Read);

    // Selecting a non-existent ID must not clobber the valid selection with a dangling ID
    sidebar.select_dialog(999);
    assert_eq!(sidebar.active_chat_id, Some(202));
    assert_eq!(sidebar.inner_widget.selected_index, Some(1));

    // Clearing selection resets both active_chat_id and inner_widget selection
    sidebar.clear_selection();
    assert_eq!(sidebar.active_chat_id, None);
    assert_eq!(sidebar.inner_widget.selected_index, None);
    assert!(sidebar.selected_dialog().is_none());
}

#[test]
fn test_sidebar_new_demo_isolates_illustrative_fixtures_and_resets_cleanly() {
    let mut demo = ChatSidebar::new_demo();
    assert_eq!(demo.dialogs.len(), 5);
    assert_eq!(demo.folders.len(), 5);
    assert_eq!(demo.active_chat_id, Some(1));
    assert_eq!(demo.stories.collapsed_preview_count(), 3);
    assert_eq!(demo.top_peers.entries.len(), 3);
    assert!(demo.suggestions.is_visible);

    // Resetting a demo-populated instance for cloud workspace removes all demo artifacts
    demo.configure_for_cloud_workspace("http://127.0.0.1:8080");
    assert!(demo.dialogs.is_empty());
    assert_eq!(demo.folders, vec![("All".to_string(), 0)]);
    assert_eq!(demo.active_chat_id, None);
    assert!(demo.stories.elements.is_empty());
    assert!(demo.top_peers.entries.is_empty());
    assert!(!demo.suggestions.is_visible);
    assert_eq!(demo.suggestion_manager.active_count(), 0);
    assert_eq!(demo.inner_widget.total_rows, 0);
}
