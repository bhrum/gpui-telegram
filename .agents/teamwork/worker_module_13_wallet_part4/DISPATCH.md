## 2026-11-07T11:07:23Z
You are worker_module_13_wallet_part4, a dedicated implementation worker for Module 13 Wallet Part 4.
Your working directory is: /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_13_wallet_part4
Target project root is: /Users/bhrum/teamwork_projects/gpui_telegram
Your parent orchestrator is: 0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345

MANDATORY INTEGRITY WARNING:
DO NOT CHEAT. All implementations must be genuine. DO NOT hardcode test results, create dummy/facade implementations, or circumvent the intended task. A teamwork_auditor will independently verify your work. Integrity violations WILL be detected and your work WILL be rejected.

MANDATORY ISOLATION & SOURCES:
- Read /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md
- Upstream C++ sources: /Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/wallet/ (and related wallet subdirectories)
- Ledger shard: /Users/bhrum/teamwork_projects/gpui_telegram/docs/specs/ledger/shards/wallet.json

SCOPE: Slice 13 covers `wallet.json` sorted targets [86..106] (indices 86..105, exactly 20 targets, 40 upstream source files):
- Targets:
  1. `crates/gpui_telegram_client/src/wallet/screens/ton_history_screen.rs`
  2. `crates/gpui_telegram_client/src/wallet/screens/ton_import_wallet_screen.rs`
  3. `crates/gpui_telegram_client/src/wallet/screens/ton_main_wallet_screen.rs`
  4. `crates/gpui_telegram_client/src/wallet/screens/ton_nft_detail_screen.rs`
  5. `crates/gpui_telegram_client/src/wallet/screens/ton_passcode_screen.rs`
  6. `crates/gpui_telegram_client/src/wallet/screens/ton_qr_receive_screen.rs`
  7. `crates/gpui_telegram_client/src/wallet/screens/ton_receive_screen.rs`
  8. `crates/gpui_telegram_client/src/wallet/screens/ton_send_amount_screen.rs`
  9. `crates/gpui_telegram_client/src/wallet/screens/ton_send_recipient_screen.rs`
  10. `crates/gpui_telegram_client/src/wallet/screens/ton_send_screen.rs`
  11. `crates/gpui_telegram_client/src/wallet/screens/ton_settings_screen.rs`
  12. `crates/gpui_telegram_client/src/wallet/screens/ton_staking_screen.rs`
  13. `crates/gpui_telegram_client/src/wallet/screens/ton_swap_screen.rs`
  14. `crates/gpui_telegram_client/src/wallet/screens/ton_token_detail_screen.rs`
  15. `crates/gpui_telegram_client/src/wallet/screens/ton_wallet_settings_screen.rs`
  16. `crates/gpui_telegram_client/src/wallet/service/ton_biometrics_service.rs`
  17. `crates/gpui_telegram_client/src/wallet/service/ton_keychain_service.rs`
  18. `crates/gpui_telegram_client/src/wallet/service/ton_price_service.rs`
  19. `crates/gpui_telegram_client/src/wallet/service/ton_secure_storage.rs`
  20. `crates/gpui_telegram_client/src/wallet/service/ton_wallet_service.rs`
- Upstream files: All 40 upstream `source_path` entries mapped to these 20 targets in `wallet.json`.

MISSION STEPS:
1. Initialize working directory with DISPATCH.md, BRIEFING.md, and progress.md.
2. Step 13A: Author `docs/specs/modules/module_13_wallet_part4.md` documenting 100% of all 40 upstream files in the manifest with Mandate #18 4-Layer Anti-Bluffing Proof (exact line citations `.cpp:L<start>-L<end>`, constants, formulas). Verify 0 omitted files.
3. Step 13B: Completely eradicate boilerplate patterns A, B, C, D across all 20 assigned targets. Implement authentic wallet screens, passcode/biometrics services, and wallet services.
4. Step 13C: Verify with `cargo check --workspace` and `python3 scripts/clean_build_garbage.py`. All commands must prepend `cd /Users/bhrum/teamwork_projects/gpui_telegram &&`. Keep single write calls <= 200 lines (Mandate #20).
5. Step 13D: Author handoff.md with 5 components and notify parent via send_message(Recipient="0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345").

## 2026-11-07T11:19:56Z
**Context**: Module 13 Wallet Part 4 Critical Surgical Closure
**Content**:
Please ensure:
1. `crates/gpui_telegram_agent/src/wallet/wallet_sending_effects.rs` is replaced with genuine logic from `Telegram/SourceFiles/wallet/wallet_sending_effects.{h,cpp}` (0 boilerplate).
2. Both `Telegram/SourceFiles/wallet/wallet_sending_effects.cpp` and `Telegram/SourceFiles/wallet/wallet_sending_effects.h` are included in the manifest table of `docs/specs/modules/module_13_wallet_part4.md` with Mandate #18 4-Layer Anti-Bluffing Proof so Gate 6.6 confirms 0 omitted files.
**Action**: Implement and include in your handoff report.
