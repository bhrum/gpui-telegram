# Module 08: Core Dialog Boxes & Confirmation Layer Spec

## 1. Executive Summary & Architectural Scope
This specification defines the 1:1 parity architecture between upstream Telegram Desktop (tdesktop C++) modal dialog layers and the GPUI-based Telegram Desktop implementation in `crates/gpui_telegram_desktop/src/views/boxes/`.

In upstream Telegram Desktop, modal dialogs inherit from `Ui::BoxContent` or utilize `Ui::GenericBox` and are displayed via `Ui::show(object_ptr<BoxType>)` or window-level layer managers (`Ui::LayerWidget`). This module covers core modal workflows:
1. `abstract_box`: The foundational modal layer container, managing overlay backdrop, dismissal on background click or Escape key, smooth alpha/scale entrance animations, and header/footer action slots.
2. `confirm_box`: General-purpose confirmation and informative dialogs (`ConfirmBox`, `InformBox`, `MakeConfirmBox`, `MakeInformBox`) with configurable confirmation/cancellation text, dangerous-action red buttons, enter/return keyboard triggers, and strict cancellation handling.
3. `passcode_box`: Local passcode lock/unlock and 2FA cloud password configuration, including PIN input fields, hint handling, password recovery, and error animations.
4. `auto_lock_box`: Passcode auto-lock interval configuration with standard discrete intervals (1m, 5m, 1h, 5h) and custom MM:SS time specification.
5. `delete_box` & `delete_messages_box`: Destructive confirmation dialogs for deleting individual messages, multi-message selections, and clearing/wiping full chat histories, with peer-specific "Delete for everyone" revoke toggles.
6. `about_box` & `about_sponsored_box`: Application version metadata, build channel (release/beta/canary/debug), FAQ localized link resolution, GPL v3 source disclosure, and Telegram sponsored messages transparency dialogs.
7. `connection_box`: Proxy connection management (Socks5, MTProto, HTTP/Web), real-time latency ping indicators, shareable `tg://` proxy links, and proxy auto-rotation intervals.
8. `share_box`: Fast sharing of links, messages, and media to multiple contacts/chats with forward options and optional message captions.

---

## 2. Mandate #19 Pillar 2 Reading Manifest (40 Upstream Files, 11,912 Lines)
Every file assigned in the ledger and upstream tdesktop repository has been completely inspected and analyzed.

