# Module 06 — Slice 3: History View Element Base & Core Message Layout

## 1. Executive Summary & Upstream Mandate

This specification details the comprehensive analysis, upstream C++ source reading, and genuine GPUI implementation for **Module 06 Slice 3: History View Element Base & Core Message Layout** of the Telegram Desktop 1:1 reconstruction in Rust/GPUI.

In strict adherence to:
- **Mandate #15**: Zero tolerance for boilerplate stubs (`pub label: String, pub value: i64`).
- **Mandate #18 (Anti-Bluffing Protocol)**: Verification via exact upstream `.cpp:L<start>-L<end>` line citations, extraction of anonymous namespace `k...` constants, pixel geometry, and layout arithmetic.
- **Mandate #19 (Pillar 2: 100% Per-File Reading Manifest)**: Exhaustive enumeration of all **37 assigned upstream source files** in `Telegram/SourceFiles/history/view/` with zero omissions.

---

## 2. 100% Upstream Source Reading Manifest

| # | Upstream Source Path | Lines | Key Symbols & Responsibilities | Extracted Constants & Formulas | Target Rust Path & Symbol | Disposition |
|---|---|---|---|---|---|---|
| 1 | `Telegram/SourceFiles/history/view/history_view_object.h` | 73 | `Object`: Root base class of all layout elements | `optimalSize()`, `currentSize()`, `resizeGetHeight(w)`, `adjustHeight(d)` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_object.rs` (`HistoryViewObjectView`) | `gpui_ui_1to1` |
| 2 | `Telegram/SourceFiles/history/view/history_view_element.h` | 931 | `Element`: Base class for chat stage items; `ElementDelegate`, `DefaultElementDelegate`, `SimpleElementDelegate`, `UnreadBar`, `DateBadge`, `ForumThreadBar`, `ServicePreMessage`, `EphemeralBadge` | `enum class Flag : uint16` (`ServiceMessage=0x1`, `NeedsResize=0x2`, `AttachedToPrevious=0x4`, `AttachedToNext=0x8`, `BubbleAttachedToPrevious=0x10`, `BubbleAttachedToNext=0x20`, `HiddenByGroup=0x40`, `SpecialOnlyEmoji=0x80`, `HeavyCustomEmoji=0x1000`, `SummaryShown=0x2000`) | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_element.rs` (`HistoryViewElementView`), `crates/gpui_telegram_desktop/src/views/stage/history_element.rs` | `gpui_ui_1to1` |
| 3 | `Telegram/SourceFiles/history/view/history_view_element.cpp` | 3,970 | `Element` implementation: layout dispatch, margin accumulation, selection shifting, date tooltip, keyboard style | `kAttachMessageToPreviousSecondsDelta = 900` (15 min), `kMaxShownLine = 1048576` (1MB line limit), `ServicePreMessage::resizeToWidth` (`msgServicePadding` 12x3x12x4, `msgServiceMargin` 10x10x10x2) | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_element.rs` (`HistoryViewElementView`) | `gpui_ui_1to1` |
| 4 | `Telegram/SourceFiles/history/view/history_view_element_overlay.h` | 67 | `ElementOverlayHost`: Floating interactive overlay container for polls, stickers, reactions, inline editors | `ItemTopFn`, `PositionFn`, `MediaStateFn`, `show()`, `hide()`, `updatePosition()`, `handleClickOutside()` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_element_overlay.rs` (`HistoryViewElementOverlayView`) | `gpui_ui_1to1` |
| 5 | `Telegram/SourceFiles/history/view/history_view_element_overlay.cpp` | 133 | `ElementOverlayHost` implementation: dynamic repositioning on scroll, dismissal handling, click outside tracking | Position recalculation hook relative to view Y coordinate; notify media state changes on open/close | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_element_overlay.rs` (`HistoryViewElementOverlayView`) | `gpui_ui_1to1` |
| 6 | `Telegram/SourceFiles/history/view/history_view_message.h` | 566 | `Message`: Core message view element; text, media, reactions, comments, reply button, topic button, right action | `RightBadge` (`tag`, `boosts`, `role`), `TextAppearing`, `CommentsButton`, `MessageSelection`, `Factcheck` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_message.rs` (`HistoryViewMessageView`), `crates/gpui_telegram_desktop/src/views/stage/history_message.rs` | `gpui_ui_1to1` |
| 7 | `Telegram/SourceFiles/history/view/history_view_message.cpp` | 7,373 | `Message` layout & painting: bubble geometry, width clamping, selection bounding, info row integration | `msgMaxWidth = 430`, `msgMinWidth = 160`, `msgPadding = margins(11, 8, 11, 8)`, `msgMargin = margins(16, 6, 56, 2)`, `bubbleRadiusLarge = 16`, `bubbleRadiusSmall = 4`, `resizeContentGetHeight` L6650-L6720 | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_message.rs` (`HistoryViewMessageView`) | `gpui_ui_1to1` |
| 8 | `Telegram/SourceFiles/history/view/history_view_service_message.h` | 168 | `Service`: Pill-shaped center service notification; `ServiceMessagePainter`: Date, action info, join/leave, pin | `WideChatWidth() = msgMaxWidth + 2 * msgPhotoSkip + 2 * msgMargin.left()` (542px), `ServiceAllowsSwipeReply` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_service_message.rs` (`HistoryViewServiceMessageView`), `crates/gpui_telegram_desktop/src/views/stage/history_service.rs` | `gpui_ui_1to1` |
| 9 | `Telegram/SourceFiles/history/view/history_view_service_message.cpp` | 1,169 | `Service` layout & painting: pill bubble drawing, `SideStyle` (`Rounded`, `Plain`, `Inverted`), `CornerVerticalSide` | `msgServicePadding = margins(12, 3, 12, 4)`, `msgServiceMargin = margins(10, 10, 10, 2)`, `performCountCurrentSize` L500-L560 | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_service_message.rs` (`HistoryViewServiceMessageView`) | `gpui_ui_1to1` |
| 10 | `Telegram/SourceFiles/history/view/history_view_bottom_info.h` | 140 | `BottomInfo`: Timestamp, delivery status ticks, views, replies, reaction effect, sponsored indicator | `Data::Flag` (`Edited=0x1`, `OutLayout=0x2`, `Sending=0x4`, `RepliesContext=0x8`, `Sponsored=0x10`, `Pinned=0x20`, `Imported=0x40`, `Silent=0x400`, `Ephemeral=0x1000`) | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_bottom_info.rs` (`HistoryViewBottomInfoView`), `crates/gpui_telegram_desktop/src/views/stage/bottom_info.rs` | `gpui_ui_1to1` |
| 11 | `Telegram/SourceFiles/history/view/history_view_bottom_info.cpp` | 774 | `BottomInfo` layout & painting: formatted date, author string truncation, view counters, effect icon layout | `SchedulePeriodText` map (`60s`, `300s`, `1d`, `7d`, `14d`, `30d`, `91d`, `182d`, `365d`), `msgDateDelta = (2, 5)`, `historyViewsSpace = 4` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_bottom_info.rs` (`HistoryViewBottomInfoView`) | `gpui_ui_1to1` |
| 12 | `Telegram/SourceFiles/history/view/history_view_reply.h` | 180 | `Reply`: Quoted message reference preview in bubble; sender name, snippet, thumbnail, emoji background | `ValidateBackgroundEmoji`, `FillBackgroundEmoji`, `FillPreviewSpoiler`, `ComposePreviewName`, `ComposePreviewQuote` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_reply.rs` (`HistoryViewReplyView`), `crates/gpui_telegram_desktop/src/views/stage/reply_view.rs` | `gpui_ui_1to1` |
| 13 | `Telegram/SourceFiles/history/view/history_view_reply.cpp` | 1,189 | `Reply` implementation: multiline measurement, outline strip, thumbnail placement, expansion collapse | `kNonExpandedLinesLimit = 5`, `historyReplyPreview = 32px`, `historyReplyPreviewMargin = margins(7, 4, 4, 4)`, `historyReplyPadding = margins(11, 2, 6, 2)`, `msgReplyBarSize = (2, 36)` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_reply.rs` (`HistoryViewReplyView`) | `gpui_ui_1to1` |
| 14 | `Telegram/SourceFiles/history/view/history_view_reply_button.h` | 153 | `ReplyButton::Button`: Floating quick-reply button attached to bubble corner; `ButtonParameters`, `Manager` | `ButtonState` (`Hidden`, `Shown`, `Active`, `Inside`), `currentScale()`, `currentOpacity()`, `ComputeInnerWidth()` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_reply_button.rs` (`HistoryViewReplyButtonView`) | `gpui_ui_1to1` |
| 15 | `Telegram/SourceFiles/history/view/history_view_reply_button.cpp` | 396 | `ReplyButton` implementation: spring/fade animation, hover timer, geometry tracking | `kToggleDuration = 120ms`, `kButtonShowDelay = 0ms`, `kButtonHideDelay = 300ms`, `st::replyCornerHeight = 28px` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_reply_button.rs` (`HistoryViewReplyButtonView`) | `gpui_ui_1to1` |
| 16 | `Telegram/SourceFiles/history/view/history_view_corner_buttons.h` | 161 | `CornerButtons`: Floating corner button manager; scroll to bottom, unread mentions, reactions, poll votes, stash | `CornerButtonType` (`Down`, `Mentions`, `Reactions`, `PollVotes`, `Stash`), `replyReturns` stack, `CornerButtonsDelegate` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_corner_buttons.rs` (`HistoryViewCornerButtonsView`) | `gpui_ui_1to1` |
| 17 | `Telegram/SourceFiles/history/view/history_view_corner_buttons.cpp` | 741 | `CornerButtons` implementation: button positioning, vertical stacking, stash bounce animation | `kStashDelay = 150ms`, `kStashArrowDuration = 106ms`, `kStashBounceDuration = 173ms`, `kStashDuration = 429ms`, `ArrowShift()`, `BubbleSwing()` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_corner_buttons.rs` (`HistoryViewCornerButtonsView`) | `gpui_ui_1to1` |
| 18 | `Telegram/SourceFiles/history/view/history_view_cursor_state.h` | 95 | `PointState`, `CursorState`, `TextState`, `StateRequest`, `InfoDisplayType`, `LookupItemByPoint` | `PointState::Outside/Inside/GroupPart`, `CursorState::None/Text/Date/Enlarge/Forwarded/FromPhoto/LogAdminService` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_cursor_state.rs` (`HistoryViewCursorStateView`) | `gpui_ui_1to1` |
| 19 | `Telegram/SourceFiles/history/view/history_view_cursor_state.cpp` | 117 | `TextState` constructors, endpoint sync, character offset arithmetic | `SyncFlatSelectionCursor`: syncs symbol & afterSymbol to `MessageSelectionEndpoint::Flat`, `LookupItemByPoint` album resolution | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_cursor_state.rs` (`HistoryViewCursorStateView`) | `gpui_ui_1to1` |
| 20 | `Telegram/SourceFiles/history/view/history_view_keyboard_text_selection.h` | 38 | `KeyboardTextSelection`: Keyboard navigation and expansion of message text selection | `IsExtendKey(key)` (Left, Right, Home, End), `extend()` with modifiers | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_keyboard_text_selection.rs` (`HistoryViewKeyboardTextSelectionView`) | `gpui_ui_1to1` |
| 21 | `Telegram/SourceFiles/history/view/history_view_keyboard_text_selection.cpp` | 115 | `KeyboardTextSelection::extend` implementation: letter by letter vs word jumping via separator skipping | macOS `AltModifier` vs Windows/Linux `ControlModifier` for word jumping; forward vs backward separator skipping loops | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_keyboard_text_selection.rs` (`HistoryViewKeyboardTextSelectionView`) | `gpui_ui_1to1` |
| 22 | `Telegram/SourceFiles/history/view/history_view_copy_selection.h` | 37 | `CopySelectionArgs`, `CopySelection()`, `IsCopyWithImageTextShortcut` | Image text extraction integration, clipboard format negotiation | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_copy_selection.rs` (`HistoryViewCopySelectionView`) | `gpui_ui_1to1` |
| 23 | `Telegram/SourceFiles/history/view/history_view_copy_selection.cpp` | 373 | `CopySelection` implementation: OCR queued dispatch, text concatenation, formatting entities preserving | `kMaxRecognitionsInFlight = 2`, `PhotoData::SideLimit() = 2560px` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_copy_selection.rs` (`HistoryViewCopySelectionView`) | `gpui_ui_1to1` |
| 24 | `Telegram/SourceFiles/history/view/history_view_draw_to_reply.h` | 31 | `ResolveDrawToReplyImage`, `OpenDrawToReplyEditor` | Photo & document image extraction for drawing over media reply | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_draw_to_reply.rs` (`HistoryViewDrawToReplyView`) | `gpui_ui_1to1` |
| 25 | `Telegram/SourceFiles/history/view/history_view_draw_to_reply.cpp` | 89 | `ResolveDrawToReplyImage` implementation: reads photo large image or decrypted document image bytes | Image format validation, editor invocation bridge | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_draw_to_reply.rs` (`HistoryViewDrawToReplyView`) | `gpui_ui_1to1` |
| 26 | `Telegram/SourceFiles/history/view/history_view_fake_items.h` | 27 | `GenerateItem`, `GenerateUser` | Test and preview mock message item generator for admin log and preview views | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_fake_items.rs` (`HistoryViewFakeItemsView`) | `gpui_ui_1to1` |
| 27 | `Telegram/SourceFiles/history/view/history_view_fake_items.cpp` | 71 | `GenerateItem`, `GenerateUser` implementation: synthetic peer ID generation, local history message insertion | `MessageFlag::FakeHistoryItem | HasFromId | HasReplyInfo`, `Data::FakePeerIdForJustName` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_fake_items.rs` (`HistoryViewFakeItemsView`) | `gpui_ui_1to1` |
| 28 | `Telegram/SourceFiles/history/view/history_view_empty_list_bubble.h` | 43 | `EmptyListBubbleWidget`: Central empty chat placeholder bubble ("No messages here yet...") | `setText()`, `setForceWidth()`, `paintEvent()`, `updateGeometry()` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_empty_list_bubble.rs` (`HistoryViewEmptyListBubbleView`) | `gpui_ui_1to1` |
| 29 | `Telegram/SourceFiles/history/view/history_view_empty_list_bubble.cpp` | 72 | `EmptyListBubbleWidget` implementation: vertical positioning at 1/3 height, service bubble background | Positioned at `(size.width() - w) / 2`, `(size.height() - h) / 3` (one-third height golden ratio) | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_empty_list_bubble.rs` (`HistoryViewEmptyListBubbleView`) | `gpui_ui_1to1` |
| 30 | `Telegram/SourceFiles/history/view/history_view_rich_quote.h` | 63 | `RichQuotePaint`, `RichQuoteHighlightYRange`, `RichQuoteForSelection` | Quote highlight geometry builder, range mapping to Markdown article blocks | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_rich_quote.rs` (`HistoryViewRichQuoteView`), `crates/gpui_telegram_desktop/src/views/stage/rich_quote.rs` | `gpui_ui_1to1` |
| 31 | `Telegram/SourceFiles/history/view/history_view_rich_quote.cpp` | 330 | `RichQuotePaint` implementation: slice edge lookup, leaf point projection, path construction | `AppendDisplayRanges`, `ProjectionPoint`, `FindSliceEdge`, `leafPointForDisplay` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_rich_quote.rs` (`HistoryViewRichQuoteView`) | `gpui_ui_1to1` |
| 32 | `Telegram/SourceFiles/history/view/history_view_summary_header.h` | 87 | `SummaryHeader`: AI / Bot summarization header strip rendered atop long conversation threads | `_animation` (`Ui::StarParticles`), `_lottie` icon, ripple animation, expand link | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_summary_header.rs` (`HistoryViewSummaryHeaderView`) | `gpui_ui_1to1` |
| 33 | `Telegram/SourceFiles/history/view/history_view_summary_header.cpp` | 320 | `SummaryHeader` implementation: particle animation updates, Lottie star rendering, dynamic width layout | Lottie asset `summary_header.json`, ministar particle burst on hover | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_summary_header.rs` (`HistoryViewSummaryHeaderView`) | `gpui_ui_1to1` |
| 34 | `Telegram/SourceFiles/history/view/history_view_text_helper.h` | 17 | `InitElementTextPart`: Text item initializer for spoilers, custom emoji, blockquotes | Spoiler revealed filter, custom emoji preview handler, collapsed blockquote expander | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_text_helper.rs` (`HistoryViewTextHelperView`) | `gpui_ui_1to1` |
| 35 | `Telegram/SourceFiles/history/view/history_view_text_helper.cpp` | 75 | `InitElementTextPart` implementation: spoiler click registration, blockquote expand callback, custom emoji reactions | `Context::MediaEditor` instant spoiler reveal; `registerShownSpoiler` on left button click | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_text_helper.rs` (`HistoryViewTextHelperView`) | `gpui_ui_1to1` |
| 36 | `Telegram/SourceFiles/history/view/history_view_drag.h` | 28 | `PhotoDragData`, `PreparePhotoDragData`, `FillDragMimeWithPhoto` | Message media drag-and-drop serialization | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_drag.rs` (`HistoryViewDragView`) | `gpui_ui_1to1` |
| 37 | `Telegram/SourceFiles/history/view/history_view_drag.cpp` | 48 | `PreparePhotoDragData` implementation: large photo extraction, JPEG MIME payload configuration | `application/x-td-use-jpeg = "1"`, `image/jpeg` MIME payload | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_drag.rs` (`HistoryViewDragView`) | `gpui_ui_1to1` |

---

## 3. Deep Architectural Analysis & Mandate #18 Line Citations

### 3.1 Base Element Dimension Lifecycle & Flags (`history_view_object.h`, `history_view_element.h/cpp`)
Upstream C++ classes:
- `HistoryView::Object`: Defined in `history_view_object.h:L12-L73`. It establishes the two-phase layout protocol used throughout Telegram history views:
  1. `countOptimalSize()` calculates maximum natural width (`_maxWidth`) and minimum single-line height (`_minHeight`).
  2. `countCurrentSize(int newWidth)` calculates final geometry given constraint `newWidth`.
- `HistoryView::Element`: Defined in `history_view_element.h:L435-L931` and `history_view_element.cpp:L82-L3970`.
  - Flags bitfield (`history_view_element.h:L441-L456`):
    - `ServiceMessage = 0x0001`
    - `NeedsResize = 0x0002`
    - `AttachedToPrevious = 0x0004`
    - `AttachedToNext = 0x0008`
    - `BubbleAttachedToPrevious = 0x0010`
    - `BubbleAttachedToNext = 0x0020`
    - `HiddenByGroup = 0x0040`
    - `SpecialOnlyEmoji = 0x0080`
    - `CustomEmojiRepainting = 0x0100`
    - `ScheduledUntilOnline = 0x0200`
    - `TopicRootReply = 0x0400`
    - `MediaOverriden = 0x0800`
    - `HeavyCustomEmoji = 0x1000`
    - `SummaryShown = 0x2000`
  - Grouping constant (`history_view_element.cpp:L105`):
    - `constexpr int kAttachMessageToPreviousSecondsDelta = 900;` (15 minutes). If two consecutive messages from the same sender arrive within 900 seconds, their bubbles attach: top/bottom margins collapse from `6px` to `0px` (`st::msgMarginTopAttached`), and corner radii on the attached edge shrink from `16px` (`bubbleRadiusLarge`) to `4px` (`bubbleRadiusSmall`).
  - Line rendering limit (`history_view_element.cpp:L106`):
    - `constexpr auto kMaxShownLine = 1024 * 1024;`

### 3.2 Message Bubble Layout & Sizing Math (`history_view_message.h/cpp`)
Defined in `history_view_message.cpp:L6650-L6720`:
```cpp
int Message::resizeContentGetHeight(int newWidth) {
    auto contentWidth = newWidth - st::msgMargin.left() - (centeredView ? st::msgMargin.left() : wideSkip);
    if (hasFromPhoto()) {
        contentWidth -= size->width() + (st::msgPhotoSkip - st::historyFastShareSize);
    }
    accumulate_min(contentWidth, maxWidth());
    _bubbleWidthLimit = (UnlimitedMessageWidth.value() && !mediaDisplayed)
        ? 0x3FFFFFF
        : std::max({ st::msgMaxWidth, monospaceMaxWidth(), mediaDisplayed ? media->bubbleWidthLimit() : 0 });
    accumulate_min(contentWidth, int(_bubbleWidthLimit));
```
Exact metrics extracted from `ui/chat/chat.style:L14-L100`:
- `msgMaxWidth = 430px;`
- `msgMinWidth = 160px;`
- `msgPhotoSize = 33px;`
- `msgPhotoSkip = 40px;`
- `msgPadding = margins(11px, 8px, 11px, 8px);` (Left: 11px, Top: 8px, Right: 11px, Bottom: 8px)
- `msgMargin = margins(16px, 6px, 56px, 2px);` (Left: 16px, Top: 6px, Right: 56px, Bottom: 2px)
- `msgMarginTopAttached = 0px;`
- `bubbleRadiusLarge = 16px;`
- `bubbleRadiusSmall = 4px;`

### 3.3 Service Message Pill Layout (`history_view_service_message.h/cpp`)
Defined in `history_view_service_message.cpp:L500-L560`:
```cpp
QSize Service::performCountCurrentSize(int newWidth) {
    auto contentWidth = newWidth;
    if (delegate()->elementChatMode() == ElementChatMode::Wide) {
        accumulate_min(contentWidth, st::msgMaxWidth + 2 * st::msgPhotoSkip + 2 * st::msgMargin.left());
    }
    contentWidth -= st::msgServiceMargin.left() + st::msgServiceMargin.left();
    if (contentWidth < st::msgServicePadding.left() + st::msgServicePadding.right() + 1) {
        contentWidth = st::msgServicePadding.left() + st::msgServicePadding.right() + 1;
    }
```
Exact constants:
- `WideChatWidth() = 430 + 2 * 40 + 2 * 16 = 542px;`
- `msgServicePadding = margins(12px, 3px, 12px, 4px);`
- `msgServiceMargin = margins(10px, 10px, 10px, 2px);`

### 3.4 Bottom Info Geometry & Ticks (`history_view_bottom_info.h/cpp`)
Defined in `history_view_bottom_info.h:L35-L67` and `history_view_bottom_info.cpp:L300-L392`:
- Flags:
  - `Edited = 0x001`
  - `OutLayout = 0x002`
  - `Sending = 0x004`
  - `RepliesContext = 0x008`
  - `Sponsored = 0x010`
  - `Pinned = 0x020`
  - `Imported = 0x040`
  - `Shortcut = 0x080`
  - `EstimateDate = 0x100`
  - `ForwardedDate = 0x200`
  - `Silent = 0x400`
  - `EditedPrimary = 0x800`
  - `Ephemeral = 0x1000`
  - `Updated = 0x2000`
  - `FullDate = 0x4000`
- Timestamp offsets and icons:
  - `msgDateDelta = point(2px, 5px);`
  - `historyViewsSpace = 4px;`
  - Outgoing delivery ticks: Single check for sent, double check for read, clock icon for pending.

### 3.5 Quoted Reply Preview Strip (`history_view_reply.h/cpp`)
Defined in `history_view_reply.cpp:L746-L799`:
- Outline bar: `msgReplyBarPos = (1px, 0px); msgReplyBarSize = (2px, 36px); msgReplyBarSkip = 10px;`
- Padding: `historyReplyPadding = margins(11px, 2px, 6px, 2px);`
- Margins: `historyReplyTop = 2px; historyReplyBottom = 2px;`
- Thumbnail: `historyReplyPreview = 32px; historyReplyPreviewMargin = margins(7px, 4px, 4px, 4px);`
- Expansion limit (`history_view_reply.cpp:L46`):
  - `constexpr auto kNonExpandedLinesLimit = 5;`

### 3.6 Floating Quick Reply Button (`history_view_reply_button.h/cpp`)
Defined in `history_view_reply_button.cpp:L20-L48`:
- `constexpr auto kToggleDuration = crl::time(120);` (120ms animation)
- `constexpr auto kButtonShowDelay = crl::time(0);`
- `constexpr auto kButtonHideDelay = crl::time(300);` (300ms hover delay)
- `st::replyCornerHeight = 28px;`

### 3.7 Floating Corner Buttons Stack (`history_view_corner_buttons.h/cpp`)
Defined in `history_view_corner_buttons.cpp:L37-L63`:
- Stash animation constants:
  - `constexpr auto kStashDelay = crl::time(150);`
  - `constexpr auto kStashArrowDuration = crl::time(106);`
  - `constexpr auto kStashBounceDuration = crl::time(173);`
  - `constexpr auto kStashDuration = 429ms;`
- Vertical stacking order (from bottom up):
  1. Jump down button (`_down`)
  2. Unread mentions button (`_mentions`)
  3. Unread reactions button (`_reactions`)
  4. Unread poll votes button (`_pollVotes`)
  5. Saved stash button (`_stash`)

### 3.8 Keyboard Navigation & Selection Math (`history_view_keyboard_text_selection.cpp:L16-L112`)
- Extended keys: Left, Right, Home, End.
- Word jumping: On macOS uses `AltModifier`, Windows/Linux uses `ControlModifier`.
- Skips separator characters (`Ui::Text::IsWordSeparator`) before seeking boundary character.

---

## 4. GPUI Target Implementation Strategy

The GPUI desktop client targets the following genuine modules:
1. `crates/gpui_telegram_desktop/src/views/stage/history_element.rs`:
   - `HistoryElementBase`, `ElementFlags`, `BubbleRounding`, `AttachedEdge`, `ElementLayoutGeometry`.
2. `crates/gpui_telegram_desktop/src/views/stage/history_message.rs`:
   - `MessageBubbleView`, message text layout, author title, right badge, reactions bar, delivery ticks.
3. `crates/gpui_telegram_desktop/src/views/stage/history_service.rs`:
   - `ServiceMessageBubbleView`, service message pill, date separator, join/leave, pin announcement.
4. `crates/gpui_telegram_desktop/src/views/stage/bottom_info.rs`:
   - `BottomInfoView`, `BottomInfoData`, timestamp, read status ticks, views counter, ephemeral timer.
5. `crates/gpui_telegram_desktop/src/views/stage/rich_quote.rs`:
   - `RichQuoteView`, quote strip, blockquote border, multiline quote rendering.
6. `crates/gpui_telegram_desktop/src/views/stage/reply_view.rs`:
   - `ReplyPreviewView`, reply sender name, text snippet, preview media thumbnail, expansion clamp.
7. Overwriting all 19 autogenerated boilerplate stubs in `crates/gpui_telegram_desktop/src/views/stage/view/*.rs` with authentic GPUI elements matching upstream signatures.

---

## 5. Verification & Testing Strategy

- `crates/gpui_telegram_desktop/tests/history_view_element_subsystem_tests.rs`:
  - Test 1: Bubble layout geometry and padding calculations (`msgPadding = (11, 8, 11, 8)`).
  - Test 2: Message attachment delta (within 900s attach = true; >900s attach = false).
  - Test 3: Corner rounding arithmetic (attached corner = 4px, standard corner = 16px).
  - Test 4: Service message pill width and wide chat width calculation (`WideChatWidth = 542px`).
  - Test 5: Bottom info bitflags and timestamp formatting.
  - Test 6: Reply preview line clamping (`kNonExpandedLinesLimit = 5`).
  - Test 7: Corner buttons stash animation timing (`kStashDuration = 429ms`).
  - Test 8: Keyboard text selection word boundary navigation.
  - Test 9: Copy selection OCR queue limits (`kMaxRecognitionsInFlight = 2`).


## 6. Complete Module 06 Upstream History File Cross-Reference Manifest

| Upstream Source Path | Line Count | SHA256 | Target Rust Path | Target Symbol | Disposition |
|---|---|---|---|---|---|
| `Telegram/SourceFiles/history/admin_log/history_admin_log_filter.cpp` | 116 | `cc6874c5f84a...` | `crates/gpui_telegram_desktop/src/views/stage/admin_log/history_admin_log_filter.rs` | `HistoryAdminLogFilterAdminLog` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/admin_log/history_admin_log_filter.h` | 31 | `32176b689b70...` | `crates/gpui_telegram_desktop/src/views/stage/admin_log/history_admin_log_filter.rs` | `HistoryAdminLogFilterAdminLog` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/admin_log/history_admin_log_filter_value.h` | 54 | `41e82b5ec7b3...` | `crates/gpui_telegram_desktop/src/views/stage/admin_log/history_admin_log_filter_value.rs` | `HistoryAdminLogFilterValueAdminLog` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/admin_log/history_admin_log_inner.cpp` | 3532 | `f24a9b14acea...` | `crates/gpui_telegram_desktop/src/views/stage/admin_log/history_admin_log_inner.rs` | `HistoryAdminLogInnerAdminLog` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/admin_log/history_admin_log_inner.h` | 498 | `9446dd2ca7a1...` | `crates/gpui_telegram_desktop/src/views/stage/admin_log/history_admin_log_inner.rs` | `HistoryAdminLogInnerAdminLog` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/admin_log/history_admin_log_item.cpp` | 2352 | `880c5748adf8...` | `crates/gpui_telegram_desktop/src/views/stage/admin_log/history_admin_log_item.rs` | `HistoryAdminLogItemAdminLog` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/admin_log/history_admin_log_item.h` | 59 | `b1e3277b3991...` | `crates/gpui_telegram_desktop/src/views/stage/admin_log/history_admin_log_item.rs` | `HistoryAdminLogItemAdminLog` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/admin_log/history_admin_log_section.cpp` | 677 | `98c9ecf37e79...` | `crates/gpui_telegram_desktop/src/views/stage/admin_log/history_admin_log_section.rs` | `HistoryAdminLogSectionAdminLog` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/admin_log/history_admin_log_section.h` | 218 | `f952019a3e33...` | `crates/gpui_telegram_desktop/src/views/stage/admin_log/history_admin_log_section.rs` | `HistoryAdminLogSectionAdminLog` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history.cpp` | 4720 | `dc2f4be1806c...` | `crates/gpui_telegram_desktop/src/views/stage/history.rs` | `History` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history.h` | 802 | `25efb6feb497...` | `crates/gpui_telegram_desktop/src/views/stage/history.rs` | `History` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_drag_area.cpp` | 774 | `6c06cff04262...` | `crates/gpui_telegram_desktop/src/views/stage/history_drag_area.rs` | `HistoryDragArea` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_drag_area.h` | 92 | `347e9bc83203...` | `crates/gpui_telegram_desktop/src/views/stage/history_drag_area.rs` | `HistoryDragArea` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_inner_widget.cpp` | 7055 | `29725621945e...` | `crates/gpui_telegram_desktop/src/views/stage/history_inner_widget.rs` | `HistoryInnerWidget` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_inner_widget.h` | 733 | `ba6c4db47ea3...` | `crates/gpui_telegram_desktop/src/views/stage/history_inner_widget.rs` | `HistoryInnerWidget` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_inner_widget_accessibility.cpp` | 1167 | `f918ddbb2e49...` | `crates/gpui_telegram_desktop/src/views/stage/history_inner_widget_accessibility.rs` | `HistoryInnerWidgetAccessibility` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_inner_widget_accessibility.h` | 81 | `094c087334f9...` | `crates/gpui_telegram_desktop/src/views/stage/history_inner_widget_accessibility.rs` | `HistoryInnerWidgetAccessibility` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_item.cpp` | 9245 | `c4b7051b5c90...` | `crates/gpui_telegram_desktop/src/views/stage/history_item.rs` | `HistoryItem` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_item.h` | 847 | `64222f456b69...` | `crates/gpui_telegram_desktop/src/views/stage/history_item.rs` | `HistoryItem` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_item_components.cpp` | 1685 | `445175edbbe9...` | `crates/gpui_telegram_desktop/src/views/stage/history_item_components.rs` | `HistoryItemComponents` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_item_components.h` | 1029 | `79b663e19e87...` | `crates/gpui_telegram_desktop/src/views/stage/history_item_components.rs` | `HistoryItemComponents` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_item_edition.cpp` | 50 | `d3b42ee521f2...` | `crates/gpui_telegram_desktop/src/views/stage/history_item_edition.rs` | `HistoryItemEdition` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_item_edition.h` | 65 | `7051f0022894...` | `crates/gpui_telegram_desktop/src/views/stage/history_item_edition.rs` | `HistoryItemEdition` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_item_helpers.cpp` | 1530 | `72ba66ae7818...` | `crates/gpui_telegram_desktop/src/views/stage/history_item_helpers.rs` | `HistoryItemHelpers` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_item_helpers.h` | 310 | `4654ecf644f3...` | `crates/gpui_telegram_desktop/src/views/stage/history_item_helpers.rs` | `HistoryItemHelpers` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_item_reply_markup.cpp` | 570 | `1ccc03adcebe...` | `crates/gpui_telegram_desktop/src/views/stage/history_item_reply_markup.rs` | `HistoryItemReplyMarkup` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_item_reply_markup.h` | 225 | `d099604c09bc...` | `crates/gpui_telegram_desktop/src/views/stage/history_item_reply_markup.rs` | `HistoryItemReplyMarkup` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_item_text.cpp` | 814 | `67d33efb972a...` | `crates/gpui_telegram_desktop/src/views/stage/history_item_text.rs` | `HistoryItemText` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_item_text.h` | 45 | `0df82d43b005...` | `crates/gpui_telegram_desktop/src/views/stage/history_item_text.rs` | `HistoryItemText` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_location_manager.cpp` | 42 | `54a84f98eac4...` | `crates/gpui_telegram_desktop/src/views/stage/history_location_manager.rs` | `HistoryLocationManager` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_location_manager.h` | 44 | `662154c0227b...` | `crates/gpui_telegram_desktop/src/views/stage/history_location_manager.rs` | `HistoryLocationManager` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_message_selection.h` | 200 | `8f8c5db739f9...` | `crates/gpui_telegram_desktop/src/views/stage/history_message_selection.rs` | `HistoryMessageSelection` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_streamed_drafts.cpp` | 541 | `cb85dffa76ec...` | `crates/gpui_telegram_desktop/src/views/stage/history_streamed_drafts.rs` | `HistoryStreamedDrafts` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_streamed_drafts.h` | 111 | `e03825467509...` | `crates/gpui_telegram_desktop/src/views/stage/history_streamed_drafts.rs` | `HistoryStreamedDrafts` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_translation.cpp` | 51 | `b0ce78294d5d...` | `crates/gpui_telegram_desktop/src/views/stage/history_translation.rs` | `HistoryTranslation` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_translation.h` | 32 | `37a0b8e93cfb...` | `crates/gpui_telegram_desktop/src/views/stage/history_translation.rs` | `HistoryTranslation` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_unread_things.cpp` | 297 | `7334c188d484...` | `crates/gpui_telegram_desktop/src/views/stage/history_unread_things.rs` | `HistoryUnreadThings` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_unread_things.h` | 153 | `844e55ddaab6...` | `crates/gpui_telegram_desktop/src/views/stage/history_unread_things.rs` | `HistoryUnreadThings` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_view_highlight_manager.cpp` | 318 | `bfc7d1ab9ed2...` | `crates/gpui_telegram_desktop/src/views/stage/history_view_highlight_manager.rs` | `HistoryViewHighlightManager` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_view_highlight_manager.h` | 102 | `b1b9fc13a0f7...` | `crates/gpui_telegram_desktop/src/views/stage/history_view_highlight_manager.rs` | `HistoryViewHighlightManager` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_view_pull_to_next_channel.cpp` | 1117 | `2b82307c98f6...` | `crates/gpui_telegram_desktop/src/views/stage/history_view_pull_to_next_channel.rs` | `HistoryViewPullToNextChannel` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_view_pull_to_next_channel.h` | 104 | `d341034121ce...` | `crates/gpui_telegram_desktop/src/views/stage/history_view_pull_to_next_channel.rs` | `HistoryViewPullToNextChannel` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_view_swipe_back_session.cpp` | 63 | `74ba60025893...` | `crates/gpui_telegram_desktop/src/views/stage/history_view_swipe_back_session.rs` | `HistoryViewSwipeBackSession` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_view_swipe_back_session.h` | 30 | `1471cb19277d...` | `crates/gpui_telegram_desktop/src/views/stage/history_view_swipe_back_session.rs` | `HistoryViewSwipeBackSession` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_view_top_toast.cpp` | 118 | `01fade529322...` | `crates/gpui_telegram_desktop/src/views/stage/history_view_top_toast.rs` | `HistoryViewTopToast` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_view_top_toast.h` | 65 | `2a10aa3466bc...` | `crates/gpui_telegram_desktop/src/views/stage/history_view_top_toast.rs` | `HistoryViewTopToast` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_widget.cpp` | 11656 | `6f885383882b...` | `crates/gpui_telegram_desktop/src/views/stage/history_widget.rs` | `HistoryWidget` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/history_widget.h` | 1040 | `8dc7363ee8d2...` | `crates/gpui_telegram_desktop/src/views/stage/history_widget.rs` | `HistoryWidget` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/compose_controls_common.h` | 114 | `977f0dc03185...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/compose_controls_common.rs` | `ComposeControlsCommonViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_bot_menu_button.cpp` | 300 | `769efce64cae...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_bot_menu_button.rs` | `HistoryViewBotMenuButtonViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_bot_menu_button.h` | 45 | `b954705ff950...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_bot_menu_button.rs` | `HistoryViewBotMenuButtonViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_bottom_controls.cpp` | 616 | `e92b68a5cd8c...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_bottom_controls.rs` | `HistoryViewBottomControlsViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_bottom_controls.h` | 146 | `e461a02c4cd4...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_bottom_controls.rs` | `HistoryViewBottomControlsViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_characters_limit.cpp` | 71 | `879848f9591a...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_characters_limit.rs` | `HistoryViewCharactersLimitViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_characters_limit.h` | 31 | `8ce4061280d2...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_characters_limit.rs` | `HistoryViewCharactersLimitViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_compose_ai_button.cpp` | 177 | `8a4fdac21832...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_ai_button.rs` | `HistoryViewComposeAiButtonViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_compose_ai_button.h` | 59 | `63eb18ff80a2...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_ai_button.rs` | `HistoryViewComposeAiButtonViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_compose_ai_tooltip.cpp` | 90 | `6fd06087d9ea...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_ai_tooltip.rs` | `HistoryViewComposeAiTooltipViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_compose_ai_tooltip.h` | 45 | `e2cf3444d773...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_ai_tooltip.rs` | `HistoryViewComposeAiTooltipViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_compose_controls.cpp` | 6646 | `01a87fc24d66...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_controls.rs` | `HistoryViewComposeControlsViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_compose_controls.h` | 695 | `a2742a7af193...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_controls.rs` | `HistoryViewComposeControlsViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_compose_media_edit_manager.cpp` | 278 | `12159bd9f3cb...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_media_edit_manager.rs` | `HistoryViewComposeMediaEditManagerViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_compose_media_edit_manager.h` | 93 | `a7752039731d...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_media_edit_manager.rs` | `HistoryViewComposeMediaEditManagerViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_compose_search.cpp` | 1221 | `3e3a4dd73da5...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_search.rs` | `HistoryViewComposeSearchViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_compose_search.h` | 64 | `a96238b9a987...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_search.rs` | `HistoryViewComposeSearchViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_compose_stash.cpp` | 272 | `8a669680dda5...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_stash.rs` | `HistoryViewComposeStashViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_compose_stash.h` | 86 | `e7f801bae38a...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_stash.rs` | `HistoryViewComposeStashViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_compose_stash_hint.cpp` | 153 | `d8091ebcd9b5...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_stash_hint.rs` | `HistoryViewComposeStashHintViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_compose_stash_hint.h` | 57 | `45e2decb5b54...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_stash_hint.rs` | `HistoryViewComposeStashHintViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_draft_options.cpp` | 1785 | `8c589706958e...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_draft_options.rs` | `HistoryViewDraftOptionsViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_draft_options.h` | 47 | `064aa3587536...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_draft_options.rs` | `HistoryViewDraftOptionsViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_forward_panel.cpp` | 437 | `29831e3aa8bf...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_forward_panel.rs` | `HistoryViewForwardPanelViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_forward_panel.h` | 96 | `9cf95d88e672...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_forward_panel.rs` | `HistoryViewForwardPanelViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_rich_draft_preview.cpp` | 417 | `56026ea9310f...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_rich_draft_preview.rs` | `HistoryViewRichDraftPreviewViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_rich_draft_preview.h` | 100 | `974fcb23f1ac...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_rich_draft_preview.rs` | `HistoryViewRichDraftPreviewViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_suggest_options.cpp` | 975 | `d9ec4e99d852...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_suggest_options.rs` | `HistoryViewSuggestOptionsViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_suggest_options.h` | 144 | `fb3c2ab5a383...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_suggest_options.rs` | `HistoryViewSuggestOptionsViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_top_controls.cpp` | 1250 | `5563bbe1d538...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_top_controls.rs` | `HistoryViewTopControlsViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_top_controls.h` | 180 | `143e4ca306e6...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_top_controls.rs` | `HistoryViewTopControlsViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_ttl_button.cpp` | 68 | `6638b6781aca...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_ttl_button.rs` | `HistoryViewTtlButtonViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_ttl_button.h` | 43 | `710213df980d...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_ttl_button.rs` | `HistoryViewTtlButtonViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_video_cover_uploader.cpp` | 409 | `208a2b794dd4...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_video_cover_uploader.rs` | `HistoryViewVideoCoverUploaderViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_video_cover_uploader.h` | 86 | `45388e3ee7a2...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_video_cover_uploader.rs` | `HistoryViewVideoCoverUploaderViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_voice_record_bar.cpp` | 3389 | `74b50c091cc5...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_voice_record_bar.rs` | `HistoryViewVoiceRecordBarViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_voice_record_bar.h` | 247 | `78f5c78344c7...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_voice_record_bar.rs` | `HistoryViewVoiceRecordBarViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_voice_record_bar.style` | 67 | `1096c4bc4e8d...` | `crates/gpui_telegram_desktop/src/theme/history/history_view_voice_record_bar_style.rs` | `HistoryViewVoiceRecordBarViewControlsStyleSheet` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_voice_record_button.cpp` | 297 | `fcb767052d1c...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_voice_record_button.rs` | `HistoryViewVoiceRecordButtonViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_voice_record_button.h` | 73 | `a0d4fc3fd3b3...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_voice_record_button.rs` | `HistoryViewVoiceRecordButtonViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_webpage_processor.cpp` | 408 | `061b786c81cf...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_webpage_processor.rs` | `HistoryViewWebpageProcessorViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/controls/history_view_webpage_processor.h` | 129 | `bac0e87d0777...` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_webpage_processor.rs` | `HistoryViewWebpageProcessorViewControls` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_about_view.cpp` | 1207 | `a6213615ccf3...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_about_view.rs` | `HistoryViewAboutViewView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_about_view.h` | 89 | `f2eb19ff4e2a...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_about_view.rs` | `HistoryViewAboutViewView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_about_view.style` | 38 | `41973460cbed...` | `crates/gpui_telegram_desktop/src/theme/history/history_view_about_view_style.rs` | `HistoryViewAboutViewViewStyleSheet` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_add_poll_option.cpp` | 565 | `c1979b90929d...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_add_poll_option.rs` | `HistoryViewAddPollOptionView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_add_poll_option.h` | 34 | `c85a20d468e9...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_add_poll_option.rs` | `HistoryViewAddPollOptionView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_chat_preview.cpp` | 1038 | `3126aa9f2da0...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_chat_preview.rs` | `HistoryViewChatPreviewView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_chat_preview.h` | 39 | `928807100632...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_chat_preview.rs` | `HistoryViewChatPreviewView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_chat_section.cpp` | 6344 | `db36c90a4701...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_chat_section.rs` | `HistoryViewChatSectionView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_chat_section.h` | 730 | `77706242e851...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_chat_section.rs` | `HistoryViewChatSectionView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_contact_status.cpp` | 1484 | `47486ba1f626...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_contact_status.rs` | `HistoryViewContactStatusView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_contact_status.h` | 221 | `9077bf83b7cb...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_contact_status.rs` | `HistoryViewContactStatusView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_context_menu.cpp` | 3118 | `f51abb41f5c1...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_context_menu.rs` | `HistoryViewContextMenuView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_context_menu.h` | 183 | `fab79df0b95c...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_context_menu.rs` | `HistoryViewContextMenuView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_emoji_interactions.cpp` | 530 | `32be7f5c642b...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_emoji_interactions.rs` | `HistoryViewEmojiInteractionsView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_emoji_interactions.h` | 147 | `47fc0c633e75...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_emoji_interactions.rs` | `HistoryViewEmojiInteractionsView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_group_call_bar.cpp` | 451 | `8a0aefe478e0...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_group_call_bar.rs` | `HistoryViewGroupCallBarView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_group_call_bar.h` | 65 | `6c27ca0f30e1...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_group_call_bar.rs` | `HistoryViewGroupCallBarView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_group_members_widget.cpp` | 112 | `47e57d7518a5...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_group_members_widget.rs` | `HistoryViewGroupMembersWidgetView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_group_members_widget.h` | 58 | `8f87ba844b44...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_group_members_widget.rs` | `HistoryViewGroupMembersWidgetView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_item_preview.h` | 55 | `7ec2cbd976ff...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_item_preview.rs` | `HistoryViewItemPreviewView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_list_widget.cpp` | 6833 | `99657ddb836f...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_list_widget.rs` | `HistoryViewListWidgetView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_list_widget.h` | 1128 | `67999847d974...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_list_widget.rs` | `HistoryViewListWidgetView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_paid_reaction_toast.cpp` | 432 | `3d4c7e5b3d0e...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_paid_reaction_toast.rs` | `HistoryViewPaidReactionToastView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_paid_reaction_toast.h` | 85 | `299689ad0e55...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_paid_reaction_toast.rs` | `HistoryViewPaidReactionToastView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_pinned_bar.cpp` | 409 | `ddcbe4e977a4...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_pinned_bar.rs` | `HistoryViewPinnedBarView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_pinned_bar.h` | 67 | `bf41a0ecb888...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_pinned_bar.rs` | `HistoryViewPinnedBarView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_pinned_section.cpp` | 853 | `1c098fd8e3ca...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_pinned_section.rs` | `HistoryViewPinnedSectionView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_pinned_section.h` | 246 | `660d91bba7f6...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_pinned_section.rs` | `HistoryViewPinnedSectionView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_pinned_tracker.cpp` | 186 | `806945063b1e...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_pinned_tracker.rs` | `HistoryViewPinnedTrackerView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_pinned_tracker.h` | 63 | `54a1d265459f...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_pinned_tracker.rs` | `HistoryViewPinnedTrackerView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_quick_action.cpp` | 19 | `8e0d8ac68f1e...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_quick_action.rs` | `HistoryViewQuickActionView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_quick_action.h` | 20 | `6f0536fe1169...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_quick_action.rs` | `HistoryViewQuickActionView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_reaction_preview.cpp` | 487 | `802563fce314...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_reaction_preview.rs` | `HistoryViewReactionPreviewView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_reaction_preview.h` | 59 | `fb77485e2f88...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_reaction_preview.rs` | `HistoryViewReactionPreviewView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_read_metrics_tracker.cpp` | 346 | `9a8d6dfabb77...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_read_metrics_tracker.rs` | `HistoryViewReadMetricsTrackerView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_read_metrics_tracker.h` | 79 | `ec6f7a9c83db...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_read_metrics_tracker.rs` | `HistoryViewReadMetricsTrackerView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_requests_bar.cpp` | 209 | `04c42cdcf83d...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_requests_bar.rs` | `HistoryViewRequestsBarView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_requests_bar.h` | 25 | `59891370be7a...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_requests_bar.rs` | `HistoryViewRequestsBarView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_schedule_box.cpp` | 436 | `4ada0f077651...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_schedule_box.rs` | `HistoryViewScheduleBoxView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_schedule_box.h` | 96 | `f7faade52ca7...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_schedule_box.rs` | `HistoryViewScheduleBoxView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_scheduled_section.cpp` | 1882 | `5b78f6b48ba5...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_scheduled_section.rs` | `HistoryViewScheduledSectionView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_scheduled_section.h` | 362 | `da70ddeed2e7...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_scheduled_section.rs` | `HistoryViewScheduledSectionView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_self_forwards_tagger.cpp` | 416 | `8273826bbb6e...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_self_forwards_tagger.rs` | `HistoryViewSelfForwardsTaggerView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_self_forwards_tagger.h` | 83 | `34c76427aecc...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_self_forwards_tagger.rs` | `HistoryViewSelfForwardsTaggerView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_send_action.cpp` | 426 | `c6a6079f3d3e...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_send_action.rs` | `HistoryViewSendActionView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_send_action.h` | 83 | `31c5fd0d0b11...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_send_action.rs` | `HistoryViewSendActionView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_sponsored_click_handler.cpp` | 38 | `40dcb425a93d...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_sponsored_click_handler.rs` | `HistoryViewSponsoredClickHandlerView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_sponsored_click_handler.h` | 18 | `0781a9b871e4...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_sponsored_click_handler.rs` | `HistoryViewSponsoredClickHandlerView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_sticker_toast.cpp` | 355 | `41a6415c7bd5...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_sticker_toast.rs` | `HistoryViewStickerToastView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_sticker_toast.h` | 74 | `ff6c0657f9a2...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_sticker_toast.rs` | `HistoryViewStickerToastView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_subsection_tabs.cpp` | 1018 | `c1af2e004492...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_subsection_tabs.rs` | `HistoryViewSubsectionTabsView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_subsection_tabs.h` | 155 | `b10a9cea4372...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_subsection_tabs.rs` | `HistoryViewSubsectionTabsView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_top_bar_widget.cpp` | 2042 | `1f09aff9fcf0...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_top_bar_widget.rs` | `HistoryViewTopBarWidgetView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_top_bar_widget.h` | 292 | `5d2a7acb002f...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_top_bar_widget.rs` | `HistoryViewTopBarWidgetView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_top_peers_selector.cpp` | 263 | `61926b96573f...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_top_peers_selector.rs` | `HistoryViewTopPeersSelectorView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_top_peers_selector.h` | 25 | `b9c6ec6e2aea...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_top_peers_selector.rs` | `HistoryViewTopPeersSelectorView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_touch_scroll.cpp` | 53 | `c9e4815dad0d...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_touch_scroll.rs` | `HistoryViewTouchScrollView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_touch_scroll.h` | 41 | `386d876de382...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_touch_scroll.rs` | `HistoryViewTouchScrollView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_transcribe_button.cpp` | 448 | `4946e729e360...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_transcribe_button.rs` | `HistoryViewTranscribeButtonView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_transcribe_button.h` | 64 | `8f0cd8c7fa46...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_transcribe_button.rs` | `HistoryViewTranscribeButtonView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_translate_bar.cpp` | 669 | `c463c05220b4...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_translate_bar.rs` | `HistoryViewTranslateBarView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_translate_bar.h` | 79 | `e4448c67557d...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_translate_bar.rs` | `HistoryViewTranslateBarView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_translate_tracker.cpp` | 535 | `5f02e50cb995...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_translate_tracker.rs` | `HistoryViewTranslateTrackerView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_translate_tracker.h` | 88 | `44c73179be8f...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_translate_tracker.rs` | `HistoryViewTranslateTrackerView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_view_button.cpp` | 332 | `70a4089087b7...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_view_button.rs` | `HistoryViewViewButtonView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_view_button.h` | 70 | `ddf2efb9918d...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_view_button.rs` | `HistoryViewViewButtonView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_webpage_preview.cpp` | 91 | `a30ddaaa46c5...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_webpage_preview.rs` | `HistoryViewWebpagePreviewView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_webpage_preview.h` | 24 | `3dd6cbdcbca3...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_webpage_preview.rs` | `HistoryViewWebpagePreviewView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_welcome_messages_section.cpp` | 1265 | `f6362154fbe1...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_welcome_messages_section.rs` | `HistoryViewWelcomeMessagesSectionView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/history_view_welcome_messages_section.h` | 290 | `36f4bed3b7cb...` | `crates/gpui_telegram_desktop/src/views/stage/view/history_view_welcome_messages_section.rs` | `HistoryViewWelcomeMessagesSectionView` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_birthday_suggestion.cpp` | 201 | `2c3b96de51aa...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_birthday_suggestion.rs` | `HistoryViewBirthdaySuggestionViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_birthday_suggestion.h` | 70 | `0e642c3f6445...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_birthday_suggestion.rs` | `HistoryViewBirthdaySuggestionViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_call.cpp` | 138 | `2b0770e0ec93...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_call.rs` | `HistoryViewCallViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_call.h` | 59 | `f355b1da8294...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_call.rs` | `HistoryViewCallViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_community_added.cpp` | 247 | `7bc20bf9269a...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_community_added.rs` | `HistoryViewCommunityAddedViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_community_added.h` | 25 | `10f9d1fac1b3...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_community_added.rs` | `HistoryViewCommunityAddedViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_contact.cpp` | 657 | `4ac15028b0b8...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_contact.rs` | `HistoryViewContactViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_contact.h` | 99 | `09abb614dac2...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_contact.rs` | `HistoryViewContactViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_custom_emoji.cpp` | 332 | `218769f7a363...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_custom_emoji.rs` | `HistoryViewCustomEmojiViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_custom_emoji.h` | 104 | `673f0d79084d...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_custom_emoji.rs` | `HistoryViewCustomEmojiViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_dice.cpp` | 179 | `0fcb98002930...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_dice.rs` | `HistoryViewDiceViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_dice.h` | 69 | `7eeec7fd1c4a...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_dice.rs` | `HistoryViewDiceViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_document.cpp` | 2123 | `02eed760e03d...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_document.rs` | `HistoryViewDocumentViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_document.h` | 200 | `ccd585afe333...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_document.rs` | `HistoryViewDocumentViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_ephemeral_plate.cpp` | 180 | `5df957a54515...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_ephemeral_plate.rs` | `HistoryViewEphemeralPlateViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_ephemeral_plate.h` | 54 | `b4f248bd2cf8...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_ephemeral_plate.rs` | `HistoryViewEphemeralPlateViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_file.cpp` | 181 | `b4095e93dcc3...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_file.rs` | `HistoryViewFileViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_file.h` | 118 | `88331526a7c0...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_file.rs` | `HistoryViewFileViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_game.cpp` | 549 | `8804dc358c5c...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_game.rs` | `HistoryViewGameViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_game.h` | 120 | `8416f799837b...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_game.rs` | `HistoryViewGameViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_gif.cpp` | 2750 | `d2cac99e3928...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_gif.rs` | `HistoryViewGifViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_gif.h` | 304 | `744d575f9d66...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_gif.rs` | `HistoryViewGifViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_giveaway.cpp` | 314 | `220207623756...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_giveaway.rs` | `HistoryViewGiveawayViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_giveaway.h` | 35 | `e3ec68de6eb9...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_giveaway.rs` | `HistoryViewGiveawayViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_gram_transfer.cpp` | 2328 | `faa1f1a7faaa...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_gram_transfer.rs` | `HistoryViewGramTransferViewMedia` | `inherited_rust_reused` |
| `Telegram/SourceFiles/history/view/media/history_view_gram_transfer.h` | 41 | `2e07cc402e98...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_gram_transfer.rs` | `HistoryViewGramTransferViewMedia` | `inherited_rust_reused` |
| `Telegram/SourceFiles/history/view/media/history_view_invoice.cpp` | 468 | `c8abd57c8dd2...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_invoice.rs` | `HistoryViewInvoiceViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_invoice.h` | 106 | `c60fc53c64a6...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_invoice.rs` | `HistoryViewInvoiceViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_large_emoji.cpp` | 187 | `302008d953f6...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_large_emoji.rs` | `HistoryViewLargeEmojiViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_large_emoji.h` | 67 | `917e1da3807b...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_large_emoji.rs` | `HistoryViewLargeEmojiViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_location.cpp` | 846 | `371651eaa0cd...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_location.rs` | `HistoryViewLocationViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_location.h` | 135 | `b1801b915f9c...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_location.rs` | `HistoryViewLocationViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_media.cpp` | 679 | `b8d4502e7171...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media.rs` | `HistoryViewMediaViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_media.h` | 472 | `357c60f98ac0...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media.rs` | `HistoryViewMediaViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_media_common.cpp` | 682 | `f80308d8259f...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_common.rs` | `HistoryViewMediaCommonViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_media_common.h` | 124 | `fb554f3fa26d...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_common.rs` | `HistoryViewMediaCommonViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_media_generic.cpp` | 1154 | `e887749114eb...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_generic.rs` | `HistoryViewMediaGenericViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_media_generic.h` | 449 | `e9b30cff69d2...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_generic.rs` | `HistoryViewMediaGenericViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_media_grouped.cpp` | 961 | `de8cb36f839d...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_grouped.rs` | `HistoryViewMediaGroupedViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_media_grouped.h` | 166 | `4067f01c26ba...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_grouped.rs` | `HistoryViewMediaGroupedViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_media_spoiler.cpp` | 9 | `d4809f690416...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_spoiler.rs` | `HistoryViewMediaSpoilerViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_media_spoiler.h` | 39 | `52069084dda4...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_spoiler.rs` | `HistoryViewMediaSpoilerViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_media_unwrapped.cpp` | 786 | `605c06662da0...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_unwrapped.rs` | `HistoryViewMediaUnwrappedViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_media_unwrapped.h` | 183 | `96a1b43f9e8c...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_unwrapped.rs` | `HistoryViewMediaUnwrappedViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_no_forwards_request.cpp` | 137 | `911a5a00b158...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_no_forwards_request.rs` | `HistoryViewNoForwardsRequestViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_no_forwards_request.h` | 25 | `a13182d6480f...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_no_forwards_request.rs` | `HistoryViewNoForwardsRequestViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_photo.cpp` | 1329 | `aa3679f00500...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_photo.rs` | `HistoryViewPhotoViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_photo.h` | 182 | `817494e99ca9...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_photo.rs` | `HistoryViewPhotoViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_poll.cpp` | 4656 | `a72bf316a73e...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_poll.rs` | `HistoryViewPollViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_poll.h` | 142 | `7c7933e61e5f...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_poll.rs` | `HistoryViewPollViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_premium_gift.cpp` | 613 | `1a3a3317b56a...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_premium_gift.rs` | `HistoryViewPremiumGiftViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_premium_gift.h` | 78 | `2c77531b6525...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_premium_gift.rs` | `HistoryViewPremiumGiftViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_save_document_action.cpp` | 132 | `613aee6f2e73...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_save_document_action.rs` | `HistoryViewSaveDocumentActionViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_save_document_action.h` | 41 | `78915c26b153...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_save_document_action.rs` | `HistoryViewSaveDocumentActionViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_service_box.cpp` | 492 | `5b2ba791d17d...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_service_box.rs` | `HistoryViewServiceBoxViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_service_box.h` | 148 | `36e62442c3de...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_service_box.rs` | `HistoryViewServiceBoxViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_similar_channels.cpp` | 651 | `93353fdc724d...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_similar_channels.rs` | `HistoryViewSimilarChannelsViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_similar_channels.h` | 104 | `bdf657486da8...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_similar_channels.rs` | `HistoryViewSimilarChannelsViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_slot_machine.cpp` | 213 | `ed4b6f3b8488...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_slot_machine.rs` | `HistoryViewSlotMachineViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_slot_machine.h` | 79 | `7be47eb61df6...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_slot_machine.rs` | `HistoryViewSlotMachineViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_sticker.cpp` | 692 | `daf8140fc06f...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_sticker.rs` | `HistoryViewStickerViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_sticker.h` | 157 | `80f8996a0edb...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_sticker.rs` | `HistoryViewStickerViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_sticker_player.cpp` | 168 | `8bd5dc0f34db...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_sticker_player.rs` | `HistoryViewStickerPlayerViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_sticker_player.h` | 92 | `a3c157ebe51a...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_sticker_player.rs` | `HistoryViewStickerPlayerViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_sticker_player_abstract.h` | 33 | `9c7fb48ecf2f...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_sticker_player_abstract.rs` | `HistoryViewStickerPlayerAbstractViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_story_mention.cpp` | 188 | `940e0a3de1f4...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_story_mention.rs` | `HistoryViewStoryMentionViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_story_mention.h` | 69 | `9cd74dd52edf...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_story_mention.rs` | `HistoryViewStoryMentionViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_suggest_decision.cpp` | 369 | `3ed5d227fab8...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_suggest_decision.rs` | `HistoryViewSuggestDecisionViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_suggest_decision.h` | 33 | `d96dcc89d830...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_suggest_decision.rs` | `HistoryViewSuggestDecisionViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_theme_document.cpp` | 894 | `c220bc5027c0...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_theme_document.rs` | `HistoryViewThemeDocumentViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_theme_document.h` | 188 | `7441e0632ef0...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_theme_document.rs` | `HistoryViewThemeDocumentViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_todo_list.cpp` | 912 | `2a6f9450fab3...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_todo_list.rs` | `HistoryViewTodoListViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_todo_list.h` | 152 | `493efca4573a...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_todo_list.rs` | `HistoryViewTodoListViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_unique_gift.cpp` | 1032 | `b1387bf06b31...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_unique_gift.rs` | `HistoryViewUniqueGiftViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_unique_gift.h` | 161 | `a690c2b5b3f5...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_unique_gift.rs` | `HistoryViewUniqueGiftViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_unsupported_notice.cpp` | 125 | `164e0ab66617...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_unsupported_notice.rs` | `HistoryViewUnsupportedNoticeViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_unsupported_notice.h` | 48 | `7f5fa343bd6f...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_unsupported_notice.rs` | `HistoryViewUnsupportedNoticeViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_userpic_suggestion.cpp` | 278 | `bebd745d245d...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_userpic_suggestion.rs` | `HistoryViewUserpicSuggestionViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_userpic_suggestion.h` | 58 | `14bf4ab6b0fa...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_userpic_suggestion.rs` | `HistoryViewUserpicSuggestionViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_video_message_seek.cpp` | 192 | `0280a7145d1c...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_video_message_seek.rs` | `HistoryViewVideoMessageSeekViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_video_message_seek.h` | 60 | `dbc62d5f5600...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_video_message_seek.rs` | `HistoryViewVideoMessageSeekViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_video_status.cpp` | 172 | `a69a265daf9b...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_video_status.rs` | `HistoryViewVideoStatusViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_video_status.h` | 46 | `a70c7a94a29b...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_video_status.rs` | `HistoryViewVideoStatusViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_web_page.cpp` | 1916 | `9ceee3a53cba...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_web_page.rs` | `HistoryViewWebPageViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_web_page.h` | 229 | `c164f015d4e0...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_web_page.rs` | `HistoryViewWebPageViewMedia` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/menu/history_view_poll_menu.cpp` | 802 | `33abccdc4874...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/menu/history_view_poll_menu.rs` | `HistoryViewPollMenuViewMediaMenu` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/menu/history_view_poll_menu.h` | 99 | `28faf1757b76...` | `crates/gpui_telegram_desktop/src/views/stage/view/media/menu/history_view_poll_menu.rs` | `HistoryViewPollMenuViewMediaMenu` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/reactions/history_view_reactions.cpp` | 1011 | `ecf1a008c0a9...` | `crates/gpui_telegram_desktop/src/views/stage/view/reactions/history_view_reactions.rs` | `HistoryViewReactionsViewReactions` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/reactions/history_view_reactions.h` | 175 | `06e1b7db88fc...` | `crates/gpui_telegram_desktop/src/views/stage/view/reactions/history_view_reactions.rs` | `HistoryViewReactionsViewReactions` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/reactions/history_view_reactions_button.cpp` | 924 | `c8537a01d806...` | `crates/gpui_telegram_desktop/src/views/stage/view/reactions/history_view_reactions_button.rs` | `HistoryViewReactionsButtonViewReactions` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/reactions/history_view_reactions_button.h` | 262 | `abe868da9baa...` | `crates/gpui_telegram_desktop/src/views/stage/view/reactions/history_view_reactions_button.rs` | `HistoryViewReactionsButtonViewReactions` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/reactions/history_view_reactions_list.cpp` | 688 | `220496543e20...` | `crates/gpui_telegram_desktop/src/views/stage/view/reactions/history_view_reactions_list.rs` | `HistoryViewReactionsListViewReactions` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/reactions/history_view_reactions_list.h` | 86 | `10f8815df4b5...` | `crates/gpui_telegram_desktop/src/views/stage/view/reactions/history_view_reactions_list.rs` | `HistoryViewReactionsListViewReactions` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/reactions/history_view_reactions_selector.cpp` | 1463 | `cb1b767a9ad7...` | `crates/gpui_telegram_desktop/src/views/stage/view/reactions/history_view_reactions_selector.rs` | `HistoryViewReactionsSelectorViewReactions` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/reactions/history_view_reactions_selector.h` | 280 | `73d212cd1b98...` | `crates/gpui_telegram_desktop/src/views/stage/view/reactions/history_view_reactions_selector.rs` | `HistoryViewReactionsSelectorViewReactions` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/reactions/history_view_reactions_strip.cpp` | 607 | `b44997ffd366...` | `crates/gpui_telegram_desktop/src/views/stage/view/reactions/history_view_reactions_strip.rs` | `HistoryViewReactionsStripViewReactions` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/reactions/history_view_reactions_strip.h` | 181 | `775cc72775e4...` | `crates/gpui_telegram_desktop/src/views/stage/view/reactions/history_view_reactions_strip.rs` | `HistoryViewReactionsStripViewReactions` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/reactions/history_view_reactions_tabs.cpp` | 214 | `a8be4b4a8263...` | `crates/gpui_telegram_desktop/src/views/stage/view/reactions/history_view_reactions_tabs.rs` | `HistoryViewReactionsTabsViewReactions` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/reactions/history_view_reactions_tabs.h` | 38 | `d9c503492e2f...` | `crates/gpui_telegram_desktop/src/views/stage/view/reactions/history_view_reactions_tabs.rs` | `HistoryViewReactionsTabsViewReactions` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/poll/poll_link_box.cpp` | 69 | `f78f07402618...` | `crates/gpui_telegram_desktop/src/views/stage/poll_link_box.rs` | `PollLinkBox` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/poll/poll_link_box.h` | 21 | `7c78ad2315c0...` | `crates/gpui_telegram_desktop/src/views/stage/poll_link_box.rs` | `PollLinkBox` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/poll/poll_link_thumbnail.cpp` | 65 | `e2eda8ecfcc5...` | `crates/gpui_telegram_desktop/src/views/stage/poll_link_thumbnail.rs` | `PollLinkThumbnail` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/poll/poll_link_thumbnail.h` | 16 | `64426a2c4607...` | `crates/gpui_telegram_desktop/src/views/stage/poll_link_thumbnail.rs` | `PollLinkThumbnail` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/poll/poll_media_upload.cpp` | 1065 | `04817af34f42...` | `crates/gpui_telegram_desktop/src/views/stage/poll_media_upload.rs` | `PollMediaUpload` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/poll/poll_media_upload.h` | 226 | `ced52d34595b...` | `crates/gpui_telegram_desktop/src/views/stage/poll_media_upload.rs` | `PollMediaUpload` | `gpui_ui_1to1` |
