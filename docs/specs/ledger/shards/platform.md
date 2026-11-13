# Subsystem Shard: `platform`

**Total Source Files**: 132  
**Total Source Lines**: 24,908  
**Primary Feature IDs**: CAP-06, TCG-N01, TCG-N10, TCG-N17  
**JSON Shard Path**: `docs/specs/ledger/shards/platform.json`  

## Component Inventory & Parity Mapping

| Source Path | Feature ID | Target Symbol | Target Rust / GPUI Path | Lines | Test ID |
|---|---|---|---|---|---|
| `Telegram/SourceFiles/platform/linux/current_geo_location_linux.cpp` | `TCG-N41` | `CurrentGeoLocationLinuxLinux` | `crates/gpui_telegram_desktop/src/platform/linux/current_geo_location_linux.rs` | 105 | `tests/test_tcg_n41_current_geo_location_linux.rs` |
| `Telegram/SourceFiles/platform/linux/current_geo_location_linux.h` | `TCG-N41` | `CurrentGeoLocationLinuxLinux` | `crates/gpui_telegram_desktop/src/platform/linux/current_geo_location_linux.rs` | 10 | `tests/test_tcg_n41_current_geo_location_linux.rs` |
| `Telegram/SourceFiles/platform/linux/file_utilities_linux.cpp` | `TCG-N46` | `FileUtilitiesLinuxLinux` | `crates/gpui_telegram_desktop/src/platform/linux/file_utilities_linux.rs` | 129 | `tests/test_tcg_n46_file_utilities_linux.rs` |
| `Telegram/SourceFiles/platform/linux/file_utilities_linux.h` | `TCG-N46` | `FileUtilitiesLinuxLinux` | `crates/gpui_telegram_desktop/src/platform/linux/file_utilities_linux.rs` | 69 | `tests/test_tcg_n46_file_utilities_linux.rs` |
| `Telegram/SourceFiles/platform/linux/integration_linux.cpp` | `TCG-N46` | `IntegrationLinuxLinux` | `crates/gpui_telegram_desktop/src/platform/linux/integration_linux.rs` | 298 | `tests/test_tcg_n46_integration_linux.rs` |
| `Telegram/SourceFiles/platform/linux/integration_linux.h` | `TCG-N46` | `IntegrationLinuxLinux` | `crates/gpui_telegram_desktop/src/platform/linux/integration_linux.rs` | 16 | `tests/test_tcg_n46_integration_linux.rs` |
| `Telegram/SourceFiles/platform/linux/launcher_linux.cpp` | `TCG-N46` | `LauncherLinuxLinux` | `crates/gpui_telegram_desktop/src/platform/linux/launcher_linux.rs` | 328 | `tests/test_tcg_n46_launcher_linux.rs` |
| `Telegram/SourceFiles/platform/linux/launcher_linux.h` | `TCG-N46` | `LauncherLinuxLinux` | `crates/gpui_telegram_desktop/src/platform/linux/launcher_linux.rs` | 30 | `tests/test_tcg_n46_launcher_linux.rs` |
| `Telegram/SourceFiles/platform/linux/main_window_linux.cpp` | `TCG-N46` | `MainWindowLinuxLinux` | `crates/gpui_telegram_desktop/src/platform/linux/main_window_linux.rs` | 569 | `tests/test_tcg_n46_main_window_linux.rs` |
| `Telegram/SourceFiles/platform/linux/main_window_linux.h` | `TCG-N46` | `MainWindowLinuxLinux` | `crates/gpui_telegram_desktop/src/platform/linux/main_window_linux.rs` | 71 | `tests/test_tcg_n46_main_window_linux.rs` |
| `Telegram/SourceFiles/platform/linux/notifications_manager_linux.cpp` | `TCG-N46` | `NotificationsManagerLinuxLinux` | `crates/gpui_telegram_desktop/src/platform/linux/notifications_manager_linux.rs` | 978 | `tests/test_tcg_n46_notifications_manager_linux.rs` |
| `Telegram/SourceFiles/platform/linux/notifications_manager_linux.h` | `TCG-N46` | `NotificationsManagerLinuxLinux` | `crates/gpui_telegram_desktop/src/platform/linux/notifications_manager_linux.rs` | 42 | `tests/test_tcg_n46_notifications_manager_linux.rs` |
| `Telegram/SourceFiles/platform/linux/org.freedesktop.Notifications.xml` | `TCG-N46` | `OrgFreedesktopNotificationsLinuxXmlAsset` | `crates/gpui_telegram_desktop/assets/org.freedesktop.Notifications_xml.rs` | 59 | `tests/test_tcg_n46_org_freedesktop_Notifications.rs` |
| `Telegram/SourceFiles/platform/linux/org.freedesktop.login1.Manager.xml` | `TCG-N46` | `OrgFreedesktopLogin1ManagerLinuxXmlAsset` | `crates/gpui_telegram_desktop/assets/org.freedesktop.login1.Manager_xml.rs` | 11 | `tests/test_tcg_n46_org_freedesktop_login1_Manager.rs` |
| `Telegram/SourceFiles/platform/linux/org.freedesktop.portal.Flatpak.xml` | `TCG-N46` | `OrgFreedesktopPortalFlatpakLinuxXmlAsset` | `crates/gpui_telegram_desktop/assets/org.freedesktop.portal.Flatpak_xml.rs` | 583 | `tests/test_tcg_n46_org_freedesktop_portal_Flatpak.rs` |
| `Telegram/SourceFiles/platform/linux/overlay_widget_linux.h` | `TCG-N46` | `OverlayWidgetLinuxLinux` | `crates/gpui_telegram_desktop/src/platform/linux/overlay_widget_linux.rs` | 20 | `tests/test_tcg_n46_overlay_widget_linux.rs` |
| `Telegram/SourceFiles/platform/linux/specific_linux.cpp` | `TCG-N46` | `SpecificLinuxLinux` | `crates/gpui_telegram_desktop/src/platform/linux/specific_linux.rs` | 951 | `tests/test_tcg_n46_specific_linux.rs` |
| `Telegram/SourceFiles/platform/linux/specific_linux.h` | `TCG-N46` | `SpecificLinuxLinux` | `crates/gpui_telegram_desktop/src/platform/linux/specific_linux.rs` | 82 | `tests/test_tcg_n46_specific_linux.rs` |
| `Telegram/SourceFiles/platform/linux/text_recognition_linux.h` | `TCG-N46` | `TextRecognitionLinuxLinux` | `crates/gpui_telegram_desktop/src/platform/linux/text_recognition_linux.rs` | 24 | `tests/test_tcg_n46_text_recognition_linux.rs` |
| `Telegram/SourceFiles/platform/linux/translate_provider_linux.cpp` | `TCG-N46` | `TranslateProviderLinuxLinux` | `crates/gpui_telegram_desktop/src/platform/linux/translate_provider_linux.rs` | 90 | `tests/test_tcg_n46_translate_provider_linux.rs` |
| `Telegram/SourceFiles/platform/linux/translate_provider_linux.h` | `TCG-N46` | `TranslateProviderLinuxLinux` | `crates/gpui_telegram_desktop/src/platform/linux/translate_provider_linux.rs` | 10 | `tests/test_tcg_n46_translate_provider_linux.rs` |
| `Telegram/SourceFiles/platform/linux/tray_linux.cpp` | `TCG-N46` | `TrayLinuxLinux` | `crates/gpui_telegram_desktop/src/platform/linux/tray_linux.rs` | 424 | `tests/test_tcg_n46_tray_linux.rs` |
| `Telegram/SourceFiles/platform/linux/tray_linux.h` | `TCG-N46` | `TrayLinuxLinux` | `crates/gpui_telegram_desktop/src/platform/linux/tray_linux.rs` | 75 | `tests/test_tcg_n46_tray_linux.rs` |
| `Telegram/SourceFiles/platform/linux/update_install_linux.cpp` | `TCG-N46` | `UpdateInstallLinuxLinux` | `crates/gpui_telegram_desktop/src/platform/linux/update_install_linux.rs` | 1299 | `tests/test_tcg_n46_update_install_linux.rs` |
| `Telegram/SourceFiles/platform/linux/update_install_linux.h` | `TCG-N46` | `UpdateInstallLinuxLinux` | `crates/gpui_telegram_desktop/src/platform/linux/update_install_linux.rs` | 49 | `tests/test_tcg_n46_update_install_linux.rs` |
| `Telegram/SourceFiles/platform/linux/webauthn_linux.cpp` | `TCG-N01` | `WebauthnLinuxLinux` | `crates/gpui_telegram_desktop/src/platform/linux/webauthn_linux.rs` | 46 | `tests/test_tcg_n01_webauthn_linux.rs` |
| `Telegram/SourceFiles/platform/mac/current_geo_location_mac.h` | `TCG-N41` | `CurrentGeoLocationMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/current_geo_location_mac.rs` | 10 | `tests/test_tcg_n41_current_geo_location_mac.rs` |
| `Telegram/SourceFiles/platform/mac/current_geo_location_mac.mm` | `TCG-N41` | `CurrentGeoLocationMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/current_geo_location_mac.rs` | 154 | `tests/test_tcg_n41_current_geo_location_mac.rs` |
| `Telegram/SourceFiles/platform/mac/file_bookmark_mac.h` | `TCG-N46` | `FileBookmarkMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/file_bookmark_mac.rs` | 33 | `tests/test_tcg_n46_file_bookmark_mac.rs` |
| `Telegram/SourceFiles/platform/mac/file_bookmark_mac.mm` | `TCG-N46` | `FileBookmarkMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/file_bookmark_mac.rs` | 133 | `tests/test_tcg_n46_file_bookmark_mac.rs` |
| `Telegram/SourceFiles/platform/mac/file_utilities_mac.h` | `TCG-N46` | `FileUtilitiesMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/file_utilities_mac.rs` | 57 | `tests/test_tcg_n46_file_utilities_mac.rs` |
| `Telegram/SourceFiles/platform/mac/file_utilities_mac.mm` | `TCG-N46` | `FileUtilitiesMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/file_utilities_mac.rs` | 572 | `tests/test_tcg_n46_file_utilities_mac.rs` |
| `Telegram/SourceFiles/platform/mac/global_menu_mac.h` | `TCG-N46` | `GlobalMenuMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/global_menu_mac.rs` | 22 | `tests/test_tcg_n46_global_menu_mac.rs` |
| `Telegram/SourceFiles/platform/mac/global_menu_mac.mm` | `TCG-N46` | `GlobalMenuMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/global_menu_mac.rs` | 740 | `tests/test_tcg_n46_global_menu_mac.rs` |
| `Telegram/SourceFiles/platform/mac/integration_mac.h` | `TCG-N46` | `IntegrationMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/integration_mac.rs` | 16 | `tests/test_tcg_n46_integration_mac.rs` |
| `Telegram/SourceFiles/platform/mac/integration_mac.mm` | `TCG-N46` | `IntegrationMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/integration_mac.rs` | 24 | `tests/test_tcg_n46_integration_mac.rs` |
| `Telegram/SourceFiles/platform/mac/launcher_mac.h` | `TCG-N46` | `LauncherMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/launcher_mac.rs` | 25 | `tests/test_tcg_n46_launcher_mac.rs` |
| `Telegram/SourceFiles/platform/mac/launcher_mac.mm` | `TCG-N46` | `LauncherMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/launcher_mac.rs` | 90 | `tests/test_tcg_n46_launcher_mac.rs` |
| `Telegram/SourceFiles/platform/mac/mac_iconv_helper.c` | `TCG-N46` | `MacIconvHelperMac` | `crates/gpui_telegram_desktop/src/platform/mac/mac_iconv_helper.rs` | 32 | `tests/test_tcg_n46_mac_iconv_helper.rs` |
| `Telegram/SourceFiles/platform/mac/main_window_mac.h` | `TCG-N46` | `MainWindowMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/main_window_mac.rs` | 73 | `tests/test_tcg_n46_main_window_mac.rs` |
| `Telegram/SourceFiles/platform/mac/main_window_mac.mm` | `TCG-N46` | `MainWindowMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/main_window_mac.rs` | 338 | `tests/test_tcg_n46_main_window_mac.rs` |
| `Telegram/SourceFiles/platform/mac/native_event_mac.h` | `TCG-N46` | `NativeEventMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/native_event_mac.rs` | 14 | `tests/test_tcg_n46_native_event_mac.rs` |
| `Telegram/SourceFiles/platform/mac/native_event_mac.mm` | `TCG-N46` | `NativeEventMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/native_event_mac.rs` | 35 | `tests/test_tcg_n46_native_event_mac.rs` |
| `Telegram/SourceFiles/platform/mac/notifications_manager_mac.h` | `TCG-N46` | `NotificationsManagerMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/notifications_manager_mac.rs` | 43 | `tests/test_tcg_n46_notifications_manager_mac.rs` |
| `Telegram/SourceFiles/platform/mac/notifications_manager_mac.mm` | `TCG-N46` | `NotificationsManagerMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/notifications_manager_mac.rs` | 741 | `tests/test_tcg_n46_notifications_manager_mac.rs` |
| `Telegram/SourceFiles/platform/mac/notifications_manager_mac_un.h` | `TCG-N46` | `NotificationsManagerMacUnMac` | `crates/gpui_telegram_desktop/src/platform/mac/notifications_manager_mac_un.rs` | 47 | `tests/test_tcg_n46_notifications_manager_mac_un.rs` |
| `Telegram/SourceFiles/platform/mac/notifications_manager_mac_un.mm` | `TCG-N46` | `NotificationsManagerMacUnMac` | `crates/gpui_telegram_desktop/src/platform/mac/notifications_manager_mac_un.rs` | 667 | `tests/test_tcg_n46_notifications_manager_mac_un.rs` |
| `Telegram/SourceFiles/platform/mac/overlay_widget_mac.h` | `TCG-N46` | `OverlayWidgetMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/overlay_widget_mac.rs` | 60 | `tests/test_tcg_n46_overlay_widget_mac.rs` |
| `Telegram/SourceFiles/platform/mac/overlay_widget_mac.mm` | `TCG-N46` | `OverlayWidgetMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/overlay_widget_mac.rs` | 349 | `tests/test_tcg_n46_overlay_widget_mac.rs` |
| `Telegram/SourceFiles/platform/mac/specific_mac.h` | `TCG-N46` | `SpecificMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/specific_mac.rs` | 76 | `tests/test_tcg_n46_specific_mac.rs` |
| `Telegram/SourceFiles/platform/mac/specific_mac.mm` | `TCG-N46` | `SpecificMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/specific_mac.rs` | 565 | `tests/test_tcg_n46_specific_mac.rs` |
| `Telegram/SourceFiles/platform/mac/specific_mac_p.h` | `TCG-N46` | `SpecificMacPMac` | `crates/gpui_telegram_desktop/src/platform/mac/specific_mac_p.rs` | 28 | `tests/test_tcg_n46_specific_mac_p.rs` |
| `Telegram/SourceFiles/platform/mac/specific_mac_p.mm` | `TCG-N46` | `SpecificMacPMac` | `crates/gpui_telegram_desktop/src/platform/mac/specific_mac_p.rs` | 360 | `tests/test_tcg_n46_specific_mac_p.rs` |
| `Telegram/SourceFiles/platform/mac/text_recognition_mac.h` | `TCG-N46` | `TextRecognitionMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/text_recognition_mac.rs` | 8 | `tests/test_tcg_n46_text_recognition_mac.rs` |
| `Telegram/SourceFiles/platform/mac/text_recognition_mac.mm` | `TCG-N46` | `TextRecognitionMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/text_recognition_mac.rs` | 111 | `tests/test_tcg_n46_text_recognition_mac.rs` |
| `Telegram/SourceFiles/platform/mac/touchbar/items/mac_formatter_item.h` | `TCG-N46` | `MacFormatterItemMacTouchbarItems` | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/items/mac_formatter_item.rs` | 16 | `tests/test_tcg_n46_mac_formatter_item.rs` |
| `Telegram/SourceFiles/platform/mac/touchbar/items/mac_formatter_item.mm` | `TCG-N46` | `MacFormatterItemMacTouchbarItems` | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/items/mac_formatter_item.rs` | 144 | `tests/test_tcg_n46_mac_formatter_item.rs` |
| `Telegram/SourceFiles/platform/mac/touchbar/items/mac_pinned_chats_item.h` | `TCG-N17` | `MacPinnedChatsItemMacTouchbarItems` | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/items/mac_pinned_chats_item.rs` | 20 | `tests/test_tcg_n17_mac_pinned_chats_item.rs` |
| `Telegram/SourceFiles/platform/mac/touchbar/items/mac_pinned_chats_item.mm` | `TCG-N17` | `MacPinnedChatsItemMacTouchbarItems` | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/items/mac_pinned_chats_item.rs` | 873 | `tests/test_tcg_n17_mac_pinned_chats_item.rs` |
| `Telegram/SourceFiles/platform/mac/touchbar/items/mac_scrubber_item.h` | `TCG-N46` | `MacScrubberItemMacTouchbarItems` | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/items/mac_scrubber_item.rs` | 21 | `tests/test_tcg_n46_mac_scrubber_item.rs` |
| `Telegram/SourceFiles/platform/mac/touchbar/items/mac_scrubber_item.mm` | `TCG-N46` | `MacScrubberItemMacTouchbarItems` | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/items/mac_scrubber_item.rs` | 717 | `tests/test_tcg_n46_mac_scrubber_item.rs` |
| `Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_audio.h` | `TCG-N46` | `MacTouchbarAudioMacTouchbar` | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/mac_touchbar_audio.rs` | 15 | `tests/test_tcg_n46_mac_touchbar_audio.rs` |
| `Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_audio.mm` | `TCG-N46` | `MacTouchbarAudioMacTouchbar` | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/mac_touchbar_audio.rs` | 176 | `tests/test_tcg_n46_mac_touchbar_audio.rs` |
| `Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_common.h` | `TCG-N46` | `MacTouchbarCommonMacTouchbar` | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/mac_touchbar_common.rs` | 32 | `tests/test_tcg_n46_mac_touchbar_common.rs` |
| `Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_common.mm` | `TCG-N46` | `MacTouchbarCommonMacTouchbar` | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/mac_touchbar_common.rs` | 29 | `tests/test_tcg_n46_mac_touchbar_common.rs` |
| `Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_controls.h` | `TCG-N46` | `MacTouchbarControlsMacTouchbar` | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/mac_touchbar_controls.rs` | 64 | `tests/test_tcg_n46_mac_touchbar_controls.rs` |
| `Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_controls.mm` | `TCG-N46` | `MacTouchbarControlsMacTouchbar` | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/mac_touchbar_controls.rs` | 278 | `tests/test_tcg_n46_mac_touchbar_controls.rs` |
| `Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_main.h` | `TCG-N46` | `MacTouchbarMainMacTouchbar` | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/mac_touchbar_main.rs` | 28 | `tests/test_tcg_n46_mac_touchbar_main.rs` |
| `Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_main.mm` | `TCG-N46` | `MacTouchbarMainMacTouchbar` | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/mac_touchbar_main.rs` | 53 | `tests/test_tcg_n46_mac_touchbar_main.rs` |
| `Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_manager.h` | `TCG-N46` | `MacTouchbarManagerMacTouchbar` | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/mac_touchbar_manager.rs` | 30 | `tests/test_tcg_n46_mac_touchbar_manager.rs` |
| `Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_manager.mm` | `TCG-N46` | `MacTouchbarManagerMacTouchbar` | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/mac_touchbar_manager.rs` | 183 | `tests/test_tcg_n46_mac_touchbar_manager.rs` |
| `Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_media_view.h` | `TCG-N10` | `MacTouchbarMediaViewMacTouchbar` | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/mac_touchbar_media_view.rs` | 23 | `tests/test_tcg_n10_mac_touchbar_media_view.rs` |
| `Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_media_view.mm` | `TCG-N10` | `MacTouchbarMediaViewMacTouchbar` | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/mac_touchbar_media_view.rs` | 224 | `tests/test_tcg_n10_mac_touchbar_media_view.rs` |
| `Telegram/SourceFiles/platform/mac/translate_provider_mac.h` | `TCG-N46` | `TranslateProviderMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/translate_provider_mac.rs` | 18 | `tests/test_tcg_n46_translate_provider_mac.rs` |
| `Telegram/SourceFiles/platform/mac/translate_provider_mac.mm` | `TCG-N46` | `TranslateProviderMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/translate_provider_mac.rs` | 118 | `tests/test_tcg_n46_translate_provider_mac.rs` |
| `Telegram/SourceFiles/platform/mac/tray_mac.h` | `TCG-N46` | `TrayMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/tray_mac.rs` | 78 | `tests/test_tcg_n46_tray_mac.rs` |
| `Telegram/SourceFiles/platform/mac/tray_mac.mm` | `TCG-N46` | `TrayMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/tray_mac.rs` | 498 | `tests/test_tcg_n46_tray_mac.rs` |
| `Telegram/SourceFiles/platform/mac/wallet_protection_mac.h` | `CAP-06` | `WalletProtectionMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/wallet_protection_mac.rs` | 16 | `tests/test_cap_06_wallet_protection_mac.rs` |
| `Telegram/SourceFiles/platform/mac/wallet_protection_mac.mm` | `CAP-06` | `WalletProtectionMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/wallet_protection_mac.rs` | 523 | `tests/test_cap_06_wallet_protection_mac.rs` |
| `Telegram/SourceFiles/platform/mac/webauthn_mac.mm` | `TCG-N01` | `WebauthnMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/webauthn_mac.rs` | 379 | `tests/test_tcg_n01_webauthn_mac.rs` |
| `Telegram/SourceFiles/platform/mac/window_title_mac.mm` | `TCG-N46` | `WindowTitleMacMac` | `crates/gpui_telegram_desktop/src/platform/mac/window_title_mac.rs` | 187 | `tests/test_tcg_n46_window_title_mac.rs` |
| `Telegram/SourceFiles/platform/platform_current_geo_location.h` | `TCG-N41` | `PlatformCurrentGeoLocation` | `crates/gpui_telegram_desktop/src/platform/platform_current_geo_location.rs` | 23 | `tests/test_tcg_n41_platform_current_geo_location.rs` |
| `Telegram/SourceFiles/platform/platform_file_bookmark.h` | `TCG-N46` | `PlatformFileBookmark` | `crates/gpui_telegram_desktop/src/platform/platform_file_bookmark.rs` | 43 | `tests/test_tcg_n46_platform_file_bookmark.rs` |
| `Telegram/SourceFiles/platform/platform_file_utilities.cpp` | `TCG-N46` | `PlatformFileUtilities` | `crates/gpui_telegram_desktop/src/platform/platform_file_utilities.rs` | 49 | `tests/test_tcg_n46_platform_file_utilities.rs` |
| `Telegram/SourceFiles/platform/platform_file_utilities.h` | `TCG-N46` | `PlatformFileUtilities` | `crates/gpui_telegram_desktop/src/platform/platform_file_utilities.rs` | 62 | `tests/test_tcg_n46_platform_file_utilities.rs` |
| `Telegram/SourceFiles/platform/platform_integration.cpp` | `TCG-N46` | `PlatformIntegration` | `crates/gpui_telegram_desktop/src/platform/platform_integration.rs` | 43 | `tests/test_tcg_n46_platform_integration.rs` |
| `Telegram/SourceFiles/platform/platform_integration.h` | `TCG-N46` | `PlatformIntegration` | `crates/gpui_telegram_desktop/src/platform/platform_integration.rs` | 23 | `tests/test_tcg_n46_platform_integration.rs` |
| `Telegram/SourceFiles/platform/platform_launcher.h` | `TCG-N46` | `PlatformLauncher` | `crates/gpui_telegram_desktop/src/platform/platform_launcher.rs` | 30 | `tests/test_tcg_n46_platform_launcher.rs` |
| `Telegram/SourceFiles/platform/platform_main_window.h` | `TCG-N46` | `PlatformMainWindow` | `crates/gpui_telegram_desktop/src/platform/platform_main_window.rs` | 26 | `tests/test_tcg_n46_platform_main_window.rs` |
| `Telegram/SourceFiles/platform/platform_notifications_manager.h` | `TCG-N46` | `PlatformNotificationsManager` | `crates/gpui_telegram_desktop/src/platform/platform_notifications_manager.rs` | 37 | `tests/test_tcg_n46_platform_notifications_manager.rs` |
| `Telegram/SourceFiles/platform/platform_overlay_widget.cpp` | `TCG-N46` | `PlatformOverlayWidget` | `crates/gpui_telegram_desktop/src/platform/platform_overlay_widget.rs` | 259 | `tests/test_tcg_n46_platform_overlay_widget.rs` |
| `Telegram/SourceFiles/platform/platform_overlay_widget.h` | `TCG-N46` | `PlatformOverlayWidget` | `crates/gpui_telegram_desktop/src/platform/platform_overlay_widget.rs` | 104 | `tests/test_tcg_n46_platform_overlay_widget.rs` |
| `Telegram/SourceFiles/platform/platform_specific.h` | `TCG-N46` | `PlatformSpecific` | `crates/gpui_telegram_desktop/src/platform/platform_specific.rs` | 94 | `tests/test_tcg_n46_platform_specific.rs` |
| `Telegram/SourceFiles/platform/platform_text_recognition.h` | `TCG-N46` | `PlatformTextRecognition` | `crates/gpui_telegram_desktop/src/platform/platform_text_recognition.rs` | 42 | `tests/test_tcg_n46_platform_text_recognition.rs` |
| `Telegram/SourceFiles/platform/platform_translate_provider.h` | `TCG-N46` | `PlatformTranslateProvider` | `crates/gpui_telegram_desktop/src/platform/platform_translate_provider.rs` | 25 | `tests/test_tcg_n46_platform_translate_provider.rs` |
| `Telegram/SourceFiles/platform/platform_tray.h` | `TCG-N46` | `PlatformTray` | `crates/gpui_telegram_desktop/src/platform/platform_tray.rs` | 26 | `tests/test_tcg_n46_platform_tray.rs` |
| `Telegram/SourceFiles/platform/platform_webauthn.h` | `TCG-N01` | `PlatformWebauthn` | `crates/gpui_telegram_desktop/src/platform/platform_webauthn.rs` | 50 | `tests/test_tcg_n01_platform_webauthn.rs` |
| `Telegram/SourceFiles/platform/platform_window_title.h` | `TCG-N46` | `PlatformWindowTitle` | `crates/gpui_telegram_desktop/src/platform/platform_window_title.rs` | 44 | `tests/test_tcg_n46_platform_window_title.rs` |
| `Telegram/SourceFiles/platform/win/current_geo_location_win.cpp` | `TCG-N41` | `CurrentGeoLocationWinWin` | `crates/gpui_telegram_desktop/src/platform/win/current_geo_location_win.rs` | 69 | `tests/test_tcg_n41_current_geo_location_win.rs` |
| `Telegram/SourceFiles/platform/win/current_geo_location_win.h` | `TCG-N41` | `CurrentGeoLocationWinWin` | `crates/gpui_telegram_desktop/src/platform/win/current_geo_location_win.rs` | 10 | `tests/test_tcg_n41_current_geo_location_win.rs` |
| `Telegram/SourceFiles/platform/win/file_utilities_win.cpp` | `TCG-N46` | `FileUtilitiesWinWin` | `crates/gpui_telegram_desktop/src/platform/win/file_utilities_win.rs` | 477 | `tests/test_tcg_n46_file_utilities_win.rs` |
| `Telegram/SourceFiles/platform/win/file_utilities_win.h` | `TCG-N46` | `FileUtilitiesWinWin` | `crates/gpui_telegram_desktop/src/platform/win/file_utilities_win.rs` | 27 | `tests/test_tcg_n46_file_utilities_win.rs` |
| `Telegram/SourceFiles/platform/win/integration_win.cpp` | `TCG-N46` | `IntegrationWinWin` | `crates/gpui_telegram_desktop/src/platform/win/integration_win.rs` | 261 | `tests/test_tcg_n46_integration_win.rs` |
| `Telegram/SourceFiles/platform/win/integration_win.h` | `TCG-N46` | `IntegrationWinWin` | `crates/gpui_telegram_desktop/src/platform/win/integration_win.rs` | 69 | `tests/test_tcg_n46_integration_win.rs` |
| `Telegram/SourceFiles/platform/win/launcher_win.cpp` | `TCG-N46` | `LauncherWinWin` | `crates/gpui_telegram_desktop/src/platform/win/launcher_win.rs` | 253 | `tests/test_tcg_n46_launcher_win.rs` |
| `Telegram/SourceFiles/platform/win/launcher_win.h` | `TCG-N46` | `LauncherWinWin` | `crates/gpui_telegram_desktop/src/platform/win/launcher_win.rs` | 34 | `tests/test_tcg_n46_launcher_win.rs` |
| `Telegram/SourceFiles/platform/win/main_window_win.cpp` | `TCG-N46` | `MainWindowWinWin` | `crates/gpui_telegram_desktop/src/platform/win/main_window_win.rs` | 848 | `tests/test_tcg_n46_main_window_win.rs` |
| `Telegram/SourceFiles/platform/win/main_window_win.h` | `TCG-N46` | `MainWindowWinWin` | `crates/gpui_telegram_desktop/src/platform/win/main_window_win.rs` | 113 | `tests/test_tcg_n46_main_window_win.rs` |
| `Telegram/SourceFiles/platform/win/notifications_manager_win.cpp` | `TCG-N46` | `NotificationsManagerWinWin` | `crates/gpui_telegram_desktop/src/platform/win/notifications_manager_win.rs` | 1039 | `tests/test_tcg_n46_notifications_manager_win.rs` |
| `Telegram/SourceFiles/platform/win/notifications_manager_win.h` | `TCG-N46` | `NotificationsManagerWinWin` | `crates/gpui_telegram_desktop/src/platform/win/notifications_manager_win.rs` | 52 | `tests/test_tcg_n46_notifications_manager_win.rs` |
| `Telegram/SourceFiles/platform/win/overlay_widget_win.h` | `TCG-N46` | `OverlayWidgetWinWin` | `crates/gpui_telegram_desktop/src/platform/win/overlay_widget_win.rs` | 20 | `tests/test_tcg_n46_overlay_widget_win.rs` |
| `Telegram/SourceFiles/platform/win/specific_win.cpp` | `TCG-N46` | `SpecificWinWin` | `crates/gpui_telegram_desktop/src/platform/win/specific_win.rs` | 818 | `tests/test_tcg_n46_specific_win.rs` |
| `Telegram/SourceFiles/platform/win/specific_win.h` | `TCG-N46` | `SpecificWinWin` | `crates/gpui_telegram_desktop/src/platform/win/specific_win.rs` | 81 | `tests/test_tcg_n46_specific_win.rs` |
| `Telegram/SourceFiles/platform/win/text_recognition_win.h` | `TCG-N46` | `TextRecognitionWinWin` | `crates/gpui_telegram_desktop/src/platform/win/text_recognition_win.rs` | 24 | `tests/test_tcg_n46_text_recognition_win.rs` |
| `Telegram/SourceFiles/platform/win/translate_provider_win.h` | `TCG-N46` | `TranslateProviderWinWin` | `crates/gpui_telegram_desktop/src/platform/win/translate_provider_win.rs` | 22 | `tests/test_tcg_n46_translate_provider_win.rs` |
| `Telegram/SourceFiles/platform/win/tray_win.cpp` | `TCG-N46` | `TrayWinWin` | `crates/gpui_telegram_desktop/src/platform/win/tray_win.rs` | 450 | `tests/test_tcg_n46_tray_win.rs` |
| `Telegram/SourceFiles/platform/win/tray_win.h` | `TCG-N46` | `TrayWinWin` | `crates/gpui_telegram_desktop/src/platform/win/tray_win.rs` | 81 | `tests/test_tcg_n46_tray_win.rs` |
| `Telegram/SourceFiles/platform/win/wallet_protection_win.cpp` | `CAP-06` | `WalletProtectionWinWin` | `crates/gpui_telegram_desktop/src/platform/win/wallet_protection_win.rs` | 712 | `tests/test_cap_06_wallet_protection_win.rs` |
| `Telegram/SourceFiles/platform/win/wallet_protection_win.h` | `CAP-06` | `WalletProtectionWinWin` | `crates/gpui_telegram_desktop/src/platform/win/wallet_protection_win.rs` | 19 | `tests/test_cap_06_wallet_protection_win.rs` |
| `Telegram/SourceFiles/platform/win/webauthn_win.cpp` | `TCG-N01` | `WebauthnWinWin` | `crates/gpui_telegram_desktop/src/platform/win/webauthn_win.rs` | 381 | `tests/test_tcg_n01_webauthn_win.rs` |
| `Telegram/SourceFiles/platform/win/windows_app_user_model_id.cpp` | `TCG-N46` | `WindowsAppUserModelIdWin` | `crates/gpui_telegram_desktop/src/platform/win/windows_app_user_model_id.rs` | 507 | `tests/test_tcg_n46_windows_app_user_model_id.rs` |
| `Telegram/SourceFiles/platform/win/windows_app_user_model_id.h` | `TCG-N46` | `WindowsAppUserModelIdWin` | `crates/gpui_telegram_desktop/src/platform/win/windows_app_user_model_id.rs` | 48 | `tests/test_tcg_n46_windows_app_user_model_id.rs` |
| `Telegram/SourceFiles/platform/win/windows_autostart_task.cpp` | `TCG-N46` | `WindowsAutostartTaskWin` | `crates/gpui_telegram_desktop/src/platform/win/windows_autostart_task.rs` | 111 | `tests/test_tcg_n46_windows_autostart_task.rs` |
| `Telegram/SourceFiles/platform/win/windows_autostart_task.h` | `TCG-N46` | `WindowsAutostartTaskWin` | `crates/gpui_telegram_desktop/src/platform/win/windows_autostart_task.rs` | 16 | `tests/test_tcg_n46_windows_autostart_task.rs` |
| `Telegram/SourceFiles/platform/win/windows_dlls.cpp` | `TCG-N46` | `WindowsDllsWin` | `crates/gpui_telegram_desktop/src/platform/win/windows_dlls.rs` | 102 | `tests/test_tcg_n46_windows_dlls.rs` |
| `Telegram/SourceFiles/platform/win/windows_dlls.h` | `TCG-N46` | `WindowsDllsWin` | `crates/gpui_telegram_desktop/src/platform/win/windows_dlls.rs` | 132 | `tests/test_tcg_n46_windows_dlls.rs` |
| `Telegram/SourceFiles/platform/win/windows_quiethours.idl` | `TCG-N46` | `WindowsQuiethoursWinIdlAsset` | `crates/gpui_telegram_desktop/assets/windows_quiethours_idl.rs` | 79 | `tests/test_tcg_n46_windows_quiethours.rs` |
| `Telegram/SourceFiles/platform/win/windows_taskbar_buttons.cpp` | `TCG-N46` | `WindowsTaskbarButtonsWin` | `crates/gpui_telegram_desktop/src/platform/win/windows_taskbar_buttons.rs` | 299 | `tests/test_tcg_n46_windows_taskbar_buttons.rs` |
| `Telegram/SourceFiles/platform/win/windows_taskbar_buttons.h` | `TCG-N46` | `WindowsTaskbarButtonsWin` | `crates/gpui_telegram_desktop/src/platform/win/windows_taskbar_buttons.rs` | 67 | `tests/test_tcg_n46_windows_taskbar_buttons.rs` |
| `Telegram/SourceFiles/platform/win/windows_toast_activator.cpp` | `TCG-N46` | `WindowsToastActivatorWin` | `crates/gpui_telegram_desktop/src/platform/win/windows_toast_activator.rs` | 92 | `tests/test_tcg_n46_windows_toast_activator.rs` |
| `Telegram/SourceFiles/platform/win/windows_toast_activator.h` | `TCG-N46` | `WindowsToastActivatorWin` | `crates/gpui_telegram_desktop/src/platform/win/windows_toast_activator.rs` | 52 | `tests/test_tcg_n46_windows_toast_activator.rs` |
| `Telegram/SourceFiles/platform/win/windows_toastactivator.idl` | `TCG-N46` | `WindowsToastactivatorWinIdlAsset` | `crates/gpui_telegram_desktop/assets/windows_toastactivator_idl.rs` | 29 | `tests/test_tcg_n46_windows_toastactivator.rs` |

