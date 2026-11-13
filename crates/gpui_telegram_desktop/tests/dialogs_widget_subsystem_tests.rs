//! Dialogs Widget Subsystem Integration Tests (Desktop Crate)
//!
//! Verifies:
//! 1. Dialogs style pixel metrics and dynamic row height calculation (`dialogs.style:L14-220`)
//! 2. Search debounce delay (900ms) and pagination limit (50) (`dialogs_widget.cpp:L114-116`)
//! 3. Virtual list keyboard navigation (`selectSkip`, `selectSkipPage`) and clamping (`dialogs_inner_widget.cpp:L5469-5650`)
//! 4. Pinned chat drag-and-drop reorder threshold (30px) and state machine (`dialogs_inner_widget.cpp:L2733-2800`)
//! 5. Suggestion priority hierarchy (1..9), dismissal, and unreviewed auth formatting (`suggestion.h:L43-53`)
//! 6. Screen reader accessibility formatting and 100-character truncation (`dialogs_inner_widget_accessibility.cpp:L87`)
//! 7. Stories horizontal carousel expansion and collapse ratio thresholds (`dialogs_stories_list.cpp:L37-43`)
//! 8. ChatSidebar integration with authentic submodules

use gpui::px;
use gpui_telegram_desktop::theme::dialogs::{
    DialogsLayoutUiStyleSheet, DialogsStyleSheet, DialogsWidgetStyleSheet,
};
use gpui_telegram_desktop::views::chat_sidebar::ChatSidebar;
use gpui_telegram_desktop::views::sidebar::dialogs_inner_widget::{
    DialogsInnerWidget, K_START_REORDER_THRESHOLD_PX,
};
use gpui_telegram_desktop::views::sidebar::dialogs_inner_widget_accessibility::{
    DialogsInnerWidgetAccessibility, PeerChatType, K_MAX_MESSAGE_LENGTH,
};
use gpui_telegram_desktop::views::sidebar::dialogs_widget::{
    redirect_text_to_search, DialogsWidget, SearchInputState, K_SEARCH_PER_PAGE,
    K_SEARCH_REQUEST_DELAY_MS,
};
use gpui_telegram_desktop::views::sidebar::suggestions::suggestion_manager::{
    SuggestionItem, SuggestionManager, SuggestionPriority, UnreviewedAuthDetails,
};
use gpui_telegram_desktop::views::sidebar::ui::dialogs_stories_list::{
    DialogsStoriesListUi, StoryElement, K_COLLAPSE_AFTER_RATIO, K_EXPAND_AFTER_RATIO,
    K_SMALL_THUMBS_SHOWN,
};

#[test]
fn test_dialogs_style_metrics() {
    let style = DialogsStyleSheet::new();

    // Verify row heights from dialogs.style
    assert_eq!(style.default_row_height, px(62.0));
    assert_eq!(style.tagged_row_height, px(72.0));
    assert_eq!(style.forum_row_height, px(80.0));
    assert_eq!(style.tagged_forum_row_height, px(96.0));
    assert_eq!(style.compact_row_height, px(54.0));

    // Verify padding and layout offsets
    assert_eq!(style.padding_horizontal, px(10.0));
    assert_eq!(style.padding_vertical, px(8.0));
    assert_eq!(style.photo_size, px(46.0));
    assert_eq!(style.name_left, px(68.0));
    assert_eq!(style.name_top, px(10.0));
    assert_eq!(style.text_left, px(68.0));
    assert_eq!(style.text_top, px(34.0));

    // Verify badge dimensions
    assert_eq!(style.unread_height, px(19.0));
    assert_eq!(style.unread_padding, px(5.0));
    assert_eq!(style.online_badge_size, px(10.0));
    assert_eq!(style.online_badge_stroke, px(2.0));
    assert_eq!(style.online_badge_duration_ms, 150);
    assert_eq!(style.subscription_badge_size, px(16.0));
    assert_eq!(style.subscription_outline_twice, px(3.0));

    // Verify dynamic row height computation
    assert_eq!(style.compute_row_height(false, false), px(62.0));
    assert_eq!(style.compute_row_height(true, false), px(72.0));
    assert_eq!(style.compute_row_height(false, true), px(80.0));
    assert_eq!(style.compute_row_height(true, true), px(96.0));
}

