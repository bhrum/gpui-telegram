# Module 10 (calls) — Slice 3: Group Calls Viewport Grid, Video Tiles, Audio Reactive Volume & UI Controls Specification

## 1. Executive Summary & Architectural Role

Module 10 Slice 3 implements the visual presentation, hardware-accelerated video rendering, audio-reactive volume controls, and modal UI dialogs for Telegram Group Calls & Live Streams in pure Rust and GPUI. It is responsible for:
- **Dynamic Viewport & Multi-tile Grid Layout**: Dynamic N-participant video grid layout algorithm balancing aspect ratio black-bars minimization, speaker pinning, and screen-share dominant spotlight view (`Viewport`, `VideoTile`, `Layout`, `countWide`, `applyLarge`).
- **Hardware-Accelerated & Software Rendering Pipelines**: Multi-backend video tile rendering across OpenGL GLSL shaders, Modern QRhi pipeline (DirectX/Vulkan/Metal), and raster software fallback with 2-pass box blur, Gaussian noise dithering, and userpic avatar fallback (`RendererGL`, `RendererRhi`, `RendererSW`).
- **Audio Reactive Participant Volume Slider**: Smooth 0% to 200% participant volume control with magnetic 100% snap (+-5% sticky window), speaker arc energy animations, and mute cross-line animations (`MenuVolumeItem`, `kMaxVolumePercent = 200`, `kSpeakerThreshold`).
- **Group Call Settings, Push-To-Talk & Mic Tester**: Call configuration box with microphone level tester (`kMicrophoneTooltipLevelThreshold = 0.2`, `kDelaysCount = 201`), push-to-talk delay calibration (0..2000ms), and invite link sharing.
- **Paid Stars Reactions & Live Stream Boxes**: Telegram Stars donation box for video streams (`VideoStreamStarsBox`, `StarsColoring`, `MaxVideoStreamStarsCount`), live stream recording controls (`StartGroupCallRecordingBox`), countdown scheduled labels (`StartsWhenText`), and screen capture source selector (`DesktopCaptureChooseSource`).

---

## 2. Upstream C++ 100% Reading Manifest (All 26 Upstream Files)

