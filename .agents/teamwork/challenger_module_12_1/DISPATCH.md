# Task Assignment — challenger_module_12_1

## Working Directory
`/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/challenger_module_12_1`

## Target Working Directory
`/Users/bhrum/teamwork_projects/gpui_telegram`

## Parent Orchestrator
Conversation ID: `0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345`

## Mission: Step 12D Adversarial Challenge (Intro & Platform Subsystems)
You are Challenger 1 for Module 12 (`intro & lang & platform`).

### Adversarial Scope:
1. **Adversarial Verification of Auth Wizard & Countries Subsystem**:
   - Stress test country database: invalid ISO codes, unicode flag conversions, search elision, hash verification, binary serialization round-trips.
   - Stress test auth state transitions: rapid step changes (Start -> Password -> Code), invalid PIN inputs, backspacing on empty buffer, countdown timer overflow/underflow, SRP validation edge cases.

2. **Adversarial Verification of Platform Subsystem**:
   - Challenge platform abstractions: Linux D-Bus notifications queueing/expiry, tray icon DPR scaling and menu actions, macOS file bookmark sandbox resolution, window title elision, Windows toast activator GUIDs.

3. **Author and Execute Stress/Oracle Tests**:
   - Write an adversarial test suite at `crates/gpui_telegram_desktop/tests/challenger_module_12_1_tests.rs` (<= 200 lines per write).
   - Run `cd /Users/bhrum/teamwork_projects/gpui_telegram && cargo test -p gpui_telegram_desktop --test challenger_module_12_1_tests`.
   - Run `cd /Users/bhrum/teamwork_projects/gpui_telegram && python3 scripts/clean_build_garbage.py`.

4. **Deliver Verdict**:
   Write `progress.md` and `handoff.md` with explicit verdict `APPROVE` or `REJECT`.
   Notify parent via `send_message(Recipient="0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345")`.

## 2026-11-07T10:30:01Z
You are challenger_module_12_1.
Working directory: /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/challenger_module_12_1
Parent orchestrator conversation ID: 0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345
Target project root: /Users/bhrum/teamwork_projects/gpui_telegram

Read /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md and /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/challenger_module_12_1/DISPATCH.md before starting work.

Adversarial Challenge Scope:
1. Adversarially challenge the auth wizard and countries subsystem: country search fuzzing, invalid ISO codes, unicode flag conversions, rapid state transitions, PIN input underflow/overflow, SRP verification.
2. Adversarially challenge the platform subsystem: D-Bus notification timeout/expiry, tray DPR scaling, bookmark resolution, window titles.
3. Write adversarial test suite in crates/gpui_telegram_desktop/tests/challenger_module_12_1_tests.rs (keep writes <= 200 lines per call).
4. Run cd /Users/bhrum/teamwork_projects/gpui_telegram && cargo test -p gpui_telegram_desktop --test challenger_module_12_1_tests.
5. Run cd /Users/bhrum/teamwork_projects/gpui_telegram && python3 scripts/clean_build_garbage.py.
6. Write progress.md and handoff.md with explicit verdict APPROVE or REJECT, then send completion message to parent (0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345).
