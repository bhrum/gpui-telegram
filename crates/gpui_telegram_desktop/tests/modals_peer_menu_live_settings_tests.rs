//! Regression tests for Telegram 1:1 Modal Popups, Top-Right 3-Dots Peer Menu,
//! In-Chat Search Stepping, Composer `send_on_enter`, and Zero-Fake-State Live Settings.

use gpui_telegram_desktop::theme::{TdesktopThemeMode, TdesktopThemePalette};
use gpui_telegram_desktop::views::boxes::modal_types::{ModalActionOutcome, ModalDialogKind};
use gpui_telegram_desktop::views::composer::KeyAction;
use gpui_telegram_desktop::views::settings::sections::{
    settings_information::ProfileEditField, SettingsActiveSessionsSections,
    SettingsBlockedPeersSections, SettingsChatSections, SettingsFoldersSections,
    SettingsGlobalTtlSections, SettingsInformationSections, SettingsLocalPasscodeSections,
    SettingsLocalStorageSections, SettingsMainSections, SettingsNotificationsSections,
    SettingsPrivacySecuritySections,
};
use gpui_telegram_desktop::views::stage::cards::{
    DeliveryStatus, MessageContent, MessageDirection, MessageItemModel,
};
use gpui_telegram_desktop::views::stage::stage_bars::TopBarUiAction;
use gpui_telegram_desktop::views::{CanonicalComposer, ConversationStage, ModalContainer};
use gpui_telegram_desktop::components::ui::layers::standalone_layer_stack::LayerStackOption;

fn sample_text_msg(id: u64, sender: &str, text: &str) -> MessageItemModel {
    MessageItemModel {
        id,
        sender_id: 100 + id,
        sender_name: sender.to_string(),
        avatar_initials: "U".to_string(),
        avatar_color: 0x2AABEE,
        direction: MessageDirection::Incoming,
        timestamp: "10:00".to_string(),
        delivery_status: DeliveryStatus::Read,
        reply_to: None,
        reactions: Vec::new(),
        content: MessageContent::Text {
            text: text.to_string(),
            is_edited: false,
        },
        bot_keyboard: None,
        is_pinned: false,
    }
}

#[test]
fn test_topbar_three_dots_peer_menu_all_actions_and_ttl_submenu() {
    let mut stage = ConversationStage::new();
    stage.set_chat(77, "Alice Peer", "online", "AP", 0x2AABEE);
    assert!(!stage.more_menu_open);
    assert!(!stage.ttl_submenu_open);

    // 1. Clicking #tb-more toggles dropdown open; clicking TTL expands inline submenu
    stage.apply_topbar_action(TopBarUiAction::ToggleTopMoreMenu);
    assert!(stage.more_menu_open);
    stage.apply_topbar_action(TopBarUiAction::ToggleTtlSubmenu);
    assert!(stage.ttl_submenu_open);

    // 2. Selecting Auto-Delete TTL sets pending_set_ttl_seconds, updates badge state, and closes menu
    stage.apply_topbar_action(TopBarUiAction::SetMessageTtl(86_400));
    assert_eq!(stage.pending_set_ttl_seconds.take(), Some(86_400));
    assert_eq!(stage.message_ttl_seconds, 86_400);
    assert!(!stage.more_menu_open);
    assert!(!stage.ttl_submenu_open);

    // 3. Mute / Pin / ViewProfile / Wallpaper / Export / ClearHistory / Block / DeleteOrLeave
    stage.apply_topbar_action(TopBarUiAction::ToggleTopMoreMenu);
    stage.apply_topbar_action(TopBarUiAction::ToggleMuteChat);
    assert!(stage.pending_toggle_mute);
    assert!(!stage.more_menu_open);
    stage.pending_toggle_mute = false;

    stage.apply_topbar_action(TopBarUiAction::ToggleTopMoreMenu);
    stage.apply_topbar_action(TopBarUiAction::TogglePinChat);
    assert!(stage.pending_toggle_pin_chat);
    assert!(!stage.more_menu_open);
    stage.pending_toggle_pin_chat = false;

    stage.apply_topbar_action(TopBarUiAction::ToggleTopMoreMenu);
    stage.apply_topbar_action(TopBarUiAction::ViewProfile);
    assert_eq!(stage.pending_open_profile_peer_id.take(), Some(77));
    assert!(!stage.more_menu_open);

    stage.apply_topbar_action(TopBarUiAction::ToggleTopMoreMenu);
    stage.apply_topbar_action(TopBarUiAction::ChangeWallpaper);
    assert!(stage.pending_change_wallpaper);
    stage.pending_change_wallpaper = false;

    stage.apply_topbar_action(TopBarUiAction::ToggleTopMoreMenu);
    stage.apply_topbar_action(TopBarUiAction::ExportChat);
    assert!(stage.pending_export_chat);
    stage.pending_export_chat = false;

    stage.apply_topbar_action(TopBarUiAction::ToggleTopMoreMenu);
    stage.apply_topbar_action(TopBarUiAction::ClearHistory);
    assert!(stage.pending_clear_history);
    stage.pending_clear_history = false;

    stage.apply_topbar_action(TopBarUiAction::ToggleTopMoreMenu);
    stage.apply_topbar_action(TopBarUiAction::ToggleBlockPeer);
    assert!(stage.pending_toggle_block_peer);
    stage.pending_toggle_block_peer = false;

    stage.apply_topbar_action(TopBarUiAction::ToggleTopMoreMenu);
    stage.apply_topbar_action(TopBarUiAction::DeleteOrLeaveChat);
    assert!(stage.pending_delete_or_leave_chat);
    stage.pending_delete_or_leave_chat = false;

    // 4. Backdrop click dismisses both more_menu_open and ttl_submenu_open
    stage.apply_topbar_action(TopBarUiAction::ToggleTopMoreMenu);
    stage.apply_topbar_action(TopBarUiAction::ToggleTtlSubmenu);
    stage.apply_topbar_action(TopBarUiAction::DismissTopMoreMenu);
    assert!(!stage.more_menu_open);
    assert!(!stage.ttl_submenu_open);
}

