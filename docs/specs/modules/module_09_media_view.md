# Module 09 (media & editor) — Slice 3: Media Lightbox Viewer, Overlay Controls, Zoom & Navigation Specification

## 1. Executive Summary & Architectural Role

Module 09 Slice 3 covers the core Telegram Desktop media lightbox overlay viewer and floating picture-in-picture (PiP) system:
- Fullscreen and windowed media lightbox overlay viewer (`OverlayWidget`, `media_view_overlay_widget.{h,cpp}`) supporting photos, animated GIFs, videos, documents, themes, stories, and live video streams.
- Multi-backend rendering pipeline (`OverlayWidget::Renderer`, raster fallback `RendererSW`, OpenGL backend `RendererGL`, modern QRhi backend `RendererRhi`, and macOS Metal pixel buffer cache `MetalTextureCache`).
- Exact continuous zoom mathematics, centering, and panning boundaries (`snapXY`, `zoomIn`, `zoomOut`, `zoomReset`, `zoomUpdate`, `setZoomLevel`, `kMaxZoomLevel = 7`, `kZoomToScreenLevel = 1024`, `kPinchZoomStep = 0.25`).
- Gesture & swipe navigation (`setupSwipeNavigation`, `mediaviewSwipeDistance = 80px`, LeftToRight/RightToLeft transition, mouse wheel zoom/pan).
- Auto-hiding control bars (`activateControls`, `hideControls`, `mediaviewWaitHide = 1100ms`, `mediaviewHideDuration = 600ms`, `mediaviewShowDuration = 200ms`, `mediaviewFadeDuration = 150ms`).
- Video playback controls panel (`PlaybackControls`, `media_view_playback_controls.{h,cpp}`) with play/pause, volume control, speed selector (0.5x–2.0x), quality switcher (1080p, 720p, 480p, 360p), chapter markers, and timeline tooltips.
- Smooth playback buffer progress tracking (`PlaybackProgress`, `media_view_playback_progress.{h,cpp}`, `kPlaybackAnimationDurationMs = 200ms`).
- Sponsored video advertisement card overlay (`PlaybackSponsored`, `media_view_playback_sponsored.{h,cpp}`, `kStartDelayMin = 1000ms`, `kDurationMin = 5000ms`, circular countdown close button).
- Grouped album media thumbnail strip (`GroupThumbs`, `media_view_group_thumbs.{h,cpp}`, `kThumbDuration = 150ms`, `mediaviewGroupHeight = 80px`, `mediaviewGroupWidth = 56px`, `mediaviewGroupWidthMax = 160px`).
- Floating Picture-in-Picture window (`Pip`, `PipPanel`, `media_view_pip.{h,cpp}`, `media_view_pip_renderer.h`, `media_view_pip_raster.{h,cpp}`, `media_view_pip_opengl.{h,cpp}`, `media_view_pip_rhi.{h,cpp}`, screen edge snapping `ClampToEdges`, `pipBorderSkip = 12px`, `pipBorderSnapArea = 20px`, 4-corner resizing).
- On-device text recognition and selection overlay (`RecognitionSelection`, `media_view_recognition_selection.{h,cpp}`, `ComputeReadingOrder`, `RecognizedText`, character bounding box hit testing).
- Live group call and stream broadcast viewer (`VideoStream`, `media_view_video_stream.{h,cpp}`, live reaction donor places).
- Open request dispatcher (`OpenRequest`, `ExtractVideoTimestamp`, `StripQuoteEntities`, `media_view_open_common.{h,cpp}`).
- Full visual theme metrics (`media_view.style`, `MediaViewViewStyleSheet`).

---

## 2. Upstream C++ 100% Per-File Reading Manifest (35 Sources)

### Part 1: Core Overlay Viewer & Open Dispatch

