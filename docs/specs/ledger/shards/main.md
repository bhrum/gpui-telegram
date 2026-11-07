# Subsystem Shard: `main`

**Total Source Files**: 21  
**Total Source Lines**: 10,007  
**Primary Feature IDs**: TCG-N02, TCG-N46  
**JSON Shard Path**: `docs/specs/ledger/shards/main.json`  

## Component Inventory & Parity Mapping

| Source Path | Feature ID | Target Symbol | Target Rust / GPUI Path | Lines | Test ID |
|---|---|---|---|---|---|
| `Telegram/SourceFiles/main.cpp` | `TCG-N46` | `Main` | `crates/gpui_telegram_desktop/src/app/main.rs` | 24 | `tests/test_tcg_n46_main.rs` |
| `Telegram/SourceFiles/main/main_account.cpp` | `TCG-N46` | `MainAccount` | `crates/gpui_telegram_desktop/src/app/main_account.rs` | 642 | `tests/test_tcg_n46_main_account.rs` |
| `Telegram/SourceFiles/main/main_account.h` | `TCG-N46` | `MainAccount` | `crates/gpui_telegram_desktop/src/app/main_account.rs` | 174 | `tests/test_tcg_n46_main_account.rs` |
| `Telegram/SourceFiles/main/main_app_config.cpp` | `TCG-N46` | `MainAppConfig` | `crates/gpui_telegram_desktop/src/app/main_app_config.rs` | 717 | `tests/test_tcg_n46_main_app_config.rs` |
| `Telegram/SourceFiles/main/main_app_config.h` | `TCG-N46` | `MainAppConfig` | `crates/gpui_telegram_desktop/src/app/main_app_config.rs` | 211 | `tests/test_tcg_n46_main_app_config.rs` |
| `Telegram/SourceFiles/main/main_app_config_values.cpp` | `TCG-N46` | `MainAppConfigValues` | `crates/gpui_telegram_desktop/src/app/main_app_config_values.rs` | 29 | `tests/test_tcg_n46_main_app_config_values.rs` |
| `Telegram/SourceFiles/main/main_app_config_values.h` | `TCG-N46` | `MainAppConfigValues` | `crates/gpui_telegram_desktop/src/app/main_app_config_values.rs` | 18 | `tests/test_tcg_n46_main_app_config_values.rs` |
| `Telegram/SourceFiles/main/main_domain.cpp` | `TCG-N46` | `MainDomain` | `crates/gpui_telegram_desktop/src/app/main_domain.rs` | 628 | `tests/test_tcg_n46_main_domain.rs` |
| `Telegram/SourceFiles/main/main_domain.h` | `TCG-N46` | `MainDomain` | `crates/gpui_telegram_desktop/src/app/main_domain.rs` | 170 | `tests/test_tcg_n46_main_domain.rs` |
| `Telegram/SourceFiles/main/main_session.cpp` | `TCG-N02` | `MainSession` | `crates/gpui_telegram_desktop/src/app/main_session.rs` | 647 | `tests/test_tcg_n02_main_session.rs` |
| `Telegram/SourceFiles/main/main_session.h` | `TCG-N02` | `MainSession` | `crates/gpui_telegram_desktop/src/app/main_session.rs` | 388 | `tests/test_tcg_n02_main_session.rs` |
| `Telegram/SourceFiles/main/main_session_settings.cpp` | `TCG-N02` | `MainSessionSettings` | `crates/gpui_telegram_desktop/src/app/main_session_settings.rs` | 1084 | `tests/test_tcg_n02_main_session_settings.rs` |
| `Telegram/SourceFiles/main/main_session_settings.h` | `TCG-N02` | `MainSessionSettings` | `crates/gpui_telegram_desktop/src/app/main_session_settings.rs` | 262 | `tests/test_tcg_n02_main_session_settings.rs` |
| `Telegram/SourceFiles/main/session/send_as_peers.cpp` | `TCG-N02` | `SendAsPeersSession` | `crates/gpui_telegram_desktop/src/app/session/send_as_peers.rs` | 188 | `tests/test_tcg_n02_send_as_peers.rs` |
| `Telegram/SourceFiles/main/session/send_as_peers.h` | `TCG-N02` | `SendAsPeersSession` | `crates/gpui_telegram_desktop/src/app/session/send_as_peers.rs` | 86 | `tests/test_tcg_n02_send_as_peers.rs` |
| `Telegram/SourceFiles/main/session/session_show.cpp` | `TCG-N02` | `SessionShowSession` | `crates/gpui_telegram_desktop/src/app/session/session_show.rs` | 92 | `tests/test_tcg_n02_session_show.rs` |
| `Telegram/SourceFiles/main/session/session_show.h` | `TCG-N02` | `SessionShowSession` | `crates/gpui_telegram_desktop/src/app/session/session_show.rs` | 28 | `tests/test_tcg_n02_session_show.rs` |
| `Telegram/SourceFiles/mainwidget.cpp` | `TCG-N46` | `Mainwidget` | `crates/gpui_telegram_desktop/src/app/mainwidget.rs` | 3284 | `tests/test_tcg_n46_mainwidget.rs` |
| `Telegram/SourceFiles/mainwidget.h` | `TCG-N46` | `Mainwidget` | `crates/gpui_telegram_desktop/src/app/mainwidget.rs` | 391 | `tests/test_tcg_n46_mainwidget.rs` |
| `Telegram/SourceFiles/mainwindow.cpp` | `TCG-N46` | `Mainwindow` | `crates/gpui_telegram_desktop/src/app/mainwindow.rs` | 793 | `tests/test_tcg_n46_mainwindow.rs` |
| `Telegram/SourceFiles/mainwindow.h` | `TCG-N46` | `Mainwindow` | `crates/gpui_telegram_desktop/src/app/mainwindow.rs` | 151 | `tests/test_tcg_n46_mainwindow.rs` |

