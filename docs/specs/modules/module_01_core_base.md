# Module 01 Specification: Core & Base Subsystem (`core_base`)

## 1. Module Overview & Architectural Objectives

Module 01 (`core & base`) forms the foundational layer of the Telegram Desktop Rust/GPUI rewrite. In the upstream C++ implementation (`Telegram/SourceFiles/core/` and related `_other/` files), this subsystem is responsible for process bootstrapping, environment sandboxing, application lifecycle coordination, cross-window management, persistent user settings, cryptographic password hashing (SRP-6A), proxy management and automatic rotation, shortcut dispatch, deadlock health monitoring, and crash reporting.

In this Rust/GPUI architecture, `gpui_telegram_core` re-implements these core primitives without C++ legacy bloat or fake placeholder stubs. It links directly with existing workspace components:
- `fabushi-messaging-core`: Telegram domain types, TDRP projections, and engine actors.
- `mahayana-node-agent-coordinator`: TDRP coordinators, passkeys, and cloud settings.
- `mahayana-host-runtime`: Task runners, approval gates, and ACP executor engines.

---

## 2. Upstream C++ Class Hierarchy & Rust GPUI Target Mapping

| Upstream C++ Source | Upstream C++ Classes / Types | Target Rust File | Target Rust Symbol | Subsystem / Responsibility |
|---|---|---|---|---|
| `core/application.h`, `application.cpp` | `Core::Application`, `LaunchState`, `QuitReason`, `ProxyChange` | `crates/gpui_telegram_core/src/core/application.rs` | `Application` | Central runtime singleton, window lifecycle, session orchestration |
| `core/core_settings.h`, `core_settings.cpp` | `Core::Settings`, `WindowPosition`, `WindowTitleContent`, `RecentEmoji` | `crates/gpui_telegram_core/src/core/core_settings.rs` | `CoreSettings` | Global configuration state, window positions, volume, auto-lock |
| `core/core_settings_proxy.h`, `core_settings_proxy.cpp` | `Core::SettingsProxy`, `ProxyData` | `crates/gpui_telegram_core/src/core/core_settings_proxy.rs` | `CoreSettingsProxy` | Proxy list, active proxy selection, IPv6, proxy rotation settings |
| `core/launcher.h`, `launcher.cpp` | `Core::Launcher`, `UpdaterLaunch` | `crates/gpui_telegram_core/src/core/launcher.rs` | `Launcher` | CLI argument parser, working directory resolver, instance hash |
| `core/sandbox.h`, `sandbox.cpp` | `Core::Sandbox`, `LocalClient`, `PostponedCall` | `crates/gpui_telegram_core/src/core/sandbox.rs` | `Sandbox` | Single-instance lockfile, event loop hook, postponed tasks |
| `core/shortcuts.h`, `shortcuts.cpp` | `Shortcuts::Command`, `Shortcuts::Request`, `KeySequence` | `crates/gpui_telegram_core/src/core/shortcuts.rs` | `Shortcuts` | Keyboard shortcut router, command binding map, switch chat |
| `core/version.h` | `AppId`, `AppVersion`, `AppVersionStr`, `AppBetaVersion` | `crates/gpui_telegram_core/src/core/version.rs` | `Version` | Semantic version constants, AppId, beta/alpha flags |
| `core/update_checker.h`, `update_checker.cpp` | `Core::UpdateChecker`, `UpdateChecker::State`, `Progress` | `crates/gpui_telegram_core/src/core/update_checker.rs` | `UpdateChecker` | Background update checking FSM, download tracking, verification |
| `core/crash_report_window.h`, `crash_report_window.cpp` | `LastCrashedWindow`, `PreLaunchWindow`, `NetworkSettingsWindow` | `crates/gpui_telegram_core/src/core/crash_report_window.rs` | `CrashReportWindow` | Crash diagnostic modal, minidump report preview, crash recovery |
| `core/crash_reports.h`, `crash_reports.cpp` | `CrashReports::Status`, `StartResult`, `SetAnnotation` | `crates/gpui_telegram_core/src/core/crash_reports.rs` | `CrashReports` | Crash report writer, panic hook, breadcrumb annotations |
| `core/core_screenshot_protection.h` | `Core::ScreenshotProtection`, `Reason`, `Applied` | `crates/gpui_telegram_core/src/core/core_screenshot_protection.rs` | `CoreScreenshotProtection` | OS-level window display affinity, DRM screenshot guard |
| `core/core_cloud_password.h`, `core_cloud_password.cpp` | `CloudPasswordAlgoModPow`, `CloudPasswordCheckRequest`, SRP-6A | `crates/gpui_telegram_core/src/core/core_cloud_password.rs` | `CoreCloudPassword` | 2FA Cloud Password SRP-6A calculation and verification |
| `core/deadlock_detector.h` | `DeadlockDetector::Pinger`, `PingThread` | `crates/gpui_telegram_core/src/core/deadlock_detector.rs` | `DeadlockDetector` | Heartbeat watchdog timer, hang detection (60s tick, 30s timeout) |
| `core/external_control.h`, `external_control.cpp` | `Core::HandleExternalControl`, JSON automation IPC | `crates/gpui_telegram_core/src/core/external_control.rs` | `ExternalControl` | Local IPC command router, window activation, automation queries |
| `config.h` | `ApiId`, `ApiHash`, `MaxSelectedItems`, `AutoSearchTimeout` | `crates/gpui_telegram_core/src/core/config.rs` | `Config` | Telegram API credentials, UI limits, timeout constants |
| `core/changelogs.h`, `changelogs.cpp` | `Core::Changelogs`, `FormatVersionDisplay` | `crates/gpui_telegram_core/src/core/changelogs.rs` | `Changelogs` | Local & cloud release changelogs fetcher and formatter |
| `apiwrap.h`, `apiwrap.cpp` | `ApiWrap`, subsystem manager interfaces | `crates/gpui_telegram_core/src/core/apiwrap.rs` | `Apiwrap` | High-level API dispatcher coordinating updates, auth, privacy |
| `logs.h` | `Logs::start`, `writeMain`, `writeDebug`, `writeMtp` | `crates/gpui_telegram_core/src/core/logs.rs` | `Logs` | Structured rotating file logger, MTP packet logger |
| `_other/packer.h`, `packer.cpp` | `Updates::MakeUpdateVersion`, `Updates::VerifyUpdate` | `crates/gpui_telegram_core/src/core/packer.rs` | `Packer` | Binary update package builder, LZMA compression, manifest sign |
| `stdafx.h` | Global shared headers, core re-exports | `crates/gpui_telegram_core/src/core/stdafx.rs` | `Stdafx` | Common prelude re-exports for core data structures |
| `core/utils.h`, `utils.cpp` | `HashMd5`, `hashSha256`, `translitRusEng`, `rowscount` | `crates/gpui_telegram_core/src/core/utils.rs` | `Utils` | Cryptographic digests, string transliteration, math utils |
| `core/websocket_client.h`, `websocket_client.cpp` | `Core::WebSocketClient`, RFC 6455 frame parser | `crates/gpui_telegram_core/src/core/websocket_client.rs` | `WebsocketClient` | Async WebSocket client over TLS, frame masking, ping/pong |
| `_other/zxcvbn_translit.cpp` | `Ui::TransliterateCyrillic`, `DecodeUtf8` | `crates/gpui_telegram_core/src/core/zxcvbn_translit.rs` | `ZxcvbnTranslit` | Cyrillic-to-Latin transliteration table for password entropy |
| `core/current_geo_location.h` | `Core::GeoLocation`, `GeoAddress`, `GeoLocationAccuracy` | `crates/gpui_telegram_core/src/core/current_geo_location.rs` | `CurrentGeoLocation` | Geolocation coordinate container, reverse geocoding resolver |
| `core/credits_amount.h` | `CreditsAmount`, `CreditsType` (Stars / Ton), nano normalization | `crates/gpui_telegram_core/src/core/credits_amount.rs` | `CreditsAmount` | Telegram Stars & TON fixed-point currency type ($10^9$ nanos) |
| `core/proxy_rotation_manager.h` | `Core::ProxyRotationManager`, proxy probe scheduler | `crates/gpui_telegram_core/src/core/proxy_rotation_manager.rs` | `ProxyRotationManager` | Multi-proxy health checker, automatic failover selector |
| `core/cached_webview_availability.h` | `Webview::Availability`, `Webview::Available` | `crates/gpui_telegram_core/src/core/cached_webview_availability.rs` | `CachedWebviewAvailability` | Platform webview engine availability cache (WebKit/WebView2) |
| `core/base_integration.h`, `base_integration.cpp` | `Core::BaseIntegration`, event loop trampoline | `crates/gpui_telegram_core/src/core/base_integration.rs` | `BaseIntegration` | Core logging, crash annotation bridge, and event dispatch |

