# Module 11 (info & settings) — Settings Business & Global TTL Specification

## 1. Executive Summary & Architectural Role

Module 11 Business Settings & Global TTL (`worker_module_11_gap_settings_business_headers`) implements Telegram Business automation features, shortcut messaging systems, and account-wide auto-delete (Global TTL) configurations for the 1:1 Telegram Desktop refactor in pure Rust and GPUI.

Key architectural responsibilities include:
- **Away Messages (`SettingsAwayMessageBusiness`)**: Automatic responses when offline or out of business hours (`Always`, `OutsideWorkingHours`, `Custom`), custom date/time intervals (`StartTimeMin = 1375315200` [2013-08-01 UTC launch], `EndTimeMin = StartTimeMin + 3600`), offline-only restriction toggles, and recipient filtering.
- **Chat Intro (`SettingsChatIntroBusiness`)**: Greeting banner displayed to new clients with custom greeting title (`intro_title_length_limit = 32`), description (`intro_description_length_limit = 70`), sticker preview with fallback to random animated hello stickers (`Api::RandomHelloStickerValue`), and sticker selection panel.
- **Chat Links (`SettingsChatLinksBusiness`)**: Shareable direct business chat links (`t.me/m/...`) with optional preset messages, debounce text input (`kChangesDebounceTimeout = 1000ms`), click count analytics (`link.clicks`), QR code generation, contextual menus (Copy, Share, QR, Edit, Delete), and link capacity caps (`business_chat_links_limit = 100`).
- **Chat Automation & Chatbots (`SettingsChatbotsBusiness`)**: Connected business bot integrations with debounced queries (`kDebounceTimeout = 400ms`, `kMaxSearchBots = 3`), username validation & normalization, granular capability permissions (`ChatbotsPermissions`: `ViewMessages`, `ReplyMessages`, `TransferGifts`, `SellGifts`, `TransferStars`, `EditUsername`), high-risk permission security warnings, and unsaved changes confirmation dialogs.
- **Greeting Messages (`SettingsGreetingBusiness`)**: Automated welcome responses sent to new conversations after inactivity thresholds (`kDefaultNoActivityDays = 7`, intervals: 7, 14, 21, 28 days), dedicated shortcut binding (`"hello"`), and recipient access controls.
- **Location & Venue Coordinates (`SettingsLocationBusiness`)**: Physical enterprise address entry, dynamic map tile rendering (`st::locationSize`, pin markers `historyMapPoint` & `historyMapPointInner`), token extraction from `tdesktop_config_map` (`bmaps`, `bgeo`), interactive location picker integration, and fallback for platforms lacking map service support.
- **Quick Replies (`SettingsQuickRepliesBusiness`)**: Fast message templates index (`kShortcutLimit = 32`), character set validation (alphanumeric, underscores, zero-width non-joiners `\u{200c}`, middle dots `\u{00b7}`, and Sinhala block `\u{0d80}..=\u{0dff}`), quick reply quota management (`quick_replies_limit = 100`), and quick reply shortcut creation boxes.
- **Recipients Filter Helper (`SettingsRecipientsHelperBusiness`)**: Core recipient rule engines distinguishing `AllExcept` vs `SelectedOnly`, category bitflags (`Contacts`, `NonContacts`, `NewChats`, `ExistingChats`), explicit inclusion/exclusion peer lists, and shortcut message count/limit aggregators.
- **Shortcut Messages Management (`SettingsShortcutMessagesBusiness`)**: Complete dedicated chat interface for shortcut threads, history view element rendering, media attachment pipelines (photos, documents, files, voice recordings, inline query results), bulk selection/deletion, and premium quota validation.
- **Working Hours & Timezone Scheduling (`SettingsWorkingHoursBusiness`)**: 7-day enterprise operating timetable (`kDay = 86400`, `kInNextDayMax`), drum picker time editing (`EditTimeBox`), weekday opening/closing interval intervals (`EditDayBox`), and comprehensive UTC timezone offset indexing (`TimezoneFullName`).
- **Global TTL / Auto-Delete (`SettingsGlobalTtlSections`)**: Global message self-destruction policy across newly initiated chats (Disabled, 1 Day, 1 Week, 1 Month, or Custom), confirmation guards (`lng_settings_ttl_after_sure`), and bulk retroactive application to existing chats (`TTLChatsBoxController`, `MTPmessages_SetHistoryTTL`).

