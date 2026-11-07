//! Comprehensive integration tests for Module 04: Window, App Shell & Themes Subsystem
//! Verifies WindowController, WindowSessionController, MainWindow, WindowAdaptive, MainMenu, and Themes.

use gpui_telegram_desktop::views::app_shell::*;
use gpui_telegram_desktop::views::app_shell::themes::*;
use gpui_telegram_desktop::window::*;

#[test]
fn test_window_controller_lifecycle_and_layers() {
    let mut controller = WindowController::new();
    assert!(controller.is_primary());
    assert!(!controller.locked());
    assert!(!controller.is_layer_shown());

    // Modal layer push and pop
    controller.show_box("Test Dialog Box");
    assert!(controller.is_layer_shown());
    assert_eq!(controller.active_layers.len(), 1);

    controller.show_layer(LayerKind::Settings);
    assert_eq!(controller.active_layers.len(), 2);

    let popped = controller.hide_layer();
    assert_eq!(popped, Some(LayerKind::Settings));
    assert_eq!(controller.active_layers.len(), 1);

    let dismissed = controller.close_layer_by_back_button();
    assert!(dismissed);
    assert!(!controller.is_layer_shown());

    // Passcode lock
    controller.setup_passcode_lock();
    assert!(controller.locked());
    // Cannot dismiss lock layer via back button
    assert!(!controller.close_layer_by_back_button());
    controller.clear_passcode_lock();
    assert!(!controller.locked());

    // Toast notifications
    controller.show_toast("Message Sent", 2000);
    assert!(controller.current_toast.is_some());
    assert_eq!(controller.current_toast.as_ref().unwrap().text, "Message Sent");

    controller.show_toast("Second Toast", 3000);
    assert_eq!(controller.toast_queue.len(), 1);

    controller.dismiss_current_toast();
    assert_eq!(controller.current_toast.as_ref().unwrap().text, "Second Toast");
    assert_eq!(controller.toast_queue.len(), 0);
}

#[test]
fn test_main_window_state_and_work_mode() {
    let mut main_win = MainWindow::new();
    assert!(main_win.is_active());
    assert_eq!(main_win.work_mode, WorkMode::WindowAndTray);

    main_win.set_title("Telegram Desktop");
    assert_eq!(main_win.title, "Telegram Desktop");

    main_win.set_unread_badge(42);
    assert_eq!(main_win.unread_badge, 42);

    // Hide no quit minimizes to tray
    let handled = main_win.hide_no_quit();
    assert!(handled);
    assert!(main_win.is_minimized);

    main_win.unminimize();
    assert!(!main_win.is_minimized);
    assert!(main_win.is_active());

    // Right column toggle
    assert!(!main_win.show_right_column);
    main_win.toggle_right_column();
    assert!(main_win.show_right_column);
}

#[test]
fn test_window_session_controller_navigation_stack() {
    let mut session = WindowSessionController::with_session("user_session_1".to_string());
    assert_eq!(session.current_section, NavigationSection::DialogsList);
    assert_eq!(session.navigation_stack.len(), 0);

    // Navigate to Chat History
    session.show_peer_history(1001, Some(55));
    assert_eq!(session.current_section, NavigationSection::History { peer_id: 1001, msg_id: Some(55) });
    assert_eq!(session.navigation_stack.len(), 1);
    assert_eq!(session.active_peer_id, Some(1001));

    // Navigate deeper to Peer Info
    session.show_peer_info(1001);
    assert_eq!(session.current_section, NavigationSection::PeerInfo { peer_id: 1001 });
    assert_eq!(session.navigation_stack.len(), 2);

    // Navigate back
    let back_ok = session.navigate_back();
    assert!(back_ok);
    assert_eq!(session.current_section, NavigationSection::History { peer_id: 1001, msg_id: Some(55) });
    assert_eq!(session.active_peer_id, Some(1001));

    // Navigate back to root
    let back_ok2 = session.navigate_back();
    assert!(back_ok2);
    assert_eq!(session.current_section, NavigationSection::DialogsList);
    assert_eq!(session.active_peer_id, None);

    // Clear stack
    session.show_settings("Notifications");
    assert_eq!(session.navigation_stack.len(), 1);
    session.clear_section_stack();
    assert_eq!(session.navigation_stack.len(), 0);
    assert_eq!(session.current_section, NavigationSection::DialogsList);
}

