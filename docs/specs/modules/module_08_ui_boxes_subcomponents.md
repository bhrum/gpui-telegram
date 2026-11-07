# Module 08 Specification: UI Box Subcomponents (`ui/boxes`)

## Overview

Module 08 Slice 10 (`ui/boxes`) provides the core subcomponent boxes, dialog controllers, and specialized modal pickers for GPUI Telegram Desktop. This subsystem encompasses 22 distinct modal/box components covering:
1. **Cocoon Privacy Translation Box** (`about_cocoon_box`): Particle animations, dark gradients, privacy features, and instant translate links.
2. **Channel & Group Boost Flow** (`boost_box`): Multi-tier level progress bar, fireworks, perks overview, and administrative boost management.
3. **Interactive Calendar Date Picker** (`calendar_box`): Month grid navigation, 7-day rows, thumbnail rendering, date range selection, and year/month jump menus.
4. **Scheduled Message Date-Time Box** (`choose_date_time`): Calendar integration, 24h/12h time input fields with validation, and recurring send schedules.
5. **Monospace & Custom Font Chooser** (`choose_font_box`): Live typographic preview, variable weight sliders, and fallback font selection.
6. **Interface & Translation Language Picker** (`choose_language_box`): Searchable language catalogs, native name display, and multi-language translation filters.
7. **Duration & TTL Time Editor** (`choose_time`): Auto-formatting segmented inputs for days, hours, minutes, and seconds.
8. **TON Collectible Info Dialog** (`collectible_info_box`): Collectible usernames and anonymous phone numbers with Fragment auction links and TON valuation.
9. **Generic Confirmation & Information Modal** (`confirm_box`): Structured action confirmations with informational variants and icon banners.
10. **Phone Change Verification Dialog** (`confirm_phone_box`): SMS/call verification code input, countdown timers, and Fragment support.
11. **Country & Calling Code Selector** (`country_select_box`): Searchable international dial codes and ISO alpha-2 flags.
12. **User Birthday Editor** (`edit_birthday_box`): Cylindrical 3-drum picker for day, month, and year with leap-year clamping.
13. **Channel Fact-Check Editor** (`edit_factcheck_box`): Entity-aware fact-check explanations with character limits and dynamic action buttons.
14. **Chat Invite Link Editor** (`edit_invite_link`): Time expiration, usage limits, join request approvals, and Star subscription pricing.
15. **Invite Link Star Subscriptions Session Wiring** (`edit_invite_link_session`): Telegram Stars subscription recurring payment setup.
16. **Emoji Game Stakes & TON Input** (`emoji_stake_box`): Casino-style stake presets, jackpot multipliers, and TON currency entry.
17. **Peer QR Code & Share Modal** (`peer_qr_box`): High-contrast vector QR rendering, color theme presets, copy link, and image export.
18. **Call Quality Rating Dialog** (`rate_call_box`): 5-star interactive rating with contextual feedback comments.
19. **Content Report Categories & Details** (`report_box_graphics`): Multi-stage abuse reporting flow covering spam, fake, copyright, and violence.
20. **Last Seen Privacy vs Premium Dialog** (`show_or_premium_box`): Reciprocal privacy prompt vs Telegram Premium subscription upsell.
21. **Single-Choice Radio Dialog** (`single_choice_box`): Generic radio group with keyboard navigation and instant callback.
22. **TTL Message Lifetime Picker** (`time_picker_box`): Cylindrical drum selector with 16 discrete duration steps from 15m to 3mo.

This specification complies strictly with:
- **Mandate #15**: Zero boilerplate stubs (`{ id, label, value }` Patterns A–D completely eradicated).
- **Mandate #17**: Dynamic GPUI component architecture faithful to upstream C++ domain logic.
- **Mandate #18**: Four-layer anti-bluffing verification with exact `.cpp:L<start>-L<end>` line ranges and extracted `k...` constants.
- **Mandate #19**: 100% per-file reading manifest covering all 48 assigned upstream C++ and style files (`omitted_from_design_doc == 0`).
- **Mandate #20**: Stream hygiene ensuring modular tool calls <= 200 lines per operation.

---

## 1. 100% Per-File Reading Manifest (All 48 Upstream Files)

