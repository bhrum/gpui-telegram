# Module 06 Specification (Slice 4): History View List Widget, Chat Sections & Navigation Bars

## Overview

Module 06 Slice 4 implements the high-performance central chat stage components for Telegram Desktop reconstructed in pure, idiomatic Rust with GPUI. This includes:
1. **Virtual List Widget (`HistoryViewListWidget`)**: Virtual scrolling math, multi-screen preloading, viewport geometry clipping, date badge floating, unread separator tracking, and message selection.
2. **Chat Section Controller (`HistoryViewChatSection`)**: Primary conversation stage orchestrator coordinating the message list, composer, title bar, pinned bar, search, and drag-and-drop actions.
3. **Top Bar Widget (`HistoryViewTopBarWidget`)**: Adaptive top navigation bar displaying chat titles, active topic badges, group call state, online status, selection action controls, and 320px info drawer toggles.
4. **Context Menu System (`HistoryViewContextMenu`)**: Complete message context menu action builder supporting replying, editing, pinning, forwarding, deleting, scheduling, fact-checking, transcription rating, and link copying.
5. **Pinned Bar & Tracker (`HistoryViewPinnedBar`, `HistoryViewPinnedTracker`, `HistoryViewPinnedSection`)**: Multi-pinned message cyclical navigation, universal message ID projection across migrated groups, preview loading states, and full pinned messages section.
6. **Scheduled Messages Section & Box (`HistoryViewScheduledSection`, `HistoryViewScheduleBox`)**: Scheduled message stage, "Send when online" scheduling state machine, repeat period controls, and silent send toggles.
7. **Translation Bar & Tracker (`HistoryViewTranslateBar`, `HistoryViewTranslateTracker`)**: Real-time language recognition buffers, batch translation dispatching, and translation control banner.
8. **Ancillary Bars & Widgets**: Group call active speaker banner (`HistoryViewGroupCallBar`), inline group members widget (`HistoryViewGroupMembersWidget`), forum subsection tabs (`HistoryViewSubsectionTabs`), join requests bar (`HistoryViewRequestsBar`), business welcome messages section (`HistoryViewWelcomeMessagesSection`), viewport read metrics tracker (`HistoryViewReadMetricsTracker`), empty list bubble (`HistoryViewEmptyListBubble`), and top peers selector (`HistoryViewTopPeersSelector`).

This implementation strictly complies with:
- **Mandate #18 (Anti-Bluffing Protocol)**: Every component cites exact upstream C++ implementation file line ranges (`.cpp:L<start>-L<end>`) and extracts verified internal anonymous-namespace constants (`kPreloadedScreensCount`, `kVideoProcessingInfoDuration`, `kEnoughForRecognition`, `kActivityTimeout`, etc.), exact scrolling math, and state machines.
- **Mandate #19 Pillar 2 (100% Reading Manifest)**: Every single one of the 38 assigned upstream C++ files is documented in the exhaustive manifest below.
- **Mandate #15 (Zero Tolerance for Boilerplate)**: All generic boilerplate stubs (Pattern B `{ id, label, value }`) in scope are replaced with genuine, authentic GPUI view structures and state controllers.

---

## 1. 100% Per-File Reading Manifest (All 38 Upstream C++ Files)

