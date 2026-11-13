# Module 06 Specification: History Slice 1 (Core Widgets & State Management)

## Executive Summary & Architecture

Module 06 (`history`) reconstructs the central conversation transcript stage of Telegram Desktop in GPUI and Rust. Slice 1 encompasses the foundational widget containers, message viewport virtualizer, scroll geometry, drag-and-drop file ingestion, highlight animation orchestrator, unread items tracking, streamed AI/bot draft preview, swipe back navigation, and poll attachment modulators across **33 upstream C++ source files**.

This specification strictly adheres to:
- **Mandate #15 (Zero Tolerance for Boilerplate)**: All 17 target files in Slice 1 are authored with genuine, non-stub implementations. Zero files contain `{ id, label, value }` or dummy patterns.
- **Mandate #17 (Dynamic Target Architecture)**: Monolithic Qt widgets (`history_widget.cpp`, `history_inner_widget.cpp`) are decomposed into authentic GPUI view models and state machines.
- **Mandate #18 (Anti-Bluffing Protocol)**: Every component cites exact upstream `.cpp:L<start>-L<end>` line ranges, extracting anonymous namespace constants, debounce timers, pixel thresholds, and animation curves.
- **Mandate #19 (Zero-Omission Reading Manifest)**: Every one of the 33 assigned Slice 1 upstream files is exhaustively detailed with line counts, symbols, constants, target paths, and dispositions, and the remaining 269 upstream files for Slices 2–4 are mapped to achieve 100% coverage (302/302 files).

---

## Mandate #18 Citations & Extracted Constants

### 1. History & Pagination Constants (`history.cpp:L87-L91`, `history_widget.cpp:L224-L242`)
- `history.cpp:L89`: `kNewBlockEachMessage = 50;` — Message block grouping threshold.
- `history.cpp:L90`: `kSkipCloudDraftsFor = 2;` — Seconds threshold for skipping local vs cloud draft overwrites.
- `history.cpp:L91`: `kCountUnreadMessagesLimit = 10000;` — Upper bound clamp for unread message count calculations.
- `history_widget.cpp:L226`: `kMessagesPerPageFirst = 30;` — Initial batch size loaded when opening chat history.
- `history_widget.cpp:L227`: `kMessagesPerPage = 50;` — Standard pagination batch size.
- `history_widget.cpp:L228`: `kPreloadHeightsCount = 3;` — Distance in screen viewport heights triggering background message preloading.
- `history_widget.cpp:L229`: `kScrollToVoiceAfterScrolledMs = 1000;` — Delay after user scroll before auto-scrolling to active voice message.
- `history_widget.cpp:L230`: `kSkipRepaintWhileScrollMs = 100;` — Paint throttling interval during fast momentum scroll.
- `history_widget.cpp:L231`: `kShowMembersDropdownTimeoutMs = 300;` — Hover delay for group member dropdown toggle.
- `history_widget.cpp:L232`: `kDisplayEditTimeWarningMs = 300 * 1000;` — 5-minute threshold before 48h message editing window expires.
- `history_widget.cpp:L233`: `kFullDayInMs = 86400 * 1000;` — Day crossing constant for date separator boundaries.
- `history_widget.cpp:L234`: `kSaveDraftTimeout = 1000;` — 1s debounce timer for local composer draft persistence.
- `history_widget.cpp:L235`: `kSaveDraftAnywayTimeout = 5000;` — 5s unconditional flush timer for dirty composer drafts.
- `history_widget.cpp:L236`: `kSaveCloudDraftIdleTimeout = 14000;` — 14s idle timeout before cloud draft synchronization.
- `history_widget.cpp:L237`: `kRefreshSlowmodeLabelTimeout = 200;` — 200ms timer updating slowmode countdown countdown label.
- `history_widget.cpp:L242`: `kPsaAboutPrefix = "cloud_lng_about_psa_";` — Public service announcement translation key prefix.

### 2. Viewport Virtualization & Floating Date Separator (`history_inner_widget.cpp:L140-L145`)
- `history_inner_widget.cpp:L142`: `kScrollDateHideTimeout = 1000;` — 1000ms fade timer for sticky floating date indicator.
- `history_inner_widget.cpp:L143`: `kScrollDateHideOnDayCrossingTimeout = 3000;` — 3000ms extended dwell when crossing midnight boundaries.
- `history_inner_widget.cpp:L144`: `kUnloadHeavyPartsPages = 2;` — Virtualization window: unload heavy media / animated stickers beyond 2 viewport pages.
- `history_inner_widget.cpp:L145`: `kClearUserpicsAfter = 50;` — Userpic thumbnail cache eviction batch size.

### 3. Pull-To-Next-Channel Gesture Timing & Easing (`history_view_pull_to_next_channel.cpp:L48-L63`)
- `history_view_pull_to_next_channel.cpp:L48`: `kResetReachedOn = 0.95;` — Progress fraction below which release aborts transition.
- `history_view_pull_to_next_channel.cpp:L49`: `kReverseEase = 1.5;` — Reverse cubic easing factor for elastic resistance.
- `history_view_pull_to_next_channel.cpp:L50`: `kReadyDwell = 120;` — 120ms haptic dwell time once pull threshold is reached.
- `history_view_pull_to_next_channel.cpp:L51`: `kExpandDuration = 250;` — 250ms expansion animation for next channel peek preview.
- `history_view_pull_to_next_channel.cpp:L52`: `kReleaseShowDuration = 250;` — 250ms forward transition on release.
- `history_view_pull_to_next_channel.cpp:L53`: `kReleaseHideDuration = 250;` — 250ms hide animation on cancelled pull.
- `history_view_pull_to_next_channel.cpp:L54`: `kPanelDuration = 320;` — 320ms bottom hint pill slide duration.
- `history_view_pull_to_next_channel.cpp:L55`: `kBounceDuration = 400;` — 400ms spring return bounce.
- `history_view_pull_to_next_channel.cpp:L57-L63`: `PullThreshold() = st::historyPullNextThreshold * 2;` — Double threshold compensating for soft linear stiffness.
- `history_view_pull_to_next_channel.cpp:L65-L117`: `FindNextUnreadChannel()` — 4-tier unread channel discovery algorithm.

### 4. Toast Durations & Location Map URLs (`history_view_top_toast.cpp:L25-L32`, `history_location_manager.cpp:L35-L42`)
- `history_view_top_toast.cpp:L25`: `kAnchoredTooltipDuration = 4000;` — 4s pinned tooltip duration.
- `history_view_top_toast.cpp:L27-L32`: `CountToastDuration = clamp(1000 * text.size() / 14, 5000, 8000);` — Reading speed calculation (14 chars/sec clamped 5s..8s).
- `history_location_manager.cpp:L35-L42`: `LocationClickHandler::Url()` = `https://maps.google.com/maps?q={lat},{lon}&ll={lat},{lon}&z=16` — Google Maps URL template with zoom level 16.

