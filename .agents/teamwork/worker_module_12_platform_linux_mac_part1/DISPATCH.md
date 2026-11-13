# Task Assignment — worker_module_12_platform_linux_mac_part1

## Working Directory
`/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_12_platform_linux_mac_part1`

## Scope: Shard `platform.json` Part 1 (43 upstream files, 26 targets)
Assigned Upstream Sources in `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/platform/`:
- `linux/current_geo_location_linux.{h,cpp}`
- `linux/file_utilities_linux.{h,cpp}`
- `linux/integration_linux.{h,cpp}`
- `linux/launcher_linux.{h,cpp}`
- `linux/main_window_linux.{h,cpp}`
- `linux/notifications_manager_linux.{h,cpp}`
- `linux/org.freedesktop.Notifications.xml`
- `linux/org.freedesktop.login1.Manager.xml`
- `linux/org.freedesktop.portal.Flatpak.xml`
- `linux/overlay_widget_linux.h`
- `linux/specific_linux.{h,cpp}`
- `linux/text_recognition_linux.h`
- `linux/translate_provider_linux.{h,cpp}`
- `linux/tray_linux.{h,cpp}`
- `linux/update_install_linux.{h,cpp}`
- `linux/webauthn_linux.cpp`
- `mac/current_geo_location_mac.{h,mm}`
- `mac/file_bookmark_mac.{h,mm}`
- `mac/file_utilities_mac.{h,mm}`
- `mac/global_menu_mac.{h,mm}`
- `mac/integration_mac.{h,mm}`
- `mac/launcher_mac.{h,mm}`
- `mac/mac_iconv_helper.c`
- `mac/main_window_mac.{h,mm}`

Target Files (26 sorted targets):
From assets/dbus xml up through `src/platform/mac/main_window_mac.rs`.

## Required Actions
1. **Step 12A — Author Design Spec `docs/specs/modules/module_12_platform_linux_mac_part1.md`**:
   - Enumerate ALL 43 assigned upstream files with 4-Layer Anti-Bluffing Proof (Mandate #18).
   - Zero omission: all 43 files must appear in the manifest table.
2. **Step 12B — Replace all Boilerplate Stubs**:
   - Inspect all 26 assigned target files. Eradicate all Patterns A, B, C, D stubs.
   - Implement genuine Linux & macOS native platform integration (D-Bus notifications, file bookmarks, global menu bar items, macOS launcher, main window shell hooks).
   - Keep write calls <= 200 lines (Mandate #20).
3. **Workspace Check & Clean**:
   - Verify `cargo check --workspace` passes cleanly.
   - Run `python3 scripts/clean_build_garbage.py`.
4. Deliver `progress.md` and `handoff.md`. Send completion message back.


## 2026-11-07T09:40:46Z
Received dispatch message:
You are worker_module_12_platform_linux_mac_part1.
Scope: Shard platform.json Part 1 (43 upstream files, 26 targets).
Task Objective:
1. Step 12A: Author docs/specs/modules/module_12_platform_linux_mac_part1.md documenting 100% of all 43 assigned upstream files in platform.json Part 1 with 4-Layer Anti-Bluffing Proof (Mandate #18). Zero omissions.
2. Step 12B: Implement genuine Linux & macOS platform integration across all assigned target files, eliminating all 4 boilerplate patterns (A, B, C, D). No shallow placeholder structs without Render.
3. Keep single write_to_file calls <= 200 lines (Mandate #20).
4. Run cargo check --workspace and python3 scripts/clean_build_garbage.py.
5. Deliver progress.md and handoff.md, then send completion message to parent.

## 2026-11-07T10:11:39Z
Received check-in message from parent:
**Context**: Step 12B Finalization & Handoff
**Content**: Slices 1, 2, 4, and 5 have completed and verified 0 boilerplate stubs across their targets. Please report your current status on test verification, cargo check, and handoff.md generation.
**Action**: Wrap up tests, run clean_build_garbage.py, write handoff.md, and deliver completion report.