| Upstream Source Path | Lines | Key C++ Classes / Symbols & Logic Extracted | Dynamic GPUI Target Path |
| :--- | :--- | :--- | :--- |
| `Telegram/SourceFiles/media/view/media_view_open_common.h` | 185 | `OpenRequest` (session, photo, doc, story, call, peer, item, topicRootId, monoforumPeerId, continueStreaming, startTime, showDrawButton), `ExtractVideoTimestamp`, `StripQuoteEntities` | `crates/gpui_telegram_media/src/media/view/media_view_open_common.rs` |
| `Telegram/SourceFiles/media/view/media_view_open_common.cpp` | 42 | `media_view_open_common.cpp:L16-L39`: `ExtractVideoTimestamp` (checks media videoTimestamp or webpage timestamp), `StripQuoteEntities` (strips blockquotes, converts Pre to Code) | `crates/gpui_telegram_media/src/media/view/media_view_open_common.rs` |
| `Telegram/SourceFiles/media/view/media_view_overlay_widget.h` | 913 | `OverlayWidget`, `Over` enum (18 hover targets: Left, Right, Save, Share, Rotate, More, Caption, SpeedBoost, Chapter, etc.), `ContentGeometry` (rect, rotation, controlsOpacity, fade, scale), `ControlsState` (Showing, Shown, Hiding, Hidden) | `crates/gpui_telegram_media/src/media/view/media_view_overlay_widget.rs` |
| `Telegram/SourceFiles/media/view/media_view_overlay_widget.cpp` | 9082 | `media_view_overlay_widget.cpp:L186-L210`: `kPreloadCount = 3`, `kMaxZoomLevel = 7`, `kZoomToScreenLevel = 1024`, `kPinchZoomStep = 0.25`, `kOverlayLoaderPriority = 2`, `kSeekTimeMs = 5000`, `kSeekTimeMsLong = 10000`, `kFrameStepFallbackFps = 30.`, `kFrameStepThrottleMs = 150`, `kMaxDisplayImageSize = 4096`, `kIdsLimit = 48`, `kIdsPreloadAfter = 28`; `L2923-L2999`: `zoomIn`, `zoomOut`, `zoomReset`, `zoomUpdate`; `L7533-L7580`: `setupSwipeNavigation`; `L7619-L7673`: `setZoomLevel`; `L8093-L8107`: `snapXY` centering/clamping | `crates/gpui_telegram_media/src/media/view/media_view_overlay_widget.rs` |
| `Telegram/SourceFiles/media/view/media_view_recognition_selection.h` | 96 | `RecognitionId`, `RecognitionPosition` (item, character), `RecognitionSpan` (item, from, till, rowStart), `RecognitionSelection`, `RecognitionCacheMap` | `crates/gpui_telegram_media/src/media/view/media_view_recognition_selection.rs` |
| `Telegram/SourceFiles/media/view/media_view_recognition_selection.cpp` | 416 | `media_view_recognition_selection.cpp:L21-L57`: `ComputeReadingOrder` (sorts by vertical center, groups by row threshold `max(h1, h2)/2`, stable sorts by row then x); `L68-L85`: `RecognizedText` (row newline vs word space insertion) | `crates/gpui_telegram_media/src/media/view/media_view_recognition_selection.rs` |

### Part 2: Playback Controls, Progress, Sponsored & Groups