---

## 3. Application Lifecycle Finite State Machine

```
              ┌─────────────────────────┐
              │      Uninitialized      │
              └────────────┬────────────┘
                           │ Launcher::exec() / Sandbox::start()
                           ▼
              ┌─────────────────────────┐
       ┌─────►│        Starting         │
       │      └────────────┬────────────┘
       │                   │ Application::new() / load_settings()
       │                   ▼
       │      ┌─────────────────────────┐
       │      │         Active          │◄───────────────────────┐
       │      └──────┬────────────┬─────┘                        │
       │             │            │                              │
       │  Auto-Lock  │            │ User Minimize / Hide         │ Window Restore /
       │  or Manual  │            │                              │ User Active
       │             ▼            ▼                              │
       │      ┌────────────┐┌───────────┐                        │
       │      │   Locked   ││Background │────────────────────────┘
       │      └──────┬─────┘└─────┬─────┘
       │             │            │
       │             │ Passcode   │
       │             │ Verified   │
       │             └────────────┘
       │
       │ Application::quit() / SIGINT / SIGTERM
       ▼
┌─────────────────────────┐
│       Terminating       │
└────────────┬────────────┘
             │ Flush DB, cancel sync, release single-instance lock
             ▼
┌─────────────────────────┐
│       Terminated        │
└─────────────────────────┘
```

