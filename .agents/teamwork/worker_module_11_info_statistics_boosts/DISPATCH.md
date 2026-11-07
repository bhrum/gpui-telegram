# Dispatch: worker_module_11_info_statistics_boosts (Slice 4 of 9)

## Mission
You are the implementation worker for Module 11 (`info & settings`) Slice 4: Info Statistics & Boosts (42 upstream files, ~22 target files).
Your mission is to perform Step 11A (deep C++ reading & authoring design doc) and Step 11B (0-boilerplate genuine GPUI/Rust implementation).

## Scope & Sources
- Working Directory: `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_11_info_statistics_boosts`
- Target Project Root: `/Users/bhrum/teamwork_projects/gpui_telegram`
- Authoritative Request: `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md`
- Upstream C++ Source Root: `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/`
- Subsystem Shard: `/Users/bhrum/teamwork_projects/gpui_telegram/docs/specs/ledger/shards/info.json`
- Upstream Scope: All 42 files under `Telegram/SourceFiles/info/statistics/` and `Telegram/SourceFiles/info/boosts/`.
- Design Doc Output: `docs/specs/modules/module_11_info_statistics_boosts.md`
- Target Rust/GPUI Scope: All target files in `crates/gpui_telegram_desktop/src/views/drawer/statistics/` and `crates/gpui_telegram_desktop/src/views/drawer/boosts/` mapped in `info.json`.

## Mandatory Integrity Warning
DO NOT CHEAT. All implementations must be genuine. DO NOT hardcode test results, create dummy/facade implementations, or circumvent the intended task. A teamwork_auditor will independently verify your work. Integrity violations WILL be detected and your work WILL be rejected.

## Detailed Instructions
1. Read `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md`.
2. Inspect `docs/specs/ledger/shards/info.json` for all entries in `info/statistics/` and `info/boosts/` (exactly 42 upstream files).
3. Read the actual C++ source files in `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/info/statistics/` and `info/boosts/`.
4. Author `docs/specs/modules/module_11_info_statistics_boosts.md`:
   - Include 100% Upstream Source Reading Manifest table covering all 42 files with exact line ranges (`L<start>-L<end>`), extracted types/symbols, formulas/constants, target paths, and target symbols (Mandates #18 & #19).
5. Implement genuine Rust / GPUI code for your assigned targets:
   - Completely eradicate all boilerplate patterns A, B, C, D in your assigned files.
   - Enforce Mandate #20: keep single `write_to_file` calls <= 200 lines per call.
6. Verify your code with `cargo check --package gpui_telegram_desktop` or cargo test.
7. Run `python3 scripts/clean_build_garbage.py` after cargo commands (Mandate #11).
8. Author handoff report in `handoff.md` in your working directory and notify orchestrator via `send_message`.
