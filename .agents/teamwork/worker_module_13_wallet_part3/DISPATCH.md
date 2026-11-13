## 2026-11-07T11:07:21Z
You are worker_module_13_wallet_part3, a dedicated implementation worker for Module 13 Wallet Part 3.
Your working directory is: /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_13_wallet_part3
Target project root is: /Users/bhrum/teamwork_projects/gpui_telegram
Your parent orchestrator is: 0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345

MANDATORY INTEGRITY WARNING:
DO NOT CHEAT. All implementations must be genuine. DO NOT hardcode test results, create dummy/facade implementations, or circumvent the intended task. A teamwork_auditor will independently verify your work. Integrity violations WILL be detected and your work WILL be rejected.

MANDATORY ISOLATION & SOURCES:
- Read /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md
- Upstream C++ sources: /Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/wallet/ (and related wallet subdirectories)
- Ledger shard: /Users/bhrum/teamwork_projects/gpui_telegram/docs/specs/ledger/shards/wallet.json

SCOPE: Slice 12 covers `wallet.json` sorted targets [66..86] (indices 66..85, exactly 20 targets, 40 upstream source files):
- Targets:
  1. `crates/gpui_telegram_client/src/wallet/core/ton_wallet_contract.rs`
  2. `crates/gpui_telegram_client/src/wallet/data/ton_account_state.rs`
  3. `crates/gpui_telegram_client/src/wallet/data/ton_config.rs`
  4. `crates/gpui_telegram_client/src/wallet/data/ton_dns_entry.rs`
  5. `crates/gpui_telegram_client/src/wallet/data/ton_jetton_balance.rs`
  6. `crates/gpui_telegram_client/src/wallet/data/ton_nft_item.rs`
  7. `crates/gpui_telegram_client/src/wallet/data/ton_staking_pool.rs`
  8. `crates/gpui_telegram_client/src/wallet/data/ton_transaction.rs`
  9. `crates/gpui_telegram_client/src/wallet/data/ton_wallet_data.rs`
  10. `crates/gpui_telegram_client/src/wallet/mod.rs`
  11. `crates/gpui_telegram_client/src/wallet/network/ton_api_client.rs`
  12. `crates/gpui_telegram_client/src/wallet/network/ton_api_error.rs`
  13. `crates/gpui_telegram_client/src/wallet/network/ton_liteserver_client.rs`
  14. `crates/gpui_telegram_client/src/wallet/network/ton_network_manager.rs`
  15. `crates/gpui_telegram_client/src/wallet/network/ton_toncenter_client.rs`
  16. `crates/gpui_telegram_client/src/wallet/network/ton_websocket.rs`
  17. `crates/gpui_telegram_client/src/wallet/screens/ton_backup_screen.rs`
  18. `crates/gpui_telegram_client/src/wallet/screens/ton_confirm_transaction_screen.rs`
  19. `crates/gpui_telegram_client/src/wallet/screens/ton_create_wallet_screen.rs`
  20. `crates/gpui_telegram_client/src/wallet/screens/ton_earn_screen.rs`
- Upstream files: All 40 upstream `source_path` entries mapped to these 20 targets in `wallet.json`.

MISSION STEPS:
1. Initialize working directory with DISPATCH.md, BRIEFING.md, and progress.md.
2. Step 13A: Author `docs/specs/modules/module_13_wallet_part3.md` documenting 100% of all 40 upstream files in the manifest with Mandate #18 4-Layer Anti-Bluffing Proof (exact line citations `.cpp:L<start>-L<end>`, constants, formulas). Verify 0 omitted files.
3. Step 13B: Completely eradicate boilerplate patterns A, B, C, D across all 20 assigned targets. Implement authentic wallet contract, data structures, network client interfaces, and screen models.
4. Step 13C: Verify with `cargo check --workspace` and `python3 scripts/clean_build_garbage.py`. All commands must prepend `cd /Users/bhrum/teamwork_projects/gpui_telegram &&`. Keep single write calls <= 200 lines (Mandate #20).
5. Step 13D: Author handoff.md with 5 components and notify parent via send_message(Recipient="0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345").