#[test]
fn test_in_chat_search_boundaries_empty_zero_matches_and_wraparound() {
    let mut stage = ConversationStage::new();
    stage.set_chat(88, "Rust Team", "3 members", "RT", 0x2AABEE);
    stage.add_message(sample_text_msg(10, "Alice", "First alpha release"));
    stage.add_message(sample_text_msg(20, "Bob", "Unrelated update"));
    stage.add_message(sample_text_msg(30, "Carol", "Second ALPHA milestone"));

    // Empty & whitespace query boundary: 0 matches, stepping does not panic
    stage.apply_topbar_action(TopBarUiAction::ToggleSearch);
    assert!(stage.search_open);
    stage.search_query = "   ".into();
    stage.recompute_search_matches();
    assert!(stage.search_match_ids.is_empty());
    stage.step_search_match(1);
    stage.step_search_match(-1);
    assert_eq!(stage.search_match_cursor, 0);

    // Non-matching query boundary: 0 matches, stepping +1/-1 safe
    stage.search_query = "nonexistent_token_xyz".into();
    stage.recompute_search_matches();
    assert!(stage.search_match_ids.is_empty());
    stage.step_search_match(-1);
    assert_eq!(stage.search_match_cursor, 0);

    // Case-insensitive reverse-chronological matches: [30, 10]
    stage.search_query = "alpha".into();
    stage.recompute_search_matches();
    assert_eq!(stage.search_match_ids, vec![30, 10]);
    assert_eq!(stage.search_match_cursor, 0);

    // Step forward (+1) -> cursor 1 (msg 10), step forward (+1) -> wraps to 0 (msg 30)
    stage.step_search_match(1);
    assert_eq!(stage.search_match_cursor, 1);
    stage.step_search_match(1);
    assert_eq!(stage.search_match_cursor, 0);

    // Step backward (-1) from 0 -> wraps to len - 1 (cursor 1)
    stage.step_search_match(-1);
    assert_eq!(stage.search_match_cursor, 1);

    // SearchClear resets query, matches, cursor, and closes search bar
    stage.apply_topbar_action(TopBarUiAction::SearchClear);
    assert!(!stage.search_open);
    assert!(stage.search_query.is_empty());
    assert!(stage.search_match_ids.is_empty());
    assert_eq!(stage.search_match_cursor, 0);
}

#[test]
fn test_composer_send_on_enter_setting_flips_enter_and_shift_enter() {
    let mut composer = CanonicalComposer::new();
    assert!(composer.send_on_enter, "Default Telegram Desktop send_on_enter is true");
    composer.set_text("hello");
    assert_eq!(composer.handle_enter(false), KeyAction::SendMessage);
    assert_eq!(composer.handle_enter(true), KeyAction::InsertNewline);

    // Toggle send_on_enter OFF (Ctrl/Shift+Enter mode)
    composer.clear();
    composer.send_on_enter = false;
    composer.set_text("line1");
    assert_eq!(composer.handle_enter(false), KeyAction::InsertNewline);
    assert_eq!(composer.text(), "line1\n");
    assert_eq!(composer.handle_enter(true), KeyAction::SendMessage);
}

#[test]
fn test_live_settings_have_zero_pavel_durov_fake_data_and_real_edit_hooks() {
    let pal = TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x5288C1, 14, 12);

    let main_live = SettingsMainSections::new_live();
    assert_ne!(main_live.profile.name, "Pavel Durov");
    assert!(main_live.profile.username.is_empty());
    assert!(main_live.profile.phone.is_empty());

    let mut info_live = SettingsInformationSections::new_live();
    assert!(info_live.first_name.is_empty());
    assert!(info_live.last_name.is_empty());
    assert!(info_live.username.is_empty());
    assert!(info_live.accounts.is_empty());

    // Inline profile editing + username character filtering + 70-char standard bio cap
    info_live.active_field = ProfileEditField::FirstName;
    info_live.push_field_text("Alice");
    info_live.active_field = ProfileEditField::Username;
    info_live.push_field_text("@alice_gpui!#");
    assert_eq!(info_live.username, "alice_gpui");
    info_live.active_field = ProfileEditField::Bio;
    let long_bio = "x".repeat(95);
    info_live.push_field_text(&long_bio);
    assert_eq!(info_live.bio.chars().count(), 70);
    assert!(info_live.save_pending);

    let sessions_live = SettingsActiveSessionsSections::new_live();
    assert!(sessions_live.sessions.is_empty());

    let blocked_live = SettingsBlockedPeersSections::new_live();
    assert!(blocked_live.blocked_list.is_empty());

    let storage_live = SettingsLocalStorageSections::new_live(pal, "zh-CN");
    assert_eq!(
        storage_live.categories.iter().map(|c| c.bytes).sum::<u64>(),
        0
    );

    let mut passcode_live = SettingsLocalPasscodeSections::new_live();
    assert!(!passcode_live.has_passcode);
    assert!(!passcode_live.set_passcode("   "), "Whitespace-only passcode must be rejected");
    assert!(passcode_live.set_passcode("2468"));
    assert!(passcode_live.has_passcode);
    assert!(!passcode_live.verify_passcode("0000"));
    assert_eq!(passcode_live.bad_tries, 1);
    assert!(passcode_live.verify_passcode("2468"));
    assert_eq!(passcode_live.bad_tries, 0);

    let mut privacy_live = SettingsPrivacySecuritySections::new_live();
    assert!(!privacy_live.two_factor_enabled);
    assert_eq!(privacy_live.blocked_count, 0);
    privacy_live.toggle_sensitive_content();
    assert!(privacy_live.sensitive_content_enabled);
    assert!(privacy_live.pending_save_settings);

    let folders_live = SettingsFoldersSections::new_live(pal, "zh-CN");
    assert!(folders_live.folders.is_empty());

    let mut chat_live = SettingsChatSections::new_live(pal, "zh-CN");
    assert!(chat_live.send_on_enter);
    chat_live.send_on_enter = false;
    assert!(!chat_live.send_on_enter);

    let notif_live = SettingsNotificationsSections::new_live();
    assert!(notif_live.private_chats_enabled);

    let ttl_live = SettingsGlobalTtlSections::new_live();
    assert_eq!(ttl_live.current_ttl, 0);
}