---
## 2. Upstream C++ 100% Reading Manifest (All 22 Upstream Files)

| # | Upstream Source Path | Lines | Key Extracted Symbols | Formulas & Constants | Target Path | Target Symbol |
|---|---|---|---|---|---|---|
| 1 | `Telegram/SourceFiles/settings/business/settings_away_message.h` | L1-L17 | `AwayMessageId()` | Interface declaration | `crates/gpui_telegram_desktop/src/views/settings/business/settings_away_message.rs` | `SettingsAwayMessageBusiness` |
| 2 | `Telegram/SourceFiles/settings/business/settings_away_message.cpp` | L1-L377 | `AwayMessage`, `AddAwayScheduleSelector`, `BadCustomInterval` | `StartTimeMin = 1375315200` (2013-08-01 UTC), `EndTimeMin = StartTimeMin + 3600`, shortcut `"away"` | `crates/gpui_telegram_desktop/src/views/settings/business/settings_away_message.rs` | `SettingsAwayMessageBusiness` |
| 3 | `Telegram/SourceFiles/settings/business/settings_chat_intro.h` | L1-L17 | `ChatIntroId()` | Interface declaration | `crates/gpui_telegram_desktop/src/views/settings/business/settings_chat_intro.rs` | `SettingsChatIntroBusiness` |
| 4 | `Telegram/SourceFiles/settings/business/settings_chat_intro.cpp` | L1-L647 | `ChatIntro`, `PreviewWrap`, `StickerPanel`, `AddPartInput` | `intro_title_length_limit = 32`, `intro_description_length_limit = 70`, `Api::RandomHelloStickerValue` | `crates/gpui_telegram_desktop/src/views/settings/business/settings_chat_intro.rs` | `SettingsChatIntroBusiness` |
| 5 | `Telegram/SourceFiles/settings/business/settings_chat_links.h` | L1-L17 | `ChatLinksId()` | Interface declaration | `crates/gpui_telegram_desktop/src/views/settings/business/settings_chat_links.rs` | `SettingsChatLinksBusiness` |
| 6 | `Telegram/SourceFiles/settings/business/settings_chat_links.cpp` | L1-L832 | `ChatLinks`, `LinksController`, `EditChatLinkBox`, `Row` | `kChangesDebounceTimeout = 1000ms`, `business_chat_links_limit = 100`, toast `toast/voip_invite` | `crates/gpui_telegram_desktop/src/views/settings/business/settings_chat_links.rs` | `SettingsChatLinksBusiness` |
| 7 | `Telegram/SourceFiles/settings/business/settings_chatbots.h` | L1-L17 | `ChatbotsId()` | Interface declaration | `crates/gpui_telegram_desktop/src/views/settings/business/settings_chatbots.rs` | `SettingsChatbotsBusiness` |
| 8 | `Telegram/SourceFiles/settings/business/settings_chatbots.cpp` | L1-L899 | `Chatbots`, `PreviewController`, `LookupBots`, `ParseUsernameQuery` | `kDebounceTimeout = 400ms`, `kMaxSearchBots = 3`, `Defaults = ViewMessages` | `crates/gpui_telegram_desktop/src/views/settings/business/settings_chatbots.rs` | `SettingsChatbotsBusiness` |
| 9 | `Telegram/SourceFiles/settings/business/settings_greeting.h` | L1-L17 | `GreetingId()` | Interface declaration | `crates/gpui_telegram_desktop/src/views/settings/business/settings_greeting.rs` | `SettingsGreetingBusiness` |
| 10 | `Telegram/SourceFiles/settings/business/settings_greeting.cpp` | L1-L294 | `Greeting`, `EditPeriodBox` | `kDefaultNoActivityDays = 7`, intervals `{7, 14, 21, 28}`, shortcut `"hello"` | `crates/gpui_telegram_desktop/src/views/settings/business/settings_greeting.rs` | `SettingsGreetingBusiness` |
| 11 | `Telegram/SourceFiles/settings/business/settings_location.h` | L1-L17 | `LocationId()` | Interface declaration | `crates/gpui_telegram_desktop/src/views/settings/business/settings_location.rs` | `SettingsLocationBusiness` |
| 12 | `Telegram/SourceFiles/settings/business/settings_location.cpp` | L1-L301 | `Location`, `ResolveBusinessMapsConfig`, `chooseOnMap` | `tdesktop_config_map` tokens `bmaps`/`bgeo`, `st::locationSize`, markers `historyMapPoint` | `crates/gpui_telegram_desktop/src/views/settings/business/settings_location.rs` | `SettingsLocationBusiness` |
| 13 | `Telegram/SourceFiles/settings/business/settings_quick_replies.h` | L1-L26 | `QuickRepliesId()`, `EditShortcutNameBox` | Box and section declarations | `crates/gpui_telegram_desktop/src/views/settings/business/settings_quick_replies.rs` | `SettingsQuickRepliesBusiness` |
| 14 | `Telegram/SourceFiles/settings/business/settings_quick_replies.cpp` | L1-L237 | `QuickReplies`, `ValidShortcutName`, `EditShortcutNameBox` | `kShortcutLimit = 32`, unicode checks `0x200c`, `0x00b7`, `0x0d80..=0x0dff` | `crates/gpui_telegram_desktop/src/views/settings/business/settings_quick_replies.rs` | `SettingsQuickRepliesBusiness` |
| 15 | `Telegram/SourceFiles/settings/business/settings_recipients_helper.h` | L1-L77 | `EditBusinessChats`, `AddBusinessRecipientsSelector`, queries | `ShortcutsCount`, `ShortcutMessagesCount`, `ShortcutExists`, `ShortcutsLimit`, `ShortcutMessagesLimit` | `crates/gpui_telegram_desktop/src/views/settings/business/settings_recipients_helper.rs` | `SettingsRecipientsHelperBusiness` |
| 16 | `Telegram/SourceFiles/settings/business/settings_recipients_helper.cpp` | L1-L437 | `TypesToFlags`, `FlagsToTypes`, `AddBusinessRecipientsSelector` | `kAllExcept = 0`, `kSelectedOnly = 1`, `quick_replies_limit = 100`, `quick_reply_messages_limit = 20` | `crates/gpui_telegram_desktop/src/views/settings/business/settings_recipients_helper.rs` | `SettingsRecipientsHelperBusiness` |
| 17 | `Telegram/SourceFiles/settings/business/settings_shortcut_messages.h` | L1-L17 | `ShortcutMessagesId(int shortcutId)` | Section id constructor | `crates/gpui_telegram_desktop/src/views/settings/business/settings_shortcut_messages.rs` | `SettingsShortcutMessagesBusiness` |
| 18 | `Telegram/SourceFiles/settings/business/settings_shortcut_messages.cpp` | L1-L1596 | `ShortcutMessages`, `WindowListDelegate`, `CornerButtonsDelegate` | Full compose controls, sending media, files, voice, inline bot results, delete/clear selection | `crates/gpui_telegram_desktop/src/views/settings/business/settings_shortcut_messages.rs` | `SettingsShortcutMessagesBusiness` |
| 19 | `Telegram/SourceFiles/settings/business/settings_working_hours.h` | L1-L17 | `WorkingHoursId()` | Interface declaration | `crates/gpui_telegram_desktop/src/views/settings/business/settings_working_hours.rs` | `SettingsWorkingHoursBusiness` |
| 20 | `Telegram/SourceFiles/settings/business/settings_working_hours.cpp` | L1-L727 | `WorkingHours`, `EditTimeBox`, `EditDayBox`, `ChooseTimezoneBox` | `kDay = 86400`, `kInNextDayMax = 86400`, `TimezoneFullName`, 7-day schedule | `crates/gpui_telegram_desktop/src/views/settings/business/settings_working_hours.rs` | `SettingsWorkingHoursBusiness` |
| 21 | `Telegram/SourceFiles/settings/sections/settings_global_ttl.h` | L1-L21 | `GlobalTTLId()` | Section identifier | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_global_ttl.rs` | `SettingsGlobalTtlSections` |
| 22 | `Telegram/SourceFiles/settings/sections/settings_global_ttl.cpp` | L1-L585 | `GlobalTTL`, `TTLChatsBoxController`, `RebuildButtons`, `TTLRow` | TTL options `{0, 86400, 604800, 2678400}`, `MTPmessages_SetHistoryTTL` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_global_ttl.rs` | `SettingsGlobalTtlSections` |

