# Module 06 — History Subsystem: Slice 2 (History Item Models, Components & Admin Log)

## Executive Summary & Scope Overview

This specification documents the analysis and genuine Rust + GPUI implementation design for **Slice 2 of Module 06 (History Subsystem)** in strict accordance with **User Mandates #15, #17, #18, and #19**.

Slice 2 covers the core message domain models, runtime component composition, edition and translation tracking, reply markups, text selection arithmetic, and administrative log event filtering and pagination across **22 upstream C++ files** (`~24,565 lines of code`) in:
- `Telegram/SourceFiles/history/history_item.*`
- `Telegram/SourceFiles/history/history_item_components.*`
- `Telegram/SourceFiles/history/history_item_edition.*`
- `Telegram/SourceFiles/history/history_item_helpers.*`
- `Telegram/SourceFiles/history/history_item_reply_markup.*`
- `Telegram/SourceFiles/history/history_item_text.*`
- `Telegram/SourceFiles/history/history_message_selection.h`
- `Telegram/SourceFiles/history/admin_log/*` (9 files)

---

## Mandate #18: Anti-Bluffing C++ Implementation Citations & Magic Constants

To guarantee genuine comprehension and prevent superficial name guessing, all critical algorithmic rules, bitmasks, and magic constants extracted directly from the `.cpp` implementation bodies are cited below with exact line numbers:

### 1. History Item Constants & Notification Rules
- **Exact Line Citation**: `Telegram/SourceFiles/history/history_item.cpp:L97-L100`
  - `constexpr auto kNotificationTextLimit = 255;`
  - `constexpr auto kPinnedMessageTextLimit = 16;`
  - `constexpr auto kMinLoginCode = 5;`
  - `constexpr auto kTonConnectRequestMaxDelay = 24 * 3600 * crl::time(1000);` (24h in milliseconds)
- **TTL Media Expiry Check**: `history_item.cpp:L109-L120` (`ShowTtlMediaAsExpired`): Returns true if media has `vttl_seconds` and message is incoming and `hasUnreadMediaFlag()` is false.

### 2. Message Edit & Revocation Time Bounds
- **Edit Time Limit**: `Telegram/SourceFiles/history/history_item.cpp:L3367-L3372` (`isTooOldForEdit`):
  ```cpp
  return !_history->peer->canEditMessagesIndefinitely()
      && !isScheduled()
      && !isWelcomeTemplate()
      && (now - date() >= _history->session().serverConfig().editTimeLimit);
  ```
  - Standard `editTimeLimit` is 48 hours (`172,800` seconds).
  - Saved Messages / Self peer has `canEditMessagesIndefinitely() == true`, so messages can be edited indefinitely.
  - Scheduled messages and welcome templates are exempt from the edit time limit.
- **Edit Permission Guard**: `Telegram/SourceFiles/history/history_item.cpp:L3374-L3386` (`allowsEdit`):
  - Must not be a service message (`!isService()`).
  - Must pass `canBeEdited()` (not forwarded, no via-bot, regular or scheduled).
  - Must pass `!isTooOldForEdit(now)`.
  - Must have `paidType() == PaidPostType::None` (paid posts cannot be edited once published).
- **Revocation / Delete For Everyone**: `Telegram/SourceFiles/history/history_item.cpp:L3490-L3524` (`canDeleteForEveryone`):
  - Private 1-on-1 chats use `revokePrivateTimeLimit`.
  - Group chats use `revokeTimeLimit`.
  - If peer is a bot (and not support), revoking for everyone is disabled (`user->isBot() && !user->isSupport()`).
  - Incoming messages can only be revoked if `config.revokePrivateInbox` is true.

