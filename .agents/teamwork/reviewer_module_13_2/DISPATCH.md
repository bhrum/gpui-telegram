# Dispatch Instructions — reviewer_module_13_2

## 2026-11-07T11:46:15Z
You are `reviewer_module_13_2`, a `teamwork_reviewer` subagent conducting the Domain Logic & UI/UX Acceptance Gate Review for Module 13 (`statistics & payments & passport & export & iv & wallet`).

### Working Directory
`/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/reviewer_module_13_2`
Target Project Root: `/Users/bhrum/teamwork_projects/gpui_telegram`

### Mandatory Reading
- `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md` (Authoritative intent)
- All 14 Module 13 design specs: `docs/specs/modules/module_13_*.md`
- Master roadmap: `docs/specs/MODULE_BY_MODULE_ROADMAP.md`

### Review Mandate
1. **Domain Logic & Crypto Soundness**:
   - Verify TON crypto algorithms (BIP39 mnemonic derivation, Ed25519 keypairs, TON Cell builder, BOC serialization, CRC32-C Castagnoli and CRC16-XMODEM checksums).
   - Verify Passport encryption invariants and Telegram Payments / Stripe / Smart Glocal data models.
2. **GPUI Rendering & UI Components**:
   - Inspect GPUI element trees in Instant View readers, Wallet screens, Payment forms, and Statistics charts.
   - Verify responsive layout, style metrics, and theme token usage.
3. **Execution Verification**:
   - Run tests: `cargo test --test passport_subsystem_tests`, `cargo test -p gpui_telegram_client --test wallet_part2_subsystem_tests`, `cargo test -p gpui_telegram_client --test wallet_part4_subsystem_tests`.
   - Run `python3 scripts/clean_build_garbage.py`.
4. **Verdict**:
   - Document findings and provide explicit verdict: `APPROVE` or `REQUEST_CHANGES` in `handoff.md`.
   - Notify parent orchestrator (`0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345`).

## 2026-11-07T11:47:06Z
You are reviewer_module_13_2, a teamwork_reviewer subagent.
Your working directory is: /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/reviewer_module_13_2
Target Project Root: /Users/bhrum/teamwork_projects/gpui_telegram
Your parent orchestrator is: 0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345

MANDATORY INSTRUCTIONS:
1. Read /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md.
2. Read your task dispatch at /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/reviewer_module_13_2/DISPATCH.md.
3. Conduct acceptance review of Domain Logic, Crypto Soundness (TON BIP39, BOC/Cell, CRC, Passport crypto), and GPUI element trees/layout across Module 13.
4. Execute relevant tests: `cargo test --test passport_subsystem_tests`, `cargo test -p gpui_telegram_client --test wallet_part2_subsystem_tests`, `cargo test -p gpui_telegram_client --test wallet_part4_subsystem_tests`.
5. Run `python3 scripts/clean_build_garbage.py` after cargo commands.
6. Write handoff.md with your explicit verdict: APPROVE or REQUEST_CHANGES.
7. Notify orchestrator 0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345 via send_message.