#[test]
fn test_dialogs_widget_and_layout_styles() {
    let widget_style = DialogsWidgetStyleSheet::new();
    assert_eq!(widget_style.search_debounce_ms, 900);
    assert_eq!(widget_style.lock_width, px(36.0));
    assert_eq!(widget_style.lock_height, px(38.0));
    assert_eq!(widget_style.load_more_height, px(36.0));
    assert_eq!(widget_style.load_more_text_top, px(9.0));
    assert_eq!(widget_style.community_requests_height, px(44.0));
    assert_eq!(widget_style.community_requests_radius, px(11.0));

    let layout_style = DialogsLayoutUiStyleSheet::new();
    assert_eq!(layout_style.poll_icon_size, px(18.0));
    assert_eq!(layout_style.mention_badge_size, px(13.0));
    assert_eq!(layout_style.reaction_badge_size, px(13.0));
}

#[test]
fn test_search_debounce_and_constants() {
    assert_eq!(K_SEARCH_REQUEST_DELAY_MS, 900);
    assert_eq!(K_SEARCH_PER_PAGE, 50);

    // Character redirection logic
    assert!(redirect_text_to_search("Hello"));
    assert!(redirect_text_to_search(" "));
    assert!(!redirect_text_to_search("\x01\x02\x05"));

    let mut widget = DialogsWidget::new();
    assert_eq!(widget.search_state, SearchInputState::Idle);

    widget.on_search_text_changed("Rust".to_string());
    assert_eq!(widget.search_state, SearchInputState::Debouncing);
    assert_eq!(widget.search_query, "Rust");

    widget.execute_search();
    assert_eq!(widget.search_state, SearchInputState::Searching);

    widget.on_search_text_changed(String::new());
    assert_eq!(widget.search_state, SearchInputState::Idle);
}

#[test]
fn test_keyboard_navigation_select_skip() {
    let mut inner = DialogsInnerWidget::new();
    assert_eq!(inner.row_height, 62);
    assert_eq!(inner.selected_index, None);

    // Initial navigation lands on 0
    inner.select_skip(1, 10);
    assert_eq!(inner.selected_index, Some(1));

    // Step down
    inner.select_skip(1, 10);
    assert_eq!(inner.selected_index, Some(2));

    // Step up
    inner.select_skip(-1, 10);
    assert_eq!(inner.selected_index, Some(1));

    // Clamping at top
    inner.select_skip(-10, 10);
    assert_eq!(inner.selected_index, Some(0));

    // Clamping at bottom
    inner.select_skip(100, 10);
    assert_eq!(inner.selected_index, Some(9));

    // Page skip math (e.g. 310px / 62px = 5 rows)
    inner.selected_index = Some(2);
    inner.select_skip_page(310, 1, 10);
    assert_eq!(inner.selected_index, Some(7));

    inner.select_skip_page(310, -1, 10);
    assert_eq!(inner.selected_index, Some(2));
}

#[test]
fn test_pinned_reorder_drag_threshold() {
    assert_eq!(K_START_REORDER_THRESHOLD_PX, 30);

    // Below 30px does not start drag
    assert!(!DialogsInnerWidget::check_reorder_pinned_start(100, 120));
    assert!(!DialogsInnerWidget::check_reorder_pinned_start(100, 129));
    assert!(!DialogsInnerWidget::check_reorder_pinned_start(100, 71));

    // 30px or more activates drag
    assert!(DialogsInnerWidget::check_reorder_pinned_start(100, 130));
    assert!(DialogsInnerWidget::check_reorder_pinned_start(100, 150));
    assert!(DialogsInnerWidget::check_reorder_pinned_start(100, 70));

    let mut inner = DialogsInnerWidget::new();
    inner.start_reorder_pinned(42, 100, vec![1, 2, 42, 4]);
    assert!(inner.pinned_drag.is_some());

    let drag = inner.pinned_drag.as_ref().unwrap();
    assert_eq!(drag.dragging_peer_id, 42);
    assert_eq!(drag.start_y, 100);

    inner.update_drag_position(160);
    assert_eq!(inner.pinned_drag.as_ref().unwrap().current_y, 160);

    let finished = inner.finish_drag();
    assert_eq!(finished, Some(vec![1, 2, 42, 4]));
    assert!(inner.pinned_drag.is_none());
}