#[test]
fn test_modal_popup_architecture_and_leave_clear_confirmation_dialogs() {
    let mut modal = ModalContainer::new();
    modal.show_dialog(
        "删除对话",
        ModalDialogKind::LeaveOrDeleteChat {
            peer_name: "Bob".into(),
            is_group: false,
        },
        Vec::new(),
        380.0,
        220.0,
        LayerStackOption::CloseOther,
    );
    assert!(modal.dialog.is_open());
    modal.dialog.pending_outcome = Some(ModalActionOutcome::ConfirmLeaveOrDeleteChat);
    assert!(matches!(
        modal.take_outcome(),
        Some(ModalActionOutcome::ConfirmLeaveOrDeleteChat)
    ));

    // Verify critical popup backdrop & deferred paint-order contracts in source
    let shell_src = concat!(
        include_str!("../src/views/shell.rs"),
        "\n",
        include_str!("../src/views/app_shell/shell_overlays_render.rs"),
        "\n",
        include_str!("../src/views/settings/settings_router_render.rs"),
    );
    assert!(shell_src.contains("telegram-settings-modal-backdrop"));
    assert!(shell_src.contains("telegram-settings-modal-box"));
    assert!(shell_src.contains("telegram-calls-modal-backdrop"));
    assert!(shell_src.contains("telegram-calls-modal-box"));
    assert!(shell_src.contains("telegram-creation-modal-backdrop"));
    assert!(shell_src.contains("telegram-creation-modal-box"));

    let sidebar_src = include_str!("../src/views/sidebar/dialogs_renderer.rs");
    assert!(sidebar_src.contains("sidebar-plus-floating-menu"));
    assert!(sidebar_src.contains("deferred("));

    let stage_bars_src = include_str!("../src/views/stage/stage_bars.rs");
    assert!(stage_bars_src.contains("topbar-more-backdrop"));
    assert!(stage_bars_src.contains("topbar-more-dropdown"));
    assert!(stage_bars_src.contains("deferred("));

    let stage_src = include_str!("../src/views/conversation_stage.rs");
    let transcript_idx = stage_src
        .find("let transcript = div()")
        .expect("transcript must exist in ConversationStage::render");
    let more_overlay_idx = stage_src
        .find("render_stage_more_menu_overlay(")
        .expect("more menu overlay must be rendered in ConversationStage::render");
    assert!(
        more_overlay_idx > transcript_idx,
        "render_stage_more_menu_overlay MUST be mounted after transcript so it paints on top!"
    );
    assert!(shell_src.contains("telegram-task-center-modal-backdrop"));
    assert!(shell_src.contains("telegram-task-center-modal-box"));
    assert!(shell_src.contains("telegram-mcp-tools-modal-backdrop"));
    assert!(shell_src.contains("telegram-mcp-tools-modal-box"));
}

#[test]
fn test_stage_set_chat_resets_cross_chat_flags_and_guards_block_peer_in_groups_and_saved() {
    let mut stage = ConversationStage::new();
    stage.set_chat(101, "Blocked Peer", "online", "BP", 0x2AABEE);
    stage.is_muted_chat = true;
    stage.is_pinned_chat = true;
    stage.is_peer_blocked = true;
    stage.message_ttl_seconds = 86_400;

    // Switching to a DIFFERENT chat must reset peer-specific flags so they never bleed across chats!
    stage.set_chat(202, "Fresh Peer", "online", "FP", 0x3390EC);
    assert!(!stage.is_muted_chat);
    assert!(!stage.is_pinned_chat);
    assert!(!stage.is_peer_blocked);
    assert!(!stage.is_group_chat);
    assert!(!stage.is_saved_messages);
    assert_eq!(stage.message_ttl_seconds, 0);

    // Group chat (even without "members" in subtitle!) and Saved Messages must NOT set pending_toggle_block_peer
    stage.is_group_chat = true;
    stage.apply_topbar_action(TopBarUiAction::ToggleBlockPeer);
    assert!(!stage.pending_toggle_block_peer);
    assert!(!stage.is_peer_blocked);

    stage.is_group_chat = false;
    stage.is_saved_messages = true;
    stage.apply_topbar_action(TopBarUiAction::ToggleBlockPeer);
    assert!(!stage.pending_toggle_block_peer);
    assert!(!stage.is_peer_blocked);

    // Direct 1:1 peer chat requests server block WITHOUT prematurely flipping is_peer_blocked before validation
    stage.is_saved_messages = false;
    stage.apply_topbar_action(TopBarUiAction::ToggleBlockPeer);
    assert!(stage.pending_toggle_block_peer);
    assert!(!stage.is_peer_blocked);
}

#[test]
fn test_bilingual_settings_search_byte_formatting_and_preferences_persistence() {
    use gpui_telegram_desktop::app::preferences::AppPreferences;
    use gpui_telegram_desktop::views::settings::settings_search::SettingsSearch;
    use gpui_telegram_desktop::views::settings::settings_type::SettingsSectionType;

    let pal = TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x5288C1, 14, 12);
    let mut search = SettingsSearch::new_live(pal, "zh-CN");

    for (query, expected_sec) in [
        ("隐私", SettingsSectionType::PrivacySecurity),
        ("黑名单", SettingsSectionType::BlockedPeers),
        ("缓存", SettingsSectionType::LocalStorage),
        ("省电", SettingsSectionType::PowerSaving),
        ("星币", SettingsSectionType::Credits),
        ("会员", SettingsSectionType::Premium),
    ] {
        search.set_query(query);
        assert!(
            search.results.iter().any(|r| r.section == expected_sec),
            "Chinese search query '{query}' must match section {expected_sec:?}"
        );
    }

    // Sub-megabyte cache sizes must NOT display as "0 MB"
    assert_eq!(SettingsLocalStorageSections::format_bytes(0), "0 B");
    assert_eq!(SettingsLocalStorageSections::format_bytes(512), "512 B");
    assert_eq!(SettingsLocalStorageSections::format_bytes(4096), "4.0 KB");
    assert_eq!(
        SettingsLocalStorageSections::format_bytes(2 * 1024 * 1024),
        "2 MB"
    );

    // AppPreferences round-trip for newly wired live settings fields
    let tmp = std::env::temp_dir().join(format!("tg_prefs_roundtrip_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&tmp);
    let mut prefs = AppPreferences::default();
    prefs.max_cache_gb = 8;
    prefs.retention_weeks = 2;
    prefs.ask_download_path = true;
    prefs.opengl_enabled = false;
    prefs.native_window_frame = true;
    prefs.tray_icon_enabled = false;
    prefs.auto_update_enabled = false;
    prefs.local_passcode_hash = Some("fnv1a64:deadbeef".into());
    prefs.auto_lock_seconds = 300;
    prefs.power_saving_enabled = true;
    prefs.power_saving_threshold_pct = 30;
    prefs.animate_stickers_chat = false;
    prefs.autoplay_videos = false;
    prefs.audio_input_device = "Built-in Mic".into();
    prefs.folder_tags_enabled = true;
    prefs.save_to_dir(&tmp).expect("save preferences.json");

    let loaded = AppPreferences::load_from_dir(&tmp).expect("load preferences.json");
    let _ = std::fs::remove_dir_all(&tmp);
    assert_eq!(loaded.max_cache_gb, 8);
    assert_eq!(loaded.retention_weeks, 2);
    assert!(loaded.ask_download_path);
    assert!(!loaded.opengl_enabled);
    assert!(loaded.native_window_frame);
    assert!(!loaded.tray_icon_enabled);
    assert!(!loaded.auto_update_enabled);
    assert_eq!(loaded.local_passcode_hash.as_deref(), Some("fnv1a64:deadbeef"));
    assert_eq!(loaded.auto_lock_seconds, 300);
    assert!(loaded.power_saving_enabled);
    assert_eq!(loaded.power_saving_threshold_pct, 30);
    assert!(!loaded.animate_stickers_chat);
    assert!(!loaded.autoplay_videos);
    assert_eq!(loaded.audio_input_device, "Built-in Mic");
    assert!(loaded.folder_tags_enabled);
}

#[test]
fn test_interface_scale_palette_math_and_wallpaper_blur_tokens() {
    let base = TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x3390EC, 14, 12);
    assert_eq!(base.ui_scale_pct, 100);
    assert!((base.scaled_px(16.0) - 16.0).abs() < 0.001);
    assert!(!base.blur_wallpaper);

    for (pct, expected_16) in [(100, 16.0), (125, 20.0), (150, 24.0), (200, 32.0)] {
        let p = base.with_ui_scale(pct);
        assert_eq!(p.ui_scale_pct, pct);
        assert!((p.scaled_px(16.0) - expected_16).abs() < 0.01);
    }

    // Extreme boundary clamping (50%..=300%)
    assert_eq!(base.with_ui_scale(10).ui_scale_pct, 50);
    assert!((base.with_ui_scale(10).scaled_px(20.0) - 10.0).abs() < 0.01);
    assert_eq!(base.with_ui_scale(999).ui_scale_pct, 300);
    assert!((base.with_ui_scale(999).scaled_px(20.0) - 60.0).abs() < 0.01);

    let blurred = base.with_blur_wallpaper(true);
    assert!(blurred.blur_wallpaper);
}

