# Module 11: Settings — Core & Main Specification (Slice 7)

## Executive Summary
This design document establishes the authoritative C++ to Rust/GPUI architectural mapping for **Module 11 (`settings`) Slice 7: Settings Core & Main**, covering exactly 40 upstream Telegram Desktop source and header files and 21 materialized Rust GPUI targets.
The subsystem covers:
- Core Settings Infrastructure & Fluent Builder DSL (`Settings::Builder::SectionBuilder`, `Settings::Builder::SearchRegistry`, `Settings::AbstractSection`, `Settings::SectionFactory`, `Settings::Type`).
- Main Settings Overview Screen & Interactive Profile Cover Card (`Settings::Main`, `Settings::Cover`, `CircleRippleIconButton`, multi-account switcher with unread badges, phone copying, quick search trigger).
- Profile Editing & Account Management (`Settings::Information`, `InformationHighlightTargets`, `SetupAccounts`, `UpdatePhotoLocally`, `ComposedBadge`, `kSaveBioTimeout = 1000ms`).
- Telegram Stars & Credits Economics (`Settings::Credits`, `Settings::CreditsGraphics`, `FillCreditOptions`, `AddBalanceWidget`, `AddWithdrawalWidget`, `SmallBalanceSource`, 3D stars particle system).
- Telegram Premium Subscription Management (`Settings::Premium`, `PremiumState`, `TopBarWithSticker`, discount tiers: 1/6/12 months, feature catalog, gifting modal).
- Directional Keyboard Navigation & Shortcuts Management (`Settings::KeyNavigation`, `Settings::Shortcuts`, command lookup, key binding modifications).
- Call Media Devices Configuration (`Settings::Calls`, microphone test level polling `kMicTestUpdateInterval = 100ms`, speaker volume, camera selector).
- Local Passcode Lock & Auto-Lock Security (`Settings::LocalPasscode`, `PasscodeCountdown` with 60s timeout, auto-lock timeouts: 1m, 5m, 1h, 5h).
- Passkeys & Authorized Login Websites (`Settings::Passkeys`, WebAuthn ceremony dispatch, `Settings::Websites`, `kShortPollTimeout = 60s`).
- In-Settings Real-time Search Index & Secret Debug Codes (`Settings::Search`, `SearchResultItem`, `Settings::CodesFeedString`).

---

## 100% Upstream Source Reading Manifest (Slice 7: 40 / 40 Files)

