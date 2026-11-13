# Module Specification & Design Documents

This directory contains the detailed C++ upstream reading, state machine mapping, and GPUI/Rust design documents for each of the 13 sequential Telegram reconstruction modules per **Mandate #15 (Module-by-Module Closed Loop)**.

## 4-Step Mandatory Module Lifecycle

For each module $k$:
1. **Step A — Deep C++ Source Reading & Design Doc (`源码对照阅读与理解落地`)**:
   - Read the real upstream C++ `.h`/`.cpp`/`.style` files in `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/<subsystem>/`.
   - Author `module_<nn>_<name>.md` recording upstream class/struct definitions, state transitions, algorithms, `.style` metrics, and ledger mappings.
2. **Step B — Genuine Rust + GPUI Implementation (`真实代码实现`)**:
   - Replace all script-generated boilerplate `{ id, label, value }` patterns with genuine Rust domain and GPUI view implementations faithful to upstream C++.
3. **Step C — Test Build Packaging (`编译发布测试版`)**:
   - Compile binaries (`cargo build --bin gpui_telegram_desktop --bin gpui_telegram_server`), clean build garbage (`clean_build_garbage.py`), and run unit/integration tests.
4. **Step D — Module Acceptance Gate (`单模块独立验收`)**:
   - Enforce 0 boilerplate files matching Patterns A/B/C/D in module. Pass Gate 6.2 before marking module `ACCEPTED` in `MODULE_BY_MODULE_ROADMAP.md`.

## Module Index

| Module | Name | Design Document | Shards | Status |
|---|---|---|---|---|
| Module 01 | Core & Base | `module_01_core_base.md` | `core.json` | PLANNED |
| Module 02 | MTProto & API | `module_02_mtproto_api.md` | `mtproto.json`, `api.json` | PLANNED |
| Module 03 | Storage & Data | `module_03_storage_data.md` | `storage.json`, `data.json` | PLANNED |
| Module 04 | Main & Window & Layout | `module_04_main_window_layout.md` | `main.json`, `window.json`, `layout.json` | PLANNED |
| Module 05 | Dialogs | `module_05_dialogs.md` | `dialogs.json` | PLANNED |
| Module 06 | History | `module_06_history.md` | `history.json` | PLANNED |
| Module 07 | Chat Helpers & Inline Bots | `module_07_chat_helpers_inline_bots.md` | `chat_helpers.json`, `inline_bots.json` | PLANNED |
| Module 08 | UI & Boxes | `module_08_ui_boxes.md` | `ui.json`, `boxes.json` | PLANNED |
| Module 09 | Media & Editor | `module_09_media_editor.md` | `media.json`, `editor.json` | PLANNED |
| Module 10 | Calls | `module_10_calls.md` | `calls.json` | PLANNED |
| Module 11 | Info & Settings | `module_11_info_settings.md` | `info.json`, `settings.json` | PLANNED |
| Module 12 | Intro & Lang & Platform | `module_12_intro_lang_platform.md` | `intro.json`, `lang.json`, `platform.json` | PLANNED |
| Module 13 | Statistics & Payments & Passport & Export & IV & Wallet | `module_13_advanced_services.md` | `statistics.json`, `payments.json`, `passport.json`, `export.json`, `iv.json`, `wallet.json` | PLANNED |
