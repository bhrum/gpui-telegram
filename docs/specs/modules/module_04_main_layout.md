# Module 04 Specification: Main & Layout Subsystem (`main_layout`)

## Overview

Module 04 (`main` & `layout`) provides the core session lifecycle, multi-account routing, application configuration, top-level window coordination, and responsive layout positioning engine for the Telegram Desktop Rust/GPUI rewrite.

This module unifies two deeply interdependent subsystems from upstream Telegram Desktop:
1. **Main Subsystem (`main`)**: The foundational domain controllers (`Main::Domain`, `Main::Account`, `Main::Session`), account credential and passcode derivation, dynamic MTProto configuration (`AppConfig`), per-session user preferences (`SessionSettings`), send-as-peer identity switching (`SendAsPeers`), and the primary desktop window and widget controllers (`MainWindow`, `MainWidget`).
2. **Layout Subsystem (`layout`)**: The layout positioning engine for chat items, message bubbles, inline mosaics, grouped media albums, and file attachment previews (`AbstractLayoutItem`, `LayoutItemBase`, `Layout::Position`, `LayoutSelection`, `MosaicLayout`, and `DocumentGenericPreview`).

---

## 1. Upstream C++ to Rust GPUI Target Ledger Mapping

The module encompasses 33 upstream C++ source files (21 in `main.json` and 12 in `layout.json`) mapped into 17 distinct target Rust files across `crates/gpui_telegram_desktop/src/app/`, `crates/gpui_telegram_desktop/src/views/stage/layout/`, and the client-side domain engine in `crates/gpui_telegram_client/src/main/`:

### Subsystem `main` Mappings (`docs/specs/ledger/shards/main.json`)

| Upstream C++ Source Files | Upstream C++ Classes / Types | Target Rust File | Target Symbol | Subsystem & Feature | Primary Responsibilities |
|---|---|---|---|---|---|
| `Telegram/SourceFiles/main.cpp` | `main`, `Core::Launcher::Create` | `crates/gpui_telegram_desktop/src/app/main.rs` | `Main` | main / `TCG-N46` | Desktop application entrypoint, OpenSSL crypto initialization with `OPENSSL_INIT_NO_ATEXIT`, launcher bootstrap. |
| `main/main_account.h`, `.cpp` | `Account`, `DestroyReason`, `ProxyChange` | `crates/gpui_telegram_desktop/src/app/main_account.rs` | `MainAccount` | main / `TCG-N46` | Account controller: MTP instance binding, auth keys, session creation, logout / forced logout, AppConfig ownership. |
| `main/main_app_config.h`, `.cpp` | `AppConfig`, `StarsColoring` | `crates/gpui_telegram_desktop/src/app/main_app_config.rs` | `MainAppConfig` | main / `TCG-N46` | Dynamic MTProto JSON configuration (`help.getAppConfig`), typed getters, stars/crypto exchange rates, limits. |
| `main/main_app_config_values.h`, `.cpp` | `FragmentLink` | `crates/gpui_telegram_desktop/src/app/main_app_config_values.rs` | `MainAppConfigValues` | main / `TCG-N46` | Value extraction helpers, Ton fragment links, username auction URL resolvers. |
| `main/main_domain.h`, `.cpp` | `Domain`, `AccountWithIndex` | `crates/gpui_telegram_desktop/src/app/main_domain.rs` | `MainDomain` | main / `TCG-N46` | Top-level domain controller, multi-account manager (3 standard, 6 premium), single-flight passcode derivation, active account switching, global unread badge tally. |
| `main/main_session.h`, `.cpp` | `Session`, `FreezeInfo` | `crates/gpui_telegram_desktop/src/app/main_session.rs` | `MainSession` | main / `TCG-N02` | Authenticated session controller, user peer binding, uniqueId (with test DC shift), freeze info, terms lock, tmp passwords, window controller tracking. |
| `main/main_session_settings.h`, `.cpp` | `SessionSettings` | `crates/gpui_telegram_desktop/src/app/main_session_settings.rs` | `MainSessionSettings` | main / `TCG-N02` | Binary serialized per-session user preferences, support flags, selector tab state, hidden sticker sections, auto-download configuration, archived chats mode. |
| `main/session/send_as_peers.h`, `.cpp` | `SendAsPeers`, `SendAsPeer`, `SendAsKey`, `SendAsType` | `crates/gpui_telegram_desktop/src/app/session/send_as_peers.rs` | `SendAsPeersSession` | main / `TCG-N02` | Sending identity switcher (user vs channel/group), SendAsKey (Message, PaidReaction, VideoStream), auto-refresh, and peer resolution. |
| `main/session/session_show.h`, `.cpp` | `SessionShow`, `SimpleSessionShow`, `MakeSessionShow` | `crates/gpui_telegram_desktop/src/app/session/session_show.rs` | `SessionShowSession` | main / `TCG-N02` | UI presentation context wrapping session-scoped modal dialogs, toasts, and frozen account error alerts. |
| `Telegram/SourceFiles/mainwidget.h`, `.cpp` | `MainWidget`, `StackItem` | `crates/gpui_telegram_desktop/src/app/mainwidget.rs` | `Mainwidget` | main / `TCG-N46` | Central 3-column / 2-column / 1-column responsive layout coordinator: dialogs list (left), history stage (center), third section (right), drag-drop routing. |
| `Telegram/SourceFiles/mainwindow.h`, `.cpp` | `MainWindow` | `crates/gpui_telegram_desktop/src/app/mainwindow.rs` | `Mainwindow` | main / `TCG-N46` | Top-level desktop window: passcode lock screen, setup email lock, intro onboarding, main content widget, modal layer stack manager. |