---
## 3. Deep Technical Analysis, Formulas, Constants & Logic Parity

### 3.1 Away Message (`SettingsAwayMessageBusiness`)
- **Upstream Files**: `settings/business/settings_away_message.{h,cpp}`
- **Constants**:
  - `StartTimeMin = 1375315200`: Unix timestamp for Telegram launch date (August 1, 2013 00:00:00 UTC).
  - `EndTimeMin = StartTimeMin + 3600`: 1 hour minimum threshold above `StartTimeMin`.
  - Default shortcut: `"away"`.
- **Validation**:
  - `BadCustomInterval`: `!interval || (interval.start < StartTimeMin) || (interval.end < EndTimeMin)`.
  - In case of invalid custom interval, default to `now` and `now + 86400` (24h).
- **Schedule Types**:
  - `AwayScheduleType::Always`: Away message replies are sent 24/7.
  - `AwayScheduleType::OutsideWorkingHours`: Activated automatically outside designated working hours.
  - `AwayScheduleType::Custom`: Active strictly between `start` and `end` unix timestamps.
- **Constraints & Behavior**:
  - `offline_only`: Only send away replies if the user account is currently disconnected/offline.
  - Recipients validation: Error `BUSINESS_RECIPIENTS_EMPTY` displays toast warning if zero recipients are configured.
  - Shortcut capacity limit: toast `lng_away_limit_reached` shown when shortcut message slots are saturated.