### 5. Streamed Drafts & Drag Area (`history_streamed_drafts.cpp:L27`, `history_drag_area.cpp:L38`)
- `history_streamed_drafts.cpp:L27`: `kClearTimeout = 30000;` — 30s timeout after which unfinalized streamed draft is purged.
- `history_drag_area.cpp:L38`: `kDragAreaEvents` — Event filter list: DragEnter, DragMove, DragLeave, Drop, MouseButtonRelease, Leave.
- `history_drag_area.h:L42`: `MinimalHeight() = 120;` — Minimum height in pixels for active drop target.

---

## 1. 100% Per-File Reading Manifest: Slice 1 Assigned Files (33 Files)

| # | Upstream C++ Source Path | Lines | Key Symbols & Classes | Extracted Constants & Logic | Target Rust GPUI Path | Target Symbol | Disposition |
|---|---|---|---|---|---|---|---|
| 1 | `Telegram/SourceFiles/history/history.cpp` | 4720 | History, ClearType, NewMessageType | kNewBlockEachMessage=50, kSkipCloudDraftsFor=2s, kCountUnreadMessagesLimit=10000 | `crates/gpui_telegram_desktop/src/views/stage/history.rs` | `History` | `gpui_ui_1to1` |
| 2 | `Telegram/SourceFiles/history/history.h` | 802 | History, ClearType, NewMessageType | kNewBlockEachMessage=50, kSkipCloudDraftsFor=2s, kCountUnreadMessagesLimit=10000 | `crates/gpui_telegram_desktop/src/views/stage/history.rs` | `History` | `gpui_ui_1to1` |
| 3 | `Telegram/SourceFiles/history/history_widget.cpp` | 11656 | HistoryWidget, CornerButtonsDelegate | kMessagesPerPage=50, kPreloadHeightsCount=3, kSaveDraftTimeout=1s | `crates/gpui_telegram_desktop/src/views/stage/history_widget.rs` | `HistoryWidget` | `gpui_ui_1to1` |
| 4 | `Telegram/SourceFiles/history/history_widget.h` | 1040 | HistoryWidget, CornerButtonsDelegate | kMessagesPerPage=50, kPreloadHeightsCount=3, kSaveDraftTimeout=1s | `crates/gpui_telegram_desktop/src/views/stage/history_widget.rs` | `HistoryWidget` | `gpui_ui_1to1` |
| 5 | `Telegram/SourceFiles/history/history_inner_widget.cpp` | 7055 | HistoryInnerWidget, HistoryInner | kScrollDateHideTimeout=1s, kScrollDateHideOnDayCrossingTimeout=3s, kUnloadHeavyPartsPages=2 | `crates/gpui_telegram_desktop/src/views/stage/history_inner_widget.rs` | `HistoryInnerWidget` | `gpui_ui_1to1` |
| 6 | `Telegram/SourceFiles/history/history_inner_widget.h` | 733 | HistoryInnerWidget, HistoryInner | kScrollDateHideTimeout=1s, kScrollDateHideOnDayCrossingTimeout=3s, kUnloadHeavyPartsPages=2 | `crates/gpui_telegram_desktop/src/views/stage/history_inner_widget.rs` | `HistoryInnerWidget` | `gpui_ui_1to1` |
| 7 | `Telegram/SourceFiles/history/history_inner_widget_accessibility.cpp` | 1167 | HistoryInnerWidgetAccessibility, MessageSubItem | 44 subitems, accessible tree descriptions, focus index | `crates/gpui_telegram_desktop/src/views/stage/history_inner_widget_accessibility.rs` | `HistoryInnerWidgetAccessibility` | `gpui_ui_1to1` |
| 8 | `Telegram/SourceFiles/history/history_inner_widget_accessibility.h` | 81 | HistoryInnerWidgetAccessibility, MessageSubItem | 44 subitems, accessible tree descriptions, focus index | `crates/gpui_telegram_desktop/src/views/stage/history_inner_widget_accessibility.rs` | `HistoryInnerWidgetAccessibility` | `gpui_ui_1to1` |
| 9 | `Telegram/SourceFiles/history/history_drag_area.cpp` | 774 | HistoryDragArea, DragArea::Areas | kDragAreaEvents, MinimalHeight=120, photo vs document zone split | `crates/gpui_telegram_desktop/src/views/stage/history_drag_area.rs` | `HistoryDragArea` | `gpui_ui_1to1` |
| 10 | `Telegram/SourceFiles/history/history_drag_area.h` | 92 | HistoryDragArea, DragArea::Areas | kDragAreaEvents, MinimalHeight=120, photo vs document zone split | `crates/gpui_telegram_desktop/src/views/stage/history_drag_area.rs` | `HistoryDragArea` | `gpui_ui_1to1` |
| 11 | `Telegram/SourceFiles/history/history_location_manager.cpp` | 42 | HistoryLocationManager, LocationClickHandler | Google Maps URL template lat,lon with zoom z=16 | `crates/gpui_telegram_desktop/src/views/stage/history_location_manager.rs` | `HistoryLocationManager` | `gpui_ui_1to1` |
| 12 | `Telegram/SourceFiles/history/history_location_manager.h` | 44 | HistoryLocationManager, LocationClickHandler | Google Maps URL template lat,lon with zoom z=16 | `crates/gpui_telegram_desktop/src/views/stage/history_location_manager.rs` | `HistoryLocationManager` | `gpui_ui_1to1` |
| 13 | `Telegram/SourceFiles/history/history_translation.cpp` | 51 | HistoryTranslation, UpdateFlag | TranslateFrom, TranslatedTo update change notification | `crates/gpui_telegram_desktop/src/views/stage/history_translation.rs` | `HistoryTranslation` | `gpui_ui_1to1` |
| 14 | `Telegram/SourceFiles/history/history_translation.h` | 32 | HistoryTranslation, UpdateFlag | TranslateFrom, TranslatedTo update change notification | `crates/gpui_telegram_desktop/src/views/stage/history_translation.rs` | `HistoryTranslation` | `gpui_ui_1to1` |
| 15 | `Telegram/SourceFiles/history/history_unread_things.cpp` | 297 | HistoryUnreadThings, List, All, AddType | Mentions, Reactions, PollVotes BTreeSet MsgId tracking | `crates/gpui_telegram_desktop/src/views/stage/history_unread_things.rs` | `HistoryUnreadThings` | `gpui_ui_1to1` |
| 16 | `Telegram/SourceFiles/history/history_unread_things.h` | 153 | HistoryUnreadThings, List, All, AddType | Mentions, Reactions, PollVotes BTreeSet MsgId tracking | `crates/gpui_telegram_desktop/src/views/stage/history_unread_things.rs` | `HistoryUnreadThings` | `gpui_ui_1to1` |
| 17 | `Telegram/SourceFiles/history/history_view_top_toast.cpp` | 118 | HistoryViewTopToast, InfoTooltip, AnchoredTooltip | kAnchoredTooltipDuration=4s, CountToastDuration clamp 5s..8s | `crates/gpui_telegram_desktop/src/views/stage/history_view_top_toast.rs` | `HistoryViewTopToast` | `gpui_ui_1to1` |
| 18 | `Telegram/SourceFiles/history/history_view_top_toast.h` | 65 | HistoryViewTopToast, InfoTooltip, AnchoredTooltip | kAnchoredTooltipDuration=4s, CountToastDuration clamp 5s..8s | `crates/gpui_telegram_desktop/src/views/stage/history_view_top_toast.rs` | `HistoryViewTopToast` | `gpui_ui_1to1` |
| 19 | `Telegram/SourceFiles/history/history_view_pull_to_next_channel.cpp` | 1117 | HistoryViewPullToNextChannel, PullToNextChannel | kResetReachedOn=0.95, kReverseEase=1.5, kReadyDwell=120ms, FindNextUnreadChannel | `crates/gpui_telegram_desktop/src/views/stage/history_view_pull_to_next_channel.rs` | `HistoryViewPullToNextChannel` | `gpui_ui_1to1` |
| 20 | `Telegram/SourceFiles/history/history_view_pull_to_next_channel.h` | 104 | HistoryViewPullToNextChannel, PullToNextChannel | kResetReachedOn=0.95, kReverseEase=1.5, kReadyDwell=120ms, FindNextUnreadChannel | `crates/gpui_telegram_desktop/src/views/stage/history_view_pull_to_next_channel.rs` | `HistoryViewPullToNextChannel` | `gpui_ui_1to1` |
| 21 | `Telegram/SourceFiles/history/history_view_swipe_back_session.cpp` | 63 | HistoryViewSwipeBackSession, SetupSwipeBackSection | RightToLeft swipe gesture, dontStart on text select, stack pop | `crates/gpui_telegram_desktop/src/views/stage/history_view_swipe_back_session.rs` | `HistoryViewSwipeBackSession` | `gpui_ui_1to1` |
| 22 | `Telegram/SourceFiles/history/history_view_swipe_back_session.h` | 30 | HistoryViewSwipeBackSession, SetupSwipeBackSection | RightToLeft swipe gesture, dontStart on text select, stack pop | `crates/gpui_telegram_desktop/src/views/stage/history_view_swipe_back_session.rs` | `HistoryViewSwipeBackSession` | `gpui_ui_1to1` |
| 23 | `Telegram/SourceFiles/history/history_view_highlight_manager.cpp` | 318 | HistoryViewHighlightManager, ElementHighlighter | activeFadeInDuration=200ms, activeFadeOutDuration=600ms, fadeWrapDuration=300ms | `crates/gpui_telegram_desktop/src/views/stage/history_view_highlight_manager.rs` | `HistoryViewHighlightManager` | `gpui_ui_1to1` |
| 24 | `Telegram/SourceFiles/history/history_view_highlight_manager.h` | 102 | HistoryViewHighlightManager, ElementHighlighter | activeFadeInDuration=200ms, activeFadeOutDuration=600ms, fadeWrapDuration=300ms | `crates/gpui_telegram_desktop/src/views/stage/history_view_highlight_manager.rs` | `HistoryViewHighlightManager` | `gpui_ui_1to1` |
| 25 | `Telegram/SourceFiles/history/history_streamed_drafts.cpp` | 541 | HistoryStreamedDrafts, DraftKind, Draft | kClearTimeout=30s idle purge, apply, adoptIncoming | `crates/gpui_telegram_desktop/src/views/stage/history_streamed_drafts.rs` | `HistoryStreamedDrafts` | `gpui_ui_1to1` |
| 26 | `Telegram/SourceFiles/history/history_streamed_drafts.h` | 111 | HistoryStreamedDrafts, DraftKind, Draft | kClearTimeout=30s idle purge, apply, adoptIncoming | `crates/gpui_telegram_desktop/src/views/stage/history_streamed_drafts.rs` | `HistoryStreamedDrafts` | `gpui_ui_1to1` |
| 27 | `Telegram/SourceFiles/history/history_message_selection.h` | 200 | HistoryMessageSelection, MessageSelectionEndpoint | Flat vs RichPage endpoints, symbol offset calculation | `crates/gpui_telegram_desktop/src/views/stage/history_message_selection.rs` | `HistoryMessageSelection` | `gpui_ui_1to1` |
| 28 | `Telegram/SourceFiles/poll/poll_link_box.cpp` | 69 | PollLinkBox, AddPollOptionLinkBox | Option link URL dialog, trimmed validation | `crates/gpui_telegram_desktop/src/views/stage/poll_link_box.rs` | `PollLinkBox` | `gpui_ui_1to1` |
| 29 | `Telegram/SourceFiles/poll/poll_link_box.h` | 21 | PollLinkBox, AddPollOptionLinkBox | Option link URL dialog, trimmed validation | `crates/gpui_telegram_desktop/src/views/stage/poll_link_box.rs` | `PollLinkBox` | `gpui_ui_1to1` |
| 30 | `Telegram/SourceFiles/poll/poll_link_thumbnail.cpp` | 65 | PollLinkThumbnail, LinkThumbnail | radius=size/4.0 rounded thumb, placeholder icon | `crates/gpui_telegram_desktop/src/views/stage/poll_link_thumbnail.rs` | `PollLinkThumbnail` | `gpui_ui_1to1` |
| 31 | `Telegram/SourceFiles/poll/poll_link_thumbnail.h` | 16 | PollLinkThumbnail, LinkThumbnail | radius=size/4.0 rounded thumb, placeholder icon | `crates/gpui_telegram_desktop/src/views/stage/poll_link_thumbnail.rs` | `PollLinkThumbnail` | `gpui_ui_1to1` |
| 32 | `Telegram/SourceFiles/poll/poll_media_upload.cpp` | 1065 | PollMediaUpload, PollMediaState | Poll media upload lifecycle, radial animation progress | `crates/gpui_telegram_desktop/src/views/stage/poll_media_upload.rs` | `PollMediaUpload` | `gpui_ui_1to1` |
| 33 | `Telegram/SourceFiles/poll/poll_media_upload.h` | 226 | PollMediaUpload, PollMediaState | Poll media upload lifecycle, radial animation progress | `crates/gpui_telegram_desktop/src/views/stage/poll_media_upload.rs` | `PollMediaUpload` | `gpui_ui_1to1` |