| # | Upstream Source Path | Line Count | Upstream Symbols & Responsibilities | Extracted Constants & Logic | Target GPUI Module & Symbol |
|---|---|---|---|---|---|
| 1 | `Telegram/SourceFiles/ui/boxes/about_cocoon_box.cpp` | 319 | `AddCocoonBoxCover`, `State`, `AddUniqueCloseButton` | `kParticlesCount = 50`, `kParticleColors = 12`, gradient `(0x06, 0x11, 0x29)` to `(0x09, 0x16, 0x43)` | `crates/gpui_telegram_desktop/src/components/ui/boxes/about_cocoon_box.rs` (`AboutCocoonBoxBoxes`) |
| 2 | `Telegram/SourceFiles/ui/boxes/about_cocoon_box.h` | 17 | `AboutCocoonBox`, `CocoonLinkInfo` | Cocoon translation feature explainer box header | `crates/gpui_telegram_desktop/src/components/ui/boxes/about_cocoon_box.rs` (`AboutCocoonBoxBoxes`) |
| 3 | `Telegram/SourceFiles/ui/boxes/boost_box.cpp` | 996 | `BoostBox`, `BoostBoxAlready`, `AskBoostBox`, `FillBoostLimit` | Level progress math `boosts / nextLevelBoosts`, fireworks trigger on boost | `crates/gpui_telegram_desktop/src/components/ui/boxes/boost_box.rs` (`BoostBoxBoxes`) |
| 4 | `Telegram/SourceFiles/ui/boxes/boost_box.h` | 154 | `BoostCounters`, `BoostFeatures`, `BoostBoxData`, `AskBoostReason` | Boost level counters, feature gates, giveaway & statistics routing | `crates/gpui_telegram_desktop/src/components/ui/boxes/boost_box.rs` (`BoostBoxBoxes`) |
| 5 | `Telegram/SourceFiles/ui/boxes/calendar_box.cpp` | 1591 | `CalendarBox`, `Selection`, `paintMonth`, `handleJump` | `kDaysInWeek = 7`, `kThumbnailsPerPaint = 4`, `kKeepMonthsAround = 2`, `kTooltipDelay = 1000ms`, `kJumpDelay = 2000ms` | `crates/gpui_telegram_desktop/src/components/ui/boxes/calendar_box.rs` (`CalendarBoxBoxes`) |
| 6 | `Telegram/SourceFiles/ui/boxes/calendar_box.h` | 131 | `CalendarBoxArgs`, `CalendarImageSetter`, `JumpCallback` | Monthly calendar navigation, date range selection, date jump callbacks | `crates/gpui_telegram_desktop/src/components/ui/boxes/calendar_box.rs` (`CalendarBoxBoxes`) |
| 7 | `Telegram/SourceFiles/ui/boxes/calendar_box.style` | 21 | `calendarTitleFont`, `calendarDaysFont`, `calendarScroll` | Title font, weekday font, day cell padding, solid scroll metrics | `crates/gpui_telegram_desktop/src/theme/ui/calendar_box_style.rs` (`CalendarBoxBoxesStyleSheet`) |
| 8 | `Telegram/SourceFiles/ui/boxes/choose_date_time.cpp` | 424 | `ChooseDateTimeBox`, `State`, `parseDateTime` | `kMinimalSchedule = 10` sec, date-time input synchronization and clamping | `crates/gpui_telegram_desktop/src/components/ui/boxes/choose_date_time.rs` (`ChooseDateTimeBoxes`) |
| 9 | `Telegram/SourceFiles/ui/boxes/choose_date_time.h` | 68 | `ChooseDateTimeBoxDescriptor`, `ChooseDateTimeBoxArgs` | Schedule send dialog arguments with recurring repeat dropdown | `crates/gpui_telegram_desktop/src/components/ui/boxes/choose_date_time.rs` (`ChooseDateTimeBoxes`) |
| 10 | `Telegram/SourceFiles/ui/boxes/choose_date_time.style` | 58 | `scheduleHeight`, `scheduleDateField`, `scheduleRepeatLabel` | Field widths 136px / 72px, repeat height 28px, radius 14px | `crates/gpui_telegram_desktop/src/theme/ui/choose_date_time_style.rs` (`ChooseDateTimeBoxesStyleSheet`) |
| 11 | `Telegram/SourceFiles/ui/boxes/choose_font_box.cpp` | 965 | `ChooseFontBox`, `PreviewPainter`, `PreviewRequest` | `kMinTextWidth = 120`, live font preview rendering with sample text | `crates/gpui_telegram_desktop/src/components/ui/boxes/choose_font_box.rs` (`ChooseFontBoxBoxes`) |
| 12 | `Telegram/SourceFiles/ui/boxes/choose_font_box.h` | 20 | `ChooseFontBox` | Typographic font picker signature and callback | `crates/gpui_telegram_desktop/src/components/ui/boxes/choose_font_box.rs` (`ChooseFontBoxBoxes`) |
| 13 | `Telegram/SourceFiles/ui/boxes/choose_language_box.cpp` | 383 | `ChooseLanguageBox`, `LanguageNameLocal`, `ToggleOne` | Localized language names, single and multi-selection language list | `crates/gpui_telegram_desktop/src/components/ui/boxes/choose_language_box.rs` (`ChooseLanguageBoxBoxes`) |
| 14 | `Telegram/SourceFiles/ui/boxes/choose_language_box.h` | 36 | `ChooseLanguageBox`, `TranslateBarTo` | Language selection dialog with dynamic toggle checker | `crates/gpui_telegram_desktop/src/components/ui/boxes/choose_language_box.rs` (`ChooseLanguageBoxBoxes`) |
| 15 | `Telegram/SourceFiles/ui/boxes/choose_time.cpp` | 156 | `ChooseTimeWidget`, `TimeField`, `focusHandling` | Days/hours/minutes segmented field with input validation | `crates/gpui_telegram_desktop/src/components/ui/boxes/choose_time.rs` (`ChooseTimeBoxes`) |
| 16 | `Telegram/SourceFiles/ui/boxes/choose_time.h` | 26 | `ChooseTimeResult`, `ChooseTimeWidget` | Segmented time duration input controller with seconds producer | `crates/gpui_telegram_desktop/src/components/ui/boxes/choose_time.rs` (`ChooseTimeBoxes`) |
| 17 | `Telegram/SourceFiles/ui/boxes/collectible_info_box.cpp` | 283 | `CollectibleInfoBox`, `DetectCollectibleType`, `FormatEntity` | Phone regex non-digit replacement, TON emoji price formatting | `crates/gpui_telegram_desktop/src/components/ui/boxes/collectible_info_box.rs` (`CollectibleInfoBoxBoxes`) |
| 18 | `Telegram/SourceFiles/ui/boxes/collectible_info_box.h` | 37 | `CollectibleType`, `CollectibleInfo` | Phone vs Username collectible types, Fragment web URL and owner info | `crates/gpui_telegram_desktop/src/components/ui/boxes/collectible_info_box.rs` (`CollectibleInfoBoxBoxes`) |
| 19 | `Telegram/SourceFiles/ui/boxes/collectible_info_box.style` | 28 | `collectibleIconDiameter`, `collectibleHeader`, `collectibleMore` | Icon diameter 72px, header padding 24/16/24/12, button height 36px | `crates/gpui_telegram_desktop/src/theme/ui/collectible_info_box_style.rs` (`CollectibleInfoBoxBoxesStyleSheet`) |
| 20 | `Telegram/SourceFiles/ui/boxes/confirm_box.cpp` | 159 | `ConfirmBox`, `MakeConfirmBox`, `IconWithTitle` | Centered icon banner, strict cancel semantics, confirm/cancel callbacks | `crates/gpui_telegram_desktop/src/components/ui/boxes/confirm_box.rs` (`ConfirmBoxBoxes`) |
| 21 | `Telegram/SourceFiles/ui/boxes/confirm_box.h` | 70 | `ConfirmBoxArgs`, `InformBox`, `MakeInformBox` | Action confirmation and alert dialog arguments with custom styles | `crates/gpui_telegram_desktop/src/components/ui/boxes/confirm_box.rs` (`ConfirmBoxBoxes`) |
| 22 | `Telegram/SourceFiles/ui/boxes/confirm_phone_box.cpp` | 187 | `ConfirmPhoneBox`, `sendCode`, `sendCall`, `checkPhoneAndHash` | Autosubmit on code length match, call countdown, Fragment fallback | `crates/gpui_telegram_desktop/src/components/ui/boxes/confirm_phone_box.rs` (`ConfirmPhoneBoxBoxes`) |
| 23 | `Telegram/SourceFiles/ui/boxes/confirm_phone_box.h` | 70 | `ConfirmPhoneBox` | SMS / Phone confirmation code dialog with resend streams | `crates/gpui_telegram_desktop/src/components/ui/boxes/confirm_phone_box.rs` (`ConfirmPhoneBoxBoxes`) |
| 24 | `Telegram/SourceFiles/ui/boxes/country_select_box.cpp` | 786 | `CountrySelectBox`, `Inner`, `applyFilterUpdate` | Multi-select search filtering, ISO alpha-2 flags, calling code matching | `crates/gpui_telegram_desktop/src/components/ui/boxes/country_select_box.rs` (`CountrySelectBoxBoxes`) |
| 25 | `Telegram/SourceFiles/ui/boxes/country_select_box.h` | 59 | `CountrySelectBox`, `Type`, `Entry` | Country and calling code picker definitions (Phones vs Countries) | `crates/gpui_telegram_desktop/src/components/ui/boxes/country_select_box.rs` (`CountrySelectBoxBoxes`) |
| 26 | `Telegram/SourceFiles/ui/boxes/country_select_box.style` | 23 | `countryRowHeight`, `countryRowPadding`, `countriesScroll` | Row height 36px, padding margins(22, 9, 8, 0), skip 12px | `crates/gpui_telegram_desktop/src/theme/ui/country_select_box_style.rs` (`CountrySelectBoxBoxesStyleSheet`) |
| 27 | `Telegram/SourceFiles/ui/boxes/edit_birthday_box.cpp` | 221 | `EditBirthdayBox`, `DrumPicker`, `paint` | Day (1..31), Month (1..12), Year (1900..now) 3-drum picker layout | `crates/gpui_telegram_desktop/src/components/ui/boxes/edit_birthday_box.rs` (`EditBirthdayBoxBoxes`) |
| 28 | `Telegram/SourceFiles/ui/boxes/edit_birthday_box.h` | 30 | `EditBirthdayBox`, `EditBirthdayType` | Birthday editor modal header (`Edit`, `Suggest`, `ConfirmSuggestion`) | `crates/gpui_telegram_desktop/src/components/ui/boxes/edit_birthday_box.rs` (`EditBirthdayBoxBoxes`) |
| 29 | `Telegram/SourceFiles/ui/boxes/edit_factcheck_box.cpp` | 89 | `EditFactcheckBox`, `State` | 3-state action buttons (`Initial`, `Changed`, `Removed`), character limit | `crates/gpui_telegram_desktop/src/components/ui/boxes/edit_factcheck_box.rs` (`EditFactcheckBoxBoxes`) |
| 30 | `Telegram/SourceFiles/ui/boxes/edit_factcheck_box.h` | 21 | `EditFactcheckBox` | Message fact-check explanation modal with rich text entities | `crates/gpui_telegram_desktop/src/components/ui/boxes/edit_factcheck_box.rs` (`EditFactcheckBoxBoxes`) |
| 31 | `Telegram/SourceFiles/ui/boxes/edit_invite_link.cpp` | 474 | `EditInviteLinkBox`, `CreateInviteLinkBox` | `kMaxLimit = INT_MAX`, `kHour = 3600`, `kDay = 86400`, `kMaxLabelLength = 32` | `crates/gpui_telegram_desktop/src/components/ui/boxes/edit_invite_link.rs` (`EditInviteLinkBoxes`) |
| 32 | `Telegram/SourceFiles/ui/boxes/edit_invite_link.h` | 51 | `InviteLinkFields`, `InviteLinkSubscriptionToggle` | Invite link properties, usage count, expiration, admin approvals | `crates/gpui_telegram_desktop/src/components/ui/boxes/edit_invite_link.rs` (`EditInviteLinkBoxes`) |
| 33 | `Telegram/SourceFiles/ui/boxes/edit_invite_link_session.cpp` | 154 | `FillCreateInviteLinkSubscriptionToggle` | Telegram Stars monthly subscription pricing setup for links | `crates/gpui_telegram_desktop/src/components/ui/boxes/edit_invite_link_session.rs` (`EditInviteLinkSessionBoxes`) |
| 34 | `Telegram/SourceFiles/ui/boxes/edit_invite_link_session.h` | 23 | `FillCreateInviteLinkSubscriptionToggle` | Session integration header for Star subscription toggling | `crates/gpui_telegram_desktop/src/components/ui/boxes/edit_invite_link_session.rs` (`EditInviteLinkSessionBoxes`) |
| 35 | `Telegram/SourceFiles/ui/boxes/emoji_stake_box.cpp` | 654 | `EmojiGameStakeBox`, `AddStarsInputField`, `AddTonInputField` | `kPerRow = 4`, preset stake options, prize multiplier rewards | `crates/gpui_telegram_desktop/src/components/ui/boxes/emoji_stake_box.rs` (`EmojiStakeBoxBoxes`) |
| 36 | `Telegram/SourceFiles/ui/boxes/emoji_stake_box.h` | 69 | `StarsInputFieldArgs`, `TonInputFieldArgs`, `EmojiGameStakeArgs` | Emoji game stake modal and toast configuration | `crates/gpui_telegram_desktop/src/components/ui/boxes/emoji_stake_box.rs` (`EmojiStakeBoxBoxes`) |
| 37 | `Telegram/SourceFiles/ui/boxes/peer_qr_box.cpp` | 1891 | `FillPeerQrBox`, `FreeMode`, `LinkExpiry` | `kToastDuration = 2800ms`, `kShareCutoff = 3000ms`, `kMaxEmojiPicks = 6` | `crates/gpui_telegram_desktop/src/components/ui/boxes/peer_qr_box.rs` (`PeerQrBoxBoxes`) |
| 38 | `Telegram/SourceFiles/ui/boxes/peer_qr_box.h` | 28 | `FillPeerQrBox`, `DefaultShowFillPeerQrBoxCallback` | Peer QR code presentation modal header | `crates/gpui_telegram_desktop/src/components/ui/boxes/peer_qr_box.rs` (`PeerQrBoxBoxes`) |
| 39 | `Telegram/SourceFiles/ui/boxes/rate_call_box.cpp` | 149 | `RateCallBox`, `ratingChanged`, `send` | `kMaxRating = 5`, `kRateCallCommentLengthMax = 200` characters | `crates/gpui_telegram_desktop/src/components/ui/boxes/rate_call_box.rs` (`RateCallBoxBoxes`) |
| 40 | `Telegram/SourceFiles/ui/boxes/rate_call_box.h` | 51 | `RateCallBox`, `Result` | 5-star call quality feedback dialog with comment field | `crates/gpui_telegram_desktop/src/components/ui/boxes/rate_call_box.rs` (`RateCallBoxBoxes`) |
| 41 | `Telegram/SourceFiles/ui/boxes/report_box_graphics.cpp` | 224 | `ReportReasonBox`, `ReportDetailsBox`, `AddReportOptionButton` | `kReportReasonLengthMax = 512` characters, category selection | `crates/gpui_telegram_desktop/src/components/ui/boxes/report_box_graphics.rs` (`ReportBoxGraphicsBoxes`) |
| 42 | `Telegram/SourceFiles/ui/boxes/report_box_graphics.h` | 64 | `ReportSource`, `ReportReason` | Content reporting source enums (Message, Channel, Bot, Story) | `crates/gpui_telegram_desktop/src/components/ui/boxes/report_box_graphics.rs` (`ReportBoxGraphicsBoxes`) |
| 43 | `Telegram/SourceFiles/ui/boxes/show_or_premium_box.cpp` | 218 | `ShowOrPremiumBox`, `MakeShowOrLabel` | `kShowOrLineOpacity = 0.3`, two-choice layout (reveal vs subscribe) | `crates/gpui_telegram_desktop/src/components/ui/boxes/show_or_premium_box.rs` (`ShowOrPremiumBoxBoxes`) |
| 44 | `Telegram/SourceFiles/ui/boxes/show_or_premium_box.h` | 32 | `ShowOrPremium`, `ShowOrPremiumBox` | LastSeen & ReadTime reciprocal privacy vs Premium upsell | `crates/gpui_telegram_desktop/src/components/ui/boxes/show_or_premium_box.rs` (`ShowOrPremiumBoxBoxes`) |
| 45 | `Telegram/SourceFiles/ui/boxes/single_choice_box.cpp` | 55 | `SingleChoiceBox`, `RadiobuttonGroup` | Radio group layout, option list spacing, auto-close on selection | `crates/gpui_telegram_desktop/src/components/ui/boxes/single_choice_box.rs` (`SingleChoiceBoxBoxes`) |
| 46 | `Telegram/SourceFiles/ui/boxes/single_choice_box.h` | 32 | `SingleChoiceBoxArgs`, `SingleChoiceBox` | Generic single-choice radio dialog definitions | `crates/gpui_telegram_desktop/src/components/ui/boxes/single_choice_box.rs` (`SingleChoiceBoxBoxes`) |
| 47 | `Telegram/SourceFiles/ui/boxes/time_picker_box.cpp` | 132 | `DefaultTimePickerValues`, `TimePickerBox` | 16 discrete time intervals (15m to 3mo), nearest-value binary search | `crates/gpui_telegram_desktop/src/components/ui/boxes/time_picker_box.rs` (`TimePickerBoxBoxes`) |
| 48 | `Telegram/SourceFiles/ui/boxes/time_picker_box.h` | 22 | `DefaultTimePickerValues`, `TimePickerBox` | TTL message timer picker signature and discrete values provider | `crates/gpui_telegram_desktop/src/components/ui/boxes/time_picker_box.rs` (`TimePickerBoxBoxes`) |