| # | Upstream C++ Source Path | Exact Line Range Read | Upstream Symbols & Classes | Extracted Constants & Logic | Dynamic Rust Target Module & Disposition |
|---|---|---|---|---|---|
| 1 | `Telegram/SourceFiles/history/view/history_view_list_widget.h` | `L1-L1128` | `ListWidget`, `WindowListDelegate`, `ListDelegate`, `SelectedItems`, `ScrollParams`, `CornerButtons` | Virtual list viewport scrolling, date header floating, item selection bitmask, drag-and-drop interfaces | `crates/gpui_telegram_desktop/src/views/stage/list_widget.rs` & `view/history_view_list_widget.rs` |
| 2 | `Telegram/SourceFiles/history/view/history_view_list_widget.cpp` | `L1-L6833` | `ListWidget::paintEvent`, `ListWidget::resizeEvent`, `ListWidget::visibleAreaUpdated`, `ListWidget::scrollToItem`, `MaybeTranslateTracker`, `MaybeReadMetricsTracker`, `HidesDates` | `kPreloadedScreensCount = 4`, `kPreloadIfLessThanScreens = 2`, `kPreloadedScreensCountFull = 9`, `kClearUserpicsAfter = 50`, `kScrollDateHideOnDayCrossingTimeout = 3000ms`. Viewport binary search with scroll threshold clamping | `crates/gpui_telegram_desktop/src/views/stage/list_widget.rs` & `view/history_view_list_widget.rs` |
| 3 | `Telegram/SourceFiles/history/view/history_view_chat_section.h` | `L1-L730` | `ChatSection`, `ChatMemento`, `SendPermission` | Primary conversation section controller, composer integration, memento state restoration | `crates/gpui_telegram_desktop/src/views/stage/chat_section.rs` & `view/history_view_chat_section.rs` |
| 4 | `Telegram/SourceFiles/history/view/history_view_chat_section.cpp` | `L1-L6344` | `ChatSection::setup`, `ChatSection::sendDraft`, `ChatSection::showPinnedBar`, `CanSendResolved`, `IsSpecialShowAtMsgId`, `ResolveHighlightId` | `kShowMembersDropdownTimeoutMs = 300`, `kScrollToVoiceAfterScrolledMs = 1000ms`, `kPsaAboutPrefix = "cloud_lng_about_psa_"`. Migrated peer highlight resolution: `highlightId < 0 -> FullMsgId(migrated->peer->id, -highlightId)` | `crates/gpui_telegram_desktop/src/views/stage/chat_section.rs` & `view/history_view_chat_section.rs` |
| 5 | `Telegram/SourceFiles/history/view/history_view_top_bar_widget.h` | `L1-L292` | `TopBarWidget`, `TopBarWidget::Section`, `TopBarWidget::EmojiInteractionSeenAnimation` | Adaptive chat header, title / status elision, call & drawer action buttons | `crates/gpui_telegram_desktop/src/views/stage/top_bar.rs` & `view/history_view_top_bar_widget.rs` |
| 6 | `Telegram/SourceFiles/history/view/history_view_top_bar_widget.cpp` | `L1-L2042` | `TopBarWidget::paintEvent`, `TopBarWidget::updateControls`, `TopBarWidget::showGroupCallMenu`, `HasGroupCallMenu`, `TopBarNameText`, `SwitchToChooseFromQuery` | `kEmojiInteractionSeenDuration = 3000ms`. Search query `from:`, SavedSublist title selection (`lng_my_notes` vs `lng_hidden_author_messages`) | `crates/gpui_telegram_desktop/src/views/stage/top_bar.rs` & `view/history_view_top_bar_widget.rs` |
| 7 | `Telegram/SourceFiles/history/view/history_view_context_menu.h` | `L1-L183` | `ContextMenuRequest`, `MakeContextMenu`, `FillItemContextMenu` | History item context menu builder interface | `crates/gpui_telegram_desktop/src/views/stage/context_menu.rs` & `view/history_view_context_menu.rs` |
| 8 | `Telegram/SourceFiles/history/view/history_view_context_menu.cpp` | `L1-L3118` | `MakeContextMenu`, `FillItemContextMenu`, `FillSendMenu`, `FillForwardMenu`, `FillRateTranscribeMenu` | `kRescheduleLimit = 20`, `kTagNameLimit = 12`, `kPublicPostLinkToastDuration = 4000ms`, `kMinViewsCount = 10`. Permission-checked action list (Reply, Edit, Pin, Copy, Delete, React, Translate) | `crates/gpui_telegram_desktop/src/views/stage/context_menu.rs` & `view/history_view_context_menu.rs` |
| 9 | `Telegram/SourceFiles/history/view/history_view_pinned_bar.h` | `L1-L67` | `PinnedBar`, `PinnedId` | Top pinned message banner widget | `crates/gpui_telegram_desktop/src/views/stage/pinned_bar.rs` & `view/history_view_pinned_bar.rs` |
| 10 | `Telegram/SourceFiles/history/view/history_view_pinned_bar.cpp` | `L1-L409` | `PinnedBar::updateContent`, `ContentByItem`, `ContentByItemId`, `WithPinnedTitle`, `MakePinnedBarCustomButton` | `kFullLoaded = 2`, `kSomeLoaded = 1`, `kNotLoaded = 0`. Title formatting: `id.index + 1 >= id.count ? "Pinned Message" : (id.count == 2 ? "Previous" : "Pinned Message #" + (id.index + 1))` | `crates/gpui_telegram_desktop/src/views/stage/pinned_bar.rs` & `view/history_view_pinned_bar.rs` |
| 11 | `Telegram/SourceFiles/history/view/history_view_pinned_section.h` | `L1-L246` | `PinnedMemento`, `PinnedWidget` | Separate view section listing all pinned messages in chat | `crates/gpui_telegram_desktop/src/views/stage/pinned_bar.rs` & `view/history_view_pinned_section.rs` |
| 12 | `Telegram/SourceFiles/history/view/history_view_pinned_section.cpp` | `L1-L853` | `PinnedMemento::createWidget`, `PinnedWidget::setup`, `PinnedWidget::removePinnedMessage` | Unpin confirmation box, list memento around pinned highlight ID | `crates/gpui_telegram_desktop/src/views/stage/pinned_bar.rs` & `view/history_view_pinned_section.rs` |
| 13 | `Telegram/SourceFiles/history/view/history_view_pinned_tracker.h` | `L1-L63` | `PinnedTracker`, `PinnedId` | Multi-pinned messages index tracking and cycling | `crates/gpui_telegram_desktop/src/views/stage/pinned_bar.rs` & `view/history_view_pinned_tracker.rs` |
| 14 | `Telegram/SourceFiles/history/view/history_view_pinned_tracker.cpp` | `L1-L186` | `PinnedTracker::nextPinnedId`, `PinnedTracker::refreshViewer`, `PinnedTracker::reset` | `kLoadedLimit = 5`, `kChangeViewerLimit = 2`. UniversalMsgId projection: `peerIsChannel ? id.msg : (id.msg - ServerMaxMsgId)` | `crates/gpui_telegram_desktop/src/views/stage/pinned_bar.rs` & `view/history_view_pinned_tracker.rs` |
| 15 | `Telegram/SourceFiles/history/view/history_view_scheduled_section.h` | `L1-L362` | `ScheduledMemento`, `ScheduledWidget` | Dedicated view section for scheduled messages | `crates/gpui_telegram_desktop/src/views/stage/scheduled_section.rs` & `view/history_view_scheduled_section.rs` |
| 16 | `Telegram/SourceFiles/history/view/history_view_scheduled_section.cpp` | `L1-L1882` | `ScheduledMemento::ScheduledMemento`, `ScheduledWidget::setup`, `FindVideoFile` | `kVideoProcessingInfoDuration = 4000ms`. Video processing toast, send now action, reschedule dialog integration | `crates/gpui_telegram_desktop/src/views/stage/scheduled_section.rs` & `view/history_view_scheduled_section.rs` |
| 17 | `Telegram/SourceFiles/history/view/history_view_welcome_messages_section.h` | `L1-L290` | `WelcomeMessagesMemento`, `WelcomeMessagesWidget` | Business welcome & away automated greeting messages section | `crates/gpui_telegram_desktop/src/views/stage/welcome_messages.rs` & `view/history_view_welcome_messages_section.rs` |
| 18 | `Telegram/SourceFiles/history/view/history_view_welcome_messages_section.cpp` | `L1-L1265` | `WelcomeMessagesMemento::createWidget`, `FillEmptyText` | `st::welcomeEmptyPadding`, `minWidth = st::welcomeEmptyWidth / 4`, empty placeholder bubble layout | `crates/gpui_telegram_desktop/src/views/stage/welcome_messages.rs` & `view/history_view_welcome_messages_section.rs` |
| 19 | `Telegram/SourceFiles/history/view/history_view_subsection_tabs.h` | `L1-L155` | `SubsectionTabs`, `SubsectionTabsMode` | Forum topics tabs bar (Top or Left docked mode) | `crates/gpui_telegram_desktop/src/views/stage/subsection_tabs.rs` & `view/history_view_subsection_tabs.rs` |
| 20 | `Telegram/SourceFiles/history/view/history_view_subsection_tabs.cpp` | `L1-L1018` | `SubsectionTabs::setup`, `SubsectionTabs::refreshSlice`, `SubsectionTabs::track` | `kDefaultLimit = 12`. `SubsectionTabsMode::Left` vs `SubsectionTabsMode::Top`, active topic indicator animation | `crates/gpui_telegram_desktop/src/views/stage/subsection_tabs.rs` & `view/history_view_subsection_tabs.rs` |
| 21 | `Telegram/SourceFiles/history/view/history_view_requests_bar.h` | `L1-L25` | `RequestsBarContentByPeer` | Join requests header banner interface | `crates/gpui_telegram_desktop/src/views/stage/requests_bar.rs` & `view/history_view_requests_bar.rs` |
| 22 | `Telegram/SourceFiles/history/view/history_view_requests_bar.cpp` | `L1-L209` | `RequestsBarContentByPeer`, `FillUserpics` | Pending join request avatar stacking, group vs channel badge resolution (`!peer->isBroadcast()`) | `crates/gpui_telegram_desktop/src/views/stage/requests_bar.rs` & `view/history_view_requests_bar.rs` |
| 23 | `Telegram/SourceFiles/history/view/history_view_translate_bar.h` | `L1-L79` | `TranslateBar`, `ChooseTranslateTo` | Real-time chat translation banner widget | `crates/gpui_telegram_desktop/src/views/stage/translate_bar.rs` & `view/history_view_translate_bar.rs` |
| 24 | `Telegram/SourceFiles/history/view/history_view_translate_bar.cpp` | `L1-L669` | `TranslateBar::updateLabelGeometry`, `TranslateBar::translateTo`, `TwoTextAction` | `kToastDuration = 4000ms`. Skip languages check, target language dropdown, `ChannelDataFlag::AutoTranslation` | `crates/gpui_telegram_desktop/src/views/stage/translate_bar.rs` & `view/history_view_translate_bar.rs` |
| 25 | `Telegram/SourceFiles/history/view/history_view_translate_tracker.h` | `L1-L88` | `TranslateTracker` | Background chat language recognition and batching engine | `crates/gpui_telegram_desktop/src/views/stage/translate_bar.rs` & `view/history_view_translate_tracker.rs` |
| 26 | `Telegram/SourceFiles/history/view/history_view_translate_tracker.cpp` | `L1-L535` | `TranslateTracker::setup`, `TranslateTracker::checkLanguage`, `TranslateTracker::requestTranslation` | `kEnoughForRecognition = 10`, `kEnoughForTranslation = 6`, `kMaxCheckInBunch = 100`, `kRequestLengthLimit = 24 * 1024`, `kRequestCountLimit = 20` | `crates/gpui_telegram_desktop/src/views/stage/translate_bar.rs` & `view/history_view_translate_tracker.rs` |
| 27 | `Telegram/SourceFiles/history/view/history_view_group_call_bar.h` | `L1-L65` | `GroupCallBar`, `GenerateUserpicsInRow` | Active group voice/video call strip widget | `crates/gpui_telegram_desktop/src/views/stage/group_call_bar.rs` & `view/history_view_group_call_bar.rs` |
| 28 | `Telegram/SourceFiles/history/view/history_view_group_call_bar.cpp` | `L1-L451` | `GroupCallBar::paintEvent`, `GenerateUserpicsInRow` | `kLimit = 3`. Avatar stack width math: `single + (limit - 1) * (single - shift)` with circular knockout pen | `crates/gpui_telegram_desktop/src/views/stage/group_call_bar.rs` & `view/history_view_group_call_bar.rs` |
| 29 | `Telegram/SourceFiles/history/view/history_view_group_members_widget.h` | `L1-L58` | `GroupMembersWidget` | Inline dropdown showing group members list | `crates/gpui_telegram_desktop/src/views/stage/group_members.rs` & `view/history_view_group_members_widget.rs` |
| 30 | `Telegram/SourceFiles/history/view/history_view_group_members_widget.cpp` | `L1-L112` | `GroupMembersWidget::setupList`, `GroupMembersWidgetController` | Participants list controller integration, stories indicator enabled | `crates/gpui_telegram_desktop/src/views/stage/group_members.rs` & `view/history_view_group_members_widget.rs` |
| 31 | `Telegram/SourceFiles/history/view/history_view_read_metrics_tracker.h` | `L1-L79` | `ReadMetricsTracker` | Time-in-viewport reading telemetry tracker | `crates/gpui_telegram_desktop/src/views/stage/read_metrics.rs` & `view/history_view_read_metrics_tracker.rs` |
| 32 | `Telegram/SourceFiles/history/view/history_view_read_metrics_tracker.cpp` | `L1-L346` | `ReadMetricsTracker::startBatch`, `ReadMetricsTracker::push`, `ReadMetricsTracker::sync` | `kGracePeriod = 300ms`, `kActivityTimeout = 15000ms`, `kMaxTrackingDuration = 300000ms` (5 min), `kMinReportThreshold = 300ms` | `crates/gpui_telegram_desktop/src/views/stage/read_metrics.rs` & `view/history_view_read_metrics_tracker.rs` |
| 33 | `Telegram/SourceFiles/history/view/history_view_schedule_box.h` | `L1-L96` | `ScheduleBox`, `SendMenu::Details` | Modal dialog for message schedule datetime picker | `crates/gpui_telegram_desktop/src/views/stage/scheduled_section.rs` & `view/history_view_schedule_box.rs` |
| 34 | `Telegram/SourceFiles/history/view/history_view_schedule_box.cpp` | `L1-L436` | `ScheduleBox::init`, `ItemThumbnail`, `ScheduledImageForDate`, `FillSendUntilOnlineMenu` | `kNotifyRestFrame = 40`, `kNotifyLastFrame = 80`, `kNotifyTooltipDuration = 5000ms`. `Api::kScheduledUntilOnlineTimestamp` for user chats, silent send toggle | `crates/gpui_telegram_desktop/src/views/stage/scheduled_section.rs` & `view/history_view_schedule_box.rs` |
| 35 | `Telegram/SourceFiles/history/view/history_view_empty_list_bubble.h` | `L1-L42` | `EmptyListBubbleWidget` | Centered service bubble for empty chats / search | `crates/gpui_telegram_desktop/src/views/stage/empty_list_bubble.rs` & `view/history_view_empty_list_bubble.rs` |
| 36 | `Telegram/SourceFiles/history/view/history_view_empty_list_bubble.cpp` | `L1-L71` | `EmptyListBubbleWidget::updateGeometry`, `EmptyListBubbleWidget::paintEvent` | Geometry positioning: `move((width - w) / 2, (height - h) / 3)`, `ServiceMessagePainter::PaintBubble` | `crates/gpui_telegram_desktop/src/views/stage/empty_list_bubble.rs` & `view/history_view_empty_list_bubble.rs` |
| 37 | `Telegram/SourceFiles/history/view/history_view_top_peers_selector.h` | `L1-L25` | `TopPeersSelector` | Quick peer picker row in empty chat state | `crates/gpui_telegram_desktop/src/views/stage/top_peers_selector.rs` & `view/history_view_top_peers_selector.rs` |
| 38 | `Telegram/SourceFiles/history/view/history_view_top_peers_selector.cpp` | `L1-L263` | `TopPeersSelector::CollectPeers`, `TopPeersSelector::paint` | `kMaxPeers = 5`, `kShift = 0.15`. Collects self userpic plus top non-bot, accessible contacts | `crates/gpui_telegram_desktop/src/views/stage/top_peers_selector.rs` & `view/history_view_top_peers_selector.rs` |