## Deep Source Reading Evidence

### `Telegram/SourceFiles/platform/linux/current_geo_location_linux.cpp`
- **Classes & Structs**: GClueSimple, GClueLocation, GClueAccuracyLevel
- **Core Methods & Signals**: Platform::LoadLibrary, QGuiApplication::desktopFileName, gsl::finally, LOAD_LIBRARY_SYMBOL
- **Target**: `CurrentGeoLocationLinuxLinux` in `crates/gpui_telegram_desktop/src/platform/linux/current_geo_location_linux.rs`

### `Telegram/SourceFiles/platform/linux/current_geo_location_linux.h`
- **Style Tokens**: asset:current_geo_location_linux.h
- **Target**: `CurrentGeoLocationLinuxLinux` in `crates/gpui_telegram_desktop/src/platform/linux/current_geo_location_linux.rs`

### `Telegram/SourceFiles/platform/linux/integration_linux.cpp`
- **Classes & Structs**: Application, LinuxIntegration
- **Core Methods & Signals**: Platform::IsWayland, Sandbox::Instance, Core::App, QString::fromStdString, QGuiApplication::sendEvent, Variant::new_dict_entry
- **Target**: `IntegrationLinuxLinux` in `crates/gpui_telegram_desktop/src/platform/linux/integration_linux.rs`

