use gpui_telegram_desktop::components::video_viewport::VideoViewport;
use gpui_telegram_desktop::platform::mac::{
    MacNotificationManager, UNNotificationParsed,
};
use gpui_telegram_desktop::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use gpui_telegram_desktop::views::app_shell::window_session_controller_link_info::{
    LinkInfoType, WindowSessionControllerLinkInfo,
};
use gpui_telegram_desktop::views::call_overlay::CallOverlay;
use gpui_telegram_webrtc::calls::calls_video_bubble::BubbleCorner;

#[test]
fn test_video_viewport_initialization_and_rgba_ingestion() {
    let pal = TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x5288C1, 14, 12);
    let mut viewport = VideoViewport::new("Alice", pal);

    assert_eq!(viewport.peer_name, "Alice");
    assert_eq!(viewport.peer_initials, "A");
    assert!(!viewport.is_remote_video_active);
    assert!(!viewport.is_local_camera_active);
    assert!(!viewport.has_active_video());

    // Ingest invalid buffer
    assert!(!viewport.ingest_raw_remote_rgba(100, 100, &[0u8; 10]));
    assert!(!viewport.has_active_video());

    // Ingest valid 32x32 RGBA8 buffer
    let raw_rgba = vec![255u8; 32 * 32 * 4];
    assert!(viewport.ingest_raw_remote_rgba(32, 32, &raw_rgba));
    assert!(viewport.is_remote_video_active);
    assert_eq!(viewport.remote_resolution, (32, 32));
    assert!(viewport.remote_frame.is_some());
    assert!(viewport.has_active_video());
}

#[test]
fn test_video_viewport_local_preview_and_bubble_corner_cycling() {
    let pal = TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x5288C1, 14, 12);
    let mut viewport = VideoViewport::new("Bob", pal);

    let local_rgba = vec![128u8; 64 * 48 * 4];
    assert!(viewport.ingest_raw_local_rgba(64, 48, &local_rgba));
    assert!(viewport.is_local_camera_active);
    assert_eq!(viewport.local_resolution, (64, 48));
    assert!(viewport.bubble.is_visible);

    // Initial corner is BottomRight
    assert_eq!(viewport.bubble.active_corner, BubbleCorner::BottomRight);

    // Cycle: BottomRight -> BottomLeft -> TopLeft -> TopRight -> BottomRight
    viewport.cycle_bubble_corner(400.0, 600.0);
    assert_eq!(viewport.bubble.active_corner, BubbleCorner::BottomLeft);

    viewport.cycle_bubble_corner(400.0, 600.0);
    assert_eq!(viewport.bubble.active_corner, BubbleCorner::TopLeft);

    viewport.cycle_bubble_corner(400.0, 600.0);
    assert_eq!(viewport.bubble.active_corner, BubbleCorner::TopRight);

    viewport.cycle_bubble_corner(400.0, 600.0);
    assert_eq!(viewport.bubble.active_corner, BubbleCorner::BottomRight);
}

#[test]
fn test_call_overlay_camera_and_screen_toggle_wires_to_viewport() {
    let mut overlay = CallOverlay::new("Charlie".to_string());
    assert!(!overlay.is_camera_on);
    assert!(!overlay.video_viewport.is_local_camera_active);
    assert!(!overlay.is_screen_sharing);
    assert!(!overlay.video_viewport.is_screen_sharing);

    overlay.toggle_camera();
    assert!(overlay.is_camera_on);
    assert!(overlay.video_viewport.is_local_camera_active);

    overlay.toggle_screen_share();
    assert!(overlay.is_screen_sharing);
    assert!(overlay.video_viewport.is_screen_sharing);
    assert!(overlay.video_viewport.has_active_video());
}

#[test]
fn test_mac_notification_manager_applescript_generation_and_mock_mode() {
    let mut mgr = MacNotificationManager::new(42).with_mock_mode(true);

    let esc = MacNotificationManager::escape_applescript_string("Hello \"World\"\nTest \\ done");
    assert_eq!(esc, "Hello \\\"World\\\" Test \\\\ done");

    let script = MacNotificationManager::build_applescript_command(
        "Telegram",
        "Direct Message",
        "Hey! Check this out",
        true,
    );
    assert!(script.contains("display notification \"Hey! Check this out\" with title \"Telegram\""));
    assert!(script.contains("subtitle \"Direct Message\""));
    assert!(script.contains("sound name \"default\""));

    let parsed = UNNotificationParsed::new(1, 98765, 4321);
    let id = mgr
        .show_system_notification("Alice", "Chat", "Test message", true, parsed)
        .expect("Notification dispatch should succeed");

    assert_eq!(id, "tg_notif_98765_4321");
    assert_eq!(mgr.dispatched_history.len(), 1);
    assert_eq!(mgr.dispatched_history[0].deep_link, "tg://chat?id=98765");

    let clicked_link = mgr.handle_notification_click(&id);
    assert_eq!(clicked_link, Some("tg://chat?id=98765".to_string()));
    assert!(mgr.in_flight_requests.is_empty());
}

