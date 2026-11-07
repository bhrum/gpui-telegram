# Specification 01: Telegram 1:1 Feature Inventory & Sharded Source-File Parity Ledger

**Document ID**: `SPEC-01-FEATURE-INVENTORY-LEDGER`  
**Revision**: `3.0.0` (Milestone 1, Explorer Gen 3)  
**Date**: `2026-11-06`  
**Status**: Authoritative Technical Specification & Source Ledger Blueprint  
**Target Repository**: `/Users/bhrum/teamwork_projects/gpui_telegram`  
**Target Path**: `docs/specs/01-telegram-1to1-feature-inventory.md`  

---

## 1. Executive Summary & Mandatory Architecture Directives

This specification establishes the authoritative, production-grade 1:1 feature inventory and source-file parity mapping baseline for the complete reconstruction of **Telegram Desktop** in **GPUI 0.2.2 (Rust)** backed by an **independent, self-hosted Rust server (`gpui_telegram_server`)**.

### 1.1 Strict Compliance with User Mandates #1 through #7

1. **Mandate #1 — Full Inheritance of Existing Rust Code Assets**:
   - Zero Rust code discarded: All existing production-grade Rust crates from `/Users/bhrum/Documents/ChatGPT/fabushi-unified-design-20261105` are completely inherited and placed into `inherited/`:
     * `source/host` (`mahayana-host-runtime` & `mahayana-app-host-runtime` with 115 passing contract tests)
     * `native/mahayana-messaging` (`fabushi-messaging-core` high-throughput message pipeline, WAL storage, secret chat crypto, 10 contract tests)
     * `source/mahayana/mahayana-rs` (`SandAgentDb`, SQLite WAL engine, kernel bridge)
     * `source/mahayana/codex-rs` (AST parser, code indexing, tree-sitter runtime)
     * `source/node-agent-coordinator` (CUA computer use, VNC streaming, TDRP extensions)
     * `source/box-exec-daemon` (sandboxed process runner)
     * `fabushi-backend` (protocol contracts & network types)
   - GPUI replaces the legacy Electron + React/TypeScript presentation layer (`desktop/` and `frontend/`), directly binding to this proven Rust engine.

2. **Mandates #2 & #3 — Upstream Telegram Desktop Parity Focus**:
   - 100% focused on upstream Telegram Desktop source tree: `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/`.
   - OpenClaw source ledger is paused/skipped; inherited Rust code in `inherited/` is preserved as-is.
   - All 52 Telegram native domains (`TCG-N01` through `TCG-N52`) and all UI/UX components are derived directly from upstream C++ implementations.

3. **Mandate #4 — 100% Full Source-File Enumeration Ledger (Zero Omission)**:
   - Every single file in `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/` — **exactly 3,441 source and `.style` files** across all 28 subdirectories — is strictly cataloged and mapped.
   - Zero omitted (`omitted = 0`), zero unmapped (`unmapped = 0`), zero unknown (`unknown = 0`).

4. **Mandate #5 — Anti-Symbol-Collapse & Anti-Monolith File Structure Rules**:
   - **Anti-Symbol-Collapse**: Except for companion `.h` + `.cpp` pairs, multiple distinct upstream business/UI source files MUST NOT collapse into generic catch-all symbols. Every distinct upstream file maps to an explicit, dedicated Rust `struct`, `enum` variant, or dedicated handler function (`target_symbol`).
   - **Anti-Monolith Architecture**: No module may be collapsed into a single monolithic `.rs` file. Target crates (particularly `gpui_telegram_desktop`, `gpui_telegram_client`, `gpui_telegram_server`) mirror upstream directory hierarchies with dedicated multi-file submodule structures (e.g., `src/boxes/*.rs`, `src/settings/*.rs`, `src/dialogs/*.rs`, `src/history/media/*.rs`).

5. **Mandate #6 — Sharded Ledger Architecture**:
   - Prevents agent context blowup and concurrent write collision by establishing a lightweight Master Index (`docs/specs/ledger/index.json`) and 28 independent subsystem shards in `docs/specs/ledger/shards/<subsystem>.json`.
   - Aggregated and mechanically validated by `scripts/verify_ledger_closure.py`.

6. **Mandate #7 — Deep Source Reading & Real Symbol Extraction**:
   - Every mapping in this specification is derived from direct source code reading of upstream C++ classes, structs, methods, signal streams (`rpl::producer`), and `.style` tokens. No blind filename guessing.

---

## 2. Complete 58-Feature Inventory & Subsystem Mapping Catalog

The feature inventory encompasses **58 comprehensive features**:
- **52 Telegram Native Feature Domains** (`TCG-N01` through `TCG-N52`)
- **6 Advanced Inherited Capability Domains** (`CAP-01` through `CAP-06`)

### 2.1 Complete Feature Directory Matrix

