# Dispatch: challenger_module_11_2 (Step 11D - Acceptance Gate)

## Mission
You are Challenger 2 for Module 11 (`info & settings` — 373 upstream files, 198 targets).
Your focus: Adversarial verification of privacy, permissions, cloud password 2FA, and upstream C++ behavior parity.

## Mandatory Inputs (Read before starting)
- `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md`
- Master spec: `docs/specs/modules/module_11_info_settings.md`
- Upstream C++ references in `docs/specs/modules/module_11_*.md`

## Scope & Verification Tasks
1. Adversarially verify privacy rules (phone number, last seen, forwards, calls, voice messages, bio, profile photo).
2. Adversarially verify 2FA cloud password state machine (password entry, hint, recovery email, SRP verification flows).
3. Run subsystem test suites:
   - `cargo test --package gpui_telegram_desktop --test settings_privacy_chat_subsystem_tests`
   - `cargo test --package gpui_telegram_desktop --test info_members_channels_subsystem_tests`
4. Verify upstream behavioral parity and absence of dummy mocks.
5. Run `python3 scripts/clean_build_garbage.py` after cargo commands (Mandate #11).
6. Deliver verdict (`APPROVE` or `REQUEST_CHANGES`) in `handoff.md`.

## Mandatory Integrity Warning
DO NOT CHEAT. All implementations must be genuine. Integrity violations WILL be detected and rejected.


## 2026-11-07T08:45:37Z
[Message] timestamp=2026-11-07T08:45:37Z sender=0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345 priority=MESSAGE_PRIORITY_HIGH content=You are Challenger 2 for Module 11 (`info & settings` — 373 upstream files, 198 targets) (challenger_module_11_2).
Working directory: /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/challenger_module_11_2
Target project root: /Users/bhrum/teamwork_projects/gpui_telegram
Parent Orchestrator ID: 0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345

MANDATORY INPUTS (Read before starting):
- /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md
- /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/challenger_module_11_2/DISPATCH.md
- Master spec: /Users/bhrum/teamwork_projects/gpui_telegram/docs/specs/modules/module_11_info_settings.md
- Upstream C++ references in `docs/specs/modules/module_11_*.md`

VERIFICATION TASKS:
1. Adversarially verify privacy rules (phone number, last seen, forwards, calls, voice messages, bio, profile photo).
2. Adversarially verify 2FA cloud password state machine (password entry, hint, recovery email, SRP verification flows).
3. Run subsystem test suites:
   - `cargo test --package gpui_telegram_desktop --test settings_privacy_chat_subsystem_tests`
   - `cargo test --package gpui_telegram_desktop --test info_members_channels_subsystem_tests`
4. Verify upstream behavioral parity and absence of dummy mocks.
5. Run `python3 scripts/clean_build_garbage.py` after cargo commands (Mandate #11).
6. Deliver verdict (`APPROVE` or `REQUEST_CHANGES`) in `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/challenger_module_11_2/handoff.md` and message orchestrator (`0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345`).

MANDATORY INTEGRITY WARNING:
DO NOT CHEAT. All implementations must be genuine. Integrity violations WILL be detected and rejected.