| Source Path | Line Range | Core Types & Symbols Extracted | Constants & Formulas | Target Rust / GPUI Path | Target Symbol |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `Telegram/SourceFiles/settings/detailed_settings_button.h` | L1-L78 | `DetailedSettingsButton`, `DetailedSettingsButtonStyle`, `AddDetailedSettingsButton` | `accessibilityRole`, `setToggleLocked`, `refreshLayout`, `toggleRect` | `crates/gpui_telegram_desktop/src/views/settings/detailed_settings_button.rs` | `DetailedSettingsButton` |
| `Telegram/SourceFiles/settings/detailed_settings_button.cpp` | L1-L306 | `DetailedSettingsButton::paintEvent`, `onStateChanged`, `toggleRect` | `iconSize = 22`, padding = 16, ripple mask animation | `crates/gpui_telegram_desktop/src/views/settings/detailed_settings_button.rs` | `DetailedSettingsButton` |
| `Telegram/SourceFiles/settings/sections/settings_calls.h` | L1-L65 | `CallsId`, `AddCameraSubsection`, `PlaybackDeviceNameValue`, `CaptureDeviceNameValue`, `CameraDeviceNameValue` | `kMicTestUpdateInterval = 100ms`, `kMicTestAnimationDuration = 200ms` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_calls.rs` | `SettingsCallsSections` |
| `Telegram/SourceFiles/settings/sections/settings_calls.cpp` | L1-L916 | `Calls`, `LevelState`, `LevelStateStub`, `ChoosePlaybackDeviceBox`, `ChooseCaptureDeviceBox` | `level = (rms - min_rms) / (max_rms - min_rms)`, audio gain scaling | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_calls.rs` | `SettingsCallsSections` |
| `Telegram/SourceFiles/settings/sections/settings_credits.h` | L1-L43 | `CreditsId`, `CreditsTopupOptions`, `BuyStarsHandler` | `rpl::loadingValue`, currency summary descriptors | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_credits.rs` | `SettingsCreditsSections` |
| `Telegram/SourceFiles/settings/sections/settings_credits.cpp` | L1-L1172 | `Credits`, `Currency`, `SectionFactory`, `BuildCurrencyWithdrawalSection` | Stars rate calculation, withdrawal threshold, TON wallet binding | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_credits.rs` | `SettingsCreditsSections` |
| `Telegram/SourceFiles/settings/sections/settings_information.h` | L1-L62 | `InformationId`, `AccountsEvents`, `SetupAccounts`, `UpdatePhotoLocally`, `Badge::UnreadBadge` | Multi-account limit = 3/6, unread badges count calculation | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_information.rs` | `SettingsInformationSections` |
| `Telegram/SourceFiles/settings/sections/settings_information.cpp` | L1-L1571 | `InformationHighlightTargets`, `ComposedBadge`, `AccountsList`, `Information` | `kSaveBioTimeout = 1000ms`, `kPlayStatusLimit = 2`, bio max length 70/140 | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_information.rs` | `SettingsInformationSections` |
| `Telegram/SourceFiles/settings/sections/settings_local_passcode.h` | L1-L18 | `LocalPasscodeCreateId`, `LocalPasscodeCheckId`, `LocalPasscodeManageId` | Inactivity lock, auto-lock timeouts (0, 60s, 300s, 3600s, 18000s) | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_local_passcode.rs` | `SettingsLocalPasscodeSections` |
| `Telegram/SourceFiles/settings/sections/settings_local_passcode.cpp` | L1-L1714 | `PasscodeCountdown`, `LocalPasscodeEnter`, `LocalPasscodeCreate`, `LocalPasscodeChange` | `kPasscodeSectionTimeout = 60000ms`, `kPasscodeCountdownTick = 1000ms`, `FormatCountdown(%1:%2)` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_local_passcode.rs` | `SettingsLocalPasscodeSections` |
| `Telegram/SourceFiles/settings/sections/settings_main.h` | L1-L46 | `MainId`, `SetupLanguageButton`, `HasInterfaceScale`, `SetupInterfaceScale`, `OpenFaq`, `OpenAskQuestionConfirm` | Interface scale steps (100%..300%), FAQ URL lookup | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_main.rs` | `SettingsMainSections` |
| `Telegram/SourceFiles/settings/sections/settings_main.cpp` | L1-L1259 | `Cover`, `CircleRippleIconButton`, `Main`, `BuildInterfaceScale`, `BuildHelpSection` | `kSugValidatePhone = "VALIDATE_PHONE_NUMBER"`, photo size 80px, nameTop = 16px | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_main.rs` | `SettingsMainSections` |
| `Telegram/SourceFiles/settings/sections/settings_passkeys.h` | L1-L28 | `PasskeysId`, `PasskeysNoneBox` | WebAuthn FIDO2 ceremony declarations | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_passkeys.rs` | `SettingsPasskeysSections` |
| `Telegram/SourceFiles/settings/sections/settings_passkeys.cpp` | L1-L536 | `Passkeys`, `Error`, `Platform::WebAuthn` | Passkey credential registration & challenge verification | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_passkeys.rs` | `SettingsPasskeysSections` |
| `Telegram/SourceFiles/settings/sections/settings_premium.h` | L1-L113 | `PremiumId`, `ShowPremium`, `ShowGiftPremium`, `StartPremiumPayment` | Tier durations: 1, 6, 12 months, discount calculations | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_premium.rs` | `SettingsPremiumSections` |
| `Telegram/SourceFiles/settings/sections/settings_premium.cpp` | L1-L2180 | `Premium`, `TopBarWithSticker`, `Order`, `WeakToast` | Monthly price vs annual discount, starburst gradient parameters | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_premium.rs` | `SettingsPremiumSections` |
| `Telegram/SourceFiles/settings/sections/settings_shortcuts.h` | L1-L25 | `ShortcutsId`, `ShortcutsHighlightId` | Command enum mapping and custom key binding registry | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_shortcuts.rs` | `SettingsShortcutsSections` |
| `Telegram/SourceFiles/settings/sections/settings_shortcuts.cpp` | L1-L868 | `Labeled`, `SetupShortcutsResult`, `Command` | Chat pinned (1..9), show account (1..6), show folder (1..9) | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_shortcuts.rs` | `SettingsShortcutsSections` |
| `Telegram/SourceFiles/settings/sections/settings_websites.h` | L1-L20 | `WebsitesId` | Authorized website session management declarations | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_websites.rs` | `SettingsWebsitesSections` |
| `Telegram/SourceFiles/settings/sections/settings_websites.cpp` | L1-L862 | `Row`, `RowDelegate`, `EntryData`, `DisconnectAll` | `kShortPollTimeout = 60000ms`, session timestamp format | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_websites.rs` | `SettingsWebsitesSections` |
| `Telegram/SourceFiles/settings/settings_builder.h` | L1-L268 | `SectionBuilder`, `SearchEntry`, `SearchRegistry`, `BuildHelper`, `WidgetContext`, `SearchContext` | Fluent builder DSL, search entry indexing, highlight registration | `crates/gpui_telegram_desktop/src/views/settings/settings_builder.rs` | `SettingsBuilder` |
| `Telegram/SourceFiles/settings/settings_builder.cpp` | L1-L494 | `SectionBuilder::addButton`, `SectionBuilder::addSectionButton`, `SectionBuilder::addSubsectionTitle` | Padding = 16, skip height = 8, divider thickness = 1 | `crates/gpui_telegram_desktop/src/views/settings/settings_builder.rs` | `SettingsBuilder` |
| `Telegram/SourceFiles/settings/settings_codes.h` | L1-L18 | `CodesFeedString` | Secret debug command processor declaration | `crates/gpui_telegram_desktop/src/views/settings/settings_codes.rs` | `SettingsCodes` |
| `Telegram/SourceFiles/settings/settings_codes.cpp` | L1-L327 | `UnpackRawGzip`, `GenerateCodes` | Debug commands: `debugmode`, `viewlogs`, `testupdate`, `loadlang`, `testmode`, `endpoints` | `crates/gpui_telegram_desktop/src/views/settings/settings_codes.rs` | `SettingsCodes` |
| `Telegram/SourceFiles/settings/settings_common.h` | L1-L360 | `HighlightArgs`, `HighlightShape`, `AbstractSection`, `IconDescriptor`, `SeparatedToggle`, `SliderWithLabel` | `opacity = 0.4`, `showDelay = 400ms`, `showDuration = 600ms`, `shownDuration = 400ms`, `hideDuration = 600ms` | `crates/gpui_telegram_desktop/src/views/settings/settings_common.rs` | `SettingsCommon` |
| `Telegram/SourceFiles/settings/settings_common.cpp` | L1-L882 | `HighlightOverlay`, `Phase`, `Icon`, `SearchWords`, `MatchesWords` | Highlight phase transitions (FadeIn, Shown, FadeOut), term matching | `crates/gpui_telegram_desktop/src/views/settings/settings_common.rs` | `SettingsCommon` |
| `Telegram/SourceFiles/settings/settings_common_session.h` | L1-L84 | `Container`, `AbstractSectionFactory`, `SectionFactory`, `Section`, `HasMenu` | Section lifecycle container (`Section`, `Layer`), singleton factory | `crates/gpui_telegram_desktop/src/views/settings/settings_common_session.rs` | `SettingsCommonSession` |
| `Telegram/SourceFiles/settings/settings_common_session.cpp` | L1-L24 | `HasMenu` | Dynamic menu availability check for settings section ID | `crates/gpui_telegram_desktop/src/views/settings/settings_common_session.rs` | `SettingsCommonSession` |
| `Telegram/SourceFiles/settings/settings_credits_graphics.h` | L1-L345 | `CreditsEntryBoxStyleOverrides`, `GiftWearBoxStyleOverride`, `SmallBalanceSource`, `SmallBalanceResult` | Small balance source variants (10 variants: Bot, Reaction, Stream, Gift, etc.) | `crates/gpui_telegram_desktop/src/views/settings/settings_credits_graphics.rs` | `SettingsCreditsGraphics` |
| `Telegram/SourceFiles/settings/settings_credits_graphics.cpp` | L1-L3928 | `FillCreditOptions`, `AddBalanceWidget`, `AddWithdrawalWidget`, `GenericCreditsEntryBox` | Star particles count = 40, floating animation period = 2000ms | `crates/gpui_telegram_desktop/src/views/settings/settings_credits_graphics.rs` | `SettingsCreditsGraphics` |
| `Telegram/SourceFiles/settings/settings_faq_suggestions.h` | L1-L43 | `FaqEntry`, `FaqSuggestions` | Cloud FAQ suggestion model (`section`, `title`, `url`) | `crates/gpui_telegram_desktop/src/views/settings/settings_faq_suggestions.rs` | `SettingsFaqSuggestions` |
| `Telegram/SourceFiles/settings/settings_faq_suggestions.cpp` | L1-L161 | `FaqSuggestions::request`, `FaqSuggestions::parse` | MTP web page parser for `telegram.org/faq` anchor extraction | `crates/gpui_telegram_desktop/src/views/settings/settings_faq_suggestions.rs` | `SettingsFaqSuggestions` |
| `Telegram/SourceFiles/settings/settings_intro.h` | L1-L55 | `LayerWidget`, `IntroWidget`, `MinimalSupportedWidth` | Intro onboarding layer container | `crates/gpui_telegram_desktop/src/views/settings/settings_intro.rs` | `SettingsIntro` |
| `Telegram/SourceFiles/settings/settings_intro.cpp` | L1-L562 | `TopBar`, `CreateIntroSettings` | Top bar height = 54px, title styling, scroll area setup | `crates/gpui_telegram_desktop/src/views/settings/settings_intro.rs` | `SettingsIntro` |
| `Telegram/SourceFiles/settings/settings_key_navigation.h` | L1-L60 | `KeyNavigation`, `KeyNavigation::Entry` | Keyboard entry list, navigation anchor geometry | `crates/gpui_telegram_desktop/src/views/settings/settings_key_navigation.rs` | `SettingsKeyNavigation` |
| `Telegram/SourceFiles/settings/settings_key_navigation.cpp` | L1-L407 | `KeyNavigation::handle`, `KeyNavigation::select`, `KeyNavigation::activate` | Directional vector math (Euclidean distance across centers) | `crates/gpui_telegram_desktop/src/views/settings/settings_key_navigation.rs` | `SettingsKeyNavigation` |
| `Telegram/SourceFiles/settings/settings_notifications_common.h` | L1-L125 | `NotificationsCount`, `NotifyViewCheckboxes`, `SplitToggle`, `ScreenCorner`, `kMaxNotificationsCount = 5` | Corner geometry preview, small/large sample dimensions | `crates/gpui_telegram_desktop/src/views/settings/settings_notifications_common.rs` | `SettingsNotificationsCommon` |
| `Telegram/SourceFiles/settings/settings_search.h` | L1-L93 | `Search`, `SearchSectionState`, `IndexedEntry`, `ResultCustomization` | Search indexer cache, query tokenizer, match ranking | `crates/gpui_telegram_desktop/src/views/settings/settings_search.rs` | `SettingsSearch` |
| `Telegram/SourceFiles/settings/settings_search.cpp` | L1-L717 | `Search::rebuildResults`, `Search::buildIndex`, `Search::bumpRecentEntry` | First-letter bucket indexing, prefix match bonus = 2x score | `crates/gpui_telegram_desktop/src/views/settings/settings_search.rs` | `SettingsSearch` |
| `Telegram/SourceFiles/settings/settings_type.h` | L1-L15 | `AbstractSectionFactory`, `Type` | Type = `std::shared_ptr<AbstractSectionFactory>` section identity | `crates/gpui_telegram_desktop/src/views/settings/settings_type.rs` | `SettingsType` |