| Feature ID | Feature Name | Upstream `tdesktop` Directory Anchor | Target Rust Crate(s) | Sharded Ledger Shard ID | Disposition |
|---|---|---|---|---|---|
| `TCG-N01` | Auth & Registration | `api/`, `intro/`, `webauthn/` | `gpui_telegram_core`, `gpui_telegram_server`, `gpui_telegram_client` | `api.json`, `intro.json`, `webauthn.json` | `rust_domain_or_server_1to1` |
| `TCG-N02` | Sessions & Security (2FA) | `core/`, `settings/sections/`, `storage/` | `gpui_telegram_core`, `gpui_telegram_server` | `core.json`, `settings.json`, `storage.json` | `rust_domain_or_server_1to1` |
| `TCG-N03` | Dialogs & Folder Tabs | `dialogs/`, `boxes/filters/` | `gpui_telegram_desktop`, `gpui_telegram_client` | `dialogs.json`, `boxes.json` | `gpui_ui_1to1` + `inherited_reused` |
| `TCG-N04` | Messages & Chat Bubbles | `history/`, `ui/chat/`, `data/` | `gpui_telegram_desktop`, `gpui_telegram_core` | `history.json`, `ui.json`, `data.json` | `gpui_ui_1to1` + `inherited_reused` |
| `TCG-N05` | Input Composer & IME Safety | `chat_helpers/compose/`, `history/view/controls/` | `gpui_telegram_desktop` | `chat_helpers.json`, `history.json` | `gpui_ui_1to1` |
| `TCG-N06` | Media & Attachments Pipeline | `storage/`, `data/`, `ui/chat/attach/` | `gpui_telegram_media`, `gpui_telegram_server` | `storage.json`, `data.json`, `ui.json` | `rust_domain_or_server_1to1` |
| `TCG-N07` | Voice Notes & 5-Bit Waveforms | `media/audio/`, `data/data_document.cpp` | `gpui_telegram_media`, `gpui_telegram_desktop` | `media.json`, `data.json` | `gpui_ui_1to1` + `rust_media` |
| `TCG-N08` | Audio Player Floating Bar | `media/player/`, `history/view/controls/` | `gpui_telegram_desktop`, `gpui_telegram_media` | `media.json`, `history.json` | `gpui_ui_1to1` |
| `TCG-N09` | Round Video Notes | `media/clip/`, `editor/video/` | `gpui_telegram_media`, `gpui_telegram_desktop` | `media.json`, `editor.json` | `gpui_ui_1to1` + `rust_media` |
| `TCG-N10` | Media Viewer Lightbox | `media/view/`, `media/stories/` | `gpui_telegram_desktop` | `media.json` | `gpui_ui_1to1` |
| `TCG-N11` | Groups & Supergroups | `data/`, `boxes/peers/`, `api/` | `gpui_telegram_client`, `gpui_telegram_server` | `data.json`, `boxes.json`, `api.json` | `rust_domain_or_server_1to1` |
| `TCG-N12` | Channels & Broadcasts | `data/`, `info/channel_statistics/`, `api/` | `gpui_telegram_client`, `gpui_telegram_server` | `data.json`, `info.json`, `api.json` | `rust_domain_or_server_1to1` |
| `TCG-N13` | Forum Topics & Threads | `data/data_forum_topic.cpp`, `dialogs/ui/` | `gpui_telegram_desktop`, `gpui_telegram_server` | `data.json`, `dialogs.json` | `gpui_ui_1to1` + `inherited_reused` |
| `TCG-N14` | Reactions & Emoji Popups | `history/view/reactions/`, `ui/controls/` | `gpui_telegram_desktop` | `history.json`, `ui.json` | `gpui_ui_1to1` |
| `TCG-N15` | Stickers & Lottie Animations | `chat_helpers/`, `data/stickers/` | `gpui_telegram_desktop`, `gpui_telegram_media` | `chat_helpers.json`, `data.json` | `gpui_ui_1to1` |
| `TCG-N16` | Polls & Quizzes | `poll/`, `boxes/polls.style` | `gpui_telegram_desktop`, `gpui_telegram_proto` | `poll.json`, `boxes.json` | `gpui_ui_1to1` + `inherited_reused` |
| `TCG-N17` | Pinned Messages Banner | `history/view/history_view_pinned_bar.cpp` | `gpui_telegram_desktop` | `history.json` | `gpui_ui_1to1` |
| `TCG-N18` | Scheduled & Silent Send | `history/view/controls/history_view_send_button.cpp` | `gpui_telegram_desktop`, `gpui_telegram_server` | `history.json`, `chat_helpers.json` | `gpui_ui_1to1` + `server` |
| `TCG-N19` | Edit, Delete & Tombstones | `history/history_item.cpp`, `api/api_sending.cpp` | `gpui_telegram_proto`, `gpui_telegram_server` | `history.json`, `api.json` | `rust_domain_or_server_1to1` |
| `TCG-N20` | PTS Sequence Sync Engine | `api/api_updates.cpp`, `storage/storage_account.cpp` | `gpui_telegram_proto`, `gpui_telegram_server` | `api.json`, `storage.json` | `rust_domain_or_server_1to1` |
| `TCG-N21` | Full-Text Search Engine | `storage/storage_databases.cpp`, `dialogs/` | `gpui_telegram_server`, `gpui_telegram_desktop` | `storage.json`, `dialogs.json` | `rust_domain_or_server_1to1` |
| `TCG-N22` | Shared Media Archive Tabs | `info/media/`, `overview/` | `gpui_telegram_desktop`, `gpui_telegram_client` | `info.json`, `overview.json` | `gpui_ui_1to1` |
| `TCG-N23` | Context Inspector Drawer | `info/`, `info/profile/`, `info/members/` | `gpui_telegram_desktop` | `info.json` | `gpui_ui_1to1` |
| `TCG-N24` | 1-on-1 VoIP Audio/Video | `calls/`, `calls/ui/`, `calls/group/` | `gpui_telegram_webrtc`, `gpui_telegram_desktop` | `calls.json` | `rust_domain_or_server_1to1` |
| `TCG-N25` | Group Voice/Video Chats | `calls/group/`, `calls/group/ui/` | `gpui_telegram_webrtc`, `gpui_telegram_desktop` | `calls.json` | `rust_domain_or_server_1to1` |
| `TCG-N26` | Low-Latency Screen Share | `calls/group/ui/desktop_capture_choose_source.cpp` | `gpui_telegram_webrtc`, `gpui_telegram_desktop` | `calls.json` | `rust_domain_or_server_1to1` |
| `TCG-N27` | Inline Keyboards & Bots | `inline_bots/`, `boxes/compose_ai_box.cpp` | `gpui_telegram_proto`, `gpui_telegram_desktop` | `inline_bots.json`, `boxes.json` | `gpui_ui_1to1` + `proto` |
| `TCG-N28` | Mini Apps Sandboxed Frame | `boxes/url_auth_box_content.cpp`, `ui/` | `gpui_telegram_desktop` | `boxes.json`, `ui.json` | `gpui_ui_1to1` |
| `TCG-N29` | Contacts & Address Book | `boxes/add_contact_box.cpp`, `data/` | `gpui_telegram_desktop`, `gpui_telegram_client` | `boxes.json`, `data.json` | `gpui_ui_1to1` |
| `TCG-N30` | User Profile & Presence | `profile/`, `info/profile/`, `data/` | `gpui_telegram_desktop`, `gpui_telegram_proto` | `profile.json`, `info.json`, `data.json` | `gpui_ui_1to1` |
| `TCG-N31` | Chat Settings & Theme Engine | `settings/`, `window/themes/` | `gpui_telegram_desktop` | `settings.json`, `window.json` | `gpui_ui_1to1` |
| `TCG-N32` | System Notifications & Badges| `platform/mac/`, `data/notify/` | `gpui_telegram_desktop`, `gpui_telegram_client` | `platform.json`, `data.json` | `gpui_ui_1to1` |
| `TCG-N33` | Privacy & Security Controls | `settings/sections/settings_privacy_controllers.cpp`| `gpui_telegram_server`, `gpui_telegram_desktop` | `settings.json` | `rust_domain_or_server_1to1` |
| `TCG-N34` | Storage & Cache Management | `settings/sections/settings_local_storage.cpp` | `gpui_telegram_client`, `gpui_telegram_desktop` | `settings.json`, `storage.json` | `rust_domain_or_server_1to1` |
| `TCG-N35` | Proxy & Network Diagnostics | `boxes/connection_box.cpp`, `mtproto/` | `gpui_telegram_client`, `gpui_telegram_desktop` | `boxes.json`, `mtproto.json` | `rust_domain_or_server_1to1` |
| `TCG-N36` | Draft Sync & Realtime Preview| `dialogs/ui/dialogs_layout.cpp`, `data/` | `gpui_telegram_client`, `gpui_telegram_desktop` | `dialogs.json`, `data.json` | `gpui_ui_1to1` |
| `TCG-N37` | Archive & Custom Filter Tabs | `dialogs/`, `boxes/filters/` | `gpui_telegram_desktop`, `gpui_telegram_client` | `dialogs.json`, `boxes.json` | `gpui_ui_1to1` |
| `TCG-N38` | Stories Top Strip & Viewer | `media/stories/`, `data/data_stories.cpp` | `gpui_telegram_desktop`, `gpui_telegram_media` | `media.json`, `data.json` | `gpui_ui_1to1` |
| `TCG-N39` | Secret Chats & E2EE Ratchet | `storage/details/storage_file_utilities.cpp` | `gpui_telegram_core`, `gpui_telegram_proto` | `storage.json`, `core.json` | `inherited_reused` |
| `TCG-N40` | Self-Destruct Timers (TTL) | `ui/controls/ttl_media.style`, `history/` | `gpui_telegram_core`, `gpui_telegram_desktop` | `ui.json`, `history.json` | `rust_domain_or_server_1to1` |
| `TCG-N41` | Location & Live Location | `ui/controls/location_picker.style` | `gpui_telegram_desktop`, `gpui_telegram_proto` | `ui.json` | `gpui_ui_1to1` |
| `TCG-N42` | Web Link Previews (OpenGraph)| `history/view/history_view_webpage.cpp` | `gpui_telegram_server`, `gpui_telegram_desktop` | `history.json` | `rust_domain_or_server_1to1` |
| `TCG-N43` | Export Chat History (JSON) | `export/`, `export/view/`, `export/output/` | `gpui_telegram_client`, `gpui_telegram_desktop` | `export.json` | `rust_domain_or_server_1to1` |
| `TCG-N44` | Language & Localization | `lang/`, `lang/lang_keys.h` | `gpui_telegram_desktop` | `lang.json` | `gpui_ui_1to1` |
| `TCG-N45` | Telegram Keybindings | `settings/settings_key_navigation.cpp` | `gpui_telegram_desktop` | `settings.json` | `gpui_ui_1to1` |
| `TCG-N46` | Custom Window Frame (Metal) | `window/window_main_menu.cpp`, `platform/mac/` | `gpui_telegram_desktop` | `window.json`, `platform.json` | `gpui_ui_1to1` |
| `TCG-N47` | Drag-and-Drop & Clipboard | `boxes/send_files_box.cpp`, `window/` | `gpui_telegram_desktop` | `boxes.json`, `window.json` | `gpui_ui_1to1` |
| `TCG-N48` | Auto-Download Policy Rules | `settings/sections/settings_data_and_storage.cpp`| `gpui_telegram_client` | `settings.json` | `rust_domain_or_server_1to1` |
| `TCG-N49` | Multi-Account Switching | `window/window_main_menu.cpp`, `core/` | `gpui_telegram_desktop`, `gpui_telegram_client` | `window.json`, `core.json` | `gpui_ui_1to1` |
| `TCG-N50` | Account Scope Generation Fencing| `data/data_session.cpp`, `core/` | `gpui_telegram_core`, `gpui_telegram_client` | `data.json`, `core.json` | `rust_domain_or_server_1to1` |
| `TCG-N51` | Chunked Media Upload Pipeline| `storage/file_upload.cpp` | `gpui_telegram_media`, `gpui_telegram_server` | `storage.json` | `rust_domain_or_server_1to1` |
| `TCG-N52` | Range-Request Media Stream | `data/data_streaming.cpp` | `gpui_telegram_server`, `gpui_telegram_media` | `data.json`, `media.json` | `rust_domain_or_server_1to1` |
| `CAP-01` | AI Multi-Agent ACP Runner | `source/host` (ACP runner, thinking loop) | `gpui_telegram_agent`, `gpui_telegram_desktop` | `inherited_host.json` | `inherited_reused` + `gpui_ui` |
| `CAP-02` | Global Task Center & Cron FSM| `source/host` (7-state task FSM, 5-field cron)| `gpui_telegram_agent`, `gpui_telegram_desktop` | `inherited_host.json` | `inherited_reused` + `gpui_ui` |
| `CAP-03` | MCP & Plugin Tools Ecosystem | `source/host` (Stdio/SSE MCP, PermissionDock) | `gpui_telegram_agent`, `gpui_telegram_desktop` | `inherited_host.json` | `inherited_reused` + `gpui_ui` |
| `CAP-04` | Remote Computer Control (CUA) | `source/node-agent-coordinator` (VNC frame stream)| `gpui_telegram_agent`, `gpui_telegram_desktop` | `inherited_node.json` | `inherited_reused` + `gpui_ui` |
| `CAP-05` | Full-Duplex Talk Mode & Energy | `source/host` (realtime audio VAD, barge-in) | `gpui_telegram_media`, `gpui_telegram_desktop` | `inherited_host.json` | `inherited_reused` + `gpui_ui` |
| `CAP-06` | A2UI Declarative Cards | `native/mahayana-messaging` (rich cards) | `gpui_telegram_proto`, `gpui_telegram_desktop` | `inherited_messaging.json` | `inherited_reused` + `gpui_ui` |

---

## 3. Deep Source-Reading Feature Specifications (TCG-N01 through TCG-N20)

### 3.1 TCG-N01: Authentication, Registration & Passkeys
- **Upstream Files**:
  * `Telegram/SourceFiles/intro/intro_code.cpp`, `intro/intro_phone.cpp`, `intro/intro_qr.cpp`, `intro/intro_signup.cpp`
  * `Telegram/SourceFiles/api/api_authorizations.cpp`, `api/api_authorizations.h`
  * `Telegram/SourceFiles/webauthn/webauthn_box.cpp`, `webauthn/webauthn_box.h`
- **Real Upstream Symbols & State Machines**:
  * Classes: `Intro::CodeWidget`, `Intro::PhoneWidget`, `Intro::QrWidget`, `Api::Authorizations`, `WebauthnBox`
  * Methods: `Intro::CodeWidget::submitCode()`, `Api::Authorizations::request()`, `WebauthnBox::startAuthentication()`
  * State FSM: `Unauthenticated -> SubmitIdentifier(Phone/Email/QR) -> ChallengeCode(SMS/App/Email) -> CloudPasswordCheck(2FA) -> Authenticated(SessionKey)`
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_core/src/auth.rs`, `crates/gpui_telegram_server/src/auth/`
  * Target Symbols: `struct AuthSessionToken`, `enum AuthChallengeStep`, `struct WebAuthnCredential`, `fn handle_passkey_login()`
  * Inherited Asset: `source/node-agent-coordinator/src/webauthn.rs`, `source/host/src/device_pairing_tokens.rs`
  * Anti-Collapse Mapping: `Intro::PhoneWidget` -> `PhoneInputStep`, `Intro::CodeWidget` -> `VerificationCodeStep`, `WebauthnBox` -> `PasskeyAuthModal`
  * Verification Test: `tests/contract_auth_handshake.rs`

### 3.2 TCG-N02: Sessions, Devices & Cloud 2FA Password
- **Upstream Files**:
  * `Telegram/SourceFiles/core/two_factor_auth.cpp`, `core/two_factor_auth.h`
  * `Telegram/SourceFiles/settings/sections/settings_active_sessions.cpp`
  * `Telegram/SourceFiles/settings/sections/settings_active_sessions.style`
  * `Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_email_confirm_widget.cpp`
- **Real Upstream Symbols & State Machines**:
  * Classes: `Core::TwoFactorAuth`, `Settings::ActiveSessions`, `Settings::CloudPassword`
  * Methods: `Core::TwoFactorAuth::checkPassword()`, `Settings::ActiveSessions::terminateSession(sessionId)`
  * State FSM: `ActiveSessionList -> SelectSession -> TerminateRequest -> InvalidationEventBroadcast`
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_server/src/auth/session_manager.rs`, `crates/gpui_telegram_desktop/src/views/settings/sessions.rs`
  * Target Symbols: `struct DeviceSessionRecord`, `struct TwoFactorCloudPassword`, `fn terminate_session_by_id()`
  * Inherited Asset: `source/node-agent-coordinator/src/tdrp/settings_cloud_passkeys.rs`, `native/mahayana-messaging/src/device.rs`
  * Anti-Collapse Mapping: `settings_active_sessions.cpp` -> `ActiveSessionsView`, `two_factor_auth.cpp` -> `TwoFactorState`
  * Verification Test: `tests/session_termination_broadcast.rs`