#[test]
fn test_sidebar_custom_folders_rule_filtering_authoritative_preservation_and_badge_modes() {
    use gpui_telegram_client::ChatSummary;
    use gpui_telegram_desktop::views::app_shell::shell_truth::apply_authoritative_chat_list;
    use gpui_telegram_desktop::views::chat_sidebar::DialogItemModel;
    use gpui_telegram_desktop::views::{ChatSidebar, ContextInspector};
    use gpui_telegram_proto::contracts::chats::ChatFolderDto;

    let mut sb = ChatSidebar::new();
    let custom = vec![
        ChatFolderDto {
            id: 10,
            title: "Work".to_string(),
            emoticon: Some("💼".to_string()),
            pinned_chats: vec![301],
            include_chats: vec![101],
            exclude_chats: vec![202],
            include_contacts: false,
            include_non_contacts: false,
            include_groups: true,
            include_channels: false,
            include_bots: false,
            exclude_muted: false,
            exclude_read: false,
            exclude_archived: true,
            sort_order: 0,
        },
        ChatFolderDto {
            id: 11,
            title: "AI Bots".to_string(),
            emoticon: Some("🤖".to_string()),
            include_bots: true,
            exclude_archived: true,
            sort_order: 1,
            ..Default::default()
        },
    ];
    sb.set_custom_folders(custom);
    assert!(sb.folders.iter().any(|(name, _)| name == "Work"));
    assert!(sb.folders.iter().any(|(name, _)| name == "AI Bots"));

    // Simulate authoritative refresh_chats: custom folders MUST NOT be wiped!
    let mut st = ConversationStage::new();
    let mut insp = ContextInspector::new();
    let server_chats = vec![
        ChatSummary {
            chat_id: 101,
            title: "Alice Contact".to_string(),
            chat_type: 0,
            unread_count: 3,
            last_message: Some("hi".to_string()),
            is_group: Some(false),
            is_bot: Some(false),
            avatar_color: Some(0x2AABEE),
            updated_at: Some(1_700_000_000_000),
        },
        ChatSummary {
            chat_id: 202,
            title: "Muted Noisy Group".to_string(),
            chat_type: 1,
            unread_count: 5,
            last_message: Some("noise".to_string()),
            is_group: Some(true),
            is_bot: Some(false),
            avatar_color: Some(0x9B59B6),
            updated_at: Some(1_700_000_001_000),
        },
        ChatSummary {
            chat_id: 301,
            title: "Engineering Group".to_string(),
            chat_type: 1,
            unread_count: 2,
            last_message: Some("ship it".to_string()),
            is_group: Some(true),
            is_bot: Some(false),
            avatar_color: Some(0x3498DB),
            updated_at: Some(1_700_000_002_000),
        },
        ChatSummary {
            chat_id: 404,
            title: "Hermes Bot".to_string(),
            chat_type: 0,
            unread_count: 4,
            last_message: Some("ready".to_string()),
            is_group: Some(false),
            is_bot: Some(true),
            avatar_color: Some(0x1ABC9C),
            updated_at: Some(1_700_000_003_000),
        },
    ];
    apply_authoritative_chat_list(&mut sb, &mut st, &mut insp, &server_chats);
    sb.toggle_mute_dialog(202);
    sb.toggle_mute_dialog(404);
    // Re-apply authoritative chat list to prove both custom folders AND mute flags survive refresh_chats!
    apply_authoritative_chat_list(&mut sb, &mut st, &mut insp, &server_chats);

    assert!(sb.folders.iter().any(|(name, _)| name == "Work"));
    assert!(sb.folders.iter().any(|(name, _)| name == "AI Bots"));

    // Select "Work": includes group 301 (pinned in folder), includes 101 (explicit include_chats), excludes 202 (exclude_chats), excludes 404 (bot, not group)
    sb.select_folder("Work");
    let work_ids: Vec<u64> = sb.filtered_dialogs().iter().map(|d| d.id).collect();
    assert_eq!(
        work_ids,
        vec![301, 101],
        "Pinned folder chat 301 must appear first, 101 included, 202 & 404 excluded"
    );

    // Select "AI Bots": includes 404 only
    sb.select_folder("AI Bots");
    let bot_ids: Vec<u64> = sb.filtered_dialogs().iter().map(|d| d.id).collect();
    assert_eq!(bot_ids, vec![404]);

    // Folder tags for dialog rows when folder_tags_enabled is toggled
    sb.select_folder("All");
    sb.folder_tags_enabled = false;
    let d101: DialogItemModel = sb.dialogs.iter().find(|d| d.id == 101).unwrap().clone();
    assert!(sb.dialog_meta(&d101).folder_tags.is_empty());
    sb.folder_tags_enabled = true;
    assert_eq!(sb.dialog_meta(&d101).folder_tags, vec!["Work".to_string()]);

    // Notification badge modes (set_notification_badge_rules(badge_count_messages, include_muted_in_badge)):
    // Unmuted unread: chat 101 (3 msgs), chat 301 (2 msgs) -> 5 msgs across 2 chats
    // Muted unread: chat 202 (5 msgs), chat 404 (4 msgs) -> 9 msgs across 2 chats
    sb.set_notification_badge_rules(true, false); // count messages, exclude muted
    assert_eq!(sb.total_unread_badge(), 5);
    assert_eq!(
        sb.folders.iter().find(|(n, _)| n == "Work").map(|(_, b)| *b),
        Some(5)
    );
    assert_eq!(
        sb.folders.iter().find(|(n, _)| n == "AI Bots").map(|(_, b)| *b),
        Some(0)
    );

    sb.set_notification_badge_rules(false, false); // count chats, exclude muted
    assert_eq!(sb.total_unread_badge(), 2);

    sb.set_notification_badge_rules(true, true); // count messages, include muted
    assert_eq!(sb.total_unread_badge(), 14);
    assert_eq!(
        sb.folders.iter().find(|(n, _)| n == "AI Bots").map(|(_, b)| *b),
        Some(4)
    );

    sb.set_notification_badge_rules(false, true); // count chats, include muted
    assert_eq!(sb.total_unread_badge(), 4);

    // Deleting active custom folder falls back cleanly to "All"
    sb.select_folder("Work");
    sb.set_custom_folders(Vec::new());
    assert_eq!(sb.active_folder, "All");
}