---

## Detailed Architectural Analysis by Cluster

### Cluster 1: Settings Infrastructure, Section Lifecycle & Fluent Builder DSL
Upstream Telegram Desktop organizes settings navigation and rendering using a declarative builder and type-erased section lifecycle:
1. **Type & Section Identity (`settings_type.h:L10-L15`, `settings_common_session.h:L37-L80`)**:
   - Every settings section is identified by a singleton `Settings::Type = std::shared_ptr<AbstractSectionFactory>`.
   - `SectionFactory<SectionType>` creates instances on-demand, caching section geometry and state transitions.
   - Sections can be embedded in a standalone `Container::Section` or an overlay `Container::Layer`.
2. **Fluent Builder DSL (`settings_builder.h:L53-L266`, `settings_builder.cpp:L1-L494`)**:
   - `SectionBuilder` supports dual-mode operation: active widget instantiation (`WidgetContext`) and headless index harvesting for in-settings search (`SearchContext`).
   - `addButton`, `addSectionButton`, `addPremiumButton`, `addPrivacyButton`, `addCheckbox`, `addSubsectionTitle`, `addDivider`, and `addSkip`.
   - Automatically registers search keywords (`SearchEntry`) and widget animation targets (`HighlightRegistry`).
3. **Interactive Detailed Button (`detailed_settings_button.h:L16-L79`, `detailed_settings_button.cpp:L25-L306`)**:
   - Renders a multi-line row with icon, title, description, badge count, and toggle switch (`Ui::ToggleView`).
   - Supports locked toggles (`setToggleLocked`), ripple clicks (`prepareRippleMask`), and accessible role state reporting.