### Lifecycle States
1. **Starting**: Parses CLI options, acquires single instance socket/lock (`Sandbox`), initializes logging (`Logs`), checks for previous crash dump (`CrashReports`), loads persisted `CoreSettings`.
2. **Active**: Main window visible, event loop processing input, network sync active, periodic timers running (auto-lock, proxy rotation, update checker).
3. **Background**: Main window minimized to tray / taskbar, high-frequency render animations paused, background network push preserved.
4. **Locked**: Triggered when inactivity exceeds `auto_lock_duration` or explicit lock shortcut triggered; UI obscured with passcode box, decryption keys wiped from working cache.
5. **Terminating**: Saves pending settings (`save_delayed`), checkpoints SQLite database, flushes write-ahead logs, closes active calls, releases lockfile, exits process cleanly.

---

## 4. CoreSettings Serialization Schema & Defaults

The `CoreSettings` struct maintains persistent desktop configuration:

```rust
pub struct CoreSettings {
    // Window state
    pub window_position: WindowPosition,
    pub title_content: WindowTitleContent,
    pub work_mode: WorkMode,
    pub close_behavior: CloseBehavior,

    // Audio & Media
    pub song_volume: f64,          // default: 0.9 (0.0 .. 1.0)
    pub video_volume: f64,         // default: 0.9 (0.0 .. 1.0)
    pub ask_download_path: bool,   // default: false
    pub download_path: String,     // default: ""
    pub playback_device_id: String,
    pub capture_device_id: String,

    // Notifications
    pub sound_notify: bool,        // default: true
    pub desktop_notify: bool,      // default: true
    pub flash_bounce_notify: bool, // default: true
    pub notify_view: NotifyView,   // default: ShowPreview
    pub native_notifications: bool,// default: true
    pub notifications_count: u32,  // default: 3
    pub notifications_corner: ScreenCorner, // default: BottomRight
    pub include_muted_counter: bool,        // default: false
    pub count_unread_messages: bool,        // default: false

    // Security & Session
    pub auto_lock: i32,            // seconds (0 = disabled, default: 3600)
    pub screen_is_locked: bool,
    pub moderate_mode_enabled: bool,
    pub adaptive_for_wide: bool,

    // Proxy Settings
    pub proxy: CoreSettingsProxy,
}
```

Serialization format: JSON-compatible binary payload or UTF-8 JSON with schema versioning tag `settings_v1`.

---

## 5. Shortcuts Registry & Key Bindings Router

Shortcuts map physical key combinations (`KeySequence`: modifiers + key code) to semantic `Command` enums:

- **Window & Navigation**: `Close`, `Lock`, `Minimize`, `Quit`, `ReopenClosedWindow`, `CloseOtherWindows`.
- **Chat Switching**: `ChatPrevious` (`Ctrl+Alt+Up`), `ChatNext` (`Ctrl+Alt+Down`), `ChatFirst`, `ChatLast`, `ChatPinned1..8` (`Ctrl+1..8`).
- **Account & Folder Switching**: `ShowAccount1..6` (`Ctrl+Alt+1..6`), `ShowFolder1..6` (`Ctrl+Shift+1..6`), `FolderNext`, `FolderPrevious`.
- **Composer & Sending**: `JustSendMessage` (`Enter`), `SendSilentMessage` (`Shift+Enter` alternate), `ScheduleMessage`, `ShowRichEditor`, `ToggleWebPagePreview`.
- **Media Playback**: `MediaPlay`, `MediaPause`, `MediaPlayPause`, `MediaPrevious`, `MediaNext`.

Router Architecture:
- `Shortcuts::register_handler(command, handler)`
- `Shortcuts::dispatch(event) -> bool`
- Inactive when typing in inputs unless modifier is held.

---

## 6. Crash Reporting & Deadlock Detection

### Crash Reporter
- Installs global panic hook in `CrashReports::start()`.
- Captures stack traces, OS architecture, app version, last breadcrumbs (`SetAnnotation("action", ...)`).
- On subsequent launch, `Sandbox::check_last_crash()` detects dump and presents `CrashReportWindow` allowing user to review anonymized dump, configure proxy, and transmit or dismiss.

### Deadlock Detector
- `DeadlockDetector` runs a dedicated background watchdog thread (`PingThread`).
- Posts ping events to main event loop at 60s intervals.
- If main event loop fails to respond within 30s, deadlock watchdog captures thread backtraces and logs critical panic.

---

## 7. Version & Update Checking FSM

- Version representation:
  - `AppVersion`: Integer encoding (e.g., `7003003` for `7.3.3`)
  - `AppVersionStr`: `"7.3.3"`
  - `AppId`: Official application GUID
- Update FSM (`UpdateChecker::State`):
  - `Idle`: Timer countdown between checks (8 hours + jitter).
  - `Checking`: Querying backend manifest endpoint.
  - `Download`: Streaming update payload with progress byte counting.
  - `Ready`: SHA-256 and RSA signature verified via `Packer::verify()`.
  - `Failed`: Network or signature verification error; backoff scheduled.

---

## 8. Verification Strategy

1. **Compilation**: `cargo check -p gpui_telegram_core` must pass with 0 errors.
2. **Zero Boilerplate**: 0 occurrences of Pattern A, B, C, or D boilerplate.
3. **Symbol Parity**: All assigned symbols exported and matching expected ledger entries.
4. **Hygiene**: `python3 scripts/clean_build_garbage.py` executed after check.


---

## 9. File Utilities, Quarantine Flags & Safe Path Checks

### 9.1 Overview & Responsibilities
Upstream source files: `core/file_utilities.h`, `core/file_utilities.cpp`, `core/file_location.h`, `core/file_location.cpp`.

In the desktop environment, file interaction spans opening URLs, revealing files in platform file managers (macOS Finder, Windows Explorer, Linux DBus/FreeDesktop file managers), safe downloads directory resolution, file bookmarking for sandbox persistence, and quarantine attribute enforcement.

### 9.2 Path & Filename Sanitation
1. **Default Download Path**:
   - Resolves via `QStandardPaths::DownloadLocation` + subfolder (`Telegram Desktop` or `Telegram Lite`).
   - If sandbox restricts default download path, falls back to session temp directory.
2. **Auto-Increment Filename Generator** (`filedialogNextFilename` / `filedialogDefaultName`):
   - Format: `<prefix>_YYYY-MM-DD_HH-mm-ss.<ext>`.
   - Collisions are safely resolved by testing existence and appending incremented counter: `"<name> (2).<ext>"`, `"<name> (3).<ext>"`.
3. **URL & File Launch Safety**:
   - Strips internal web authentication tokens (`UrlWithoutWebAuthTokens`) to prevent account hijacking when third parties share crafted links.
   - External URLs invoke default OS browser asynchronously on the main loop.
   - Mailto links (`OpenEmailLink`) validate email address format before calling desktop services.
   - Local file execution (`Launch` / `OpenWith`): prevents delayed activation and queries platform dropdown or default app.

