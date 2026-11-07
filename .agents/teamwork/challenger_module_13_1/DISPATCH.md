# Dispatch Instructions — challenger_module_13_1

## 2026-11-07T11:46:15Z
You are `challenger_module_13_1`, a `teamwork_challenger` subagent conducting Adversarial Stress & Correctness Verification for Module 13 subdomains: Instant View (IV), Statistics, and Payments.

### Working Directory
`/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/challenger_module_13_1`
Target Project Root: `/Users/bhrum/teamwork_projects/gpui_telegram`

### Mandatory Reading
- `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md` (Authoritative intent)
- Design specs: `module_13_statistics.md`, `module_13_payments_part1.md`, `module_13_payments_part2.md`, `module_13_iv_part1.md`, `module_13_iv_part2.md`, `module_13_iv_part3.md`, `module_13_iv_part4.md`

### Challenger Mandate
1. **Adversarial & Stress Scenarios**:
   - Write and execute an adversarial test harness (or test file in `crates/gpui_telegram_desktop/tests/` or standalone test binary) that rigorously challenges:
     - Instant View document parsing and tree building under corrupted, recursive, or oversized blocks (nested blockquotes, tables with 0 rows/columns, invalid media types).
     - Statistics chart interpolation, zero-division in percentage deltas, timestamp out-of-order points, huge zoom ranges.
     - Payments invoice state machine under conflicting callbacks, currency precision overflow, double-charge attempts, and network drops.
2. **Execution & Cleanup**:
   - Run your adversarial test suite and verify all test cases pass.
   - Run `python3 scripts/clean_build_garbage.py` after each test execution.
3. **Verdict**:
   - Document adversarial test matrix, results, and provide explicit verdict: `APPROVE` or `REQUEST_CHANGES` in `handoff.md`.
   - Notify parent orchestrator (`0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345`).


## 2026-11-07T11:47:13Z
You are challenger_module_13_1, a teamwork_challenger subagent.
Your working directory is: /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/challenger_module_13_1
Target Project Root: /Users/bhrum/teamwork_projects/gpui_telegram
Your parent orchestrator is: 0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345

MANDATORY INSTRUCTIONS:
1. Read /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md.
2. Read your task dispatch at /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/challenger_module_13_1/DISPATCH.md.
3. Design, execute, and verify an adversarial stress test suite for Instant View (IV), Statistics, and Payments subdomains (testing parser boundaries, corrupted/recursive blocks, chart math, zero-division, invoice concurrency, overflow).
4. Run your adversarial test suite and verify all test cases pass cleanly.
5. Run `python3 scripts/clean_build_garbage.py` after test commands.
6. Write handoff.md with your test matrix and explicit verdict: APPROVE or REQUEST_CHANGES.
7. Notify orchestrator 0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345 via send_message.