4. **Widget Highlighting & Reveal Animations (`settings_common.h:L75-L107`, `settings_common.cpp:L1-L882`)**:
   - Highlights targets with 4-phase animation: `Wait`, `FadeIn` (600ms), `Shown` (400ms), and `FadeOut` (600ms) with `opacity = 0.4`.

### Cluster 2: Main Settings Window & Profile Overview
The primary entry point into Telegram Desktop configuration is `Settings::Main` (`settings_main.h:L21-L46`, `settings_main.cpp:L97-L650`):
1. **Profile Cover Card (`settings_main.cpp:L104-L350`)**:
   - Houses user avatar (`UserpicButton`), full name with verified / premium badges (`Profile::BadgeContentForPeer`), phone number (`Profile::PhoneValue`), and username (`@username`).
   - Click handlers:
     - Avatar / Name -> Navigate to Profile Information (`InformationId()`).
     - Phone / Username -> Copy to system clipboard (`QGuiApplication::clipboard()`) with toast notification.
     - QR code share button (`DefaultShowFillPeerQrBoxCallback`).
2. **Settings Section Catalog**:
   - Categorized into logical navigation blocks:
     - My Account (`InformationId()`)
     - Chat Settings (`ChatId()`)
     - Notifications and Sounds (`NotificationsId()`)
     - Privacy and Security (`PrivacySecurityId()`)
     - Data and Storage (`LocalStorageId()`)
     - Power Saving (`PowerSavingBox`)
     - Chat Folders (`FoldersId()`)
     - Devices / Active Sessions (`ActiveSessionsId()`)
     - Language (`LanguageBox`)
     - Calls (`CallsId()`)
     - Telegram Stars / Credits (`CreditsId()`)
     - Telegram Premium (`PremiumId()`)
     - Advanced (`AdvancedId()`)
     - Shortcuts (`ShortcutsId()`)
