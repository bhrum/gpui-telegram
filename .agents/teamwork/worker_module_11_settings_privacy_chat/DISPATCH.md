# Dispatch: worker_module_11_settings_privacy_chat (Slice 8 of 9)

## Mission
You are the implementation worker for Module 11 (`info & settings`) Slice 8: Settings Privacy, Security & Chat (40 upstream files, ~21 target files).
Your mission is to perform Step 11A (deep C++ reading & authoring design doc) and Step 11B (0-boilerplate genuine GPUI/Rust implementation).

## Scope & Sources
- Working Directory: `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_11_settings_privacy_chat`
- Target Project Root: `/Users/bhrum/teamwork_projects/gpui_telegram`
- Authoritative Request: `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md`
- Upstream C++ Source Root: `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/`
- Subsystem Shard: `/Users/bhrum/teamwork_projects/gpui_telegram/docs/specs/ledger/shards/settings.json`
- Upstream Scope: 40 files in `settings.json` covering: `settings_privacy_security.*`, `settings_chat.*`, `settings_notifications.*`, and related privacy/chat settings files.
- Design Doc Output: `docs/specs/modules/module_11_settings_privacy_chat.md`
- Target Rust/GPUI Scope: All target files in `crates/gpui_telegram_desktop/src/views/settings/privacy/`, `crates/gpui_telegram_desktop/src/views/settings/chat/`, and `crates/gpui_telegram_desktop/src/views/settings/notifications/` mapped in `settings.json`.

## Mandatory Integrity Warning
DO NOT CHEAT. All implementations must be genuine. DO NOT hardcode test results, create dummy/facade implementations, or circumvent the intended task. A teamwork_auditor will independently verify your work. Integrity violations WILL be detected and your work WILL be rejected.

## Detailed Instructions
1. Read `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md`.
2. Inspect `docs/specs/ledger/shards/settings.json` for all entries in your assigned privacy, security, and chat scope (exactly 40 upstream files).
3. Read the actual C++ source files in `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/settings/`.
4. Author `docs/specs/modules/module_11_settings_privacy_chat.md`:
   - Include 100% Upstream Source Reading Manifest table covering all 40 files with exact line ranges (`L<start>-L<end>`), extracted types/symbols, formulas/constants, target paths, and target symbols (Mandates #18 & #19).
5. Implement genuine Rust / GPUI code for your assigned targets:
   - Completely eradicate all boilerplate patterns A, B, C, D in your assigned files.
   - Enforce Mandate #20: keep single `write_to_file` calls <= 200 lines per call.
6. Verify your code with `cargo check --package gpui_telegram_desktop` or cargo test.
7. Run `python3 scripts/clean_build_garbage.py` after cargo commands (Mandate #11).
8. Author handoff report in `handoff.md` in your working directory and notify orchestrator via `send_message`.


## 2026-11-07T07:23:12Z
[Message] timestamp=2026-11-07T07:23:12Z sender=0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345 priority=MESSAGE_PRIORITY_HIGH content=You are worker_module_11_settings_privacy_chat for Module 11 Slice 8.
Your working directory is: /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_11_settings_privacy_chat
Read your assignment at: /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_11_settings_privacy_chat/DISPATCH.md
Read the authoritative request at: /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md

MANDATORY INTEGRITY WARNING:
DO NOT CHEAT. All implementations must be genuine. DO NOT hardcode test results, create dummy/facade implementations, or circumvent the intended task. A teamwork_auditor will independently verify your work. Integrity violations WILL be detected and your work WILL be rejected.

Scope: 40 upstream files in settings.json covering settings_privacy_security.*, settings_chat.*, settings_notifications.*.
Deliverables:
1. docs/specs/modules/module_11_settings_privacy_chat.md with 100% reading manifest.
2. 0-boilerplate genuine implementation in your assigned targets (Mandates #15, #18, #19, #20: writes <= 200 lines).
3. Clean build garbage: run `python3 scripts/clean_build_garbage.py`.
4. Handoff report in your working directory and message to orchestrator.