### 3.3 TCG-N03: Dialogs, Chats List & Folder Tabs
- **Upstream Files**:
  * `Telegram/SourceFiles/dialogs/dialogs_widget.cpp`, `dialogs/dialogs_widget.h`
  * `Telegram/SourceFiles/dialogs/dialogs_inner_widget.cpp`, `dialogs/dialogs_inner_widget.h`
  * `Telegram/SourceFiles/dialogs/dialogs_row.cpp`, `dialogs/dialogs_row.h`
  * `Telegram/SourceFiles/dialogs/dialogs.style`, `dialogs/ui/dialogs_layout.style`
  * `Telegram/SourceFiles/boxes/filters/edit_filter_box.cpp`
- **Real Upstream Symbols & Metrics**:
  * Metrics: `defaultDialogRow.height = 62px`, `photoSize = 46px`, `padding = margins(10px, 8px, 10px, 8px)`, `nameLeft = 68px`, `nameTop = 10px`, `textLeft = 68px`, `textTop = 34px`
  * Classes: `Dialogs::Widget`, `Dialogs::InnerWidget`, `Dialogs::Row`, `Dialogs::Key`, `FilterId`
  * Methods: `Dialogs::InnerWidget::paintRow()`, `Dialogs::Widget::applyFilter()`, `Dialogs::Row::updateUnreadBadge()`
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_desktop/src/views/sidebar/` (`dialog_list.rs`, `dialog_row.rs`, `folder_tabs.rs`)
  * Target Symbols: `struct ChatSidebarView`, `struct DialogRowComponent`, `struct FolderTabStrip`, `enum DialogFilterType`
  * Inherited Asset: `native/mahayana-messaging/src/engine_tdrp_dialogs.rs`, `source/node-agent-coordinator/src/tdrp/folders.rs`
  * Anti-Collapse Mapping: `dialogs_widget.cpp` -> `ChatSidebarView`, `dialogs_row.cpp` -> `DialogRowComponent`, `edit_filter_box.cpp` -> `EditFolderModal`
  * Verification Test: `tests/dialog_list_virtual_scroll.rs`

### 3.4 TCG-N04: Messages Pipeline & Chat Bubbles
- **Upstream Files**:
  * `Telegram/SourceFiles/history/history.cpp`, `history/history.h`
  * `Telegram/SourceFiles/history/history_item.cpp`, `history/history_item.h`
  * `Telegram/SourceFiles/history/view/history_view_element.cpp`, `history/view/history_view_message.cpp`
  * `Telegram/SourceFiles/ui/chat/chat.style`, `ui/chat/chat_style.style`
- **Real Upstream Symbols & Metrics**:
  * Metrics: `msgMaxWidth = 430px`, `bubbleRadiusLarge = 16px`, `bubbleRadiusSmall = 4px`, `msgPadding = margins(11px, 8px, 11px, 8px)`, `msgMargin = margins(16px, 6px, 56px, 2px)`
  * Classes: `History`, `HistoryItem`, `HistoryView::Element`, `HistoryView::Message`
  * Methods: `History::addMessage()`, `HistoryView::Message::layout()`, `HistoryView::Message::paint()`
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_desktop/src/views/stage/` (`conversation_stage.rs`, `message_bubble.rs`)
  * Target Symbols: `struct ConversationStageView`, `struct MessageBubbleComponent`, `enum MessageBubbleSide { Incoming, Outgoing }`
  * Inherited Asset: `native/mahayana-messaging/src/message.rs`, `native/mahayana-messaging/src/conversation.rs`
  * Anti-Collapse Mapping: `history_view_message.cpp` -> `MessageBubbleComponent`, `history_item.cpp` -> `MessageDomainItem`
  * Verification Test: `tests/message_bubble_layout.rs`

### 3.5 TCG-N05: Input Composer & IME Protection
- **Upstream Files**:
  * `Telegram/SourceFiles/chat_helpers/compose/compose_show.cpp`, `chat_helpers/compose/compose_show.h`
  * `Telegram/SourceFiles/history/view/controls/history_view_compose_controls.cpp`
  * `Telegram/SourceFiles/ui/widgets/fields.cpp`, `ui/widgets/fields.h`
- **Real Upstream Symbols & Behaviors**:
  * Classes: `HistoryView::Controls::ComposeControls`, `Ui::InputField`, `ChatHelpers::ComposeFeatures`
  * IME Safe Invariant: Never commit or send message when `composition_active == true` (IME Japanese/Chinese candidate active)
  * Growth Limits: Single line min 44px, multi-line auto-expand up to max 192px (approx 8 lines) with custom scrollbar
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_desktop/src/views/composer/canonical_composer.rs`
  * Target Symbols: `struct CanonicalComposerView`, `struct ImeInputBuffer`, `enum ComposerActionType`
  * Anti-Collapse Mapping: `history_view_compose_controls.cpp` -> `CanonicalComposerView`, `fields.cpp` -> `GrowingInputTextArea`
  * Verification Test: `tests/composer_ime_safety.rs`

### 3.6 TCG-N06: Media & Attachments Pipeline
- **Upstream Files**:
  * `Telegram/SourceFiles/storage/file_upload.cpp`, `storage/file_download.cpp`
  * `Telegram/SourceFiles/data/data_photo.cpp`, `data/data_document.cpp`
  * `Telegram/SourceFiles/ui/chat/attach/attach_prepare.cpp`
- **Real Upstream Symbols & Logic**:
  * Classes: `Storage::UploadPreparation`, `Storage::DownloadManager`, `Data::Photo`, `Data::Document`
  * Chunk Sizes: 512KB for files < 100MB; 1MB for files >= 100MB up to 2GB/4GB
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_media/src/chunked_upload.rs`, `crates/gpui_telegram_server/src/media/`
  * Target Symbols: `struct MediaUploadPipeline`, `struct ChunkPartToken`, `struct CasBlobStore`
  * Inherited Asset: `native/mahayana-messaging/src/blob_store.rs`, `native/mahayana-messaging/src/media_cache.rs`
  * Anti-Collapse Mapping: `file_upload.cpp` -> `ChunkedUploader`, `file_download.cpp` -> `ChunkedDownloader`
  * Verification Test: `tests/media_chunked_upload.rs`

### 3.7 TCG-N07: Voice Notes & 5-Bit Waveform Codec
- **Upstream Files**:
  * `Telegram/SourceFiles/media/audio/media_audio_capture.cpp`, `media/audio/media_audio_capture.h`
  * `Telegram/SourceFiles/data/data_document.cpp` (lines 1390–1434: `documentWaveformEncode5bit`, `documentWaveformDecode5bit`)
  * `Telegram/SourceFiles/history/view/controls/history_view_voice_record_bar.style`
- **Real Upstream Symbols & Bitwise Algorithm**:
  * Dimensions: `historyRecordLockSize = size(75px, 133px)`, `historyRecordLockIconSize = size(14px, 17px)`
  * Codec: Exactly 100 samples per voice note, scaled 0..31 (5-bit). Packed into `(100 * 5 + 7) / 8 = 63 bytes`.
  * Encoding Loop: `((sample & 0x1F) << bitShift)` merged across byte boundaries via `u16` bitwise OR.
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_media/src/waveform.rs`, `crates/gpui_telegram_desktop/src/views/composer/voice_record.rs`
  * Target Symbols: `pub fn document_waveform_encode_5bit(samples: &[u8]) -> Vec<u8>`, `pub fn document_waveform_decode_5bit(encoded: &[u8]) -> Vec<u8>`, `struct VoiceRecordBarComponent`
  * Anti-Collapse Mapping: `media_audio_capture.cpp` -> `AudioCaptureEngine`, `history_view_voice_record_bar.cpp` -> `VoiceRecordGestureOverlay`
  * Verification Test: `tests/waveform_5bit_codec_roundtrip.rs`

### 3.8 TCG-N08: Audio Player Floating Dock
- **Upstream Files**:
  * `Telegram/SourceFiles/media/player/media_player_widget.cpp`, `media/player/media_player_instance.cpp`
  * `Telegram/SourceFiles/media/player/media_player.style`
- **Real Upstream Symbols & Metrics**:
  * Classes: `Media::Player::Widget`, `Media::Player::Instance`
  * Features: Play/Pause, 0.5x..2.0x playback speed switcher, seek bar scrubbing, track title & artist label
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_desktop/src/views/media/floating_audio_player.rs`
  * Target Symbols: `struct FloatingAudioPlayerDock`, `struct PlaybackSpeedToggle`, `enum AudioPlayerState`
  * Anti-Collapse Mapping: `media_player_widget.cpp` -> `FloatingAudioPlayerDock`
  * Verification Test: `tests/audio_player_scrub.rs`

### 3.9 TCG-N09: Round Video Notes
- **Upstream Files**:
  * `Telegram/SourceFiles/media/clip/media_clip_reader.cpp`, `editor/video/editor_video_crop.cpp`
  * `Telegram/SourceFiles/ui/chat/chat.style` (`maxVideoMessageSize: 240px`)
- **Real Upstream Symbols & Rendering**:
  * Classes: `Media::Clip::Reader`, `HistoryView::RoundVideo`
  * Geometry: Circular mask rendering at 240px diameter with outer elapsed time progress circle ring
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_desktop/src/components/cards/card_video_note.rs`
  * Target Symbols: `struct RoundVideoNoteCard`, `struct VideoProgressRing`
  * Anti-Collapse Mapping: `media_clip_reader.cpp` -> `ClipVideoReader`
  * Verification Test: `tests/round_video_rendering.rs`

### 3.10 TCG-N10: Media Viewer Lightbox
- **Upstream Files**:
  * `Telegram/SourceFiles/media/view/media_view_overlay_widget.cpp`, `media/view/media_view_pip_rhi.cpp`
  * `Telegram/SourceFiles/media/view/media_view.style`
- **Real Upstream Symbols & Interactions**:
  * Classes: `Media::View::OverlayWidget`, `Media::View::Pip`
  * Gestures: Smooth mouse wheel zoom, click-drag pan, left/right carousel arrow navigation, ESC to dismiss
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_desktop/src/views/overlays/media_viewer_lightbox.rs`
  * Target Symbols: `struct MediaViewerLightboxModal`, `struct LightboxZoomController`
  * Anti-Collapse Mapping: `media_view_overlay_widget.cpp` -> `MediaViewerLightboxModal`
  * Verification Test: `tests/lightbox_navigation.rs`

### 3.11 TCG-N11: Groups & Supergroups
- **Upstream Files**:
  * `Telegram/SourceFiles/data/data_chat.cpp`, `data/data_channel.cpp`, `data/data_chat_participant_status.cpp`
  * `Telegram/SourceFiles/boxes/peers/edit_peer_members.cpp`