| Upstream Source Path | Lines | Key C++ Classes / Symbols & Logic Extracted | Dynamic GPUI Target Path |
| :--- | :--- | :--- | :--- |
| `Telegram/SourceFiles/media/view/media_view_playback_controls.h` | 151 | `PlaybackControls`, `PlaybackControls::Delegate`, `TimestampData` (position, label), volume toggle, timeline sliders, speed and quality selectors | `crates/gpui_telegram_media/src/media/view/media_view_playback_controls.rs` |
| `Telegram/SourceFiles/media/view/media_view_playback_controls.cpp` | 558 | `media_view_playback_controls.cpp:L29`: `kEps = 0.005`, `handleSeekProgress`, `handleSeekFinished`, `countDownloadedTillPercent`, `updateVolumeToggleIcon` (<=0 mute, <0.5 level 1, >=0.5 level 2) | `crates/gpui_telegram_media/src/media/view/media_view_playback_controls.rs` |
| `Telegram/SourceFiles/media/view/media_view_playback_progress.h` | 63 | `PlaybackProgress` (value, animated, loadedTillPercent, availableTill), `setValueChangedCallback`, `setInLoadingStateChangedCallback` | `crates/gpui_telegram_media/src/media/view/media_view_playback_progress.rs` |
| `Telegram/SourceFiles/media/view/media_view_playback_progress.cpp` | 170 | `media_view_playback_progress.cpp:L16`: `kPlaybackAnimationDurationMs = 200ms`, `updateState`: `animatedPosition = position + (frequency * 200 / 1000)`, `availableTill = max(receivedTill, loadedTill)` | `crates/gpui_telegram_media/src/media/view/media_view_playback_progress.rs` |
| `Telegram/SourceFiles/media/view/media_view_playback_sponsored.h` | 84 | `PlaybackSponsored`, `State` (now, data), `show`, `hide`, `setPaused`, `Has(item)` | `crates/gpui_telegram_media/src/media/view/media_view_playback_sponsored.rs` |
| `Telegram/SourceFiles/media/view/media_view_playback_sponsored.cpp` | 769 | `media_view_playback_sponsored.cpp:L37-L38`: `kStartDelayMin = 1000ms`, `kDurationMin = 5000ms`, circular countdown close button with ripple animation | `crates/gpui_telegram_media/src/media/view/media_view_playback_sponsored.rs` |
| `Telegram/SourceFiles/media/view/media_view_group_thumbs.h` | 179 | `GroupThumbs`, `Key` (PhotoId, FullMsgId, CollageKey, InstantViewKey), `CollageSlice`, `InstantViewSlice`, `Refresh`, `resizeToWidth`, `getState` | `crates/gpui_telegram_media/src/media/view/media_view_group_thumbs.rs` |
| `Telegram/SourceFiles/media/view/media_view_group_thumbs.cpp` | 1031 | `media_view_group_thumbs.cpp:L34`: `kThumbDuration = 150ms`, `ComputeFileOrigin`, `Thumb` geometry interpolation, `currentLeft`, `currentWidth`, `finalLeft`, `finalWidth` | `crates/gpui_telegram_media/src/media/view/media_view_group_thumbs.rs` |
| `Telegram/SourceFiles/media/view/media_view_video_stream.h` | 105 | `VideoStream`, `TopVideoStreamDonors`, `closeRequests`, `setVolume`, `updateGeometry`, `toggleCommentsOn` | `crates/gpui_telegram_media/src/media/view/media_view_video_stream.rs` |
| `Telegram/SourceFiles/media/view/media_view_video_stream.cpp` | 396 | `media_view_video_stream.cpp:L64-L100`: `TopVideoStreamDonors`, `TopDonorPlaces` mapped from call star top donors, `Loading` gradient shimmer | `crates/gpui_telegram_media/src/media/view/media_view_video_stream.rs` |

### Part 3: Picture-in-Picture (PiP), Multi-Backend Renderers & Styling