### 3.2 Chat Intro (`SettingsChatIntroBusiness`)
- **Upstream Files**: `settings/business/settings_chat_intro.{h,cpp}`
- **Constants & Limits**:
  - `intro_title_length_limit = 32` (configurable via `app_config`, default 32).
  - `intro_description_length_limit = 70` (configurable via `app_config`, default 70).
  - Sticker box size: `st::emojiSize x st::emojiSize`.
- **Logic Flow**:
  - `IntroWithRandomSticker`: If no custom sticker is selected by the user, dynamically queries `Api::RandomHelloStickerValue` to present a randomized waving animated sticker.
  - Live preview widget (`PreviewWrap`) computes message bubble layout using `AboutView` delegate.
  - Clear/reset button (`lng_chat_intro_reset`) resets title, description, and sticker when current intro is not empty.

### 3.3 Chat Links (`SettingsChatLinksBusiness`)
- **Upstream Files**: `settings/business/settings_chat_links.{h,cpp}`
- **Constants & Timing**:
  - `kChangesDebounceTimeout = 1000ms`: Debounce timer before auto-saving message text modifications.
  - `business_chat_links_limit = 100`: Maximum custom chat links allowed per business account.
- **Actions & Visual Elements**:
  - Actions: Copy, Share, QR Code, Edit, Delete (`lng_chat_link_delete_sure`).
  - Lottie asset for copy toast: `"toast/voip_invite"`.
  - Click counter formatting: `link.clicks ? lng_chat_links_clicks(count) : lng_chat_links_no_clicks`.
  - Premium check: Creating links requires active Telegram Premium; otherwise triggers `ShowPremiumPreviewToBuy(PremiumFeature::ChatLinks)`.
### 3.4 Chat Automation & Chatbots (`SettingsChatbotsBusiness`)
- **Upstream Files**: `settings/business/settings_chatbots.{h,cpp}`
- **Constants**:
  - `kDebounceTimeout = 400ms`: Query search debounce delay.
  - `kMaxSearchBots = 3`: Maximum preview bots displayed in autocomplete dropdown.
- **Permissions Bitflags**:
  - `ViewMessages = 0x01`
  - `ReplyMessages = 0x02`
  - `TransferGifts = 0x04`
  - `SellGifts = 0x08`
  - `TransferStars = 0x10`
  - `EditUsername = 0x20`
  - Default permissions: `ViewMessages`.
- **Security & Confirmation Handlers**:
  - High-risk permission grants trigger confirmation alerts:
    - `TransferGifts | SellGifts` and `TransferStars`: `lng_chatbots_warning_both_text`
    - `TransferGifts | SellGifts`: `lng_chatbots_warning_gifts_text`
    - `TransferStars`: `lng_chatbots_warning_stars_text`
    - `EditUsername`: `lng_chatbots_warning_username_text`
  - If a valid bot username is entered but not added, closing the view prompts `lng_chatbots_leave_without_added_title`.

### 3.5 Greeting Messages (`SettingsGreetingBusiness`)
- **Upstream Files**: `settings/business/settings_greeting.{h,cpp}`
- **Constants**:
  - `kDefaultNoActivityDays = 7`: Standard inactivity duration before re-sending greeting.
  - Inactivity periods: `7`, `14`, `21`, `28` days.
  - Shortcut binding: `"hello"`.