#[test]
fn test_window_adaptive_breakpoints() {
    let mut adaptive = WindowAdaptive::new();

    // Mobile / compact (< 600px)
    adaptive.update_from_width(480.0);
    assert!(adaptive.is_one_column());
    assert!(!adaptive.is_normal());
    assert!(!adaptive.is_three_column());
    assert!(!adaptive.is_chat_wide());

    // Tablet / normal (600px - 1024px)
    adaptive.update_from_width(800.0);
    assert!(!adaptive.is_one_column());
    assert!(adaptive.is_normal());
    assert!(!adaptive.is_three_column());
    assert!(!adaptive.is_chat_wide());

    // Desktop widescreen (>= 1024px)
    adaptive.update_from_width(1280.0);
    assert!(!adaptive.is_one_column());
    assert!(!adaptive.is_normal());
    assert!(adaptive.is_three_column());
    assert!(adaptive.is_chat_wide());
}

#[test]
fn test_window_main_menu_and_unread_states() {
    let mut menu = WindowMainMenu::new();
    assert!(!menu.is_open);

    menu.open();
    assert!(menu.is_open);

    // Toggle night mode
    let initial_night = menu.night_mode;
    let toggled = menu.toggle_night_mode();
    assert_eq!(toggled, !initial_night);

    // Multi-account unread state calculation
    menu.accounts.push(AccountEntry {
        account_id: 2,
        name: "Secondary User".to_string(),
        phone: "+1 555 0200".to_string(),
        unread_count: 5,
        is_active: false,
    });
    menu.accounts.push(AccountEntry {
        account_id: 3,
        name: "Work Channel".to_string(),
        phone: "+1 555 0300".to_string(),
        unread_count: 12,
        is_active: false,
    });

    let unread_state = menu.others_unread_state();
    assert_eq!(unread_state.count, 17);
    assert!(!unread_state.all_muted);

    menu.close();
    assert!(!menu.is_open);
}

#[test]
fn test_theme_system_embedded_and_recipes() {
    let mut theme_mgr = WindowThemeThemes::new();
    assert!(theme_mgr.is_night_mode);
    assert_eq!(theme_mgr.active_theme, EmbeddedThemeType::Night);

    // Night mode toggling
    theme_mgr.toggle_night_mode();
    assert!(!theme_mgr.is_night_mode);
    assert_eq!(theme_mgr.active_theme, EmbeddedThemeType::DayBlue);
    assert_eq!(theme_mgr.accent_color, 0x2AABEE);

    // Builtin schemes
    let night_scheme = WindowThemesEmbeddedThemes::scheme_for(BuiltinTheme::Night);
    assert_eq!(night_scheme.background, 0x18222D);
    assert_eq!(night_scheme.bubble_out, 0x2B5278);

    let green_scheme = WindowThemesEmbeddedThemes::scheme_for(BuiltinTheme::NightGreen);
    assert_eq!(green_scheme.accent, 0x4FA882);

    // Name generator
    let name_dark = WindowThemesGenerateNameThemes::generate_name(0x5288C1, true);
    assert_eq!(name_dark, "Midnight Sapphire");

    let name_light = WindowThemesGenerateNameThemes::generate_name(0x2AABEE, false);
    assert_eq!(name_light, "Sunny Sapphire");
}

#[test]
fn test_notifications_manager_queue_and_cleanup() {
    let mut notif_mgr = NotificationsManager::new();
    assert_eq!(notif_mgr.total_badge_count, 0);

    let _id1 = notif_mgr.schedule_notification(101, "Alice", "Hey there!", true);
    let id2 = notif_mgr.schedule_notification(102, "Bob", "Check this link", true);
    assert_eq!(notif_mgr.notifications.len(), 2);
    assert_eq!(notif_mgr.total_badge_count, 2);

    notif_mgr.clear_for_peer(101);
    assert_eq!(notif_mgr.notifications.len(), 1);
    assert_eq!(notif_mgr.total_badge_count, 1);
    assert_eq!(notif_mgr.notifications[0].id, id2);

    notif_mgr.clear_all();
    assert_eq!(notif_mgr.notifications.len(), 0);
    assert_eq!(notif_mgr.total_badge_count, 0);
}
