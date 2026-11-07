# Dispatch: worker_module_11_gap_settings_business_headers

## Mission
You are the Gap-Closure Worker for Module 11 Settings Business & Header Cleanup.
Your mission is to replace all 11 boilerplate targets with genuine domain models, state machines, and GPUI views, author the specification covering all 22 upstream files, and strip residual Pattern C header comments from 14 files.

## Target Project Root
`/Users/bhrum/teamwork_projects/gpui_telegram`

## Mandatory Inputs (Read before starting)
- `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md`
- Upstream source tree: `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/settings/`
- Ledger shard: `/Users/bhrum/teamwork_projects/gpui_telegram/docs/specs/ledger/shards/settings.json`

## Upstream Files (22 files) & 11 Target Files:
1. `Telegram/SourceFiles/settings/business/settings_away_message.{h,cpp}` -> `crates/gpui_telegram_desktop/src/views/settings/business/settings_away_message.rs`
2. `Telegram/SourceFiles/settings/business/settings_chat_intro.{h,cpp}` -> `crates/gpui_telegram_desktop/src/views/settings/business/settings_chat_intro.rs`
3. `Telegram/SourceFiles/settings/business/settings_chat_links.{h,cpp}` -> `crates/gpui_telegram_desktop/src/views/settings/business/settings_chat_links.rs`
4. `Telegram/SourceFiles/settings/business/settings_chatbots.{h,cpp}` -> `crates/gpui_telegram_desktop/src/views/settings/business/settings_chatbots.rs`
5. `Telegram/SourceFiles/settings/business/settings_greeting.{h,cpp}` -> `crates/gpui_telegram_desktop/src/views/settings/business/settings_greeting.rs`
6. `Telegram/SourceFiles/settings/business/settings_location.{h,cpp}` -> `crates/gpui_telegram_desktop/src/views/settings/business/settings_location.rs`
7. `Telegram/SourceFiles/settings/business/settings_quick_replies.{h,cpp}` -> `crates/gpui_telegram_desktop/src/views/settings/business/settings_quick_replies.rs`
8. `Telegram/SourceFiles/settings/business/settings_recipients_helper.{h,cpp}` -> `crates/gpui_telegram_desktop/src/views/settings/business/settings_recipients_helper.rs`
9. `Telegram/SourceFiles/settings/business/settings_shortcut_messages.{h,cpp}` -> `crates/gpui_telegram_desktop/src/views/settings/business/settings_shortcut_messages.rs`
10. `Telegram/SourceFiles/settings/business/settings_working_hours.{h,cpp}` -> `crates/gpui_telegram_desktop/src/views/settings/business/settings_working_hours.rs`
11. `Telegram/SourceFiles/settings/sections/settings_global_ttl.{h,cpp}` -> `crates/gpui_telegram_desktop/src/views/settings/sections/settings_global_ttl.rs`

## 14 Files with Residual Headers to Strip:
Remove `//! Materialized Rust GPUI Component` header lines and ensure genuine upstream citations from:
- `crates/gpui_telegram_desktop/src/theme/settings/settings_active_sessions_style.rs`
- `crates/gpui_telegram_desktop/src/theme/settings/settings_local_storage_style.rs`
- `crates/gpui_telegram_desktop/src/theme/settings/settings_premium_style.rs`
- `crates/gpui_telegram_desktop/src/theme/settings/settings_style.rs`
- `crates/gpui_telegram_desktop/src/views/settings/detailed_settings_button.rs`
- `crates/gpui_telegram_desktop/src/views/settings/sections/settings_information.rs`
- `crates/gpui_telegram_desktop/src/views/settings/sections/settings_main.rs`
- `crates/gpui_telegram_desktop/src/views/settings/settings_builder.rs`
- `crates/gpui_telegram_desktop/src/views/settings/settings_common.rs`
- `crates/gpui_telegram_desktop/src/views/settings/settings_common_session.rs`
- `crates/gpui_telegram_desktop/src/views/settings/settings_intro.rs`
- `crates/gpui_telegram_desktop/src/views/settings/settings_notifications_common.rs` (ensure it contains `Upstream` citation)
- `crates/gpui_telegram_desktop/src/views/settings/settings_search.rs`
- `crates/gpui_telegram_desktop/src/views/settings/settings_type.rs`

## Tasks:
1. Deeply inspect the 22 upstream C++ files.
2. Author `docs/specs/modules/module_11_settings_business.md` containing the 100% Upstream Source Reading Manifest covering all 22 files with exact line ranges (`L<start>-L<end>`), constants, formulas, and target symbols (Mandates #18 & #19).
3. Replace all 11 target files with genuine, non-stub GPUI code (0 Pattern A, B, C, D).
4. Strip the residual `//! Materialized Rust GPUI Component` lines from all 14 files above.
5. Keep single `write_to_file` calls <= 200 lines per call (Mandate #20).
6. Run `cargo check --package gpui_telegram_desktop` to verify clean compilation.
7. Run `python3 scripts/clean_build_garbage.py` after cargo commands (Mandate #11).
8. Deliver `handoff.md` and message orchestrator.

## Mandatory Integrity Warning
DO NOT CHEAT. All implementations must be genuine. DO NOT hardcode test results, create dummy/facade implementations, or circumvent the intended task. A teamwork_auditor will independently verify your work. Integrity violations WILL be detected and your work WILL be rejected.

## 2026-11-07T08:14:47Z
**Context**: Module 11 Settings Business & Headers Gap Closure
**Content**: Heartbeat status check from Orchestrator: Please report your current progress on inspecting the 22 upstream C++ files, authoring `docs/specs/modules/module_11_settings_business.md`, stripping the residual headers from the 14 files, and implementing the 11 target files.
**Action**: Update your progress.md and send a brief status report.
