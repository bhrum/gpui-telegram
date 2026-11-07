# AGENTS.md — Workspace & Local Build Hygiene Mandates

## 1. Mandatory Local Build Garbage & Cache Cleanup (强制本地构建垃圾与缓存清理规范)

To prevent disk exhaustion during large Rust/GPUI multi-crate and multi-agent builds, **all agents (Builders, Explorers, Reviewers, Challengers, Auditors, Orchestrators) MUST strictly enforce the following local build hygiene rules**:

1. **Lean Cargo Profiles (`debug = 0`, `strip = "debuginfo"`, `incremental = false`)**:
   - Root `Cargo.toml` MUST keep `debug = 0`, `strip = "debuginfo"`, and `incremental = false` in `[profile.dev]`, `[profile.test]`, and `[profile.ci]`.
   - Never enable full DWARF debug info (`debug = 1` or `debug = 2`) or create separate `--target-dir` / `.cargo-target-*` directories on disk.
2. **Immediate Post-Build & Post-Test Garbage Cleanup**:
   - Immediately after executing any local `cargo build`, `cargo check --all-targets`, or `cargo test` command, agents MUST run:
     ```bash
     python3 scripts/clean_build_garbage.py
     ```
   - This script removes `target/*/incremental`, `*.dSYM` bundles, unreferenced test executable binaries in `target/debug/deps`, and older orphaned hash revisions of `.rlib` / `.rmeta` artifacts while preserving the latest compiled dependencies.
3. **Zero Temp/Scratch Accumulation**:
   - Temporary test databases, Tantivy index directories, and media CAS test fixtures created during test runs MUST use `std::env::temp_dir()` with RAII cleanup (`tempfile::TempDir`) or be explicitly deleted upon test completion.
4. **Gate Enforcement**:
   - `scripts/verify_ledger_closure.py` automatically invokes `clean_build_garbage.py` to guarantee that `target/` never accumulates stale build garbage across milestones.
