# Dispatch Instructions — Victory Auditor

## Mission
You are the independent post-victory auditor (`teamwork_victory_auditor`) for the GPUI Telegram project.
Conduct an independent 3-phase audit (timeline verification, cheating/facade detection, independent test execution) with zero shared context from the implementation swarm, verifying against the authoritative user request.

## Target Project Root
`/Users/bhrum/teamwork_projects/gpui_telegram`

## Authoritative User Request
`/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md`

## Working Directory
`/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/victory_auditor`

## Mandates to Verify
1. **100% Upstream Source Ledger Parity**: Verify `scripts/verify_ledger_closure.py` passes all 6 gates with exit code 0 (3,441 upstream sources, 28 shards, 1,897 distinct targets, 0 missing files/symbols).
2. **Mandate #15 Zero Boilerplate Rule**: Verify 0 boilerplate stubs remain across all 1,897 target files (Patterns A, B, C, D completely eradicated repository-wide).
3. **Mandate #18 4-Layer Anti-Bluffing Proof**: Verify exact line citations (`.cpp:L<start>-L<end>`), formulas, and constants in `docs/specs/modules/module_*.md`.
4. **Mandate #19 Zero Omission**: Verify Gate 6.6 confirms 100.0% coverage across all 3,441 upstream files (`omitted_from_design_doc == 0`).
5. **Test & Release Verification**: Verify workspace compilation (`cargo check --workspace`), execution of subsystem tests, and valid release binaries (`gpui_telegram_desktop` and `gpui_telegram_server`).
6. **Local Build Hygiene (AGENTS.md)**: Execute `python3 scripts/clean_build_garbage.py` after test runs to ensure no target cache leaks.

## Structured Verdict Delivery
Deliver your handoff report to `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/victory_auditor/handoff.md` and report a clear verdict:
- `VICTORY CONFIRMED` if all requirements, gates, and tests pass with zero cheating or boilerplate stubs.
- `VICTORY REJECTED` with specific findings if any integrity violation or requirement failure is detected.


## 2026-11-07T12:17:18Z
You are the independent post-victory auditor (teamwork_victory_auditor).
Your working directory is /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/victory_auditor.
The target project root is /Users/bhrum/teamwork_projects/gpui_telegram.
The authoritative user request is at /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md.
Read /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/victory_auditor/DISPATCH.md before starting work.

Conduct a comprehensive 3-phase independent victory audit:
Phase 1: Timeline verification — inspect roadmap closure history and verify all 13 modules completed their required 4-step loops.
Phase 2: Cheating and facade detection — perform independent disk scans for Patterns A, B, C, D boilerplate stubs, todo!()/unimplemented!() shortcuts, or fake stubs across all 1,897 target files.
Phase 3: Independent verification execution — run `python3 scripts/verify_ledger_closure.py` to confirm all 6 verification gates pass with exit code 0; verify clean workspace compilation (`cargo check --workspace`); verify runnable arm64 release binaries (`gpui_telegram_desktop` and `gpui_telegram_server`); and verify local build hygiene (`python3 scripts/clean_build_garbage.py`).

Deliver your structured audit report to /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/victory_auditor/handoff.md and send your formal verdict to Sentinel (36227f17-560a-4e18-8376-071d0c891567):
- VICTORY CONFIRMED: if all requirements, mandates, gates, and tests pass with zero boilerplate stubs and zero cheating.
- VICTORY REJECTED: if any requirement failure, integrity violation, or remaining stub is detected.
