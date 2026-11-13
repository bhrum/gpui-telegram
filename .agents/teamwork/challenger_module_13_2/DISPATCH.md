# Dispatch Instructions — challenger_module_13_2

## 2026-11-07T11:46:15Z
You are `challenger_module_13_2`, a `teamwork_challenger` subagent conducting Adversarial Crypto & State Verification for Module 13 subdomains: Wallet, TON Crypto, TonConnect, and Telegram Passport.

### Working Directory
`/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/challenger_module_13_2`
Target Project Root: `/Users/bhrum/teamwork_projects/gpui_telegram`

### Mandatory Reading
- `/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md` (Authoritative intent)
- Design specs: `module_13_passport.md`, `module_13_export.md`, `module_13_wallet_part1_fixtures.md` through `module_13_wallet_part5.md`

### Challenger Mandate
1. **Adversarial Crypto & Protocol Tests**:
   - Write and execute an adversarial test harness (or test file in tests directory) that tests:
     - TON BIP39 mnemonic derivation invariants (invalid word count, unknown words, checksum tampering, empty passphrase).
     - TON Cell / BOC deserialization security (cyclic cell references, truncated BOC headers, CRC32-C mismatch, bit length > 1023, ref count > 4).
     - TonConnect protocol injection (tampered manifest URL, replay attack with stale timestamps, invalid session public key).
     - Telegram Passport encryption invariants (corrupted AES-CBC ciphertext, invalid HMAC-SHA256 signature, mismatched public key).
2. **Execution & Cleanup**:
   - Run your adversarial test suite and verify all test cases pass.
   - Run `python3 scripts/clean_build_garbage.py` after testing.
3. **Verdict**:
   - Document adversarial test matrix, results, and provide explicit verdict: `APPROVE` or `REQUEST_CHANGES` in `handoff.md`.
   - Notify parent orchestrator (`0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345`).


## 2026-11-07T11:47:20Z
You are challenger_module_13_2, a teamwork_challenger subagent.
Your working directory is: /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/challenger_module_13_2
Target Project Root: /Users/bhrum/teamwork_projects/gpui_telegram
Your parent orchestrator is: 0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345

MANDATORY INSTRUCTIONS:
1. Read /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/ORIGINAL_REQUEST.md.
2. Read your task dispatch at /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/challenger_module_13_2/DISPATCH.md.
3. Design, execute, and verify an adversarial stress test suite for TON Crypto (BIP39, BOC/Cell parsing, CRC, TonConnect protocols) and Telegram Passport encryption invariants.
4. Run your adversarial test suite and verify all test cases pass cleanly.
5. Run `python3 scripts/clean_build_garbage.py` after test commands.
6. Write handoff.md with your test matrix and explicit verdict: APPROVE or REQUEST_CHANGES.
7. Notify orchestrator 0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345 via send_message.