#[test]
fn test_advanced_proxy_no_premature_mutation_and_stage_select_messages_mode() {
    use gpui_telegram_desktop::views::settings::sections::settings_advanced::{
        ProxyType, SettingsAdvancedSections,
    };

    let pal = TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x3390EC, 14, 12);
    let mut adv = SettingsAdvancedSections::new_live(pal, "zh-CN");
    assert_eq!(adv.proxy_type, ProxyType::Direct);

    // Opening Connection Type modal does NOT mutate proxy_type before user saves!
    adv.pending_open_connection_box = true;
    assert_eq!(adv.proxy_type, ProxyType::Direct);

    // Syncing saved SOCKS5 / MTProto / HTTP / Direct proxies updates display state accurately
    adv.sync_proxy("SOCKS5", "127.0.0.1", 1080, true);
    assert_eq!(adv.proxy_type, ProxyType::Socks5);
    assert_eq!(adv.proxy_host, "127.0.0.1");
    assert_eq!(adv.proxy_port, 1080);

    adv.sync_proxy("MTProto", "mtproxy.example.org", 8443, true);
    assert_eq!(adv.proxy_type, ProxyType::Mtproto);

    adv.sync_proxy("SOCKS5", "127.0.0.1", 1080, false);
    assert_eq!(adv.proxy_type, ProxyType::Direct);
    assert!(adv.proxy_host.is_empty());

    // Host audio device enumeration returns graceful non-empty lists even in headless CI
    let (inputs, outputs) = gpui_telegram_media::enumerate_audio_devices();
    assert!(!inputs.is_empty());
    assert!(!outputs.is_empty());

    // Top-right 3-dots "Select Messages" enters select_mode_active with 0 selected messages,
    // allows toggling individual messages, and exits cleanly via CancelSelection / clear_selected_messages
    let mut stage = ConversationStage::new();
    stage.set_chat(55, "Peer", "online", "P", 0x2AABEE);
    stage.add_message(sample_text_msg(1, "Peer", "Msg 1"));
    stage.add_message(sample_text_msg(2, "Peer", "Msg 2"));

    stage.apply_topbar_action(TopBarUiAction::ToggleTopMoreMenu);
    assert!(stage.more_menu_open);
    stage.apply_topbar_action(TopBarUiAction::EnterSelectMode);
    assert!(!stage.more_menu_open);
    assert!(stage.select_mode_active);
    assert!(stage.is_selecting_messages());
    assert!(stage.selected_message_ids.is_empty());

    stage.toggle_select_message(1);
    assert_eq!(stage.selected_message_ids, vec![1]);
    stage.apply_topbar_action(TopBarUiAction::ClearSelection);
    assert!(!stage.select_mode_active);
    assert!(!stage.is_selecting_messages());
    assert!(stage.selected_message_ids.is_empty());
}

const HEADLESS_CHILD_ENV: &str = "GPUI_ROUND2_HEADLESS_CHILD";

#[cfg(target_os = "macos")]
#[used]
#[unsafe(link_section = "__DATA,__mod_init_func")]
static MACOS_ROUND2_HEADLESS_MAIN_THREAD_ENTRY: extern "C" fn() =
    macos_round2_mod_init_headless_entry;

#[cfg(target_os = "macos")]
extern "C" fn macos_round2_mod_init_headless_entry() {
    if std::env::var(HEADLESS_CHILD_ENV).as_deref() == Ok("1") {
        run_headless_appshell_round2_child();
    }
}