| # | Upstream Source Path | Lines | Core Logic & Extracted State | Target GPUI Component |
|---|----------------------|-------|------------------------------|-----------------------|
| 1 | `Telegram/SourceFiles/boxes/abstract_box.h` | 51 | `Ui::showBox`, `Ui::show`, `Ui::hideLayer`, `Ui::isLayerShown` layer controller APIs | `views/boxes/abstract_box.rs` |
| 2 | `Telegram/SourceFiles/boxes/abstract_box.cpp` | 48 | Global window layer delegation to `Core::App().activePrimaryWindow()` | `views/boxes/abstract_box.rs` |
| 3 | `Telegram/SourceFiles/ui/boxes/confirm_box.h` | 70 | `ConfirmBoxArgs`, `ConfirmBox`, `InformBox`, `MakeConfirmBox`, `IconWithTitle` declarations | `views/boxes/confirm_box.rs` |
| 4 | `Telegram/SourceFiles/ui/boxes/confirm_box.cpp` | 159 | Enter/Return key accelerator, callback dispatch, cancel-on-close binding, title/label row layout | `views/boxes/confirm_box.rs` |
| 5 | `Telegram/SourceFiles/boxes/share_box.h` | 217 | `ShareBox`, `FastShareLink`, `FastShareMessage`, `ShareBoxStyleOverrides`, `Descriptor` | `views/boxes/share_box.rs` |
| 6 | `Telegram/SourceFiles/boxes/share_box.cpp` | 2410 | Multi-peer selection, forward options, comment input, recipient filtering, sending dispatch | `views/boxes/share_box.rs` |
| 7 | `Telegram/SourceFiles/boxes/share_box.style` | 35 | `shareBox`, `shareMultiSelect`, `shareComment`, `shareTitle` metric tokens | `views/boxes/share_box.rs` |
| 8 | `Telegram/SourceFiles/boxes/passcode_box.h` | 276 | `PasscodeBox`, `CloudFields`, password verification, email recovery, SRP check callbacks | `views/boxes/passcode_box.rs` |
| 9 | `Telegram/SourceFiles/boxes/passcode_box.cpp` | 1732 | Local passcode change/remove, 2FA cloud password SRP verification, error shakes, hint/recovery | `views/boxes/passcode_box.rs` |
| 10 | `Telegram/SourceFiles/boxes/passcode_box.style` | 39 | `passcodePadding`, `passcodeSkip`, `passcodeAboutSkip`, `passcodeTextLine` | `views/boxes/passcode_box.rs` |
| 11 | `Telegram/SourceFiles/boxes/auto_lock_box.h` | 28 | `AutoLockBox`, `durationChanged` signal declaration | `views/boxes/auto_lock_box.rs` |
| 12 | `Telegram/SourceFiles/boxes/auto_lock_box.cpp` | 151 | `kOptions` (60, 300, 3600, 18000, max), `kDefaultCustom`, radio list, time parser, settings update | `views/boxes/auto_lock_box.rs` |
| 13 | `Telegram/SourceFiles/boxes/delete_messages_box.h` | 88 | `DeleteMessagesBox`, `RevokeConfig`, wipe history modes, `hasScheduledMessages` checks | `views/boxes/delete_messages_box.rs` |
| 14 | `Telegram/SourceFiles/boxes/delete_messages_box.cpp` | 587 | `kDeleteMessagesBoxAnimationDuration` (80ms), wipe history by date/peer, revoke checkbox, remember choice | `views/boxes/delete_messages_box.rs`, `views/boxes/delete_box.rs` |
| 15 | `Telegram/SourceFiles/boxes/about_box.h` | 22 | `AboutBox`, `telegramFaqLink`, `currentVersionText`, `currentVersionShortText` | `views/boxes/about_box.rs` |
| 16 | `Telegram/SourceFiles/boxes/about_box.cpp` | 334 | Version link formatting, alpha link generation, platform architecture tags, FAQ locale paths | `views/boxes/about_box.rs` |
| 17 | `Telegram/SourceFiles/boxes/about_sponsored_box.h` | 16 | `AboutSponsoredBox` declaration | `views/boxes/about_sponsored_box.rs` |
| 18 | `Telegram/SourceFiles/boxes/about_sponsored_box.cpp` | 76 | `kUrl` ("https://ads.telegram.org"), description paragraphs, ads policy button, dismiss logic | `views/boxes/about_sponsored_box.rs` |
| 19 | `Telegram/SourceFiles/boxes/connection_box.h` | 151 | `ProxiesBoxController`, `ItemView`, `ItemState`, `ProxyData`, proxy management actions | `views/boxes/connection_box.rs` |
| 20 | `Telegram/SourceFiles/boxes/connection_box.cpp` | 2624 | `kSaveSettingsDelayedTimeout` (1000ms), `ProxyDataToQueryPath`, QR rendering, ping, IPv6 fallback | `views/boxes/connection_box.rs` |
| 21 | `Telegram/SourceFiles/boxes/connection_box.style` | 89 | `proxyRow`, `proxyPing`, `proxyMenu`, `connectionBox` sizing and color metrics | `views/boxes/connection_box.rs` |
| 22 | `Telegram/SourceFiles/boxes/boxes.style` | 773 | General box metrics (`boxPadding`, `boxWideWidth`, `boxTitle`, `boxButton`, `boxRadius`) | `views/boxes/mod.rs` |
| 23 | `Telegram/SourceFiles/ui/boxes/confirm_phone_box.h` | 70 | `ConfirmPhoneBoxArgs`, phone verification callbacks | `views/boxes/confirm_box.rs` |
| 24 | `Telegram/SourceFiles/ui/boxes/confirm_phone_box.cpp` | 187 | Phone confirmation dialog layout, call timer, validation code input fields | `views/boxes/confirm_box.rs` |
| 25 | `Telegram/SourceFiles/ui/boxes/single_choice_box.h` | 32 | `SingleChoiceBox` single-select radio list modal dialog | `views/boxes/confirm_box.rs` |
| 26 | `Telegram/SourceFiles/ui/boxes/single_choice_box.cpp` | 55 | Radio button list builder, current value selection, submit callback dispatch | `views/boxes/confirm_box.rs` |
| 27 | `Telegram/SourceFiles/ui/boxes/about_cocoon_box.h` | 17 | `AboutCocoonBox` declaration | `views/boxes/about_box.rs` |
| 28 | `Telegram/SourceFiles/ui/boxes/about_cocoon_box.cpp` | 319 | Cocoon browser integration dialog, title, description, URL links | `views/boxes/about_box.rs` |
| 29 | `Telegram/SourceFiles/ui/boxes/choose_time.h` | 26 | `ChooseTimeBox` time selection popup declaration | `views/boxes/auto_lock_box.rs` |
| 30 | `Telegram/SourceFiles/ui/boxes/choose_time.cpp` | 156 | Hour/Minute time sliders, discrete step increments, time formatted strings | `views/boxes/auto_lock_box.rs` |
| 31 | `Telegram/SourceFiles/ui/boxes/time_picker_box.h` | 22 | `TimePickerBox` widget declaration | `views/boxes/auto_lock_box.rs` |
| 32 | `Telegram/SourceFiles/ui/boxes/time_picker_box.cpp` | 132 | Time picker layout, wheel event handling, AM/PM formatting | `views/boxes/auto_lock_box.rs` |
| 33 | `Telegram/SourceFiles/ui/boxes/rate_call_box.h` | 51 | `RateCallBox` audio/video call rating star rating box | `views/boxes/confirm_box.rs` |
| 34 | `Telegram/SourceFiles/ui/boxes/rate_call_box.cpp` | 149 | 1-5 star selection, optional feedback text field, submission payload | `views/boxes/confirm_box.rs` |
| 35 | `Telegram/SourceFiles/ui/boxes/show_or_premium_box.h` | 32 | `ShowOrPremiumBox` paywall upsell modal declaration | `views/boxes/about_sponsored_box.rs` |
| 36 | `Telegram/SourceFiles/ui/boxes/show_or_premium_box.cpp` | 218 | Premium feature preview, unlock button, subscribe CTA dispatch | `views/boxes/about_sponsored_box.rs` |
| 37 | `Telegram/SourceFiles/boxes/self_destruction_box.h` | 52 | `SelfDestructionBox` account self-destruction ttl selector | `views/boxes/auto_lock_box.rs` |
| 38 | `Telegram/SourceFiles/boxes/self_destruction_box.cpp` | 231 | 1 month, 3 months, 6 months, 1 year account lifetime radio options | `views/boxes/auto_lock_box.rs` |
| 39 | `Telegram/SourceFiles/boxes/unconfirmed_auth_denied_box.h` | 21 | `UnconfirmedAuthDeniedBox` security alert dialog | `views/boxes/confirm_box.rs` |
| 40 | `Telegram/SourceFiles/boxes/unconfirmed_auth_denied_box.cpp` | 166 | Session termination warning, IP / device details, security lock confirmation | `views/boxes/confirm_box.rs` |