### 9.3 File Location & Sandboxed Bookmarking
The `FileLocation` struct (`core/file_location.h`) tracks persistent media cache and local files:
- `fname`: full path or special sentinel `*media_cache*` (`InMediaCacheLocation()`).
- `modified`: timestamp used for validation check.
- `size`: size capped at 4000 MB (`kMaxFileSize`).
- `bookmark`: OS bookmark bytes (e.g. macOS security-scoped bookmark `CFURLCreateBookmarkDataFromURL`) enabling sandboxed re-access across app relaunches without prompting the user.
- `check()`: verifies readable status, size match, and modification time match against recorded metadata.

---

## 10. MIME Types Database & Safe Extension Mapping

### 10.1 Upstream Architecture
Upstream source files: `core/mime_type.h`, `core/mime_type.cpp`.

Desktop Telegram implements strict MIME classification and filename extension analysis to protect users from malicious file execution, IP leaks through embedded external resources, and incorrect rendering.

### 10.2 Known MIME Types & Custom Formats
- `image/webp`: WebP images & static stickers (magic bytes check: `RIFF....WEBP`).
- `application/x-tgsticker`: Animated Telegram Lottie stickers (`.tgs`, gzip-compressed JSON).
- `application/x-tgwallpattern`: Wallpaper patterns (`.tgv`).
- `application/x-tdesktop-theme`, `application/x-tgtheme-tdesktop`: Desktop themes (`.tdesktop-theme`).
- `application/x-tdesktop-palette`: Palette overrides (`.tdesktop-palette`).

### 10.3 NameType Classification (`DetectNameType`)
Files are categorized into disjoint semantic categories:
1. `Executable`: Executable binaries and scripts that must show warning dialogs before execution.
   - Windows: `exe, bat, cmd, com, msi, vbs, ps1, reg, scr, lnk, dll, cpl, ...`
   - macOS: `app, command, terminal, workflow, applescript, action, pkg, mpkg, ...`
   - Linux: `bin, sh, run, desktop, deb, rpm, out, ...`
2. `Image`: `jpg, jpeg, png, gif, webp, svg, bmp, ico, tiff, heic, avif, jxl, psd, ...`
3. `Video`: `mp4, mov, avi, mkv, webm, flv, 3gp, m4v, ...`
4. `Audio`: `mp3, ogg, opus, flac, wav, aac, m4a, wma, ...`
5. `Document`: `pdf, doc, docx, xls, xlsx, ppt, pptx, txt, rtf, odt, csv, epub, ...`
6. `Archive`: `zip, rar, 7z, tar, gz, bz2, xz, zst, ...`
7. `ThemeFile`: `tdesktop-theme, tdesktop-palette, attheme, tgios-theme`
8. `OtherBenign`: Programming source code, configs, hashes, logs.
9. `Unknown`: Fallback.

### 10.4 IP-Revealing Path Protection (`IsIpRevealingPath`)
Certain file formats can trigger outbound network requests when opened or previewed (e.g., SVG with external `<image>`, HTML with `<img src="...">`, Microsoft Office documents with external OLE links, KML/KMZ Google Earth overlays). `IsIpRevealingPath` flags these files so the UI displays explicit privacy warnings before opening or rendering inline thumbnails.

---

## 11. Click Handlers Architecture

### 11.1 Upstream Architecture
Upstream source files: `core/click_handler_types.h`, `core/click_handler_types.cpp`, `core/basic_click_handlers.h`, `core/bank_card_click_handler.h`, `core/phone_click_handler.h`.

In GPUI Telegram, rich text elements, formatted message entities, mentions, hashtags, and interactive widgets attach `ClickHandler` instances. When clicked or activated via keyboard, the handler executes contextual actions:

```
                  ┌──────────────────────┐
                  │     ClickHandler     │
                  └──────────┬───────────┘
                             │
     ┌───────────────────────┼───────────────────────┐
     ▼                       ▼                       ▼
┌──────────────┐     ┌──────────────┐       ┌─────────────────┐
│UrlClickHandle│     │TextClickHandl│       │ Specialized     │
└──────┬───────┘     └──────┬───────┘       │ Handlers        │
       │                    │               └────────┬────────┘
       ▼                    ▼                        ▼
HiddenUrlClickHandler  MentionClickHandler    BankCardClickHandler
BotGameUrlClickHandler HashtagClickHandler    PhoneClickHandler
                       CashtagClickHandler    MentionNameClickHandler
                       BotCommandClickHandler FormattedDateClickHandler
                       MonospaceClickHandler  TonAddressClickHandler
```