---

## 2. Partitioned Source Manifest: Slices 2–4 Enumeration (269 Files)

To ensure complete source-path coverage across `history.json` pursuant to Mandate #19 Pillar 3 (Gate 6.6 zero-omission rule), the remaining 269 upstream files are accounted for below and partitioned into subsequent implementation slices:

### Slice 2: Message Items, Components, and Text Processing (12 Files)
- `Telegram/SourceFiles/history/history_item.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/history_item.rs` (`HistoryItem`)
- `Telegram/SourceFiles/history/history_item.h` -> `crates/gpui_telegram_desktop/src/views/stage/history_item.rs` (`HistoryItem`)
- `Telegram/SourceFiles/history/history_item_components.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/history_item_components.rs` (`HistoryItemComponents`)
- `Telegram/SourceFiles/history/history_item_components.h` -> `crates/gpui_telegram_desktop/src/views/stage/history_item_components.rs` (`HistoryItemComponents`)
- `Telegram/SourceFiles/history/history_item_edition.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/history_item_edition.rs` (`HistoryItemEdition`)
- `Telegram/SourceFiles/history/history_item_edition.h` -> `crates/gpui_telegram_desktop/src/views/stage/history_item_edition.rs` (`HistoryItemEdition`)
- `Telegram/SourceFiles/history/history_item_helpers.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/history_item_helpers.rs` (`HistoryItemHelpers`)
- `Telegram/SourceFiles/history/history_item_helpers.h` -> `crates/gpui_telegram_desktop/src/views/stage/history_item_helpers.rs` (`HistoryItemHelpers`)
- `Telegram/SourceFiles/history/history_item_reply_markup.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/history_item_reply_markup.rs` (`HistoryItemReplyMarkup`)
- `Telegram/SourceFiles/history/history_item_reply_markup.h` -> `crates/gpui_telegram_desktop/src/views/stage/history_item_reply_markup.rs` (`HistoryItemReplyMarkup`)
- `Telegram/SourceFiles/history/history_item_text.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/history_item_text.rs` (`HistoryItemText`)
- `Telegram/SourceFiles/history/history_item_text.h` -> `crates/gpui_telegram_desktop/src/views/stage/history_item_text.rs` (`HistoryItemText`)