| Upstream Source Path | Lines | Key C++ Classes / Symbols & Logic Extracted | Dynamic GPUI Target Path |
| :--- | :--- | :--- | :--- |
| `Telegram/SourceFiles/calls/group/calls_group_settings.h` | 49 | `SettingsBox`, `ShareInviteLinkAction`, `MicLevelTester` (`_loudCount`, `_quietCount`, `_tester`, `_deviceId`, `check()`). | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_settings.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_settings.cpp` | 1014 | `calls_group_settings.cpp:L64-L72`: `kDelaysCount = 201`, `kMicrophoneTooltipAfterLoudCount = 3`, `kDropLoudAfterQuietCount = 5`, `kMicrophoneTooltipLevelThreshold = 0.2`, `kMicrophoneTooltipCheckInterval = 500ms`, `kCheckAccessibilityInterval = 500ms`; `calls_group_settings.cpp:L121-L135`: `DelayByIndex(index) = index * 10ms`, `FormatDelay` ms vs s; `SaveCallJoinMuted`, `SaveCallMessagesEnabled`, `ShareInviteLinkBox`. | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_settings.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_stars_box.h` | 54 | `MaxVideoStreamStarsCount`, `VideoStreamStarsBoxArgs` (`show`, `top`, `min`, `current`, `sending`, `admin`, `save`, `name`), `VideoStreamStarsBox`, `MakeVideoStreamStarsBox`. | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_stars_box.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_stars_box.cpp` | 147 | `calls_group_stars_box.cpp:L27-L28`: `kMaxStarsFallback = 10000`, `kDefaultStars = 10`; `calls_group_stars_box.cpp:L32-L39`: `MaxVideoStreamStarsCount` clamped to min 2; `calls_group_stars_box.cpp:L41-L140`: `VideoStreamStarsBox` credits loading, `PaidReactionTop` sorting, anonymous handling. | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_stars_box.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_toasts.h` | 38 | `Toasts`: `setup()`, `setupJoinAsChanged()`, `setupTitleChanged()`, `setupRequestedToSpeak()`, `setupAllowedToSpeak()`, `setupPinnedVideo()`, `setupError()`. | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_toasts.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_toasts.cpp` | 183 | `calls_group_toasts.cpp:L24`: `kErrorDuration = 2000ms`; `calls_group_toasts.cpp:L45-L62`: `setupJoinAsChanged` toast; `calls_group_toasts.cpp:L84-L104`: `setupAllowedToSpeak`; `calls_group_toasts.cpp:L106-L145`: `setupPinnedVideo` (camera vs screen, me vs user); `calls_group_toasts.cpp:L147-L156`: `ForceMuted` -> `RaisedHand` toast; `calls_group_toasts.cpp:L158-L181`: 7 call errors. | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_toasts.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_viewport.h` | 253 | `VideoTileTrack`, `Viewport`: `setMode`, `setControlsShown`, `setCursorShown`, `setGeometry`, `resizeToWidth`, `add`, `remove`, `removeByRow`, `showLarge`, `kShadowMaxAlpha = 80`, `Geometry`, `Layout`, `Selection`, `countWide`, `applyLarge`. | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_viewport.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_viewport.cpp` | 1146 | `calls_group_viewport.cpp:L41-L53`: `InterpolateRect`; `calls_group_viewport.cpp:L207-L263`: mouse selection; `calls_group_viewport.cpp:L372-L422`: `prepareLargeChangeAnimation`; `calls_group_viewport.cpp:L424-L489`: `applyLarge` coordinate translation; `calls_group_viewport.cpp:L524-L613`: `countWide` columns vs rows black-bar minimization (`slices = ceil(sqrt(count))`). | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_viewport.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_viewport_opengl.h` | 165 | `Viewport::RendererGL`: `init`, `deinit`, `paint`, `clearColor`, `TileData`, `Program` (`argb32`, `yuv420`), `transformRect`, `drawDownscalePass`, `drawFirstBlurPass`, `validateNoiseTexture`. | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_viewport_opengl.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_viewport_opengl.cpp` | 1482 | `calls_group_viewport_opengl.cpp:L29-L41`: `kScaleForBlurTextureIndex = 3`, `kFirstBlurPassTextureIndex = 4`, `kNoiseTextureSize = 256`, `kBlurTextureSizeFactor = 4.0`, `kBlurOpacity = 0.65`, `kDitherNoiseAmount = 0.002`, `kQuads = 9`, `kQuadVertices = 36`, `kQuadValues = 144`, `kValues = 152`; `calls_group_viewport_opengl.cpp:L43-L74`: 2-pass separable blur shader; `calls_group_viewport_opengl.cpp:L76-L125`: `FragmentGenerateNoise`. | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_viewport_opengl.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_viewport_raster.h` | 67 | `Viewport::RendererSW`: `paintFallback`, `TileData` (`userpicFrame`, `blurredFrame`), `paintTile`, `paintTileOutline`, `paintTileControls`, `validateUserpicFrame`. | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_viewport_raster.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_viewport_raster.cpp` | 347 | `calls_group_viewport_raster.cpp:L25`: `kBlurRadius = 15`; `calls_group_viewport_raster.cpp:L39-L71`: `paintFallback` clip traversal and stale tile cleanup; `calls_group_viewport_raster.cpp:L73-L90`: `validateUserpicFrame` blurred userpic generator; `calls_group_viewport_raster.cpp:L92-L135`: `paintTile` paused/userpic fallback. | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_viewport_raster.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_viewport_rhi.h` | 257 | `Viewport::RendererRhi`: `initialize`, `render`, `renderOffscreen`, `renderOnscreen`, `releaseResources`, `TileData` (RGBA/YUV textures, render targets, descriptors). | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_viewport_rhi.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_viewport_rhi.cpp` | 2035 | `calls_group_viewport_rhi.cpp:L31-L34`: `kNoiseTextureSize = 256`, `kBlurTextureSizeFactor = 4.0`, `kBlurOpacity = 0.65f`; `calls_group_viewport_rhi.cpp:L36-L50`: `GroupFrameUniforms` (128 bytes); `calls_group_viewport_rhi.cpp:L52-L63`: `BlurUniforms` (16 bytes), `ImageUniforms` (16 bytes); `calls_group_viewport_rhi.cpp:L71-L85`: `NdcFlipY` Vulkan check, `OffscreenQuad` (16 floats); `CountBlurredSize`. | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_viewport_rhi.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_viewport_tile.h` | 142 | `Viewport::VideoTile`: `track`, `row`, `peer`, `rtmp`, `geometry`, `animation`, `pinned`, `hidden`, `visible`, `self`, `mirror`, `pinOuter`, `pinInner`, `backOuter`, `backInner`, `endpoint`, `trackSize`, `trackOrUserpicSize`, `PausedVideoSize`, `screencast`, `PinInnerSize`, `PaintPinButton`. | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_viewport_tile.rs` |
| `Telegram/SourceFiles/calls/group/calls_group_viewport_tile.cpp` | 278 | `calls_group_viewport_tile.cpp:L23`: `kPausedVideoSize = 90`; `calls_group_viewport_tile.cpp:L54-L56`: `mirror()` self camera check; `calls_group_viewport_tile.cpp:L74-L79`: `topControlsSlide`; `calls_group_viewport_tile.cpp:L122-L131`: `updateRequestedQuality`; `calls_group_viewport_tile.cpp:L133-L170`: `PinInnerSize` & `PaintPinButton`. | `crates/gpui_telegram_webrtc/src/calls/group/calls_group_viewport_tile.rs` |
| `Telegram/SourceFiles/calls/group/calls_volume_item.h` | 93 | `MenuVolumeItem`: `setCanMute`, `toggleMuteRequests`, `toggleMuteLocallyRequests`, `changeVolumeRequests`, `changeVolumeLocallyRequests`, `initArcsAnimation`, `setCloudVolume`, `setSliderVolume`, `updateSliderColor`. | `crates/gpui_telegram_webrtc/src/calls/group/calls_volume_item.rs` |
| `Telegram/SourceFiles/calls/group/calls_volume_item.cpp` | 305 | `calls_volume_item.cpp:L23`: `kMaxVolumePercent = 200`; `calls_volume_item.cpp:L25-L28`: `kSpeakerThreshold = { 10/200, 50/200, 150/200 }`; `calls_volume_item.cpp:L30-L39`: `kVolumeStickedValues` (snap pairs for 25%, 50%, 75%, 100% [+-5%], 125%, 150%, 175%); `calls_volume_item.cpp:L122-L140`: slider progress callback and mute cross-line trigger. | `crates/gpui_telegram_webrtc/src/calls/group/calls_volume_item.rs` |
| `Telegram/SourceFiles/calls/group/ui/calls_group_recording_box.h` | 42 | `RecordingType { AudioOnly, VideoLandscape, VideoPortrait }`, `EditGroupCallTitleBox`, `StartGroupCallRecordingBox`, `AddTitleGroupCallRecordingBox`, `StopGroupCallRecordingBox`. | `crates/gpui_telegram_webrtc/src/calls/group/ui/calls_group_recording_box.rs` |
| `Telegram/SourceFiles/calls/group/ui/calls_group_recording_box.cpp` | 370 | `calls_group_recording_box.cpp:L26-L28`: `kRoundRadius = 9`, `kMaxGroupCallLength = 40`, `kSwitchDuration = 200`; `GraphicButton`, `RecordingInfo`, `Switcher`. | `crates/gpui_telegram_webrtc/src/calls/group/ui/calls_group_recording_box.rs` |
| `Telegram/SourceFiles/calls/group/ui/calls_group_scheduled_labels.h` | 27 | `StartsWhenText`, `CreateGradientLabel`. | `crates/gpui_telegram_webrtc/src/calls/group/ui/calls_group_scheduled_labels.rs` |
| `Telegram/SourceFiles/calls/group/ui/calls_group_scheduled_labels.cpp` | 139 | `calls_group_scheduled_labels.cpp:L32-L35`: `kDay = 86400`, `tillTomorrow`, `tillToday`, `tillAfter` time difference thresholds; `calls_group_scheduled_labels.cpp:L108-L114`: 3-stop gradient { 0.0, 0.7, 1.0 }. | `crates/gpui_telegram_webrtc/src/calls/group/ui/calls_group_scheduled_labels.rs` |
| `Telegram/SourceFiles/calls/group/ui/calls_group_stars_coloring.h` | 47 | `StarsColoring` (`bgLight`, `bgDark`, `fromStars`, `secondsPin`, `charactersMax`, `emojiLimit`), `StarsColoringForCount`, `StarsRequiredForMessage`, `VideoStreamStarsLevel`. | `crates/gpui_telegram_webrtc/src/calls/group/ui/calls_group_stars_coloring.rs` |
| `Telegram/SourceFiles/calls/group/ui/calls_group_stars_coloring.cpp` | 119 | `calls_group_stars_coloring.cpp:L20-L30`: `StarsColoringForCount` binary lower-bound search; `calls_group_stars_coloring.cpp:L32-L55`: `StarsRequiredForMessage` emoji parsing and character length limit matching; `VideoStreamStarsLevel` pin duration string format. | `crates/gpui_telegram_webrtc/src/calls/group/ui/calls_group_stars_coloring.rs` |
| `Telegram/SourceFiles/calls/group/ui/desktop_capture_choose_source.h` | 34 | `ChooseSourceDelegate` (`chooseSourceParent`, `chooseSourceActiveDeviceId`, `chooseSourceActiveWithAudio`, `chooseSourceWithAudioSupported`, `chooseSourceAccepted`, `chooseSourceStop`), `ChooseSource`. | `crates/gpui_telegram_webrtc/src/calls/group/ui/desktop_capture_choose_source.rs` |
| `Telegram/SourceFiles/calls/group/ui/desktop_capture_choose_source.cpp` | 616 | `desktop_capture_choose_source.cpp:L31-L32`: `kColumns = 3`, `kRows = 2` (3x2 thumbnail page grid); `Preview`, `SourceButton`, `Source`, `ChooseSourceProcess`, `withAudio` capture option toggle. | `crates/gpui_telegram_webrtc/src/calls/group/ui/desktop_capture_choose_source.rs` |

---

## 3. Dynamic Rust / GPUI Architecture & Module Mapping

Under Mandate #17, the 26 C++ upstream files are mapped to authentic, cohesive Rust modules in `crates/gpui_telegram_webrtc/src/calls/group/`:

```
crates/gpui_telegram_webrtc/src/calls/group/
├── mod.rs                                    # Re-exports group call types and target symbols
├── calls_group_settings.rs                   # CallsGroupSettingsGroup: PTT delays, mic level tester (0.2 threshold)
├── calls_group_stars_box.rs                  # CallsGroupStarsBoxGroup: Video stream stars donation dialog & ranking
├── calls_group_toasts.rs                     # CallsGroupToastsGroup: Event notifications (2000ms duration, 7 errors)
├── calls_group_viewport.rs                   # CallsGroupViewportGroup: N-tile layout optimization (columns vs rows)
├── calls_group_viewport_opengl.rs            # CallsGroupViewportOpenglGroup: GLSL ARGB/YUV shaders, 2-pass blur
├── calls_group_viewport_raster.rs            # CallsGroupViewportRasterGroup: Software fallback with 15px radius blur
├── calls_group_viewport_rhi.rs               # CallsGroupViewportRhiGroup: 128B GroupFrameUniforms, Vulkan NdcFlipY
├── calls_group_viewport_tile.rs              # CallsGroupViewportTileGroup: 90px paused video tile, mirror, pin button
├── calls_volume_item.rs                      # CallsVolumeItemGroup: 0..200% volume slider with +-5% 100% magnetic snap
└── ui/
    ├── mod.rs                                # Re-exports group call UI subcomponents
    ├── calls_group_recording_box.rs          # CallsGroupRecordingBoxGroupUi: Audio/Landscape/Portrait recorder
    ├── calls_group_scheduled_labels.rs       # CallsGroupScheduledLabelsGroupUi: Live countdown timer & gradient label
    ├── calls_group_stars_coloring.rs         # CallsGroupStarsColoringGroupUi: Star tier thresholds & emoji limits
    └── desktop_capture_choose_source.rs      # DesktopCaptureChooseSourceGroupUi: 3x2 screen/window capture picker
