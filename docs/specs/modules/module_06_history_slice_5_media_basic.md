# Module 06 — History Slice 5: Media Basic Views Specification
## Documents, Photos, GIFs, Stickers, Grouped Media & Spoilers

### 1. Architectural Overview & Responsibilities

History Slice 5 implements the core media presentation views of the Telegram history chat stage, matching the C++ upstream implementation in `Telegram/SourceFiles/history/view/media/`. In Telegram Desktop, history media objects represent the visual contents embedded inside message bubbles or rendered as unwrapped standalone elements (such as stickers and custom emoji).

The core responsibilities of this slice include:
1. **Base Media View Abstraction (`HistoryView::Media`)**: Sizing calculation (`countOptimalSize`, `countCurrentSize`), coordinate mapping, text state hit testing, selection handling, bubble margin adjustment, and timestamp extraction (`history_view_media.cpp:L44-L95`).
2. **File & Document Bubble Presentation (`HistoryView::File`, `HistoryView::Document`)**: Download/upload status progress indicators, status size encoding (`0xFFFFFFF0LL`, `0xFFFFFFF1LL`, `0xFFFFFFF2LL`), audio and voice message blob animations with organic deformers (`history_view_document.cpp:L50-L81`), song title formatting, and TTL badges.
3. **Photo View Rendering (`HistoryView::Photo`)**: Aspect ratio bounding, 9:16 story sizing (`kStoryWidth = 720, kStoryHeight = 1280`), bubble-bounded scaling, photo crop adjustments, and enlarged previews (`history_view_photo.cpp:L53-L54, L330-L425`).
4. **GIF & Video Playback Views (`HistoryView::Gif`)**: Inline video dimension constraints (`kMaxInlineArea = 1920 * 1080`), corner download/status badges, playback progress tracking, seek thumbnail intervals, and video message seeking (`history_view_gif.cpp:L75-L80`, `history_view_video_status.cpp:L20-L59`, `history_view_video_message_seek.cpp:L19-L26`).
5. **Sticker Presentation & Playback (`HistoryView::Sticker`, `StickerPlayer`)**: Unwrapped bubble-less media rendering, sticker sizing rules (`kMaxSizeFixed = 512, kMaxEmojiSizeFixed = 256, kPremiumMultiplier = 1.49`), dice animation frame caching, and multi-format sticker player backends (`LottiePlayer`, `WebmPlayer`, `StaticStickerPlayer`).
6. **Grouped Media Mosaic Layout (`HistoryView::GroupedMedia`)**: 10-item maximum collage grouping (`kMaxSize = 10`), mode detection (`Mode::Grid` for visual media vs `Mode::Column` for documents/playlists), mosaic proportional scaling math (`newWidth / maxWidth`), and edge-side coordinate alignment with `RectParts` (`history_view_media_grouped.cpp:L71-L258`).
7. **Media Spoiler & Paid/Sensitive Tags (`MediaSpoiler`, `MediaSpoilerTag`)**: Shimmering particle dust animation, 60-frame looping dust simulation, smooth reveal animation, and pill badge overlays for sensitive content and Telegram Stars paid media (`history_view_media.cpp:L346-L515`).

---

### 2. 100% Upstream Source Reading Manifest (33 / 33 Files)

The following manifest documents all 33 upstream C++ source files assigned to Slice 5:

| Source Path | Line Count | Core Symbols | Key Constants / Logic Extracted | Target Rust Path | Disposition |
|---|---|---|---|---|---|
| `Telegram/SourceFiles/history/view/media/history_view_media.h` | 473 | `Media`, `MediaInBubbleState`, `PaidInformation`, `TodoTaskInfo` | Virtual methods for bubble geometry, selection, spoiler tags, play/stop animation | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_media.cpp` | 680 | `DurationForTimestampLinks`, `TimestampLinkBase`, `TimeFromMatch`, `Media::fillImageSpoiler`, `Media::drawSpoilerTag` | `kMaxYouTubeTimestampDuration = 360,000s`, timestamp regex, spoiler opacity decay `1.0 - revealed`, sensitive tag width skip `width * 1.4` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_media_common.h` | 125 | `TtlPaintState`, `CreateAttach`, `NonEmptySize`, `DownscaledSize`, `PrepareWithBlurredBackground` | Aspect ratio scaling, `max(w, 1), max(h, 1)`, TTL countdown state holder | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_common.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_media_common.cpp` | 683 | `AgeVerifyAbout`, `ShowSensitiveConfirm`, `MakeSensitiveMediaLink`, `ShowAgeVerification` | `kMediaUnlockedTooltipDuration = 5s`, sensitive media confirmation modal, age verify bot workflow | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_common.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_media_generic.h` | 450 | `MediaGenericPart`, `MediaGenericDescriptor`, `MediaGeneric` | Composable multi-part media layout, background paint factories, selection dispatch | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_generic.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_media_generic.cpp` | 1154 | `MediaGeneric::countOptimalSize`, `MediaGeneric::draw`, `MediaGeneric::textState` | Service bubble geometry, minimum widths, full-area click link dispatch | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_generic.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_media_grouped.h` | 167 | `GroupedMedia`, `GroupedMedia::Part`, `GroupedMedia::Mode` | `kMaxSize = 10`, `Grid` vs `Column` mode detection, per-part geometry caching | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_grouped.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_media_grouped.cpp` | 962 | `GroupedMedia::countOptimalSize`, `GroupedMedia::countCurrentSize`, `LayoutPlaylist` | Proportional width factor `newWidth / maxWidth`, delta spacing scaling, side bitmasks (`Top/Bottom/Left/Right`) | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_grouped.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_media_spoiler.h` | 40 | `MediaSpoiler`, `MediaSpoilerTag` | Particle animation holder, background caching, bitfield fields (`price: 63, sensitive: 1`) | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_spoiler.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_media_spoiler.cpp` | 10 | Includes & linkage | Clean translation unit for spoiler struct definitions | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_spoiler.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_media_unwrapped.h` | 184 | `UnwrappedMedia`, `UnwrappedMedia::Content` | `needsBubble = false`, `unwrapped = true`, `drawsOwnEphemeralBadge = true`, sticker & reaction bounds | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_unwrapped.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_media_unwrapped.cpp` | 786 | `UnwrappedMedia::draw`, `UnwrappedMedia::pointState`, `UnwrappedMedia::textState` | Isolated item text layout, timestamp overlay positioning, reply/forward panel anchoring | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_media_unwrapped.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_document.h` | 201 | `Document`, `TtlPaintCallback` | Document bubble view, voice/music progress callbacks, monospace width contribution | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_document.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_document.cpp` | 2124 | `Document::draw`, `VoicePlaybackBlobs`, `TTLRectFromInner` | `kVoiceBlobMinorScale = 0.88`, `kVoiceBlobMajorScale = 0.85`, `kVoiceBlobLevelDuration = 265ms`, `kVoiceBlobAlpha = 0.25`, 6 & 8 blob segments | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_document.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_photo.h` | 183 | `Photo`, `Photo::Enlarge` | Photo thumbnail sizing, spoiler tag drawing, userpic frame painting | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_photo.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_photo.cpp` | 1330 | `PhotoDesiredMediaSize`, `Photo::countOptimalSize`, `Photo::countCurrentSize`, `adjustHeightForLessCrop` | `kStoryWidth = 720`, `kStoryHeight = 1280` (9:16 aspect ratio), bubble min width clamping, thumbnail expansion | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_photo.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_file.h` | 119 | `File`, `FileClickHandlerPtr` | Status size codes: `>=0` loaded bytes, `<0` played secs, `0xFFFFFFF0` not downloaded, `0xFFFFFFF1` downloaded, `0xFFFFFFF2` failed | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_file.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_file.cpp` | 181 | `File::setStatusSize`, `File::setDocumentLinks` | Status string builder, download/upload percentage string, duration formatting | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_file.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_gif.h` | 305 | `Gif`, `TtlRoundPaintCallback` | GIF/video element, grouped video support, autoplay animation control | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_gif.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_gif.cpp` | 2751 | `InlineFrameSize`, `Gif::countOptimalSize`, `Gif::draw` | `kMaxInlineArea = 1920*1080` (2,073,600), `kSeekPreviewInterval = 100ms`, `kMaxGifForwardedBarLines = 4`, `kUseNonBlurredThreshold = 240` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_gif.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_sticker.h` | 158 | `Sticker`, `Sticker::Size` | `atTheEnd()`, `frameIndex()`, `framesCount()`, premium multiplier, dice emoji indexing | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_sticker.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_sticker.cpp` | 693 | `Sticker::countOptimalSize`, `ComputeEmojiTextColor`, `CacheDiceImage` | `kMaxSizeFixed = 512`, `kMaxEmojiSizeFixed = 256`, `kPremiumMultiplier = 1.49`, `kEmojiMultiplier = 3`, alpha blending `0x100 - ca` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_sticker.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_sticker_player.h` | 93 | `LottiePlayer`, `WebmPlayer`, `StaticStickerPlayer` | Multi-backend sticker frame decoders, clip notification callbacks | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_sticker_player.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_sticker_player.cpp` | 169 | `LottiePlayer::frame`, `WebmPlayer::clipCallback` | Single player frame extraction, DP scale multiplication, clip reader start | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_sticker_player.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_sticker_player_abstract.h` | 34 | `StickerPlayer`, `StickerPlayer::FrameInfo` | Abstract interface for sticker players: `ready()`, `framesCount()`, `frame()`, `markFrameShown()` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_sticker_player_abstract.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_theme_document.h` | 189 | `ThemeDocument`, `ThemeDocumentBox` | Wallpaper preview generator, pattern opacity, gradient rotation angle, service box content | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_theme_document.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_theme_document.cpp` | 894 | `ThemeDocument::countOptimalSize`, `ThemeDocument::draw`, `ParamsFromUrl` | Wallpaper pattern overlay, blurred background preparation, dimming intensity | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_theme_document.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_save_document_action.h` | 42 | `AddSaveDocumentAction` | Context menu save action provider for files, audio, and video | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_save_document_action.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_save_document_action.cpp` | 133 | `AddSaveDocumentAction` implementation | Menu item labeling (`Save Video`, `Save Audio`, `Save File`), delayed dispatch | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_save_document_action.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_video_status.h` | 47 | `VideoCornerStatus`, `PaintVideoCornerStatus`, `VideoCornerDownloadRect` | Corner overlay status badge for streaming videos, download button bounds | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_video_status.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_video_status.cpp` | 173 | `PaintVideoCornerStatus`, `PaintVideoTimestampMark` | Delta padding math (`msgDateImgDelta + padding.x`), download icon layout | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_video_status.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_video_message_seek.h` | 61 | `VideoMessageSeek` | Round video message circular scrubber, ghost seek indicator, shadow layer | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_video_message_seek.rs` | `gpui_ui_1to1` |
| `Telegram/SourceFiles/history/view/media/history_view_video_message_seek.cpp` | 193 | `VideoMessageSeek::setDraggedProgress`, `VideoMessageSeek::grabPoint` | `kShowDuration = 220ms`, `kGrabDuration = 150ms`, `kGhostJump = 0.9`, radius vs grab band check | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_video_message_seek.rs` | `gpui_ui_1to1` |

---

### 3. Detailed Algorithmic & Mathematical Specifications

#### 3.1 Grouped Media Mosaic Algorithm (`history_view_media_grouped.cpp:L142-L258`)
When up to 10 items (`kMaxSize = 10`) arrive with identical grouped IDs:
- Mode Detection: If any item is a document that is neither video nor animation, mode is `Mode::Column` (rendered as a vertical playlist). Otherwise, mode is `Mode::Grid`.
- Optimal Layout: `Ui::LayoutMediaGroup` arranges the items into rows according to aspect ratios, computing `initialGeometry` and edge connectivity bitmasks (`RectParts` with `Top`, `Bottom`, `Left`, `Right`).
- Dynamic Proportional Width Scaling:
  ```text
  factor = newWidth / maxWidth
  scale(v) = round(v * factor)
  spacing = scale(initialSpacing)
  left = scale(initialLeft)
  top = scale(initialTop)
  width = scale(initialRight) - left - (needRightSkip ? spacing : 0)
  height = scale(initialBottom) - top - (needBottomSkip ? spacing : 0)
  ```

#### 3.2 Voice Playback Organic Blob Deformers (`history_view_document.cpp:L53-L81`)
Voice messages during playback animate organic undulating blobs behind the play button:
- 2 concentric blob layers:
  - Minor blob: 6 segments, `minScale = 0.88`, `alpha = 0.25`, `speed = 2.5`
  - Major blob: 8 segments, `minScale = 0.85`, `alpha = 0.25`, `speed = 2.5`
- Level duration: `kVoiceBlobLevelDuration = 100.0 + 500.0 * 0.33 = 265.0 ms`. Idle amplitude: `kVoiceBlobIdleLevel = 0.45`.

#### 3.3 Spoiler Particle Animation & Paid Tag Pill (`history_view_media.cpp:L346-L476`)
- Continuous dust animation: 60-frame looping particle simulation with pseudo-random spatial jitter and alpha decay.
- Paid Stars Badge: Formatted as `⭐ {price}` pill with semibold text, rounded corners `min(w, h) / 2`.
- Sensitive Media Badge: Formatted as `Sensitive Content` with stealth icon prefix skipping `1.4 * width`.
- Smooth Reveal Transition: Cubic easing alpha curve `1.0 - progress` smoothly unveiling the content on click.

#### 3.4 Circular Video Message Scrubber (`history_view_video_message_seek.cpp:L19-L59`)
- Scrubber angle calculations matching video progress `0.0 .. 1.0` (0 to 360 degrees).
- Ghost Jump: When user seeks across > 90% (`kGhostJump = 0.9`) of duration in one interaction, ghost indicator fades out over `kGhostDuration = 150ms`.
- Grab Hit Testing: Radial distance from center must fall within `[radius - band, radius + band]`.

---

### 4. Target Architecture in GPUI Telegram Desktop

The implementation is split into two complementary layers:
1. **Core Media View Components (`crates/gpui_telegram_desktop/src/views/stage/media/`)**:
   - `mod.rs`: Clean exports and shared geometric data models.
   - `document.rs`: GPUI element for files, audio, and voice bubbles with authentic progress rings.
   - `photo.rs`: GPUI element for photo thumbnails with 9:16 story bounds and spoiler overlay.
   - `gif.rs`: GPUI element for GIF/video streaming previews with corner time status.
   - `sticker.rs`: GPUI element for stickers with Lottie/WebM/Static playback FSM and dice indexing.
   - `grouped.rs`: GPUI grouped album container executing the 10-item mosaic layout math.
   - `spoiler.rs`: GPUI spoiler particle canvas and paid/sensitive pill badge renderer.
2. **Ledger-Aligned Shard Target Modules (`crates/gpui_telegram_desktop/src/views/stage/view/media/`)**:
   - All 17 assigned target files completely rewritten to eliminate all boilerplate stubs.
   - Each module implements authentic data models, state machines, and view helpers matching its upstream `.h` / `.cpp` declarations.

---

### 5. Verification & Acceptance Criteria
- 100% of the 33 assigned upstream C++ files documented with exact line citations.
- 0 boilerplate stubs matching Patterns A, B, C, D in the assigned 17 files.
- All unit and subsystem integration tests pass with 0 failures.
- `cargo test -p gpui_telegram_desktop --test history_media_basic_subsystem_tests` passes exit code 0.
- Mandatory build cache cleanup executed via `python3 scripts/clean_build_garbage.py`.