### Subsystem `layout` Mappings (`docs/specs/ledger/shards/layout.json`)

| Upstream C++ Source Files | Upstream C++ Classes / Types | Target Rust File | Target Symbol | Subsystem & Feature | Primary Responsibilities |
|---|---|---|---|---|---|
| `layout/abstract_layout_item.h`, `.cpp` | `AbstractLayoutItem`, `PaintContextBase` | `crates/gpui_telegram_desktop/src/views/stage/layout/abstract_layout_item.rs` | `AbstractLayoutItem` | layout / `TCG-N04` | Fundamental layout item bounding box, maxWidth/minHeight metrics, resizeGetHeight calculation, position index, shift offsets, point hit testing. |
| `layout/layout_document_generic_preview.h`, `.cpp` | `DocumentGenericPreview`, `ColorIndexForName` | `crates/gpui_telegram_desktop/src/views/stage/layout/layout_document_generic_preview.rs` | `LayoutDocumentGenericPreview` | layout / `TCG-N04` | 4-color palette document preview generator (Blue=0 docs/text, Green=1 tables, Red=2 pdf/slides, Yellow=3 archives/media, modulo fallback), file extension parser. |
| `layout/layout_item_base.h`, `.cpp` | `LayoutItemBase` | `crates/gpui_telegram_desktop/src/views/stage/layout/layout_item_base.rs` | `LayoutItemBase` | layout / `TCG-N04` | Base class for history view layout items, text state query (`getState`), text selection adjustment (`adjustSelection`). |
| `layout/layout_mosaic.h`, `.cpp` | `AbstractMosaicLayout`, `MosaicLayout`, `FoundItem` | `crates/gpui_telegram_desktop/src/views/stage/layout/layout_mosaic.rs` | `LayoutMosaic` | layout / `TCG-N04` | Responsive mosaic layout algorithm for inline media/GIFs, max 5 items per row, proportional width distribution, point hit-testing, dirty row validation. |
| `layout/layout_position.h`, `.cpp` | `Position`, `IndexToPosition`, `PositionToIndex` | `crates/gpui_telegram_desktop/src/views/stage/layout/layout_position.rs` | `LayoutPosition` | layout / `TCG-N04` | Matrix position encoding with `kMatrixRowShift = 40000`: `IndexToPosition(index) -> (row, col)`, `PositionToIndex(row, col) -> index`. |
| `layout/layout_selection.h`, `.cpp` | `TextSelection`, `IsSubGroupSelection`, `AddGroupItemSelection` | `crates/gpui_telegram_desktop/src/views/stage/layout/layout_selection.rs` | `LayoutSelection` | layout / `TCG-N04` | Packed 16-bit selection representation for grouped media items (up to 15 items in a group), bitwise selection mask manipulation. |

---

## 2. Main Subsystem Architecture & Domain Hierarchy

