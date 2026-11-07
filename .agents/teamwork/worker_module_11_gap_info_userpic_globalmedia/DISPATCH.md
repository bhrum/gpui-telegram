# Dispatch: worker_module_11_gap_info_userpic_globalmedia

## Mission
You are the Gap-Closure Worker for Module 11 Info Userpic & Global Media.
Your mission is to replace all 12 boilerplate targets with genuine domain models, state machines, and GPUI views, and author the specification covering all 24 upstream files.

## Target Project Root
`/Users/bhrum/teamwork_projects/gpui_telegram`

## Mandatory Inputs (Read before starting)
- `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md`
- Upstream source tree: `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/info/`
- Ledger shard: `/Users/bhrum/teamwork_projects/gpui_telegram/docs/specs/ledger/shards/info.json`

## Upstream Files (24 files) & 12 Target Files:
1. `Telegram/SourceFiles/info/global_media/info_global_media_inner_widget.{h,cpp}` -> `crates/gpui_telegram_desktop/src/views/drawer/global_media/info_global_media_inner_widget.rs`
2. `Telegram/SourceFiles/info/global_media/info_global_media_provider.{h,cpp}` -> `crates/gpui_telegram_desktop/src/views/drawer/global_media/info_global_media_provider.rs`
3. `Telegram/SourceFiles/info/global_media/info_global_media_widget.{h,cpp}` -> `crates/gpui_telegram_desktop/src/views/drawer/global_media/info_global_media_widget.rs`
4. `Telegram/SourceFiles/info/userpic/info_userpic_bubble_wrap.{h,cpp}` -> `crates/gpui_telegram_desktop/src/views/drawer/userpic/info_userpic_bubble_wrap.rs`
5. `Telegram/SourceFiles/info/userpic/info_userpic_color_circle_button.{h,cpp}` -> `crates/gpui_telegram_desktop/src/views/drawer/userpic/info_userpic_color_circle_button.rs`
6. `Telegram/SourceFiles/info/userpic/info_userpic_colors_editor.{h,cpp}` -> `crates/gpui_telegram_desktop/src/views/drawer/userpic/info_userpic_colors_editor.rs`
7. `Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder.{h,cpp}` -> `crates/gpui_telegram_desktop/src/views/drawer/userpic/info_userpic_emoji_builder.rs`
8. `Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder_common.{h,cpp}` -> `crates/gpui_telegram_desktop/src/views/drawer/userpic/info_userpic_emoji_builder_common.rs`
9. `Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder_layer.{h,cpp}` -> `crates/gpui_telegram_desktop/src/views/drawer/userpic/info_userpic_emoji_builder_layer.rs`
10. `Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder_menu_item.{h,cpp}` -> `crates/gpui_telegram_desktop/src/views/drawer/userpic/info_userpic_emoji_builder_menu_item.rs`
11. `Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder_preview.{h,cpp}` -> `crates/gpui_telegram_desktop/src/views/drawer/userpic/info_userpic_emoji_builder_preview.rs`
12. `Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder_widget.{h,cpp}` -> `crates/gpui_telegram_desktop/src/views/drawer/userpic/info_userpic_emoji_builder_widget.rs`

## Tasks:
1. Deeply inspect the 24 upstream C++ files.
2. Author `docs/specs/modules/module_11_info_userpic_global_media.md` containing the 100% Upstream Source Reading Manifest covering all 24 files with exact line ranges (`L<start>-L<end>`), constants, formulas, and target symbols (Mandates #18 & #19).
3. Replace all 12 target files with genuine, non-stub GPUI code (0 Pattern A, B, C, D). Ensure each target includes upstream citation comments (`// Upstream: Telegram/SourceFiles/info/...`).
4. Keep single `write_to_file` calls <= 200 lines per call (Mandate #20).
5. Run `cargo check --package gpui_telegram_desktop` to verify clean compilation.
6. Run `python3 scripts/clean_build_garbage.py` after cargo commands (Mandate #11).
7. Deliver `handoff.md` and message orchestrator.

## Mandatory Integrity Warning
DO NOT CHEAT. All implementations must be genuine. DO NOT hardcode test results, create dummy/facade implementations, or circumvent the intended task. A teamwork_auditor will independently verify your work. Integrity violations WILL be detected and your work WILL be rejected.