3. **Interface Scaling & Help (`settings_main.cpp:L476-L650`)**:
   - `HasInterfaceScale()`: Detects display support for UI scaling factor adjustment (100% to 300% in 25% increments).
   - `OpenFaq()`: Opens `https://telegram.org/faq` or localized cloud FAQ.
   - `OpenAskQuestionConfirm()`: Prompts confirmation modal before initiating Volunteer Support chat.

### Cluster 3: Profile Information & Multi-Account Management
User account identity and profile customization are managed in `Settings::Information` (`settings_information.h:L29-L62`, `settings_information.cpp:L78-L950`):
1. **Profile Attributes Editing**:
   - First Name & Last Name (with real-time validation).
   - Bio Editor: 70-character limit (standard) or 140-character limit (Premium), with `kSaveBioTimeout = 1000ms` debounce timer before cloud sync.
   - Username Editor: `@username` availability checker via MTProto `account.checkUsername`.
   - Peer Color Selection: Personal profile background and icon accent customization (`edit_peer_color_box`).
2. **Multi-Account Switching (`settings_information.h:L37-L40`, `settings_information.cpp:L350-L550`)**:
   - Displays all authenticated user accounts (up to 3 for standard users, up to 6 for Telegram Premium users).
   - Unread badges: Computes aggregate unread message count and mute status per account (`Badge::CreateUnread`).
   - Account switcher triggers session switch without application restart.
