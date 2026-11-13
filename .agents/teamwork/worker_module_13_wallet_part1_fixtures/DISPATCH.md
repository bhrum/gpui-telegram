## 2026-11-07T11:06:52Z
You are worker_module_13_wallet_part1_fixtures, a dedicated implementation worker for Module 13 Wallet Part 1 (Fixtures/Assets).
Your working directory is: /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_13_wallet_part1_fixtures
Target project root is: /Users/bhrum/teamwork_projects/gpui_telegram
Your parent orchestrator is: 0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345

MANDATORY INTEGRITY WARNING:
DO NOT CHEAT. All implementations must be genuine. DO NOT hardcode test results, create dummy/facade implementations, or circumvent the intended task. A teamwork_auditor will independently verify your work. Integrity violations WILL be detected and your work WILL be rejected.

MANDATORY ISOLATION & SOURCES:
- Read /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md
- Ledger shard: /Users/bhrum/teamwork_projects/gpui_telegram/docs/specs/ledger/shards/wallet.json

SCOPE: Slice 10 covers `wallet.json` sorted targets [0..45] (indices 0..44, exactly 45 targets, 45 upstream source files):
- Targets: All 45 SVG asset/fixture targets under `crates/gpui_telegram_desktop/assets/wallet/*` (from `assets/wallet/badge_danger.svg` through `assets/wallet/transaction_swap.svg`).
- Note: In Gate 6.2, these asset files have 0 boilerplate stubs.
- Mission: Author the authoritative design specification `docs/specs/modules/module_13_wallet_part1_fixtures.md` documenting 100% of all 45 upstream files in the manifest table with Mandate #18 4-Layer Anti-Bluffing Proof (exact upstream source paths, SVG XML dimensions, viewbox, color tokens, and asset mapping). Verify that all 45 target SVG assets are intact and valid.
- Run `cargo check --workspace` and `python3 scripts/clean_build_garbage.py`. All commands must prepend `cd /Users/bhrum/teamwork_projects/gpui_telegram &&`.
- Author handoff.md with 5 components and notify parent via send_message(Recipient="0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345").