---

## 3. Component Specifications & State Architecture

### 3.1 AbstractBox (`abstract_box.rs`)
- **Upstream Basis**: `Telegram/SourceFiles/boxes/abstract_box.h:L28-L51`, `abstract_box.cpp:L15-L48`.
- **Core Purpose**: Serves as the top-level backdrop overlay and modal chrome for all dialog boxes in the application.
- **Visual Presentation**:
  - Semi-transparent backdrop overlay: `rgba(0, 0, 0, 0.45)`.
  - Box container: rounded corners `px(10.0)`, background `rgb(0xFFFFFF)` (or dark mode `rgb(0x212121)`), soft drop shadow.
  - Centered alignment on screen with dynamic dimension constraints (minimum width 320px, maximum width 560px, default 380px).
- **Interactions & Semantics**:
  - Click on background overlay outside modal bounds triggers dismissal if `close_on_click_outside` is enabled (default `true`).
  - Escape key triggers dismissal callback.
  - Layer stack management: tracks active nested box layers and unwinds top-to-bottom.
### 3.2 ConfirmBox & InformBox (`confirm_box.rs`)
- **Upstream Basis**: `Telegram/SourceFiles/ui/boxes/confirm_box.h:L13-L71`, `confirm_box.cpp:L17-L160`.
- **Core Purpose**: Reusable dialog for binary user decisions (Confirm vs Cancel) or informational alerts (OK only).
- **Architecture**:
  - `ConfirmBoxArgs`:
    - `title`: Optional dialog title header string.
    - `text`: Primary informative or prompt body text.
    - `confirm_text`: Custom primary action label (defaults to "OK" or "Delete" for destructive flows).
    - `cancel_text`: Custom cancel action label (defaults to "Cancel").
    - `inform`: When `true`, hides the Cancel button and renders as single-button `InformBox`.
    - `strict_cancel`: When `true`, cancellation callback executes exclusively when Cancel is explicitly clicked, not on general window close.
    - `is_danger`: When `true`, renders the confirm button with red attention styling (`#E53935` / `#D32F2F`).
  - **Keyboard Accelerator**: Enter / Return key automatically triggers the confirm action (`confirm_box.cpp:L82-L92`).
  - **Button Order**: Upstream tdesktop places Confirm on the right and Cancel on the left (or Cancel first in bottom-right row flow depending on platform conventions).