fn run_headless_appshell_round2_child() -> ! {
    use gpui::{App, AppContext as _, Application};
    use gpui_telegram_client::session_persistence::ProxyEndpointRecord;
    use gpui_telegram_client::{ClientConfig, TelegramClient};
    use gpui_telegram_desktop::theme::TelegramAssetSource;
    use gpui_telegram_desktop::views::settings::sections::settings_advanced::ProxyType;
    use gpui_telegram_desktop::views::settings::settings_type::SettingsSectionType;
    use gpui_telegram_desktop::views::settings::SettingsRouterEvent;
    use gpui_telegram_desktop::views::AppShell;
    use std::path::PathBuf;
    use std::sync::Arc;

    let outer = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        Application::headless()
            .with_assets(TelegramAssetSource::new())
            .run(|cx: &mut App| {
                let inner = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let temp_path = PathBuf::from(
                        std::env::var("GPUI_TELEGRAM_CONFIG_DIR")
                            .expect("GPUI_TELEGRAM_CONFIG_DIR set by parent"),
                    );
                    let cfg = ClientConfig {
                        server_url: "http://127.0.0.1:9".to_string(),
                        http_timeout_secs: 2,
                        ws_ping_interval_secs: 25,
                        max_reconnect_attempts: 1,
                        ..ClientConfig::default()
                    };
                    let client1 = Arc::new(
                        TelegramClient::open_with_dir(cfg, &temp_path)
                            .expect("open isolated client"),
                    );
                    let _rt_guard = client1.runtime().enter();

                    // 1. Clean AppShell with NO local_passcode_hash MUST start unlocked and with non-inverted default badge rules!
                    let shell1 = cx.new(|cx| {
                        let mut s = AppShell::new_with_client(client1.clone(), cx);
                        s.is_authenticated = true;
                        s
                    });
                    assert!(!shell1.read(cx).overlays.is_passcode_locked);
                    assert!(!shell1.update(cx, |s, cx| s.lock_app_now(cx)));
                    assert!(
                        shell1.read(cx).sidebar.read(cx).badge_count_messages,
                        "Default AppShell startup must count unread messages (not inverted)"
                    );
                    assert!(
                        !shell1.read(cx).sidebar.read(cx).include_muted_in_badge,
                        "Default AppShell startup must exclude muted chats from badge (not inverted)"
                    );

                    // 2. Live Interface Scale (100% -> 150%) updates palette & persists to disk
                    shell1.update(cx, |s, cx| {
                        s.handle_settings_router_event(
                            &SettingsRouterEvent::UpdateInterfaceScale(150),
                            cx,
                        );
                    });
                    assert_eq!(shell1.read(cx).preferences.interface_scale_pct, 150);
                    assert_eq!(shell1.read(cx).theme_state.palette.ui_scale_pct, 150);
                    assert_eq!(shell1.read(cx).stage.read(cx).palette.ui_scale_pct, 150);
                    assert_eq!(shell1.read(cx).sidebar.read(cx).palette.ui_scale_pct, 150);

                    // 3. Live Chat Appearance (bubble_radius=16, font_size=18, blur_wallpaper=true, loop_animated_stickers=false, suggest_stickers=false, send_on_enter=false)
                    shell1.update(cx, |s, cx| {
                        s.handle_settings_router_event(
                            &SettingsRouterEvent::UpdateChatPrefs {
                                font_size: 18,
                                bubble_radius: 16,
                                send_on_enter: false,
                                suggest_stickers: false,
                                loop_animated_stickers: false,
                                blur_wallpaper: true,
                            },
                            cx,
                        );
                    });
                    {
                        let s = shell1.read(cx);
                        assert_eq!(s.preferences.appearance.bubble_radius, 16);
                        assert_eq!(s.preferences.appearance.font_size, 18);
                        assert!(s.preferences.appearance.blur_wallpaper);
                        assert!(!s.preferences.appearance.loop_animated_stickers);
                        assert_eq!(s.theme_state.palette.bubble_radius_px, 16);
                        assert_eq!(s.theme_state.palette.font_size_px, 18);
                        assert!(s.theme_state.palette.blur_wallpaper);
                        assert!(s.stage.read(cx).palette.blur_wallpaper);
                        assert!(!s.stage.read(cx).loop_animated_stickers);
                        assert!(!s.composer.read(cx).send_on_enter);
                        assert!(!s.composer.read(cx).suggest_stickers_by_emoji);
                        assert!(!s.composer.read(cx).loop_animated_stickers);
                    }

                    // 4. Notification reactions/types, folder tags, proxy outcome, passkeys, websites, shortcuts, business
                    shell1.update(cx, |s, cx| {
                        s.handle_settings_router_event(
                            &SettingsRouterEvent::UpdateFolderTags(true),
                            cx,
                        );
                        s.handle_settings_router_event(
                            &SettingsRouterEvent::UpdateNotificationReactionsPrefs {
                                notify_reactions: false,
                                notify_stories: false,
                                contacts_only: true,
                                sound_enabled: false,
                            },
                            cx,
                        );
                        s.handle_settings_router_event(
                            &SettingsRouterEvent::UpdateNotificationTypePrefs {
                                alerts_enabled: false,
                                preview_enabled: false,
                                sound_enabled: false,
                            },
                            cx,
                        );
                        s.handle_settings_router_event(
                            &SettingsRouterEvent::UpdatePasskeysPrefs(vec![
                                "MacBook Touch ID".into()
                            ]),
                            cx,
                        );
                        s.handle_settings_router_event(
                            &SettingsRouterEvent::UpdateWebsitesPrefs(vec!["fragment.com".into()]),
                            cx,
                        );
                        s.handle_settings_router_event(
                            &SettingsRouterEvent::UpdateShortcutsPrefs {
                                enabled: false,
                                filter: "lock".into(),
                            },
                            cx,
                        );
                        s.handle_settings_router_event(
                            &SettingsRouterEvent::UpdateBusinessPrefs(vec![
                                "greeting".into(),
                                "away".into(),
                            ]),
                            cx,
                        );
                        s.handle_settings_router_event(
                            &SettingsRouterEvent::UpdateNotificationPrefs {
                                private_chats: true,
                                groups: true,
                                channels: false,
                                sound_enabled: true,
                                flash_taskbar: true,
                                include_muted_in_badge: true,
                                badge_count_messages: false,
                            },
                            cx,
                        );
                        s.handle_modal_outcome(
                            ModalActionOutcome::AddProxy(ProxyEndpointRecord {
                                id: 99,
                                kind: "SOCKS5".into(),
                                host: "10.0.0.8".into(),
                                port: 1088,
                                secret_or_user: String::new(),
                                username: None,
                                password: None,
                                ping_ms: 12,
                                enabled: true,
                            }),
                            cx,
                        );
                    });
                    {
                        let s = shell1.read(cx);
                        assert!(s.sidebar.read(cx).folder_tags_enabled);
                        assert!(
                            !s.sidebar.read(cx).badge_count_messages,
                            "UpdateNotificationPrefs(badge_count_messages=false) must set CountChats"
                        );
                        assert!(
                            s.sidebar.read(cx).include_muted_in_badge,
                            "UpdateNotificationPrefs(include_muted_in_badge=true) must include muted chats"
                        );
                        let adv = s
                            .settings_router
                            .read(cx)
                            .settings_advanced
                            .as_ref()
                            .expect("advanced materialized")
                            .read(cx);
                        assert_eq!(adv.proxy_type, ProxyType::Socks5);
                        assert_eq!(adv.proxy_host, "10.0.0.8");
                        assert_eq!(adv.proxy_port, 1088);
                    }

                    // 4b. SelectProxy switching between saved proxy and Direct (proxy_id=0)
                    shell1.update(cx, |s, cx| {
                        s.modal_container.update(cx, |m, _| {
                            m.dialog.proxies = vec![ProxyEndpointRecord {
                                id: 77,
                                kind: "MTProto".into(),
                                host: "mt.telegram.example".into(),
                                port: 443,
                                secret_or_user: "ee1122".into(),
                                username: None,
                                password: None,
                                ping_ms: 8,
                                enabled: true,
                            }];
                        });
                        s.handle_modal_outcome(ModalActionOutcome::SelectProxy { proxy_id: 77 }, cx);
                    });
                    {
                        let adv = shell1
                            .read(cx)
                            .settings_router
                            .read(cx)
                            .settings_advanced
                            .as_ref()
                            .unwrap()
                            .read(cx);
                        assert_eq!(adv.proxy_type, ProxyType::Mtproto);
                        assert_eq!(adv.proxy_host, "mt.telegram.example");
                        assert_eq!(adv.proxy_port, 443);
                    }
                    shell1.update(cx, |s, cx| {
                        s.handle_modal_outcome(ModalActionOutcome::SelectProxy { proxy_id: 0 }, cx);
                    });
                    {
                        let adv = shell1
                            .read(cx)
                            .settings_router
                            .read(cx)
                            .settings_advanced
                            .as_ref()
                            .unwrap()
                            .read(cx);
                        assert_eq!(adv.proxy_type, ProxyType::Direct);
                        assert!(adv.proxy_host.is_empty());
                    }

                    // 5. Live Mic Test metering via external PCM feed + clean teardown when closing Settings
                    shell1.update(cx, |s, cx| {
                        s.navigate_settings_to(SettingsSectionType::Calls, cx);
                        s.rtc_talk
                            .voice_recorder
                            .feed_external_pcm_i16(&[6000i16; 320]);
                        s.handle_settings_router_event(&SettingsRouterEvent::ToggleMicTest(true), cx);
                    });
                    assert!(shell1.read(cx).rtc_talk.voice_recorder.current_level() > 0);
                    shell1.update(cx, |s, cx| s.close_settings(cx));
                    assert!(!shell1.read(cx).rtc_talk.voice_recorder.is_recording());

                    // 6. Set Local Passcode ("2580", auto_lock = 60s), verify LockNow, input isolation, Escape block, wrong PIN, unlock, and idle auto-lock!
                    shell1.update(cx, |s, cx| {
                        s.update_preferences(cx, |p| p.localization.language_code = "zh-CN".into());
                        s.handle_modal_outcome(
                            ModalActionOutcome::SetLocalPasscode(Some("2580".into())),
                            cx,
                        );
                        s.update_preferences(cx, |p| p.auto_lock_seconds = 60);
                    });
                    assert!(shell1.read(cx).has_local_passcode_configured());

                    // Idle auto-lock boundary: 59s does NOT lock, 60s DOES lock!
                    assert!(!shell1.update(cx, |s, cx| s.check_idle_auto_lock(59, cx)));
                    assert!(!shell1.read(cx).overlays.is_passcode_locked);
                    assert!(shell1.update(cx, |s, cx| s.check_idle_auto_lock(60, cx)));
                    assert!(shell1.read(cx).overlays.is_passcode_locked);

                    // While locked: typing goes ONLY to passcode_unlock_input (composer stays empty!) and Escape cannot bypass lock!
                    shell1.update(cx, |s, cx| {
                        s.insert_committed_text("9999", cx);
                    });
                    assert_eq!(shell1.read(cx).overlays.passcode_unlock_input, "9999");
                    assert_eq!(shell1.read(cx).composer.read(cx).text(), "");

                    // Wrong PIN rejected with localized Chinese error
                    shell1.update(cx, |s, cx| {
                        s.handle_non_char_key("enter", false, cx);
                    });
                    assert!(shell1.read(cx).overlays.is_passcode_locked);
                    assert!(shell1
                        .read(cx)
                        .overlays
                        .passcode_unlock_error
                        .as_deref()
                        .unwrap_or("")
                        .contains("密码"));

                    // Escape clears input/error but keeps lockscreen active!
                    shell1.update(cx, |s, cx| {
                        s.insert_committed_text("12", cx);
                        s.handle_non_char_key("escape", false, cx);
                    });
                    assert!(shell1.read(cx).overlays.is_passcode_locked);
                    assert!(shell1.read(cx).overlays.passcode_unlock_input.is_empty());

                    // Correct PIN ("2580") + Enter unlocks!
                    shell1.update(cx, |s, cx| {
                        s.insert_committed_text("2580", cx);
                        s.handle_non_char_key("enter", false, cx);
                    });
                    assert!(!shell1.read(cx).overlays.is_passcode_locked);

                    // Stage multi-select Escape dismissal and CopySelected auto-exit when unlocked
                    shell1.update(cx, |s, cx| {
                        s.stage.update(cx, |st, _| {
                            st.apply_topbar_action(TopBarUiAction::EnterSelectMode);
                        });
                    });
                    assert!(shell1.read(cx).stage.read(cx).is_selecting_messages());
                    shell1.update(cx, |s, cx| {
                        assert!(s.handle_non_char_key("escape", false, cx));
                    });
                    assert!(!shell1.read(cx).stage.read(cx).is_selecting_messages());

                    shell1.update(cx, |s, cx| {
                        s.stage.update(cx, |st, st_cx| {
                            st.apply_topbar_action(TopBarUiAction::EnterSelectMode);
                            st.handle_topbar_action(TopBarUiAction::CopySelected, st_cx);
                        });
                    });
                    assert!(
                        !shell1.read(cx).stage.read(cx).is_selecting_messages(),
                        "CopySelected must clear select_mode_active as well as selected_message_ids"
                    );

                    // 7. Restart persistence: constructing a second AppShell against the same persisted config dir
                    // hydrates interface_scale_pct=150, bubble_radius=16, blur_wallpaper=true, notification badge rules, passkeys/websites/business, AND starts LOCKED because local_passcode_hash is non-empty!
                    let shell2 = cx.new(|cx| AppShell::new_with_client(client1.clone(), cx));
                    {
                        let s2 = shell2.read(cx);
                        assert!(
                            s2.overlays.is_passcode_locked,
                            "Restarted AppShell with persisted passcode hash MUST start locked"
                        );
                        assert_eq!(s2.preferences.interface_scale_pct, 150);
                        assert_eq!(s2.theme_state.palette.ui_scale_pct, 150);
                        assert_eq!(s2.preferences.appearance.bubble_radius, 16);
                        assert_eq!(s2.theme_state.palette.bubble_radius_px, 16);
                        assert!(s2.preferences.appearance.blur_wallpaper);
                        assert!(!s2.sidebar.read(cx).badge_count_messages);
                        assert!(s2.sidebar.read(cx).include_muted_in_badge);
                        assert_eq!(s2.preferences.passkey_names, vec!["MacBook Touch ID"]);
                        assert_eq!(
                            s2.preferences.connected_website_domains,
                            vec!["fragment.com"]
                        );
                        assert!(!s2.preferences.shortcuts_enabled);
                        assert_eq!(s2.preferences.shortcuts_filter, "lock");
                        assert_eq!(
                            s2.preferences.business_configured_tools,
                            vec!["greeting", "away"]
                        );
                    }
                    assert!(shell2.update(cx, |s, cx| s.try_unlock_with_passcode("2580", cx)));
                    assert!(!shell2.read(cx).overlays.is_passcode_locked);
                }));
                match inner {
                    Ok(()) => std::process::exit(0),
                    Err(payload) => {
                        let msg = payload
                            .downcast_ref::<&str>()
                            .copied()
                            .or_else(|| payload.downcast_ref::<String>().map(|s| s.as_str()))
                            .unwrap_or("panic");
                        eprintln!("[headless round2 child panic] {msg}");
                        std::process::exit(101);
                    }
                }
            });
    }));
    if let Err(payload) = outer {
        let msg = payload
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| payload.downcast_ref::<String>().map(|s| s.as_str()))
            .unwrap_or("panic");
        eprintln!("[headless round2 outer panic] {msg}");
        std::process::exit(101);
    }
    std::process::exit(0);
}