## Deep Source Reading Evidence

### `Telegram/SourceFiles/main/main_account.cpp`
- **Classes & Structs**: ProxyChange
- **Core Methods & Signals**: QString::number, Account::Account, Account::~Account, Account::domainLocal, Account::legacyStart, Account::prepareToStart
- **Target**: `MainAccount` in `crates/gpui_telegram_desktop/src/app/main_account.rs`

### `Telegram/SourceFiles/main/main_account.h`
- **Classes & Structs**: Account, Domain, StartResult, AuthKey, Config, Session, SessionSettings, AppConfig, DestroyReason
- **Core Methods & Signals**: Account, prepareToStartAdded, start, createSession, logOut, forcedLogOut
- **Target**: `MainAccount` in `crates/gpui_telegram_desktop/src/app/main_account.rs`

### `Telegram/SourceFiles/main/main_app_config.h`
- **Classes & Structs**: ColorIndicesCompressed, StarsColoring, Account, AppConfig
- **Core Methods & Signals**: AppConfig, start, constexpr, getDouble, getString, getBool
- **Target**: `MainAppConfig` in `crates/gpui_telegram_desktop/src/app/main_app_config.rs`

### `Telegram/SourceFiles/main/main_app_config_values.cpp`
- **Classes & Structs**: Strings
- **Core Methods & Signals**: ranges::find_if, FragmentLink
- **Target**: `MainAppConfigValues` in `crates/gpui_telegram_desktop/src/app/main_app_config_values.rs`

### `Telegram/SourceFiles/main/main_app_config_values.h`
- **Classes & Structs**: Session
- **Target**: `MainAppConfigValues` in `crates/gpui_telegram_desktop/src/app/main_app_config_values.rs`

### `Telegram/SourceFiles/main/main_domain.h`
- **Classes & Structs**: Domain, PasscodeDerivation, StartResult, Environment, VaultRuntime, Account, Session, AccountWithIndex
- **Core Methods & Signals**: Domain, resetWithForgottenPasscode, finish, notifyUnreadBadgeChanged, maybeActivate, activate
- **Target**: `MainDomain` in `crates/gpui_telegram_desktop/src/app/main_domain.rs`

### `Telegram/SourceFiles/main/main_session.cpp`
- **Classes & Structs**: State, Flag
- **Core Methods & Signals**: MTP::ConfigFields, Session::Session, Core::App, Helper::Create, Account::session, Spellchecker::Start
- **Target**: `MainSession` in `crates/gpui_telegram_desktop/src/app/main_session.rs`

### `Telegram/SourceFiles/main/main_session.h`
- **Classes & Structs**: ApiWrap, Updates, SendProgressManager, Instance, ConfigFields, Helper, Templates, FastButtonsBots, Session, Changes, GiftAuctions, RecentInlineBots
- **Core Methods & Signals**: bool, Session, validateSelf, saveSettings, saveSettingsDelayed, saveSettingsNowIfNeeded
- **Target**: `MainSession` in `crates/gpui_telegram_desktop/src/app/main_session.rs`

### `Telegram/SourceFiles/main/main_session_settings.h`
- **Classes & Structs**: SwitchSettings, SelectorTab, SetupEmailState, SessionSettings, ThreadId
- **Core Methods & Signals**: SessionSettings, addFromSerialized, setSupportSwitch, setSupportFixChatsOrder, setSupportTemplatesAutocomplete, setSupportChatsTimeSlice
- **Target**: `MainSessionSettings` in `crates/gpui_telegram_desktop/src/app/main_session_settings.rs`

### `Telegram/SourceFiles/main/session/send_as_peers.cpp`
- **Classes & Structs**: Flag
- **Core Methods & Signals**: SendAsPeers::SendAsPeers, SendAsPeers::shouldChoose, Data::CanSendAnything, SendAsPeers::refresh, SendAsPeers::list, SendAsPeers::updated
- **Target**: `SendAsPeersSession` in `crates/gpui_telegram_desktop/src/app/session/send_as_peers.rs`

