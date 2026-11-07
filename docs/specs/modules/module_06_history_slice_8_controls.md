# Module 06 Specification: History View Controls Subsystem (`history_slice_8_controls`)

## Executive Summary & Mandate Protocols

This specification defines the complete Rust GPUI architecture and implementation plan for **Slice 8: History Compose Controls, Voice Record Bar & Draft Options** of Module 06 (History Subsystem) in `gpui_telegram_desktop`, faithfully reconstructing Telegram Desktop's input area and auxiliary chat controls in strict adherence to:
- **Mandate #15 (Zero Tolerance for Boilerplate)**: Complete elimination of all 21 placeholder boilerplate files (`pub label: String, pub value: i64`, Patterns A/B/C/D) in `crates/gpui_telegram_desktop/src/views/stage/view/controls/` and delivery of authentic GPUI composer controls in `crates/gpui_telegram_desktop/src/views/composer/controls/`.
- **Mandate #17 (Dynamic Target File Architecture)**: Clean separation of live GPUI UI view components (`composer/controls/`) and stage history view control models (`stage/view/controls/`), ensuring 100% type safety and bidirectional event coordination.
- **Mandate #18 (Anti-Bluffing Protocol)**: Every component, timing parameter, and gesture threshold is cited directly from actual C++ line ranges (`.cpp:L<start>-L<end>`, `.h:L<start>-L<end>`, `.style:L<start>-L<end>`) in `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/history/view/controls/`.
- **Mandate #19 Pillar 2 (100% Reading Manifest)**: Exhaustive documentation of all **42 assigned upstream C++ source and style files** without omission or truncation.

---

## 1. 100% Upstream C++ Reading Manifest (42 Assigned Files)

The following table provides the exhaustive manifest of all 42 source files assigned to Slice 8 in `Telegram/SourceFiles/history/view/controls/`:

| # | Upstream Source Path | Lines | Key Symbols / Classes | Constants & Metrics | Target Rust GPUI Path | Disposition |
|---|---|---|---|---|---|---|
| 1 | `compose_controls_common.h` | 114 | `MessageToEdit`, `VoiceToSend`, `SendActionUpdate`, `WriteRestrictionType`, `WriteRestriction`, `SetHistoryArgs`, `ReplyNextRequest`, `ToggleCommentsState`, `SendStarButtonEffect` | `kOptionMacCmdReplyImmediately` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/compose_controls_common.rs` | 1:1 Domain Model & Contracts |
| 2 | `history_view_bot_menu_button.h` | 45 | `BotMenuButton` | Text fade animations, dynamic sizing | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_bot_menu_button.rs` | 1:1 GPUI Component |
| 3 | `history_view_bot_menu_button.cpp` | 300 | `BotMenuButton::refresh`, `BotMenuButton::animateSize`, `BotMenuIconEmoji` | `RoundButton`, custom emoji integration | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_bot_menu_button.rs` | 1:1 Implementation |
| 4 | `history_view_bottom_controls.h` | 146 | `BottomControls`, `BottomControlsMode`, `BottomControlsAction`, `BottomControlsDescriptor` | Action modes (History, Replies, Sublist) | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_bottom_controls.rs` | 1:1 GPUI Component |
| 5 | `history_view_bottom_controls.cpp` | 616 | `BottomControls::setupButtons`, `BottomControls::updateControlsVisibility`, `BottomControls::recomputeContentHeight` | Button states: Unblock, BotStart, JoinChannel, JoinGroup, MuteUnmute, Report | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_bottom_controls.rs` | 1:1 Implementation |
| 6 | `history_view_characters_limit.h` | 31 | `CharactersLimitLabel` | Alignment (Top, Bottom), margins | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_characters_limit.rs` | 1:1 GPUI Component |
| 7 | `history_view_characters_limit.cpp` | 71 | `CharactersLimitLabel::setLeft`, `CountDigits` | `kMinus = 0x2212`, `kLimit = 999`, red warning styling | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_characters_limit.rs` | 1:1 Implementation |
| 8 | `history_view_compose_ai_button.h` | 59 | `ComposeAiButton` | Star animation opacity, premium star overlay | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_ai_button.rs` & `composer/controls/ai_button.rs` | 1:1 GPUI Component |
| 9 | `history_view_compose_ai_button.cpp` | 177 | `ComposeAiButton::paintIcons`, `ComposeAiButton::validateFrame` | `kAnimationDuration = 640ms`, 4-phase sparkle cycle | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_ai_button.rs` & `composer/controls/ai_button.rs` | 1:1 Implementation |
| 10 | `history_view_compose_ai_tooltip.h` | 45 | `ComposeTooltipManager`, `AiTooltipManager` | Local preference key memory, geometry updates | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_ai_tooltip.rs` & `composer/controls/ai_button.rs` | 1:1 GPUI Component |
| 11 | `history_view_compose_ai_tooltip.cpp` | 90 | `ComposeTooltipManager::hideAndRemember`, `updateVisibility`, `updateGeometry` | `st::historyAiComposeTooltipSkip`, `writePref` persistence | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_ai_tooltip.rs` & `composer/controls/ai_button.rs` | 1:1 Implementation |
| 12 | `history_view_compose_controls.h` | 695 | `ComposeControls`, `ComposeControlsDescriptor`, `ComposeControlsMode` | Send menu details, slowmode timer, star reactions | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_controls.rs` & `composer/controls/compose_controls.rs` | 1:1 Orchestrator & View |
| 13 | `history_view_compose_controls.cpp` | 6646 | `ComposeControls::init`, `setHistory`, `updateFeatures`, `prepareOnSendPress` | Dynamic field resizing, autocomplete bounding, star boosts | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_controls.rs` & `composer/controls/compose_controls.rs` | 1:1 Implementation |
| 14 | `history_view_compose_media_edit_manager.h` | 93 | `MediaEditManager` | Spoilered flag, invert caption flag, video cover edit | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_media_edit_manager.rs` | 1:1 Domain Manager |
| 15 | `history_view_compose_media_edit_manager.cpp` | 278 | `MediaEditManager::start`, `apply`, `showMenu` | SendMenu integration, cover upload pipeline | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_media_edit_manager.rs` | 1:1 Implementation |
| 16 | `history_view_compose_search.h` | 64 | `ComposeSearch`, `Activation` | Search in chat, jump handlers, calendar filter | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_search.rs` | 1:1 Search Bar |
| 17 | `history_view_compose_search.cpp` | 1221 | `ComposeSearch::Inner`, `setQuery`, `setSearchFilter` | Query debounce, filter pills, jump navigation | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_search.rs` | 1:1 Implementation |
| 18 | `history_view_compose_stash.h` | 86 | `StashManager`, `StashManagerDescriptor` | Draft stashing, exchange drafts, take from send box | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_stash.rs` | 1:1 Stash Manager |
| 19 | `history_view_compose_stash.cpp` | 272 | `StashManager::exchange`, `applyStash`, `updateButton` | Stash storage coordination, file list preservation | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_stash.rs` | 1:1 Implementation |
| 20 | `history_view_compose_stash_hint.h` | 57 | `StashHintManager`, `StashHintDescriptor`, `CutMemory` | Toast prompt for cut draft preservation | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_stash_hint.rs` | 1:1 Hint Component |
| 21 | `history_view_compose_stash_hint.cpp` | 153 | `StashHintManager::trackChange`, `maybeShow`, `markUsed` | Cut memory threshold (peak length vs current length) | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_compose_stash_hint.rs` | 1:1 Implementation |
| 22 | `history_view_draft_options.h` | 47 | `EditDraftOptionsArgs`, `EditDraftOptions`, `ShowReplyToChatBox` | Edit options for reply, forward, and link previews | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_draft_options.rs` & `composer/controls/draft_options.rs` | 1:1 Options Controller |
| 23 | `history_view_draft_options.cpp` | 1785 | `PreviewWrap`, `NormalizeForwardOptions`, `HighlightParsedLinks` | `Section` FSM (Reply/Forward/Link), `InvertMedia`, quote selector | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_draft_options.rs` & `composer/controls/draft_options.rs` | 1:1 Implementation |
| 24 | `history_view_forward_panel.h` | 96 | `ForwardPanel`, `NormalizeForwardOptions`, `CanHideForwardAuthor` | Forward sender attribution, caption hiding | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_forward_panel.rs` & `composer/controls/forward_panel.rs` | 1:1 Panel Component |
| 25 | `history_view_forward_panel.cpp` | 437 | `ForwardPanel::updateTexts`, `ForwardPanel::editToNextOption` | `kNameWithCaptionsVersion = -2`, `kNameNoCaptionsVersion = -3` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_forward_panel.rs` & `composer/controls/forward_panel.rs` | 1:1 Implementation |
| 26 | `history_view_rich_draft_preview.h` | 100 | `RichDraftPreview` | Instant View / article rich draft preview inside composer | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_rich_draft_preview.rs` | 1:1 Preview View |
| 27 | `history_view_rich_draft_preview.cpp` | 417 | `RichDraftPreview::rebuildPreparedContent`, `resizeGetHeight` | Markdown article formatting, fade pixmap gradient | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_rich_draft_preview.rs` | 1:1 Implementation |
| 28 | `history_view_suggest_options.h` | 144 | `SuggestOptionsBar`, `SuggestMode`, `StarsTonPriceInput` | Suggested posts, stars/ton price picker | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_suggest_options.rs` | 1:1 Suggestion Bar |
| 29 | `history_view_suggest_options.cpp` | 975 | `SuggestOptionsBar::paintBar`, `PriceAfterCommission` | Commission percentage calculation, custom emoji tag | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_suggest_options.rs` | 1:1 Implementation |
| 30 | `history_view_top_controls.h` | 180 | `TopControls`, `TopControlsDescriptor` | Pinned messages, group call bar, join requests, topics | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_top_controls.rs` | 1:1 Header Bar |
| 31 | `history_view_top_controls.cpp` | 1250 | `TopControls::setupRootView`, `updatePinnedViewer` | Elastic scroll offset sync, translation tracker | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_top_controls.rs` | 1:1 Implementation |
| 32 | `history_view_ttl_button.h` | 43 | `TTLButton` | Autodelete/TTL period badge | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_ttl_button.rs` | 1:1 TTL Button |
| 33 | `history_view_ttl_button.cpp` | 68 | `TTLButton::TTLButton`, `FormatTTLTiny` | TTL validator trigger, dynamic tiny label (e.g. 1d, 1w) | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_ttl_button.rs` | 1:1 Implementation |
| 34 | `history_view_video_cover_uploader.h` | 86 | `VideoCoverUploader` | Video message custom thumbnail cover upload | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_video_cover_uploader.rs` | 1:1 Uploader Model |
| 35 | `history_view_video_cover_uploader.cpp` | 409 | `VideoCoverUploader::startUpload`, `prepared`, `uploaded` | Radial progress animation, upload generation ID | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_video_cover_uploader.rs` | 1:1 Implementation |
| 36 | `history_view_voice_record_bar.h` | 247 | `VoiceRecordBar`, `VoiceRecordBarDescriptor`, `StopType` | Waveform, duration timer, cancel/lock gesture | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_voice_record_bar.rs` & `composer/controls/voice_record_bar.rs` | 1:1 Voice Bar Component |
| 37 | `history_view_voice_record_bar.cpp` | 3389 | `VoiceRecordBar::calcLockProgress`, `recordUpdated`, `stop` | `kAudioVoiceUpdateView = 200ms`, `kAudioVoiceMaxLength = 6000s`, `kAnimationThreshold = 0.35` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_voice_record_bar.rs` & `composer/controls/voice_record_bar.rs` | 1:1 Implementation |
| 38 | `history_view_voice_record_bar.style` | 67 | Record bar style tokens | `historyRecordLockSize = (75, 133)`, `historyRecordSignalRadius = 5px`, `historyRecordCancelButtonWidth = 100px` | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_voice_record_bar.rs` & `composer/controls/voice_record_bar.rs` | Style & Metrics Constants |
| 39 | `history_view_voice_record_button.h` | 73 | `VoiceRecordButton`, `Type` | Recording wave level pulsating button | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_voice_record_button.rs` & `composer/controls/voice_record_bar.rs` | 1:1 Pulsating Button |
| 40 | `history_view_voice_record_button.cpp` | 297 | `VoiceRecordButton::paintEvent`, `inCircle` | Waveform level circle hit-testing, amplitude pulse | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_voice_record_button.rs` & `composer/controls/voice_record_bar.rs` | 1:1 Implementation |
| 41 | `history_view_webpage_processor.h` | 129 | `WebpageProcessor`, `WebpageResolver`, `WebPageText` | Link extraction from composer input field | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_webpage_processor.rs` | 1:1 Webpage Resolver |
| 42 | `history_view_webpage_processor.cpp` | 408 | `WebpageProcessor::apply`, `checkPreview`, `TitleAndDescriptionFromWebPage` | Debounced URL fetching, title & description extraction | `crates/gpui_telegram_desktop/src/views/stage/view/controls/history_view_webpage_processor.rs` | 1:1 Implementation |

Total Lines of Upstream C++ & Style Read: **21,906 lines** across 42 files.

---

## 2. Deep Architectural Analysis & Upstream Citations (Mandate #18 Proof)

### 2.1 Voice Recording Bar & Gesture State Machine
- **Upstream Files**: `history_view_voice_record_bar.cpp:L50-L3389`, `history_view_voice_record_bar.h:L1-L248`, `history_view_voice_record_bar.style:L1-L68`
- **Core Timing Constants**:
  - `kAudioVoiceUpdateView = crl::time(200)` (`history_view_voice_record_bar.cpp:L60`): Timer tick interval for refreshing recording duration and animated sound wave levels.
  - `kAudioVoiceMaxLength = 100 * 60` (`history_view_voice_record_bar.cpp:L61`): Maximum voice/video note length of 100 minutes (6,000 seconds).
  - `kLockArcAngle = 15.0` (`history_view_voice_record_bar.cpp:L69`): Curvature angle of the animated lock shackle.
  - `kAnimationThreshold = 0.35` (`history_view_voice_record_bar.cpp:L2690`): Threshold ratio above which upward vertical drag immediately triggers the lock animation.
  - `kPrecision = 10` (`history_view_voice_record_bar.cpp:L67`): Level quantizer for normalized audio amplitude.
  - `kHideWaveformBgOffset = 50` (`history_view_voice_record_bar.cpp:L71`): Horizontal offset for hiding background waveform.
  - `kTrimPlaybackEpsilon = 0.0001` (`history_view_voice_record_bar.cpp:L72`): Boundary tolerance for audio trimming playback cursor.
- **Style Metrics (`history_view_voice_record_bar.style:L14-L67`)**:
  - `historyRecordSignalRadius = 5px`: Red recording signal dot radius.
  - `historyRecordFont = 13px`: Duration counter font size.
  - `historyRecordDurationSkip = 12px`: Spacing between recording indicator and duration text.
  - `historyRecordTextLeft = 15px`, `historyRecordTextRight = 25px`: Margins for slide-to-cancel prompt.
  - `historyRecordLockSize = (75px, 133px)`: Total bounding box of floating lock slider.
  - `historyRecordLockIconSize = (14px, 17px)`: Padlock icon dimensions.
  - `historyRecordCancelButtonWidth = 100px`: Width allocated for the cancel ripple button.
  - `historyRecordWaveformBgRadius = 7px`, `historyRecordWaveformBar = 3px`: Audio waveform visualization geometry.
- **Gesture Mechanics**:
  - **Swipe Up to Lock**: `calcLockProgress(globalPos) = (start_y - current_y) / lock_height` (`history_view_voice_record_bar.cpp:L3244-3249`). When progress >= 1.0 (or captures past 0.35), the recording enters hands-free `Locked` mode, replacing the hold gesture with a dedicated Stop / Send / Pause button.
  - **Swipe Left to Cancel**: Cursor tracked against `_inField = rect().contains(localPos) || _level->inCircle(...)` (`history_view_voice_record_bar.cpp:L2730-2735`). If the user drags left past the cancel threshold (out of field), releasing mouse triggers `StopType::Cancel` and discards audio buffers. Releasing within bounds triggers `StopType::Send`.
  - **TTL One-Time Voice**: `peekTTLState()` / `takeTTLState()` (`history_view_voice_record_bar.cpp:L3251-3262`): Interrogates TTL toggle to tag voice message as "View Once" (destructs immediately after playback).

### 2.2 Character Limitation Warnings
- **Upstream Files**: `history_view_characters_limit.cpp:L13-L72`, `history_view_characters_limit.h:L1-L32`
- **Internal Constants**:
  - `kMinus = QChar(0x2212)`: Mathematical unicode minus sign `−` (`history_view_characters_limit.cpp:L15`).
  - `kLimit = 999`: Maximum display limit for over-character counter (`history_view_characters_limit.cpp:L16`).
  - `CountDigits(n)`: Logarithmic digit counting (`history_view_characters_limit.cpp:L18-20`).
- **Behavioral Logic**:
  - When `value <= 0`: User has characters remaining (`-value` characters left). Text is displayed in muted secondary color (`st::windowSubTextFg`).
  - When `value > 0`: User has exceeded maximum character limit. Label text switches to `kMinus + min(value, 999)` (e.g. `−42`) with prominent error highlighting (`st::historyCharsLimitationLabel.textFg`).

### 2.3 Compose AI Sparkle & Tooltip Persistence
- **Upstream Files**: `history_view_compose_ai_button.cpp:L15-L98`, `history_view_compose_ai_tooltip.cpp:L17-L91`
- **Animation Cycle**:
  - `kAnimationDuration = 640ms` (`history_view_compose_ai_button.cpp:L17`).
  - 4-phase opacity interpolation for dual sparkle stars (`history_view_compose_ai_button.cpp:L77-90`):
    - `0.00 <= p < 0.25`: `star1Opacity = 1.0 - (p / 0.25)`, `star2Opacity = 1.0`
    - `0.25 <= p < 0.50`: `star1Opacity = 0.0`, `star2Opacity = 1.0 - ((p - 0.25) / 0.25)`
    - `0.50 <= p < 0.75`: `star1Opacity = (p - 0.50) / 0.25`, `star2Opacity = 0.0`
    - `0.75 <= p <= 1.00`: `star1Opacity = 1.0`, `star2Opacity = (p - 0.75) / 0.25`
- **Tooltip Local Memory**:
  - `hideAndRemember()` checks `!Core::App().settings().readPref<bool>(_prefKey)` and writes `true` on dismissal (`history_view_compose_ai_tooltip.cpp:L48-54`).
  - `st::historyAiComposeTooltipSkip = 8px`: Floating tooltip offset above AI button.

### 2.4 Draft Options Synchronization FSM
- **Upstream Files**: `history_view_draft_options.cpp:L61-L420`, `history_view_draft_options.h:L23-L48`
- **FSM Sections**: `enum class Section { Reply, Forward, Link }` (`history_view_draft_options.cpp:L64-68`).
- **Forward Options Normalization**:
  - `Options::PreserveInfo`: Full forward attribution preserved.
  - `Options::NoNames`: Forward attribution stripped (`dropNames = true`).
  - `Options::NoNamesAndCaptions`: Sender name and media captions stripped (`dropNames = true, dropCaptions = true`).
- **WebPage Preview Options**:
  - `invertMedia`: Positions link preview above message text instead of below (`history_view_draft_options.cpp:L376`).
  - `forceLargeMedia`: Toggles between expanded full-width media banner and compact side-thumbnail preview.

### 2.5 Forward Panel Multi-Option Cycle
- **Upstream Files**: `history_view_forward_panel.cpp:L39-L100`, `history_view_forward_panel.h:L34-L97`
- **Internal Versions**:
  - `kUnknownVersion = -1`, `kNameWithCaptionsVersion = -2`, `kNameNoCaptionsVersion = -3`.
- **Options Carousel**:
  - Tapping or clicking the forward header invokes `editToNextOption()`, cycling seamlessly between:
    1. "Forward 3 messages from Alice" (PreserveInfo)
    2. "Forward 3 messages (hide author name)" (NoNames)
    3. "Forward 3 messages (hide name and captions)" (NoNamesAndCaptions)

---

## 3. Rust GPUI Target Subsystem Architecture

The target implementation organizes Slice 8 into two complementary, mutually supportive tiers:

1. **`crates/gpui_telegram_desktop/src/views/composer/controls/`**:
   - `compose_controls.rs`: Master GPUI composition orchestrator component supporting text fields, attachments, voice bar toggle, reply preview banners, send options dropdown, and scheduled send.
   - `voice_record_bar.rs`: Interactive voice & video note recording bar with 200ms timer, waveform level collector, swipe-to-lock, swipe-to-cancel, and view-once TTL toggle.
   - `draft_options.rs`: Interactive dialog/popover for tuning reply quote selections, forward privacy options, and link preview positioning.
   - `forward_panel.rs`: Live forward preview bar showing source channel/user attribution with quick-cycle toggle.
   - `ai_button.rs`: AI compose sparkle button with 640ms phase animation and persistent first-time tooltip.
   - `mod.rs`: Re-exports and wires controls into `views::composer`.

2. **`crates/gpui_telegram_desktop/src/views/stage/view/controls/`**:
   - 21 authentic domain and GPUI models replacing all 21 autogenerated stubs:
     - `compose_controls_common.rs`
     - `history_view_bot_menu_button.rs`
     - `history_view_bottom_controls.rs`
     - `history_view_characters_limit.rs`
     - `history_view_compose_ai_button.rs`
     - `history_view_compose_ai_tooltip.rs`
     - `history_view_compose_controls.rs`
     - `history_view_compose_media_edit_manager.rs`
     - `history_view_compose_search.rs`
     - `history_view_compose_stash.rs`
     - `history_view_compose_stash_hint.rs`
     - `history_view_draft_options.rs`
     - `history_view_forward_panel.rs`
     - `history_view_rich_draft_preview.rs`
     - `history_view_suggest_options.rs`
     - `history_view_top_controls.rs`
     - `history_view_ttl_button.rs`
     - `history_view_video_cover_uploader.rs`
     - `history_view_voice_record_bar.rs`
     - `history_view_voice_record_button.rs`
     - `history_view_webpage_processor.rs`
     - `mod.rs`

All `pub label: String, pub value: i64` boilerplate patterns are completely eliminated (0 count).

---

## 4. Verification & Testing Matrix

Subsystem tests are centralized in `crates/gpui_telegram_desktop/tests/history_controls_subsystem_tests.rs`:
- `test_voice_record_gesture_lock_and_cancel_thresholds`: Verifies upward swipe progress reaches locked state at >= 1.0 or past 0.35 capture, and horizontal swipe left cancels.
- `test_voice_record_duration_timer_and_max_length`: Verifies 200ms tick updates, 6000s max cap, and waveform accumulation.
- `test_characters_limit_label_formatting`: Verifies positive remaining count formatting, negative over-limit formatting with unicode `−`, and 999 cap.
- `test_compose_ai_sparkle_animation_cycle`: Verifies 640ms cycle with 4-phase star opacity calculations and tooltip persistence.
- `test_forward_panel_options_carousel`: Verifies cycling across PreserveInfo -> NoNames -> NoNamesAndCaptions.
- `test_draft_options_fsm_sections`: Verifies Reply, Forward, and Link configuration state transitions.
- `test_write_restrictions_and_send_progress`: Verifies SendActionUpdate, SendProgressType, and restriction barriers (Premium, Frozen, Rights).
- `test_zero_boilerplate_stubs_in_history_view_controls`: Programmatic verification that zero `pub label: String, pub value: i64` patterns remain in `src/views/stage/view/controls/`.