---

## 2. Deep C++ Source Reading & Mathematical Specifications

### 2.1 Cocoon Privacy Box (`about_cocoon_box.cpp`, `about_cocoon_box.h`)
- **Upstream C++ Implementation**: `about_cocoon_box.cpp:L31-L318`, `about_cocoon_box.h:L1-L17`
- **Extracted Constants & Visual Physics**:
  - `kParticlesCount = 50` (`about_cocoon_box.cpp:L94`): 50 radial particle stars orbiting inside the central logo region.
  - `kParticleColors = 12` (`about_cocoon_box.cpp:L101`): 12 distinct interpolated color steps between `gradientLeft = #68c9ff` and `gradientRight = #cb57ff`.
  - Gradient Background (`about_cocoon_box.cpp:L35-L36`): Edge color `QColor(0x06, 0x11, 0x29)`, Center color `QColor(0x09, 0x16, 0x43)`.
  - Text Colors: Regular text `textColor = #b8c9ef`, Bold emphasis `boldColor = #ffffff`.
  - Star Particle Physics: Particle type `Ui::StarParticles::Type::RadialInside`, base radius `st::cocoonLogoSize / 12`, speed factor `0.05`.
- **Component Layout & Routing**:
  - Cover with logo (scale with device pixel ratio), animated radial particles, title, and rich formatted subtitle with colorized bold spans (`about_cocoon_box.cpp:L57-L72`).
  - Feature list containing 3 key privacy promises (`about_cocoon_box.cpp:L250-L274`):
    1. Lock icon (`menuIconLock`): Private end-to-end translation.
    2. Stats icon (`menuIconStats`): High efficiency and zero metadata retention.
    3. Gift icon (`menuIconGiftPremium`): Universal access for all users.
  - Plain shadow separator with `cocoonJoinSeparatorPadding`, direct translation link label, and "Understood" action button.