- **Real Upstream Symbols & Permissions**:
  * Classes: `Data::Chat`, `Data::Channel`, `ChatAdminRights`, `ChatBannedRights`
  * Invariant: Transition from basic group (up to 200) to supergroup (up to 200,000 members) with full admin delegation
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_server/src/groups/`, `crates/gpui_telegram_client/src/groups/`
  * Target Symbols: `struct SupergroupEntity`, `struct AdminRightsBitflags`, `struct MemberRoster`
  * Inherited Asset: `native/mahayana-messaging/src/conversation.rs`, `source/node-agent-coordinator/src/tdrp/moderation.rs`
  * Anti-Collapse Mapping: `edit_peer_members.cpp` -> `GroupMembersEditModal`, `data_channel.cpp` -> `SupergroupRecord`
  * Verification Test: `tests/supergroup_permissions.rs`

### 3.12 TCG-N12: Channels & Broadcasts
- **Upstream Files**:
  * `Telegram/SourceFiles/data/data_channel.cpp`, `info/channel_statistics/info_channel_statistics_widget.cpp`
  * `Telegram/SourceFiles/api/api_chat_invite.cpp`
- **Real Upstream Symbols**:
  * Classes: `Data::Channel`, `Info::ChannelStatistics::Widget`, `Api::ChatInvite`
  * Features: Broadcast messages without replies, linked discussion group comments, public link handles (`@channel`)
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_server/src/channels/`, `crates/gpui_telegram_desktop/src/views/channels/`
  * Target Symbols: `struct BroadcastChannelModel`, `struct ChannelDiscussionBridge`, `struct ChannelStatsSnapshot`
  * Anti-Collapse Mapping: `info_channel_statistics_widget.cpp` -> `ChannelStatisticsDrawer`
  * Verification Test: `tests/channel_broadcast_sync.rs`

### 3.13 TCG-N13: Forum Topics & Thread Hierarchies
- **Upstream Files**:
  * `Telegram/SourceFiles/data/data_forum_topic.cpp`, `data/data_forum.cpp`
  * `Telegram/SourceFiles/dialogs/ui/dialogs_layout.style` (`dialogsForumIcon`, `largeForumTopicIcon: 26px`)
  * `Telegram/SourceFiles/dialogs/dialogs.style` (`forumDialogRow.height = 80px`)
- **Real Upstream Symbols**:
  * Classes: `Data::ForumTopic`, `Data::Forum`, `Dialogs::Row`
  * Methods: `Data::Forum::topicFor(msgId)`, `Data::ForumTopic::applyNewMessage()`
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_server/src/topics/`, `crates/gpui_telegram_desktop/src/views/sidebar/topic_list.rs`
  * Target Symbols: `struct ForumTopicDescriptor`, `struct TopicBreadcrumbBar`, `struct ForumTopicList`
  * Inherited Asset: `source/node-agent-coordinator/src/tdrp/forum_topics.rs`, `native/mahayana-messaging/src/engine_tdrp_topics_channels.rs`
  * Anti-Collapse Mapping: `data_forum_topic.cpp` -> `ForumTopicDescriptor`, `dialogs_forum_row` -> `ForumTopicRowComponent`
  * Verification Test: `tests/forum_topics_routing.rs`

### 3.14 TCG-N14: Reactions & Emoji Popups
- **Upstream Files**:
  * `Telegram/SourceFiles/history/view/reactions/history_view_reactions.cpp`
  * `Telegram/SourceFiles/history/view/reactions/history_view_reactions_strip.cpp`
  * `Telegram/SourceFiles/ui/chat/chat.style` (`reactionMenu`)
- **Real Upstream Symbols**:
  * Classes: `HistoryView::Reactions::Manager`, `HistoryView::Reactions::Strip`
  * Features: Hover reaction quick-bar, emoji picker drawer, aggregated reaction pill badges below chat bubbles
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_desktop/src/components/reactions/reaction_strip.rs`
  * Target Symbols: `struct ReactionQuickStrip`, `struct ReactionPillBadge`, `struct ToggleReactionPayload`
  * Anti-Collapse Mapping: `history_view_reactions_strip.cpp` -> `ReactionQuickStrip`
  * Verification Test: `tests/reaction_pill_toggle.rs`

### 3.15 TCG-N15: Stickers & Lottie Animated Graphics
- **Upstream Files**:
  * `Telegram/SourceFiles/chat_helpers/stickers_list_widget.cpp`, `chat_helpers/stickers_box.cpp`
  * `Telegram/SourceFiles/data/stickers/data_stickers.cpp`
  * `Telegram/SourceFiles/ui/chat/chat.style` (`maxStickerSize: 224px`)
- **Real Upstream Symbols**:
  * Classes: `ChatHelpers::StickersListWidget`, `Data::Stickers`, `StickerSet`
  * Features: Lottie `.tgs` vector animation playback, WebP static/animated stickers, recently used sticker shelf
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_desktop/src/components/cards/card_sticker.rs`, `crates/gpui_telegram_media/src/lottie_player.rs`
  * Target Symbols: `struct StickerCardComponent`, `struct LottieAnimationState`, `struct StickerPickerDrawer`
  * Anti-Collapse Mapping: `stickers_list_widget.cpp` -> `StickerPickerDrawer`, `data_stickers.cpp` -> `StickerSetRegistry`
  * Verification Test: `tests/sticker_card_render.rs`

### 3.16 TCG-N16: Polls & Quizzes
- **Upstream Files**:
  * `Telegram/SourceFiles/poll/poll_data.cpp`, `poll/poll_widget.cpp`
  * `Telegram/SourceFiles/boxes/polls.style`, `history/view/media/history_view_poll.cpp`
- **Real Upstream Symbols**:
  * Classes: `PollData`, `PollWidget`, `HistoryView::Poll`
  * Attributes: `anonymous: bool`, `multiple_answers: bool`, `quiz: bool`, `correct_option_id: Option<usize>`
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_desktop/src/components/cards/card_poll.rs`, `crates/gpui_telegram_proto/src/poll.rs`
  * Target Symbols: `struct PollCardComponent`, `struct PollVoteAction`, `struct PollOptionState`
  * Inherited Asset: `source/node-agent-coordinator/src/tdrp/polls.rs`
  * Anti-Collapse Mapping: `poll_widget.cpp` -> `PollCardComponent`, `poll_data.cpp` -> `PollState`
  * Verification Test: `tests/poll_voting_roundtrip.rs`

### 3.17 TCG-N17: Pinned Messages Banner
- **Upstream Files**:
  * `Telegram/SourceFiles/history/view/history_view_pinned_bar.cpp`, `history/view/history_view_pinned_bar.h`
  * `Telegram/SourceFiles/data/data_peer.cpp`
- **Real Upstream Symbols**:
  * Classes: `HistoryView::PinnedBar`, `Data::PinnedMessages`
  * Interactions: Header sticky banner, tap to scroll stage to pinned message target, pin stack cycling
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_desktop/src/views/stage/pinned_banner.rs`
  * Target Symbols: `struct PinnedMessageBanner`, `struct PinCycleNavigation`
  * Anti-Collapse Mapping: `history_view_pinned_bar.cpp` -> `PinnedMessageBanner`
  * Verification Test: `tests/pinned_banner_cycle.rs`

### 3.18 TCG-N18: Scheduled & Silent Messages
- **Upstream Files**:
  * `Telegram/SourceFiles/history/view/controls/history_view_send_button.cpp`
  * `Telegram/SourceFiles/ui/boxes/choose_date_time.style`, `ui/boxes/calendar_box.style`
- **Real Upstream Symbols**:
  * Classes: `HistoryView::SendButton`, `Ui::ChooseDateTimeBox`
  * Triggers: Right-click send button -> "Send Without Sound" or "Schedule Message" (date/time picker modal)
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_desktop/src/views/composer/schedule_modal.rs`, `crates/gpui_telegram_server/src/messaging/schedule_queue.rs`
  * Target Symbols: `struct ScheduleMessageModal`, `struct ScheduledMessageEntry`, `struct ScheduleQueueWorker`
  * Anti-Collapse Mapping: `choose_date_time.cpp` -> `ScheduleDatePickerModal`
  * Verification Test: `tests/scheduled_message_dispatch.rs`

### 3.19 TCG-N19: Edit, Delete & Tombstone Synchronization
- **Upstream Files**:
  * `Telegram/SourceFiles/api/api_sending.cpp`, `history/history_item.cpp`
  * `Telegram/SourceFiles/history/history_item_components.cpp`
- **Real Upstream Symbols**:
  * Classes: `Api::Sending`, `HistoryItem`, `HistoryMessageEdition`
  * Invariants: 48-hour editing window, edit history badge, soft delete with atomic tombstone broadcast to all devices
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_server/src/messaging/tombstones.rs`, `crates/gpui_telegram_proto/src/events.rs`
  * Target Symbols: `struct MessageEditAction`, `struct MessageTombstoneEvent`, `fn broadcast_tombstone()`
  * Inherited Asset: `native/mahayana-messaging/src/message.rs`
  * Anti-Collapse Mapping: `api_sending.cpp` -> `MessageUpdatePipeline`
  * Verification Test: `tests/message_edit_tombstone_sync.rs`

### 3.20 TCG-N20: Sequence Sync Engine (PTS Architecture)
- **Upstream Files**:
  * `Telegram/SourceFiles/api/api_updates.cpp`, `api/api_updates.h`
  * `Telegram/SourceFiles/storage/storage_account.cpp`
- **Real Upstream Symbols & PTS Protocol**:
  * Classes: `Api::Updates`, `Storage::Account`
  * Methods: `Api::Updates::applyUpdate()`, `Api::Updates::getDifference()`
  * Invariant: Atomic monotonically increasing PTS counter per user/chat. Gap detected (`incoming_pts > client_local_pts + 1`) triggers immediate `SyncGetDifference` query.
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_server/src/sync/pts_engine.rs`, `crates/gpui_telegram_client/src/sync/coordinator.rs`
  * Target Symbols: `struct PtsSequenceCounter`, `struct SyncGetDifferenceRequest`, `struct SyncGetDifferenceResponse`, `fn handle_pts_gap()`
  * Inherited Asset: `native/mahayana-messaging/src/service_tdrp_sync.rs`
  * Anti-Collapse Mapping: `api_updates.cpp` -> `PtsSyncCoordinator`
  * Verification Test: `tests/pts_sequence_catchup.rs`

---

## 4. Deep Source-Reading Feature Specifications (TCG-N21 through TCG-N40)

### 4.1 TCG-N21: Full-Text Search Engine (Tantivy Integration)
- **Upstream Files**:
  * `Telegram/SourceFiles/storage/storage_databases.cpp`, `dialogs/dialogs_key.cpp`
  * `Telegram/SourceFiles/boxes/peer_list_section_index.style`
