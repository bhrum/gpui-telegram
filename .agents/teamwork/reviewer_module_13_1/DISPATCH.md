# Dispatch Instructions — reviewer_module_13_1

## 2026-11-07T11:46:15Z
You are `reviewer_module_13_1`, a `teamwork_reviewer` subagent conducting the Acceptance Gate Review for Module 13 (`statistics & payments & passport & export & iv & wallet`).

### Working Directory
`/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/reviewer_module_13_1`
Target Project Root: `/Users/bhrum/teamwork_projects/gpui_telegram`

### Mandatory Reading
- `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md` (Authoritative intent)
- All 14 Module 13 design specs: `docs/specs/modules/module_13_*.md`
- Master roadmap: `docs/specs/MODULE_BY_MODULE_ROADMAP.md`
- Ledger shards: `docs/specs/ledger/shards/{statistics,payments,passport,export,iv,wallet}.json`

### Review Mandate
1. **Spec Adherence & Manifest Completeness**:
   - Verify that all 502 upstream source files across all 6 shards are documented with exact line citations (`.cpp:L<start>-L<end>`), constants, formulas, and 0 omissions.
2. **Code Quality & Architecture**:
   - Inspect target files in `gpui_telegram_agent`, `gpui_telegram_client`, and `gpui_telegram_desktop`.
   - Verify proper error handling, type safety, modularity, and clean separation between data models and view logic.
3. **Zero Boilerplate**:
   - Confirm 0 occurrences of Patterns A, B, C, or D in all 289 Module 13 targets.
4. **Verification Execution**:
   - Run `cargo check --workspace` and relevant tests.
   - Run `python3 scripts/verify_ledger_closure.py`.
   - Run `python3 scripts/clean_build_garbage.py` after commands.
5. **Verdict**:
   - Document observations, logic chain, and issue explicit verdict: `APPROVE` or `REQUEST_CHANGES` in `handoff.md`.
   - Notify parent orchestrator (`0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345`).


## 2026-11-07T11:46:59Z
You are reviewer_module_13_1, a teamwork_reviewer subagent.
Your working directory is: /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/reviewer_module_13_1
Target Project Root: /Users/bhrum/teamwork_projects/gpui_telegram
Your parent orchestrator is: 0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345

MANDATORY INSTRUCTIONS:
1. Read /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md.
2. Read your task dispatch at /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/reviewer_module_13_1/DISPATCH.md.
3. Conduct acceptance review of code quality, architecture, zero boilerplate, and spec adherence across all 14 Module 13 design specs (502 upstream sources, 289 targets).
4. Run `cargo check --workspace` and `python3 scripts/verify_ledger_closure.py`.
5. Run `python3 scripts/clean_build_garbage.py` after cargo commands.
6. Write handoff.md with your explicit verdict: APPROVE or REQUEST_CHANGES.
7. Notify orchestrator 0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345 via send_message.
