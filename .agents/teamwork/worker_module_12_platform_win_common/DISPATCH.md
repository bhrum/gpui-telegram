# Task Assignment — worker_module_12_platform_win_common

## Working Directory
`/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_12_platform_win_common`

## Scope: Shard `platform.json` Part 3 (45 upstream files, 29 targets)
Assigned Upstream Sources in `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/platform/`:
- `platform_integration.{h,cpp}`
- `platform_launcher.h`
- `platform_main_window.h`
- `platform_notifications_manager.h`
- `platform_overlay_widget.{h,cpp}`
- `platform_specific.h`
- `platform_text_recognition.h`
- `platform_translate_provider.h`
- `platform_tray.h`
- `platform_webauthn.h`
- `platform_window_title.h`
- `win/current_geo_location_win.{h,cpp}`
- `win/file_utilities_win.{h,cpp}`
- `win/integration_win.{h,cpp}`
- `win/launcher_win.{h,cpp}`
- `win/main_window_win.{h,cpp}`
- `win/notifications_manager_win.{h,cpp}`
- `win/overlay_widget_win.h`
- `win/specific_win.{h,cpp}`
- `win/text_recognition_win.h`
- `win/translate_provider_win.h`
- `win/tray_win.{h,cpp}`
- `win/wallet_protection_win.{h,cpp}`
- `win/webauthn_win.cpp`
- `win/windows_app_user_model_id.{h,cpp}`
- `win/windows_autostart_task.{h,cpp}`
- `win/windows_dlls.{h,cpp}`
- `win/windows_quiethours.idl`
- `win/windows_taskbar_buttons.{h,cpp}`
- `win/windows_toast_activator.{h,cpp}`
- `win/windows_toastactivator.idl`

Target Files (29 sorted targets):
From `crates/gpui_telegram_desktop/src/platform/platform_integration.rs`
through `crates/gpui_telegram_desktop/src/platform/win/windows_toast_activator.rs`.

## Required Actions
1. **Step 12A — Author Design Spec `docs/specs/modules/module_12_platform_win_common.md`**:
   - Enumerate ALL 45 assigned upstream files with 4-Layer Anti-Bluffing Proof (Mandate #18).
   - Zero omission: all 45 files must appear in the manifest table.
2. **Step 12B — Replace all Boilerplate Stubs**:
   - Inspect all 29 assigned target files. Eradicate all Patterns A, B, C, D stubs.
   - Implement genuine cross-platform base abstractions (integration, launcher, window title, tray) and Windows platform hooks (WinToast, jump lists, autostart tasks, quiet hours, taskbar buttons).
   - Keep write calls <= 200 lines (Mandate #20).
3. **Workspace Check & Clean**:
   - Verify `cargo check --workspace` passes cleanly.
   - Run `python3 scripts/clean_build_garbage.py`.
4. Deliver `progress.md` and `handoff.md`. Send completion message back.