```
                          ┌───────────────────────┐
                          │     Main::Domain      │
                          │  (Multi-Account Mgr)  │
                          └──────────┬────────────┘
                                     │
           ┌─────────────────────────┼─────────────────────────┐
           ▼                         ▼                         ▼
 ┌───────────────────┐     ┌───────────────────┐     ┌───────────────────┐
 │   Account 0       │     │   Account 1       │     │   Account N (≤6)  │
 │  (Main::Account)  │     │  (Main::Account)  │     │  (Main::Account)  │
 └─────────┬─────────┘     └───────────────────┘     └───────────────────┘
           │
           ├───► MTP::Instance (Network Session)
           ├───► Main::AppConfig (Remote JSON Config)
           └───► Main::Session (Active Authenticated State)
                       │
                       ├───► UserId / PeerId
                       ├───► Main::SessionSettings (User Prefs)
                       ├───► Main::SendAsPeers (Identity Switcher)
                       └───► Window::SessionController & MainWidget
```

### 2.1 Main::Domain (Multi-Account Domain Controller)
`Main::Domain` acts as the root singleton coordinating all accounts, local passcode protection, and active session routing:
- **Account Limits**: Supports up to 3 concurrent accounts for standard users (`kMaxAccounts = 3`) and up to 6 accounts for Telegram Premium subscribers (`kPremiumMaxAccounts = 6`).
- **Passcode Protection & Derivation**:
  - `tryPasscode(passcode, done)`: Enforces a single-flight derivation lock (`_passcodeDeriving`). While a derivation worker runs, subsequent attempts return `false` without invoking callbacks.
  - Automatically clears passcodes upon final account logout (`clearPasscodeAfterLastLogout`).
- **Active Account & Session Streams**:
  - Maintains `_active` (`Account*`) and fires events via `activeChanges()` and `activeSessionChanges()`.
  - Exposes `maybeLastOrSomeAuthedAccount()` to restore the last active authenticated account upon startup.
- **Global Aggregate Unread Badge**:
  - `_unreadBadge`: Aggregated unread message and mention count across all active accounts.
  - `_unreadBadgeMuted`: Flag indicating whether all accounts with unread messages are muted.

### 2.2 Main::Account (Account State & Protocol Binding)
`Main::Account` encapsulates the identity, storage, and networking for a single Telegram account:
- **Index & Data Directory**: Identified by `index` (0..5) and storage subfolder `dataName`.
- **MTP Protocol Instance**: Owns an `MTP::Instance` for MTProto transport, DC handshakes, and RPC dispatch.
- **Session Lifecycle**:
  - `prepareToStart`: Initializes MTProto config and keys.
  - `createSession`: Constructs `Main::Session` when user credentials and authorization are established.
  - `logOut` / `forcedLogOut`: Gracefully tears down session, cancels active transfers, and marks account for re-login.

### 2.3 Main::Session (Authenticated User Session)
`Main::Session` represents an active authenticated login session:
- **Identity Properties**:
  - `userId()`: 64-bit Telegram user ID.
  - `userPeerId()`: Associated `PeerId` (User peer type).
  - `uniqueId()`: User ID shifted by test DC offset for test-mode isolation.
- **Freeze & Moderation Tracking**:
  - `FreezeInfo`: Contains `since` timestamp, `until` timestamp, and `appealUrl`. When active, restricts posting and triggers frozen alert dialogs.
  - `TermsLock`: Manages terms of service acceptance gates.
- **Temporary Password Support**:
  - `setTmpPassword(password, validUntil)`: Stores temporary passwords for 2FA actions.
  - `validTmpPassword()`: Validates expiration before providing cached credentials.
- **Session Settings**: Owns `Main::SessionSettings`.
- **SendAsPeers Controller**: Owns `Main::SendAsPeers`.

### 2.4 Main::SessionSettings (Per-Session Configuration & Preferences)
`SessionSettings` persists user preferences across restarts:
- **Serialization**: Compact binary serialization with version header.
- **Features & Flags**:
  - Support mode switches (`supportSwitch`, `supportFixChatsOrder`, `supportTemplatesAutocomplete`, `supportChatsTimeSlice`).
  - Active selector tab (Emoji, Stickers, GIFs).
  - Hidden sticker/emoji sets per chat.
  - Auto-download configurations (`Data::AutoDownload::Full`).
  - Archived chats display mode (`archiveCollapsed`, `archiveInMainMenu`).
  - Hidden pinned message tracking per chat and topic.

### 2.5 Main::AppConfig & AppConfigValues (Dynamic Remote Config)
`Main::AppConfig` parses the dynamic JSON dictionary returned from MTProto `help.getAppConfig`:
- **Typed Accessors with Safe Fallbacks**:
  - `quoteLengthMax()`: Max characters in blockquote replies (default 1024).
  - `stargiftConvertPeriodMax()`: Max seconds to convert star gifts into stars (default 7776000, 90 days).
  - `starrefSetupAllowed()` / `starrefJoinAllowed()`: Referral program flags.
  - `starsWithdrawRate()` / `currencyWithdrawRate()`: Exchange rates for Telegram Stars.
  - `pollOptionsLimit()`: Max options in a poll (default 10).
  - `todoListItemsLimit()`: Max items in a checklist/todo (default 30).
  - `confcallSizeLimit()`: Max participants in group video calls (default 1000).
