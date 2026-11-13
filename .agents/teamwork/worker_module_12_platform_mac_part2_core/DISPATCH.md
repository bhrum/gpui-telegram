# Task Assignment — worker_module_12_platform_mac_part2_core

## Working Directory
`/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_12_platform_mac_part2_core`

## Scope: Shard `platform.json` Part 2 (44 upstream files, 24 targets)
Assigned Upstream Sources in `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/platform/`:
- `mac/native_event_mac.{h,mm}`
- `mac/notifications_manager_mac.{h,mm}`
- `mac/notifications_manager_mac_un.{h,mm}`
- `mac/overlay_widget_mac.{h,mm}`
- `mac/specific_mac.{h,mm}`
- `mac/specific_mac_p.{h,mm}`
- `mac/text_recognition_mac.{h,mm}`
- `mac/touchbar/items/mac_formatter_item.{h,mm}`
- `mac/touchbar/items/mac_pinned_chats_item.{h,mm}`
- `mac/touchbar/items/mac_scrubber_item.{h,mm}`
- `mac/touchbar/mac_touchbar_audio.{h,mm}`
- `mac/touchbar/mac_touchbar_common.{h,mm}`
- `mac/touchbar/mac_touchbar_controls.{h,mm}`
- `mac/touchbar/mac_touchbar_main.{h,mm}`
- `mac/touchbar/mac_touchbar_manager.{h,mm}`
- `mac/touchbar/mac_touchbar_media_view.{h,mm}`
- `mac/translate_provider_mac.{h,mm}`
- `mac/tray_mac.{h,mm}`
- `mac/wallet_protection_mac.{h,mm}`
- `mac/webauthn_mac.mm`
- `mac/window_title_mac.mm`
- `platform_current_geo_location.h`
- `platform_file_bookmark.h`
- `platform_file_utilities.{h,cpp}`

Target Files (24 sorted targets):
From `crates/gpui_telegram_desktop/src/platform/mac/native_event_mac.rs`
through `crates/gpui_telegram_desktop/src/platform/platform_file_utilities.rs`.

## Required Actions
1. **Step 12A — Author Design Spec `docs/specs/modules/module_12_platform_mac_part2_core.md`**:
   - Enumerate ALL 44 assigned upstream files with 4-Layer Anti-Bluffing Proof (Mandate #18).
   - Zero omission: all 44 files must appear in the manifest table.
2. **Step 12B — Replace all Boilerplate Stubs**:
   - Inspect all 24 assigned target files. Eradicate all Patterns A, B, C, D stubs.
   - Implement genuine macOS touchbar items (formatter, pinned chats, audio scrubber, media view controls), native notification center integration, tray menu, and platform file utilities.
   - Keep write calls <= 200 lines (Mandate #20).
3. **Workspace Check & Clean**:
   - Verify `cargo check --workspace` passes cleanly.
   - Run `python3 scripts/clean_build_garbage.py`.
4. Deliver `progress.md` and `handoff.md`. Send completion message back.


## 2026-11-07T09:40:54Z
Received system task message from 0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345:
Module 12 Platform Mac Part 2 Core.
Scope: 44 upstream files in platform.json Part 2, 24 target files.
Step 12A: Author docs/specs/modules/module_12_platform_mac_part2_core.md.
Step 12B: Implement genuine macOS touchbar items, native notification center, platform utilities across all 24 assigned target files.
Keep single write_to_file calls <= 200 lines.
Run cargo check --workspace and python3 scripts/clean_build_garbage.py.
Deliver progress.md and handoff.md, send completion message.
