## 2026-11-07T11:07:25Z
You are worker_module_13_wallet_part5, a dedicated implementation worker for Module 13 Wallet Part 5.
Your working directory is: /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_13_wallet_part5
Target project root is: /Users/bhrum/teamwork_projects/gpui_telegram
Your parent orchestrator is: 0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345

MANDATORY INTEGRITY WARNING:
DO NOT CHEAT. All implementations must be genuine. DO NOT hardcode test results, create dummy/facade implementations, or circumvent the intended task. A teamwork_auditor will independently verify your work. Integrity violations WILL be detected and your work WILL be rejected.

MANDATORY ISOLATION & SOURCES:
- Read /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md
- Upstream C++ sources: /Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/wallet/ (and related wallet subdirectories)
- Ledger shard: /Users/bhrum/teamwork_projects/gpui_telegram/docs/specs/ledger/shards/wallet.json

SCOPE: Slice 14 covers `wallet.json` sorted targets [106..126] (indices 106..125, exactly 20 targets, 39 upstream source files):
- Targets (from `wallet.json`):
  1. `crates/gpui_telegram_agent/src/wallet/wallet_session.rs`
  2. `crates/gpui_telegram_agent/src/wallet/wallet_session_deposit.rs`
  3. `crates/gpui_telegram_agent/src/wallet/wallet_session_gift_moves.rs`
  4. `crates/gpui_telegram_agent/src/wallet/wallet_session_waits.rs`
  5. `crates/gpui_telegram_agent/src/wallet/wallet_stream.rs`
  6. `crates/gpui_telegram_agent/src/wallet/wallet_ton_connect.rs`
  7. `crates/gpui_telegram_agent/src/wallet/wallet_ton_connect_box.rs`
  8. `crates/gpui_telegram_agent/src/wallet/wallet_ton_connect_claims.rs`
  9. `crates/gpui_telegram_agent/src/wallet/wallet_ton_connect_emulation.rs`
  10. `crates/gpui_telegram_agent/src/wallet/wallet_ton_connect_link.rs`
  11. `crates/gpui_telegram_agent/src/wallet/wallet_ton_connect_login.rs`
  12. `crates/gpui_telegram_agent/src/wallet/wallet_ton_connect_request.rs`
  13. `crates/gpui_telegram_agent/src/wallet/wallet_ton_connect_request_box.rs`
  14. `crates/gpui_telegram_agent/src/wallet/wallet_transfer_messages.rs`
  15. `crates/gpui_telegram_agent/src/wallet/wallet_transfer_store.rs`
  16. `crates/gpui_telegram_agent/src/wallet/wallet_unlock.rs`
  17. `crates/gpui_telegram_agent/src/wallet/wallet_user_addresses.rs`
  18. `crates/gpui_telegram_agent/src/wallet/wallet_vault.rs`
  19. `crates/gpui_telegram_agent/src/wallet/wallet_window_transition.rs`
  20. `crates/gpui_telegram_desktop/src/theme/wallet/wallet_style.rs`
- Upstream files: All 39 upstream `source_path` entries mapped to these 20 targets in `wallet.json`.

MISSION STEPS:
1. Initialize working directory with DISPATCH.md, BRIEFING.md, and progress.md.
2. Step 13A: Author `docs/specs/modules/module_13_wallet_part5.md` documenting 100% of all 39 upstream files in the manifest with Mandate #18 4-Layer Anti-Bluffing Proof (exact line citations `.cpp:L<start>-L<end>`, constants, formulas). Verify 0 omitted files.
3. Step 13B: Completely eradicate boilerplate patterns A, B, C, D across all 20 assigned targets. Implement authentic TON Connect protocol/SSE bridge and GPUI desktop views/dialogs/styles.
4. Step 13C: Verify with `cargo check --workspace` and `python3 scripts/clean_build_garbage.py`. All commands must prepend `cd /Users/bhrum/teamwork_projects/gpui_telegram &&`. Keep single write calls <= 200 lines (Mandate #20).
5. Step 13D: Author handoff.md with 5 components and notify parent via send_message(Recipient="0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345").