- **Fragment Links**: Resolves Ton fragment auction and star ref URLs.

### 2.6 Main::SendAsPeers (Identity Switcher)
`Main::SendAsPeers` coordinates posting on behalf of channels or public groups:
- **Key Types**: `SendAsType::Message`, `SendAsType::PaidReaction`, `SendAsType::VideoStream`.
- **Selection Resolution**: `resolveChosen(peer)` verifies whether the selected identity is still valid in the current channel's permitted sender list; falls back to user peer if revoked.

### 2.7 MainWidget & MainWindow (GPUI Responsive Layout & Window Controller)
`MainWidget` coordinates the responsive multi-column layout:
- **Adaptive Columns**:
  - **3-Column Mode** (Width >= 1200px): Left dialogs list, center history stage, right info drawer.
  - **2-Column Mode** (Width >= 800px): Left dialogs list, center history stage (drawer opens as modal layer).
  - **1-Column Mode** (Width < 800px): Single view with back-button navigation stack.
- **Chat Stack Navigation**: Push and pop navigation stack with slide animations.
- **MainWindow Layer Stack**: Manages full-window modal overlays (Passcode Lock, Setup Email, Media Viewer, Box content).

---

## 3. Layout Subsystem Architecture & Positioning Engine

### 3.1 AbstractLayoutItem & LayoutItemBase
`AbstractLayoutItem` is the abstract base geometry container:
- **Dimensions**:
  - `_maxw`: Maximum allowable width (intrinsic content width).
  - `_minh`: Minimum allowable height.
  - `_width`, `_height`: Computed rendered dimensions.
  - `resizeGetHeight(int width)`: Reflows layout for a given target width and computes new height:
    $$\text{width} = \min(\text{width}, \text{maxw}), \quad \text{height} = \text{minh}$$
- **Coordinate Shifts & Hit Testing**:
  - `_shiftX`, `_shiftY`: Local coordinate translation within parent container.
  - `hasPoint(point)`: Returns `true` if `(0, 0, width, height)` contains `point`.
  - `_position`: 1D or matrix position index (`< 0` indicates item removed from layout).

### 3.2 Matrix Position Encoding (`Layout::Position`)
In composite layouts (such as sticker grids, emoji selectors, and GIF mosaics), 2D row/column coordinates are packed into a single 32-bit integer:
- **Constant**: `kMatrixRowShift = 40000`
- **Encoding**:
  $$\text{Index} = \text{row} \times 40000 + \text{column}$$
- **Decoding**:
  $$\text{row} = \begin{cases} \text{index} / 40000 & \text{if } \text{index} \ge 0 \\ -1 & \text{otherwise} \end{cases}$$
  $$\text{column} = \begin{cases} \text{index} \% 40000 & \text{if } \text{index} \ge 0 \\ -1 & \text{otherwise} \end{cases}$$

### 3.3 Packed Group Selection (`LayoutSelection`)
For multi-item message groupings (such as 2–10 photos in an album), Telegram avoids allocating separate selection objects by packing sub-item selection state into a 16-bit integer pair (`TextSelection { from: u16, to: u16 }`):
- **Full Selection**: `from = 0xFFFF, to = 0xFFFF` (entire group selected).
- **Sub-group Selection Guard**:
  $$\text{IsSubGroupSelection}(s) \iff (s.\text{from} == 0xFFFF) \land (s.\text{to} \ne 0xFFFF)$$
- **Bitmask Item Selection**:
  Item $i$ ($0 \le i < 15$) is selected if and only if bit $i$ is set in $s.\text{to}$:
  $$\text{IsGroupItemSelection}(s, i) \iff \text{IsSubGroupSelection}(s) \land ((s.\text{to} \ \& \ (1 \ll i)) \ne 0)$$
- **Selection Modification**:
  $$\text{AddGroupItemSelection}(s, i) = \text{TextSelection} \{ 0xFFFF, \text{IsSubGroupSelection}(s) ? (s.\text{to} \mid (1 \ll i)) : (1 \ll i) \}$$
- **First Selected Index**: Scans bits $0 \dots 14$ and returns the lowest index set, or $-1$.