- **Real Upstream Symbols**:
  * Classes: `Storage::Databases`, `Dialogs::IndexedList`, `Dialogs::SearchQuery`
  * Features: Global message search, in-conversation scoped search, author filtering, sub-millisecond query latency
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_server/src/search/tantivy_indexer.rs`, `crates/gpui_telegram_desktop/src/views/search/universal_search.rs`
  * Target Symbols: `struct TantivyMessageIndex`, `struct SearchHit`, `fn query_messages_with_snippets()`
  * Inherited Asset: `native/mahayana-messaging/src/search.rs`
  * Anti-Collapse Mapping: `storage_databases.cpp` -> `TantivyMessageIndex`, `dialogs_search` -> `SearchInputHeader`
  * Verification Test: `tests/fulltext_search_queries.rs`

### 4.2 TCG-N22: Shared Media Archive Tabs
- **Upstream Files**:
  * `Telegram/SourceFiles/info/media/info_media_widget.cpp`, `info/media/info_media_widget.h`
  * `Telegram/SourceFiles/overview/overview_layout.cpp`
- **Real Upstream Symbols**:
  * Classes: `Info::Media::Widget`, `Overview::Layout`
  * Tabs: Photos, Videos, Files, Audio, Shared Links, Voice Notes
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_desktop/src/views/drawer/media_archive_tab.rs`
  * Target Symbols: `struct SharedMediaArchiveTab`, `enum SharedMediaTypeFilter`
  * Anti-Collapse Mapping: `info_media_widget.cpp` -> `SharedMediaArchiveTab`
  * Verification Test: `tests/shared_media_tabs.rs`

### 4.3 TCG-N23: Context Inspector Drawer (320px Parity)
- **Upstream Files**:
  * `Telegram/SourceFiles/info/info_wrap_widget.cpp`, `info/profile/info_profile_widget.cpp`
  * `Telegram/SourceFiles/info/info.style`, `info/profile/info_profile_top_bar.style`
- **Real Upstream Symbols & Dimensions**:
  * Fixed Width: 320px unified right sliding drawer
  * Classes: `Info::WrapWidget`, `Info::Profile::Widget`, `Info::Members::Widget`
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_desktop/src/views/drawer/context_inspector.rs`
  * Target Symbols: `struct ContextInspectorDrawer`, `enum InspectorActivePane`
  * Anti-Collapse Mapping: `info_wrap_widget.cpp` -> `ContextInspectorDrawer`, `info_profile_widget.cpp` -> `ProfileDetailsPane`
  * Verification Test: `tests/context_inspector_toggle.rs`

### 4.4 TCG-N24: 1-on-1 VoIP Audio & Video Calls
- **Upstream Files**:
  * `Telegram/SourceFiles/calls/calls_call.cpp`, `calls/calls_instance.cpp`
  * `Telegram/SourceFiles/calls/calls_controller_webrtc.cpp`, `calls/calls.style`
- **Real Upstream Symbols**:
  * Classes: `Calls::Call`, `Calls::Instance`, `Calls::ControllerWebRTC`
  * Signaling: SDP Offer/Answer negotiation, ICE trickle candidates, mute mic/camera, end call teardown
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_webrtc/src/voip_session.rs`, `crates/gpui_telegram_desktop/src/views/calls/call_window.rs`
  * Target Symbols: `struct VoipCallSession`, `struct SdpHandshake`, `struct CallOverlayView`
  * Inherited Asset: `native/mahayana-messaging/src/realtime.rs`, `native/mahayana-messaging/src/signaling.rs`
  * Anti-Collapse Mapping: `calls_call.cpp` -> `VoipCallSession`, `calls_controller_webrtc.cpp` -> `WebRtcSignalingEngine`
  * Verification Test: `tests/voip_sdp_handshake.rs`

### 4.5 TCG-N25: Group Voice & Video Chats (SFU Bridge)
- **Upstream Files**:
  * `Telegram/SourceFiles/calls/group/calls_group_call.cpp`, `calls/group/calls_group_panel.cpp`
  * `Telegram/SourceFiles/calls/group/calls_group_members.cpp`
- **Real Upstream Symbols**:
  * Classes: `Calls::Group::Call`, `Calls::Group::Panel`, `Calls::Group::Members`
  * Features: Active speaker animated halo ring, participant grid layout, raise hand queue
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_webrtc/src/sfu_client.rs`, `crates/gpui_telegram_desktop/src/views/calls/group_call_grid.rs`
  * Target Symbols: `struct SfuGroupCallBridge`, `struct ParticipantVideoTile`, `struct ActiveSpeakerTracker`
  * Inherited Asset: `source/node-agent-coordinator/src/tdrp/calls_group_calls.rs`
  * Anti-Collapse Mapping: `calls_group_panel.cpp` -> `GroupCallPanel`, `calls_group_members.cpp` -> `GroupCallMembersDrawer`
  * Verification Test: `tests/group_call_sfu_tracks.rs`

### 4.6 TCG-N26: Low-Latency Screen Sharing
- **Upstream Files**:
  * `Telegram/SourceFiles/calls/group/ui/desktop_capture_choose_source.cpp`
  * `Telegram/SourceFiles/calls/group/calls_group_viewport_rhi.cpp`
- **Real Upstream Symbols**:
  * Classes: `Calls::Group::DesktopCaptureChooseSource`, `Calls::Group::Viewport`
  * Features: Full screen / application window capture selector, low-latency 60fps frame pipeline
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_webrtc/src/screen_capture.rs`, `crates/gpui_telegram_desktop/src/views/calls/screen_source_picker.rs`
  * Target Symbols: `struct ScreenCaptureSourcePicker`, `struct ScreenStreamPipe`
  * Anti-Collapse Mapping: `desktop_capture_choose_source.cpp` -> `ScreenCaptureSourcePicker`
  * Verification Test: `tests/screen_capture_stream.rs`

### 4.7 TCG-N27: Bot Platform & Inline Keyboards
- **Upstream Files**:
  * `Telegram/SourceFiles/inline_bots/inline_bot_result.cpp`, `inline_bots/inline_bot_layout_item.cpp`
  * `Telegram/SourceFiles/boxes/compose_ai_box.cpp`
- **Real Upstream Symbols**:
  * Classes: `InlineBots::Result`, `InlineBots::LayoutItem`, `HistoryView::BotKeyboard`
  * Features: 2D button grids, URL buttons, callback query buttons, bot command popup menus (`/start`, `/help`)
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_proto/src/bot.rs`, `crates/gpui_telegram_desktop/src/components/cards/card_bot_keyboard.rs`
  * Target Symbols: `struct InlineKeyboardGrid`, `struct InlineKeyboardButton`, `struct BotCommandMenu`
  * Inherited Asset: `native/mahayana-messaging/src/conversation.rs`
  * Anti-Collapse Mapping: `inline_bot_result.cpp` -> `InlineBotResultView`, `compose_ai_box.cpp` -> `BotPromptDialog`
  * Verification Test: `tests/bot_keyboard_click.rs`

### 4.8 TCG-N28: Mini Apps Sandbox Frame
- **Upstream Files**:
  * `Telegram/SourceFiles/boxes/url_auth_box_content.cpp`, `boxes/url_auth_box_content.style`
  * `Telegram/SourceFiles/payments/ui/payments.style`
- **Real Upstream Symbols**:
  * Classes: `UrlAuthBoxContent`, `Payments::Ui`
  * Security: Sandboxed iframe/WebView, strict CSP, message-passing bridge for theme and user context
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_desktop/src/views/miniapp/miniapp_sandbox.rs`
  * Target Symbols: `struct MiniAppSandboxModal`, `struct MiniAppBridgeProtocol`
  * Anti-Collapse Mapping: `url_auth_box_content.cpp` -> `MiniAppSandboxModal`
  * Verification Test: `tests/miniapp_bridge_event.rs`

### 4.9 TCG-N29: Contacts & Address Book
- **Upstream Files**:
  * `Telegram/SourceFiles/boxes/add_contact_box.cpp`, `boxes/add_contact_box.style`
  * `Telegram/SourceFiles/data/data_user.cpp`
- **Real Upstream Symbols**:
  * Classes: `AddContactBox`, `Data::User`
  * Features: Contact addition by phone or username, alphabet indexed contacts roster, contact sync
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_desktop/src/views/contacts/contact_roster.rs`, `crates/gpui_telegram_client/src/contacts/`
  * Target Symbols: `struct ContactRosterView`, `struct AddContactModal`, `struct ContactEntry`
  * Anti-Collapse Mapping: `add_contact_box.cpp` -> `AddContactModal`, `data_user.cpp` -> `UserEntity`
  * Verification Test: `tests/contact_search_roster.rs`

### 4.10 TCG-N30: User Profile & Presence Status
- **Upstream Files**:
  * `Telegram/SourceFiles/profile/profile.style`, `info/profile/info_profile_widget.cpp`
  * `Telegram/SourceFiles/data/data_user.cpp`
- **Real Upstream Symbols**:
  * Classes: `Profile::Widget`, `Data::User`
  * Status: Online, Offline (with last seen timestamp), Recently, Within a week, Within a month
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_proto/src/presence.rs`, `crates/gpui_telegram_desktop/src/components/profile/user_presence_badge.rs`
  * Target Symbols: `struct UserPresenceStatus`, `enum PresenceState`, `struct UserBioSection`
  * Anti-Collapse Mapping: `info_profile_widget.cpp` -> `UserProfileView`
  * Verification Test: `tests/presence_heartbeat_sync.rs`

### 4.11 TCG-N31: Chat Settings & Theme Engine
- **Upstream Files**:
  * `Telegram/SourceFiles/settings/sections/settings_chat.cpp`, `window/themes/window_themes_embedded.cpp`
  * `Telegram/SourceFiles/settings/settings.style`, `window/themes/window_themes_cloud_list.cpp`
