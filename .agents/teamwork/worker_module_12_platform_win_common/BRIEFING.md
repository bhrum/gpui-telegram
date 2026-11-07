# BRIEFING — 2026-11-07T18:05:00Z

## Mission
Complete Module 12 (platform.json Part 3: Cross-platform common + Windows integration). Author `docs/specs/modules/module_12_platform_win_common.md` with 4-layer anti-bluffing proof covering all assigned upstream files. Implement genuine cross-platform & Windows integration across all 29 target files, eliminating Patterns A, B, C, D boilerplate stubs, compile cleanly, and run garbage cleanup.

## 🔒 My Identity
- Archetype: implementer, qa, specialist
- Roles: implementer, qa, specialist
- Working directory: /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_12_platform_win_common
- Original parent: 0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345
- Milestone: Module 12 (platform.json Part 3: platform common + win)

## 🔒 Key Constraints
- Mandate #11: Single workspace target/, clean_build_garbage.py after builds/tests.
- Mandate #15: 0 fake boilerplate stubs across Patterns A, B, C, D. Genuine logic, real state machines, GPUI Render implementations where appropriate.
- Mandate #18: 4-Layer Anti-Bluffing Proof of Genuine .cpp source reading (exact line citations L<start>-L<end>, real symbols, constants, formulas).
- Mandate #19: 100% upstream source coverage (zero omissions) in design doc.
- Mandate #20: Keep single write_to_file calls <= 200 lines per call.

## Current Parent
- Conversation ID: 0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345
- Updated: 2026-11-07T18:05:00Z

## Task Summary
- **What to build**:
  1. Design doc `docs/specs/modules/module_12_platform_win_common.md` (47 upstream files in platform.json Part 3).
  2. Implement 29 target Rust files in `crates/gpui_telegram_desktop/src/platform/` and `src/platform/win/`.
  3. Verify `cargo check --workspace` and `python3 scripts/clean_build_garbage.py`.
- **Success criteria**:
  - 100% upstream file coverage in design doc with 4-Layer Anti-Bluffing proof.
  - 0 boilerplate files in the 29 assigned targets.
  - `cargo check --workspace` passes cleanly.
  - `clean_build_garbage.py` passes cleanly.
  - All 6 Verification Gates in `verify_ledger_closure.py` pass cleanly with exit code 0.

## Change Tracker
- **Files modified**:
  - `docs/specs/modules/module_12_platform_win_common.md`: Authoritative design specification (47/47 files, 100% manifest).
  - 29 Target files in `crates/gpui_telegram_desktop/src/platform/`:
    - `platform_integration.rs`: PlatformIntegration with power/sleep and dark mode hooks.
    - `platform_launcher.rs`: PlatformLauncher with CLI argument parser and launch URLs.
    - `platform_main_window.rs`: PlatformMainWindow with frame geometry, shadow and state machine.
    - `platform_notifications_manager.rs`: PlatformNotificationsManager with toast suppression and sound checks.
    - `platform_overlay_widget.rs`: PlatformOverlayWidget with upstream color blend formula and opacity states.
    - `platform_specific.rs`: PlatformSpecific with screenshot protection and permission queries.
    - `platform_text_recognition.rs`: PlatformTextRecognition with bounding box extraction.
    - `platform_translate_provider.rs`: PlatformTranslateProvider with language routing.
    - `platform_tray.rs`: PlatformTray with unread count and monochrome settings.
    - `platform_webauthn.rs`: PlatformWebauthn with FIDO2 passkey credentials.
    - `platform_window_title.rs`: PlatformWindowTitle with OS-dependent title alignment and metrics.
    - `win/current_geo_location_win.rs`: CurrentGeoLocationWinWin WinRT geolocation.
    - `win/file_utilities_win.rs`: FileUtilitiesWinWin file shell verbs and path normalization.
    - `win/integration_win.rs`: IntegrationWinWin Win32 event filter and custom jump lists.
    - `win/launcher_win.rs`: LauncherWinWin updater execution and DPI v2 awareness.
    - `win/main_window_win.rs`: MainWindowWinWin Win32 WM_NCHITTEST router and DWM frame integration.
    - `win/notifications_manager_win.rs`: NotificationsManagerWinWin Action Center Toast XML and quick reply.
    - `win/overlay_widget_win.rs`: OverlayWidgetWinWin overlay helper hooks.
    - `win/specific_win.rs`: SpecificWinWin window screenshot protection and URI scheme registration.
    - `win/text_recognition_win.rs`: TextRecognitionWinWin WinRT Media OcrEngine bridge.
    - `win/translate_provider_win.rs`: TranslateProviderWinWin native Windows translation provider.
    - `win/tray_win.rs`: TrayWinWin Shell_NotifyIconW notification area integration.
    - `win/wallet_protection_win.rs`: WalletProtectionWinWin Windows Hello KeyCredentialManager biometric signing.
    - `win/webauthn_win.rs`: WebauthnWinWin webauthn.dll dynamic resolution.
    - `win/windows_app_user_model_id.rs`: WindowsAppUserModelIdWin AUMID hash generation and shortcut validation.
    - `win/windows_autostart_task.rs`: WindowsAutostartTaskWin WinRT startup task and registry fallback.
    - `win/windows_dlls.rs`: WindowsDllsWin safe DLL loading guards (LOAD_LIBRARY_SEARCH_SYSTEM32).
    - `win/windows_taskbar_buttons.rs`: WindowsTaskbarButtonsWin ITaskbarList3 thumbnail playback buttons.
    - `win/windows_toast_activator.rs`: WindowsToastActivatorWin COM INotificationActivationCallback.
- **Build status**: `cargo check --workspace` PASS (exit code 0), `verify_ledger_closure.py` PASS (exit code 0).
- **Pending issues**: None.

## Quality Status
- **Build/test result**: All 6 verification gates pass cleanly (exit code 0).
- **Lint status**: 0 compile errors in workspace, clean build hygiene.
- **Tests added/modified**: Unit tests implemented in all 29 target files.

## Loaded Skills
- None specified by orchestrator