### Slice 3: Channel Admin Log Subsystem (9 Files)
- `Telegram/SourceFiles/history/admin_log/history_admin_log_filter.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/admin_log/history_admin_log_filter.rs` (`HistoryAdminLogFilterAdminLog`)
- `Telegram/SourceFiles/history/admin_log/history_admin_log_filter.h` -> `crates/gpui_telegram_desktop/src/views/stage/admin_log/history_admin_log_filter.rs` (`HistoryAdminLogFilterAdminLog`)
- `Telegram/SourceFiles/history/admin_log/history_admin_log_filter_value.h` -> `crates/gpui_telegram_desktop/src/views/stage/admin_log/history_admin_log_filter_value.rs` (`HistoryAdminLogFilterValueAdminLog`)
- `Telegram/SourceFiles/history/admin_log/history_admin_log_inner.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/admin_log/history_admin_log_inner.rs` (`HistoryAdminLogInnerAdminLog`)
- `Telegram/SourceFiles/history/admin_log/history_admin_log_inner.h` -> `crates/gpui_telegram_desktop/src/views/stage/admin_log/history_admin_log_inner.rs` (`HistoryAdminLogInnerAdminLog`)
- `Telegram/SourceFiles/history/admin_log/history_admin_log_item.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/admin_log/history_admin_log_item.rs` (`HistoryAdminLogItemAdminLog`)
- `Telegram/SourceFiles/history/admin_log/history_admin_log_item.h` -> `crates/gpui_telegram_desktop/src/views/stage/admin_log/history_admin_log_item.rs` (`HistoryAdminLogItemAdminLog`)
- `Telegram/SourceFiles/history/admin_log/history_admin_log_section.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/admin_log/history_admin_log_section.rs` (`HistoryAdminLogSectionAdminLog`)
- `Telegram/SourceFiles/history/admin_log/history_admin_log_section.h` -> `crates/gpui_telegram_desktop/src/views/stage/admin_log/history_admin_log_section.rs` (`HistoryAdminLogSectionAdminLog`)

