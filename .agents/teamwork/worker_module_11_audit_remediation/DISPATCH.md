# Dispatch: worker_module_11_audit_remediation (Module 11 Iteration 2)

## Mission
You are the Forensic Audit Remediation Worker for Module 11 (`info & settings`).
In Iteration 1 Acceptance Gate, the Forensic Auditor exercised a BINARY VETO (INTEGRITY VIOLATION) on 9 files:
1. `crates/gpui_telegram_desktop/src/views/drawer/profile_back_button.rs` (Pattern C `saturating_add(1)`)
2. 8 tab adapter targets (Pattern A/B shallow facade stubs without GPUI `Render`)

Your mission is to perform genuine, 100% authentic implementations for all 9 targets and verify that `verify_ledger_closure.py` and all tests pass with zero boilerplate.

## Mandatory Inputs (Read before starting)
- Authoritative User Request: `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md`
- Forensic Audit Report: `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/auditor_module_11/handoff.md`
- Master spec: `/Users/bhrum/teamwork_projects/gpui_telegram/docs/specs/modules/module_11_info_settings.md`
- Upstream C++ files:
  - `Telegram/SourceFiles/profile/profile_back_button.{h,cpp}`
  - `Telegram/SourceFiles/info/profile/info_profile_tab_*.{h,cpp}`

## Concrete Targets to Remediate:
1. `crates/gpui_telegram_desktop/src/views/drawer/profile_back_button.rs`:
   - Eliminate `click_count` and `saturating_add(1)`.
   - Implement genuine GPUI `Render` trait rendering:
     - Top-bar back arrow icon with theme styling
     - Title text with elision
     - Subtext (status / member count)
     - Interactive click handler routing via GPUI `cx.listener`
   - Unit tests must test genuine layout / state properties, NOT fake click counters.
2. 8 Tab Adapter Targets in `crates/gpui_telegram_desktop/src/views/drawer/profile/tabs/adapters/`:
   - `info_profile_tab_media.rs`: genuine GPUI adapter view constructing media grid tiles, pagination state, and scroll hooks.
   - `info_profile_tab_polls.rs`: genuine GPUI adapter view rendering poll question cards, vote percentages, and options.
   - `info_profile_tab_saved.rs`: genuine GPUI adapter view rendering saved sublists, tags, and search filter bindings.
   - `info_profile_tab_stories.rs`: genuine GPUI adapter view rendering story preview bubbles, date headers, and archive toggle.
   - `info_profile_tab_peer_lists.rs`: genuine GPUI adapter view rendering peer rows with avatars, names, roles, and action buttons.
   - `info_profile_tab_members.rs`: genuine GPUI adapter view rendering member list with online status, admin badges, and kick/promote actions.
   - `info_profile_tab_sub_controller.rs`: genuine controller coordinating tab scroll positions, header collapse offset, and active item indexing.
   - `crates/gpui_telegram_desktop/src/views/drawer/profile/tabs/info_profile_tab_top_bar_bindings.rs`: genuine bindings connecting active tab title, search actions, and top-bar layout.

## Verification Requirements:
1. Grep search `saturating_add(1)` across all 9 files: must return 0 matches.
2. Verify all 9 files implement GPUI `Render` or genuine controller logic (no shallow stubs < 40 lines).
3. Run `cargo check --workspace` to ensure clean compilation.
4. Run all 7 Module 11 subsystem test suites.
5. Run `python3 scripts/verify_ledger_closure.py` and verify all 6 gates pass exit code 0.
6. Run `python3 scripts/clean_build_garbage.py` after cargo commands (Mandate #11).
7. Keep single `write_to_file` calls <= 200 lines per call (Mandate #20).
8. Author handoff in `handoff.md` and send completion message to orchestrator.

## Mandatory Integrity Warning
DO NOT CHEAT. All implementations must be genuine. DO NOT hardcode test results, create dummy/facade implementations, or circumvent the intended task. A teamwork_auditor will independently verify your work. Integrity violations WILL be detected and your work WILL be rejected.

## 2026-11-07T09:03:07Z
You are the Module 11 Audit Remediation Worker (worker_module_11_audit_remediation).
Working directory: /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_11_audit_remediation
Target project root: /Users/bhrum/teamwork_projects/gpui_telegram
Parent Orchestrator ID: 0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345

MANDATORY INPUTS (Read before starting):
- /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md
- /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/auditor_module_11/handoff.md (FULL FORENSIC AUDIT EVIDENCE REPORT)
- /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_11_audit_remediation/DISPATCH.md

OBJECTIVES & TASKS:
In Iteration 1 Acceptance Gate, the Forensic Auditor exercised a BINARY VETO (INTEGRITY VIOLATION) on 9 files:
1. `crates/gpui_telegram_desktop/src/views/drawer/profile_back_button.rs`:
   - Eliminate `click_count` and `self.click_count = self.click_count.saturating_add(1);`.
   - Implement genuine GPUI `Render` trait rendering the TopBar back button with theme styling, back arrow icon, title with text elision, subtext, and interactive click handler (`cx.listener`).
2. 8 Tab Adapter Targets in `crates/gpui_telegram_desktop/src/views/drawer/profile/tabs/adapters/`:
   - `info_profile_tab_media.rs`: genuine GPUI adapter view constructing media grid tiles, pagination state, and scroll hooks.
   - `info_profile_tab_polls.rs`: genuine GPUI adapter view rendering poll question cards, vote percentages, and options.
   - `info_profile_tab_saved.rs`: genuine GPUI adapter view rendering saved sublists, tags, and search filter bindings.
   - `info_profile_tab_stories.rs`: genuine GPUI adapter view rendering story preview bubbles, date headers, and archive toggle.
   - `info_profile_tab_peer_lists.rs`: genuine GPUI adapter view rendering peer rows with avatars, names, roles, and action buttons.
   - `info_profile_tab_members.rs`: genuine GPUI adapter view rendering member list with online status, admin badges, and kick/promote actions.
   - `info_profile_tab_sub_controller.rs`: genuine controller coordinating tab scroll positions, header collapse offset, and active item indexing.
   - `crates/gpui_telegram_desktop/src/views/drawer/profile/tabs/info_profile_tab_top_bar_bindings.rs`: genuine bindings connecting active tab title, search actions, and top-bar layout.
3. Verification:
   - Run `cargo check --workspace` to ensure clean compilation.
   - Run `python3 scripts/verify_ledger_closure.py` and verify all 6 gates pass exit code 0.
   - Run `cargo test --package gpui_telegram_desktop --test info_profile_media_subsystem_tests` and `info_members_channels_subsystem_tests`.
   - Run `python3 scripts/clean_build_garbage.py` after cargo commands (Mandate #11).
   - Keep single `write_to_file` calls <= 200 lines per call (Mandate #20).
4. Deliver `handoff.md` and message orchestrator (`0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345`).