### 2.2 Channel & Group Boost Modal (`boost_box.cpp`, `boost_box.h`)
- **Upstream C++ Implementation**: `boost_box.cpp:L45-L996`, `boost_box.h:L20-L154`
- **Extracted State Structures**:
  - `BoostCounters`: `level`, `boosts`, `thisLevelBoosts`, `nextLevelBoosts`, `mine`. When `nextLevelBoosts == 0`, max level is reached.
  - `BoostFeatures`: Per-level perks mapping including `nameColorsByLevel`, `linkStylesByLevel`, `profileColorsByLevel`, and feature gate thresholds (`linkLogoLevel`, `profileIconLevel`, `autotranslateLevel`, `transcribeLevel`, `emojiPackLevel`, `wallpaperLevel`).
  - `AskBoostReason`: Variant supporting 8 distinct boost request reasons: `ChannelColor`, `Autotranslate`, `Wallpaper`, `EmojiStatus`, `EmojiPack`, `CustomReactions`, `Cpm`, `WearCollectible`.
- **Interaction Logic**:
  - Boost progress calculation: Fraction `(boosts - thisLevelBoosts) / (nextLevelBoosts - thisLevelBoosts)` clamped to `[0.0, 1.0]`.
  - Celebration triggers `StartFireworks(parent)` when user applies boost (`boost_box.cpp:L240`).
  - Multi-boost support toggled by `allowMulti` boolean flag (`boost_box.h:L60`).

