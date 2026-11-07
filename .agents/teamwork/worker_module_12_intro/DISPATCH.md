# Task Assignment — worker_module_12_intro

## Working Directory
`/Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_12_intro`

## Scope: Shard `intro.json` (38 upstream files, 23 targets)
Assigned Upstream Sources in `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/`:
- `countries/countries_instance.{h,cpp}`
- `countries/countries_manager.{h,cpp}`
- `intro/intro.style`
- `intro/intro_code.{h,cpp}`
- `intro/intro_code_input.{h,cpp}`
- `intro/intro_email.{h,cpp}`
- `intro/intro_password_check.{h,cpp}`
- `intro/intro_phone.{h,cpp}`
- `intro/intro_qr.{h,cpp}`
- `intro/intro_signup.{h,cpp}`
- `intro/intro_start.{h,cpp}`
- `intro/intro_step.{h,cpp}`
- `intro/intro_widget.{h,cpp}`
- `webauthn/cable.{h,cpp}` (or cable.h)
- `webauthn/cable_box.{h,cpp}`
- `webauthn/cable_ceremony.cpp`
- `webauthn/cable_core.{h,cpp}`
- `webauthn/cable_scanner.h`
- `webauthn/cable_scanner_linux.cpp`
- `webauthn/cable_scanner_mac.mm`
- `webauthn/cable_scanner_win.cpp`
- `webauthn/org.bluez.xml`
- `webauthn/webauthn_common.{h,cpp}`

Target Files (23 files):
- `crates/gpui_telegram_desktop/src/views/auth/countries_instance.rs`
- `crates/gpui_telegram_desktop/src/views/auth/countries_manager.rs`
- `crates/gpui_telegram_desktop/src/theme/intro/intro_style.rs`
- `crates/gpui_telegram_desktop/src/views/auth/intro_code.rs`
- `crates/gpui_telegram_desktop/src/views/auth/intro_code_input.rs`
- `crates/gpui_telegram_desktop/src/views/auth/intro_email.rs`
- `crates/gpui_telegram_desktop/src/views/auth/intro_password_check.rs`
- `crates/gpui_telegram_desktop/src/views/auth/intro_phone.rs`
- `crates/gpui_telegram_desktop/src/views/auth/intro_qr.rs`
- `crates/gpui_telegram_desktop/src/views/auth/intro_signup.rs`
- `crates/gpui_telegram_desktop/src/views/auth/intro_start.rs`
- `crates/gpui_telegram_desktop/src/views/auth/intro_step.rs`
- `crates/gpui_telegram_desktop/src/views/auth/intro_widget.rs`
- `crates/gpui_telegram_desktop/src/views/auth/cable.rs`
- `crates/gpui_telegram_desktop/src/views/auth/cable_box.rs`
- `crates/gpui_telegram_desktop/src/views/auth/cable_ceremony.rs`
- `crates/gpui_telegram_desktop/src/views/auth/cable_core.rs`
- `crates/gpui_telegram_desktop/src/views/auth/cable_scanner.rs`
- `crates/gpui_telegram_desktop/src/views/auth/cable_scanner_linux.rs`
- `crates/gpui_telegram_desktop/src/views/auth/cable_scanner_mac.rs`
- `crates/gpui_telegram_desktop/src/views/auth/cable_scanner_win.rs`
- `crates/gpui_telegram_desktop/assets/org.bluez_xml.rs`
- `crates/gpui_telegram_desktop/src/views/auth/webauthn_common.rs`

## Required Actions
1. **Step 12A — Author Design Spec `docs/specs/modules/module_12_intro.md`**:
   - Enumerate ALL 38 upstream files with 4-Layer Anti-Bluffing Proof (Mandate #18).
   - Zero omission: all 38 files must appear in the manifest table.
2. **Step 12B — Replace all Boilerplate Stubs**:
   - Inspect all 23 target files. Eradicate all Patterns A, B, C, D stubs.
   - Implement genuine GPUI auth wizard components (QR code scan, SMS code inputs, phone number validation, 2FA password verification, WebAuthn/caBLE handshake).
   - Keep write calls <= 200 lines (Mandate #20).
3. **Workspace Check & Clean**:
   - Verify `cargo check --workspace` passes cleanly.
   - Run `python3 scripts/clean_build_garbage.py`.
4. Deliver `progress.md` and `handoff.md`. Send completion message back.


## 2026-11-07T09:40:30Z
[Message] timestamp=2026-11-07T09:40:30Z sender=0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345 priority=MESSAGE_PRIORITY_HIGH content=You are worker_module_12_intro.
Your working directory is: /Users/bhrum/teamwork_projects/gpui_telegram/.agents/teamwork/worker_module_12_intro
The target project root is: /Users/bhrum/teamwork_projects/gpui_telegram
Your parent conversation ID is: 0fd1ccb2-1122-4f5c-ab3f-9de36c0a0345
