# Subsystem Shard: `core`

**Total Source Files**: 276  
**Total Source Lines**: 104,319  
**Primary Feature IDs**: CAP-06, TCG-N02, TCG-N09, TCG-N11  
**JSON Shard Path**: `docs/specs/ledger/shards/core.json`  

## Component Inventory & Parity Mapping

| Source Path | Feature ID | Target Symbol | Target Rust / GPUI Path | Lines | Test ID |
|---|---|---|---|---|---|
| `Telegram/SourceFiles/AGENTS.md` | `TCG-N02` | `AgentsDoc` | `crates/gpui_telegram_core/docs/AGENTS_doc.rs` | 394 | `tests/test_tcg_n02_AGENTS.rs` |
| `Telegram/SourceFiles/_other/packer.cpp` | `TCG-N02` | `Packer` | `crates/gpui_telegram_core/src/core/packer.rs` | 1085 | `tests/test_tcg_n02_packer.rs` |
| `Telegram/SourceFiles/_other/packer.h` | `TCG-N02` | `Packer` | `crates/gpui_telegram_core/src/core/packer.rs` | 42 | `tests/test_tcg_n02_packer.rs` |
| `Telegram/SourceFiles/_other/startup_task_win.cpp` | `TCG-N02` | `StartupTaskWin` | `crates/gpui_telegram_core/src/core/startup_task_win.rs` | 55 | `tests/test_tcg_n02_startup_task_win.rs` |
| `Telegram/SourceFiles/_other/updater.h` | `TCG-N02` | `Updater` | `crates/gpui_telegram_core/src/core/updater.rs` | 49 | `tests/test_tcg_n02_updater.rs` |
| `Telegram/SourceFiles/_other/updater_linux.cpp` | `TCG-N02` | `UpdaterLinux` | `crates/gpui_telegram_core/src/core/updater_linux.rs` | 548 | `tests/test_tcg_n02_updater_linux.rs` |
| `Telegram/SourceFiles/_other/updater_osx.m` | `TCG-N02` | `UpdaterOsx` | `crates/gpui_telegram_core/src/core/updater_osx.rs` | 285 | `tests/test_tcg_n02_updater_osx.rs` |
| `Telegram/SourceFiles/_other/updater_win.cpp` | `TCG-N02` | `UpdaterWin` | `crates/gpui_telegram_core/src/core/updater_win.rs` | 756 | `tests/test_tcg_n02_updater_win.rs` |
| `Telegram/SourceFiles/_other/zxcvbn_translit.cpp` | `TCG-N02` | `ZxcvbnTranslit` | `crates/gpui_telegram_core/src/core/zxcvbn_translit.rs` | 156 | `tests/test_tcg_n02_zxcvbn_translit.rs` |
| `Telegram/SourceFiles/apiwrap.cpp` | `TCG-N02` | `Apiwrap` | `crates/gpui_telegram_core/src/core/apiwrap.rs` | 5941 | `tests/test_tcg_n02_apiwrap.rs` |
| `Telegram/SourceFiles/apiwrap.h` | `TCG-N02` | `Apiwrap` | `crates/gpui_telegram_core/src/core/apiwrap.rs` | 883 | `tests/test_tcg_n02_apiwrap.rs` |
| `Telegram/SourceFiles/codegen/scheme/codegen_scheme.py` | `TCG-N02` | `CodegenSchemeSchemePyAsset` | `crates/gpui_telegram_core/assets/codegen_scheme_py.rs` | 100 | `tests/test_tcg_n02_codegen_scheme.rs` |
| `Telegram/SourceFiles/config.h` | `TCG-N02` | `Config` | `crates/gpui_telegram_core/src/core/config.rs` | 97 | `tests/test_tcg_n02_config.rs` |
| `Telegram/SourceFiles/core/application.cpp` | `TCG-N02` | `Application` | `crates/gpui_telegram_core/src/core/application.rs` | 2185 | `tests/test_tcg_n02_application.rs` |
| `Telegram/SourceFiles/core/application.h` | `TCG-N02` | `Application` | `crates/gpui_telegram_core/src/core/application.rs` | 517 | `tests/test_tcg_n02_application.rs` |
| `Telegram/SourceFiles/core/bank_card_click_handler.cpp` | `TCG-N02` | `BankCardClickHandler` | `crates/gpui_telegram_core/src/core/bank_card_click_handler.rs` | 279 | `tests/test_tcg_n02_bank_card_click_handler.rs` |
| `Telegram/SourceFiles/core/bank_card_click_handler.h` | `TCG-N02` | `BankCardClickHandler` | `crates/gpui_telegram_core/src/core/bank_card_click_handler.rs` | 30 | `tests/test_tcg_n02_bank_card_click_handler.rs` |
| `Telegram/SourceFiles/core/base_integration.cpp` | `TCG-N02` | `BaseIntegration` | `crates/gpui_telegram_core/src/core/base_integration.rs` | 47 | `tests/test_tcg_n02_base_integration.rs` |
| `Telegram/SourceFiles/core/base_integration.h` | `TCG-N02` | `BaseIntegration` | `crates/gpui_telegram_core/src/core/base_integration.rs` | 29 | `tests/test_tcg_n02_base_integration.rs` |
| `Telegram/SourceFiles/core/cached_webview_availability.h` | `TCG-N34` | `CachedWebviewAvailability` | `crates/gpui_telegram_core/src/core/cached_webview_availability.rs` | 19 | `tests/test_tcg_n34_cached_webview_availability.rs` |
| `Telegram/SourceFiles/core/changelogs.cpp` | `TCG-N02` | `Changelogs` | `crates/gpui_telegram_core/src/core/changelogs.rs` | 170 | `tests/test_tcg_n02_changelogs.rs` |
| `Telegram/SourceFiles/core/changelogs.h` | `TCG-N02` | `Changelogs` | `crates/gpui_telegram_core/src/core/changelogs.rs` | 42 | `tests/test_tcg_n02_changelogs.rs` |
| `Telegram/SourceFiles/core/click_handler_types.cpp` | `TCG-N02` | `ClickHandlerTypes` | `crates/gpui_telegram_core/src/core/click_handler_types.rs` | 954 | `tests/test_tcg_n02_click_handler_types.rs` |
| `Telegram/SourceFiles/core/click_handler_types.h` | `TCG-N02` | `ClickHandlerTypes` | `crates/gpui_telegram_core/src/core/click_handler_types.rs` | 280 | `tests/test_tcg_n02_click_handler_types.rs` |
| `Telegram/SourceFiles/core/core_cloud_password.cpp` | `TCG-N02` | `CoreCloudPassword` | `crates/gpui_telegram_core/src/core/core_cloud_password.rs` | 358 | `tests/test_tcg_n02_core_cloud_password.rs` |
| `Telegram/SourceFiles/core/core_cloud_password.h` | `TCG-N02` | `CoreCloudPassword` | `crates/gpui_telegram_core/src/core/core_cloud_password.rs` | 145 | `tests/test_tcg_n02_core_cloud_password.rs` |
| `Telegram/SourceFiles/core/core_screenshot_protection.cpp` | `TCG-N02` | `CoreScreenshotProtection` | `crates/gpui_telegram_core/src/core/core_screenshot_protection.rs` | 107 | `tests/test_tcg_n02_core_screenshot_protection.rs` |
| `Telegram/SourceFiles/core/core_screenshot_protection.h` | `TCG-N02` | `CoreScreenshotProtection` | `crates/gpui_telegram_core/src/core/core_screenshot_protection.rs` | 58 | `tests/test_tcg_n02_core_screenshot_protection.rs` |
| `Telegram/SourceFiles/core/core_settings.cpp` | `TCG-N02` | `CoreSettings` | `crates/gpui_telegram_core/src/core/core_settings.rs` | 2010 | `tests/test_tcg_n02_core_settings.rs` |
| `Telegram/SourceFiles/core/core_settings.h` | `TCG-N02` | `CoreSettings` | `crates/gpui_telegram_core/src/core/core_settings.rs` | 1234 | `tests/test_tcg_n02_core_settings.rs` |
| `Telegram/SourceFiles/core/core_settings_proxy.cpp` | `TCG-N35` | `CoreSettingsProxy` | `crates/gpui_telegram_core/src/core/core_settings_proxy.rs` | 427 | `tests/test_tcg_n35_core_settings_proxy.rs` |
| `Telegram/SourceFiles/core/core_settings_proxy.h` | `TCG-N35` | `CoreSettingsProxy` | `crates/gpui_telegram_core/src/core/core_settings_proxy.rs` | 91 | `tests/test_tcg_n35_core_settings_proxy.rs` |
| `Telegram/SourceFiles/core/crash_report_window.cpp` | `TCG-N02` | `CrashReportWindow` | `crates/gpui_telegram_core/src/core/crash_report_window.rs` | 1248 | `tests/test_tcg_n02_crash_report_window.rs` |
| `Telegram/SourceFiles/core/crash_report_window.h` | `TCG-N02` | `CrashReportWindow` | `crates/gpui_telegram_core/src/core/crash_report_window.rs` | 214 | `tests/test_tcg_n02_crash_report_window.rs` |
| `Telegram/SourceFiles/core/crash_reports.cpp` | `TCG-N02` | `CrashReports` | `crates/gpui_telegram_core/src/core/crash_reports.rs` | 616 | `tests/test_tcg_n02_crash_reports.rs` |
| `Telegram/SourceFiles/core/crash_reports.h` | `TCG-N02` | `CrashReports` | `crates/gpui_telegram_core/src/core/crash_reports.rs` | 55 | `tests/test_tcg_n02_crash_reports.rs` |
| `Telegram/SourceFiles/core/credits_amount.h` | `TCG-N02` | `CreditsAmount` | `crates/gpui_telegram_core/src/core/credits_amount.rs` | 185 | `tests/test_tcg_n02_credits_amount.rs` |
| `Telegram/SourceFiles/core/current_geo_location.cpp` | `TCG-N41` | `CurrentGeoLocation` | `crates/gpui_telegram_core/src/core/current_geo_location.rs` | 243 | `tests/test_tcg_n41_current_geo_location.rs` |
| `Telegram/SourceFiles/core/current_geo_location.h` | `TCG-N41` | `CurrentGeoLocation` | `crates/gpui_telegram_core/src/core/current_geo_location.rs` | 60 | `tests/test_tcg_n41_current_geo_location.rs` |
| `Telegram/SourceFiles/core/deadlock_detector.h` | `TCG-N02` | `DeadlockDetector` | `crates/gpui_telegram_core/src/core/deadlock_detector.rs` | 83 | `tests/test_tcg_n02_deadlock_detector.rs` |
| `Telegram/SourceFiles/core/deep_links/deep_links_chats.cpp` | `TCG-N11` | `DeepLinksChatsDeepLinks` | `crates/gpui_telegram_core/src/core/deep_links/deep_links_chats.rs` | 50 | `tests/test_tcg_n11_deep_links_chats.rs` |
| `Telegram/SourceFiles/core/deep_links/deep_links_chats.h` | `TCG-N11` | `DeepLinksChatsDeepLinks` | `crates/gpui_telegram_core/src/core/deep_links/deep_links_chats.rs` | 16 | `tests/test_tcg_n11_deep_links_chats.rs` |
| `Telegram/SourceFiles/core/deep_links/deep_links_contacts.cpp` | `TCG-N02` | `DeepLinksContactsDeepLinks` | `crates/gpui_telegram_core/src/core/deep_links/deep_links_contacts.rs` | 67 | `tests/test_tcg_n02_deep_links_contacts.rs` |
| `Telegram/SourceFiles/core/deep_links/deep_links_contacts.h` | `TCG-N02` | `DeepLinksContactsDeepLinks` | `crates/gpui_telegram_core/src/core/deep_links/deep_links_contacts.rs` | 16 | `tests/test_tcg_n02_deep_links_contacts.rs` |
| `Telegram/SourceFiles/core/deep_links/deep_links_new.cpp` | `TCG-N02` | `DeepLinksNewDeepLinks` | `crates/gpui_telegram_core/src/core/deep_links/deep_links_new.rs` | 139 | `tests/test_tcg_n02_deep_links_new.rs` |
| `Telegram/SourceFiles/core/deep_links/deep_links_new.h` | `TCG-N02` | `DeepLinksNewDeepLinks` | `crates/gpui_telegram_core/src/core/deep_links/deep_links_new.rs` | 16 | `tests/test_tcg_n02_deep_links_new.rs` |
| `Telegram/SourceFiles/core/deep_links/deep_links_router.cpp` | `TCG-N02` | `DeepLinksRouterDeepLinks` | `crates/gpui_telegram_core/src/core/deep_links/deep_links_router.rs` | 204 | `tests/test_tcg_n02_deep_links_router.rs` |
| `Telegram/SourceFiles/core/deep_links/deep_links_router.h` | `TCG-N02` | `DeepLinksRouterDeepLinks` | `crates/gpui_telegram_core/src/core/deep_links/deep_links_router.rs` | 52 | `tests/test_tcg_n02_deep_links_router.rs` |
| `Telegram/SourceFiles/core/deep_links/deep_links_settings.cpp` | `TCG-N02` | `DeepLinksSettingsDeepLinks` | `crates/gpui_telegram_core/src/core/deep_links/deep_links_settings.rs` | 2179 | `tests/test_tcg_n02_deep_links_settings.rs` |
| `Telegram/SourceFiles/core/deep_links/deep_links_settings.h` | `TCG-N02` | `DeepLinksSettingsDeepLinks` | `crates/gpui_telegram_core/src/core/deep_links/deep_links_settings.rs` | 22 | `tests/test_tcg_n02_deep_links_settings.rs` |
| `Telegram/SourceFiles/core/deep_links/deep_links_types.h` | `TCG-N02` | `DeepLinksTypesDeepLinks` | `crates/gpui_telegram_core/src/core/deep_links/deep_links_types.rs` | 65 | `tests/test_tcg_n02_deep_links_types.rs` |
| `Telegram/SourceFiles/core/external_control.cpp` | `TCG-N02` | `ExternalControl` | `crates/gpui_telegram_core/src/core/external_control.rs` | 638 | `tests/test_tcg_n02_external_control.rs` |
| `Telegram/SourceFiles/core/external_control.h` | `TCG-N02` | `ExternalControl` | `crates/gpui_telegram_core/src/core/external_control.rs` | 14 | `tests/test_tcg_n02_external_control.rs` |
| `Telegram/SourceFiles/core/file_location.cpp` | `TCG-N41` | `FileLocation` | `crates/gpui_telegram_core/src/core/file_location.rs` | 133 | `tests/test_tcg_n41_file_location.rs` |
| `Telegram/SourceFiles/core/file_location.h` | `TCG-N41` | `FileLocation` | `crates/gpui_telegram_core/src/core/file_location.rs` | 77 | `tests/test_tcg_n41_file_location.rs` |
| `Telegram/SourceFiles/core/file_utilities.cpp` | `TCG-N02` | `FileUtilities` | `crates/gpui_telegram_core/src/core/file_utilities.rs` | 433 | `tests/test_tcg_n02_file_utilities.rs` |
| `Telegram/SourceFiles/core/file_utilities.h` | `TCG-N02` | `FileUtilities` | `crates/gpui_telegram_core/src/core/file_utilities.rs` | 125 | `tests/test_tcg_n02_file_utilities.rs` |
| `Telegram/SourceFiles/core/launcher.cpp` | `TCG-N02` | `Launcher` | `crates/gpui_telegram_core/src/core/launcher.rs` | 686 | `tests/test_tcg_n02_launcher.rs` |
| `Telegram/SourceFiles/core/launcher.h` | `TCG-N02` | `Launcher` | `crates/gpui_telegram_core/src/core/launcher.rs` | 96 | `tests/test_tcg_n02_launcher.rs` |
| `Telegram/SourceFiles/core/local_url_handlers.cpp` | `TCG-N02` | `LocalUrlHandlers` | `crates/gpui_telegram_core/src/core/local_url_handlers.rs` | 2407 | `tests/test_tcg_n02_local_url_handlers.rs` |
| `Telegram/SourceFiles/core/local_url_handlers.h` | `TCG-N02` | `LocalUrlHandlers` | `crates/gpui_telegram_core/src/core/local_url_handlers.rs` | 62 | `tests/test_tcg_n02_local_url_handlers.rs` |
| `Telegram/SourceFiles/core/mime_type.cpp` | `TCG-N02` | `MimeType` | `crates/gpui_telegram_core/src/core/mime_type.rs` | 370 | `tests/test_tcg_n02_mime_type.rs` |
| `Telegram/SourceFiles/core/mime_type.h` | `TCG-N02` | `MimeType` | `crates/gpui_telegram_core/src/core/mime_type.rs` | 90 | `tests/test_tcg_n02_mime_type.rs` |
| `Telegram/SourceFiles/core/phone_click_handler.cpp` | `TCG-N02` | `PhoneClickHandler` | `crates/gpui_telegram_core/src/core/phone_click_handler.rs` | 367 | `tests/test_tcg_n02_phone_click_handler.rs` |
| `Telegram/SourceFiles/core/phone_click_handler.h` | `TCG-N02` | `PhoneClickHandler` | `crates/gpui_telegram_core/src/core/phone_click_handler.rs` | 30 | `tests/test_tcg_n02_phone_click_handler.rs` |
| `Telegram/SourceFiles/core/proxy_rotation_manager.cpp` | `TCG-N35` | `ProxyRotationManager` | `crates/gpui_telegram_core/src/core/proxy_rotation_manager.rs` | 364 | `tests/test_tcg_n35_proxy_rotation_manager.rs` |
| `Telegram/SourceFiles/core/proxy_rotation_manager.h` | `TCG-N35` | `ProxyRotationManager` | `crates/gpui_telegram_core/src/core/proxy_rotation_manager.rs` | 80 | `tests/test_tcg_n35_proxy_rotation_manager.rs` |
| `Telegram/SourceFiles/core/sandbox.cpp` | `TCG-N02` | `Sandbox` | `crates/gpui_telegram_core/src/core/sandbox.rs` | 892 | `tests/test_tcg_n02_sandbox.rs` |
| `Telegram/SourceFiles/core/sandbox.h` | `TCG-N02` | `Sandbox` | `crates/gpui_telegram_core/src/core/sandbox.rs` | 157 | `tests/test_tcg_n02_sandbox.rs` |
| `Telegram/SourceFiles/core/shortcuts.cpp` | `TCG-N45` | `Shortcuts` | `crates/gpui_telegram_core/src/core/shortcuts.rs` | 1135 | `tests/test_tcg_n45_shortcuts.rs` |
| `Telegram/SourceFiles/core/shortcuts.h` | `TCG-N45` | `Shortcuts` | `crates/gpui_telegram_core/src/core/shortcuts.rs` | 194 | `tests/test_tcg_n45_shortcuts.rs` |
| `Telegram/SourceFiles/core/ton_explorer_url.cpp` | `TCG-N02` | `TonExplorerUrl` | `crates/gpui_telegram_core/src/core/ton_explorer_url.rs` | 45 | `tests/test_tcg_n02_ton_explorer_url.rs` |
| `Telegram/SourceFiles/core/ton_explorer_url.h` | `TCG-N02` | `TonExplorerUrl` | `crates/gpui_telegram_core/src/core/ton_explorer_url.rs` | 23 | `tests/test_tcg_n02_ton_explorer_url.rs` |
| `Telegram/SourceFiles/core/ui_integration.cpp` | `TCG-N02` | `UiIntegration` | `crates/gpui_telegram_core/src/core/ui_integration.rs` | 623 | `tests/test_tcg_n02_ui_integration.rs` |
| `Telegram/SourceFiles/core/ui_integration.h` | `TCG-N02` | `UiIntegration` | `crates/gpui_telegram_core/src/core/ui_integration.rs` | 110 | `tests/test_tcg_n02_ui_integration.rs` |
| `Telegram/SourceFiles/core/update_channel.h` | `TCG-N12` | `UpdateChannel` | `crates/gpui_telegram_core/src/core/update_channel.rs` | 88 | `tests/test_tcg_n12_update_channel.rs` |
| `Telegram/SourceFiles/core/update_checker.cpp` | `TCG-N02` | `UpdateChecker` | `crates/gpui_telegram_core/src/core/update_checker.rs` | 2267 | `tests/test_tcg_n02_update_checker.rs` |
| `Telegram/SourceFiles/core/update_checker.h` | `TCG-N02` | `UpdateChecker` | `crates/gpui_telegram_core/src/core/update_checker.rs` | 60 | `tests/test_tcg_n02_update_checker.rs` |
| `Telegram/SourceFiles/core/update_keys.cpp` | `TCG-N02` | `UpdateKeys` | `crates/gpui_telegram_core/src/core/update_keys.rs` | 32 | `tests/test_tcg_n02_update_keys.rs` |
| `Telegram/SourceFiles/core/update_keys.h` | `TCG-N02` | `UpdateKeys` | `crates/gpui_telegram_core/src/core/update_keys.rs` | 22 | `tests/test_tcg_n02_update_keys.rs` |
| `Telegram/SourceFiles/core/update_unpack.cpp` | `TCG-N02` | `UpdateUnpack` | `crates/gpui_telegram_core/src/core/update_unpack.rs` | 292 | `tests/test_tcg_n02_update_unpack.rs` |
| `Telegram/SourceFiles/core/update_unpack.h` | `TCG-N02` | `UpdateUnpack` | `crates/gpui_telegram_core/src/core/update_unpack.rs` | 30 | `tests/test_tcg_n02_update_unpack.rs` |
| `Telegram/SourceFiles/core/update_verify.cpp` | `TCG-N02` | `UpdateVerify` | `crates/gpui_telegram_core/src/core/update_verify.rs` | 926 | `tests/test_tcg_n02_update_verify.rs` |
| `Telegram/SourceFiles/core/update_verify.h` | `TCG-N02` | `UpdateVerify` | `crates/gpui_telegram_core/src/core/update_verify.rs` | 202 | `tests/test_tcg_n02_update_verify.rs` |
| `Telegram/SourceFiles/core/utils.cpp` | `TCG-N02` | `Utils` | `crates/gpui_telegram_core/src/core/utils.rs` | 512 | `tests/test_tcg_n02_utils.rs` |
| `Telegram/SourceFiles/core/utils.h` | `TCG-N02` | `Utils` | `crates/gpui_telegram_core/src/core/utils.rs` | 127 | `tests/test_tcg_n02_utils.rs` |
| `Telegram/SourceFiles/core/version.h` | `TCG-N02` | `Version` | `crates/gpui_telegram_core/src/core/version.rs` | 28 | `tests/test_tcg_n02_version.rs` |
| `Telegram/SourceFiles/core/websocket_client.cpp` | `TCG-N02` | `WebsocketClient` | `crates/gpui_telegram_core/src/core/websocket_client.rs` | 310 | `tests/test_tcg_n02_websocket_client.rs` |
| `Telegram/SourceFiles/core/websocket_client.h` | `TCG-N02` | `WebsocketClient` | `crates/gpui_telegram_core/src/core/websocket_client.rs` | 67 | `tests/test_tcg_n02_websocket_client.rs` |
| `Telegram/SourceFiles/logs.cpp` | `TCG-N02` | `Logs` | `crates/gpui_telegram_core/src/core/logs.rs` | 609 | `tests/test_tcg_n02_logs.rs` |
| `Telegram/SourceFiles/logs.h` | `TCG-N02` | `Logs` | `crates/gpui_telegram_core/src/core/logs.rs` | 72 | `tests/test_tcg_n02_logs.rs` |
| `Telegram/SourceFiles/settings.cpp` | `TCG-N02` | `Settings` | `crates/gpui_telegram_core/src/core/settings.rs` | 65 | `tests/test_tcg_n02_settings.rs` |
| `Telegram/SourceFiles/settings.h` | `TCG-N02` | `Settings` | `crates/gpui_telegram_core/src/core/settings.rs` | 140 | `tests/test_tcg_n02_settings.rs` |
| `Telegram/SourceFiles/stdafx.h` | `TCG-N02` | `Stdafx` | `crates/gpui_telegram_core/src/core/stdafx.rs` | 145 | `tests/test_tcg_n02_stdafx.rs` |
| `Telegram/SourceFiles/tde2e/tde2e_api.cpp` | `TCG-N02` | `Tde2eApi` | `crates/gpui_telegram_core/src/core/tde2e_api.rs` | 638 | `tests/test_tcg_n02_tde2e_api.rs` |
| `Telegram/SourceFiles/tde2e/tde2e_api.h` | `TCG-N02` | `Tde2eApi` | `crates/gpui_telegram_core/src/core/tde2e_api.rs` | 201 | `tests/test_tcg_n02_tde2e_api.rs` |
| `Telegram/SourceFiles/tde2e/tde2e_integration.cpp` | `TCG-N02` | `Tde2eIntegration` | `crates/gpui_telegram_core/src/core/tde2e_integration.rs` | 27 | `tests/test_tcg_n02_tde2e_integration.rs` |
| `Telegram/SourceFiles/tde2e/tde2e_integration.h` | `TCG-N02` | `Tde2eIntegration` | `crates/gpui_telegram_core/src/core/tde2e_integration.rs` | 20 | `tests/test_tcg_n02_tde2e_integration.rs` |
| `Telegram/SourceFiles/test/README.md` | `TCG-N02` | `ReadmeDoc` | `crates/gpui_telegram_core/docs/README_doc.rs` | 123 | `tests/test_tcg_n02_README.rs` |
| `Telegram/SourceFiles/test/docs/account-fixtures.md` | `TCG-N02` | `AccountFixturesDocsDoc` | `crates/gpui_telegram_core/docs/account-fixtures_doc.rs` | 94 | `tests/test_tcg_n02_account_fixtures.rs` |
| `Telegram/SourceFiles/test/docs/adding-helpers.md` | `TCG-N02` | `AddingHelpersDocsDoc` | `crates/gpui_telegram_core/docs/adding-helpers_doc.rs` | 109 | `tests/test_tcg_n02_adding_helpers.rs` |
| `Telegram/SourceFiles/test/docs/chat-lists.md` | `TCG-N11` | `ChatListsDocsDoc` | `crates/gpui_telegram_core/docs/chat-lists_doc.rs` | 51 | `tests/test_tcg_n11_chat_lists.rs` |
| `Telegram/SourceFiles/test/docs/diagnostics.md` | `TCG-N02` | `DiagnosticsDocsDoc` | `crates/gpui_telegram_core/docs/diagnostics_doc.rs` | 117 | `tests/test_tcg_n02_diagnostics.rs` |
| `Telegram/SourceFiles/test/docs/helpers/README.md` | `TCG-N02` | `ReadmeDocsHelpersDoc` | `crates/gpui_telegram_core/docs/README_doc.rs` | 73 | `tests/test_tcg_n02_README.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_activation.md` | `TCG-N02` | `TestActivationDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_activation_doc.rs` | 19 | `tests/test_tcg_n02_test_activation.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_agent.md` | `TCG-N02` | `TestAgentDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_agent_doc.rs` | 34 | `tests/test_tcg_n02_test_agent.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_animation_clock.md` | `TCG-N02` | `TestAnimationClockDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_animation_clock_doc.rs` | 129 | `tests/test_tcg_n02_test_animation_clock.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_box_button.md` | `TCG-N02` | `TestBoxButtonDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_box_button_doc.rs` | 24 | `tests/test_tcg_n02_test_box_button.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_capture.md` | `TCG-N02` | `TestCaptureDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_capture_doc.rs` | 131 | `tests/test_tcg_n02_test_capture.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_chat_menu.md` | `TCG-N11` | `TestChatMenuDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_chat_menu_doc.rs` | 64 | `tests/test_tcg_n11_test_chat_menu.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_clipboard.md` | `TCG-N09` | `TestClipboardDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_clipboard_doc.rs` | 60 | `tests/test_tcg_n09_test_clipboard.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_console_lock.md` | `TCG-N02` | `TestConsoleLockDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_console_lock_doc.rs` | 84 | `tests/test_tcg_n02_test_console_lock.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_corner_patch.md` | `TCG-N02` | `TestCornerPatchDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_corner_patch_doc.rs` | 100 | `tests/test_tcg_n02_test_corner_patch.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_custom_emoji.md` | `TCG-N15` | `TestCustomEmojiDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_custom_emoji_doc.rs` | 17 | `tests/test_tcg_n15_test_custom_emoji.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_gated_stage.md` | `TCG-N02` | `TestGatedStageDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_gated_stage_doc.rs` | 19 | `tests/test_tcg_n02_test_gated_stage.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_gram_reconcile.md` | `CAP-06` | `TestGramReconcileDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_gram_reconcile_doc.rs` | 103 | `tests/test_cap_06_test_gram_reconcile.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_history_fixtures.md` | `TCG-N02` | `TestHistoryFixturesDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_history_fixtures_doc.rs` | 17 | `tests/test_tcg_n02_test_history_fixtures.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_hover.md` | `TCG-N02` | `TestHoverDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_hover_doc.rs` | 17 | `tests/test_tcg_n02_test_hover.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_ink.md` | `TCG-N02` | `TestInkDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_ink_doc.rs` | 80 | `tests/test_tcg_n02_test_ink.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_lang_pack.md` | `TCG-N44` | `TestLangPackDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_lang_pack_doc.rs` | 42 | `tests/test_tcg_n44_test_lang_pack.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_launch_fuse.md` | `TCG-N02` | `TestLaunchFuseDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_launch_fuse_doc.rs` | 16 | `tests/test_tcg_n02_test_launch_fuse.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_layer_root.md` | `TCG-N02` | `TestLayerRootDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_layer_root_doc.rs` | 20 | `tests/test_tcg_n02_test_layer_root.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_log.md` | `TCG-N02` | `TestLogDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_log_doc.rs` | 133 | `tests/test_tcg_n02_test_log.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_log_lines.md` | `TCG-N02` | `TestLogLinesDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_log_lines_doc.rs` | 22 | `tests/test_tcg_n02_test_log_lines.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_log_scan_internal.md` | `TCG-N02` | `TestLogScanInternalDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_log_scan_internal_doc.rs` | 32 | `tests/test_tcg_n02_test_log_scan_internal.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_mapped_target.md` | `TCG-N41` | `TestMappedTargetDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_mapped_target_doc.rs` | 21 | `tests/test_tcg_n41_test_mapped_target.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_marking_read.md` | `TCG-N02` | `TestMarkingReadDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_marking_read_doc.rs` | 45 | `tests/test_tcg_n02_test_marking_read.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_menu.md` | `TCG-N02` | `TestMenuDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_menu_doc.rs` | 81 | `tests/test_tcg_n02_test_menu.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_messages.md` | `TCG-N02` | `TestMessagesDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_messages_doc.rs` | 15 | `tests/test_tcg_n02_test_messages.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_notify_override.md` | `TCG-N32` | `TestNotifyOverrideDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_notify_override_doc.rs` | 61 | `tests/test_tcg_n32_test_notify_override.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_open_handoff.md` | `TCG-N02` | `TestOpenHandoffDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_open_handoff_doc.rs` | 16 | `tests/test_tcg_n02_test_open_handoff.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_palette.md` | `TCG-N31` | `TestPaletteDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_palette_doc.rs` | 78 | `tests/test_tcg_n31_test_palette.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_panel.md` | `TCG-N02` | `TestPanelDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_panel_doc.rs` | 37 | `tests/test_tcg_n02_test_panel.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_post_paint.md` | `TCG-N02` | `TestPostPaintDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_post_paint_doc.rs` | 104 | `tests/test_tcg_n02_test_post_paint.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_press_origin.md` | `TCG-N02` | `TestPressOriginDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_press_origin_doc.rs` | 49 | `tests/test_tcg_n02_test_press_origin.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_probe.md` | `TCG-N02` | `TestProbeDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_probe_doc.rs` | 49 | `tests/test_tcg_n02_test_probe.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_publication_identity_scan.md` | `TCG-N02` | `TestPublicationIdentityScanDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_publication_identity_scan_doc.rs` | 161 | `tests/test_tcg_n02_test_publication_identity_scan.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_publication_identity_scan_self_test.md` | `TCG-N02` | `TestPublicationIdentityScanSelfTestDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_publication_identity_scan_self_test_doc.rs` | 28 | `tests/test_tcg_n02_test_publication_identity_scan_self_test.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_queued_drag.md` | `TCG-N02` | `TestQueuedDragDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_queued_drag_doc.rs` | 40 | `tests/test_tcg_n02_test_queued_drag.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_rpc_fixture.md` | `TCG-N02` | `TestRpcFixtureDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_rpc_fixture_doc.rs` | 29 | `tests/test_tcg_n02_test_rpc_fixture.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_rpc_retry.md` | `TCG-N02` | `TestRpcRetryDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_rpc_retry_doc.rs` | 22 | `tests/test_tcg_n02_test_rpc_retry.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_runner.md` | `TCG-N02` | `TestRunnerDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_runner_doc.rs` | 89 | `tests/test_tcg_n02_test_runner.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_scenario.md` | `TCG-N02` | `TestScenarioDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_scenario_doc.rs` | 19 | `tests/test_tcg_n02_test_scenario.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_secrecy_scan.md` | `TCG-N02` | `TestSecrecyScanDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_secrecy_scan_doc.rs` | 219 | `tests/test_tcg_n02_test_secrecy_scan.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_session_gate.md` | `TCG-N02` | `TestSessionGateDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_session_gate_doc.rs` | 131 | `tests/test_tcg_n02_test_session_gate.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_style.md` | `TCG-N02` | `TestStyleDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_style_doc.rs` | 16 | `tests/test_tcg_n02_test_style.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_text_drop.md` | `TCG-N02` | `TestTextDropDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_text_drop_doc.rs` | 25 | `tests/test_tcg_n02_test_text_drop.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_text_reads.md` | `TCG-N02` | `TestTextReadsDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_text_reads_doc.rs` | 21 | `tests/test_tcg_n02_test_text_reads.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_toast_capture.md` | `TCG-N02` | `TestToastCaptureDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_toast_capture_doc.rs` | 31 | `tests/test_tcg_n02_test_toast_capture.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_transfer.md` | `TCG-N02` | `TestTransferDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_transfer_doc.rs` | 16 | `tests/test_tcg_n02_test_transfer.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_via_window.md` | `TCG-N02` | `TestViaWindowDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_via_window_doc.rs` | 21 | `tests/test_tcg_n02_test_via_window.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_wallet_intercept.md` | `CAP-06` | `TestWalletInterceptDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_wallet_intercept_doc.rs` | 49 | `tests/test_cap_06_test_wallet_intercept.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_wallet_ready.md` | `CAP-06` | `TestWalletReadyDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_wallet_ready_doc.rs` | 19 | `tests/test_cap_06_test_wallet_ready.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_web_cache.md` | `TCG-N34` | `TestWebCacheDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_web_cache_doc.rs` | 32 | `tests/test_tcg_n34_test_web_cache.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_wheel.md` | `TCG-N02` | `TestWheelDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_wheel_doc.rs` | 19 | `tests/test_tcg_n02_test_wheel.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_widgets.md` | `TCG-N02` | `TestWidgetsDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_widgets_doc.rs` | 149 | `tests/test_tcg_n02_test_widgets.rs` |
| `Telegram/SourceFiles/test/docs/helpers/test_window_exposure.md` | `TCG-N02` | `TestWindowExposureDocsHelpersDoc` | `crates/gpui_telegram_core/docs/test_window_exposure_doc.rs` | 74 | `tests/test_tcg_n02_test_window_exposure.rs` |
| `Telegram/SourceFiles/test/docs/launch.md` | `TCG-N02` | `LaunchDocsDoc` | `crates/gpui_telegram_core/docs/launch_doc.rs` | 73 | `tests/test_tcg_n02_launch.rs` |
| `Telegram/SourceFiles/test/docs/media-fixtures.md` | `TCG-N02` | `MediaFixturesDocsDoc` | `crates/gpui_telegram_core/docs/media-fixtures_doc.rs` | 47 | `tests/test_tcg_n02_media_fixtures.rs` |
| `Telegram/SourceFiles/test/docs/overlay-example.md` | `TCG-N02` | `OverlayExampleDocsDoc` | `crates/gpui_telegram_core/docs/overlay-example_doc.rs` | 76 | `tests/test_tcg_n02_overlay_example.rs` |
| `Telegram/SourceFiles/test/docs/scenarios.md` | `TCG-N02` | `ScenariosDocsDoc` | `crates/gpui_telegram_core/docs/scenarios_doc.rs` | 47 | `tests/test_tcg_n02_scenarios.rs` |
| `Telegram/SourceFiles/test/docs/teardown.md` | `TCG-N02` | `TeardownDocsDoc` | `crates/gpui_telegram_core/docs/teardown_doc.rs` | 58 | `tests/test_tcg_n02_teardown.rs` |
| `Telegram/SourceFiles/test/test_activation.cpp` | `TCG-N02` | `TestActivation` | `crates/gpui_telegram_core/src/core/test_activation.rs` | 453 | `tests/test_tcg_n02_test_activation.rs` |
| `Telegram/SourceFiles/test/test_activation.h` | `TCG-N02` | `TestActivation` | `crates/gpui_telegram_core/src/core/test_activation.rs` | 69 | `tests/test_tcg_n02_test_activation.rs` |
| `Telegram/SourceFiles/test/test_agent.cpp` | `TCG-N02` | `TestAgent` | `crates/gpui_telegram_core/src/core/test_agent.rs` | 424 | `tests/test_tcg_n02_test_agent.rs` |
| `Telegram/SourceFiles/test/test_agent.h` | `TCG-N02` | `TestAgent` | `crates/gpui_telegram_core/src/core/test_agent.rs` | 105 | `tests/test_tcg_n02_test_agent.rs` |
| `Telegram/SourceFiles/test/test_animation_clock.cpp` | `TCG-N02` | `TestAnimationClock` | `crates/gpui_telegram_core/src/core/test_animation_clock.rs` | 805 | `tests/test_tcg_n02_test_animation_clock.rs` |
| `Telegram/SourceFiles/test/test_animation_clock.h` | `TCG-N02` | `TestAnimationClock` | `crates/gpui_telegram_core/src/core/test_animation_clock.rs` | 303 | `tests/test_tcg_n02_test_animation_clock.rs` |
| `Telegram/SourceFiles/test/test_box_button.cpp` | `TCG-N02` | `TestBoxButton` | `crates/gpui_telegram_core/src/core/test_box_button.rs` | 993 | `tests/test_tcg_n02_test_box_button.rs` |
| `Telegram/SourceFiles/test/test_box_button.h` | `TCG-N02` | `TestBoxButton` | `crates/gpui_telegram_core/src/core/test_box_button.rs` | 158 | `tests/test_tcg_n02_test_box_button.rs` |
| `Telegram/SourceFiles/test/test_capture.cpp` | `TCG-N02` | `TestCapture` | `crates/gpui_telegram_core/src/core/test_capture.rs` | 748 | `tests/test_tcg_n02_test_capture.rs` |
| `Telegram/SourceFiles/test/test_capture.h` | `TCG-N02` | `TestCapture` | `crates/gpui_telegram_core/src/core/test_capture.rs` | 311 | `tests/test_tcg_n02_test_capture.rs` |
| `Telegram/SourceFiles/test/test_chat_menu.cpp` | `TCG-N11` | `TestChatMenu` | `crates/gpui_telegram_core/src/core/test_chat_menu.rs` | 2992 | `tests/test_tcg_n11_test_chat_menu.rs` |
| `Telegram/SourceFiles/test/test_chat_menu.h` | `TCG-N11` | `TestChatMenu` | `crates/gpui_telegram_core/src/core/test_chat_menu.rs` | 352 | `tests/test_tcg_n11_test_chat_menu.rs` |
| `Telegram/SourceFiles/test/test_clipboard.cpp` | `TCG-N09` | `TestClipboard` | `crates/gpui_telegram_core/src/core/test_clipboard.rs` | 1044 | `tests/test_tcg_n09_test_clipboard.rs` |
| `Telegram/SourceFiles/test/test_clipboard.h` | `TCG-N09` | `TestClipboard` | `crates/gpui_telegram_core/src/core/test_clipboard.rs` | 292 | `tests/test_tcg_n09_test_clipboard.rs` |
| `Telegram/SourceFiles/test/test_console_lock.cpp` | `TCG-N02` | `TestConsoleLock` | `crates/gpui_telegram_core/src/core/test_console_lock.rs` | 222 | `tests/test_tcg_n02_test_console_lock.rs` |
| `Telegram/SourceFiles/test/test_console_lock.h` | `TCG-N02` | `TestConsoleLock` | `crates/gpui_telegram_core/src/core/test_console_lock.rs` | 147 | `tests/test_tcg_n02_test_console_lock.rs` |
| `Telegram/SourceFiles/test/test_corner_patch.cpp` | `TCG-N02` | `TestCornerPatch` | `crates/gpui_telegram_core/src/core/test_corner_patch.rs` | 1510 | `tests/test_tcg_n02_test_corner_patch.rs` |
| `Telegram/SourceFiles/test/test_corner_patch.h` | `TCG-N02` | `TestCornerPatch` | `crates/gpui_telegram_core/src/core/test_corner_patch.rs` | 383 | `tests/test_tcg_n02_test_corner_patch.rs` |
| `Telegram/SourceFiles/test/test_custom_emoji.cpp` | `TCG-N15` | `TestCustomEmoji` | `crates/gpui_telegram_core/src/core/test_custom_emoji.rs` | 97 | `tests/test_tcg_n15_test_custom_emoji.rs` |
| `Telegram/SourceFiles/test/test_custom_emoji.h` | `TCG-N15` | `TestCustomEmoji` | `crates/gpui_telegram_core/src/core/test_custom_emoji.rs` | 62 | `tests/test_tcg_n15_test_custom_emoji.rs` |
| `Telegram/SourceFiles/test/test_gated_stage.cpp` | `TCG-N02` | `TestGatedStage` | `crates/gpui_telegram_core/src/core/test_gated_stage.rs` | 77 | `tests/test_tcg_n02_test_gated_stage.rs` |
| `Telegram/SourceFiles/test/test_gated_stage.h` | `TCG-N02` | `TestGatedStage` | `crates/gpui_telegram_core/src/core/test_gated_stage.rs` | 50 | `tests/test_tcg_n02_test_gated_stage.rs` |
| `Telegram/SourceFiles/test/test_gram_reconcile.cpp` | `CAP-06` | `TestGramReconcile` | `crates/gpui_telegram_core/src/core/test_gram_reconcile.rs` | 1381 | `tests/test_cap_06_test_gram_reconcile.rs` |
| `Telegram/SourceFiles/test/test_gram_reconcile.h` | `CAP-06` | `TestGramReconcile` | `crates/gpui_telegram_core/src/core/test_gram_reconcile.rs` | 230 | `tests/test_cap_06_test_gram_reconcile.rs` |
| `Telegram/SourceFiles/test/test_history_fixtures.cpp` | `TCG-N02` | `TestHistoryFixtures` | `crates/gpui_telegram_core/src/core/test_history_fixtures.rs` | 135 | `tests/test_tcg_n02_test_history_fixtures.rs` |
| `Telegram/SourceFiles/test/test_history_fixtures.h` | `TCG-N02` | `TestHistoryFixtures` | `crates/gpui_telegram_core/src/core/test_history_fixtures.rs` | 95 | `tests/test_tcg_n02_test_history_fixtures.rs` |
| `Telegram/SourceFiles/test/test_hover.cpp` | `TCG-N02` | `TestHover` | `crates/gpui_telegram_core/src/core/test_hover.rs` | 529 | `tests/test_tcg_n02_test_hover.rs` |
| `Telegram/SourceFiles/test/test_hover.h` | `TCG-N02` | `TestHover` | `crates/gpui_telegram_core/src/core/test_hover.rs` | 54 | `tests/test_tcg_n02_test_hover.rs` |
| `Telegram/SourceFiles/test/test_ink.cpp` | `TCG-N02` | `TestInk` | `crates/gpui_telegram_core/src/core/test_ink.rs` | 2346 | `tests/test_tcg_n02_test_ink.rs` |
| `Telegram/SourceFiles/test/test_ink.h` | `TCG-N02` | `TestInk` | `crates/gpui_telegram_core/src/core/test_ink.rs` | 393 | `tests/test_tcg_n02_test_ink.rs` |
| `Telegram/SourceFiles/test/test_lang_pack.cpp` | `TCG-N44` | `TestLangPack` | `crates/gpui_telegram_core/src/core/test_lang_pack.rs` | 939 | `tests/test_tcg_n44_test_lang_pack.rs` |
| `Telegram/SourceFiles/test/test_lang_pack.h` | `TCG-N44` | `TestLangPack` | `crates/gpui_telegram_core/src/core/test_lang_pack.rs` | 404 | `tests/test_tcg_n44_test_lang_pack.rs` |
| `Telegram/SourceFiles/test/test_launch_fuse.cpp` | `TCG-N02` | `TestLaunchFuse` | `crates/gpui_telegram_core/src/core/test_launch_fuse.rs` | 134 | `tests/test_tcg_n02_test_launch_fuse.rs` |
| `Telegram/SourceFiles/test/test_launch_fuse.h` | `TCG-N02` | `TestLaunchFuse` | `crates/gpui_telegram_core/src/core/test_launch_fuse.rs` | 63 | `tests/test_tcg_n02_test_launch_fuse.rs` |
| `Telegram/SourceFiles/test/test_layer_root.cpp` | `TCG-N02` | `TestLayerRoot` | `crates/gpui_telegram_core/src/core/test_layer_root.rs` | 450 | `tests/test_tcg_n02_test_layer_root.rs` |
| `Telegram/SourceFiles/test/test_layer_root.h` | `TCG-N02` | `TestLayerRoot` | `crates/gpui_telegram_core/src/core/test_layer_root.rs` | 59 | `tests/test_tcg_n02_test_layer_root.rs` |
| `Telegram/SourceFiles/test/test_log.cpp` | `TCG-N02` | `TestLog` | `crates/gpui_telegram_core/src/core/test_log.rs` | 237 | `tests/test_tcg_n02_test_log.rs` |
| `Telegram/SourceFiles/test/test_log.h` | `TCG-N02` | `TestLog` | `crates/gpui_telegram_core/src/core/test_log.rs` | 181 | `tests/test_tcg_n02_test_log.rs` |
| `Telegram/SourceFiles/test/test_log_lines.cpp` | `TCG-N02` | `TestLogLines` | `crates/gpui_telegram_core/src/core/test_log_lines.rs` | 251 | `tests/test_tcg_n02_test_log_lines.rs` |
| `Telegram/SourceFiles/test/test_log_lines.h` | `TCG-N02` | `TestLogLines` | `crates/gpui_telegram_core/src/core/test_log_lines.rs` | 88 | `tests/test_tcg_n02_test_log_lines.rs` |
| `Telegram/SourceFiles/test/test_log_scan_internal.h` | `TCG-N02` | `TestLogScanInternal` | `crates/gpui_telegram_core/src/core/test_log_scan_internal.rs` | 37 | `tests/test_tcg_n02_test_log_scan_internal.rs` |
| `Telegram/SourceFiles/test/test_mapped_target.cpp` | `TCG-N41` | `TestMappedTarget` | `crates/gpui_telegram_core/src/core/test_mapped_target.rs` | 516 | `tests/test_tcg_n41_test_mapped_target.rs` |
| `Telegram/SourceFiles/test/test_mapped_target.h` | `TCG-N41` | `TestMappedTarget` | `crates/gpui_telegram_core/src/core/test_mapped_target.rs` | 47 | `tests/test_tcg_n41_test_mapped_target.rs` |
| `Telegram/SourceFiles/test/test_marking_read.cpp` | `TCG-N02` | `TestMarkingRead` | `crates/gpui_telegram_core/src/core/test_marking_read.rs` | 497 | `tests/test_tcg_n02_test_marking_read.rs` |
| `Telegram/SourceFiles/test/test_marking_read.h` | `TCG-N02` | `TestMarkingRead` | `crates/gpui_telegram_core/src/core/test_marking_read.rs` | 87 | `tests/test_tcg_n02_test_marking_read.rs` |
| `Telegram/SourceFiles/test/test_menu.cpp` | `TCG-N02` | `TestMenu` | `crates/gpui_telegram_core/src/core/test_menu.rs` | 1617 | `tests/test_tcg_n02_test_menu.rs` |
| `Telegram/SourceFiles/test/test_menu.h` | `TCG-N02` | `TestMenu` | `crates/gpui_telegram_core/src/core/test_menu.rs` | 449 | `tests/test_tcg_n02_test_menu.rs` |
| `Telegram/SourceFiles/test/test_messages.cpp` | `TCG-N02` | `TestMessages` | `crates/gpui_telegram_core/src/core/test_messages.rs` | 274 | `tests/test_tcg_n02_test_messages.rs` |
| `Telegram/SourceFiles/test/test_messages.h` | `TCG-N02` | `TestMessages` | `crates/gpui_telegram_core/src/core/test_messages.rs` | 43 | `tests/test_tcg_n02_test_messages.rs` |
| `Telegram/SourceFiles/test/test_notify_override.cpp` | `TCG-N32` | `TestNotifyOverride` | `crates/gpui_telegram_core/src/core/test_notify_override.rs` | 1160 | `tests/test_tcg_n32_test_notify_override.rs` |
| `Telegram/SourceFiles/test/test_notify_override.h` | `TCG-N32` | `TestNotifyOverride` | `crates/gpui_telegram_core/src/core/test_notify_override.rs` | 430 | `tests/test_tcg_n32_test_notify_override.rs` |
| `Telegram/SourceFiles/test/test_open_handoff.cpp` | `TCG-N02` | `TestOpenHandoff` | `crates/gpui_telegram_core/src/core/test_open_handoff.rs` | 255 | `tests/test_tcg_n02_test_open_handoff.rs` |
| `Telegram/SourceFiles/test/test_open_handoff.h` | `TCG-N02` | `TestOpenHandoff` | `crates/gpui_telegram_core/src/core/test_open_handoff.rs` | 77 | `tests/test_tcg_n02_test_open_handoff.rs` |
| `Telegram/SourceFiles/test/test_palette.cpp` | `TCG-N31` | `TestPalette` | `crates/gpui_telegram_core/src/core/test_palette.rs` | 1584 | `tests/test_tcg_n31_test_palette.rs` |
| `Telegram/SourceFiles/test/test_palette.h` | `TCG-N31` | `TestPalette` | `crates/gpui_telegram_core/src/core/test_palette.rs` | 365 | `tests/test_tcg_n31_test_palette.rs` |
| `Telegram/SourceFiles/test/test_panel.cpp` | `TCG-N02` | `TestPanel` | `crates/gpui_telegram_core/src/core/test_panel.rs` | 827 | `tests/test_tcg_n02_test_panel.rs` |
| `Telegram/SourceFiles/test/test_panel.h` | `TCG-N02` | `TestPanel` | `crates/gpui_telegram_core/src/core/test_panel.rs` | 309 | `tests/test_tcg_n02_test_panel.rs` |
| `Telegram/SourceFiles/test/test_post_paint.cpp` | `TCG-N02` | `TestPostPaint` | `crates/gpui_telegram_core/src/core/test_post_paint.rs` | 1461 | `tests/test_tcg_n02_test_post_paint.rs` |
| `Telegram/SourceFiles/test/test_post_paint.h` | `TCG-N02` | `TestPostPaint` | `crates/gpui_telegram_core/src/core/test_post_paint.rs` | 350 | `tests/test_tcg_n02_test_post_paint.rs` |
| `Telegram/SourceFiles/test/test_press_origin.cpp` | `TCG-N02` | `TestPressOrigin` | `crates/gpui_telegram_core/src/core/test_press_origin.rs` | 2166 | `tests/test_tcg_n02_test_press_origin.rs` |
| `Telegram/SourceFiles/test/test_press_origin.h` | `TCG-N02` | `TestPressOrigin` | `crates/gpui_telegram_core/src/core/test_press_origin.rs` | 190 | `tests/test_tcg_n02_test_press_origin.rs` |
| `Telegram/SourceFiles/test/test_probe.cpp` | `TCG-N02` | `TestProbe` | `crates/gpui_telegram_core/src/core/test_probe.rs` | 384 | `tests/test_tcg_n02_test_probe.rs` |
| `Telegram/SourceFiles/test/test_probe.h` | `TCG-N02` | `TestProbe` | `crates/gpui_telegram_core/src/core/test_probe.rs` | 303 | `tests/test_tcg_n02_test_probe.rs` |
| `Telegram/SourceFiles/test/test_publication_identity_scan.cpp` | `TCG-N02` | `TestPublicationIdentityScan` | `crates/gpui_telegram_core/src/core/test_publication_identity_scan.rs` | 1272 | `tests/test_tcg_n02_test_publication_identity_scan.rs` |
| `Telegram/SourceFiles/test/test_publication_identity_scan.h` | `TCG-N02` | `TestPublicationIdentityScan` | `crates/gpui_telegram_core/src/core/test_publication_identity_scan.rs` | 159 | `tests/test_tcg_n02_test_publication_identity_scan.rs` |
| `Telegram/SourceFiles/test/test_publication_identity_scan_self_test.cpp` | `TCG-N02` | `TestPublicationIdentityScanSelfTest` | `crates/gpui_telegram_core/src/core/test_publication_identity_scan_self_test.rs` | 1298 | `tests/test_tcg_n02_test_publication_identity_scan_self_test.rs` |
| `Telegram/SourceFiles/test/test_queued_drag.cpp` | `TCG-N02` | `TestQueuedDrag` | `crates/gpui_telegram_core/src/core/test_queued_drag.rs` | 766 | `tests/test_tcg_n02_test_queued_drag.rs` |
| `Telegram/SourceFiles/test/test_queued_drag.h` | `TCG-N02` | `TestQueuedDrag` | `crates/gpui_telegram_core/src/core/test_queued_drag.rs` | 246 | `tests/test_tcg_n02_test_queued_drag.rs` |
| `Telegram/SourceFiles/test/test_rpc_fixture.cpp` | `TCG-N02` | `TestRpcFixture` | `crates/gpui_telegram_core/src/core/test_rpc_fixture.rs` | 459 | `tests/test_tcg_n02_test_rpc_fixture.rs` |
| `Telegram/SourceFiles/test/test_rpc_fixture.h` | `TCG-N02` | `TestRpcFixture` | `crates/gpui_telegram_core/src/core/test_rpc_fixture.rs` | 163 | `tests/test_tcg_n02_test_rpc_fixture.rs` |
| `Telegram/SourceFiles/test/test_rpc_retry.cpp` | `TCG-N02` | `TestRpcRetry` | `crates/gpui_telegram_core/src/core/test_rpc_retry.rs` | 322 | `tests/test_tcg_n02_test_rpc_retry.rs` |
| `Telegram/SourceFiles/test/test_rpc_retry.h` | `TCG-N02` | `TestRpcRetry` | `crates/gpui_telegram_core/src/core/test_rpc_retry.rs` | 63 | `tests/test_tcg_n02_test_rpc_retry.rs` |
| `Telegram/SourceFiles/test/test_runner.cpp` | `TCG-N02` | `TestRunner` | `crates/gpui_telegram_core/src/core/test_runner.rs` | 639 | `tests/test_tcg_n02_test_runner.rs` |
| `Telegram/SourceFiles/test/test_runner.h` | `TCG-N02` | `TestRunner` | `crates/gpui_telegram_core/src/core/test_runner.rs` | 168 | `tests/test_tcg_n02_test_runner.rs` |
| `Telegram/SourceFiles/test/test_scenario.cpp` | `TCG-N02` | `TestScenario` | `crates/gpui_telegram_core/src/core/test_scenario.rs` | 22 | `tests/test_tcg_n02_test_scenario.rs` |
| `Telegram/SourceFiles/test/test_secrecy_scan.cpp` | `TCG-N02` | `TestSecrecyScan` | `crates/gpui_telegram_core/src/core/test_secrecy_scan.rs` | 5709 | `tests/test_tcg_n02_test_secrecy_scan.rs` |
| `Telegram/SourceFiles/test/test_secrecy_scan.h` | `TCG-N02` | `TestSecrecyScan` | `crates/gpui_telegram_core/src/core/test_secrecy_scan.rs` | 360 | `tests/test_tcg_n02_test_secrecy_scan.rs` |
| `Telegram/SourceFiles/test/test_session_gate.cpp` | `TCG-N02` | `TestSessionGate` | `crates/gpui_telegram_core/src/core/test_session_gate.rs` | 1663 | `tests/test_tcg_n02_test_session_gate.rs` |
| `Telegram/SourceFiles/test/test_session_gate.h` | `TCG-N02` | `TestSessionGate` | `crates/gpui_telegram_core/src/core/test_session_gate.rs` | 432 | `tests/test_tcg_n02_test_session_gate.rs` |
| `Telegram/SourceFiles/test/test_style.cpp` | `TCG-N02` | `TestStyle` | `crates/gpui_telegram_core/src/core/test_style.rs` | 305 | `tests/test_tcg_n02_test_style.rs` |
| `Telegram/SourceFiles/test/test_style.h` | `TCG-N02` | `TestStyle` | `crates/gpui_telegram_core/src/core/test_style.rs` | 70 | `tests/test_tcg_n02_test_style.rs` |
| `Telegram/SourceFiles/test/test_text_drop.cpp` | `TCG-N02` | `TestTextDrop` | `crates/gpui_telegram_core/src/core/test_text_drop.rs` | 722 | `tests/test_tcg_n02_test_text_drop.rs` |
| `Telegram/SourceFiles/test/test_text_drop.h` | `TCG-N02` | `TestTextDrop` | `crates/gpui_telegram_core/src/core/test_text_drop.rs` | 56 | `tests/test_tcg_n02_test_text_drop.rs` |
| `Telegram/SourceFiles/test/test_text_reads.cpp` | `TCG-N02` | `TestTextReads` | `crates/gpui_telegram_core/src/core/test_text_reads.rs` | 188 | `tests/test_tcg_n02_test_text_reads.rs` |
| `Telegram/SourceFiles/test/test_text_reads.h` | `TCG-N02` | `TestTextReads` | `crates/gpui_telegram_core/src/core/test_text_reads.rs` | 82 | `tests/test_tcg_n02_test_text_reads.rs` |
| `Telegram/SourceFiles/test/test_toast_capture.cpp` | `TCG-N02` | `TestToastCapture` | `crates/gpui_telegram_core/src/core/test_toast_capture.rs` | 1067 | `tests/test_tcg_n02_test_toast_capture.rs` |
| `Telegram/SourceFiles/test/test_toast_capture.h` | `TCG-N02` | `TestToastCapture` | `crates/gpui_telegram_core/src/core/test_toast_capture.rs` | 204 | `tests/test_tcg_n02_test_toast_capture.rs` |
| `Telegram/SourceFiles/test/test_transfer.cpp` | `TCG-N02` | `TestTransfer` | `crates/gpui_telegram_core/src/core/test_transfer.rs` | 209 | `tests/test_tcg_n02_test_transfer.rs` |
| `Telegram/SourceFiles/test/test_transfer.h` | `TCG-N02` | `TestTransfer` | `crates/gpui_telegram_core/src/core/test_transfer.rs` | 83 | `tests/test_tcg_n02_test_transfer.rs` |
| `Telegram/SourceFiles/test/test_via_window.cpp` | `TCG-N02` | `TestViaWindow` | `crates/gpui_telegram_core/src/core/test_via_window.rs` | 551 | `tests/test_tcg_n02_test_via_window.rs` |
| `Telegram/SourceFiles/test/test_via_window.h` | `TCG-N02` | `TestViaWindow` | `crates/gpui_telegram_core/src/core/test_via_window.rs` | 76 | `tests/test_tcg_n02_test_via_window.rs` |
| `Telegram/SourceFiles/test/test_wallet_intercept.cpp` | `CAP-06` | `TestWalletIntercept` | `crates/gpui_telegram_core/src/core/test_wallet_intercept.rs` | 1525 | `tests/test_cap_06_test_wallet_intercept.rs` |
| `Telegram/SourceFiles/test/test_wallet_intercept.h` | `CAP-06` | `TestWalletIntercept` | `crates/gpui_telegram_core/src/core/test_wallet_intercept.rs` | 456 | `tests/test_cap_06_test_wallet_intercept.rs` |
| `Telegram/SourceFiles/test/test_wallet_ready.cpp` | `CAP-06` | `TestWalletReady` | `crates/gpui_telegram_core/src/core/test_wallet_ready.rs` | 426 | `tests/test_cap_06_test_wallet_ready.rs` |
| `Telegram/SourceFiles/test/test_wallet_ready.h` | `CAP-06` | `TestWalletReady` | `crates/gpui_telegram_core/src/core/test_wallet_ready.rs` | 167 | `tests/test_cap_06_test_wallet_ready.rs` |
| `Telegram/SourceFiles/test/test_web_cache.cpp` | `TCG-N34` | `TestWebCache` | `crates/gpui_telegram_core/src/core/test_web_cache.rs` | 446 | `tests/test_tcg_n34_test_web_cache.rs` |
| `Telegram/SourceFiles/test/test_web_cache.h` | `TCG-N34` | `TestWebCache` | `crates/gpui_telegram_core/src/core/test_web_cache.rs` | 86 | `tests/test_tcg_n34_test_web_cache.rs` |
| `Telegram/SourceFiles/test/test_wheel.cpp` | `TCG-N02` | `TestWheel` | `crates/gpui_telegram_core/src/core/test_wheel.rs` | 313 | `tests/test_tcg_n02_test_wheel.rs` |
| `Telegram/SourceFiles/test/test_wheel.h` | `TCG-N02` | `TestWheel` | `crates/gpui_telegram_core/src/core/test_wheel.rs` | 44 | `tests/test_tcg_n02_test_wheel.rs` |
| `Telegram/SourceFiles/test/test_widgets.cpp` | `TCG-N02` | `TestWidgets` | `crates/gpui_telegram_core/src/core/test_widgets.rs` | 1074 | `tests/test_tcg_n02_test_widgets.rs` |
| `Telegram/SourceFiles/test/test_widgets.h` | `TCG-N02` | `TestWidgets` | `crates/gpui_telegram_core/src/core/test_widgets.rs` | 568 | `tests/test_tcg_n02_test_widgets.rs` |
| `Telegram/SourceFiles/test/test_window_exposure.cpp` | `TCG-N02` | `TestWindowExposure` | `crates/gpui_telegram_core/src/core/test_window_exposure.rs` | 883 | `tests/test_tcg_n02_test_window_exposure.rs` |
| `Telegram/SourceFiles/test/test_window_exposure.h` | `TCG-N02` | `TestWindowExposure` | `crates/gpui_telegram_core/src/core/test_window_exposure.rs` | 172 | `tests/test_tcg_n02_test_window_exposure.rs` |
| `Telegram/SourceFiles/tests/test_main.cpp` | `TCG-N02` | `TestMain` | `crates/gpui_telegram_core/src/core/test_main.rs` | 226 | `tests/test_tcg_n02_test_main.rs` |
| `Telegram/SourceFiles/tests/test_main.h` | `TCG-N02` | `TestMain` | `crates/gpui_telegram_core/src/core/test_main.rs` | 116 | `tests/test_tcg_n02_test_main.rs` |
| `Telegram/SourceFiles/tests/test_text.cpp` | `TCG-N02` | `TestText` | `crates/gpui_telegram_core/src/core/test_text.rs` | 853 | `tests/test_tcg_n02_test_text.rs` |
| `Telegram/SourceFiles/tests/test_update_unpack.cpp` | `TCG-N02` | `TestUpdateUnpack` | `crates/gpui_telegram_core/src/core/test_update_unpack.rs` | 349 | `tests/test_tcg_n02_test_update_unpack.rs` |
| `Telegram/SourceFiles/tests/test_update_verify.cpp` | `TCG-N02` | `TestUpdateVerify` | `crates/gpui_telegram_core/src/core/test_update_verify.rs` | 928 | `tests/test_tcg_n02_test_update_verify.rs` |
| `Telegram/SourceFiles/tests/test_updater_win.cpp` | `TCG-N02` | `TestUpdaterWin` | `crates/gpui_telegram_core/src/core/test_updater_win.rs` | 232 | `tests/test_tcg_n02_test_updater_win.rs` |
| `Telegram/SourceFiles/tray.cpp` | `TCG-N02` | `Tray` | `crates/gpui_telegram_core/src/core/tray.rs` | 206 | `tests/test_tcg_n02_tray.rs` |
| `Telegram/SourceFiles/tray.h` | `TCG-N02` | `Tray` | `crates/gpui_telegram_core/src/core/tray.rs` | 47 | `tests/test_tcg_n02_tray.rs` |
| `Telegram/SourceFiles/tray_accounts_menu.cpp` | `TCG-N02` | `TrayAccountsMenu` | `crates/gpui_telegram_core/src/core/tray_accounts_menu.rs` | 84 | `tests/test_tcg_n02_tray_accounts_menu.rs` |
| `Telegram/SourceFiles/tray_accounts_menu.h` | `TCG-N02` | `TrayAccountsMenu` | `crates/gpui_telegram_core/src/core/tray_accounts_menu.rs` | 19 | `tests/test_tcg_n02_tray_accounts_menu.rs` |
| `Telegram/SourceFiles/tray_accounts_menu_dummy.cpp` | `TCG-N02` | `TrayAccountsMenuDummy` | `crates/gpui_telegram_core/src/core/tray_accounts_menu_dummy.rs` | 20 | `tests/test_tcg_n02_tray_accounts_menu_dummy.rs` |