### 3. Flag Conversion Bitmask Rules
- **Exact Line Citation**: `Telegram/SourceFiles/history/history_item_helpers.cpp:L949-L991` (`FlagsFromMTP`):
  - `IsServerMsgId(id) ? Flag::HistoryEntry : 0`
  - `flags & f_out -> Flag::Outgoing`
  - `flags & f_mentioned -> Flag::MentionsMe`
  - `flags & f_media_unread -> Flag::MediaIsUnread`
  - `flags & f_silent -> Flag::Silent`
  - `flags & f_post -> Flag::Post`
  - `flags & f_legacy -> Flag::Legacy`
  - `flags & f_edit_hide -> Flag::HideEdited`
  - `flags & f_pinned -> Flag::Pinned`
  - `flags & f_from_id -> Flag::HasFromId`
  - `flags & f_reply_to -> Flag::HasReplyInfo`
  - `flags & f_reply_markup -> Flag::HasReplyMarkup`
  - `flags & f_quick_reply_shortcut_id -> Flag::ShortcutMessage`
  - `flags & f_from_scheduled -> Flag::IsOrWasScheduled`
  - `flags & f_views -> Flag::HasViews`
  - `flags & f_noforwards -> Flag::NoForwards`
  - `flags & f_invert_media -> Flag::InvertMedia`
  - `flags & f_video_processing_pending -> Flag::EstimatedDate`
  - `flags & f_paid_suggested_post_ton -> Flag::TonPaidSuggested`
  - `flags & f_paid_suggested_post_stars -> Flag::StarsPaidSuggested`
  - `flags & f_summary_from_language -> Flag::CanBeSummarized`
  - `flags & f_guestchat_via_from -> Flag::GuestChatViaFrom`

### 4. Reply Markup Layout & Button Rules
- **Exact Line Citation**: `Telegram/SourceFiles/history/history_item_reply_markup.cpp:L27-L50` (`kButtonTypeIcons`):
  - Maps all 22 button types to their respective icons: `Url` -> `TypeIcon::Url`, `Buy` -> `TypeIcon::Payment`, `SwitchInline`/`SwitchInlineSame` -> `TypeIcon::SwitchPm`, `WebView`/`SimpleWebView` -> `TypeIcon::Webview`, `CopyText` -> `TypeIcon::Copy`.
- **Activation Loading**: `history_item_reply_markup.cpp:L346-L350` (`LoadsOnActivate`):
  - Only `Callback`, `CallbackWithPassword`, and `Game` display loading spinners when activated.
- **Rich Page Button Key**: `history_item_reply_markup.cpp:L359-L368` (`RichPageButtonKey`):
  - Encoded as `type;buttonId;peerTypes;data`.
- **Single Buy Button Optimization**: `history_item_reply_markup.cpp:L438-L442`:
  - When `rows.size() == 1 && rows[0].size() == 1 && rows[0][0].type == Type::Buy`, sets `ReplyMarkupFlag::OnlyBuyButton`.
- **Forwarded Markup Rewriting**: `history_item_reply_markup.cpp:L476-L508` (`fillForwardedData`):
  - Converts `SwitchInlineSame` to `SwitchInline`.
  - Replaces text with `forwardText` if non-empty: `button.forwardText.isEmpty() ? button.text : button.forwardText`.

### 5. Selection Math & Copy Text Limits
- **Exact Line Citation**: `Telegram/SourceFiles/history/history_item_text.cpp:L32`:
  - `constexpr auto kSelectedCopyReplyPreviewLimit = 64;`
- **Selection Endpoints Math**: `Telegram/SourceFiles/history/history_message_selection.h:L21-L31`:
  - `offset() = symbol + (afterSymbol ? 1 : 0)`.
- **Selection Normalization**: `Telegram/SourceFiles/history/history_message_selection.h:L115-L129`:
  - `anchorOffset < focusOffset ? TextSelection(anchorOffset, focusOffset) : TextSelection(focusOffset, anchorOffset)`.
- **Selection Contains**: `history_message_selection.h:L191-L197`:
  - `isFlat() && endpoint.isFlat() && (endpoint.flat.symbol >= flat.from) && (endpoint.flat.symbol < flat.to)`.