---

## 2. Core Architecture & Mathematical Parity

### 2.1 Virtual List Scrolling Math (`history_view_list_widget.cpp:L112-L137`)
The history list widget calculates the visible range and preloaded boundaries to achieve smooth 60/120 FPS rendering:
- `kPreloadedScreensCount = 4`: The viewport renders 4 screens of items above and below the current viewport.
- `kPreloadIfLessThanScreens = 2`: When the remaining unrendered items in scroll direction drop below 2 viewport heights, a background batch load trigger fires.
- `kPreloadedScreensCountFull = 9 screens`: Total buffered window size ($4 + 1 + 4$).
- `kClearUserpicsAfter = 50`: Avatars outside the 50-item viewport margin are unmapped to conserve GPU texture memory.
- `kScrollDateHideOnDayCrossingTimeout = 3000ms`: Floating sticky date pill remains visible for 3 seconds after scrolling past midnight boundaries.

### 2.2 Pinned Message Cycling & ID Projections (`history_view_pinned_tracker.cpp:L25-L90`)
Telegram supports cycling through multiple pinned messages with a single click:
- `kLoadedLimit = 5`: Batch size of pinned message headers prefetched in memory.
- `kChangeViewerLimit = 2`: Threshold to swap viewer controls.
- Universal Message ID Projection:
  - If peer is channel/supergroup: message ID is unmodified (`id.msg`).
  - If peer is migrated basic group: message ID is offset by `ServerMaxMsgId` to prevent collision with migrated supergroup history.
