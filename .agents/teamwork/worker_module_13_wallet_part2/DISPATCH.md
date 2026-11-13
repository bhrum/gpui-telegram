## 2026-11-07T11:06:54Z
You are worker_module_13_wallet_part2, a dedicated implementation worker for Module 13 Wallet Part 2.
Your working directory is: /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_13_wallet_part2
Target project root is: /Users/bhrum/teamwork_projects/gpui_telegram
Your parent orchestrator is: 0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345

MANDATORY INTEGRITY WARNING:
DO NOT CHEAT. All implementations must be genuine. DO NOT hardcode test results, create dummy/facade implementations, or circumvent the intended task. A teamwork_auditor will independently verify your work. Integrity violations WILL be detected and your work WILL be rejected.

MANDATORY ISOLATION & SOURCES:
- Read /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md
- Upstream C++ sources: /Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/wallet/ (and related wallet subdirectories)
- Ledger shard: /Users/bhrum/teamwork_projects/gpui_telegram/docs/specs/ledger/shards/wallet.json

SCOPE: Slice 11 covers `wallet.json` sorted targets [45..66] (indices 45..65, exactly 21 targets, 31 upstream source files):
- Targets:
  1. `crates/gpui_telegram_client/src/wallet/components/ton_address_input.rs`
  2. `crates/gpui_telegram_client/src/wallet/components/ton_amount_input.rs`
  3. `crates/gpui_telegram_client/src/wallet/components/ton_balance_card.rs`
  4. `crates/gpui_telegram_client/src/wallet/components/ton_comment_input.rs`
  5. `crates/gpui_telegram_client/src/wallet/components/ton_fee_chip.rs`
  6. `crates/gpui_telegram_client/src/wallet/components/ton_keypad.rs`
  7. `crates/gpui_telegram_client/src/wallet/components/ton_mnemonic_word_chip.rs`
  8. `crates/gpui_telegram_client/src/wallet/components/ton_nft_grid_item.rs`
  9. `crates/gpui_telegram_client/src/wallet/components/ton_qr_scanner_overlay.rs`
  10. `crates/gpui_telegram_client/src/wallet/components/ton_segmented_tab.rs`
  11. `crates/gpui_telegram_client/src/wallet/components/ton_token_selector.rs`
  12. `crates/gpui_telegram_client/src/wallet/components/ton_transaction_row.rs`
  13. `crates/gpui_telegram_client/src/wallet/components/ton_wallet_app_bar.rs`
  14. `crates/gpui_telegram_client/src/wallet/core/ton_account_id.rs`
  15. `crates/gpui_telegram_client/src/wallet/core/ton_amount.rs`
  16. `crates/gpui_telegram_client/src/wallet/core/ton_bip39.rs`
  17. `crates/gpui_telegram_client/src/wallet/core/ton_cell.rs`
  18. `crates/gpui_telegram_client/src/wallet/core/ton_crypto.rs`
  19. `crates/gpui_telegram_client/src/wallet/core/ton_hash.rs`
  20. `crates/gpui_telegram_client/src/wallet/core/ton_keypair.rs`
  21. `crates/gpui_telegram_client/src/wallet/core/ton_mnemonic.rs`
- Upstream files: All 31 upstream `source_path` entries mapped to these 21 targets in `wallet.json`.

MISSION STEPS:
1. Initialize working directory with DISPATCH.md, BRIEFING.md, and progress.md.
2. Step 13A: Author `docs/specs/modules/module_13_wallet_part2.md` documenting 100% of all 31 upstream files in the manifest with Mandate #18 4-Layer Anti-Bluffing Proof (exact line citations `.cpp:L<start>-L<end>`, constants, formulas). Verify 0 omitted files.
3. Step 13B: Completely eradicate boilerplate patterns A, B, C, D across all 21 assigned targets. Implement authentic TON crypto, BIP39/mnemonic, TON Cell serialization, balance/address components.
4. Step 13C: Verify with `cargo check --workspace` and `python3 scripts/clean_build_garbage.py`. All commands must prepend `cd /Users/bhrum/teamwork_projects/gpui_telegram &&`. Keep single write calls <= 200 lines (Mandate #20).
5. Step 13D: Author handoff.md with 5 components and notify parent via send_message(Recipient="0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345").