#[test]
fn test_headless_appshell_passcode_lock_scale_chat_appearance_mic_and_settings_persistence() {
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    if std::env::var(HEADLESS_CHILD_ENV).as_deref() == Ok("1") {
        run_headless_appshell_round2_child();
    }

    static NEXT_TMP_SEQ: AtomicU64 = AtomicU64::new(1);
    struct TempDirGuard(PathBuf);
    impl TempDirGuard {
        fn new(prefix: &str) -> Self {
            let seq = NEXT_TMP_SEQ.fetch_add(1, Ordering::Relaxed);
            let p = std::env::temp_dir().join(format!(
                "gpui_tg_{prefix}_{}_{seq}",
                std::process::id()
            ));
            std::fs::create_dir_all(&p).expect("create RAII temp dir");
            Self(p)
        }
    }
    impl Drop for TempDirGuard {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    let temp_dir = TempDirGuard::new("appshell_round2");
    let exe = std::env::current_exe().expect("current_exe");
    let output = std::process::Command::new(exe)
        .env(HEADLESS_CHILD_ENV, "1")
        .env("GPUI_TELEGRAM_CONFIG_DIR", &temp_dir.0)
        .env("GPUI_OFFLINE_TEST", "1")
        .arg("--exact")
        .arg("test_headless_appshell_passcode_lock_scale_chat_appearance_mic_and_settings_persistence")
        .arg("--nocapture")
        .output()
        .expect("spawn headless gpui round2 child");

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!(
            "headless round2 child failed (status {:?})\n--- stdout ---\n{stdout}\n--- stderr ---\n{stderr}",
            output.status.code()
        );
    }
}