#[test]
fn test_suggestions_priority_hierarchy() {
    // Assert strictly ordered priority values
    assert!(SuggestionPriority::UserpicSetup < SuggestionPriority::PremiumOffer);
    assert!(SuggestionPriority::PremiumOffer < SuggestionPriority::BirthdaySetup);
    assert!(SuggestionPriority::BirthdaySetup < SuggestionPriority::BirthdayContacts);
    assert!(SuggestionPriority::BirthdayContacts < SuggestionPriority::LowCreditsSubs);
    assert!(SuggestionPriority::LowCreditsSubs < SuggestionPriority::PremiumGrace);
    assert!(SuggestionPriority::PremiumGrace < SuggestionPriority::CustomPromo);
    assert!(SuggestionPriority::CustomPromo < SuggestionPriority::GiftAuctions);
    assert!(SuggestionPriority::GiftAuctions < SuggestionPriority::UnreviewedAuth);

    let mut manager = SuggestionManager::new();

    // Add lower priority first
    manager.add_suggestion(SuggestionItem {
        priority: SuggestionPriority::UnreviewedAuth,
        key: "AUTH_1".to_string(),
        title: "Login Alert".to_string(),
        description: "New login".to_string(),
        button_text: None,
        action_url: None,
        can_close: true,
    });

    // Add higher priority later
    manager.add_suggestion(SuggestionItem {
        priority: SuggestionPriority::UserpicSetup,
        key: "USERPIC_1".to_string(),
        title: "Set Avatar".to_string(),
        description: "Upload photo".to_string(),
        button_text: None,
        action_url: None,
        can_close: true,
    });

    // Highest priority (UserpicSetup = 1) must be current
    assert_eq!(
        manager.current_suggestion().map(|s| s.priority),
        Some(SuggestionPriority::UserpicSetup)
    );

    // Dismiss highest priority
    manager.dismiss("USERPIC_1");
    assert_eq!(
        manager.current_suggestion().map(|s| s.priority),
        Some(SuggestionPriority::UnreviewedAuth)
    );

    // Unreviewed auth formatting test
    let auth = UnreviewedAuthDetails {
        location: "Berlin, Germany".to_string(),
        device: "MacBook Pro".to_string(),
        date: "Today".to_string(),
    };
    assert_eq!(auth.format_auth_info(), "🌍 Berlin, Germany (📱 MacBook Pro)");
}

#[test]
fn test_screen_reader_accessibility_formatting() {
    assert_eq!(K_MAX_MESSAGE_LENGTH, 100);

    let short_text = "Hello world";
    assert_eq!(
        DialogsInnerWidgetAccessibility::format_message_snippet(short_text),
        "Hello world"
    );

    let long_text = "A".repeat(150);
    let snippet = DialogsInnerWidgetAccessibility::format_message_snippet(&long_text);
    assert_eq!(snippet.len(), 103); // 100 + "..."
    assert!(snippet.ends_with("..."));

    let mut a11y = DialogsInnerWidgetAccessibility::new("Rust Devs".to_string());
    a11y.chat_type = PeerChatType::Group;
    a11y.is_pinned = true;
    a11y.unread_count = 3;
    a11y.message_text = "Meeting at 5pm".to_string();

    let announcement = a11y.build_accessible_announcement();
    assert!(announcement.contains("Rust Devs"));
    assert!(announcement.contains("group"));
    assert!(announcement.contains("pinned"));
    assert!(announcement.contains("3 unread messages"));
    assert!(announcement.contains("Meeting at 5pm"));
}

