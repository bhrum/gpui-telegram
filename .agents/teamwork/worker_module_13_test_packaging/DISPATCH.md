# Dispatch Instructions — worker_module_13_test_packaging

## 2026-11-07T11:35:00Z
You are `worker_module_13_test_packaging`, a `teamwork_worker` subagent responsible for Step 13C: Test Packaging & Workspace Verification for Module 13 (`statistics & payments & passport & export & iv & wallet`).

### Working Directory
`/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_13_test_packaging`
Target Project Root: `/Users/bhrum/teamwork_projects/gpui_telegram`

### Mandatory Reading
- `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md` (Authoritative user request)
- All Module 13 design specs: `docs/specs/modules/module_13_*.md` (14 specs covering 502 upstream sources)

### Task Execution Plan
1. Check workspace compilation:
   `cd /Users/bhrum/teamwork_projects/gpui_telegram && cargo check --workspace && python3 scripts/clean_build_garbage.py`
2. Run full workspace build:
   `cd /Users/bhrum/teamwork_projects/gpui_telegram && cargo build --workspace && python3 scripts/clean_build_garbage.py`
3. Execute all Module 13 test suites:
   - `cargo test --test statistics_module_tests`
   - `cargo test --test payments_part1_tests`
   - `cargo test --test payments_part2_tests`
   - `cargo test --test passport_subsystem_tests`
   - `cargo test --test passport_style_tests`
   - `cargo test --test iv_part1_subsystem_tests`
   - `cargo test --test iv_part2_subsystem_tests`
   - `cargo test --test iv_part3_subsystem_tests`
   - `cargo test --test iv_part4_subsystem_tests`
   - `cargo test -p gpui_telegram_agent --lib -- wallet::gram_tests_main`
   - `cargo test -p gpui_telegram_client --test wallet_part2_subsystem_tests`
   - `cargo test -p gpui_telegram_client --test wallet_part4_subsystem_tests`
   - `cargo test -p gpui_telegram_desktop --test wallet_part5_subsystem_tests`
4. Run full ledger closure verification:
   `cd /Users/bhrum/teamwork_projects/gpui_telegram && python3 scripts/verify_ledger_closure.py`
   Confirm ALL 6 GATES PASS with exit code 0 (Gate 6.2: 0 boilerplate stubs in Module 13 across all 289 targets; Gate 6.6: 502/502 upstream files documented).
5. Build binary targets:
   - `cargo build -p gpui_telegram_desktop --bin gpui_telegram_desktop`
   - `cargo build -p gpui_telegram_server --bin gpui_telegram_server`
6. Clean build garbage after each step:
   `python3 scripts/clean_build_garbage.py`
7. Author `handoff.md` in your directory documenting all outputs and notify parent orchestrator (`0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345`).

### Mandatory Integrity Warning
DO NOT CHEAT. All implementations must be genuine. DO NOT hardcode test results, create dummy/facade implementations, or circumvent the intended task. A teamwork_auditor will independently verify your work. Integrity violations WILL be detected and your work WILL be rejected.
