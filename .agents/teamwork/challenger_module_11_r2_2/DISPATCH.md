# Dispatch: challenger_module_11_r2_2 (Step 11D - Acceptance Gate Iteration 2)

## Mission
You are Challenger 2 for Module 11 (`info & settings` — 373 upstream files, 198 targets) Iteration 2 Re-Gate.
Your focus: Adversarial verification of privacy rules, 2FA cloud password state machine, tab adapter event handling, and upstream C++ behavior parity.

## Mandatory Inputs (Read before starting)
- `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md`
- Master spec: `docs/specs/modules/module_11_info_settings.md`
- Subsystem test suites in `crates/gpui_telegram_desktop/tests/`
- Remediation handoff: `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_11_audit_remediation/handoff.md`

## Verification Tasks
1. Adversarially verify privacy rules (phone number, last seen, forwards, calls, voice messages, bio, profile photo).
2. Adversarially verify 2FA cloud password wizard & SRP derivation.
3. Verify tab adapter state machines and event emission.
4. Run subsystem test suites:
   - `cargo test --package gpui_telegram_desktop --test settings_privacy_chat_subsystem_tests`
   - `cargo test --package gpui_telegram_desktop --test info_members_channels_subsystem_tests`
   - `cargo test --package gpui_telegram_desktop --test settings_business_subsystem_tests`
5. Run `python3 scripts/clean_build_garbage.py` after cargo commands (Mandate #11).
6. Deliver verdict (`APPROVE` or `REQUEST_CHANGES`) in `handoff.md`.

## Mandatory Integrity Warning

## 2026-11-07T09:18:21Z
You are Challenger 2 for Module 11 (`info & settings` — 373 upstream files, 198 targets) Iteration 2 Re-Gate (challenger_module_11_r2_2).
Working directory: /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/challenger_module_11_r2_2
Target project root: /Users/bhrum/teamwork_projects/gpui_telegram
Parent Orchestrator ID: 0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345

MANDATORY INPUTS (Read before starting):
- /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md
- /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/challenger_module_11_r2_2/DISPATCH.md
- Master spec: /Users/bhrum/teamwork_projects/gpui_telegram/docs/specs/modules/module_11_info_settings.md
- Remediation handoff: /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_11_audit_remediation/handoff.md

VERIFICATION TASKS:
1. Adversarially verify privacy rules (phone number, last seen, forwards, calls, voice messages, bio, profile photo).
2. Adversarially verify 2FA cloud password wizard & SRP derivation.
3. Verify tab adapter state machines and event emission.
4. Run subsystem test suites:
   - `cargo test --package gpui_telegram_desktop --test settings_privacy_chat_subsystem_tests`
   - `cargo test --package gpui_telegram_desktop --test info_members_channels_subsystem_tests`
   - `cargo test --package gpui_telegram_desktop --test settings_business_subsystem_tests`
5. Run `python3 scripts/clean_build_garbage.py` after cargo commands (Mandate #11).
6. Deliver verdict (`APPROVE` or `REQUEST_CHANGES`) in `handoff.md` and message orchestrator (`0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345`).

MANDATORY INTEGRITY WARNING:
DO NOT CHEAT. All implementations must be genuine. Integrity violations WILL be detected and rejected.