#[test]
fn test_reviewer_regressions_mute_badge_recompute_direct_proxy_global_ttl_reset_and_zh_labels() {
    use gpui_telegram_desktop::views::chat_sidebar::{ChatSidebar, DialogDeliveryStatus, DialogItemModel};
    use gpui_telegram_desktop::views::settings::sections::settings_advanced::{ProxyType, SettingsAdvancedSections};
    use gpui_telegram_desktop::views::settings::sections::settings_credits::SettingsCreditsSections;
    use gpui_telegram_desktop::views::settings::sections::settings_global_ttl::SettingsGlobalTtlSections;
    use gpui_telegram_desktop::views::sidebar::dialogs_overlays::localize_folder_label;

    // 1. Muting/unmuting a chat while `include_muted_in_badge == false` MUST immediately recompute folder unread badges
    let mut sb = ChatSidebar::new();
    sb.set_language("zh-CN");
    sb.dialogs = vec![DialogItemModel {
        id: 501,
        peer_name: "Alice".into(),
        avatar_initials: "A".into(),
        avatar_color: 0x2AABEE,
        last_message: "Hi".into(),
        timestamp: "10:00".into(),
        unread_count: 4,
        is_muted: false,
        is_pinned: false,
        is_online: true,
        typing_indicator: None,
        delivery_status: DialogDeliveryStatus::Read,
        folder: "Personal".into(),
    }];
    sb.set_notification_badge_rules(true, false);
    assert_eq!(
        sb.folders.iter().find(|(n, _)| n == "All").map(|(_, c)| *c),
        Some(4)
    );
    sb.toggle_mute_dialog(501);
    assert_eq!(
        sb.folders.iter().find(|(n, _)| n == "All").map(|(_, c)| *c),
        Some(0),
        "Muting a chat when include_muted_in_badge=false must recompute All badge to 0"
    );
    sb.toggle_mute_dialog(501);
    assert_eq!(
        sb.folders.iter().find(|(n, _)| n == "All").map(|(_, c)| *c),
        Some(4),
        "Unmuting a chat must restore All badge to 4"
    );

    // 2. SettingsAdvancedSections::sync_proxy("Direct", ...) never maps to SOCKS5 even if host is non-empty
    let mut adv = SettingsAdvancedSections::new();
    adv.sync_proxy("SOCKS5", "127.0.0.1", 1080, true);
    assert_eq!(adv.proxy_type, ProxyType::Socks5);
    adv.sync_proxy("Direct", "127.0.0.1", 1080, true);
    assert_eq!(adv.proxy_type, ProxyType::Direct);
    assert!(adv.proxy_host.is_empty());

    // 3. SettingsGlobalTtlSections::apply_to_existing works at TTL_OFF (0) to batch-disable existing private chat TTLs
    let mut gttl = SettingsGlobalTtlSections::new_live();
    gttl.set_language("zh-CN");
    gttl.existing_chats_count = 3;
    assert_eq!(gttl.current_ttl, 0);
    gttl.apply_to_existing();
    assert_eq!(gttl.pending_apply_to_existing, Some(0));

    // 4. Built-in folder tab labels & Credits top-up entries localize cleanly under zh-CN while preserving custom folder titles
    assert_eq!(localize_folder_label("All", "zh-CN"), "全部");
    assert_eq!(localize_folder_label("Bots", "zh-CN"), "机器人");
    assert_eq!(localize_folder_label("Groups", "zh-CN"), "群组");
    assert_eq!(localize_folder_label("Channels", "zh-CN"), "频道");
    assert_eq!(localize_folder_label("Personal", "zh-CN"), "私聊");
    assert_eq!(localize_folder_label("VIP 客户", "zh-CN"), "VIP 客户");
    assert_eq!(localize_folder_label("All", "en-US"), "All");

    let mut credits = SettingsCreditsSections::new();
    credits.set_language("zh-CN");
    credits.topup(500);
    assert_eq!(credits.transactions[0].title, "星币充值");
    assert_eq!(credits.transactions[0].date, "刚刚");
}