## Deep Source Reading Evidence

### `Telegram/SourceFiles/AGENTS.md`
- **Style Tokens**: asset:AGENTS.md
- **Target**: `AgentsDoc` in `crates/gpui_telegram_core/docs/AGENTS_doc.rs`

### `Telegram/SourceFiles/_other/packer.cpp`
- **Classes & Structs**: BIODeleter, V2Keys, uchar, uint32, int32
- **Core Methods & Signals**: QString::fromLatin1, Updates::ParseVerifiedManifest, QDateTime::currentSecsSinceEpoch, Updates::ChannelName, Updates::VerifyUpdate, Updates::MakeUpdateVersion
- **Target**: `Packer` in `crates/gpui_telegram_core/src/core/packer.rs`

### `Telegram/SourceFiles/_other/packer.h`
- **Style Tokens**: #include <QtCore/QCoreApplication>, #include <QtCore/QFileInfo>, #include <QtCore/QFile>, #include <QtCore/QDir>, #include <QtCore/QStringList>, #include <QtCore/QBuffer>
- **Target**: `Packer` in `crates/gpui_telegram_core/src/core/packer.rs`

### `Telegram/SourceFiles/_other/updater_linux.cpp`
- **Classes & Structs**: stat, tm, passwd
- **Core Methods & Signals**: do_mkdir, openLog, closeLog, writeLog, copyFile, remove_directory
- **Target**: `UpdaterLinux` in `crates/gpui_telegram_core/src/core/updater_linux.rs`