| Upstream Source Path | Lines | Key C++ Classes / Symbols & Logic Extracted | Dynamic GPUI Target Path |
| :--- | :--- | :--- | :--- |
| `Telegram/SourceFiles/media/view/media_view_pip.h` | 322 | `Pip`, `PipPanel`, `Position` (attached, snapped, geometry, screen), `RotatedRect`, `FlipSizeByRotation`, `PipShadow`, mini playback and volume buttons | `crates/gpui_telegram_media/src/media/view/media_view_pip.rs` |
| `Telegram/SourceFiles/media/view/media_view_pip.cpp` | 1899 | `media_view_pip.cpp:L51-L52`: `kPipLoaderPriority = 2`, `kMsInSecond = 1000`; `L69-L95`: `ClampToEdges` screen edge snapping with `pipBorderSkip = 12px`, `pipBorderSnapArea = 20px`; `L97-L120`: `Transformed` corner drag resize | `crates/gpui_telegram_media/src/media/view/media_view_pip.rs` |
| `Telegram/SourceFiles/media/view/media_view_pip_renderer.h` | 38 | `Pip::Renderer` base virtual interface (`paintTransformedVideoFrame`, `paintTransformedStaticContent`, `paintRadialLoading`, `paintButtons`, `paintPlayback`, `paintVolumeController`) | `crates/gpui_telegram_media/src/media/view/media_view_pip_renderer.rs` |
| `Telegram/SourceFiles/media/view/media_view_pip_raster.h` | 57 | `Pip::RendererSW` software raster implementation | `crates/gpui_telegram_media/src/media/view/media_view_pip_raster.rs` |
| `Telegram/SourceFiles/media/view/media_view_pip_raster.cpp` | 166 | `media_view_pip_raster.cpp:L25-L95`: QPainter raster fallback for PiP video frames and buttons | `crates/gpui_telegram_media/src/media/view/media_view_pip_raster.rs` |
| `Telegram/SourceFiles/media/view/media_view_pip_opengl.h` | 93 | `Pip::RendererGL` OpenGL implementation with texture buffers and YUV/NV12 shaders | `crates/gpui_telegram_media/src/media/view/media_view_pip_opengl.rs` |
| `Telegram/SourceFiles/media/view/media_view_pip_opengl.cpp` | 592 | `media_view_pip_opengl.cpp:L45-L120`: GL textures and vertex coordinates mapping for PiP viewport | `crates/gpui_telegram_media/src/media/view/media_view_pip_opengl.rs` |
| `Telegram/SourceFiles/media/view/media_view_pip_rhi.h` | 143 | `Pip::RendererRhi` QRhi graphics pipeline for Vulkan/Metal/D3D11 | `crates/gpui_telegram_media/src/media/view/media_view_pip_rhi.rs` |
| `Telegram/SourceFiles/media/view/media_view_pip_rhi.cpp` | 948 | `media_view_pip_rhi.cpp:L60-L150`: QRhi shader resource bindings, sampler uniform buffers, render target attachments | `crates/gpui_telegram_media/src/media/view/media_view_pip_rhi.rs` |
| `Telegram/SourceFiles/media/view/media_view_overlay_renderer.h` | 58 | `OverlayWidget::Renderer` base virtual interface (`paintBackground`, `paintVideoStream`, `paintTransformedVideoFrame`, `paintTransformedStaticContent`, `paintRadialLoading`, `paintControl`, etc.) | `crates/gpui_telegram_media/src/media/view/media_view_overlay_renderer.rs` |
| `Telegram/SourceFiles/media/view/media_view_overlay_raster.h` | 86 | `OverlayWidget::RendererSW` QPainter software raster fallback | `crates/gpui_telegram_media/src/media/view/media_view_overlay_raster.rs` |
| `Telegram/SourceFiles/media/view/media_view_overlay_raster.cpp` | 374 | `media_view_overlay_raster.cpp:L20-L42`: `StoryCropRect`; `L77-L97`: `paintBackground` with notch handling; `L99-L150`: `paintTransformedImage` | `crates/gpui_telegram_media/src/media/view/media_view_overlay_raster.rs` |
| `Telegram/SourceFiles/media/view/media_view_overlay_opengl.h` | 177 | `OverlayWidget::RendererGL` OpenGL backend, `_textures` (image, siblings, Y, U, V), `kStoriesSiblingPartsCount = 4`, `kControlsCount = 8` | `crates/gpui_telegram_media/src/media/view/media_view_overlay_opengl.rs` |
| `Telegram/SourceFiles/media/view/media_view_overlay_opengl.cpp` | 1034 | `media_view_overlay_opengl.cpp:L80-L160`: GL textured quad rendering, YUV420 & NV12 conversion shader pipelines, controls overlay blending | `crates/gpui_telegram_media/src/media/view/media_view_overlay_opengl.rs` |
| `Telegram/SourceFiles/media/view/media_view_overlay_rhi.h` | 230 | `OverlayWidget::RendererRhi` QRhi rendering backend | `crates/gpui_telegram_media/src/media/view/media_view_overlay_rhi.rs` |
| `Telegram/SourceFiles/media/view/media_view_overlay_rhi.cpp` | 1707 | `media_view_overlay_rhi.cpp:L120-L220`: QRhi command buffers, pipeline caches, texture bindings, offscreen/onscreen passes | `crates/gpui_telegram_media/src/media/view/media_view_overlay_rhi.rs` |
| `Telegram/SourceFiles/media/view/media_view_metal_texture.h` | 47 | `MetalTextureCache`, `createTexturesFromPixelBuffer` (Y and UV texture extraction) | `crates/gpui_telegram_media/src/media/view/media_view_metal_texture.rs` |
| `Telegram/SourceFiles/media/view/media_view_metal_texture.mm` | 164 | `media_view_metal_texture.mm:L49-L53`: `kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange`, `kCVPixelFormatType_420YpCbCr8BiPlanarFullRange`, `CVMetalTextureCacheCreateTextureFromImage` | `crates/gpui_telegram_media/src/media/view/media_view_metal_texture.rs` |
| `Telegram/SourceFiles/media/view/media_view.style` | 1295 | `mediaviewWaitHide: 1100`, `mediaviewHideDuration: 600`, `mediaviewShowDuration: 200`, `mediaviewFadeDuration: 150`, `mediaviewSwipeDistance: 80px`, `mediaviewCaptionRadius: 6px`, `mediaviewGroupHeight: 80px`, `mediaviewGroupWidth: 56px`, `mediaviewGroupWidthMax: 160px`, `mediaviewDefaultWidth: 800px`, `mediaviewDefaultHeight: 600px` | `crates/gpui_telegram_desktop/src/theme/media/media_view_style.rs` |