### 6. Admin Log Filter Bitmasks & Pagination
- **Exact Line Citation**: `Telegram/SourceFiles/history/admin_log/history_admin_log_filter_value.h:L13-L35` (`FilterValue::Flag`):
  - 19 bitmask flags from `Join = (1U << 0)` through `EditRank = (1U << 18)`, `MAX_FIELD = (1U << 18)`.
- **Channel vs Group Filter Categories**: `Telegram/SourceFiles/history/admin_log/history_admin_log_filter.cpp:L17-L84` (`FilterValueLabels`):
  - Subscribers/Members: `adminRights = Promote | Demote`, `restrictions = Ban | Unban | Kick | Unkick`, `membersNew = Join | Invite`, `membersRemoved = Leave`, `EditRank`.
  - Settings: `info = Info | Settings`, `invites = Invites`, `calls = GroupCall`, `SubExtend`, and conditional `Topics` (groups only).
  - Messages: `Delete`, `Edit`, and conditional `Pinned` (groups only).
- **Admin Log Pagination Constants**: `Telegram/SourceFiles/history/admin_log/history_admin_log_inner.cpp:L89-L103`:
  - `kMaxChannelAdmins = 200;`
  - `kEventsFirstPage = 100;`
  - `kEventsPerPage = 100;`
  - `kPreloadUpDisplayItemsBuffer = 4;`
  - `kNewEventsCheckInterval = crl::time(5000);` (5s)
  - `kDeleteGroupTimeWindowSeconds = TimeId(5);` (Max 5s window to group consecutive message deletions by the same admin!)

---

## Mandate #19 Pillar 2: 100% Reading Manifest (All 22 Assigned Upstream Files)