### `Telegram/SourceFiles/_other/zxcvbn_translit.cpp`
- **Classes & Structs**: Decoded
- **Core Methods & Signals**: Ui::TransliterateCyrillic, main, Fail
- **Target**: `ZxcvbnTranslit` in `crates/gpui_telegram_core/src/core/zxcvbn_translit.rs`

### `Telegram/SourceFiles/apiwrap.cpp`
- **Classes & Structs**: Callbacks, SharedCallback, PhotoFileLocationId, DocumentFileLocationId, UpdatedFileReferences, Flag, IndexAndMessageIds, MessageIdsByChannel, SendFlag, PageFlag, PendingFlag
- **Core Methods & Signals**: Editor::IsComposeBoxOpen, Core::App, QString::number, ApiWrap::ApiWrap, MTP::Sender, ApiWrap::~ApiWrap
- **Target**: `Apiwrap` in `crates/gpui_telegram_core/src/core/apiwrap.rs`

### `Telegram/SourceFiles/apiwrap.h`
- **Classes & Structs**: TaskQueue, HistoryItem, MessageGroupId, SendingAlbum, SendMediaType, FileLoadTo, ChatRestrictionsInfo, Session, RichPage, ReactionId, UpdatedFileReferences, WallPaper
- **Core Methods & Signals**: QString::number, details::ToString, ranges::accumulate, ToString, RequestKey, ApiWrap
- **Target**: `Apiwrap` in `crates/gpui_telegram_core/src/core/apiwrap.rs`

### `Telegram/SourceFiles/codegen/scheme/codegen_scheme.py`
- **Style Tokens**: asset:codegen_scheme.py
- **Target**: `CodegenSchemeSchemePyAsset` in `crates/gpui_telegram_core/assets/codegen_scheme_py.rs`

### `Telegram/SourceFiles/core/application.cpp`
- **Classes & Structs**: Application, State, WindowRaw, Command
- **Core Methods & Signals**: Clip::Finish, CrashReports::SetAnnotation, GL::CurrentANGLE, Application::Application, Integration::Create, Integration::Set
- **Target**: `Application` in `crates/gpui_telegram_core/src/core/application.rs`

### `Telegram/SourceFiles/core/application.h`
- **Classes & Structs**: History, BatterySaving, Integration, Databases, Controller, SavedWindows, System, EmojiKeywords, Domain, Account, Session, Instance
- **Core Methods & Signals**: run, hasActiveWindow, setActivePrimaryWindow, ensureSeparateWindowFor, closeWindow, windowActivated
- **Target**: `Application` in `crates/gpui_telegram_core/src/core/application.rs`