- **Real Upstream Symbols**:
  * Classes: `Settings::Chat`, `Window::Theme::EmbeddedScheme`
  * Built-in Schemes: `DayBlue` (#40a7e3, #d7f0ff), `Night` (#5288c1, #5ca7d4), `NightGreen` (#3fc1b0), `Classic`
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_desktop/src/theme/`, `crates/gpui_telegram_desktop/src/views/settings/theme_settings.rs`
  * Target Symbols: `struct ThemeEngine`, `struct ColorPalette`, `enum BuiltinThemeScheme`, `fn apply_theme()`
  * Anti-Collapse Mapping: `window_themes_embedded.cpp` -> `BuiltinThemePalettes`, `settings_chat.cpp` -> `ChatSettingsView`
  * Verification Test: `tests/theme_switch_tokens.rs`

### 4.12 TCG-N32: System Notifications & Dock Badges
- **Upstream Files**:
  * `Telegram/SourceFiles/platform/mac/notifications_manager_mac.mm`
  * `Telegram/SourceFiles/settings/sections/settings_notifications.style`, `data/notify/data_notify_settings.cpp`
- **Real Upstream Symbols**:
  * Classes: `Notifications::Manager`, `Data::NotifySettings`
  * Features: Native macOS notification center alerts, mute per chat/folder, dock unread count badge counter
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_desktop/src/notifications/notification_manager.rs`
  * Target Symbols: `struct NotificationManager`, `struct NotificationPayload`, `fn update_dock_badge_count()`
  * Anti-Collapse Mapping: `notifications_manager_mac.mm` -> `MacNotificationBridge`
  * Verification Test: `tests/notification_badge_count.rs`

### 4.13 TCG-N33: Privacy & Security Controls
- **Upstream Files**:
  * `Telegram/SourceFiles/settings/sections/settings_privacy_controllers.cpp`
  * `Telegram/SourceFiles/settings/sections/settings_privacy_security.cpp`
- **Real Upstream Symbols**:
  * Classes: `Settings::PrivacySecurity`, `PrivacyRule`
  * Rules: Phone number visibility, last seen, profile photos, forwards author, voice messages, peer blocklist
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_server/src/privacy/`, `crates/gpui_telegram_desktop/src/views/settings/privacy_settings.rs`
  * Target Symbols: `struct PrivacyRuleset`, `enum PrivacyRuleScope { Everybody, Contacts, Nobody }`
  * Anti-Collapse Mapping: `settings_privacy_controllers.cpp` -> `PrivacySettingsView`
  * Verification Test: `tests/privacy_visibility_rules.rs`

### 4.14 TCG-N34: Storage & Local Cache Management
- **Upstream Files**:
  * `Telegram/SourceFiles/settings/sections/settings_local_storage.cpp`
  * `Telegram/SourceFiles/settings/sections/settings_local_storage.style`
  * `Telegram/SourceFiles/storage/storage_databases.cpp`
- **Real Upstream Symbols**:
  * Classes: `Settings::LocalStorage`, `Storage::Databases`
  * Actions: Maximum cache size limits (5GB, 10GB, No Limit), auto-purge media older than X days, clear cache by type
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_client/src/storage/cache_cleaner.rs`, `crates/gpui_telegram_desktop/src/views/settings/storage_settings.rs`
  * Target Symbols: `struct CacheManagementService`, `struct CacheStatistics`, `fn purge_expired_cache()`
  * Anti-Collapse Mapping: `settings_local_storage.cpp` -> `LocalStorageSettingsView`
  * Verification Test: `tests/cache_auto_purge.rs`

### 4.15 TCG-N35: Proxy & Network Diagnostics
- **Upstream Files**:
  * `Telegram/SourceFiles/boxes/connection_box.cpp`, `boxes/connection_box.style`
  * `Telegram/SourceFiles/mtproto/connection.cpp`
- **Real Upstream Symbols**:
  * Classes: `ConnectionBox`, `MTP::Connection`
  * Protocols: SOCKS5, HTTP/HTTPS Proxy, round-trip ping diagnostics (RTT in ms), auto-failover
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_client/src/network/proxy_config.rs`, `crates/gpui_telegram_desktop/src/views/settings/proxy_modal.rs`
  * Target Symbols: `struct ProxyConfiguration`, `struct ProxyPingDiagnostic`, `enum ProxyProtocol`
  * Anti-Collapse Mapping: `connection_box.cpp` -> `ProxySettingsModal`
  * Verification Test: `tests/proxy_ping_probe.rs`

### 4.16 TCG-N36: Draft Synchronization & Realtime Fencing
- **Upstream Files**:
  * `Telegram/SourceFiles/dialogs/ui/dialogs_layout.cpp` (`dialogsTextPaletteDraft`, `dialogsDraftFg`)
  * `Telegram/SourceFiles/data/data_drafts.cpp`, `data/data_drafts.h`
- **Real Upstream Symbols**:
  * Classes: `Data::Drafts`, `Dialogs::Row`
  * Features: Per-peer debounce autosave (500ms), red "Draft: ..." preview in chat sidebar row, reply-to link
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_client/src/drafts/draft_coordinator.rs`, `crates/gpui_telegram_desktop/src/views/composer/`
  * Target Symbols: `struct DraftRecord`, `struct DraftSyncCoordinator`, `fn persist_draft_debounced()`
  * Anti-Collapse Mapping: `data_drafts.cpp` -> `DraftCoordinator`
  * Verification Test: `tests/draft_autosave_sync.rs`

### 4.17 TCG-N37: Archive & Filter Chips
- **Upstream Files**:
  * `Telegram/SourceFiles/dialogs/dialogs_widget.cpp`, `dialogs/dialogs.style` (`dialogsFilterPadding: point(7px, 7px)`)
  * `Telegram/SourceFiles/boxes/filters/edit_filter_box.cpp`
- **Real Upstream Symbols**:
  * Classes: `Dialogs::Widget`, `Dialogs::FilterTabs`, `EditFilterBox`
  * Features: Archived chats swipe hide, custom folder tabs ("Personal", "Work", "Bots", "Unread"), folder reordering
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_desktop/src/views/sidebar/folder_tabs.rs`
  * Target Symbols: `struct FolderTabsBar`, `struct FilterChipBadge`, `struct EditFolderModal`
  * Inherited Asset: `source/node-agent-coordinator/src/tdrp/folders.rs`
  * Anti-Collapse Mapping: `edit_filter_box.cpp` -> `EditFolderModal`, `dialogs_filter` -> `FolderTabsBar`
  * Verification Test: `tests/folder_filter_reorder.rs`

### 4.18 TCG-N38: Stories Top Strip & Viewer
- **Upstream Files**:
  * `Telegram/SourceFiles/media/stories/media_stories_controller.cpp`, `media/stories/media_stories.style`
  * `Telegram/SourceFiles/data/data_stories.cpp`
- **Real Upstream Symbols**:
  * Classes: `Media::Stories::Controller`, `Media::Stories::Viewer`, `Data::Stories`
  * Features: Top horizontal avatar carousel strip, 24h photo/video story playback with timed segmented progress bar
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_desktop/src/views/stories/stories_bar.rs`, `crates/gpui_telegram_desktop/src/views/stories/stories_viewer.rs`
  * Target Symbols: `struct StoriesTopStrip`, `struct StoryPlaybackViewer`, `struct StorySegmentTimer`
  * Anti-Collapse Mapping: `media_stories_controller.cpp` -> `StoriesController`, `media_stories_viewer` -> `StoryPlaybackViewer`
  * Verification Test: `tests/stories_segment_timer.rs`

### 4.19 TCG-N39: Secret Chats & End-to-End Encryption
- **Upstream Files**:
  * `Telegram/SourceFiles/storage/details/storage_file_utilities.cpp`
  * `Telegram/SourceFiles/data/data_session.cpp`
- **Real Upstream Symbols**:
  * Classes: `Data::Session`, `Storage::Details`
  * Encryption: Ephemeral Diffie-Hellman key exchange, Double Ratchet / forward secrecy, zero server plaintext persistence
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_core/src/crypto/e2ee.rs`, `crates/gpui_telegram_proto/src/secret.rs`
  * Target Symbols: `struct SecretChatSession`, `struct RatchetState`, `fn encrypt_secret_payload()`
  * Inherited Asset: `native/mahayana-messaging/src/secret.rs` (X25519 + HKDF-SHA256 + XChaCha20Poly1305)
  * Anti-Collapse Mapping: `storage_file_utilities.cpp` -> `SecretChatKeyStore`
  * Verification Test: `tests/secret_chat_e2ee_ratchet.rs`

### 4.20 TCG-N40: Self-Destruct Timers (TTL)
- **Upstream Files**:
  * `Telegram/SourceFiles/ui/controls/ttl_media.style`, `ui/controls/ttl_media.cpp`
  * `Telegram/SourceFiles/history/history_item.cpp`
- **Real Upstream Symbols**:
  * Classes: `Ui::TtlMedia`, `HistoryItem`
  * TTL Duration: 1s, 2s, 5s, 10s, 30s, 1m, 1h, 1d, 1w. Timer fires on recipient view, auto-erasing payload locally and upstream.
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_core/src/ttl_timer.rs`, `crates/gpui_telegram_desktop/src/components/cards/ttl_indicator.rs`
  * Target Symbols: `struct TtlIndicatorBadge`, `struct SelfDestructWorker`, `fn schedule_message_destruction()`
  * Anti-Collapse Mapping: `ttl_media.cpp` -> `TtlMediaIndicator`
  * Verification Test: `tests/ttl_countdown_purge.rs`

---

## 5. Deep Source-Reading Feature Specifications (TCG-N41 through TCG-N52 & CAP-01 through CAP-06)

### 5.1 TCG-N41: Location & Live Location Sharing
- **Upstream Files**:
  * `Telegram/SourceFiles/ui/controls/location_picker.style`, `ui/controls/location_picker.cpp`
  * `Telegram/SourceFiles/history/view/media/history_view_location.cpp`
- **Real Upstream Symbols**:
  * Classes: `Ui::LocationPicker`, `HistoryView::Location`
  * Features: Static map thumbnail with pin, live location stream with heading beacon and expiration timer
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_desktop/src/components/cards/card_location.rs`, `crates/gpui_telegram_proto/src/location.rs`
  * Target Symbols: `struct LocationCardComponent`, `struct LiveLocationUpdate`, `struct LocationPickerModal`
  * Anti-Collapse Mapping: `location_picker.cpp` -> `LocationPickerModal`
  * Verification Test: `tests/location_card_render.rs`

### 5.2 TCG-N42: Web Link Previews (OpenGraph Extraction)
- **Upstream Files**:
  * `Telegram/SourceFiles/history/view/history_view_webpage.cpp`, `history/view/history_view_webpage.h`
  * `Telegram/SourceFiles/data/data_web_page.cpp`
- **Real Upstream Symbols**:
  * Classes: `HistoryView::WebPage`, `Data::WebPage`
  * Features: OpenGraph metadata parser (title, description, site name, thumbnail preview, instant view badge)
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_server/src/media/og_scraper.rs`, `crates/gpui_telegram_desktop/src/components/cards/card_webpage.rs`
  * Target Symbols: `struct WebpageCardComponent`, `struct OpenGraphMetadata`, `fn scrape_opengraph_preview()`
  * Anti-Collapse Mapping: `history_view_webpage.cpp` -> `WebpageCardComponent`
  * Verification Test: `tests/webpage_preview_parse.rs`

### 5.3 TCG-N43: Export Chat History (JSON / HTML)
- **Upstream Files**:
  * `Telegram/SourceFiles/export/export_controller.cpp`, `export/view/export_view_panel_controller.cpp`
  * `Telegram/SourceFiles/export/output/export_output_result.cpp`, `export/view/export.style`
- **Real Upstream Symbols**:
  * Classes: `Export::Controller`, `Export::View::PanelController`, `Export::Output::Result`
  * Formats: Machine-readable JSON and human-readable HTML package with static assets
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_client/src/export/export_manager.rs`, `crates/gpui_telegram_desktop/src/views/settings/export_modal.rs`
  * Target Symbols: `struct ExportManager`, `struct ChatExportPackage`, `struct ExportProgressState`
  * Anti-Collapse Mapping: `export_controller.cpp` -> `ExportManager`, `export_view_panel_controller.cpp` -> `ExportModal`
  * Verification Test: `tests/chat_history_export_json.rs`

### 5.4 TCG-N44: Language & Internationalization
- **Upstream Files**:
  * `Telegram/SourceFiles/lang/lang_instance.cpp`, `lang/lang_cloud_manager.cpp`, `lang/lang_keys.h`
- **Real Upstream Symbols**:
  * Classes: `Lang::Instance`, `Lang::CloudManager`, `tr::lng_*` tokens
  * Features: Dynamic runtime language switching, cloud translation pack sync, pluralization rules
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_desktop/src/i18n/localization_manager.rs`
  * Target Symbols: `struct I18nManager`, `struct LocaleBundle`, `macro_rules! tr`
  * Anti-Collapse Mapping: `lang_instance.cpp` -> `I18nManager`
  * Verification Test: `tests/i18n_plural_strings.rs`

### 5.5 TCG-N45: Keybindings & Shortcut Manager
- **Upstream Files**:
  * `Telegram/SourceFiles/settings/settings_key_navigation.cpp`, `settings/settings_key_navigation.h`
  * `Telegram/SourceFiles/boxes/keys_legend.style`
- **Real Upstream Symbols**:
  * Classes: `Settings::KeyNavigation`, `KeysLegend`
  * Shortcuts: `Cmd+K` (universal search), `Cmd+1..9` (folder tabs), `Cmd+Up/Down` (chat list navigation), `Esc` (close modal)
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_desktop/src/shortcuts/keymap.rs`
  * Target Symbols: `struct KeymapRegistry`, `enum TelegramKeyAction`, `fn register_global_keybindings()`
  * Anti-Collapse Mapping: `settings_key_navigation.cpp` -> `KeymapRegistry`
  * Verification Test: `tests/keybinding_dispatch.rs`

### 5.6 TCG-N46: Custom Window Frame (Metal & TitleBar)
- **Upstream Files**:
  * `Telegram/SourceFiles/window/window_main_menu.cpp`, `window/window.style`
  * `Telegram/SourceFiles/platform/mac/mac_window.mm`
- **Real Upstream Symbols**:
  * Classes: `Window::Controller`, `Platform::MacWindow`
  * Parity: Full-size content view, macOS native traffic light alignment, draggable titlebar with peer header info
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_desktop/src/views/app_shell/window_frame.rs`
  * Target Symbols: `struct CustomWindowFrame`, `struct TitlebarHeaderComponent`
  * Anti-Collapse Mapping: `window_main_menu.cpp` -> `AppWindowShell`
  * Verification Test: `tests/custom_window_frame_hit_test.rs`

### 5.7 TCG-N47: Drag-and-Drop & Clipboard Attachments
- **Upstream Files**:
  * `Telegram/SourceFiles/boxes/send_files_box.cpp`, `boxes/send_files_box.style`
  * `Telegram/SourceFiles/window/window_main_widget.cpp`
- **Real Upstream Symbols**:
  * Classes: `SendFilesBox`, `Window::MainWidget`
  * Actions: File drop over composer reveals "Drop files to send as photos" vs "Drop files to send as documents" overlay
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_desktop/src/views/composer/drop_target_overlay.rs`
  * Target Symbols: `struct DropTargetOverlay`, `struct ClipboardMediaHandler`
  * Anti-Collapse Mapping: `send_files_box.cpp` -> `SendFilesConfirmationModal`
  * Verification Test: `tests/drag_and_drop_detection.rs`

### 5.8 TCG-N48: Auto-Download Policy Rules
- **Upstream Files**:
  * `Telegram/SourceFiles/settings/sections/settings_data_and_storage.cpp`
  * `Telegram/SourceFiles/data/data_auto_download.cpp`
- **Real Upstream Symbols**:
  * Classes: `Settings::DataAndStorage`, `Data::AutoDownload`
  * Policies: Auto-download photos/videos/files up to X MB based on network type (WiFi, Cellular, Roaming)
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_client/src/storage/download_policy.rs`
  * Target Symbols: `struct AutoDownloadPolicy`, `enum NetworkMediaType`, `fn should_auto_download()`
  * Anti-Collapse Mapping: `settings_data_and_storage.cpp` -> `DataStorageSettingsView`
  * Verification Test: `tests/auto_download_policy.rs`

### 5.9 TCG-N49: Multi-Account Profile Switching
- **Upstream Files**:
  * `Telegram/SourceFiles/window/window_main_menu.cpp`, `window/window_main_menu.style`
  * `Telegram/SourceFiles/core/application.cpp`
- **Real Upstream Symbols**:
  * Classes: `Window::MainMenu`, `Core::Application`
  * Features: Support up to 6 concurrent active accounts with instant sidebar profile avatar switching
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_desktop/src/views/nav_rail/account_switcher.rs`, `crates/gpui_telegram_client/src/session/account_pool.rs`
  * Target Symbols: `struct MultiAccountSwitcherRail`, `struct AccountSessionPool`, `fn switch_active_account()`
  * Anti-Collapse Mapping: `window_main_menu.cpp` -> `AccountSwitcherRail`
  * Verification Test: `tests/multi_account_switch.rs`

### 5.10 TCG-N50: Monotonic Account Scope Generation Fencing
- **Upstream Files**:
  * `Telegram/SourceFiles/data/data_session.cpp`, `core/application.cpp`
- **Real Upstream Symbols**:
  * Classes: `Data::Session`, `Core::Application`
  * Principle: Monotonically increasing `accountScopeGeneration: u64`. Stale asynchronous network responses from previous account sessions are automatically fenced and dropped.
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_core/src/scope_fence.rs`, `crates/gpui_telegram_client/src/session/fence.rs`
  * Target Symbols: `struct AccountScopeFence`, `struct GenerationToken`, `fn validate_scope_generation()`
  * Anti-Collapse Mapping: `data_session.cpp` -> `AccountScopeFence`
  * Verification Test: `tests/scope_generation_fence.rs`

### 5.11 TCG-N51: Chunked Media Upload Pipeline
- **Upstream Files**:
  * `Telegram/SourceFiles/storage/file_upload.cpp`, `storage/file_upload.h`
- **Real Upstream Symbols**:
  * Classes: `Storage::UploadPreparation`, `Storage::FileUpload`
  * Specs: 512KB..1MB chunks, SHA-256 CAS deduplication, resumable byte offset tracking
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_media/src/chunked_upload.rs`, `crates/gpui_telegram_server/src/media/chunk_sink.rs`
  * Target Symbols: `struct ChunkedUploadPipeline`, `struct ChunkPartReceipt`, `fn append_chunk()`
  * Inherited Asset: `native/mahayana-messaging/src/blob_store.rs`
  * Anti-Collapse Mapping: `file_upload.cpp` -> `ChunkedUploadPipeline`
  * Verification Test: `tests/chunked_upload_pipeline.rs`

### 5.12 TCG-N52: Range-Request Media Streaming (HTTP 206)
- **Upstream Files**:
  * `Telegram/SourceFiles/data/data_streaming.cpp`, `media/streaming/media_streaming_instance.cpp`
- **Real Upstream Symbols**:
  * Classes: `Data::Streaming`, `Media::Streaming::Instance`
  * Protocol: Standard HTTP 206 Partial Content byte ranges (`bytes=start-end/total`), sub-50ms audio/video playback start
- **Target Rust Architecture**:
  * Crate: `crates/gpui_telegram_server/src/media/range_responder.rs`, `crates/gpui_telegram_media/src/range_stream.rs`
  * Target Symbols: `struct RangeStreamResponder`, `struct Http206StreamRequest`, `fn stream_media_range()`
  * Anti-Collapse Mapping: `data_streaming.cpp` -> `RangeStreamResponder`
  * Verification Test: `tests/http206_range_streaming.rs`

---

### 5.13 CAP-01: AI Multi-Agent ACP Runner & Swarm Lineage
- **Inherited Production Assets**:
  * `source/host/src/runner/mod.rs`, `source/host/src/acp/`, `source/host/tests/contract_acp_turns.rs`
- **Responsibilities**:
  * Multi-agent turn-taking engine, swarm subagent trees, collapsible thinking steps, token counter
- **GPUI Integration**:
  * Crate: `crates/gpui_telegram_agent/src/acp_runner.rs`, `crates/gpui_telegram_desktop/src/components/cards/card_thinking.rs`
  * Target Symbols: `struct AcpThinkingCard`, `struct AgentSwarmLineage`, `struct ThoughtStreamChunk`
  * Verification Test: `source/host/tests/contract_acp_turns.rs`

### 5.14 CAP-02: Global Task Center & 7-State FSM
- **Inherited Production Assets**:
  * `source/host/src/tasks/`, `source/host/src/cron/`, `source/host/tests/contract_task_center.rs`
- **Responsibilities**:
  * 7-State FSM: `Draft -> Scheduled -> PendingApproval -> Running -> Blocked -> Completed -> Failed`
  * 5-field cron routines (`* * * * *`), manual approval gates
- **GPUI Integration**:
  * Crate: `crates/gpui_telegram_agent/src/task_center.rs`, `crates/gpui_telegram_desktop/src/views/drawer/task_center_tab.rs`
  * Target Symbols: `struct GlobalTaskCenterTab`, `struct TaskItemCard`, `struct CronRoutineScheduler`
  * Verification Test: `source/host/tests/contract_task_center.rs`

### 5.15 CAP-03: MCP & Plugin Ecosystem
- **Inherited Production Assets**:
  * `source/host/src/mcp/`, `source/mahayana/mahayana-rs/mcp/`
- **Responsibilities**:
  * Stdio & SSE tool server registry, tool discovery schema, PermissionDock with interactive approval
- **GPUI Integration**:
  * Crate: `crates/gpui_telegram_agent/src/mcp_registry.rs`, `crates/gpui_telegram_desktop/src/views/drawer/mcp_dock.rs`
  * Target Symbols: `struct McpPermissionDock`, `struct ToolInvocationCard`, `struct ToolServerRegistry`
  * Verification Test: `source/host/tests/contract_mcp_registration.rs`

### 5.16 CAP-04: Remote Device & Cloud Computer Control (CUA)
- **Inherited Production Assets**:
  * `source/node-agent-coordinator/src/vnc/`, `source/node-agent-coordinator/src/cua/`, `source/box-exec-daemon/`
- **Responsibilities**:
  * Low-latency VNC frame streaming, mouse/keyboard CUA input injection, exclusive device lease broker
- **GPUI Integration**:
  * Crate: `crates/gpui_telegram_agent/src/cua_client.rs`, `crates/gpui_telegram_desktop/src/components/cards/card_computer_workbench.rs`
  * Target Symbols: `struct ComputerWorkbenchCard`, `struct VncFrameStreamView`, `struct RemoteControlLeaseBadge`
  * Verification Test: `source/node-agent-coordinator/tests/contract_vnc_stream.rs`

### 5.17 CAP-05: Full-Duplex Talk Mode & Energy Orb
- **Inherited Production Assets**:
  * `source/host/src/talk/`, `native/mahayana-messaging/src/realtime.rs`
- **Responsibilities**:
  * Realtime audio pipeline, Voice Activity Detection (VAD), 60ms barge-in interruption, energy orb frequency animation
- **GPUI Integration**:
  * Crate: `crates/gpui_telegram_media/src/talk_mode.rs`, `crates/gpui_telegram_desktop/src/components/cards/card_talk_mode_orb.rs`
  * Target Symbols: `struct TalkModeEnergyOrb`, `struct VadDetector`, `struct FullDuplexAudioBridge`
  * Verification Test: `source/host/tests/contract_talk_session.rs`

### 5.18 CAP-06: A2UI Declarative Interactive Cards
- **Inherited Production Assets**:
  * `native/mahayana-messaging/src/cards/`, `native/mahayana-messaging/src/message.rs`
- **Responsibilities**:
  * Declarative card schemas: ToDos, interactive confirmation forms, progress bars, multi-button actions
- **GPUI Integration**:
  * Crate: `crates/gpui_telegram_proto/src/cards.rs`, `crates/gpui_telegram_desktop/src/components/cards/card_todo.rs`
  * Target Symbols: `struct TodoCardComponent`, `struct A2UiFormBuilder`, `struct CardActionCallback`
  * Verification Test: `native/mahayana-messaging/tests/contract_cards.rs`

---

## 6. Anti-Symbol-Collapse & Anti-Monolith File Layout Architecture

In strict enforcement of **User Mandate #5**, target crates are organized into granular, fine-grained subdirectories and files that mirror upstream `tdesktop` modules:

### 6.1 `crates/gpui_telegram_desktop` Subdirectory Architecture

```
crates/gpui_telegram_desktop/src/
├── main.rs
├── app.rs
├── theme/
│   ├── mod.rs
│   ├── palette.rs              # Upstream window_themes_embedded.cpp
│   ├── tokens.rs               # Upstream basic.style & chat.style
│   └── icons.rs                # Upstream menu_icons.style
├── views/
│   ├── mod.rs
│   ├── app_shell/
│   │   ├── mod.rs
│   │   ├── window_frame.rs     # Upstream window_main_menu.cpp (Metal Titlebar)
│   │   ├── primary_nav_rail.rs # 52px Left Primary Rail
│   │   └── split_container.rs  # 4-pane Resizable Split Container
│   ├── sidebar/
│   │   ├── mod.rs
│   │   ├── chat_sidebar.rs     # 288px ChatSidebar with uniform_list
│   │   ├── dialog_row.rs       # Upstream dialogs_row.cpp (62px defaultDialogRow)
│   │   ├── folder_tabs.rs      # Upstream edit_filter_box.cpp & dialogs.style
│   │   └── topic_list.rs       # Upstream data_forum_topic.cpp
│   ├── stage/
│   │   ├── mod.rs
│   │   ├── conversation_stage.rs # Virtualized SumTree Message List
│   │   ├── message_bubble.rs   # Upstream history_view_message.cpp (16px / 4px radii)
│   │   └── pinned_banner.rs    # Upstream history_view_pinned_bar.cpp
│   ├── composer/
│   │   ├── mod.rs
│   │   ├── canonical_composer.rs # Upstream compose_show.cpp (IME safe)
│   │   ├── voice_record.rs     # Upstream history_view_voice_record_bar.style
│   │   ├── drop_target.rs      # Upstream send_files_box.cpp
│   │   └── schedule_modal.rs   # Upstream choose_date_time.style
│   ├── drawer/
│   │   ├── mod.rs
│   │   ├── context_inspector.rs # 320px Right Unified Drawer
│   │   ├── profile_pane.rs     # Upstream info_profile_widget.cpp
│   │   ├── members_pane.rs     # Upstream info_members_widget.cpp
│   │   ├── media_archive_pane.rs # Upstream info_media_widget.cpp
│   │   ├── task_center_tab.rs  # CAP-02 Global Task Center Tab
│   │   └── mcp_dock_tab.rs     # CAP-03 MCP Permission Dock Tab
│   ├── boxes/
│   │   ├── mod.rs
│   │   ├── share_box.rs        # Upstream boxes/share_box.cpp
│   │   ├── add_contact_box.rs  # Upstream boxes/add_contact_box.cpp
│   │   ├── connection_box.rs   # Upstream boxes/connection_box.cpp
│   │   ├── passcode_box.rs     # Upstream boxes/passcode_box.cpp
│   │   └── url_auth_box.rs     # Upstream boxes/url_auth_box_content.cpp
│   └── settings/
│       ├── mod.rs
│       ├── settings_main.rs    # Upstream settings/settings.style
│       ├── chat_settings.rs    # Upstream settings/sections/settings_chat.cpp
│       ├── storage_settings.rs # Upstream settings/sections/settings_local_storage.cpp
│       ├── privacy_settings.rs # Upstream settings/sections/settings_privacy_controllers.cpp
│       └── sessions_settings.rs# Upstream settings/sections/settings_active_sessions.cpp
└── components/
    ├── mod.rs
    └── cards/
        ├── mod.rs
        ├── card_text.rs
        ├── card_media.rs
        ├── card_voice.rs
        ├── card_document.rs
        ├── card_sticker.rs
        ├── card_poll.rs
        ├── card_todo.rs
        ├── card_thinking.rs
        ├── card_talk_mode_orb.rs
        ├── card_computer_workbench.rs
        └── card_webapp_payment.rs
```

---

## 7. Sharded Ledger Architecture Blueprint

### 7.1 Master Index File: `docs/specs/ledger/index.json`
The master index provides lightweight, O(1) status tracking across all 28 subsystems:

```json
{
  "format_version": 4,
  "project_id": "GPUI-TELEGRAM-PARITY",
  "spec_revision": "SPEC-01",
  "upstream_root": "tdesktop/Telegram/SourceFiles/",
  "total_source_files": 3441,
  "total_mapped_files": 3441,
  "omitted_count": 0,
  "unmapped_count": 0,
  "shards_count": 28,
  "shards": [
    { "subsystem": "api", "path": "docs/specs/ledger/shards/api.json", "file_count": 116 },
    { "subsystem": "boxes", "path": "docs/specs/ledger/shards/boxes.json", "file_count": 219 },
    { "subsystem": "calls", "path": "docs/specs/ledger/shards/calls.json", "file_count": 88 },
    { "subsystem": "chat_helpers", "path": "docs/specs/ledger/shards/chat_helpers.json", "file_count": 57 },
    { "subsystem": "core", "path": "docs/specs/ledger/shards/core.json", "file_count": 76 },
    { "subsystem": "data", "path": "docs/specs/ledger/shards/data.json", "file_count": 226 },
    { "subsystem": "dialogs", "path": "docs/specs/ledger/shards/dialogs.json", "file_count": 83 },
    { "subsystem": "editor", "path": "docs/specs/ledger/shards/editor.json", "file_count": 95 },
    { "subsystem": "export", "path": "docs/specs/ledger/shards/export.json", "file_count": 35 },
    { "subsystem": "history", "path": "docs/specs/ledger/shards/history.json", "file_count": 296 },
    { "subsystem": "info", "path": "docs/specs/ledger/shards/info.json", "file_count": 239 },
    { "subsystem": "inline_bots", "path": "docs/specs/ledger/shards/inline_bots.json", "file_count": 20 },
    { "subsystem": "intro", "path": "docs/specs/ledger/shards/intro.json", "file_count": 21 },
    { "subsystem": "iv", "path": "docs/specs/ledger/shards/iv.json", "file_count": 147 },
    { "subsystem": "lang", "path": "docs/specs/ledger/shards/lang.json", "file_count": 25 },
    { "subsystem": "layout", "path": "docs/specs/ledger/shards/layout.json", "file_count": 12 },
    { "subsystem": "main", "path": "docs/specs/ledger/shards/main.json", "file_count": 16 },
    { "subsystem": "media", "path": "docs/specs/ledger/shards/media.json", "file_count": 146 },
    { "subsystem": "mtproto", "path": "docs/specs/ledger/shards/mtproto.json", "file_count": 74 },
    { "subsystem": "passport", "path": "docs/specs/ledger/shards/passport.json", "file_count": 29 },
    { "subsystem": "payments", "path": "docs/specs/ledger/shards/payments.json", "file_count": 53 },
    { "subsystem": "platform", "path": "docs/specs/ledger/shards/platform.json", "file_count": 127 },
    { "subsystem": "profile", "path": "docs/specs/ledger/shards/profile.json", "file_count": 7 },
    { "subsystem": "settings", "path": "docs/specs/ledger/shards/settings.json", "file_count": 121 },
    { "subsystem": "statistics", "path": "docs/specs/ledger/shards/statistics.json", "file_count": 43 },
    { "subsystem": "storage", "path": "docs/specs/ledger/shards/storage.json", "file_count": 47 },
    { "subsystem": "ui", "path": "docs/specs/ledger/shards/ui.json", "file_count": 454 },
    { "subsystem": "window", "path": "docs/specs/ledger/shards/window.json", "file_count": 83 },
    { "subsystem": "root_and_other", "path": "docs/specs/ledger/shards/root_and_other.json", "file_count": 53 }
  ]
}
```

### 7.2 Subsystem Shard Entry Schema (`docs/specs/ledger/shards/<subsystem>.json`)
Each entry in a subsystem shard contains deep source-reading evidence (Mandate #7):

```json
{
  "source_path": "Telegram/SourceFiles/dialogs/dialogs_widget.cpp",
  "source_sha256": "4e1b8...authentic_hash",
  "source_line_count": 1184,
  "subsystem": "dialogs",
  "feature_id": "TCG-N03",
  "source_classes_and_structs": [
    "Dialogs::Widget",
    "Dialogs::IndexedList",
    "Dialogs::SearchQuery"
  ],
  "source_methods_and_signals": [
    "Dialogs::Widget::applyFilter()",
    "Dialogs::Widget::scrollToPeer()",
    "Dialogs::Widget::chosenRow()"
  ],
  "disposition": "gpui_ui_1to1",
  "inherited_rust_path": "native/mahayana-messaging/src/engine_tdrp_dialogs.rs",
  "target_rust_gpui_path": "crates/gpui_telegram_desktop/src/views/sidebar/chat_sidebar.rs",
  "target_symbol": "ChatSidebarView",
  "verification_test_id": "tests/dialog_list_virtual_scroll.rs"
}
```

---

## 8. Verification Strategy & Gate Checks

The ledger and feature inventory are enforced by `scripts/verify_ledger_closure.py`:
- **Gate 1 (Completeness)**: Scans all 3,441 source files under `tdesktop/Telegram/SourceFiles/`. Asserts $Count(ledger\_files) = 3,441$. Fails if any file is missing.
- **Gate 2 (Anti-Symbol-Collapse)**: Verifies that distinct source files do not map to generic collapsed target symbols. Maximum fan-in is $\le 2$ (only for natural `.h` + `.cpp` pairs).
- **Gate 3 (Anti-Monolith File Layout)**: Verifies that target crate paths are granularly distributed across subdirectories, rejecting monolithic `.rs` files.
- **Gate 4 (Test Association)**: Verifies that every single ledger entry has an associated `verification_test_id`.
- **Gate 5 (Authentic Source Extraction)**: Verifies that `source_sha256`, `source_line_count`, and authentic C++ symbols/tokens are present and non-empty.