3. **Avatar Photo Update (`settings_information.h:L41`, `settings_information.cpp:L560-L720`)**:
   - Local photo preview and crop tool (`UpdatePhotoLocally`).
   - Supports static image upload, video avatar loop, and animated emoji status.

### Cluster 4: Telegram Stars & Credits Economics
The virtual currency and creator economy hub is implemented across `Settings::Credits` (`settings_credits.h:L29-L43`, `settings_credits.cpp:L75-L1172`) and `Settings::CreditsGraphics` (`settings_credits_graphics.h:L63-L345`, `settings_credits_graphics.cpp:L1-L3928`):
1. **Stars Balance & Floating Graphics**:
   - Real-time balance polling via `api/api_credits.h` and formatted via `Lang::FormatCreditsAmountDecimal`.
   - Floating glowing 3D star particles (40 particles, sinusoidal float path, periodic burst animations).
2. **Stars Top-Up & Payment Options (`FillCreditOptions`, `BuyStarsHandler`)**:
   - In-app topup packages (e.g. 50, 100, 250, 500, 1000, 2500 Stars).
   - Localized fiat currency pricing and payment provider routing (Stripe / Fragment / App Store).
3. **Withdrawals & Bot/Channel Monetization (`AddWithdrawalWidget`)**:
   - Calculates available balance, locked balance, and minimum withdrawal threshold.
   - Connects to TON wallet via Fragment for Star rewards conversion.
4. **Small Balance Prompts (`SmallBalanceSource`, `SmallBalanceResult`)**:
   - Handles low balance intercepts for 10 distinct actions: `Bot`, `Reaction`, `VideoStream`, `Subscription`, `DeepLink`, `StarGift`, `Message`, `Suggest`, `Offer`, and `Search`.


### Cluster 5: Telegram Premium Subscriptions & Feature Showcase
Telegram Premium management is concentrated in `Settings::Premium` (`settings_premium.h:L21-L113`, `settings_premium.cpp:L50-L1250`):
1. **Tier Options & Discount Matrix**:
   - Monthly subscription, 6-month tier, and annual (12-month) subscription with discount percentages.
   - Dynamic currency conversion based on user location and payment method (In-App Purchase vs Fragment).
2. **Feature Showcase Catalog**:
   - Interactive feature cards: 4 GB upload limit, accelerated download speeds, voice-to-text audio transcription, premium animated stickers & emoji statuses, advanced chat management, real-time translation, animated profile pictures, and exclusive app icons.
3. **Gift Premium Flow (`ShowGiftPremium`, `StartPremiumPayment`)**:
   - Contact recipient selector and custom gift card customization.

### Cluster 6: Keyboard Navigation, Shortcuts & Secret Debug Codes
Keyboard-driven operation and developer tooling:
1. **Directional Key Navigation (`settings_key_navigation.h:L19-L60`, `settings_key_navigation.cpp:L20-L407`)**:
   - Builds a 2D spatial layout tree of interactive entries (`KeyNavigation::Entry`: buttons, sliders, checkboxes).
   - Arrow keys (Up, Down, Left, Right) compute Euclidean distance to neighbor centers: `dist = sqrt(dx^2 + dy^2)`, auto-focusing the nearest element.
   - Enter/Space triggers `activate()`; Escape exits subsection.
2. **Shortcuts Editor (`settings_shortcuts.h:L20-L25`, `settings_shortcuts.cpp:L30-L868`)**:
   - Custom key binding registry categorized into Chatting, Navigation, Media Playback, and Window controls.
   - Multi-account switching hotkeys (Ctrl+1..6 / Cmd+1..6) and folder navigation hotkeys (Ctrl+Shift+1..9).