### 11.2 Context & Execution Semantics
- `ClickHandlerContext`: Encapsulates message item ID (`FullMsgId`), delegate callbacks, weak session window pointer, UI show controller, bot auto-login suppression flags, and external URL confirmation overrides.
- `UrlRequiresConfirmation`: External links prompt user with target domain inspection, except verified Telegram internal domains (`telegram.org`, `t.me`, `telesco.pe`).
- `BankCardClickHandler`: Formats bank card numbers with Luhn check validation, allows one-click copy with toast notification, and routes to payment forms.
- `PhoneClickHandler`: Formats E.164 phone numbers, provides quick actions to initiate Telegram Voice Call, send message to contact, or copy phone number.
- `MentionNameClickHandler`: Handles inline user mentions by user ID + access hash, routing to peer profile view without requiring a public username.
- `TonAddressClickHandler`: Detects TON wallet raw (`0:...`) and user-friendly base64url (`EQ...` / `UQ...`) addresses, routing to wallet send flow or blockchain explorer.

---

## 12. Deep Links URL Grammar & Action Routing Table

### 12.1 Deep Links Architecture
Upstream source files: `core/deep_links/deep_links_types.h`, `core/deep_links/deep_links_router.h`, `core/deep_links/deep_links_router.cpp`, `core/deep_links/deep_links_chats.cpp`, `core/deep_links/deep_links_contacts.cpp`, `core/deep_links/deep_links_new.cpp`, `core/deep_links/deep_links_settings.cpp`, `core/local_url_handlers.cpp`.

Telegram deep links follow two equivalent representations:
1. Custom URI scheme: `tg://<command>[?<params>]`
2. HTTPS canonical redirect: `https://t.me/<command>[?<params>]`

The `DeepLinksRouter` normalizes and parses the command into:
`Context { section, path, params: HashMap<String, String>, controller }`.

### 12.2 Action Routing Table

| URI Scheme Pattern | Target Section & Path | Target Handler / Action | Required Parameters / Notes |
|---|---|---|---|
| `tg://resolve?domain=<username>&start=<token>` | `resolve` | Opens peer chat, passes `/start <token>` | `domain`, optional `start`, `startgroup`, `post` |
| `tg://join?invite=<hash>` | `join` | Resolves chat invite, displays Join Chat dialog | `invite` hash |
| `tg://addstickers?set=<short_name>` | `addstickers` | Opens sticker set preview box | `set` name |
| `tg://addtheme?slug=<slug>` | `addtheme` | Opens cloud theme preview & install box | `slug` |
| `tg://msg_url?url=<url>&text=<text>` | `msg_url` | Opens fast share box with prepopulated text | `url`, `text` |
| `tg://settings` | `settings/` | Opens Settings drawer | Defaults to Main settings |
| `tg://settings/privacy` | `settings/privacy` | Opens Privacy & Security settings | Sub-controls: `passcode`, `2fa`, `sessions` |
| `tg://settings/notifications` | `settings/notifications` | Opens Notifications settings | Sub-controls: `sound`, `preview`, `badge` |
| `tg://settings/folders` | `settings/folders` | Opens Chat Folders management | |
| `tg://settings/language` | `settings/language` | Opens Language selection dialog | |
| `tg://settings/devices` | `settings/devices` | Opens Active Sessions / Devices list | |
| `tg://contacts` | `contacts/` | Opens Contacts list modal | |
| `tg://contacts/new` | `contacts/new` | Opens Add Contact modal | |
| `tg://chats/search` | `chats/search` | Focuses global message search input | |
| `tg://chats/emoji-status` | `chats/emoji-status` | Opens Emoji Status picker in main menu | |
| `tg://new/group` | `new/group` | Opens New Group wizard | |
| `tg://new/channel` | `new/channel` | Opens New Channel wizard | |
| `tg://newbot?manager=<bot>&name=<n>` | `newbot` | Opens Managed Bot creation wizard | Requires bot owner permissions |
| `tg://socks` / `tg://proxy` | `proxy` | Opens Proxy configuration modal | `server`, `port`, `user`, `pass`, `secret` |
| `tg://passport` | `passport` | Opens Telegram Passport credentials form | Nonce, bot_id, scope |
| `tg://tonconnect?v=2&id=<id>` | `tonconnect` | Intercepts TonConnect v2 wallet request | Connects to dApp via internal wallet |

