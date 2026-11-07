# Dispatch: worker_module_11_settings_core_main (Slice 7 of 9)

## Mission
You are the implementation worker for Module 11 (`info & settings`) Slice 7: Settings Core & Main (40 upstream files, ~21 target files).
Your mission is to perform Step 11A (deep C++ reading & authoring design doc) and Step 11B (0-boilerplate genuine GPUI/Rust implementation).

## Scope & Sources
- Working Directory: `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_11_settings_core_main`
- Target Project Root: `/Users/bhrum/teamwork_projects/gpui_telegram`
- Authoritative Request: `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md`
- Upstream C++ Source Root: `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/`
- Subsystem Shard: `/Users/bhrum/teamwork_projects/gpui_telegram/docs/specs/ledger/shards/settings.json`
- Upstream Scope: 40 files in `settings.json` covering: `settings_account.*`, `settings_main.*`, `settings_common.*`, `settings_type.*`, and root settings files.
- Design Doc Output: `docs/specs/modules/module_11_settings_core_main.md`
- Target Rust/GPUI Scope: All target files in `crates/gpui_telegram_desktop/src/views/settings/core/` and related root settings files mapped in `settings.json`.

## Mandatory Integrity Warning
DO NOT CHEAT. All implementations must be genuine. DO NOT hardcode test results, create dummy/facade implementations, or circumvent the intended task. A teamwork_auditor will independently verify your work. Integrity violations WILL be detected and your work WILL be rejected.

## Detailed Instructions
1. Read `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md`.
2. Inspect `docs/specs/ledger/shards/settings.json` for all entries in your assigned core/main scope (exactly 40 upstream files).
3. Read the actual C++ source files in `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/settings/`.
4. Author `docs/specs/modules/module_11_settings_core_main.md`:
   - Include 100% Upstream Source Reading Manifest table covering all 40 files with exact line ranges (`L<start>-L<end>`), extracted types/symbols, formulas/constants, target paths, and target symbols (Mandates #18 & #19).
5. Implement genuine Rust / GPUI code for your assigned targets:
   - Completely eradicate all boilerplate patterns A, B, C, D in your assigned files.
   - Enforce Mandate #20: keep single `write_to_file` calls <= 200 lines per call.
6. Verify your code with `cargo check --package gpui_telegram_desktop` or cargo test.
7. Run `python3 scripts/clean_build_garbage.py` after cargo commands (Mandate #11).
8. Author handoff report in `handoff.md` in your working directory and notify orchestrator via `send_message`.

## 2026-11-07T07:54:55Z
From: 0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345 (Orchestrator)
**Context**: Module 11 Slice 7 Completion
**Content**: Handoff report received and verified. Design doc `docs/specs/modules/module_11_settings_core_main.md` (40/40 manifest) and genuine implementation across all 21 target files with 0 boilerplate confirmed. 2 Subsystem test suites (`settings_core_subsystem_tests.rs` and `settings_sections_subsystem_tests.rs`) noted.
**Action**: None. Stand down confirmed; orchestrator will aggregate during Step 11C and Step 11D.