### 3.3 PasscodeBox (`passcode_box.rs`)
- **Upstream Basis**: `Telegram/SourceFiles/boxes/passcode_box.h:L33-L200`, `passcode_box.cpp:L344-L443`.
- **Core Purpose**: Local application lock passcode management (create, change, disable) and 2FA cloud password authentication.
- **State Machine**:
  - `PasscodeMode`: `Create`, `Change`, `TurnOff`, `CheckOnly`.
  - Field inputs:
    - `old_passcode`: Required when `Change` or `TurnOff`.
    - `new_passcode`: Required for `Create` and `Change`.
    - `reenter_passcode`: Required for confirmation match during `Create` and `Change`.
    - `hint`: Optional user-specified password hint.
  - Validation:
    - Minimum length check (4 characters/digits).
    - Match verification between `new_passcode` and `reenter_passcode`.
    - Error state tracking with shake animation signal and red error text display.
  - Salt & Hash:
    - Real SHA-256 password hashing with user salt (`storage/storage_domain.h`, `passcode_box.cpp`).
### 3.4 AutoLockBox (`auto_lock_box.rs`)
- **Upstream Basis**: `Telegram/SourceFiles/boxes/auto_lock_box.h:L16-L29`, `auto_lock_box.cpp:L21-L151`.
- **Core Purpose**: Configures the inactivity period before Telegram locks itself and prompts for the passcode.
- **Constants**:
  - `kOptions`: `[60, 300, 3600, 18000, i32::MAX]` (`auto_lock_box.cpp:L22`).
    - 60s: 1 minute (`tr::lng_minutes(1)`)
    - 300s: 5 minutes (`tr::lng_minutes(5)`)
    - 3600s: 1 hour (`tr::lng_hours(1)`)
    - 18000s: 5 hours (`tr::lng_hours(5)`)
    - `kCustom`: User-specified interval (defaults to `10:00` or custom MM:SS).
- **Behavior**:
  - Radio button selection with custom time input widget.
  - Parsing custom string `mm:ss` into total seconds (`minutes * 60 + seconds`).
  - Automatic persistence to local settings (`Core::App().settings().setAutoLock(seconds)`).

### 3.5 DeleteBox & DeleteMessagesBox (`delete_box.rs`, `delete_messages_box.rs`)
- **Upstream Basis**: `Telegram/SourceFiles/boxes/delete_messages_box.h:L26-L89`, `delete_messages_box.cpp:L43-L240`.
- **Core Purpose**:
  - `delete_box`: Lightweight, fast-action dialog for single or small batch message deletion with a "Delete for everyone" checkbox.
  - `delete_messages_box`: Full Telegram modal supporting multi-message deletion, wipe-history by date range, clear channel history, and "Remember choice" preferences.
- **Destructive Action Highlight**:
  - Delete button is colored with `st::attentionBoxButton` (red text/background on hover).
- **Revoke Semantics**:
  - Peer-specific checkbox: "Also delete for {PeerName}".
  - Channels / Supergroups: indicates permanent removal for all subscribers.
  - SlideWrap animation for `remember_choice` when user toggles away from default preference.
### 3.6 AboutBox & AboutSponsoredBox (`about_box.rs`, `about_sponsored_box.rs`)
- **Upstream Basis**:
  - `Telegram/SourceFiles/boxes/about_box.h:L12-L23`, `about_box.cpp:L67-L188`.
  - `Telegram/SourceFiles/boxes/about_sponsored_box.h:L10-L17`, `about_sponsored_box.cpp:L20-L77`.
- **Core Purpose**:
  - `about_box`: Displays official application branding, current build version, platform architecture (x64, arm64, macOS, Windows, Linux), clickable FAQ URL, source code licenses (GNU GPL v3), and alpha build download link copier.
  - `about_sponsored_box`: Explains Telegram sponsored messages privacy policies with an external link button to `https://ads.telegram.org`.
- **FAQ Link Localization**:
  - `telegramFaqLink()` checks language codes (`de`, `es`, `it`, `ko`, `pt-br` -> `br`) and appends them to `https://telegram.org/faq`.
- **Version Formatting**:
  - Full version text with commit hash for canary builds, architecture tags, and debug build indicator (`about_box.cpp:L155-L178`).

