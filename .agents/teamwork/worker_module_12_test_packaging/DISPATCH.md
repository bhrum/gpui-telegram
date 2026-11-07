# Task Assignment — worker_module_12_test_packaging

## Working Directory
`/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_12_test_packaging`

## Target Working Directory
`/Users/bhrum/teamwork_projects/gpui_telegram`

## Parent Orchestrator
Conversation ID: `0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345`

## Mission: Step 12C Test Packaging & Comprehensive Verification
You are the Test Packager and Workspace Verification specialist for Module 12 (`intro & lang & platform`). All 5 implementation slices have completed on disk covering all 195 upstream files and 116 target files with 0 boilerplate stubs.

Your job is to execute the end-to-end build, test execution, gate closure, and binary build verification, resolving any compilation, test, or lint defects discovered.

### Mandatory Verification Steps (Execute in `/Users/bhrum/teamwork_projects/gpui_telegram`):
1. **Workspace Compilation**:
   Run `cd /Users/bhrum/teamwork_projects/gpui_telegram && cargo check --workspace`.
   Verify exit code 0.

2. **Subsystem Test Execution**:
   Run the dedicated Module 12 integration and unit tests:
   - `cd /Users/bhrum/teamwork_projects/gpui_telegram && cargo test -p gpui_telegram_desktop --test intro_subsystem_tests`
   - `cd /Users/bhrum/teamwork_projects/gpui_telegram && cargo test -p gpui_telegram_desktop --test i18n_lang_subsystem_tests`
   - Run additional unit tests in `gpui_telegram_desktop` as appropriate.
   Ensure all tests pass with 0 failures.

3. **Ledger Closure Gate Check**:
   Run `cd /Users/bhrum/teamwork_projects/gpui_telegram && python3 scripts/verify_ledger_closure.py`.
   Assert all 6 gates pass with Exit Code 0:
   - Gate 6.2: Module 12 reports Targets: 116, Boilerplate: 0.
   - Gate 6.6: All 5 specs in `docs/specs/modules/module_12_*.md` report 100% upstream source coverage (195 / 195 sources, 0 omitted).

4. **Binary Target Build**:
   Run `cd /Users/bhrum/teamwork_projects/gpui_telegram && cargo build --bin gpui_telegram_desktop --bin gpui_telegram_server`.
   Verify exit code 0.

5. **Build Hygiene (Mandate #11 & AGENTS.md)**:
   Always run `cd /Users/bhrum/teamwork_projects/gpui_telegram && python3 scripts/clean_build_garbage.py` after cargo commands.

6. **Documentation & Handoff**:
   Write `progress.md` and a complete 5-Component `handoff.md` (Observation, Logic Chain, Caveats, Conclusion, Verification Method) in your working directory.
   Then send a completion message back to parent (`0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345`).

## 2026-11-07T10:20:15Z
[Message] timestamp=2026-11-07T10:20:15Z sender=0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345 priority=MESSAGE_PRIORITY_HIGH content=You are worker_module_12_test_packaging.
Tasks for Step 12C (Module 12 intro & lang & platform):
1. cd /Users/bhrum/teamwork_projects/gpui_telegram && cargo check --workspace (verify exit code 0).
2. Run tests:
   cd /Users/bhrum/teamwork_projects/gpui_telegram && cargo test -p gpui_telegram_desktop --test intro_subsystem_tests
   cd /Users/bhrum/teamwork_projects/gpui_telegram && cargo test -p gpui_telegram_desktop --test i18n_lang_subsystem_tests
   Ensure all tests pass.
3. Verify ledger closure:
   cd /Users/bhrum/teamwork_projects/gpui_telegram && python3 scripts/verify_ledger_closure.py
   Assert all 6 gates pass with exit code 0. Module 12 reports 0 boilerplate stubs and 195/195 upstream files documented.
4. Build binary targets:
   cd /Users/bhrum/teamwork_projects/gpui_telegram && cargo build --bin gpui_telegram_desktop --bin gpui_telegram_server
   Verify exit code 0.
5. Build hygiene:
   cd /Users/bhrum/teamwork_projects/gpui_telegram && python3 scripts/clean_build_garbage.py
6. Write progress.md and handoff.md in your working directory, then notify parent via send_message(Recipient="0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345").
