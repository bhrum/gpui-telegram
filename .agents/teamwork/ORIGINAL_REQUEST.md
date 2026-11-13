# Authoritative User Request — GPUI Telegram Desktop 1:1 Refactor (`gpui_telegram`)

- **Target Working Directory**: `/Users/bhrum/teamwork_projects/gpui_telegram`
- **Upstream C++ Reference**: `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles` (3,441 files across 28 shards in `docs/specs/ledger/shards/*.json`)
- **Authoritative Sequential Roadmap**: `/Users/bhrum/teamwork_projects/gpui_telegram/docs/specs/MODULE_BY_MODULE_ROADMAP.md`
- **Verification Gate Script**: `/Users/bhrum/teamwork_projects/gpui_telegram/scripts/verify_ledger_closure.py`
- **Build Garbage Cleanup Script**: `/Users/bhrum/teamwork_projects/gpui_telegram/scripts/clean_build_garbage.py`

## Mandates #1 – #20
1. **Mandate #1–#10**: Full inheritance of `inherited/*` crates, 3,441 upstream source-file ledger across 28 shards, parallel multi-worker execution, and physical Gate 6 verification.
2. **Mandate #11 (`AGENTS.md`)**: Single `target/` directory, `debug = 0`, `strip = "debuginfo"`, `incremental = false`, and run `python3 scripts/clean_build_garbage.py` after builds/tests.
3. **Mandate #15–#17**: Execute sequentially across 13 Modules (`Module 01` through `Module 13`). Every module must complete Step A (Deep C++ source reading & `docs/specs/modules/module_<nn>_*.md`), Step B (`0 Boilerplate` genuine Rust/GPUI implementation), Step C (Compile test binaries & run module tests), and Step D (`5-Agent Acceptance Gate`: 2 Reviewers + 2 Challengers + 1 Forensic Auditor with binary veto -> `5/5 APPROVE / CLEAN`).
4. **Mandate #18**: 4-Layer Anti-Bluffing Proof of Genuine `.cpp` Source Reading (exact `.cpp:L<start>-L<end>` line citations, Reviewer spot-checks, Challenger blind C++ oracle tests, Forensic Auditor binary veto).
5. **Mandate #19**: 3-Pillar Zero-Omitted-File Guarantee (`25-45` upstream files per slice worker, 100% per-file reading manifest in `docs/specs/modules/module_<nn>_*.md`, and Gate 6.6 `omitted_from_design_doc == 0`).
6. **Mandate #20**: Keep single `write_to_file` calls <= 200 lines per call to avoid 60s stream interruption timeouts.
