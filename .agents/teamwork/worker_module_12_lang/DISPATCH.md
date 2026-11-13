# Task Assignment — worker_module_12_lang

## Working Directory
`/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_12_lang`

## Scope: Shard `lang.json` (25 upstream files, 14 targets)
Assigned Upstream Sources in `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/lang/`:
- `lang/lang_cloud_manager.{h,cpp}`
- `lang/lang_file_parser.{h,cpp}`
- `lang/lang_hardcoded.h`
- `lang/lang_instance.{h,cpp}`
- `lang/lang_keys.{h,cpp}`
- `lang/lang_numbers_animation.{h,cpp}`
- `lang/lang_pch.h`
- `lang/lang_tag.{h,cpp}`
- `lang/lang_text_entity.{h,cpp}`
- `lang/lang_translator.{h,cpp}`
- `lang/lang_values.h`
- `lang/translate_mtproto_provider.{h,cpp}`
- `lang/translate_provider.{h,cpp}`
- `lang/translate_url_provider.{h,cpp}`

Target Files (14 files):
- `crates/gpui_telegram_desktop/src/i18n/lang_cloud_manager.rs`
- `crates/gpui_telegram_desktop/src/i18n/lang_file_parser.rs`
- `crates/gpui_telegram_desktop/src/i18n/lang_hardcoded.rs`
- `crates/gpui_telegram_desktop/src/i18n/lang_instance.rs`
- `crates/gpui_telegram_desktop/src/i18n/lang_keys.rs`
- `crates/gpui_telegram_desktop/src/i18n/lang_numbers_animation.rs`
- `crates/gpui_telegram_desktop/src/i18n/lang_pch.rs`
- `crates/gpui_telegram_desktop/src/i18n/lang_tag.rs`
- `crates/gpui_telegram_desktop/src/i18n/lang_text_entity.rs`
- `crates/gpui_telegram_desktop/src/i18n/lang_translator.rs`
- `crates/gpui_telegram_desktop/src/i18n/lang_values.rs`
- `crates/gpui_telegram_desktop/src/i18n/translate_mtproto_provider.rs`
- `crates/gpui_telegram_desktop/src/i18n/translate_provider.rs`
- `crates/gpui_telegram_desktop/src/i18n/translate_url_provider.rs`

## Required Actions
1. **Step 12A — Author Design Spec `docs/specs/modules/module_12_lang.md`**:
   - Enumerate ALL 25 upstream files with 4-Layer Anti-Bluffing Proof (Mandate #18).
   - Zero omission: all 25 files must appear in the manifest table.
2. **Step 12B — Replace all Boilerplate Stubs**:
   - Inspect all 14 target files. Eradicate all Patterns A, B, C, D stubs.
   - Implement genuine i18n string formatting, cloud language pack difference sync, plural rules, translation providers (MTProto & URL).
   - Keep write calls <= 200 lines (Mandate #20).
3. **Workspace Check & Clean**:
   - Verify `cargo check --workspace` passes cleanly.
   - Run `python3 scripts/clean_build_garbage.py`.
4. Deliver `progress.md` and `handoff.md`. Send completion message back.

## 2026-11-07T09:40:37Z
Received dispatch message:
You are worker_module_12_lang.
Working directory: /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_12_lang
Target project root: /Users/bhrum/teamwork_projects/gpui_telegram
Parent conversation ID: 0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345
TASK OBJECTIVE:
1. Step 12A: Author docs/specs/modules/module_12_lang.md documenting 100% of all 25 upstream files in lang.json with 4-Layer Anti-Bluffing Proof (Mandate #18). Zero omissions.
2. Step 12B: Implement genuine i18n logic, translation providers, and plural rules across all 14 assigned target files, eliminating all 4 boilerplate patterns (A, B, C, D). No shallow placeholder structs without Render.
3. Keep single write_to_file calls <= 200 lines (Mandate #20).
4. Run cargo check --workspace and python3 scripts/clean_build_garbage.py.
5. Deliver progress.md and handoff.md, then send completion message to parent.
