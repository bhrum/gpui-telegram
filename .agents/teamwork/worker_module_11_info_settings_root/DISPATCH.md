# Dispatch: worker_module_11_info_settings_root (Slice 6 of 9)

## Mission
You are the implementation worker for Module 11 (`info & settings`) Slice 6: Info Settings & Root/Bot (42 upstream files, ~23 target files).
Your mission is to perform Step 11A (deep C++ reading & authoring design doc) and Step 11B (0-boilerplate genuine GPUI/Rust implementation).

## Scope & Sources
- Working Directory: `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_11_info_settings_root`
- Target Project Root: `/Users/bhrum/teamwork_projects/gpui_telegram`
- Authoritative Request: `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md`
- Upstream C++ Source Root: `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/`
- Subsystem Shard: `/Users/bhrum/teamwork_projects/gpui_telegram/docs/specs/ledger/shards/info.json`
- Upstream Scope: All remaining 42 files in `info.json`: `Telegram/SourceFiles/info/settings/`, `Telegram/SourceFiles/info/bot/`, root info files, and info `.style` files.
- Design Doc Output: `docs/specs/modules/module_11_info_settings_root.md`
- Target Rust/GPUI Scope: All target files in `crates/gpui_telegram_desktop/src/views/drawer/settings/`, `crates/gpui_telegram_desktop/src/views/drawer/bot/`, and root drawer styles/modules mapped in `info.json`.

## Mandatory Integrity Warning
DO NOT CHEAT. All implementations must be genuine. DO NOT hardcode test results, create dummy/facade implementations, or circumvent the intended task. A teamwork_auditor will independently verify your work. Integrity violations WILL be detected and your work WILL be rejected.

## Detailed Instructions
1. Read `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md`.
2. Inspect `docs/specs/ledger/shards/info.json` for all entries in `info/settings/`, `info/bot/`, root info files, and info `.style` files (exactly 42 upstream files).
3. Read the actual C++ source files in `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/info/`.
4. Author `docs/specs/modules/module_11_info_settings_root.md`:
   - Include 100% Upstream Source Reading Manifest table covering all 42 files with exact line ranges (`L<start>-L<end>`), extracted types/symbols, formulas/constants, target paths, and target symbols (Mandates #18 & #19).
5. Implement genuine Rust / GPUI code for your assigned targets:
   - Completely eradicate all boilerplate patterns A, B, C, D in your assigned files.
   - Enforce Mandate #20: keep single `write_to_file` calls <= 200 lines per call.
6. Verify your code with `cargo check --package gpui_telegram_desktop` or cargo test.
7. Run `python3 scripts/clean_build_garbage.py` after cargo commands (Mandate #11).
8. Author handoff report in `handoff.md` in your working directory and notify orchestrator via `send_message`.

## 2026-11-07T07:23:07Z
From: 0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345
Content: You are worker_module_11_info_settings_root for Module 11 Slice 6.
Your working directory is: /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_11_info_settings_root
Read your assignment at: /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_11_info_settings_root/DISPATCH.md
Read the authoritative request at: /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md

MANDATORY INTEGRITY WARNING:
DO NOT CHEAT. All implementations must be genuine. DO NOT hardcode test results, create dummy/facade implementations, or circumvent the intended task. A teamwork_auditor will independently verify your work. Integrity violations WILL be detected and your work WILL be rejected.

Scope: 42 upstream files in Telegram/SourceFiles/info/settings/, info/bot/, root info files, and info .style files.
Deliverables:
1. docs/specs/modules/module_11_info_settings_root.md with 100% reading manifest.
2. 0-boilerplate genuine implementation in your assigned targets (Mandates #15, #18, #19, #20: writes <= 200 lines).
3. Clean build garbage: run `python3 scripts/clean_build_garbage.py`.
4. Handoff report in your working directory and message to orchestrator.

## 2026-11-07T07:50:51Z
From: 0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345
Content: **Context**: Module 11 Slice 6 Completion
**Content**: Handoff report received and verified. Design doc `docs/specs/modules/module_11_info_settings_root.md` (49/49 manifest) and genuine implementation across all 30 target files with 0 boilerplate and 32 unit tests confirmed.
**Action**: None. Stand down confirmed; orchestrator will aggregate during Step 11C and Step 11D.