| # | Upstream Source Path | Line Count | Core Symbols Extracted | Constants & Bitmasks | Target Rust Path | Disposition |
|---|---|---|---|---|---|---|
| 1 | `Telegram/SourceFiles/history/history_item.h` | 847 | `HistoryItem`, `HistoryItemCommonFields`, `HistoryReactionSource`, `PaidPostType` | MessageFlags, SuggestionActions | `crates/gpui_telegram_client/src/history/item.rs` | `rust_domain_1to1` |
| 2 | `Telegram/SourceFiles/history/history_item.cpp` | 9,245 | `CreateMedia`, `allowsEdit`, `isTooOldForEdit`, `canDeleteForEveryone`, `destroy` | `kNotificationTextLimit = 255`, `kPinnedMessageTextLimit = 16`, `kMinLoginCode = 5`, `kTonConnectRequestMaxDelay = 86400000` | `crates/gpui_telegram_client/src/history/item.rs` | `rust_domain_1to1` |
| 3 | `Telegram/SourceFiles/history/history_item_components.h` | 1,029 | `HistoryMessageVia`, `HistoryMessageViews`, `HistoryMessageReply`, `HistoryMessageForwarded`, `HistoryMessageEdited` | 26 Component structs, `kMaxRecentRepliers = 3` | `crates/gpui_telegram_client/src/history/item_components.rs` | `rust_domain_1to1` |
| 4 | `Telegram/SourceFiles/history/history_item_components.cpp` | 1,685 | `fillForwardedData`, `createComponents`, `resolveDependent`, `refreshRepliesText` | `FastButtonsMode`, `kOptionFastButtonsMode` | `crates/gpui_telegram_client/src/history/item_components.rs` | `rust_domain_1to1` |
| 5 | `Telegram/SourceFiles/history/history_item_edition.h` | 66 | `HistoryMessageEdition`, `HistoryMessageContent` | `useSameViews`, `useSameMarkup`, `invertMedia` | `crates/gpui_telegram_client/src/history/item.rs` | `rust_domain_1to1` |
| 6 | `Telegram/SourceFiles/history/history_item_edition.cpp` | 51 | `HistoryMessageEdition::HistoryMessageEdition` | MTPDmessage parsing, TTL period addition | `crates/gpui_telegram_client/src/history/item.rs` | `rust_domain_1to1` |
| 7 | `Telegram/SourceFiles/history/history_item_helpers.h` | 311 | `PreparedServiceText`, `FlagsFromMTP`, `CheckMessageMedia`, `MediaCheckResult` | `kMaxUnreadReactions = 5` | `crates/gpui_telegram_client/src/history/item.rs` | `rust_domain_1to1` |
| 8 | `Telegram/SourceFiles/history/history_item_helpers.cpp` | 1,530 | `FlagsFromMTP`, `ComputeSendingMessagesCount`, `PeerCallKnown` | 22 `MessageFlags` bitwise conversions | `crates/gpui_telegram_client/src/history/item.rs` | `rust_domain_1to1` |
| 9 | `Telegram/SourceFiles/history/history_item_reply_markup.h` | 226 | `ReplyMarkupFlag`, `HistoryMessageMarkupButton`, `HistoryMessageMarkupData`, `RequestPeerQuery` | 13 `ReplyMarkupFlags`, 22 `ButtonType` enum values | `crates/gpui_telegram_client/src/history/item_reply_markup.rs` | `rust_domain_1to1` |
| 10 | `Telegram/SourceFiles/history/history_item_reply_markup.cpp` | 570 | `ParseButton`, `LoadsOnActivate`, `RichPageButtonKey`, `RegisterRichPageButton`, `fillForwardedData` | `kButtonTypeIcons` table, `OnlyBuyButton` flag rule | `crates/gpui_telegram_client/src/history/item_reply_markup.rs` | `rust_domain_1to1` |
| 11 | `Telegram/SourceFiles/history/history_item_text.h` | 46 | `HistorySelectedTextEntry`, `HistorySelectedTextImage`, `HistorySelectedTextParts`, `HistoryItemText` | MIME data composition, text slice helpers | `crates/gpui_telegram_client/src/history/selection.rs` | `rust_domain_1to1` |
| 12 | `Telegram/SourceFiles/history/history_item_text.cpp` | 814 | `HistorySelectedItemsTextParts`, `AppendExtraCopyText`, `LogEntryOriginalText` | `kSelectedCopyReplyPreviewLimit = 64` | `crates/gpui_telegram_client/src/history/selection.rs` | `rust_domain_1to1` |
| 13 | `Telegram/SourceFiles/history/history_message_selection.h` | 201 | `MessageSelectionKind`, `MessageSelectionFlatEndpoint`, `MessageSelectionEndpoint`, `MessageSelection` | `symbol + (afterSymbol ? 1 : 0)`, selection contains logic | `crates/gpui_telegram_client/src/history/selection.rs` | `rust_domain_1to1` |
| 14 | `Telegram/SourceFiles/history/admin_log/history_admin_log_filter.h` | 32 | `FillFilterValueList`, `FilterValueLabels` | Channel vs Supergroup descriptors | `crates/gpui_telegram_client/src/history/admin_log.rs` | `rust_domain_1to1` |
| 15 | `Telegram/SourceFiles/history/admin_log/history_admin_log_filter.cpp` | 117 | `FilterValueLabels`, `adminRights`, `restrictions`, `membersNew`, `membersRemoved` | Section groupings, dynamic topic/pinned inclusion | `crates/gpui_telegram_client/src/history/admin_log.rs` | `rust_domain_1to1` |
| 16 | `Telegram/SourceFiles/history/admin_log/history_admin_log_filter_value.h` | 55 | `FilterValue`, `FilterValue::Flag` | 19 bitmask flags (`Join = 1<<0` to `EditRank = 1<<18`) | `crates/gpui_telegram_client/src/history/admin_log.rs` | `rust_domain_1to1` |
| 17 | `Telegram/SourceFiles/history/admin_log/history_admin_log_inner.h` | 499 | `AdminLog::InnerWidget`, `DeleteGroup` | Event streams, pagination bounds | `crates/gpui_telegram_client/src/history/admin_log.rs` | `rust_domain_1to1` |
| 18 | `Telegram/SourceFiles/history/admin_log/history_admin_log_inner.cpp` | 3,532 | `enumerateItems`, `loadEvents`, `DeleteGroup` collection | `kMaxChannelAdmins = 200`, `kEventsPerPage = 100`, `kDeleteGroupTimeWindowSeconds = 5`, `kPreloadUpDisplayItemsBuffer = 4` | `crates/gpui_telegram_client/src/history/admin_log.rs` | `rust_domain_1to1` |
| 19 | `Telegram/SourceFiles/history/admin_log/history_admin_log_item.h` | 60 | `AdminLog::OwnedItem`, `GenerateItems` | `ElementDelegate`, `HistoryView::Element` adapter | `crates/gpui_telegram_client/src/history/admin_log.rs` | `rust_domain_1to1` |
| 20 | `Telegram/SourceFiles/history/admin_log/history_admin_log_item.cpp` | 2,352 | `GenerateItems`, `ExtractSentDate`, `ExtractRealMsgId`, `PrepareLogReply` | ChannelAdminLogEvent mapping to service items | `crates/gpui_telegram_client/src/history/admin_log.rs` | `rust_domain_1to1` |
| 21 | `Telegram/SourceFiles/history/admin_log/history_admin_log_section.h` | 219 | `AdminLog::Widget`, `SectionMemento` | ElasticScroll, JumpDownButton | `crates/gpui_telegram_desktop/src/views/stage/admin_log/history_admin_log_section.rs` | `gpui_ui_1to1` |
| 22 | `Telegram/SourceFiles/history/admin_log/history_admin_log_section.cpp` | 678 | `applyFilter`, `showFilter`, `scrollDownClicked`, `saveState`, `restoreState` | UI event handling & scroll position sync | `crates/gpui_telegram_desktop/src/views/stage/admin_log/history_admin_log_section.rs` | `gpui_ui_1to1` |

