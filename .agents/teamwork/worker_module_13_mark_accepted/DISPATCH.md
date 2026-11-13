# Dispatch Instructions — worker_module_13_mark_accepted

## 2026-11-07T12:10:00Z
You are `worker_module_13_mark_accepted`, a `teamwork_worker` subagent responsible for marking Module 13 as ACCEPTED and advancing `docs/specs/MODULE_BY_MODULE_ROADMAP.md` to 13 / 13 ACCEPTED (100.0%).

### Working Directory
`/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_13_mark_accepted`
Target Project Root: `/Users/bhrum/teamwork_projects/gpui_telegram`

### Mandatory Reading
- `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md`
- `/Users/bhrum/teamwork_projects/gpui_telegram/docs/specs/MODULE_BY_MODULE_ROADMAP.md`
- `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/orchestrator_gen2/GATE_STATUS.md`

### Task Instructions
1. Inspect `/Users/bhrum/teamwork_projects/gpui_telegram/docs/specs/MODULE_BY_MODULE_ROADMAP.md`.
2. Update Module 13 row status in the Roadmap table to `ACCEPTED`:
   - Change `PLANNED` or `IN_PROGRESS` to `ACCEPTED`.
   - Update the overall roadmap progress counter to `13 / 13 ACCEPTED (100.0%)`.
   - Update Module 13 section details: 289 targets, 0 boilerplate stubs, 502/502 upstream sources manifest, 170+ tests passing exit 0.
3. Run ledger closure verification:
   `cd /Users/bhrum/teamwork_projects/gpui_telegram && python3 scripts/verify_ledger_closure.py`
   Confirm ALL 6 GATES PASS with exit code 0.
4. Run build cache cleaner:
   `python3 scripts/clean_build_garbage.py`
5. Author `handoff.md` in your working directory and notify orchestrator (`0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345`).

### Mandatory Integrity Warning
DO NOT CHEAT. All implementations must be genuine. DO NOT hardcode test results, create dummy/facade implementations, or circumvent the intended task. Integrity violations WILL be detected and your work WILL be rejected.