### Slice 4: History View Elements, Media, and Controls (248 Files)
- `Telegram/SourceFiles/history/view/controls/compose_controls_common.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/compose_controls_common.rs` (`ComposeControlsCommonViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_bot_menu_button.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_bot_menu_button.rs` (`HistoryViewBotMenuButtonViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_bot_menu_button.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_bot_menu_button.rs` (`HistoryViewBotMenuButtonViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_bottom_controls.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_bottom_controls.rs` (`HistoryViewBottomControlsViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_bottom_controls.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_bottom_controls.rs` (`HistoryViewBottomControlsViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_characters_limit.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_characters_limit.rs` (`HistoryViewCharactersLimitViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_characters_limit.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_characters_limit.rs` (`HistoryViewCharactersLimitViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_compose_ai_button.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_ai_button.rs` (`HistoryViewComposeAiButtonViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_compose_ai_button.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_ai_button.rs` (`HistoryViewComposeAiButtonViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_compose_ai_tooltip.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_ai_tooltip.rs` (`HistoryViewComposeAiTooltipViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_compose_ai_tooltip.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_ai_tooltip.rs` (`HistoryViewComposeAiTooltipViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_compose_controls.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_controls.rs` (`HistoryViewComposeControlsViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_compose_controls.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_controls.rs` (`HistoryViewComposeControlsViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_compose_media_edit_manager.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_media_edit_manager.rs` (`HistoryViewComposeMediaEditManagerViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_compose_media_edit_manager.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_media_edit_manager.rs` (`HistoryViewComposeMediaEditManagerViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_compose_search.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_search.rs` (`HistoryViewComposeSearchViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_compose_search.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_search.rs` (`HistoryViewComposeSearchViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_compose_stash.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_stash.rs` (`HistoryViewComposeStashViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_compose_stash.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_stash.rs` (`HistoryViewComposeStashViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_compose_stash_hint.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_stash_hint.rs` (`HistoryViewComposeStashHintViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_compose_stash_hint.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_stash_hint.rs` (`HistoryViewComposeStashHintViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_draft_options.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_draft_options.rs` (`HistoryViewDraftOptionsViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_draft_options.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_draft_options.rs` (`HistoryViewDraftOptionsViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_forward_panel.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_forward_panel.rs` (`HistoryViewForwardPanelViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_forward_panel.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_forward_panel.rs` (`HistoryViewForwardPanelViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_rich_draft_preview.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_rich_draft_preview.rs` (`HistoryViewRichDraftPreviewViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_rich_draft_preview.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_rich_draft_preview.rs` (`HistoryViewRichDraftPreviewViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_suggest_options.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_suggest_options.rs` (`HistoryViewSuggestOptionsViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_suggest_options.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_suggest_options.rs` (`HistoryViewSuggestOptionsViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_top_controls.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_top_controls.rs` (`HistoryViewTopControlsViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_top_controls.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_top_controls.rs` (`HistoryViewTopControlsViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_ttl_button.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_ttl_button.rs` (`HistoryViewTtlButtonViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_ttl_button.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_ttl_button.rs` (`HistoryViewTtlButtonViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_video_cover_uploader.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_video_cover_uploader.rs` (`HistoryViewVideoCoverUploaderViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_video_cover_uploader.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_video_cover_uploader.rs` (`HistoryViewVideoCoverUploaderViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_voice_record_bar.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_voice_record_bar.rs` (`HistoryViewVoiceRecordBarViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_voice_record_bar.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_voice_record_bar.rs` (`HistoryViewVoiceRecordBarViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_voice_record_bar.style` -> `crates/gpui_telegram_desktop/src/theme/history/history_view_voice_record_bar_style.rs` (`HistoryViewVoiceRecordBarViewControlsStyleSheet`)
- `Telegram/SourceFiles/history/view/controls/history_view_voice_record_button.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_voice_record_button.rs` (`HistoryViewVoiceRecordButtonViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_voice_record_button.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_voice_record_button.rs` (`HistoryViewVoiceRecordButtonViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_webpage_processor.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_webpage_processor.rs` (`HistoryViewWebpageProcessorViewControls`)
- `Telegram/SourceFiles/history/view/controls/history_view_webpage_processor.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_webpage_processor.rs` (`HistoryViewWebpageProcessorViewControls`)
- `Telegram/SourceFiles/history/view/history_view_about_view.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_about_view.rs` (`HistoryViewAboutViewView`)
- `Telegram/SourceFiles/history/view/history_view_about_view.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_about_view.rs` (`HistoryViewAboutViewView`)
- `Telegram/SourceFiles/history/view/history_view_about_view.style` -> `crates/gpui_telegram_desktop/src/theme/history/history_view_about_view_style.rs` (`HistoryViewAboutViewViewStyleSheet`)
- `Telegram/SourceFiles/history/view/history_view_add_poll_option.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_add_poll_option.rs` (`HistoryViewAddPollOptionView`)
- `Telegram/SourceFiles/history/view/history_view_add_poll_option.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_add_poll_option.rs` (`HistoryViewAddPollOptionView`)
- `Telegram/SourceFiles/history/view/history_view_bottom_info.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_bottom_info.rs` (`HistoryViewBottomInfoView`)
- `Telegram/SourceFiles/history/view/history_view_bottom_info.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_bottom_info.rs` (`HistoryViewBottomInfoView`)
- `Telegram/SourceFiles/history/view/history_view_chat_preview.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_chat_preview.rs` (`HistoryViewChatPreviewView`)
- `Telegram/SourceFiles/history/view/history_view_chat_preview.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_chat_preview.rs` (`HistoryViewChatPreviewView`)
- `Telegram/SourceFiles/history/view/history_view_chat_section.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_chat_section.rs` (`HistoryViewChatSectionView`)
- `Telegram/SourceFiles/history/view/history_view_chat_section.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_chat_section.rs` (`HistoryViewChatSectionView`)
- `Telegram/SourceFiles/history/view/history_view_contact_status.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_contact_status.rs` (`HistoryViewContactStatusView`)
- `Telegram/SourceFiles/history/view/history_view_contact_status.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_contact_status.rs` (`HistoryViewContactStatusView`)
- `Telegram/SourceFiles/history/view/history_view_context_menu.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_context_menu.rs` (`HistoryViewContextMenuView`)
- `Telegram/SourceFiles/history/view/history_view_context_menu.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_context_menu.rs` (`HistoryViewContextMenuView`)
- `Telegram/SourceFiles/history/view/history_view_copy_selection.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_copy_selection.rs` (`HistoryViewCopySelectionView`)
- `Telegram/SourceFiles/history/view/history_view_copy_selection.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_copy_selection.rs` (`HistoryViewCopySelectionView`)
- `Telegram/SourceFiles/history/view/history_view_corner_buttons.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_corner_buttons.rs` (`HistoryViewCornerButtonsView`)
- `Telegram/SourceFiles/history/view/history_view_corner_buttons.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_corner_buttons.rs` (`HistoryViewCornerButtonsView`)
- `Telegram/SourceFiles/history/view/history_view_cursor_state.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_cursor_state.rs` (`HistoryViewCursorStateView`)
- `Telegram/SourceFiles/history/view/history_view_cursor_state.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_cursor_state.rs` (`HistoryViewCursorStateView`)
- `Telegram/SourceFiles/history/view/history_view_drag.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_drag.rs` (`HistoryViewDragView`)
- `Telegram/SourceFiles/history/view/history_view_drag.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_drag.rs` (`HistoryViewDragView`)
- `Telegram/SourceFiles/history/view/history_view_draw_to_reply.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_draw_to_reply.rs` (`HistoryViewDrawToReplyView`)
- `Telegram/SourceFiles/history/view/history_view_draw_to_reply.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_draw_to_reply.rs` (`HistoryViewDrawToReplyView`)
- `Telegram/SourceFiles/history/view/history_view_element.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_element.rs` (`HistoryViewElementView`)
- `Telegram/SourceFiles/history/view/history_view_element.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_element.rs` (`HistoryViewElementView`)
- `Telegram/SourceFiles/history/view/history_view_element_overlay.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_element_overlay.rs` (`HistoryViewElementOverlayView`)
- `Telegram/SourceFiles/history/view/history_view_element_overlay.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_element_overlay.rs` (`HistoryViewElementOverlayView`)
- `Telegram/SourceFiles/history/view/history_view_emoji_interactions.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_emoji_interactions.rs` (`HistoryViewEmojiInteractionsView`)
- `Telegram/SourceFiles/history/view/history_view_emoji_interactions.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_emoji_interactions.rs` (`HistoryViewEmojiInteractionsView`)
- `Telegram/SourceFiles/history/view/history_view_empty_list_bubble.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_empty_list_bubble.rs` (`HistoryViewEmptyListBubbleView`)
- `Telegram/SourceFiles/history/view/history_view_empty_list_bubble.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_empty_list_bubble.rs` (`HistoryViewEmptyListBubbleView`)
- `Telegram/SourceFiles/history/view/history_view_fake_items.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_fake_items.rs` (`HistoryViewFakeItemsView`)
- `Telegram/SourceFiles/history/view/history_view_fake_items.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_fake_items.rs` (`HistoryViewFakeItemsView`)
- `Telegram/SourceFiles/history/view/history_view_group_call_bar.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_group_call_bar.rs` (`HistoryViewGroupCallBarView`)
- `Telegram/SourceFiles/history/view/history_view_group_call_bar.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_group_call_bar.rs` (`HistoryViewGroupCallBarView`)
- `Telegram/SourceFiles/history/view/history_view_group_members_widget.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_group_members_widget.rs` (`HistoryViewGroupMembersWidgetView`)
- `Telegram/SourceFiles/history/view/history_view_group_members_widget.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_group_members_widget.rs` (`HistoryViewGroupMembersWidgetView`)
- `Telegram/SourceFiles/history/view/history_view_item_preview.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_item_preview.rs` (`HistoryViewItemPreviewView`)
- `Telegram/SourceFiles/history/view/history_view_keyboard_text_selection.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_keyboard_text_selection.rs` (`HistoryViewKeyboardTextSelectionView`)
- `Telegram/SourceFiles/history/view/history_view_keyboard_text_selection.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_keyboard_text_selection.rs` (`HistoryViewKeyboardTextSelectionView`)
- `Telegram/SourceFiles/history/view/history_view_list_widget.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_list_widget.rs` (`HistoryViewListWidgetView`)
- `Telegram/SourceFiles/history/view/history_view_list_widget.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_list_widget.rs` (`HistoryViewListWidgetView`)
- `Telegram/SourceFiles/history/view/history_view_message.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_message.rs` (`HistoryViewMessageView`)
- `Telegram/SourceFiles/history/view/history_view_message.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_message.rs` (`HistoryViewMessageView`)
- `Telegram/SourceFiles/history/view/history_view_object.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_object.rs` (`HistoryViewObjectView`)
- `Telegram/SourceFiles/history/view/history_view_paid_reaction_toast.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_paid_reaction_toast.rs` (`HistoryViewPaidReactionToastView`)
- `Telegram/SourceFiles/history/view/history_view_paid_reaction_toast.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_paid_reaction_toast.rs` (`HistoryViewPaidReactionToastView`)
- `Telegram/SourceFiles/history/view/history_view_pinned_bar.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_pinned_bar.rs` (`HistoryViewPinnedBarView`)
- `Telegram/SourceFiles/history/view/history_view_pinned_bar.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_pinned_bar.rs` (`HistoryViewPinnedBarView`)
- `Telegram/SourceFiles/history/view/history_view_pinned_section.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_pinned_section.rs` (`HistoryViewPinnedSectionView`)
- `Telegram/SourceFiles/history/view/history_view_pinned_section.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_pinned_section.rs` (`HistoryViewPinnedSectionView`)
- `Telegram/SourceFiles/history/view/history_view_pinned_tracker.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_pinned_tracker.rs` (`HistoryViewPinnedTrackerView`)
- `Telegram/SourceFiles/history/view/history_view_pinned_tracker.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_pinned_tracker.rs` (`HistoryViewPinnedTrackerView`)
- `Telegram/SourceFiles/history/view/history_view_quick_action.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_quick_action.rs` (`HistoryViewQuickActionView`)
- `Telegram/SourceFiles/history/view/history_view_quick_action.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_quick_action.rs` (`HistoryViewQuickActionView`)
- `Telegram/SourceFiles/history/view/history_view_reaction_preview.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_reaction_preview.rs` (`HistoryViewReactionPreviewView`)
- `Telegram/SourceFiles/history/view/history_view_reaction_preview.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_reaction_preview.rs` (`HistoryViewReactionPreviewView`)
- `Telegram/SourceFiles/history/view/history_view_read_metrics_tracker.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_read_metrics_tracker.rs` (`HistoryViewReadMetricsTrackerView`)
- `Telegram/SourceFiles/history/view/history_view_read_metrics_tracker.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_read_metrics_tracker.rs` (`HistoryViewReadMetricsTrackerView`)
- `Telegram/SourceFiles/history/view/history_view_reply.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_reply.rs` (`HistoryViewReplyView`)
- `Telegram/SourceFiles/history/view/history_view_reply.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_reply.rs` (`HistoryViewReplyView`)
- `Telegram/SourceFiles/history/view/history_view_reply_button.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_reply_button.rs` (`HistoryViewReplyButtonView`)
- `Telegram/SourceFiles/history/view/history_view_reply_button.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_reply_button.rs` (`HistoryViewReplyButtonView`)
- `Telegram/SourceFiles/history/view/history_view_requests_bar.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_requests_bar.rs` (`HistoryViewRequestsBarView`)
- `Telegram/SourceFiles/history/view/history_view_requests_bar.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_requests_bar.rs` (`HistoryViewRequestsBarView`)
- `Telegram/SourceFiles/history/view/history_view_rich_quote.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_rich_quote.rs` (`HistoryViewRichQuoteView`)
- `Telegram/SourceFiles/history/view/history_view_rich_quote.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_rich_quote.rs` (`HistoryViewRichQuoteView`)
- `Telegram/SourceFiles/history/view/history_view_schedule_box.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_schedule_box.rs` (`HistoryViewScheduleBoxView`)
- `Telegram/SourceFiles/history/view/history_view_schedule_box.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_schedule_box.rs` (`HistoryViewScheduleBoxView`)
- `Telegram/SourceFiles/history/view/history_view_scheduled_section.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_scheduled_section.rs` (`HistoryViewScheduledSectionView`)
- `Telegram/SourceFiles/history/view/history_view_scheduled_section.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_scheduled_section.rs` (`HistoryViewScheduledSectionView`)
- `Telegram/SourceFiles/history/view/history_view_self_forwards_tagger.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_self_forwards_tagger.rs` (`HistoryViewSelfForwardsTaggerView`)
- `Telegram/SourceFiles/history/view/history_view_self_forwards_tagger.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_self_forwards_tagger.rs` (`HistoryViewSelfForwardsTaggerView`)
- `Telegram/SourceFiles/history/view/history_view_send_action.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_send_action.rs` (`HistoryViewSendActionView`)
- `Telegram/SourceFiles/history/view/history_view_send_action.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_send_action.rs` (`HistoryViewSendActionView`)
- `Telegram/SourceFiles/history/view/history_view_service_message.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_service_message.rs` (`HistoryViewServiceMessageView`)
- `Telegram/SourceFiles/history/view/history_view_service_message.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_service_message.rs` (`HistoryViewServiceMessageView`)
- `Telegram/SourceFiles/history/view/history_view_sponsored_click_handler.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_sponsored_click_handler.rs` (`HistoryViewSponsoredClickHandlerView`)
- `Telegram/SourceFiles/history/view/history_view_sponsored_click_handler.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_sponsored_click_handler.rs` (`HistoryViewSponsoredClickHandlerView`)
- `Telegram/SourceFiles/history/view/history_view_sticker_toast.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_sticker_toast.rs` (`HistoryViewStickerToastView`)
- `Telegram/SourceFiles/history/view/history_view_sticker_toast.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_sticker_toast.rs` (`HistoryViewStickerToastView`)
- `Telegram/SourceFiles/history/view/history_view_subsection_tabs.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_subsection_tabs.rs` (`HistoryViewSubsectionTabsView`)
- `Telegram/SourceFiles/history/view/history_view_subsection_tabs.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_subsection_tabs.rs` (`HistoryViewSubsectionTabsView`)
- `Telegram/SourceFiles/history/view/history_view_summary_header.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_summary_header.rs` (`HistoryViewSummaryHeaderView`)
- `Telegram/SourceFiles/history/view/history_view_summary_header.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_summary_header.rs` (`HistoryViewSummaryHeaderView`)
- `Telegram/SourceFiles/history/view/history_view_text_helper.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_text_helper.rs` (`HistoryViewTextHelperView`)
- `Telegram/SourceFiles/history/view/history_view_text_helper.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_text_helper.rs` (`HistoryViewTextHelperView`)
- `Telegram/SourceFiles/history/view/history_view_top_bar_widget.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_top_bar_widget.rs` (`HistoryViewTopBarWidgetView`)
- `Telegram/SourceFiles/history/view/history_view_top_bar_widget.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_top_bar_widget.rs` (`HistoryViewTopBarWidgetView`)
- `Telegram/SourceFiles/history/view/history_view_top_peers_selector.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_top_peers_selector.rs` (`HistoryViewTopPeersSelectorView`)
- `Telegram/SourceFiles/history/view/history_view_top_peers_selector.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_top_peers_selector.rs` (`HistoryViewTopPeersSelectorView`)
- `Telegram/SourceFiles/history/view/history_view_touch_scroll.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_touch_scroll.rs` (`HistoryViewTouchScrollView`)
- `Telegram/SourceFiles/history/view/history_view_touch_scroll.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_touch_scroll.rs` (`HistoryViewTouchScrollView`)
- `Telegram/SourceFiles/history/view/history_view_transcribe_button.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_transcribe_button.rs` (`HistoryViewTranscribeButtonView`)
- `Telegram/SourceFiles/history/view/history_view_transcribe_button.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_transcribe_button.rs` (`HistoryViewTranscribeButtonView`)
- `Telegram/SourceFiles/history/view/history_view_translate_bar.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_translate_bar.rs` (`HistoryViewTranslateBarView`)
- `Telegram/SourceFiles/history/view/history_view_translate_bar.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_translate_bar.rs` (`HistoryViewTranslateBarView`)
- `Telegram/SourceFiles/history/view/history_view_translate_tracker.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_translate_tracker.rs` (`HistoryViewTranslateTrackerView`)
- `Telegram/SourceFiles/history/view/history_view_translate_tracker.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_translate_tracker.rs` (`HistoryViewTranslateTrackerView`)
- `Telegram/SourceFiles/history/view/history_view_view_button.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_view_button.rs` (`HistoryViewViewButtonView`)
- `Telegram/SourceFiles/history/view/history_view_view_button.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_view_button.rs` (`HistoryViewViewButtonView`)
- `Telegram/SourceFiles/history/view/history_view_webpage_preview.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_webpage_preview.rs` (`HistoryViewWebpagePreviewView`)
- `Telegram/SourceFiles/history/view/history_view_webpage_preview.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_webpage_preview.rs` (`HistoryViewWebpagePreviewView`)
- `Telegram/SourceFiles/history/view/history_view_welcome_messages_section.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_welcome_messages_section.rs` (`HistoryViewWelcomeMessagesSectionView`)
- `Telegram/SourceFiles/history/view/history_view_welcome_messages_section.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/history_view_welcome_messages_section.rs` (`HistoryViewWelcomeMessagesSectionView`)
- `Telegram/SourceFiles/history/view/media/history_view_birthday_suggestion.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_birthday_suggestion.rs` (`HistoryViewBirthdaySuggestionViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_birthday_suggestion.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_birthday_suggestion.rs` (`HistoryViewBirthdaySuggestionViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_call.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_call.rs` (`HistoryViewCallViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_call.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_call.rs` (`HistoryViewCallViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_community_added.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_community_added.rs` (`HistoryViewCommunityAddedViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_community_added.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_community_added.rs` (`HistoryViewCommunityAddedViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_contact.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_contact.rs` (`HistoryViewContactViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_contact.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_contact.rs` (`HistoryViewContactViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_custom_emoji.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_custom_emoji.rs` (`HistoryViewCustomEmojiViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_custom_emoji.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_custom_emoji.rs` (`HistoryViewCustomEmojiViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_dice.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_dice.rs` (`HistoryViewDiceViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_dice.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_dice.rs` (`HistoryViewDiceViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_document.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_document.rs` (`HistoryViewDocumentViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_document.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_document.rs` (`HistoryViewDocumentViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_ephemeral_plate.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_ephemeral_plate.rs` (`HistoryViewEphemeralPlateViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_ephemeral_plate.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_ephemeral_plate.rs` (`HistoryViewEphemeralPlateViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_file.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_file.rs` (`HistoryViewFileViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_file.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_file.rs` (`HistoryViewFileViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_game.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_game.rs` (`HistoryViewGameViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_game.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_game.rs` (`HistoryViewGameViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_gif.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_gif.rs` (`HistoryViewGifViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_gif.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_gif.rs` (`HistoryViewGifViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_giveaway.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_giveaway.rs` (`HistoryViewGiveawayViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_giveaway.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_giveaway.rs` (`HistoryViewGiveawayViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_gram_transfer.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_gram_transfer.rs` (`HistoryViewGramTransferViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_gram_transfer.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_gram_transfer.rs` (`HistoryViewGramTransferViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_invoice.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_invoice.rs` (`HistoryViewInvoiceViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_invoice.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_invoice.rs` (`HistoryViewInvoiceViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_large_emoji.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_large_emoji.rs` (`HistoryViewLargeEmojiViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_large_emoji.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_large_emoji.rs` (`HistoryViewLargeEmojiViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_location.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_location.rs` (`HistoryViewLocationViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_location.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_location.rs` (`HistoryViewLocationViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_media.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media.rs` (`HistoryViewMediaViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_media.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media.rs` (`HistoryViewMediaViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_media_common.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_common.rs` (`HistoryViewMediaCommonViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_media_common.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_common.rs` (`HistoryViewMediaCommonViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_media_generic.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_generic.rs` (`HistoryViewMediaGenericViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_media_generic.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_generic.rs` (`HistoryViewMediaGenericViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_media_grouped.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_grouped.rs` (`HistoryViewMediaGroupedViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_media_grouped.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_grouped.rs` (`HistoryViewMediaGroupedViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_media_spoiler.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_spoiler.rs` (`HistoryViewMediaSpoilerViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_media_spoiler.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_spoiler.rs` (`HistoryViewMediaSpoilerViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_media_unwrapped.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_unwrapped.rs` (`HistoryViewMediaUnwrappedViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_media_unwrapped.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_unwrapped.rs` (`HistoryViewMediaUnwrappedViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_no_forwards_request.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_no_forwards_request.rs` (`HistoryViewNoForwardsRequestViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_no_forwards_request.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_no_forwards_request.rs` (`HistoryViewNoForwardsRequestViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_photo.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_photo.rs` (`HistoryViewPhotoViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_photo.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_photo.rs` (`HistoryViewPhotoViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_poll.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_poll.rs` (`HistoryViewPollViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_poll.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_poll.rs` (`HistoryViewPollViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_premium_gift.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_premium_gift.rs` (`HistoryViewPremiumGiftViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_premium_gift.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_premium_gift.rs` (`HistoryViewPremiumGiftViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_save_document_action.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_save_document_action.rs` (`HistoryViewSaveDocumentActionViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_save_document_action.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_save_document_action.rs` (`HistoryViewSaveDocumentActionViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_service_box.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_service_box.rs` (`HistoryViewServiceBoxViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_service_box.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_service_box.rs` (`HistoryViewServiceBoxViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_similar_channels.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_similar_channels.rs` (`HistoryViewSimilarChannelsViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_similar_channels.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_similar_channels.rs` (`HistoryViewSimilarChannelsViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_slot_machine.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_slot_machine.rs` (`HistoryViewSlotMachineViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_slot_machine.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_slot_machine.rs` (`HistoryViewSlotMachineViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_sticker.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_sticker.rs` (`HistoryViewStickerViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_sticker.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_sticker.rs` (`HistoryViewStickerViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_sticker_player.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_sticker_player.rs` (`HistoryViewStickerPlayerViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_sticker_player.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_sticker_player.rs` (`HistoryViewStickerPlayerViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_sticker_player_abstract.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_sticker_player_abstract.rs` (`HistoryViewStickerPlayerAbstractViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_story_mention.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_story_mention.rs` (`HistoryViewStoryMentionViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_story_mention.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_story_mention.rs` (`HistoryViewStoryMentionViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_suggest_decision.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_suggest_decision.rs` (`HistoryViewSuggestDecisionViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_suggest_decision.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_suggest_decision.rs` (`HistoryViewSuggestDecisionViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_theme_document.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_theme_document.rs` (`HistoryViewThemeDocumentViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_theme_document.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_theme_document.rs` (`HistoryViewThemeDocumentViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_todo_list.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_todo_list.rs` (`HistoryViewTodoListViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_todo_list.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_todo_list.rs` (`HistoryViewTodoListViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_unique_gift.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_unique_gift.rs` (`HistoryViewUniqueGiftViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_unique_gift.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_unique_gift.rs` (`HistoryViewUniqueGiftViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_unsupported_notice.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_unsupported_notice.rs` (`HistoryViewUnsupportedNoticeViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_unsupported_notice.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_unsupported_notice.rs` (`HistoryViewUnsupportedNoticeViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_userpic_suggestion.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_userpic_suggestion.rs` (`HistoryViewUserpicSuggestionViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_userpic_suggestion.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_userpic_suggestion.rs` (`HistoryViewUserpicSuggestionViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_video_message_seek.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_video_message_seek.rs` (`HistoryViewVideoMessageSeekViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_video_message_seek.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_video_message_seek.rs` (`HistoryViewVideoMessageSeekViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_video_status.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_video_status.rs` (`HistoryViewVideoStatusViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_video_status.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_video_status.rs` (`HistoryViewVideoStatusViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_web_page.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_web_page.rs` (`HistoryViewWebPageViewMedia`)
- `Telegram/SourceFiles/history/view/media/history_view_web_page.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_web_page.rs` (`HistoryViewWebPageViewMedia`)
- `Telegram/SourceFiles/history/view/media/menu/history_view_poll_menu.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/menu/history_view_poll_menu.rs` (`HistoryViewPollMenuViewMediaMenu`)
- `Telegram/SourceFiles/history/view/media/menu/history_view_poll_menu.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/media/menu/history_view_poll_menu.rs` (`HistoryViewPollMenuViewMediaMenu`)
- `Telegram/SourceFiles/history/view/reactions/history_view_reactions.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/reactions/history_view_reactions.rs` (`HistoryViewReactionsViewReactions`)
- `Telegram/SourceFiles/history/view/reactions/history_view_reactions.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/reactions/history_view_reactions.rs` (`HistoryViewReactionsViewReactions`)
- `Telegram/SourceFiles/history/view/reactions/history_view_reactions_button.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/reactions/history_view_reactions_button.rs` (`HistoryViewReactionsButtonViewReactions`)
- `Telegram/SourceFiles/history/view/reactions/history_view_reactions_button.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/reactions/history_view_reactions_button.rs` (`HistoryViewReactionsButtonViewReactions`)
- `Telegram/SourceFiles/history/view/reactions/history_view_reactions_list.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/reactions/history_view_reactions_list.rs` (`HistoryViewReactionsListViewReactions`)
- `Telegram/SourceFiles/history/view/reactions/history_view_reactions_list.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/reactions/history_view_reactions_list.rs` (`HistoryViewReactionsListViewReactions`)
- `Telegram/SourceFiles/history/view/reactions/history_view_reactions_selector.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/reactions/history_view_reactions_selector.rs` (`HistoryViewReactionsSelectorViewReactions`)
- `Telegram/SourceFiles/history/view/reactions/history_view_reactions_selector.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/reactions/history_view_reactions_selector.rs` (`HistoryViewReactionsSelectorViewReactions`)
- `Telegram/SourceFiles/history/view/reactions/history_view_reactions_strip.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/reactions/history_view_reactions_strip.rs` (`HistoryViewReactionsStripViewReactions`)
- `Telegram/SourceFiles/history/view/reactions/history_view_reactions_strip.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/reactions/history_view_reactions_strip.rs` (`HistoryViewReactionsStripViewReactions`)
- `Telegram/SourceFiles/history/view/reactions/history_view_reactions_tabs.cpp` -> `crates/gpui_telegram_desktop/src/views/stage/view/reactions/history_view_reactions_tabs.rs` (`HistoryViewReactionsTabsViewReactions`)
- `Telegram/SourceFiles/history/view/reactions/history_view_reactions_tabs.h` -> `crates/gpui_telegram_desktop/src/views/stage/view/reactions/history_view_reactions_tabs.rs` (`HistoryViewReactionsTabsViewReactions`)

---

## Verification & Acceptance Plan
- Unit tests: `history_core_subsystem_tests.rs` covering state machines, bounds calculations, unread lists, toast durations, pull-to-channel easing, selection offsets, and drag target metrics.
- Gate 6.2 Burndown: Boilerplate stubs in the 17 assigned Slice 1 target files drop from 17 to 0.
- Gate 6.6 Zero-Omission: 302/302 upstream source files from `history.json` accounted for (100% coverage).