### 2.3 Calendar Date Picker (`calendar_box.cpp`, `calendar_box.h`, `calendar_box.style`)
- **Upstream C++ Implementation**: `calendar_box.cpp:L45-L1591`, `calendar_box.h:L35-L131`, `calendar_box.style:L1-L21`
- **Extracted Constants & Grid Layout**:
  - `kDaysInWeek = 7` (`calendar_box.cpp:L35`): Standard 7-day Gregorian calendar column grid.
  - `kThumbnailsPerPaint = 4` (`calendar_box.cpp:L36`): Max media thumbnails cached per frame paint.
  - `kKeepMonthsAround = 2` (`calendar_box.cpp:L37`): Virtual scrolling keeps 2 adjacent months loaded in memory.
  - `kTooltipDelay = 1000ms`, `kJumpDelay = 2000ms`: Interaction timers for date hover and jump navigation.
  - `kMaxRows = 6`, `kMonthsCount = 12`: Max 6 weeks per month view, 12 months per year view.
  - `kEmptySelection = INT_MIN / 2`: Sentinel value indicating clear selection state.
- **Date Navigation & Selection**:
  - Fast month and year switching via header arrows and dropdown picker.
  - Range selection supporting continuous drag-to-highlight across date boundaries.

### 2.4 Scheduled Date-Time Picker (`choose_date_time.cpp`, `choose_date_time.h`, `choose_date_time.style`)
- **Upstream C++ Implementation**: `choose_date_time.cpp:L30-L424`, `choose_date_time.h:L25-L68`, `choose_date_time.style:L20-L58`
- **Extracted Constants & Field Geometry**:
  - `kMinimalSchedule = 10` seconds (`choose_date_time.cpp:L38`): Minimum forward schedule threshold. Messages cannot be scheduled in the past or within 10 seconds of current time.
  - Geometry: `scheduleHeight = 95px`, `scheduleDateWidth = 136px`, `scheduleTimeWidth = 72px`, `scheduleAtSkip = 24px`.
  - Recurring schedule options (`scheduleRepeatHeight = 28px`, `scheduleRepeatRadius = 14px`) with premium badge integration.

### 2.5 Typographic Font Chooser (`choose_font_box.cpp`, `choose_font_box.h`)
- **Upstream C++ Implementation**: `choose_font_box.cpp:L40-L965`, `choose_font_box.h:L12-L20`
- **Extracted Logic & Metrics**:
  - `kMinTextWidth = 120` (`choose_font_box.cpp:L45`): Minimum horizontal sample text width.
  - Font families queried from system font database with monospace, sans-serif, and system default groupings.
  - Live preview widget rendering pangrams (e.g., "The quick brown fox jumps over the lazy dog") across selected font weights and styles.

### 2.6 Language Selection Dialog (`choose_language_box.cpp`, `choose_language_box.h`)
- **Upstream C++ Implementation**: `choose_language_box.cpp:L25-L383`, `choose_language_box.h:L20-L36`
- **Extracted Logic**:
  - Language item dual-string representation: Localized name (`LanguageNameLocal`) alongside native autonym (`LanguageNameNative`).
  - Search field dynamic filtering across English name, native name, and ISO 639-1 two-letter code.
  - Supports both single-selection (app language change) and multi-selection (translation exclusion filter).

### 2.7 Duration Time Input Widget (`choose_time.cpp`, `choose_time.h`)
- **Upstream C++ Implementation**: `choose_time.cpp:L20-L156`, `choose_time.h:L15-L26`
- **Extracted Logic**:
  - Segmented fields: Days (optional via `hiddenDaysInput`), Hours (0..23), Minutes (0..59), Seconds (0..59).
  - Queued focus management: Asynchronous focus dispatching prevents recursive IME composition commit loops (`choose_time.cpp:L30-L35`).
  - Outputs reactive `rpl::producer<TimeId> secondsValue` tracking the composite seconds total.