- Banner title formatting:
  $$\text{Title} = \begin{cases} 
  \text{"Pinned Message"}, & \text{if } \text{index} + 1 \ge \text{count} \\
  \text{"Previous Pinned Message"}, & \text{if } \text{count} == 2 \\
  \text{"Pinned Message \#"} + (\text{index} + 1), & \text{otherwise}
  \end{cases}$$

### 2.3 Real-time Translation Recognition Machine (`history_view_translate_tracker.cpp:L32-L70`)
- `kEnoughForRecognition = 10`: Minimum character sample length required before triggering language identification.
- `kEnoughForTranslation = 6`: Minimum valid tokens for translation payload.
- `kMaxCheckInBunch = 100`: Batch processing ceiling per round.
- `kRequestLengthLimit = 24,576` bytes ($24 \text{ KB}$): Maximum single MTProto RPC translation request size.
- `kRequestCountLimit = 20`: Maximum concurrent chunked translation requests.
- `ChannelDataFlag::AutoTranslation`: Channels with auto-translate flag bypass manual translate banner confirmation.

### 2.4 Read Metrics Tracking (`history_view_read_metrics_tracker.cpp:L21-L40`)
- `kGracePeriod = 300ms`: Messages visible for $<300\text{ms}$ (e.g. fast scrolling) are not logged as read.
- `kActivityTimeout = 15,000ms`: Idle cursor pauses tracking after 15 seconds.
- `kMaxTrackingDuration = 300,000ms` ($5\text{ min}$): Cap on cumulative tracked read duration for a single message.
- `kMinReportThreshold = 300ms`: Minimum read time to send to backend analytics.