---

## 13. Multi-Platform Auto-Updater Mechanics

### 13.1 Architecture & Security Model
Upstream source files: `_other/updater.h`, `_other/updater_linux.cpp`, `_other/updater_osx.m`, `_other/updater_win.cpp`, `core/update_channel.h`, `core/update_keys.h`, `core/update_unpack.h`, `core/update_verify.h`.

Telegram Desktop updates are packaged in a cryptographically signed binary container:
```
┌──────────────────────────────────────────────────────────┐
│ Magic: "TDUP" (4 bytes)                                  │
├──────────────────────────────────────────────────────────┤
│ Format Version: uint32 (currently 2)                     │
├──────────────────────────────────────────────────────────┤
│ Target OS (Windows=0, Mac=1, Linux=2) & Arch (x86/x64/arm│
├──────────────────────────────────────────────────────────┤
│ Target Version: uint64 (high 32: base, low 32: counter)  │
├──────────────────────────────────────────────────────────┤
│ Key ID & Algorithm (Ed25519 or ES256)                    │
├──────────────────────────────────────────────────────────┤
│ Digital Signature over Header + Payload Digest           │
├──────────────────────────────────────────────────────────┤
│ Payload: Compressed archive (LZMA / TAR / ZIP)           │
└──────────────────────────────────────────────────────────┘
```

### 13.2 Key Hierarchy & Verification
1. **Root Public Key (`RootPublicKeyPem`)**: Embedded at compile time.
2. **Key Manifest (`EmbeddedManifest`)**: Signed by Root key, contains list of active signing keys with expiration dates.
3. **Payload Verification (`update_verify.rs`)**:
   - Matches target OS and architecture against running client.
   - Ensures target version is strictly greater than running version (anti-rollback protection).
   - Validates Ed25519 signature over package bytes.
4. **Unpacking & Atomicity (`update_unpack.rs`)**:
   - Payload is decompressed into temporary staging directory (`tupdate_XXXXXX`).
   - File hashes verified against manifest.
   - Platform updater executable is staged alongside updated application files.

### 13.3 Platform-Specific Updater Implementations
- **Windows (`updater_win.rs`, `_other/updater_win.cpp`)**:
  - Waits for parent `Telegram.exe` process to terminate using `WaitForSingleObject(processHandle, 60000)`.
  - Replaces `Telegram.exe` and DLLs atomically via `MoveFileEx(..., MOVEFILE_REPLACE_EXISTING)`.
  - Relaunches main application with `-workdir` and clean startup flags.
- **macOS (`updater_osx.rs`, `_other/updater_osx.m`)**:
  - Updates `.app` bundle via atomic folder swap.
  - Fixes file quarantine attributes (`com.apple.quarantine`) and code signature cache.
- **Linux (`updater_linux.rs`, `_other/updater_linux.cpp`)**:
  - Atomic rename into `~/.local/share/TelegramDesktop/` or system binary path.
  - Updates desktop entry file (`telegramdesktop.desktop`) and icon resources.

---

## 14. System Tray Integration & Test Harness

### 14.1 System Tray & Multi-Account Menu
Upstream source files: `tray.h`, `tray.cpp`, `tray_accounts_menu.h`, `tray_accounts_menu.cpp`, `tray_accounts_menu_dummy.cpp`.
- `Tray`: Manages system status indicator, tray icon badge counters (unread chats count, mention badges), and tooltip formatting (`"Telegram Desktop (3)"`).
- Context menu actions: Minimize/Restore, Toggle Mute Notifications, Settings, Multi-Account Quick Switcher, Quit.
- `TrayAccountsMenu`: Dynamically enumerates active `Main::Account` instances, displays user avatars, handles unread counts per account, and switches active domain session on click.

### 14.2 End-to-End Test Harness & Verification Fixtures
Upstream source files: `tde2e/`, `test/`.
- `TdE2E`: Deterministic End-to-End test integration framework supporting synthetic user actions, network latency emulation, and encrypted call verification.
- Test Fixtures (`test_log_lines`, `test_marking_read`, `test_menu`, `test_palette`, `test_text`, `test_update_verify`, `test_wallet_intercept`, etc.):
  - Provide automated verification for individual UI components, logging pipelines, formatting models, and cryptography without requiring external network or live sessions.