### 3.7 ConnectionBox (`connection_box.rs`)
- **Upstream Basis**: `Telegram/SourceFiles/boxes/connection_box.h:L37-L108`, `connection_box.cpp:L70-L195`.
- **Core Purpose**: Comprehensive proxy configuration and server status inspection.
- **Proxy Types**:
  - `Socks5`: host, port, username, password.
  - `Mtproto`: host, port, secret (hex/base64 encoded).
  - `WebProxy`: HTTPS-based proxy transport.
  - `None`: Direct internet connection.
- **Link Generation & Sharing**:
  - `ProxyDataToQueryPath`: generates query string `?server=...&port=...&secret=...`.
  - `ProxyDataToLocalLink`: creates deep-link `tg://proxy?server=...` or `tg://socks?server=...`.
- **Latency & Status Tracking**:
  - Item states: `Online`, `Checking`, `Available`, `Unavailable`.
  - Ping in milliseconds displayed next to each configured proxy.
  - Auto-rotation timeout settings with discrete intervals.
### 3.8 ShareBox (`share_box.rs`)
- **Upstream Basis**: `Telegram/SourceFiles/boxes/share_box.h:L63-L140`, `share_box.cpp:L74-L200`.
- **Core Purpose**: Multi-contact message forwarding and direct link sharing dialog.
- **Features**:
  - `FastShareLink`: Quick sharing of URL to recent or selected chats.
  - `FastShareMessage`: Forwards existing messages or media with caption support.
  - Multi-select recipient list with search filter.
  - Forward options toggle (hide sender name, hide captions).
  - Copy link action button for external clipboard sharing.

---

## 4. Exact Metrics & Styling Constants (Parity with `boxes.style` & Upstream C++)
| Metric Name | Value | Upstream C++ / Style Definition |
|-------------|-------|---------------------------------|
| `box_default_width` | `380px` | `st::boxWideWidth` / `st::boxWidth` |
| `box_wide_width` | `440px` | `st::boxWideWidth` (`styles/style_boxes.h`) |
| `box_radius` | `10px` | `st::boxRadius` |
| `box_padding_top` | `24px` | `st::boxPadding.top()` |
| `box_padding_horizontal` | `24px` | `st::boxPadding.left()` |
| `box_padding_bottom` | `18px` | `st::boxPadding.bottom()` |
| `box_button_height` | `36px` | `st::defaultBoxButton.height` |
| `box_button_radius` | `6px` | `st::defaultBoxButton.roundRadius` |
| `box_button_padding` | `16px` | `st::defaultBoxButton.padding` |
| `k_autolock_options` | `[60, 300, 3600, 18000, MAX]` | `auto_lock_box.cpp:L22` |
| `k_delete_messages_animation_ms` | `80ms` | `delete_messages_box.cpp:L39` |
| `k_save_settings_timeout_ms` | `1000ms` | `connection_box.cpp:L70` |
| `color_overlay_backdrop` | `rgba(0, 0, 0, 0.45)` | `st::layerBg` |
| `color_danger_action` | `#E53935` | `st::attentionBoxButton.textFg` |
| `color_primary_button` | `#5288C1` | `st::defaultBoxButton.textFg` |
| `color_card_bg` | `#FFFFFF` (Dark: `#212121`) | `st::boxBg` |

---

## 5. Verification & Subsystem Testing Strategy
All implemented GPUI components will be comprehensively validated via integration unit tests in `crates/gpui_telegram_desktop/tests/boxes_core_subsystem_tests.rs`:
1. **AbstractBox**: Backdrop layout, click-outside dismissal behavior, custom content embedding, header/footer button bar slots.
2. **ConfirmBox & InformBox**: Default OK / Cancel action buttons, custom text rendering, dangerous attention highlight, keyboard Enter key triggers, strict cancel semantics.
3. **PasscodeBox**: Local passcode validation, match checking, PIN error shake state, salt & hashing verification.
4. **AutoLockBox**: Standard timeout selection (1m, 5m, 1h, 5h), custom time string parsing (`MM:SS`), persistence callback dispatch.
5. **DeleteBox & DeleteMessagesBox**: Message count formatting, "delete for everyone" revoke toggle, destructive styling, remember choice state.
6. **AboutBox & AboutSponsoredBox**: App version formatting, FAQ link resolution with multi-locale rules, sponsored ads policy URL link handling.
7. **ConnectionBox**: Proxy data structure encoding (`tg://proxy?server=...`), ping metric updates, proxy type switching.
8. **ShareBox**: Recipient selection toggles, link copy generation, forward options flags.
