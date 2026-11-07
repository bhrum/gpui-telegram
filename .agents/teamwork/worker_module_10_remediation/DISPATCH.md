# Dispatch: worker_module_10_remediation

## Task Description
Remediate Module 10 Emoji Fingerprint Parity defect identified by `challenger_module_10_1`:
In `crates/gpui_telegram_webrtc/src/calls/calls_emoji_fingerprint.rs`, `EMOJI_PALETTE` currently has only 321 emojis instead of upstream C++'s 333 emojis (`K_EMOJI_COUNT = 333`), and `compute_emoji_fingerprint` uses `% 321` instead of `% 333`. This produces divergent emoji selections from official Telegram clients.

## Upstream Reference
Inspect `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/calls/calls_emoji_fingerprint.cpp:L34-L121` to extract the full 333 emojis.

## Instructions
1. Read `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md`.
2. Inspect `Telegram/SourceFiles/calls/calls_emoji_fingerprint.cpp:L34-L121` for the full 333 emojis.
3. Update `crates/gpui_telegram_webrtc/src/calls/calls_emoji_fingerprint.rs`:
   - Expand `EMOJI_PALETTE` so that its length is exactly 333 (`K_EMOJI_COUNT = 333`).
   - In `compute_emoji_fingerprint`, use `indices[i] = (val as usize) % K_EMOJI_COUNT;` (or `EMOJI_PALETTE.len()`, which will now be 333).
   - Comply with Mandate #20 (writes <= 200 lines per call).
4. Run:
   `cargo test --package gpui_telegram_webrtc --test calls_core_adversarial_oracle_tests`
   Verify all 10/10 tests pass with 0 failures!
5. Run:
   `cargo test --package gpui_telegram_webrtc --test calls_core_window_subsystem_tests`
   `cargo test --package gpui_telegram_webrtc --test calls_group_part1_subsystem_tests`
   `cargo test --package gpui_telegram_webrtc --test calls_group_part2_subsystem_tests`
   `cargo test --package gpui_telegram_webrtc --test calls_group_adversarial_oracle_tests`
   `cargo test --package gpui_telegram_desktop --lib theme::calls::calls_style::tests`
6. Run `python3 scripts/verify_ledger_closure.py` and verify all 6 gates pass exit 0 (confirming 0 boilerplate across all 45 targets in Module 10).
7. Run `python3 scripts/clean_build_garbage.py`.
8. Author handoff report in `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_10_remediation/handoff.md` and send completion message to orchestrator.

## 2026-11-07T06:51:44Z
Remediate the Emoji Fingerprint Parity defect in `crates/gpui_telegram_webrtc/src/calls/calls_emoji_fingerprint.rs`:
1. Inspect `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/calls/calls_emoji_fingerprint.cpp:L34-L121` to extract the full 333 emojis.
2. Update `crates/gpui_telegram_webrtc/src/calls/calls_emoji_fingerprint.rs`:
   - Expand `EMOJI_PALETTE` to exactly 333 emojis (`K_EMOJI_COUNT = 333`).
   - In `compute_emoji_fingerprint`, use `indices[i] = (val as usize) % K_EMOJI_COUNT;`.
3. Run tests and verify 10/10 tests pass.
4. Run `python3 scripts/verify_ledger_closure.py`.
5. Run `python3 scripts/clean_build_garbage.py`.
6. Author handoff report and send completion message to orchestrator.
