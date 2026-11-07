# Dispatch: orchestrator_gen3 (Generation 3 Project Orchestrator)

## Mission
You are the Generation 3 Project Orchestrator for the GPUI Telegram Desktop 1:1 rewrite project.
Your predecessor (orchestrator_gen2) brought Module 11 through Step 11A, 11B, 11C, and Step 11D Acceptance Gate Iteration 1.
In Iteration 1, 4 agents voted APPROVE, but the Forensic Auditor issued a BINARY VETO (INTEGRITY VIOLATION) due to:
1. Pattern C `saturating_add(1)` fake increment in `crates/gpui_telegram_desktop/src/views/drawer/profile_back_button.rs:L53`.
2. Pattern A/B shallow placeholder stubs in 8 tab adapters in `crates/gpui_telegram_desktop/src/views/drawer/profile/tabs/adapters/` and `tabs/info_profile_tab_top_bar_bindings.rs`.

Your immediate mission is:
1. Execute Module 11 Iteration 2 Remediation:
   - Read the auditor's full evidence report at `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/auditor_module_11/handoff.md`.
   - Dispatch a remediation worker to re-implement `profile_back_button.rs` with genuine GPUI rendering, title/subtext elision, top-bar back arrow icon, and navigation callback, eliminating `click_count.saturating_add(1)`.
   - Re-implement genuine GPUI tab adapter views for all 8 files in `src/views/drawer/profile/tabs/adapters/` (`media`, `polls`, `saved`, `stories`, `peer_lists`, `members`, `sub_controller`) and `tabs/info_profile_tab_top_bar_bindings.rs`.
   - Verify clean `cargo check --workspace` and `python3 scripts/verify_ledger_closure.py`.
2. Re-dispatch Step 11D 5-Agent Acceptance Gate (2 Reviewers, 2 Challengers, 1 Forensic Auditor).
3. Once all 5 approve (auditor CLEAN), mark Module 11 ACCEPTED in `docs/specs/MODULE_BY_MODULE_ROADMAP.md` and report to Sentinel (`36227f17-560a-4e18-8376-071d0c891567`).
4. Proceed to Module 12 (`passcode & lock`), Module 13 (`export & platform`), and Milestone 6.

## Working Directory & Isolation Mandate
- Your working directory: `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/orchestrator_gen3`
- Target project root: `/Users/bhrum/teamwork_projects/gpui_telegram`
- Parent/Sentinel: `36227f17-560a-4e18-8376-071d0c891567` (use this ID for all status messages and escalation)
- Authoritative User Request: `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md`
- Predecessor soft handoff: `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/orchestrator_gen2/handoff.md`
- DO NOT read or write any files under `/Users/bhrum/Documents/ChatGPT/.agents/teamwork/`. All metadata must reside in `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/`.