### `Telegram/SourceFiles/platform/linux/integration_linux.h`
- **Classes & Structs**: Integration
- **Target**: `IntegrationLinuxLinux` in `crates/gpui_telegram_desktop/src/platform/linux/integration_linux.rs`

### `Telegram/SourceFiles/platform/linux/launcher_linux.cpp`
- **Classes & Structs**: SpawnOutput
- **Core Methods & Signals**: QFile::encodeName, gsl::finally, QDateTime::currentDateTime, QByteArray::number, Launcher::Launcher, Core::Launcher
- **Target**: `LauncherLinuxLinux` in `crates/gpui_telegram_desktop/src/platform/linux/launcher_linux.rs`

### `Telegram/SourceFiles/platform/linux/launcher_linux.h`
- **Classes & Structs**: Launcher
- **Core Methods & Signals**: Launcher, exec, launchUpdater, installProtectedUpdate, relaunchAfterUpdate
- **Target**: `LauncherLinuxLinux` in `crates/gpui_telegram_desktop/src/platform/linux/launcher_linux.rs`

### `Telegram/SourceFiles/platform/linux/main_window_linux.cpp`
- **Classes & Structs**: WorkMode, Field
- **Core Methods & Signals**: XCB::GetRootWindow, XCB::GetAtom, QApplication::focusWidget, QApplication::postEvent, MainWindow::MainWindow, Window::MainWindow
- **Target**: `MainWindowLinuxLinux` in `crates/gpui_telegram_desktop/src/platform/linux/main_window_linux.rs`