#[test]
fn test_stories_expansion_ratios() {
    assert_eq!(K_SMALL_THUMBS_SHOWN, 3);
    assert_eq!(K_EXPAND_AFTER_RATIO, 0.72);
    assert_eq!(K_COLLAPSE_AFTER_RATIO, 0.68);

    assert!(DialogsStoriesListUi::should_expand(0.75));
    assert!(DialogsStoriesListUi::should_expand(0.72));
    assert!(!DialogsStoriesListUi::should_expand(0.71));

    assert!(DialogsStoriesListUi::should_collapse(0.65));
    assert!(DialogsStoriesListUi::should_collapse(0.68));
    assert!(!DialogsStoriesListUi::should_collapse(0.70));

    let mut stories = DialogsStoriesListUi::new();
    stories.set_elements(vec![
        StoryElement {
            id: 1,
            peer_name: "User 1".to_string(),
            avatar_initials: "U1".to_string(),
            avatar_color: 0x123456,
            story_count: 1,
            unread_count: 1,
            has_video_stream: false,
        },
        StoryElement {
            id: 2,
            peer_name: "User 2".to_string(),
            avatar_initials: "U2".to_string(),
            avatar_color: 0x654321,
            story_count: 2,
            unread_count: 0,
            has_video_stream: false,
        },
        StoryElement {
            id: 3,
            peer_name: "User 3".to_string(),
            avatar_initials: "U3".to_string(),
            avatar_color: 0xABCDEF,
            story_count: 1,
            unread_count: 1,
            has_video_stream: true,
        },
        StoryElement {
            id: 4,
            peer_name: "User 4".to_string(),
            avatar_initials: "U4".to_string(),
            avatar_color: 0xFEDCBA,
            story_count: 3,
            unread_count: 0,
            has_video_stream: false,
        },
    ]);

    assert_eq!(stories.collapsed_preview_count(), 3);
}

#[test]
fn test_chat_sidebar_wiring() {
    let mut sidebar = ChatSidebar::new();
    assert_eq!(sidebar.dialogs.len(), 5);
    assert_eq!(sidebar.folders.len(), 5);

    // Test folder filtering
    sidebar.select_folder("Work");
    let work_dialogs = sidebar.filtered_dialogs();
    assert_eq!(work_dialogs.len(), 1);
    assert_eq!(work_dialogs[0].peer_name, "Rust Core Devs");

    // Test search filtering
    sidebar.select_folder("All");
    sidebar.set_search_query("alice");
    let search_results = sidebar.filtered_dialogs();
    assert_eq!(search_results.len(), 1);
    assert_eq!(search_results[0].peer_name, "Alice Smith");

    // Verify submodules are authentically instantiated
    assert_eq!(sidebar.style.default_row_height, px(62.0));
    assert_eq!(sidebar.stories.collapsed_preview_count(), 3);
    assert!(sidebar.suggestions.is_visible);
    assert_eq!(
        sidebar.suggestion_manager.current_suggestion().map(|s| s.priority),
        Some(SuggestionPriority::PremiumGrace)
    );
}