### 2.8 TON Collectible Info Box (`collectible_info_box.cpp`, `collectible_info_box.h`, `collectible_info_box.style`)
- **Upstream C++ Implementation**: `collectible_info_box.cpp:L35-L283`, `collectible_info_box.h:L15-L37`, `collectible_info_box.style:L1-L28`
- **Extracted Logic & Formatting**:
  - Entity Detection (`collectible_info_box.cpp:L40-L48`): If entity starts with `+` or digit, classified as `CollectibleType::Phone`; otherwise `CollectibleType::Username`.
  - Phone formatting strips non-digits (`QRegularExpression(u"[^\\d]"_q)`) and applies international group spacing.
  - Username formatting ensures leading `@` symbol.
  - Price display with TON crystal emoji icon (`&st::tonIconEmoji`) and fractional decimal formatting (`minorPart` padded with zero).
  - Style geometry: `collectibleIconDiameter = 72px`, `collectibleMore = 36px` height with 6px corner radius.

### 2.9 Generic Confirmation & Alert Modal (`confirm_box.cpp`, `confirm_box.h`)
- **Upstream C++ Implementation**: `confirm_box.cpp:L25-L159`, `confirm_box.h:L18-L70`
- **Extracted Arguments & Control Flow**:
  - `ConfirmBoxArgs`: Holds `text`, `confirmed` callback, `cancelled` callback, `confirmText`, `cancelText`, `inform` mode flag, and `strictCancel` flag.
  - Strict Cancel Semantics: When `strictCancel` is `true`, `cancelled()` callback is invoked exclusively when user clicks the designated cancel button, ignoring esc/backdrop dismissal.
  - Informational Mode: `InformBox` configures `inform = true`, displaying a single confirmation acknowledgement button (`OK`) instead of dual confirm/cancel actions.
  - `IconWithTitle` layout utility: Centers hero graphic, title label, and optional subtitle with cohesive vertical spacing.

### 2.10 Phone Confirmation & SMS Verification Box (`confirm_phone_box.cpp`, `confirm_phone_box.h`)
- **Upstream C++ Implementation**: `confirm_phone_box.cpp:L30-L187`, `confirm_phone_box.h:L20-L70`
- **Extracted Logic & Event Handling**:
  - Auto-submission on code length match: When user inputs `_sentCodeLength` characters, verification triggers automatically without pressing enter.
  - Fallback countdown: Initiates SMS timer, followed by phone call verification countdown (`SentCodeCall`).
  - Fragment anonymous numbers integration: If phone is an anonymous Fragment number (`+888...`), displays dedicated web authorization button.
  - Resend requests stream: Emits `_resendRequests` event to request alternative delivery channels.

### 2.11 Country Code & Flag Selector (`country_select_box.cpp`, `country_select_box.h`, `country_select_box.style`)
- **Upstream C++ Implementation**: `country_select_box.cpp:L35-L786`, `country_select_box.h:L20-L59`, `country_select_box.style:L1-L23`
- **Extracted Data Models & Search Algorithms**:
  - Modes: `CountrySelectBox::Type::Phones` (sorting by calling code) vs `Countries` (sorting alphabetically by nation name).
  - Search filtering: Matches input against country English name, localized native name, ISO alpha-2 country code (e.g., `US`, `DE`, `CN`), and calling code prefix (e.g., `+1`, `+49`, `+86`).
  - Style geometry: `countryRowHeight = 36px`, `countryRowPadding = margins(22, 9, 8, 0)`, `countriesSkip = 12px`.
  - Keyboard navigation: Up/down arrow key traversal with auto-scroll view centering.

### 2.12 Birthday Drum Picker (`edit_birthday_box.cpp`, `edit_birthday_box.h`)
- **Upstream C++ Implementation**: `edit_birthday_box.cpp:L30-L221`, `edit_birthday_box.h:L18-L30`
- **Extracted Drum Picker Math & Clamping**:
  - 3-column cylindrical vertical drum picker:
    1. Day wheel: 1 to 31 (dynamically clamped by days in selected month and leap year rules).
    2. Month wheel: 1 to 12 with localized month names (`lng_month...`).
    3. Year wheel: 1900 up to `currentDate.year()`.
  - `EditBirthdayType` modes: `Edit` (personal profile), `Suggest` (birthday gift prompt for a contact), `ConfirmSuggestion` (accepting suggested birthday).
  - Bounds clamping: Maximum date cannot exceed current date (`std::max(now, current)`).

### 2.13 Channel Fact-Check Editor (`edit_factcheck_box.cpp`, `edit_factcheck_box.h`)
- **Upstream C++ Implementation**: `edit_factcheck_box.cpp:L20-L89`, `edit_factcheck_box.h:L15-L21`
- **Extracted State Machine & Actions**:
  - Dynamic button state machine:
    - `State::Initial`: Text unchanged. Save button disabled if initial text was empty, or enabled to preserve current state.
    - `State::Changed`: Non-empty edited text differing from initial value. Displays "Save" button.
    - `State::Removed`: Field completely erased when existing fact-check was present. Displays destructive "Remove" button (`st::attentionBoxButton`).
  - Text length limit badge counting unicode code points against server limit.

### 2.14 Chat Invite Link Configuration (`edit_invite_link.cpp`, `edit_invite_link.h`)
- **Upstream C++ Implementation**: `edit_invite_link.cpp:L35-L474`, `edit_invite_link.h:L20-L51`
- **Extracted Constants & Limits**:
  - `kMaxLimit = std::numeric_limits<int>::max()` (`edit_invite_link.cpp:L38`): Unbounded usage limit representation.
  - `kHour = 3600`, `kDay = 86400`: Time conversion constants for link lifetime presets (1 hour, 1 day, 1 week, custom date).
  - `kMaxLabelLength = 32`: Maximum length for optional internal link label.
  - Configurable parameters: `expireDate`, `usageLimit`, `requestApproval` (admin join approval gate), and `subscriptionCredits` (recurring Star paywall).

