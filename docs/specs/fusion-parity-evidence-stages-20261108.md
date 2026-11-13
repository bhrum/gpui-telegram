# FUS-V001a: multi-stage capability evidence schema and historical claim demotion

Status: implementation delivered for `FUS-V001a` documentation/ledger schema slice; independent coordinator review pending; no parity or release acceptance claimed. Parent scope remains full Telegram (`tdesktop` @ `f985723b552b30fe1c17d377d57f44e61a15a8c4`) and prior Fabushi (`fabushi-unified-design-20261105`) fusion reacceptance.

## Current and target behavior

Currently:
1. [`docs/verification/fusion-20261108/capability-coverage.json`](file:///Users/bhrum/teamwork_projects/gpui_telegram/docs/verification/fusion-20261108/capability-coverage.json) records 52 Telegram (`TCG-N01..TCG-N52`) and 6 aggregate Fabushi (`CAP-01..CAP-06`) capabilities using a single flat `"state": "requires_behavior_audit"` field. It does not separate static file/symbol mapping from runtime entrypoint reachability, behavioral test verification, actual running GPUI UI evidence, or packaged application acceptance.
2. [`docs/specs/MODULE_BY_MODULE_ROADMAP.md`](file:///Users/bhrum/teamwork_projects/gpui_telegram/docs/specs/MODULE_BY_MODULE_ROADMAP.md) marks all 13 modules (`3,441` upstream files, `1,897` target files) as `ACCEPTED` (`13 / 13 ACCEPTED (100.0%)`) based on `scripts/verify_ledger_closure.py` Gate 6.2 regex/boilerplate burndown (`0` matches across 4 literal template patterns), even though [`docs/specs/ledger/index.json`](file:///Users/bhrum/teamwork_projects/gpui_telegram/docs/specs/ledger/index.json) reports `"implemented_count": 0` and `"verified_count": 0` across all 28 shards, 7 shard files referenced in `capability-coverage.json` (`webauthn.json`, `poll.json`, `overview.json`, `profile.json`, `inherited_host.json`, `inherited_node.json`, `inherited_messaging.json`) are absent on disk, 5 existing shards (`iv.json`, `passport.json`, `payments.json`, `statistics.json`, `wallet.json` — 467 files) are unreferenced in `capability-coverage.json`, and multiple desktop/server/agent surfaces remain unmounted or stubbed.

Target behavior for `FUS-V001a`:
1. Introduce an explicit 5-stage capability evidence schema in `docs/verification/fusion-20261108/capability-coverage.json` (`file_mapped`, `entrypoint_reachable`, `behavior_tested`, `ui_evidence_captured`, `packaged_accepted`) plus structured `historical_claims` attribution on every capability while preserving all 58 capability entries and all existing keys for downstream compatibility.
2. Demote unsupported aggregate `ACCEPTED` / `13 / 13 ACCEPTED (100.0%)` assertions in `docs/specs/MODULE_BY_MODULE_ROADMAP.md` to active `REVIEW` status (`0 / 13 ACCEPTED` for the 2026-11-08 fusion campaign) while preserving all original file/target/boilerplate counts and 2026-11-06..07 module review/challenger/auditor records explicitly labeled as historical pre-fusion claims.
3. Provide a standalone, zero-Cargo Python validator test suite (`scripts/test_fusion_evidence_stages.py`) that verifies positive and negative stage-promotion invariants, shard existence truth, and roadmap demotion without touching active server/desktop code or running Cargo builds.

## Ownership

Exclusive write boundary for `FUS-V001a`:
- `docs/specs/fusion-parity-evidence-stages-20261108.md`
- `docs/verification/fusion-20261108/capability-coverage.json`
- `docs/specs/MODULE_BY_MODULE_ROADMAP.md`
- `scripts/test_fusion_evidence_stages.py`

Explicitly excluded from `FUS-V001a` (owned by separate follow-up slices):
- **`FUS-V001b`**: `scripts/verify_ledger_closure.py` and `docs/specs/ledger/index.json` (hardening automated gates to fail closed on unreachable symbols, `#![allow(dead_code)]` facades, bare `pub use *` shims in `domain.rs`/`tdrp/mod.rs`, and non-existent `verification_test_id` files).
- **`FUS-V001c`**: `docs/specs/01-telegram-1to1-feature-inventory.md` and `docs/specs/ledger/shards/*.json` (adding `TCG-N53+` capability domains for TON Wallet, Payments/Stars/Gifts, Passport, Instant View/Editor, Statistics/Boosts, Business/Premium, and Media Editor, and reclassifying misassigned shard entries).
- **`FUS-V001d`**: Inherited Fabushi capability decomposition (`CAP-01..CAP-06` expansion across `mahayana-host-runtime`, `fabushi-messaging-core`, `mahayana-node-agent-coordinator`, `fabushi-box-exec-daemon`, and excluded sub-workspaces `mahayana-rs` and `codex-rs`, plus materializing `inherited_*.json` shards).
- **`FUS-V001e`**: Sanitizing regex-scraped C++ pseudo-symbols (`bool`, `uchar`, `uint32`, `#include` directives) and non-code entries (`Telegram/SourceFiles/AGENTS.md`) in `docs/specs/ledger/shards/*.json`.
- **`FUS-V001f`**: Implementing concrete Tier 3 Headless GPUI and Tier 4 E2E drivers in `crates/gpui_telegram_test_harness/`.

## Contract

### 1. Five-stage capability evidence model (`capability-coverage.json`)

Every capability entry in `capabilities` MUST preserve all legacy keys (`capability_id`, `capability`, `upstream_anchors`, `candidate_owners`, `ledger_shards`, `state`, `scope_source`, `next_task`) and include:
- `source_oracle`:
  - `kind`: `"upstream_tdesktop"` (`TCG-*`) or `"inherited_fabushi"` (`CAP-*`)
  - `revision`: `"f985723b552b30fe1c17d377d57f44e61a15a8c4"` (`TCG-*`) or `"fabushi-unified-design-20261105"` (`CAP-*`)
- `evidence_stages`:
  1. `file_mapped`:
     - `status`: `"verified"` | `"partial"` | `"missing_shard"` | `"unverified"`
     - `existing_shards`: array of shard filenames from `ledger_shards` that physically exist under `docs/specs/ledger/shards/`
     - `missing_shards`: array of shard filenames from `ledger_shards` that do not exist on disk
     - `shard_mapped_files`: sum of `mapped_count` across `existing_shards` in `docs/specs/ledger/index.json`
     - `shard_implemented_count`: sum of `implemented_count` across `existing_shards` in `docs/specs/ledger/index.json` (currently `0`)
     - `shard_verified_count`: sum of `verified_count` across `existing_shards` in `docs/specs/ledger/index.json` (currently `0`)
     - **Rule**: `file_mapped.status` MUST be `"missing_shard"` if `existing_shards` is empty, `"partial"` if `missing_shards` is non-empty, and `"verified"` only when all referenced shards exist on disk.
  2. `entrypoint_reachable`:
     - `status`: `"verified"` | `"partial"` | `"stubbed"` | `"unverified"`
     - `verified_entrypoints`: array of verified runtime entrypoints (currently `[]` pending full audit)
     - `blocking_findings`: array of concrete reachability/stub findings or pending audit notes
     - **Rule**: Static symbol presence (`pub struct/enum/fn`) or unused `pub use *` re-export shims (`domain.rs`, `tdrp/mod.rs`) MUST NOT set `entrypoint_reachable.status` to `"verified"`.
  3. `behavior_tested`:
     - `status`: `"verified"` | `"partial"` | `"unverified"`
     - `verified_runs`: array of independent coordinator-verified test run references (e.g., `20261108T212435` for `fusion_durable_chat_routes` 8/8 on `TCG-N11`/`TCG-N12`, `20261108T212341` for `fusion_atomic_message_commit` 6/6 on `TCG-N04`/`TCG-N20`)
     - `unverified_scope`: explicit description of unverified positive/negative behavior, restart durability, and edge cases
     - **Rule**: Non-empty `verification_test_id` strings in ledger shards or historical pre-fusion unit test summaries MUST NOT set `behavior_tested.status` to `"verified"`. Focused slice runs set `"status": "partial"` until full capability behavior is verified.
  4. `ui_evidence_captured`:
     - `status`: `"verified"` | `"unverified"`
     - `artifacts`: array of verified GPUI binary capture/interaction artifacts (currently `[]` for all 58 capabilities)
     - **Rule**: AppKit substitutes, static `Render` trait implementations with `#![allow(dead_code)]`, or placeholder banners (`ModalContainer::render`) MUST NOT satisfy `ui_evidence_captured`.
  5. `packaged_accepted`:
     - `status`: `"verified"` | `"unverified"`
     - `artifacts`: array of packaged `.app`/release acceptance records with candidate provenance (currently `[]` for all 58 capabilities)
     - **Rule**: `packaged_accepted.status` MUST NOT be `"verified"` unless `file_mapped`, `entrypoint_reachable`, `behavior_tested`, and `ui_evidence_captured` are all `"verified"`.
- `historical_claims`:
  - `pre_fusion_roadmap_modules`: array of Module IDs (`"Module 01"` .. `"Module 13"`) or `"unmapped_in_13_module_table"`
  - `pre_fusion_claimed_status`: preserved historical claim string (`"ACCEPTED (historical Gate 6.2 boilerplate-0 claim, 2026-11-06..07)"`)
  - `disposition`: `"historical_claim_retained_not_accepted_for_fusion_20261108"`

### 2. Roadmap status demotion and historical preservation (`MODULE_BY_MODULE_ROADMAP.md`)

- All 13 module rows in the Master Module Acceptance & Burndown Table MUST demote active `Status` from `ACCEPTED` to `REVIEW` and preserve `ACCEPTED (Historical 2026-11-06..07 Gate 6.2 Claim)` in a dedicated `Historical Claim` column.
- The `TOTAL` row MUST report `0 / 13 ACCEPTED (13 / 13 REVIEW; Historical Claims Preserved)` instead of `13 / 13 ACCEPTED (100.0%)`, while preserving the exact historical file counts (`3,441` upstream files, `1,897` target files, `1,501` baseline stubs).
- Each `### Module 01` .. `### Module 13` section MUST set `- **Status**: \`REVIEW\`` (maintaining parser compatibility with `scripts/verify_ledger_closure.py` lines 394–410 and 594–601 so Gate 6.6 remains active while `accepted_count` becomes `0`), add an explicit `- **Fusion 2026-11-08 Evidence Stage Summary**:`, and retain the original acceptance date, reviewer/challenger/auditor verdict, and test counts under `- **Historical Pre-Fusion Claim (2026-11-06..07 — Not Fusion Acceptance)**:`.

## Proposing `FUS-V001b` (Separate Follow-Up Slice)

`FUS-V001b` owns `scripts/verify_ledger_closure.py` and `docs/specs/ledger/index.json` (plus validator unit tests) with the following contract:
1. Cross-check `docs/verification/fusion-20261108/capability-coverage.json` stage invariants during gate execution.
2. Fail Gate 2 / Gate 6 if any entry marked `implemented` or `verified` references a non-existent `verification_test_id` path on disk.
3. Fail Gate 6.2 if any `gpui_ui_1to1` or `rust_domain_1to1` file claimed as `entrypoint_reachable` contains `#![allow(dead_code)]`, is unreferenced from `crates/gpui_telegram_desktop/src/main.rs` (`AppShell`) or `crates/gpui_telegram_server/src/server.rs` (`build_router_with_state`), or uses paraphrased timestamp-mutation stub bodies (`self.updated_at = chrono::Utc::now().timestamp(); Ok(())`).
4. Fail Gate 6.1a / Gate 6.5 when `"disposition": "inherited_rust_reused"` target files do not import/call symbols from their declared `inherited_rust_path`, or when crate-level reuse relies solely on unused `pub use *` shims (`domain.rs`, `tdrp/mod.rs`).

## Verification

Run the standalone Python validator suite (no Cargo build or network access required):
```bash
python3 -m unittest discover -s scripts -p "test_fusion_evidence_stages.py" -v
```
The validator tests:
- All 58 capabilities (`TCG-N01..TCG-N52`, `CAP-01..CAP-06`) and all legacy fields are preserved intact.
- Every capability has complete 5-stage `evidence_stages`, `source_oracle`, and `historical_claims` objects.
- Shard existence on disk matches `file_mapped.status` (`partial` for `TCG-N01`, `TCG-N16`, `TCG-N22`, `TCG-N30`; `missing_shard` for `CAP-01..CAP-06`; `verified` with `shard_implemented_count == 0` and `shard_verified_count == 0` for the remaining 48 capabilities).
- Negative promotion guards reject any capability claiming `accepted`/`complete` or `packaged_accepted == "verified"` when upstream stages are `unverified`/`partial`/`stubbed`/`missing_shard`.
- `docs/specs/MODULE_BY_MODULE_ROADMAP.md` has `0` active `ACCEPTED` modules in `scripts/verify_ledger_closure.py`'s parser logic (`module_status_map[mid] == "REVIEW"` for all 13 modules) while preserving all 13 historical claim records and original `3,441` / `1,897` / `1,501` counts.