---

## Architectural Mapping & Target Rust Structure

In accordance with **Mandate #17 (Dynamic Target Architecture Determined After Reading)**:
- Core domain logic, state machines, and mathematical models reside in `crates/gpui_telegram_client/src/history/`:
  - `item.rs`: `HistoryItem`, `HistoryItemCommonFields`, `MessageFlags`, `MessageEdition`, `MessageContent`, edit & delete bounds math.
  - `item_components.rs`: Full component tree (`HistoryMessageVia`, `Views`, `Reply`, `Forwarded`, `Factcheck`, `Suggestion`, `ServiceComponents`).
  - `item_reply_markup.rs`: `HistoryMessageMarkupData`, `HistoryMessageMarkupButton`, button types, flags, grid layout rules, rich page key encoding.
  - `selection.rs`: Text selection endpoints, offset math, boundary containment, multi-item copy formatting.
  - `admin_log.rs`: `FilterValue`, `FilterFlags` bitmask, admin log event parser, 5s delete grouping window, pagination controller (`kEventsPerPage = 100`, `kPreloadUpDisplayItemsBuffer = 4`).
- Desktop GPUI Stage Views reside in `crates/gpui_telegram_desktop/src/views/stage/`:
  - Replaces all 12 existing boilerplate files (`history_item.rs`, `history_item_components.rs`, `history_item_edition.rs`, `history_item_helpers.rs`, `history_item_reply_markup.rs`, `history_item_text.rs`, `history_message_selection.rs`, and the 5 `admin_log/*.rs` files) with genuine GPUI rendering elements and models.
  - Stage-specific wrapper components in `crates/gpui_telegram_desktop/src/views/stage/item/`.