### 3.4 Mosaic Layout Algorithm (`MosaicLayout`)
`MosaicLayout` positions variable-aspect media items (GIFs, images, stickers) into aligned rows:
1. **Constraints**:
   - Max 5 items per row: `kInlineItemsMaxPerRow = 5`.
   - Row finalized when items reach 5, or accumulated width exceeds target width, or `force` is true.
2. **Width Distribution Algorithm**:
   - For a finalized row of items $I_0 \dots I_{k-1}$ and available container width $W_{\text{avail}}$:
   - Sort item indices by ascending `maxWidth()`.
   - Distribute available width proportionally:
     $$w_i = \max\left(\frac{I_i.\text{maxWidth}() \times W_{\text{avail}}}{\sum_{j} I_j.\text{maxWidth}()}, \text{minWidth}\right)$$
   - Subtract allocated width and padding from remaining available width.
   - Set row height to $\max_i(I_i.\text{resizeGetHeight}(w_i))$.
3. **Point Hit-Testing (`findByPoint`)**:
   - Performs binary/sequential search across rows by cumulative heights.
   - Searches row items by cumulative widths.
   - Returns `FoundItem { index, exact, relative }` with matrix index and local coordinate offset.

### 3.5 Generic Document Attachment Preview (`DocumentGenericPreview`)
When a document has no image thumbnail, Telegram renders an iconic file badge categorized into 4 semantic color classes:
1. **Color Classification Rules**:
   - **Color 0 (Blue)**: Word documents, plain text, Photoshop files (`.doc`, `.docx`, `.txt`, `.psd`), or any MIME type starting with `text/`.
   - **Color 1 (Green)**: Spreadsheets and tabular data (`.xls`, `.xlsx`, `.csv`).
   - **Color 2 (Red)**: PDF and presentation decks (`.pdf`, `.ppt`, `.pptx`, `.key`).
   - **Color 3 (Yellow)**: Compressed archives, audio, and video containers (`.zip`, `.rar`, `.ai`, `.mp3`, `.mov`, `.avi`).
   - **Fallback Classification**:
     If extension does not match known sets, hash the first character of the extension (or filename/MIME):
     $$\text{colorIndex} = (\text{char.unicode}() \% 4) \ \& \ 3$$
2. **Palette Styling Tokens**:
   - Each color index specifies primary background (`color`), dark badge tone (`dark`), hover state (`over`), and selected highlight (`selected`).

---

## 4. Headless Client Domain Engine (`crates/gpui_telegram_client/src/main/`)

To support headless CLI operations, background daemon sync, and fast test suites without initializing GPUI display contexts, the client crate includes a pure domain representation of the `main` subsystem under `crates/gpui_telegram_client/src/main/`:
- `DomainEngine`: Coordinates multiple accounts, storage vaults, and active session tokens.
- `AccountEngine`: Handles background MTProto authentication, RPC calls, and session state.
- `SessionEngine`: In-memory authenticated session state, temporary password stores, and send-as resolution.

---

## 5. Implementation Roadmap & Verification Protocol

1. **Phase 1: Zero Boilerplate Replacement**:
   - Overwrite all 11 target files in `crates/gpui_telegram_desktop/src/app/` with authentic structs, methods, and state machines.
   - Overwrite all 6 target files in `crates/gpui_telegram_desktop/src/views/stage/layout/` with authentic layout positioning logic.
   - Provide client-side main engine in `crates/gpui_telegram_client/src/main/` and desktop layout module re-exports in `crates/gpui_telegram_desktop/src/layout/`.
2. **Phase 2: Comprehensive Integration Test Suite**:
   - Author integration tests in `crates/gpui_telegram_client/tests/main_layout_subsystem_tests.rs`.
   - Verify matrix position shift math (`40000` multiplier and modulo), group selection bitmask math (`IsGroupItemSelection`, `AddGroupItemSelection`), mosaic layout item distribution, document 4-color categorization, domain account limits (3 standard, 6 premium), and session freeze state.
3. **Phase 3: Compilation & Build Hygiene**:
   - Execute `cargo check -p gpui_telegram_client -p gpui_telegram_desktop`.
   - Execute `cargo test -p gpui_telegram_client --test main_layout_subsystem_tests`.
   - Run `python3 scripts/clean_build_garbage.py`.
4. **Phase 4: Gate 6.2 Ledger Closure Verification**:
   - Execute `python3 scripts/verify_ledger_closure.py`.
   - Confirm 0 boilerplate files in `main.json` and `layout.json` targets.