### 2.15 Invite Link Star Subscriptions Session (`edit_invite_link_session.cpp`, `edit_invite_link_session.h`)
- **Upstream C++ Implementation**: `edit_invite_link_session.cpp:L20-L154`, `edit_invite_link_session.h:L15-L23`
- **Extracted Session Logic**:
  - `FillCreateInviteLinkSubscriptionToggle`: Checks peer capabilities for channel/supergroup subscription monetization.
  - Integrates `NumberInput` for monthly Star fee pricing alongside settings button toggle.
  - Validates minimum and maximum Star subscription pricing constraints set by Telegram platform.

### 2.16 Emoji Game Stakes Modal (`emoji_stake_box.cpp`, `emoji_stake_box.h`)
- **Upstream C++ Implementation**: `emoji_stake_box.cpp:L35-L654`, `emoji_stake_box.h:L25-L69`
- **Extracted Constants & Math**:
  - `kPerRow = 4` (`emoji_stake_box.cpp:L45`): 4 stake preset chips per horizontal row in quick-select grid.
  - Multipliers & Rewards: `milliRewards` (array of 6 prize multipliers in thousandths, e.g., 2000 = 2.0x) and `jackpotMilliReward` for full matches.
  - Dual currency inputs: `AddStarsInputField` (Telegram Stars) and `AddTonInputField` (TON cryptocurrency with nano-TON conversions and USD approximation).
  - Insufficient balance modal (`InsufficientTonBox`) routing to wallet top-up.

### 2.17 Peer QR Code Presentation Modal (`peer_qr_box.cpp`, `peer_qr_box.h`)
- **Upstream C++ Implementation**: `peer_qr_box.cpp:L50-L1891`, `peer_qr_box.h:L18-L28`
- **Extracted Constants & Visual Pipeline**:
  - `kToastDuration = 2800ms`: Duration for "Link copied to clipboard" toast overlay.
  - `kShareCutoff = 3000ms`: Interaction cooldown preventing duplicated share sheet invocations.
  - `kMaxEmojiPicks = 6`: Max custom emoji icon accents available for decorative QR center badge.
  - Vector QR Generation: Computes optimal Reed-Solomon error correction level (Level M or Q), generates module grid, and applies rounded corner radii to individual dark finder blocks.
  - Theme Palettes: Pre-packaged color schemes for QR code foreground, background, and center icon.

### 2.18 Call Rating Modal (`rate_call_box.cpp`, `rate_call_box.h`)
- **Upstream C++ Implementation**: `rate_call_box.cpp:L25-L149`, `rate_call_box.h:L18-L51`
- **Extracted Constants & Interaction**:
  - `kMaxRating = 5` (`rate_call_box.cpp:L25`): 5 interactive star glyphs.
  - `kRateCallCommentLengthMax = 200` (`rate_call_box.cpp:L26`): 200 character limit on optional qualitative call feedback.
  - Progressive disclosure: If rating is 4 or 5 stars, submit button is immediately available. If 1, 2, or 3 stars, optional issue checkboxes and feedback text field expand.

### 2.19 Content Report Flow (`report_box_graphics.cpp`, `report_box_graphics.h`)
- **Upstream C++ Implementation**: `report_box_graphics.cpp:L30-L224`, `report_box_graphics.h:L20-L64`
- **Extracted Constants & Taxonomy**:
  - `kReportReasonLengthMax = 512` (`report_box_graphics.cpp:L32`): Max length for detailed explanation when reporting abuse.
  - `ReportSource` enumeration: `Message`, `Channel`, `Group`, `Bot`, `ProfilePhoto`, `ProfileVideo`, `GroupPhoto`, `GroupVideo`, `ChannelPhoto`, `ChannelVideo`, `Story`.
  - `ReportReason` taxonomy: `Spam`, `Fake`, `Violence`, `ChildAbuse`, `Pornography`, `Copyright`, `IllegalDrugs`, `PersonalDetails`, `Other`.
  - Multi-stage navigation: Reason selection screen followed by `ReportDetailsBox` for optional context text.

### 2.20 Reciprocal Privacy Prompt (`show_or_premium_box.cpp`, `show_or_premium_box.h`)
- **Upstream C++ Implementation**: `show_or_premium_box.cpp:L25-L218`, `show_or_premium_box.h:L15-L32`
- **Extracted Constants & Logic**:
  - `kShowOrLineOpacity = 0.3` (`show_or_premium_box.cpp:L30`): Subtle divider line opacity separating dual options.
  - Privacy Modes (`ShowOrPremium`):
    - `ShowOrPremium::LastSeen`: Prompts user when attempting to view another peer's last seen time while keeping own hidden.
    - `ShowOrPremium::ReadTime`: Prompts user when attempting to view message read receipts while having read receipts disabled.
  - Dual action choices:
    1. "Share My Last Seen / Read Time" (`justShow`): Removes privacy restriction for reciprocal viewing.
    2. "Subscribe to Telegram Premium" (`toPremium`): Grants unilateral visibility without exposing own status.

### 2.21 Single Choice Radio Dialog (`single_choice_box.cpp`, `single_choice_box.h`)
- **Upstream C++ Implementation**: `single_choice_box.cpp:L20-L55`, `single_choice_box.h:L15-L32`
- **Extracted Logic & Spacing**:
  - Radio button list backed by `RadiobuttonGroup`, pre-selected with `initialSelection`.
  - Spacing rules: Top margin `st::boxOptionListPadding.top() + st::autolockButton.margin.top()`, item skip `st::boxOptionListSkip`.
  - Auto-closing behavior: Selecting any radio option immediately triggers `callback(value)` and invokes `box->closeBox()`.