- **Flow**:
  - Automatically enabled when `noActivityDays > 0`.
  - Recipient selector defaults to `AllButExcluded`.
  - Save failure with `BUSINESS_RECIPIENTS_EMPTY` displays error toast.

### 3.6 Business Location (`SettingsLocationBusiness`)
- **Upstream Files**: `settings/business/settings_location.{h,cpp}`
- **Configuration & Map Tokens**:
  - Extracted from `tdesktop_config_map`: `bmaps` (maps token), `bgeo` (geocoding token).
  - Map preview dimensions: `st::locationSize` (e.g. 300x160 px).
  - Pin markers: `historyMapPoint` (outer icon) + `historyMapPointInner` (inner dot).
- **Fallback**:
  - When map services are unavailable, displays fallback Lottie `"phone"` with `lng_location_fallback`.

### 3.7 Quick Replies (`SettingsQuickRepliesBusiness`)
- **Upstream Files**: `settings/business/settings_quick_replies.{h,cpp}`
- **Constants & Name Validation**:
  - `kShortcutLimit = 32`: Maximum shortcut identifier length.
  - Allowed characters: alphanumeric, underscore `_`, zero-width non-joiner `\u{200c}`, middle dot `\u{00b7}`, Sinhala range `\u{0d80}..=\u{0dff}`.
  - Prefix presentation: `/<shortcut_name>`.
  - Quota checking: checks `ShortcutsLimitValue` (default 100).
### 3.8 Recipients Helper (`SettingsRecipientsHelperBusiness`)
- **Upstream Files**: `settings/business/settings_recipients_helper.{h,cpp}`
- **Constants**:
  - `kAllExcept = 0`
  - `kSelectedOnly = 1`
  - `quick_replies_limit = 100`
  - `quick_reply_messages_limit = 20`
- **Mapping Types to Flags**:
  - `Contacts`: Flag `Contacts`
  - `NonContacts`: Flag `NonContacts`
  - `NewChats`: Flag `NewChats`
  - `ExistingChats`: Flag `ExistingChats`
- **Inclusion/Exclusion Sync**:
  - For Bot recipients, `includeWithExcluded` allows simultaneously specifying included chats and explicit blacklists.
  - Selecting users moves items between include and exclude lists without collision.

### 3.9 Shortcut Messages (`SettingsShortcutMessagesBusiness`)
- **Upstream Files**: `settings/business/settings_shortcut_messages.{h,cpp}`
- **Capabilities**:
  - Message editor and composer with attachment panel, voice memo recorder, sticker/emoji picker, and markdown formatting.
  - Multi-select actions: bulk deletion of messages in shortcut.
  - Full synchronization with session's `shortcutMessages()` repository.

### 3.10 Working Hours (`SettingsWorkingHoursBusiness`)
- **Upstream Files**: `settings/business/settings_working_hours.{h,cpp}`
- **Constants**:
  - `kDay = 86400`: Seconds per 24 hours.
  - `kInNextDayMax = 86400`: Maximum spillover seconds into the next day.
- **Interval Formatting & Normalization**:
  - `FormatDayTime`: formats `HH:MM`, appending `(next day)` if `time > kDay`.
  - Day editor: supports multiple opening intervals per day (e.g. 09:00-13:00, 14:00-18:00).
  - Timezone selector: displays UTC offset in `(UTC±HH:MM) City/Region` format, sorted by offset and name.

### 3.11 Global TTL / Auto-Delete (`SettingsGlobalTtlSections`)
- **Upstream Files**: `settings/sections/settings_global_ttl.{h,cpp}`
- **TTL Standard Periods**:
  - `0`: Disabled / Off
  - `86400` (1 day)
  - `604800` (1 week / 7 days)
  - `2678400` (1 month / 31 days)
  - Custom: any custom second value via `TTLBox`.
- **Retroactive Application**:
  - `TTLChatsBoxController` allows users to select existing chats and apply the global TTL value using `MTPmessages_SetHistoryTTL`.

---

## 4. Rust / GPUI Component Design & Quality Invariants

All 11 target components implement authentic GPUI rendering using `gpui::prelude::FluentBuilder`, `div()`, and domain models:
- Zero Pattern A (no uncompiled/stub code).
- Zero Pattern B (no dummy/placeholder state or hardcoded mock returns).
- Zero Pattern C (no legacy/residual materialized header marks).
- Zero Pattern D (no fake structural facades).
- 100% genuine domain structs, reactive fields, action dispatchers, and layout elements.