```
## 4. Deep Technical Analysis & Implementation Details

### 4.1 Settings, Push-To-Talk Delays & Mic Level Tester (`calls_group_settings.cpp:L64-L135`)
- **PTT Delay Quantization**: The upstream C++ code defines `kDelaysCount = 201` (`calls_group_settings.cpp:L64`). Each delay index maps to `index * 10ms` via `DelayByIndex(int index)` (`calls_group_settings.cpp:L121-L123`), providing delay steps from 0 ms to 2000 ms (2.0s). The formatter `FormatDelay` displays millisecond precision `< 1000ms` as integer ms (e.g. `250 ms`), and `>= 1000ms` as seconds with 2 fractional digits (e.g. `1.50 s`) (`calls_group_settings.cpp:L125-L135`).
- **Microphone Tooltip Heuristics**: The `MicLevelTester` samples audio input levels every `kMicrophoneTooltipCheckInterval = 500ms` (`calls_group_settings.cpp:L68`). If the RMS level exceeds `kMicrophoneTooltipLevelThreshold = 0.2` (`calls_group_settings.cpp:L67`), `_loudCount` increments and `_quietCount` resets to 0. When `_loudCount >= kMicrophoneTooltipAfterLoudCount = 3` (`calls_group_settings.cpp:L65`) while muted, a tooltip is triggered suggesting the user unmute or use push-to-talk. When the user is quiet for `kDropLoudAfterQuietCount = 5` cycles (`calls_group_settings.cpp:L66`), the loud accumulator resets.
- **Group Call Flags**: Toggle settings serialize via `MTPphone_ToggleGroupCallSettings` with flags `Flag::f_join_muted` and `Flag::f_messages_enabled` (`calls_group_settings.cpp:L87-L118`).

### 4.2 Stars Donation & Video Stream Stars Box (`calls_group_stars_box.cpp:L27-L140`)
- **Max Stars Bounds**: Maximum stars allowed for group call messages is read from `session->appConfig().get<int>("stars_groupcall_message_amount_max", kMaxStarsFallback)` (`calls_group_stars_box.cpp:L32-L39`), defaulting to `kMaxStarsFallback = 10000` (`calls_group_stars_box.cpp:L27`) and clamped to a minimum of 2 stars. The default pre-selected amount is `kDefaultStars = 10` (`calls_group_stars_box.cpp:L28`).
- **Top Donor Ranking & Deduplication**: The box receives `std::vector<MessageReactionsTopPaid> top` (`calls_group_stars_box.cpp:L69-L95`). It inserts the current authenticated user's entry using `barePeerId` deduplication into a flat set (`calls_group_stars_box.cpp:L97-L114`). Donors are then sorted using `ranges::stable_sort` with `ranges::greater()` on donation `count` (`calls_group_stars_box.cpp:L115`). Anonymous donations are labeled via `lng_paid_react_anonymous` with avatar hidden (`calls_group_stars_box.cpp:L74, L84`).

### 4.3 Group Call Toasts & Permissions Notifications (`calls_group_toasts.cpp:L24-L181`)
- **Toast Durations**: Error toasts are displayed with `kErrorDuration = 2000ms` (`calls_group_toasts.cpp:L24, L179`).
- **Rejoin & Title Changes**: Identity switches (`wasJoinAs != nowJoinAs`) trigger toast notification `lng_group_call_join_as_changed` once the call enters `State::Joined` (`calls_group_toasts.cpp:L46-L61`). Title updates trigger broadcast vs normal group notification (`calls_group_toasts.cpp:L64-L82`).
- **Permission & Pin State Machine**: Transition from `MuteState::ForceMuted` to `MuteState::RaisedHand` confirms the hand raise to the user (`calls_group_toasts.cpp:L148-L155`). The pinned video notifications distinguish four distinct scenarios (`calls_group_toasts.cpp:L121-L142`):
  1. Camera self-pinned / unpinned (`lng_group_call_pinned_camera_me` / `unpinned`)
  2. Screen self-pinned / unpinned (`lng_group_call_pinned_screen_me` / `unpinned`)
  3. Camera peer pinned / unpinned (`lng_group_call_pinned_camera` / `unpinned`)
  4. Screen peer pinned / unpinned (`lng_group_call_pinned_screen` / `unpinned`)
- **Hardware & Permission Errors**: Handles 7 distinct call error enumerations (`calls_group_toasts.cpp:L162-L177`): `NoCamera`, `CameraFailed`, `ScreenFailed`, `MutedNoCamera`, `MutedNoScreen`, `DisabledNoCamera`, `DisabledNoScreen`.

### 4.4 Multi-Tile Viewport Optimization Grid (`calls_group_viewport.cpp:L41-L53, L524-L613`)
- **Columns vs Rows Black-Bar Minimization Algorithm**: In `countWide(int outerWidth, int outerHeight)` (`calls_group_viewport.cpp:L524-L613`), the layout calculates optimal grid dimensions for `N` active video tiles:
  1. Calculate square root slice count: `slices = ceil(sqrt(count))` (`calls_group_viewport.cpp:L546`).
  2. Test **Column-dominant layout**: iterate `column` from `0..slices`, compute column width with `skip = st::groupCallVideoLargeSkip`, and distribute remaining items into rows `(count - index) / (columns - column)`. Calculate aggregate black-bar area `columnsBlack` by comparing scaled aspect ratio against tile box (`calls_group_viewport.cpp:L568-L575`).
  3. Test **Row-dominant layout**: iterate `row` from `0..slices`, distribute items into columns `(count - index) / (rows - row)`. Calculate aggregate black-bar area `rowsBlack` (`calls_group_viewport.cpp:L600-L607`).
  4. Pick orientation with minimal black space: `result.useColumns = (columnsBlack < rowsBlack)` (`calls_group_viewport.cpp:L611`).
- **Spotlight Large Video Mode**: `applyLarge` scales the spotlighted tile to fill `outer` geometry while pushing non-dominant tiles into a secondary row/column rail (`calls_group_viewport.cpp:L424-L489`). Transitions animate via `InterpolateRect` with `SimpleAnimation` over `st::slideDuration` (`calls_group_viewport.cpp:L41-L53, L417-L422`).

### 4.5 Hardware-Accelerated OpenGL & GLSL Shaders (`calls_group_viewport_opengl.cpp:L29-L125`)
- **GLSL Blur & Dither Constants**:
  - `kNoiseTextureSize = 256` (`calls_group_viewport_opengl.cpp:L31`)
  - `kBlurTextureSizeFactor = 4.0` (`calls_group_viewport_opengl.cpp:L34`)
  - `kBlurOpacity = 0.65` (`calls_group_viewport_opengl.cpp:L35`)
  - `kDitherNoiseAmount = 0.002` (`calls_group_viewport_opengl.cpp:L36`)
  - Geometry quads: `kQuads = 9`, `kQuadVertices = 36`, `kQuadValues = 144`, `kValues = 152` (`calls_group_viewport_opengl.cpp:L38-L41`).
- **Two-Pass Separable Gaussian Blur**: Kernel with `radius = 15` and `diameter = 31` samples texels along horizontal `offsets = vec2(1, 0)` then vertical `offsets = vec2(0, 1)`, weighting with triangular box filter `fradius + 1.0 - abs(float(i) - fradius)` and normalizing by accumulated alpha (`calls_group_viewport_opengl.cpp:L43-L73`).
- **Dithering Noise Generation**: Perlin/simplex hash synthesis (`calls_group_viewport_opengl.cpp:L86-L120`) prevents 8-bit banding on dark video backgrounds.
### 4.6 Raster Software Fallback Renderer (`calls_group_viewport_raster.cpp:L25-L135`)
- **Software Blur**: Uses `kBlurRadius = 15` (`calls_group_viewport_raster.cpp:L25`) for `Images::BlurLargeImage` on userpic avatar fallback and paused video frames.
- **Paint Traversal & Stale Lifecycle**: Each paint pass flags all `_tileData` as `stale = true` (`calls_group_viewport_raster.cpp:L46-L48`). Visible tiles clear their stale flag; unvisited tiles are cleanly evicted from the map at the end of the frame (`calls_group_viewport_raster.cpp:L64-L70`).
- **Userpic / Paused Handling**: When track format is `FrameFormat::None`, `validateUserpicFrame` generates the blurred avatar background (`calls_group_viewport_raster.cpp:L73-L90`). Paused video freezes the last frame, applies 15px blur, and overlays pause status icons (`calls_group_viewport_raster.cpp:L107-L115`).

### 4.7 Modern QRhi Pipeline & Uniform Blocks (`calls_group_viewport_rhi.cpp:L31-L100`)
- **Uniform Memory Layouts (C++ `static_assert`)**:
  - `GroupFrameUniforms` (128 bytes): viewport dimensions, flipY, frame background color vec4, shadow color vec4, paused float, roundRect parameters, and outline color vec4 (`calls_group_viewport_rhi.cpp:L36-L50`).
  - `BlurUniforms` (16 bytes): `texelOffset` and 12 bytes of padding (`calls_group_viewport_rhi.cpp:L52-L56`).
  - `ImageUniforms` (16 bytes): viewport vec2, global opacity, flipY (`calls_group_viewport_rhi.cpp:L58-L63`).
- **Cross-Platform Vulkan/Metal/DirectX NDC Flip**: Vulkan uses NDC Y-down, requiring inverted Y coordinates: `NdcFlipY(rhi) = rhi->isYUpInNDC() ? 1.0f : -1.0f` (`calls_group_viewport_rhi.cpp:L71-L74`).
- **Offscreen Texture Passes**: Fullscreen quad with 16 floats (`OffscreenQuad`) (`calls_group_viewport_rhi.cpp:L77-L85`) coordinates downscaling, horizontal blur, vertical blur, and compositing passes.

### 4.8 Video Tile Controls, Geometry & Quality Ladder (`calls_group_viewport_tile.cpp:L23-L140`)
- **Paused Video Dimensions**: `kPausedVideoSize = 90` (`calls_group_viewport_tile.cpp:L23`). Paused tracks return a fixed 90x90 square aspect for avatar fallback (`calls_group_viewport_tile.cpp:L81-L90`).
- **Self Camera Mirroring**: Front-facing local camera feeds are mirrored horizontally (`mirror() = _self && _endpoint.type == VideoEndpointType::Camera`), whereas screen shares and peer cameras are never mirrored (`calls_group_viewport_tile.cpp:L54-L56`).
- **Quality Adaptation**: Tile quality switches between `VideoQuality::Thumbnail`, `VideoQuality::Medium`, and `VideoQuality::Full` (`calls_group_viewport_tile.cpp:L122-L131`). Hidden tiles drop their quality allocation to save network bandwidth.
- **Pin & Back Controls**: Buttons slide in/out from top of tile via `topControlsSlide` interpolated animation (`calls_group_viewport_tile.cpp:L74-L79`).

### 4.9 Participant Volume Slider with Magnetic Snap (`calls_volume_item.cpp:L23-L150`)
- **Volume Range**: Supports 0% to 200% volume amplification (`kMaxVolumePercent = 200`, `calls_volume_item.cpp:L23`).
- **Speaker Arcs Energy Thresholds**: Arcs animation renders soundwave rings at 3 volume tier thresholds (`calls_volume_item.cpp:L25-L28`):
  1. Low volume threshold: `10.0 / 200.0` (5%)
  2. Medium volume threshold: `50.0 / 200.0` (25%)
  3. High volume threshold: `150.0 / 200.0` (75%)
- **Magnetic Snap Points (`kVolumeStickedValues`)**: Provides haptic/visual stickiness at standard intervals (`calls_volume_item.cpp:L30-L39`):
  - 25%: `value = 0.125`, stick radius `2 / 200` (1%)
  - 50%: `value = 0.250`, stick radius `2 / 200` (1%)
  - 75%: `value = 0.375`, stick radius `2 / 200` (1%)
  - **100% (Default Normal)**: `value = 0.500`, **wide stick radius `10 / 200` (5%)** to easily snap back to standard 100% volume without fiddling!
  - 125%: `value = 0.625`, stick radius `2 / 200` (1%)
  - 150%: `value = 0.750`, stick radius `2 / 200` (1%)
  - 175%: `value = 0.875`, stick radius `2 / 200` (1%)
- **Mute Cross-Line Animation**: Dragging slider to 0% triggers local mute with animated diagonal slash across speaker icon (`calls_volume_item.cpp:L122-L133`).

### 4.10 Call Recording Box & Formatting (`calls_group_recording_box.cpp:L26-L100`)
- **Visual Constants**: `kRoundRadius = 9`, `kMaxGroupCallLength = 40`, `kSwitchDuration = 200ms` (`calls_group_recording_box.cpp:L26-L28`).
- **Recording Modes**: `RecordingType::AudioOnly`, `RecordingType::VideoLandscape`, and `RecordingType::VideoPortrait` with graphic button toggles and orientation preview icons (`calls_group_recording_box.h:L16-L20`, `calls_group_recording_box.cpp:L83-L100`).
### 4.11 Scheduled Call Countdown Labels & Linear Gradients (`calls_group_scheduled_labels.cpp:L22-L137`)
- **Relative Date Thresholds**: Uses `kDay = 86400` seconds (`calls_group_scheduled_labels.cpp:L32`). Compares current Unix timestamp against scheduled time to determine 3 relative stages (`calls_group_scheduled_labels.cpp:L33-L35`):
  1. `tillTomorrow > 0`: Formats as full calendar date (`lng_group_call_starts_short_date`) with transition timer to tomorrow.
  2. `tillToday > 0`: Formats as "Tomorrow at HH:MM" (`lng_group_call_starts_short_tomorrow`) with transition timer to today.
  3. `tillAfter > 0`: Formats as "Today at HH:MM" (`lng_group_call_starts_short_today`) with transition timer to starting live.
- **Three-Stop Linear Text Gradient**: Renders custom gradient on countdown digits with stops:
  - `0.0`: `groupCallForceMutedBar1`
  - `0.7`: `groupCallForceMutedBar2`
  - `1.0`: `groupCallForceMutedBar3`
  Scales down proportionally if rendered width exceeds container width (`calls_group_scheduled_labels.cpp:L108-L131`).

### 4.12 Telegram Stars Donation Tier Coloring (`calls_group_stars_coloring.cpp:L20-L117`)
- **Stars Tier Matching (`StarsColoringForCount`)**: Iterates through ascending `StarsColoring` tiers (`fromStars`), returning the highest tier where `fromStars <= stars` (`calls_group_stars_coloring.cpp:L20-L30`).
- **Emoji and Character Length Verification (`StarsRequiredForMessage`)**: Parses text for custom/Unicode emojis (`Emoji::Find`) and measures total character count (`calls_group_stars_coloring.cpp:L32-L48`). Returns the minimum star tier needed to satisfy both `emojis <= entry.emojiLimit` and `length <= entry.charactersMax` (`calls_group_stars_coloring.cpp:L49-L55`).
- **Formatted Pin Duration**: Formats pin duration in seconds into `lng_hours_tiny`, `lng_minutes_tiny`, or `lng_seconds_tiny` (`calls_group_stars_coloring.cpp:L74-L80`).

### 4.13 Desktop Screen & Window Capture Source Chooser (`desktop_capture_choose_source.cpp:L31-L120`)
- **Thumbnail Grid Layout**: Arranged in `kColumns = 3` and `kRows = 2` (6 thumbnails per scroll page) (`desktop_capture_choose_source.cpp:L31-L32`).
- **Capture Source Management**: Handles source enumeration from WebRTC/tgcalls `DesktopCaptureSourceManager`, generates live thumbnail frames, and binds "Share Audio" checkbox (`calls_group_recording_box.cpp:L114`, `desktop_capture_choose_source.h:L23-L28`).

---

## 5. Subsystem Test Plan (Coverage for >= 12 Subsystem Tests)

The test suite in `crates/gpui_telegram_webrtc/tests/calls_group_part2_subsystem_tests.rs` validates all 13 modules against upstream C++ behavior:
1. `test_ptt_delay_quantization_and_formatting`: Validates `DelayByIndex` 0..200 indices (0..2000ms) and `FormatDelay` ms vs s threshold at 1000ms.
2. `test_mic_level_tester_tooltip_trigger`: Validates `kMicrophoneTooltipLevelThreshold = 0.2`, `loudCount >= 3` trigger, and `quietCount >= 5` reset.
3. `test_video_stream_stars_box_clamp_and_ranking`: Validates `MaxVideoStreamStarsCount` min 2, fallback 10000, and `PaidReactionTop` descending stable sort.
4. `test_group_call_toasts_error_mapping_and_duration`: Validates 7 call error mappings and `kErrorDuration = 2000ms`.
5. `test_group_call_toasts_pinned_video_scenarios`: Validates camera vs screen and self vs peer pinned string selection.
6. `test_viewport_tile_grid_columns_vs_rows_minimization`: Validates `countWide` square root slices and black-bar minimization choice between columns and rows.
7. `test_viewport_large_spotlight_geometry_transformation`: Validates `applyLarge` coordinate translation for spotlight tile and rail tiles.
8. `test_opengl_blur_kernel_and_dither_constants`: Validates radius 15, diameter 31, triangular weights, and noise texture dimensions.
9. `test_raster_software_fallback_tile_lifecycle`: Validates `validateUserpicFrame` 15px blur and stale tile eviction pass.
10. `test_rhi_uniform_buffer_sizes_and_vulkan_ndc_flip`: Validates `GroupFrameUniforms` 128B, `BlurUniforms` 16B, `ImageUniforms` 16B, and `NdcFlipY`.
11. `test_video_tile_mirroring_and_paused_size`: Validates 90px paused size, camera mirroring for self only, and screen share non-mirroring.
12. `test_volume_slider_magnetic_snap_and_arc_thresholds`: Validates 0..200% scaling, 100% magnetic snap (+-5% window), and 3 speaker arc thresholds.
13. `test_scheduled_call_labels_date_stages_and_gradient`: Validates relative countdown transitions (Tomorrow -> Today -> Live) and 3-stop gradient.
14. `test_stars_tier_coloring_and_message_limits`: Validates `StarsColoringForCount` lower bound and `StarsRequiredForMessage` emoji/character limits.
15. `test_desktop_capture_source_grid_paging`: Validates 3x2 grid paging and audio capture delegation.