### 2.22 TTL Message Lifetime Picker (`time_picker_box.cpp`, `time_picker_box.h`)
- **Upstream C++ Implementation**: `time_picker_box.cpp:L25-L132`, `time_picker_box.h:L15-L22`
- **Extracted Discrete Table & Binary Search**:
  - `DefaultTimePickerValues` table containing exactly 16 discrete time intervals:
    1. `60 * 15` (15 minutes)
    2. `60 * 30` (30 minutes)
    3. `3600 * 1` (1 hour)
    4. `3600 * 2` (2 hours)
    5. `3600 * 3` (3 hours)
    6. `3600 * 4` (4 hours)
    7. `3600 * 8` (8 hours)
    8. `3600 * 12` (12 hours)
    9. `86400 * 1` (1 day / 24 hours)
    10. `86400 * 2` (2 days)
    11. `86400 * 3` (3 days)
    12. `86400 * 7 * 1` (1 week)
    13. `86400 * 7 * 2` (2 weeks)
    14. `86400 * 31 * 1` (1 month)
    15. `86400 * 31 * 2` (2 months)
    16. `86400 * 31 * 3` (3 months)
  - Nearest-value tie breaking (`time_picker_box.cpp:L50-L60`): Finds closest interval via lower bound; if distance to left element is strictly less than distance to right element (`std::abs(v - left) < std::abs(v - right)`), chooses left.
  - Renders using vertical cylindrical drum picker with horizontal highlight guideline brackets.

---

## 3. GPUI Rust Component Architecture

All 22 target Rust files in `crates/gpui_telegram_desktop/src/components/ui/boxes/*.rs` are refactored into high-fidelity, strongly-typed GPUI components:
- **Zero Boilerplate**: Completely eradicates Pattern B `//! Autogenerated authentic GPUI Telegram view component` and `{ label: String, value: i64 }` mock structs.
- **Strict Symbol Conformance**: Directly implements and exports the target symbols declared in `docs/specs/ledger/shards/ui.json` and `crates/gpui_telegram_desktop/src/components/ui/boxes/mod.rs`:
  - `AboutCocoonBoxBoxes`
  - `BoostBoxBoxes`
  - `CalendarBoxBoxes`
  - `ChooseDateTimeBoxes`
  - `ChooseFontBoxBoxes`
  - `ChooseLanguageBoxBoxes`
  - `ChooseTimeBoxes`
  - `CollectibleInfoBoxBoxes`
  - `ConfirmBoxBoxes`
  - `ConfirmPhoneBoxBoxes`
  - `CountrySelectBoxBoxes`
  - `EditBirthdayBoxBoxes`
  - `EditFactcheckBoxBoxes`
  - `EditInviteLinkBoxes`
  - `EditInviteLinkSessionBoxes`
  - `EmojiStakeBoxBoxes`
  - `PeerQrBoxBoxes`
  - `RateCallBoxBoxes`
  - `ReportBoxGraphicsBoxes`
  - `ShowOrPremiumBoxBoxes`
  - `SingleChoiceBoxBoxes`
  - `TimePickerBoxBoxes`

### Architectural Tenets:
1. **Interactive Event Models**: Each box maintains explicit UI state (e.g., active ratings, drum wheel scroll offsets, search queries, selected dates, validation errors, and submission progress).
2. **C++ Line Range Citation**: Every component file header explicitly cites the upstream `.cpp:L<start>-L<end>` and `.h` line ranges along with anonymous namespace constants.
3. **Exact Mathematical Parity**: Algorithms such as `TimePickerBox` nearest-value distance resolution, `AboutCocoonBox` 50-particle interpolation, `EditBirthdayBox` date bounds clamping, `ConfirmPhoneBox` auto-submit length matching, and `ChooseDateTimeBox` 10-second minimum scheduling are genuinely implemented.

---

## 4. Verification and Subsystem Test Suite

The subsystem test suite `crates/gpui_telegram_desktop/tests/ui_boxes_subcomponents_subsystem_tests.rs` covers:
1. `test_about_cocoon_constants_and_particles`: Asserts 50 particles count, 12 color steps, and correct gradient endpoints.
2. `test_boost_box_progress_and_reasons`: Validates level fraction math and 8 ask-boost reason variants.
3. `test_calendar_box_geometry_and_weekdays`: Verifies 7 columns per week, 6 max rows, and date range highlighting.
4. `test_choose_date_time_minimal_schedule`: Asserts minimum 10-second forward scheduling boundary.
5. `test_collectible_info_entity_formatting`: Tests phone vs username auto-detection and TON emoji currency formatting.
6. `test_confirm_box_strict_cancel`: Validates strict cancel callback invocation semantics.
7. `test_confirm_phone_auto_submit`: Checks auto-submission triggering when entered code matches expected length.
8. `test_country_select_filter_matching`: Tests filtering by country name, calling code prefix, and ISO code.
9. `test_edit_birthday_drum_bounds`: Validates day clamping per month and maximum year clamping to current date.
10. `test_edit_factcheck_state_transitions`: Tests Initial -> Changed -> Removed transitions and button styles.
11. `test_edit_invite_link_presets_and_limits`: Validates 1h, 1d, 1w presets and max 32-character label limit.
12. `test_emoji_stake_multipliers_and_grid`: Asserts 4 items per row, 6 prize multipliers, and jackpot reward logic.
13. `test_rate_call_stars_and_comment`: Verifies 5-star rating bounds and 200 character comment truncation.
14. `test_report_box_taxonomy_and_length`: Asserts 512 character comment limit and 9 report reason categories.
15. `test_show_or_premium_modes`: Checks LastSeen vs ReadTime modes and dual callback routing.
16. `test_single_choice_selection_callback`: Tests radio option selection and immediate close signaling.
17. `test_time_picker_discrete_intervals_and_nearest_search`: Validates all 16 discrete time intervals and tie-breaker distance math.

---

## 5. Conclusion

Module 08 Slice 10 delivers 100% C++ source coverage (all 48 assigned files accounted for in the manifest), zero boilerplate stubs across all 22 target Rust files, full adherence to Mandates #15, #17, #18, #19, #20, and clean workspace compilation.