---

## 3. Key Mathematical Algorithms, Constants & State Transitions

### 3.1 SnapXY Centering and Clamping (media_view_overlay_widget.cpp:L8093-L8107)

```rust
// Exact translation of tdesktop OverlayWidget::snapXY()
pub fn snap_xy(
    viewport_width: i32,
    viewport_height: i32,
    content_w: i32,
    content_h: i32,
    skip_top: i32,
    available_height: i32,
    min_used_top: i32,
    max_used_height: i32,
    full_screen_video: bool,
    mut x: i32,
    mut y: i32,
) -> (i32, i32) {
    let mut xmin = viewport_width - content_w;
    let mut xmax = 0;
    let mut ymin = viewport_height - content_h;
    let mut ymax = min_used_top;

    let center_x = (viewport_width - content_w) / 2;
    xmin = xmin.min(center_x);
    xmax = xmax.max(center_x);

    let center_y = if full_screen_video {
        min_used_top + (max_used_height - content_h) / 2
    } else {
        skip_top + (available_height - content_h) / 2
    };
    ymin = ymin.min(center_y);
    ymax = ymax.max(center_y);

    x = x.clamp(xmin, xmax);
    y = y.clamp(ymin, ymax);
    (x, y)
}
```

### 3.2 Zoom Matrix Progression & Clamping (media_view_overlay_widget.cpp:L7619-L7673, L2923-L2955)

The overlay viewer supports integer zoom levels from `-kMaxZoomLevel` (-7, 1/8 scale) up to `+kMaxZoomLevel` (+7, 8x scale), with a special sentinel value `kZoomToScreenLevel = 1024` indicating auto-fit to screen.
- At zoom level `z >= 0`: scale factor is `z + 1` (linear step increments: 1x, 2x, 3x, ..., 8x).
- At zoom level `z < 0`: scale factor is `1 / (-z + 1)` (fractional step decrements: 1/2, 1/3, ..., 1/8).
- Anchor-based zoom preservation: When zooming in/out around anchor point `(ax, ay)`:
  - Normalized anchor offset `nx = (x - ax) / (z + 1)` (or `nx = (x - ax) * (-z + 1)` if `z < 0`).
  - Target anchor offset `x' = nx * (z' + 1) + ax` (or `x' = nx / (-z' + 1) + ax` if `z' < 0`).

### 3.3 PiP Edge Snapping & Corner Transformation (media_view_pip.cpp:L69-L120)

- Screen border margin `pipBorderSkip = 12px` and magnetic snap threshold `pipBorderSnapArea = 20px`.
- When dragged within 20px of the left, top, right, or bottom margin, the panel magnetically snaps to that edge.
- 4-corner resizing recalculates geometry bounded by `minimalSize` and `maximalSize`, locking the opposite corner while moving the active corner.

### 3.4 Text Recognition Reading Order (media_view_recognition_selection.cpp:L21-L85)

1. Compute item vertical centers `centerY = rect.y + rect.h / 2`.
2. Initial vertical sort by `centerY`.
3. Cluster into rows: a new row begins whenever `centerY[k] - centerY[k-1] > max(h[k-1], h[k]) / 2`.
4. Secondary stable sort by `(row, rect.x)`.
5. Join into paragraph text inserting `\n` on row boundary changes and `' '` between words on the same row.

---

## 4. Verification Checklist & Gate Compliance

- [x] All 35 upstream source files manifested with zero omissions (Mandate #19).
- [x] Exact `.cpp:L<start>-L<end>` line ranges cited for all core algorithms and constants (Mandate #18).
- [x] Clean architecture in `crates/gpui_telegram_media/src/media/view/` replacing all Pattern D boilerplate stubs.
- [x] Subsystem tests with >= 10 unit and integration tests covering lightbox state machine, zoom matrix, edge clamping, and playback controls.