#[test]
fn test_deep_link_chat_and_openmessage_parsing() {
    // 1. tg://chat?id=12345
    let res = WindowSessionControllerLinkInfo::parse_deep_link("tg://chat?id=12345");
    match res {
        LinkInfoType::Peer(peer) => {
            assert_eq!(peer.username_or_id, Some("12345".to_string()));
            assert_eq!(peer.channel_id, Some(12345));
        }
        other => panic!("Expected LinkInfoType::Peer, got {:?}", other),
    }

    // 2. tg://openmessage?chat_id=9876&id=54321
    let res = WindowSessionControllerLinkInfo::parse_deep_link("tg://openmessage?chat_id=9876&id=54321");
    match res {
        LinkInfoType::Peer(peer) => {
            assert_eq!(peer.username_or_id, Some("9876".to_string()));
            assert_eq!(peer.channel_id, Some(9876));
            assert_eq!(peer.message_id, Some(54321));
        }
        other => panic!("Expected LinkInfoType::Peer, got {:?}", other),
    }

    // 3. Empty query values must resolve to LinkInfoType::Unknown
    assert_eq!(
        WindowSessionControllerLinkInfo::parse_deep_link("tg://chat?id="),
        LinkInfoType::Unknown
    );
    assert_eq!(
        WindowSessionControllerLinkInfo::parse_deep_link("tg://openmessage?chat_id=&id=5"),
        LinkInfoType::Unknown
    );
    assert_eq!(
        WindowSessionControllerLinkInfo::parse_deep_link("tg://resolve?domain="),
        LinkInfoType::Unknown
    );
}

#[test]
fn test_video_viewport_checked_dimensions_and_overlay_sync() {
    let pal = TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x5288C1, 14, 12);
    let mut viewport = VideoViewport::new("OverflowTest", pal.clone());

    // Overflowing dimensions must safely return false without panicking
    assert!(!viewport.ingest_raw_remote_rgba(u32::MAX, 1, &[0u8; 4]));
    assert!(!viewport.ingest_raw_remote_rgba(65536, 65536, &[]));
    assert!(!viewport.ingest_raw_local_rgba(u32::MAX, 2, &[0u8; 8]));

    // CallOverlay::set_palette syncs to video_viewport.palette
    let mut overlay = CallOverlay::new("SyncTest".to_string());
    let day_pal = TdesktopThemePalette::build(TdesktopThemeMode::Day, 0x3390EC, 14, 12);
    overlay.set_palette(day_pal.clone());
    assert_eq!(overlay.video_viewport.palette.wallpaper_top, day_pal.wallpaper_top);
}

#[test]
fn test_escape_applescript_string_strips_nul_and_control_chars_preserving_tab() {
    let input = "Hello\0World\x07\tTab\nLine\x1fEnd";
    let escaped = MacNotificationManager::escape_applescript_string(input);
    assert!(!escaped.contains('\0'), "NUL byte must be stripped");
    assert!(!escaped.contains('\x07'), "BEL control char must be stripped");
    assert!(!escaped.contains('\x1f'), "Control char < 0x20 must be stripped");
    assert!(escaped.contains('\t'), "Tab must be preserved");
    assert_eq!(escaped, "HelloWorld\tTab LineEnd");
}

#[test]
fn test_notification_preference_and_window_focus_gating() {
    use gpui_telegram_desktop::app::preferences::NotificationPreferences;
    use gpui_telegram_desktop::views::app_shell::shell_pts_updates::should_dispatch_message_notification;

    let mut prefs = NotificationPreferences::default();
    // Unmuted private background chat notifies when enabled
    assert!(should_dispatch_message_notification(false, false, true, false, false, false, &prefs));
    // Active chat suppressed when window focused, allowed when window unfocused
    assert!(!should_dispatch_message_notification(false, true, true, false, false, false, &prefs));
    assert!(should_dispatch_message_notification(false, true, false, false, false, false, &prefs));
    // Outgoing & muted suppressed
    assert!(!should_dispatch_message_notification(true, false, false, false, false, false, &prefs));
    assert!(!should_dispatch_message_notification(false, false, false, true, false, false, &prefs));
    // Channels default to false -> suppressed until enabled
    assert!(!should_dispatch_message_notification(false, false, false, false, true, false, &prefs));
    prefs.channels = true;
    assert!(should_dispatch_message_notification(false, false, false, false, true, false, &prefs));
    // Global disabled suppresses all
    prefs.enabled = false;
    assert!(!should_dispatch_message_notification(false, false, false, false, false, false, &prefs));
}