### `Telegram/SourceFiles/platform/linux/main_window_linux.h`
- **Classes & Structs**: QMenuBar, PopupMenu, MainWindow
- **Core Methods & Signals**: MainWindow, updateWindowIcon, eventFilter, unreadCounterChangedHook, updateGlobalMenuHook, workmodeUpdated
- **Target**: `MainWindowLinuxLinux` in `crates/gpui_telegram_desktop/src/platform/linux/main_window_linux.rs`

### `Telegram/SourceFiles/platform/linux/notifications_manager_linux.cpp`
- **Classes & Structs**: ServerInformation, Manager, NotificationData, Notification
- **Core Methods & Signals**: ranges::contains, Gio::bus_get_sync, DBus::ListActivatableNames, Sandbox::Instance, Core::App, DBus::StartServiceByNameAsync
- **Target**: `NotificationsManagerLinuxLinux` in `crates/gpui_telegram_desktop/src/platform/linux/notifications_manager_linux.rs`

### `Telegram/SourceFiles/platform/linux/notifications_manager_linux.h`
- **Classes & Structs**: Manager, Private
- **Core Methods & Signals**: Manager, doShowNativeNotification, doClearAllFast, doClearFromItem, doClearFromTopic, doClearFromSublist
- **Target**: `NotificationsManagerLinuxLinux` in `crates/gpui_telegram_desktop/src/platform/linux/notifications_manager_linux.rs`