#[test]
fn test_sidebar_plus_menu_contact_group_creation_and_categorized_search() {
    use gpui_telegram_desktop::views::chat_sidebar::{
        AddContactKind, CreationInputField, SidebarCreationModal, TELEGRAM_SEARCH_CATEGORIES,
        TELEGRAM_SEARCH_TAGS,
    };

    let mut sidebar = ChatSidebar::new();
    let initial_len = sidebar.dialogs.len();

    // 1. Toggle `+` menu next to search box
    assert!(!sidebar.is_plus_menu_open);
    sidebar.toggle_plus_menu();
    assert!(sidebar.is_plus_menu_open);

    // 2. Open Add Contact modal -> Create AI Bot sub-mode
    sidebar.open_add_contact_modal(AddContactKind::AiBot);
    assert!(!sidebar.is_plus_menu_open);
    assert_eq!(sidebar.active_creation_modal, Some(SidebarCreationModal::AddContact));
    assert_eq!(sidebar.contact_kind, AddContactKind::AiBot);
    assert_eq!(sidebar.active_creation_field, CreationInputField::PrimaryName);

    sidebar.push_creation_text("Coder AI Bot");
    sidebar.cycle_creation_field();
    assert_eq!(sidebar.active_creation_field, CreationInputField::SecondaryHandle);
    sidebar.push_creation_text("@coder_ai");
    sidebar.cycle_creation_field();
    assert_eq!(sidebar.active_creation_field, CreationInputField::TertiaryPromptOrPhone);
    sidebar.push_creation_text("Write clean Rust GPUI code.");

    let created_bot_id = sidebar.submit_creation_modal().expect("Should create AI Bot dialog");
    assert_eq!(sidebar.active_creation_modal, None);
    assert_eq!(sidebar.dialogs.len(), initial_len + 1);
    assert_eq!(sidebar.active_chat_id, Some(created_bot_id));
    let pending_bot = sidebar
        .pending_add_contact_request
        .take()
        .expect("Should emit PendingAddContactRequest for AI Bot");
    assert!(pending_bot.is_bot);
    assert_eq!(pending_bot.first_name, "Coder AI Bot");
    assert_eq!(pending_bot.username.as_deref(), Some("coder_ai_bot"));
    assert_eq!(
        pending_bot.system_prompt.as_deref(),
        Some("Write clean Rust GPUI code.")
    );

    // 3. Open Add Contact modal -> Add Friend sub-mode
    sidebar.open_add_contact_modal(AddContactKind::Friend);
    assert_eq!(sidebar.contact_kind, AddContactKind::Friend);
    sidebar.push_creation_text("Linus Torvalds");
    sidebar.cycle_creation_field();
    sidebar.push_creation_text("linus_t");
    sidebar.cycle_creation_field();
    sidebar.push_creation_text("+15550100");
    let created_friend_id = sidebar.submit_creation_modal().expect("Should create Friend dialog");
    assert_eq!(sidebar.active_chat_id, Some(created_friend_id));
    let pending_friend = sidebar
        .pending_add_contact_request
        .take()
        .expect("Should emit PendingAddContactRequest for Friend");
    assert!(!pending_friend.is_bot);
    assert_eq!(pending_friend.first_name, "Linus Torvalds");
    assert_eq!(pending_friend.username.as_deref(), Some("linus_t"));
    assert_eq!(pending_friend.phone.as_deref(), Some("+15550100"));

    // 4. Open Create Group modal -> test all 3 fields (Title, @username/numeric members, Description)
    sidebar.open_create_group_modal();
    assert_eq!(sidebar.active_creation_modal, Some(SidebarCreationModal::CreateGroup));
    assert_eq!(sidebar.active_creation_field, CreationInputField::PrimaryName);
    sidebar.push_creation_text("Telegram GPUI Architects");
    sidebar.cycle_creation_field();
    assert_eq!(sidebar.active_creation_field, CreationInputField::SecondaryHandle);
    sidebar.push_creation_text("@coder_ai_bot, @linus_t, 42");
    sidebar.cycle_creation_field();
    assert_eq!(
        sidebar.active_creation_field,
        CreationInputField::TertiaryPromptOrPhone,
        "CreateGroup Tab cycling must visit the 3rd field (Description/About)"
    );
    sidebar.push_creation_text("1:1 Telegram SourceFiles Rust parity group");
    let created_group_id = sidebar.submit_creation_modal().expect("Should create Group dialog");
    assert_eq!(sidebar.active_chat_id, Some(created_group_id));
    let pending_group = sidebar
        .pending_create_group_request
        .take()
        .expect("Should emit PendingCreateGroupRequest");
    assert_eq!(pending_group.title, "Telegram GPUI Architects");
    assert_eq!(pending_group.member_ids, vec![42]);
    assert_eq!(
        pending_group.member_usernames,
        vec!["coder_ai_bot".to_string(), "linus_t".to_string()]
    );
    assert_eq!(
        pending_group.about.as_deref(),
        Some("1:1 Telegram SourceFiles Rust parity group")
    );

    // 5. Click into search box -> verify 10 Telegram search categories & tags & results
    assert_eq!(TELEGRAM_SEARCH_CATEGORIES.len(), 10);
    assert!(TELEGRAM_SEARCH_TAGS.len() >= 6);
    sidebar.focus_search();
    assert!(sidebar.search_focused);
    assert_eq!(sidebar.active_search_category, "Chats");

    let all_chat_results = sidebar.categorized_search_results();
    assert!(!all_chat_results.is_empty());

    sidebar.select_search_category("Apps");
    assert_eq!(sidebar.active_search_category, "Apps");
    let app_results = sidebar.categorized_search_results();
    assert!(
        app_results.iter().any(|r| r.title.contains("Bot")),
        "Apps category should filter to AI Bot dialogs"
    );
    assert!(
        !app_results.iter().any(|r| r.title == "Linus Torvalds"),
        "Apps category must NOT include human contact dialogs"
    );

    sidebar.select_search_category("Channels");
    let channel_results = sidebar.categorized_search_results();
    assert!(
        channel_results
            .iter()
            .any(|r| r.title == "Telegram GPUI Architects"),
        "Channels category should include created group/channel"
    );

    // Populate remote search results and verify Downloads / Voice filtering does not include unrelated local chats
    sidebar.remote_search_results = vec![
        gpui_telegram_desktop::views::chat_sidebar::CategorizedSearchResult {
            chat_id: created_group_id,
            message_id: 101,
            title: "Telegram GPUI Architects".to_string(),
            subtitle: "architecture_spec.pdf".to_string(),
            category: "files".to_string(),
            timestamp: "刚刚".to_string(),
            avatar_initials: "TG".to_string(),
            avatar_color: 0x9B59B6,
            is_bot: false,
        },
        gpui_telegram_desktop::views::chat_sidebar::CategorizedSearchResult {
            chat_id: created_friend_id,
            message_id: 102,
            title: "Linus Torvalds".to_string(),
            subtitle: "Voice message (0:14)".to_string(),
            category: "voice".to_string(),
            timestamp: "刚刚".to_string(),
            avatar_initials: "LT".to_string(),
            avatar_color: 0x2AABEE,
            is_bot: false,
        },
    ];
    sidebar.select_search_category("Downloads");
    let download_results = sidebar.categorized_search_results();
    assert!(
        download_results
            .iter()
            .any(|r| r.subtitle == "architecture_spec.pdf"),
        "Downloads category must include remote 'files'/'downloads' results"
    );
    sidebar.select_search_category("Voice");
    let voice_results = sidebar.categorized_search_results();
    assert!(
        voice_results
            .iter()
            .any(|r| r.subtitle == "Voice message (0:14)"),
        "Voice category must include remote voice search result"
    );
    assert!(
        voice_results
            .iter()
            .all(|r| r.subtitle.to_lowercase().contains("voice")),
        "Voice category must only return genuine voice results, not unrelated local dialogs"
    );
    assert!(
        !voice_results
            .iter()
            .any(|r| r.title == "Alice Smith" || r.title == "Telegram GPUI Architects"),
        "Unrelated dialogs must be excluded from Voice category"
    );

    sidebar.select_search_tag("#ai_summary");
    assert_eq!(sidebar.active_search_tag, "#ai_summary");
    sidebar.select_search_tag("#ai_summary"); // toggle off back to "all"
    assert_eq!(sidebar.active_search_tag, "all");

    sidebar.exit_search();
    assert!(!sidebar.search_focused);
    assert!(sidebar.search_query.is_empty());

    // 6. Verify Chinese labels match Telegram screenshot media_1794178867127_7a80469d.jpg
    let expected_labels = [
        "聊天", "频道", "应用", "贴文", "图片", "视频", "链接", "文件", "音乐", "语音",
    ];
    let actual_labels: Vec<&str> = TELEGRAM_SEARCH_CATEGORIES.iter().map(|(_, l)| *l).collect();
    assert_eq!(actual_labels, expected_labels);

    // 7. Verify Contacts modal state and folder badge recomputation
    assert!(!sidebar.is_contacts_modal_open);
    sidebar.open_contacts_modal();
    assert!(sidebar.is_contacts_modal_open);
    assert!(!sidebar.is_plus_menu_open);
    sidebar.close_contacts_modal();
    assert!(!sidebar.is_contacts_modal_open);

    sidebar.recompute_folder_badges();
    let all_badge = sidebar
        .folders
        .iter()
        .find(|f| f.0 == "All")
        .map(|f| f.1)
        .unwrap_or(0);
    assert!(all_badge >= 1, "All folder unread badge should reflect real dialog unread counts");
}