3. **Secret Debug Codes (`settings_codes.h:L10-L18`, `settings_codes.cpp:L40-L327`)**:
   - Processed via real-time search input stream (`CodesFeedString`):
     - `debugmode`: Toggles verbose network & event logging.
     - `viewlogs`: Opens `log.txt` directory.
     - `testupdate`: Forces check against update canary channel.
     - `testmode`: Switches between MTProto Production and Test DCs (DC 1-5 vs Test DC 1-3).
     - `loadcolors`: Opens custom theme palette file picker (`.tdesktop-palette`).
     - `endpoints`: Overrides connection endpoints from `.tdesktop-endpoints`.

### Cluster 7: Voice/Video Calls, Passcode Lock, Passkeys, Websites & Search
1. **Call Media Devices (`settings_calls.h:L31-L65`, `settings_calls.cpp:L20-L916`)**:
   - Microphone input level monitor: Samples audio volume every `kMicTestUpdateInterval = 100ms`, rendering green-to-red audio bar with `kMicTestAnimationDuration = 200ms`.
   - Speaker test chime playback and output device enumeration.
   - Webcam preview surface (`Webrtc::VideoTrack`).
2. **Local Passcode Security (`settings_local_passcode.h:L12-L18`, `settings_local_passcode.cpp:L50-L1714`)**:
   - Passcode setup, validation, change, and reset flows.
   - `PasscodeCountdown`: 60-second inactivity countdown timer (`kPasscodeSectionTimeout = 60000ms`, `FormatCountdown(%1:%2)`).
   - Configurable auto-lock timeouts (Never, 1 minute, 5 minutes, 1 hour, 5 hours).
3. **Passkeys & Authorized Websites (`settings_passkeys.*`, `settings_websites.*`)**:
   - FIDO2 / WebAuthn passkey registration using OS platform authenticator.
   - Authorized Telegram Login sessions list with domain, IP, and disconnect actions (`kShortPollTimeout = 60s`).
4. **Real-time In-Settings Search (`settings_search.h:L28-L93`, `settings_search.cpp:L25-L717`)**:
   - Tokenizes settings tree by title, subtitle, keywords, and deep links (`SearchWords`, `MatchesWords`).
   - Prefix matching bonus (2x) and first-letter hash indexing for zero-latency filter queries.
   - Integrates cloud FAQ suggestions (`FaqSuggestions`).

---

## Target Rust & GPUI Architecture & Layout

```
crates/gpui_telegram_desktop/src/views/settings/
├── detailed_settings_button.rs        # DetailedSettingsButton GPUI component
├── settings_builder.rs                # SectionBuilder fluent DSL & SearchRegistry
├── settings_codes.rs                  # Secret developer debug codes & triggers
├── settings_common.rs                 # Shared settings models, icons & highlight
├── settings_common_session.rs         # AbstractSection trait & SectionFactory
├── settings_credits_graphics.rs       # 3D stars particles & balance graphics
├── settings_faq_suggestions.rs        # Cloud FAQ query suggestions
├── settings_intro.rs                  # Settings onboarding layer widget
├── settings_key_navigation.rs         # 2D directional arrow key navigation
├── settings_notifications_common.rs   # Notification badges & corner positions
├── settings_search.rs                 # Settings search index & view
├── settings_type.rs                   # Section Type identifier enum
└── sections/
    ├── settings_calls.rs              # Microphone, speaker, camera device view
    ├── settings_credits.rs            # Telegram Stars balance, topup, withdrawal
    ├── settings_information.rs        # Profile edit, bio, username, multi-account
    ├── settings_local_passcode.rs     # Passcode lock & 60s countdown timer
    ├── settings_main.rs               # Main settings overview & profile cover card
    ├── settings_passkeys.rs           # FIDO2 / WebAuthn passkeys list & enrollment
    ├── settings_premium.rs            # Premium subscription tiers & features
    ├── settings_shortcuts.rs          # Keyboard shortcuts management
    └── settings_websites.rs           # Active website authorizations list
```

All 21 targets eradicate dummy stubs, state-saturating boilerplate patterns (A, B, C, D), and implement genuine GPUI elements, interactive handlers, realistic mock and production states, and proper layout tokens.
