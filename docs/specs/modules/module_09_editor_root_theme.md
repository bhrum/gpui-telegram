# Module 09 Specification: Editor Root Components & Theme Subsystem (`editor/*`, `theme/editor/*`)

## Overview

Module 09 Slice 7 delivers the core editing engine, visual canvas controls, media timelines, brush tools, color pickers, and global editor stylesheet metrics for GPUI Telegram Desktop. This subsystem provides comprehensive photo transformation and cropping, raster paint and drawing tools, vector shapes and typography presets, audio track timelines with waveform visualization, video trim and playback timelines with millisecond stamp parsing, link pill badges, message preview rendering, and full theme metric integration from `editor.style`.

This specification is authored in strict compliance with:
- **Mandate #15 (Zero Tolerance for Boilerplate)**: All generic boilerplate stubs (`{ id, label, value }` Patterns A, B, C, D) are completely eradicated and replaced with authentic domain models, exact mathematical algorithms, and live GPUI components.
- **Mandate #17 (Dynamic Target File Architecture)**: Upstream C++ source files serve as the authoritative reading checklist. Target Rust/GPUI architecture decomposes Qt widgets into idiomatic GPUI elements, state machines, and theme sheets.
- **Mandate #18 (Anti-Bluffing Protocol)**: Every component, algorithm, cache key, and metric cites exact upstream C++ implementation file line ranges (`.cpp:L<start>-L<end>`), header definitions (`.h:L<start>-L<end>`), and style tokens (`.style:L<start>-L<end>`), extracting anonymous-namespace `k...` constants and mathematical formulas.
- **Mandate #19 (Zero-Omitted-File Reading Guarantee)**: Every single one of the 44 assigned upstream C++ and style files in `Telegram/SourceFiles/editor/` (43 C++ files + 1 `.style` file) is accounted for in the 100% Per-File Reading Manifest below (`omitted_from_design_doc == 0`).
- **Mandate #20 (Stream Hygiene)**: Authored in modular segments under 200 lines per tool call.

---

## 1. 100% Per-File Reading Manifest (All 44 Upstream C++ & Style Sources)

| # | Upstream C++ Source Path | Exact Line Range Read | Upstream Symbols & Classes | Extracted Constants & Logic | Dynamic Rust Target Module & Disposition |
|---|---|---|---|---|---|
| 1 | `Telegram/SourceFiles/editor/audio_track_timeline.cpp` | `L1-L81` | `AudioTrackTimeline` | `resizeToWidth`, `photoEditorButtonBarHeight` | `crates/gpui_telegram_media/src/editor/audio_track_timeline.rs` (`AudioTrackTimeline`, `gpui_ui_1to1`) |
| 2 | `Telegram/SourceFiles/editor/audio_track_timeline.h` | `L1-L43` | `AudioTrackTimeline` | `setTrack`, `refreshTrim`, `commitPendingEdit` | `crates/gpui_telegram_media/src/editor/audio_track_timeline.rs` (`AudioTrackTimeline`, `gpui_ui_1to1`) |
| 3 | `Telegram/SourceFiles/editor/color_picker.cpp` | `L1-L1117` | `ColorPicker`, `ColorButton`, `PlusCircle`, `ToolLottieButton` | `kMinBrushSize=0.1`, `kMinBrushWidth=1.`, `kMaxBrushWidth=25.`, `kCircleDuration=200ms`, `kSizeControlSwitchDuration=140ms`, `PaletteColors` (10 colors), conical gradient 7 stops | `crates/gpui_telegram_media/src/editor/color_picker.rs` (`ColorPicker`, `gpui_ui_1to1`) |
| 4 | `Telegram/SourceFiles/editor/color_picker.h` | `L1-L121` | `ColorPicker` | `moveLine`, `setCanvasRect`, `sizeControlRatioFromY`, `sizeControlShapePath` | `crates/gpui_telegram_media/src/editor/color_picker.rs` (`ColorPicker`, `gpui_ui_1to1`) |
| 5 | `Telegram/SourceFiles/editor/editor.style` | `L1-L400` | Style definitions | `photoEditorControlsHeight: 146px`, `photoEditorButtonBarHeight: 48px`, `photoEditorMarkerOpacity: 0.35`, `photoEditorArrowHeadLengthFactor: 2.5`, `photoEditorBlurRadius: 20` | `crates/gpui_telegram_desktop/src/theme/editor/editor_style.rs` (`EditorStyleSheet`, `gpui_ui_1to1`) |
| 6 | `Telegram/SourceFiles/editor/editor_audio_disc_button.cpp` | `L1-L155` | `AudioDiscButton` | `kSpinDuration = 4000ms`, `angle = fmod(spinBase + passed * 360 / kSpinDuration, 360)`, `st::photoEditorAudioDiscCoverSize` | `crates/gpui_telegram_media/src/editor/editor_audio_disc_button.rs` (`EditorAudioDiscButton`, `gpui_ui_1to1`) |
| 7 | `Telegram/SourceFiles/editor/editor_audio_disc_button.h` | `L1-L58` | `AudioDiscButton` | `setTrack`, `setActive`, `angle`, `prepareRippleMask` | `crates/gpui_telegram_media/src/editor/editor_audio_disc_button.rs` (`EditorAudioDiscButton`, `gpui_ui_1to1`) |
| 8 | `Telegram/SourceFiles/editor/editor_audio_menu.cpp` | `L1-L229` | `MenuVolumeItem`, `AddVolumeAction`, `CreateAudioMenu` | `kMuteDuration = 150ms`, `kSpeakerThresholds = {0.05, 0.25, 0.75}`, `kVolumeStickedValues = {{0.25, 0.02}, {0.5, 0.02}, {0.75, 0.02}}` | `crates/gpui_telegram_media/src/editor/editor_audio_menu.rs` (`EditorAudioMenu`, `gpui_ui_1to1`) |
| 9 | `Telegram/SourceFiles/editor/editor_audio_menu.h` | `L1-L81` | `MenuVolumeItem` | `applyVolume`, `updateSliderColor`, `unmuteColor`, `muteColor` | `crates/gpui_telegram_media/src/editor/editor_audio_menu.rs` (`EditorAudioMenu`, `gpui_ui_1to1`) |
| 10 | `Telegram/SourceFiles/editor/editor_audio_timeline.cpp` | `L1-L299` | `AudioTimeline` | `kWaveformBarDuration = 20ms`, `kMaxWaveformBars = 16384`, `kLoadedDuration = 600ms`, `TrimDescriptor` | `crates/gpui_telegram_media/src/editor/editor_audio_timeline.rs` (`EditorAudioTimeline`, `gpui_ui_1to1`) |
| 11 | `Telegram/SourceFiles/editor/editor_audio_timeline.h` | `L1-L70` | `AudioTimeline` | `setPlaying`, `refreshTrim`, `loadWaveform`, `paintWaveform` | `crates/gpui_telegram_media/src/editor/editor_audio_timeline.rs` (`EditorAudioTimeline`, `gpui_ui_1to1`) |
| 12 | `Telegram/SourceFiles/editor/editor_crop.cpp` | `L1-L664` | `Crop`, `PointOfEdge`, `FlipSizeByRotation`, `AdjustCropToInner` | `kMaxCanvasRatio = 2.0`, `OriginalCrop`, `3x3 grid line painter`, `cornerLength = min(st::photoEditorCropPointSize * 2, min(w, h)/2)` | `crates/gpui_telegram_media/src/editor/editor_crop.rs` (`EditorCrop`, `gpui_ui_1to1`) |
| 13 | `Telegram/SourceFiles/editor/editor_crop.h` | `L1-L126` | `Crop` | `applyTransform`, `saveCropRect`, `setAspectRatio`, `setCornersLevel` | `crates/gpui_telegram_media/src/editor/editor_crop.rs` (`EditorCrop`, `gpui_ui_1to1`) |
| 14 | `Telegram/SourceFiles/editor/editor_keys_legend.cpp` | `L1-L302` | `KeysLegendButton`, `FixedCropRows`, `CropSection`, `CanvasSection` | Key mappings (`CtrlKey`, `ShiftKey`, `DeleteKey`), gesture strings | `crates/gpui_telegram_media/src/editor/editor_keys_legend.rs` (`EditorKeysLegend`, `gpui_ui_1to1`) |
| 15 | `Telegram/SourceFiles/editor/editor_keys_legend.h` | `L1-L50` | `KeysLegendButton`, `KeysLegendContext` | `toggle`, `showLegend`, `tooltipText` | `crates/gpui_telegram_media/src/editor/editor_keys_legend.rs` (`EditorKeysLegend`, `gpui_ui_1to1`) |
| 16 | `Telegram/SourceFiles/editor/editor_layer_widget.cpp` | `L1-L93` | `LayerWidget` | `BlurOverlayAlpha(night)`: night ? 128 : 192, forwardKeyEvent | `crates/gpui_telegram_media/src/editor/editor_layer_widget.rs` (`EditorLayerWidget`, `gpui_ui_1to1`) |
| 17 | `Telegram/SourceFiles/editor/editor_layer_widget.h` | `L1-L51` | `LayerWidget` | `parentResized`, `closeByOutsideClick`, `forwardKeyEvent` | `crates/gpui_telegram_media/src/editor/editor_layer_widget.rs` (`EditorLayerWidget`, `gpui_ui_1to1`) |
| 18 | `Telegram/SourceFiles/editor/editor_link_box.cpp` | `L1-L1009` | `LinkBox`, `LoadingMessage` | `kResolveDelay = 700ms`, `kLoadingTitleRatio = 0.45`, `kLoadingLineRatios = {0.32, 0.58, 0.86}`, `kGlareTimeout = 1000ms`, `kGlareDuration = 1000ms` | `crates/gpui_telegram_media/src/editor/editor_link_box.rs` (`EditorLinkBox`, `gpui_ui_1to1`) |
| 19 | `Telegram/SourceFiles/editor/editor_link_box.h` | `L1-L38` | `LinkBoxResult`, `LinkBoxArgs`, `LinkBox` | `LinkBoxResult` structure, dialog box constructor | `crates/gpui_telegram_media/src/editor/editor_link_box.rs` (`EditorLinkBox`, `gpui_ui_1to1`) |
| 20 | `Telegram/SourceFiles/editor/editor_link_pill.cpp` | `L1-L136` | `LinkPill`, `WithoutScheme` | `kDpPerWidth = 360.`, `kPaddingLeft = 4.`, `kPaddingTop = 4.33`, `kPaddingRight = 7.66`, `kPaddingBottom = 3.`, `kIconSize = 30.`, `kFontSize = 24.`, `kSemiTransparentAlpha = 0x4C` | `crates/gpui_telegram_media/src/editor/editor_link_pill.rs` (`EditorLinkPill`, `gpui_ui_1to1`) |
| 21 | `Telegram/SourceFiles/editor/editor_link_pill.h` | `L1-L59` | `LinkPreview`, `LinkStyle`, `LinkPill` | `DensityFor(canvasWidth) = canvasWidth / 360.`, `setStyle`, `paint` | `crates/gpui_telegram_media/src/editor/editor_link_pill.rs` (`EditorLinkPill`, `gpui_ui_1to1`) |
| 22 | `Telegram/SourceFiles/editor/editor_message_render.cpp` | `L1-L398` | `MessageRenderer`, `OpaqueBounds`, `CornerRadius` | `OpaqueBounds` alpha scanning, `CachedCornerRadiusValue`, `MakeChatTheme` | `crates/gpui_telegram_media/src/editor/editor_message_render.rs` (`EditorMessageRender`, `gpui_ui_1to1`) |
| 23 | `Telegram/SourceFiles/editor/editor_message_render.h` | `L1-L71` | `MessageRenderer` | `setRepaintCallback`, `setDark`, `mediaRect`, `videoMask`, `render` | `crates/gpui_telegram_media/src/editor/editor_message_render.rs` (`EditorMessageRender`, `gpui_ui_1to1`) |
| 24 | `Telegram/SourceFiles/editor/editor_message_source.cpp` | `L1-L136` | `MessageSource`, `MessageForbidsRender`, `CanRenderMessage` | `MessageToRender`, `watchRemoval` | `crates/gpui_telegram_media/src/editor/editor_message_source.rs` (`EditorMessageSource`, `gpui_ui_1to1`) |
| 25 | `Telegram/SourceFiles/editor/editor_message_source.h` | `L1-L59` | `MessageSource` | `session`, `item`, `link`, `webpage`, `video`, `removed` | `crates/gpui_telegram_media/src/editor/editor_message_source.rs` (`EditorMessageSource`, `gpui_ui_1to1`) |
| 26 | `Telegram/SourceFiles/editor/editor_message_video.cpp` | `L1-L186` | `MessageVideo` | `readSource`, `waitForLoader`, `fail`, `progress` | `crates/gpui_telegram_media/src/editor/editor_message_video.rs` (`EditorMessageVideo`, `gpui_ui_1to1`) |
| 27 | `Telegram/SourceFiles/editor/editor_message_video.h` | `L1-L58` | `MessageVideo` | `Find`, `document`, `load`, `loading`, `progress`, `source` | `crates/gpui_telegram_media/src/editor/editor_message_video.rs` (`EditorMessageVideo`, `gpui_ui_1to1`) |
| 28 | `Telegram/SourceFiles/editor/editor_paint.cpp` | `L1-L1197` | `Paint`, `BrushSize`, `DefaultShapeSize` | `kMaxBrush=25.`, `kMinBrush=1.`, `kShapeSizeRatio=2/5`, `kMediaSizeRatio=1/2`, `kImageMaxSizeRatio=4.`, `kMessageMaxWidthRatio=0.88`, `kMessageMaxHeightRatio=0.7`, `kMinCanvasZoom=1.`, `kMaxCanvasZoom=8.`, `kCanvasZoomStep=1.15` | `crates/gpui_telegram_media/src/editor/editor_paint.rs` (`EditorPaint`, `gpui_ui_1to1`) |
| 29 | `Telegram/SourceFiles/editor/editor_paint.h` | `L1-L203` | `Paint` | `saveScene`, `restoreScene`, `applyTransform`, `applyBrush`, `zoomSceneItems` | `crates/gpui_telegram_media/src/editor/editor_paint.rs` (`EditorPaint`, `gpui_ui_1to1`) |
| 30 | `Telegram/SourceFiles/editor/editor_trim_timeline.cpp` | `L1-L1208` | `TrimTimeline`, `Stamp`, `ParseStamp` | `kZoomStep=1.2`, `kMinVisibleSpan=2000ms`, `kMinSelection=100ms`, `kEdgeScrollSpeed=9.`, `Stamp` precision 3 formatting, `ParseStamp` milliseconds parser | `crates/gpui_telegram_media/src/editor/editor_trim_timeline.rs` (`EditorTrimTimeline`, `gpui_ui_1to1`) |
| 31 | `Telegram/SourceFiles/editor/editor_trim_timeline.h` | `L1-L208` | `TrimTimeline`, `TrimTimelineDescriptor` | `from`, `till`, `cover`, `setTrim`, `setPlaybackPosition`, `timeAt`, `xAt` | `crates/gpui_telegram_media/src/editor/editor_trim_timeline.rs` (`EditorTrimTimeline`, `gpui_ui_1to1`) |
| 32 | `Telegram/SourceFiles/editor/photo_editor.cpp` | `L1-L713` | `PhotoEditor`, `DefaultBrushes`, `Serialize` | `kPrecision=100000`, `kBrushesVersion=-2`, `kDefaultBrushSizeRatio=0.9`, default colors: Pen(234,39,57), Arrow(252,150,77), Marker(252,222,101), Eraser(0,0,0), Blur(0,0,0) | `crates/gpui_telegram_media/src/editor/photo_editor.rs` (`PhotoEditor`, `gpui_ui_1to1`) |
| 33 | `Telegram/SourceFiles/editor/photo_editor.h` | `L1-L100` | `PhotoEditor`, `InitEditorLayer` | `save`, `doneRequests`, `cancelRequests`, `updateColorPickerVisibility` | `crates/gpui_telegram_media/src/editor/photo_editor.rs` (`PhotoEditor`, `gpui_ui_1to1`) |
| 34 | `Telegram/SourceFiles/editor/photo_editor_common.cpp` | `L1-L309` | `ApplyShapeMask`, `RoundedCornersMultiplier`, `ExpandCanvas` | `kAnimatedMaxSide=854`, `kAnimatedFps=30.`, `kAnimatedMinDuration=1000ms`, `RoundedCornersMultiplier`: Circle=0.5, Large=0.44, Medium=0.2, Small=0.12, None=0.0 | `crates/gpui_telegram_media/src/editor/photo_editor_common.rs` (`PhotoEditorCommon`, `gpui_ui_1to1`) |
| 35 | `Telegram/SourceFiles/editor/photo_editor_common.h` | `L1-L100` | `AudioTrack`, `RoundedCornersLevel`, `EditorData`, `PhotoModifications` | `AudioTrack::length`, `PhotoModifications::empty`, `ImageModified`, `ComposeAnimatedJob` | `crates/gpui_telegram_media/src/editor/photo_editor_common.rs` (`PhotoEditorCommon`, `gpui_ui_1to1`) |
| 36 | `Telegram/SourceFiles/editor/photo_editor_content.cpp` | `L1-L631` | `PhotoEditorContent`, `MaxDistance` | `kExpansionRoomRatio=0.7`, `kLayoutSmoothTau=60.`, `kLayoutMaxFrameDelta=64ms`, `kLayoutEpsilon=0.5`, `MaxDistance` formula | `crates/gpui_telegram_media/src/editor/photo_editor_content.rs` (`PhotoEditorContent`, `gpui_ui_1to1`) |
| 37 | `Telegram/SourceFiles/editor/photo_editor_content.h` | `L1-L128` | `PhotoEditorContent` | `applyModifications`, `applyMode`, `applyBrush`, `createTextItem`, `createShapeItem` | `crates/gpui_telegram_media/src/editor/photo_editor_content.rs` (`PhotoEditorContent`, `gpui_ui_1to1`) |
| 38 | `Telegram/SourceFiles/editor/photo_editor_controls.cpp` | `L1-L1396` | `PhotoEditorControls`, `CheckAction`, `ShapeAction`, `EdgeButton` | `EdgeButton` ripple calculation, `RoundCheckAction` animation, button top alignment | `crates/gpui_telegram_media/src/editor/photo_editor_controls.rs` (`PhotoEditorControls`, `gpui_ui_1to1`) |
| 39 | `Telegram/SourceFiles/editor/photo_editor_controls.h` | `L1-L149` | `PhotoEditorControls` | `rotateRequests`, `flipRequests`, `paintModeRequests`, `setShapeToolActive` | `crates/gpui_telegram_media/src/editor/photo_editor_controls.rs` (`PhotoEditorControls`, `gpui_ui_1to1`) |
| 40 | `Telegram/SourceFiles/editor/photo_editor_inner_common.h` | `L1-L106` | `PhotoEditorMode`, `Brush`, `TextStyle`, `TextTypeface`, `TextAlignment`, `TextPrefs`, `ShapeType`, `ShapeRequest`, `VideoTrim` | Core domain enums and structs for tools, brush parameters, text formatting, and video trim | `crates/gpui_telegram_media/src/editor/photo_editor_inner_common.rs` (`PhotoEditorInnerCommon`, `gpui_ui_1to1`) |
| 41 | `Telegram/SourceFiles/editor/photo_editor_layer_widget.cpp` | `L1-L182` | `OpenWithPreparedFile`, `PrepareProfilePhoto`, `PrepareProfilePhotoFromFile` | `ValidateThumbDimensions`, `PhotoSideLimit`, layer callback flow | `crates/gpui_telegram_media/src/editor/photo_editor_layer_widget.rs` (`PhotoEditorLayerWidget`, `gpui_ui_1to1`) |
| 42 | `Telegram/SourceFiles/editor/photo_editor_layer_widget.h` | `L1-L57` | `kProfilePhotoSize = 640` | `OpenWithPreparedFile`, `PrepareProfilePhoto`, `PrepareProfilePhotoFromFile` | `crates/gpui_telegram_media/src/editor/photo_editor_layer_widget.rs` (`PhotoEditorLayerWidget`, `gpui_ui_1to1`) |
| 43 | `Telegram/SourceFiles/editor/video_item_timeline.cpp` | `L1-L112` | `VideoItemTimeline` | `VideoTimelineDescriptor`, `TimelineSeeker`, volume action popup | `crates/gpui_telegram_media/src/editor/video_item_timeline.rs` (`VideoItemTimeline`, `gpui_ui_1to1`) |
| 44 | `Telegram/SourceFiles/editor/video_item_timeline.h` | `L1-L51` | `VideoItemTimeline` | `setClip`, `refreshTrim`, `commitPendingEdit`, `lengthChanges` | `crates/gpui_telegram_media/src/editor/video_item_timeline.rs` (`VideoItemTimeline`, `gpui_ui_1to1`) |

### Reading Manifest Verification Summary

- **Total Assigned Upstream Files**: 44 / 44 (100.0% coverage, 0 omitted)
  - Root `Telegram/SourceFiles/editor/` C++ files: 43 entries (21 `.h`/`.cpp` pairs + 1 `.h` `photo_editor_inner_common.h`)
  - Root style file: 1 entry (`Telegram/SourceFiles/editor/editor.style`)
- **Target Rust Files in Scope**:
  - `crates/gpui_telegram_media/src/editor/*.rs` (22 root modules)
  - `crates/gpui_telegram_desktop/src/theme/editor/editor_style.rs` (1 theme stylesheet)
- **Zero Boilerplate Guarantee**: All 23 target files transition completely from autogenerated `{ id, label, value }` and `{ id, tag, value }` stubs to authentic, mathematically faithful domain models and state machines.


---

## 2. Deep Source Analysis & Mathematical Algorithms

### 2.1 Color Picker, HSV Color Wheel & Size Control (`color_picker.cpp:L27-L300`)

1. **Brush Size Bounds & Interpolation** (`color_picker.cpp:L28-L43`):
   - Minimum brush size normalized: `kMinBrushSize = 0.1`
   - Minimum brush pixel width: `kMinBrushWidth = 1.0`
   - Maximum brush pixel width: `kMaxBrushWidth = 25.0`
   - Linear interpolation:
     $$\text{InterpolateF}(a, b, t) = a + (b - a) \times t$$
     $$\text{InterpolationRatio}(\text{from}, \text{to}, \text{val}) = \frac{\text{val} - \text{from}}{\text{to} - \text{from}}$$
2. **Brush Tools & Fixed Color Tools** (`color_picker.cpp:L44-L78`):
   - Tool Indices: Pen (0), Arrow (1), Marker (2), Blur (3), Eraser (4)
   - Fixed Color Tools: `FixedColorTool(tool)` returns `true` for `Eraser` and `Blur`. Their color is fixed to `QColor(0, 0, 0)` (Black).
3. **Color Wheel Conical Gradient** (`color_picker.cpp:L143-L151`):
   The outer rainbow ring is rendered via a conical gradient centered at the button with 7 discrete angle stops:
   - $0/7$: `#EB4B4B` (Red)
   - $1/7$: `#FFA500` (Orange)
   - $2/7$: `#FFFF00` (Yellow)
   - $3/7$: `#8FCE00` (Green)
   - $4/7$: `#00FFFF` (Cyan)
   - $5/7$: `#6080E4` (Blue)
   - $6/7$: `#EE82EE` (Violet)
   - $7/7$: `#EB4B4B` (Red loop)
4. **Default Color Palette** (`color_picker.cpp:L266-L279`):
   The quick-select palette provides 10 standard colors:
   - `#000000` (Black), `#FFFFFF` (White), `#EA2739` (Red), `#FC964D` (Orange), `#FCDE65` (Yellow),
   - `#80C864` (Green), `#49C5ED` (Cyan/Light Blue), `#3051E3` (Blue), `#DB3AD2` (Magenta), `#FF72A9` (Pink).
5. **Size Control Geometry & Handle** (`color_picker.cpp:L61-L72`):
   - Collapsed width: 6px; Expanded top width: 25px; Expanded bottom width: 4px.
   - Ratio to Y coordinate mapping:
     $$y(\text{ratio}) = y_{\text{bottom}} - \text{ratio} \times (y_{\text{bottom}} - y_{\text{top}})$$
     $$\text{ratio}(y) = \frac{y_{\text{bottom}} - y}{y_{\text{bottom}} - y_{\text{top}}}$$

### 2.2 Paint & Brush Engine (`editor_paint.cpp:L54-L147`, `editor.style:L186-L195`)

1. **Brush Size Computation** (`editor_paint.cpp:L81-L84`):
   $$\text{BrushSize}(\text{brush}) = kMinBrush + (kMaxBrush - kMinBrush) \times \text{brush.sizeRatio}$$
   Where $kMinBrush = 1.0$ and $kMaxBrush = 25.0$.
2. **Default Shape & Media Ratios** (`editor_paint.cpp:L56-L63`):
   - Shape Size Ratio: $kShapeSizeRatio = 2/5 = 0.40$ of the smaller image dimension.
   - Media Size Ratio: $kMediaSizeRatio = 1/2 = 0.50$.
   - Max Image Size Ratio: $kImageMaxSizeRatio = 4.0$.
   - Message Max Bounds: Width ratio $0.88$, Height ratio $0.70$, Cascade offset ratio $1/20 = 0.05$.
3. **Special Brush Characteristics** (`editor.style:L186-L195`):
   - **Marker**: Opacity $0.35$, Size Multiplier $2.5\times$.
   - **Arrow**: Head Length Factor $2.5\times$, Min Distance Factor $1.5\times$, Head Angle $26^\circ$.
   - **Eraser**: Preview Opacity $0.25$.
   - **Blur**: Size Multiplier $3.0\times$, Preview Opacity $0.25$, Blur Kernel Radius $20\text{px}$.
4. **Canvas Zoom & Smoothing** (`editor_paint.cpp:L92-L102`):
   - Zoom range: $kMinCanvasZoom = 1.0$ to $kMaxCanvasZoom = 8.0$.
   - Step: $1.15\times$ (normal wheel), $1.015\times$ (fine wheel with Shift).
   - Smoothing: Exponential smoothing filter with time constant $kZoomSmoothTau = 60.0\text{ms}$.
   - Text item default font size: $\text{fontSize} = \min(\text{width}, \text{height}) / 15.0$.

### 2.3 Crop Controls & Geometry Transforms (`editor_crop.cpp:L17-L83`, `photo_editor_common.cpp:L93-L100`)

1. **Rotation & Flip Math** (`editor_crop.cpp:L49-L51`):
   - When angle is rotated by $90^\circ$ or $270^\circ$ (`((angle / 90) % 2) == 1`), width and height are transposed.
   - Scaled matrix incorporates center translation, horizontal flip (`scale(-1, 1)` if flipped), and rotation.
2. **Rounded Corners Multipliers** (`photo_editor_common.cpp:L93-L100`):
   - `Circle`: $0.50$ (half of min dimension = circular pill/ellipse)
   - `Large`: $0.44$ (Squircle / Forum Monoforum avatar radius)
   - `Medium`: $0.20$
   - `Small`: $0.12$
   - `None`: $0.00$
3. **Rule of Thirds Grid Painting** (`editor_crop.cpp:L257-L281`):
   - Grid divides crop area into a $3 \times 3$ grid with horizontal steps at $w/3$ and $2w/3$, and vertical steps at $h/3$ and $2h/3$.
   - Corner handle lines: length $\min(20\text{px}, \min(w, h) / 2)$, line thickness $4 \times \text{st::lineWidth}$.


### 2.4 Timelines & Playback Control (`editor_trim_timeline.cpp:L25-L100`, `editor_audio_disc_button.cpp:L23-L93`, `editor_audio_menu.cpp:L25-L35`)

1. **Trim Timeline Bounds & Stamp Parsing** (`editor_trim_timeline.cpp:L25-L100`):
   - $kMinVisibleSpan = 2000\text{ms}$ (minimum visible span on screen)
   - $kMinSelection = 100\text{ms}$ (minimum allowed trim clip duration)
   - $kZoomStep = 1.20$, $kZoomSnap = 0.001$
   - $kEdgeScrollSpeed = 9.0$ pixels per second overshoot, max $3.0\times$ strip width/sec.
   - Timestamp parsing (`ParseStamp`): parses `SS.mmm`, `MM:SS.mmm`, or `HH:MM:SS.mmm` into exact integer milliseconds with precision up to 3 fractional digits.
   - Timestamp formatting (`Stamp`): formats milliseconds into `MM:SS.f` with customizable decimal precision.
2. **Audio Disc Spinning Animation** (`editor_audio_disc_button.cpp:L23-L93`):
   - Full rotation duration: $kSpinDuration = 4000\text{ms}$ (4.0 seconds per 360-degree rotation).
   - Angle at time $t$:
     $$\theta(t) = \left(\theta_0 + \frac{(t - t_{\text{start}}) \times 360.0}{4000.0}\right) \pmod{360.0}$$
3. **Volume Slider Magnetic Snapping** (`editor_audio_menu.cpp:L27-L34`):
   - Speaker thresholds for icon arcs: `[0.05, 0.25, 0.75]`.
   - Magnetic snap points: `0.25` (snap window $\pm 0.02$), `0.50` ($\pm 0.02$), `0.75` ($\pm 0.02$).

### 2.5 Link Previews, Pills & Message Rendering (`editor_link_pill.cpp:L16-L97`, `editor_link_box.cpp:L47-L52`, `editor_message_render.cpp:L35-L78`)

1. **Link Pill Density & Sizing** (`editor_link_pill.cpp:L16-L63`):
   - Canvas baseline width: $kDpPerWidth = 360.0\text{px}$.
   - Density: $\text{density} = \text{canvasWidth} / 360.0$.
   - Padding metrics: Left $4.0\text{px}$, Top $4.33\text{px}$, Right $7.66\text{px}$, Bottom $3.0\text{px}$, Icon Size $30.0\text{px}$, Icon Padding $3.25\text{px}$, Font Size $24.0\text{px}$.
   - Pill Style Background & Foreground:
     - `Framed`: Background `#FFFFFF`, Foreground `#3391D4`.
     - `SemiTransparent`: Background `#000000` with alpha `0x4C` (76/255 = 30%), Foreground `#FFFFFF`.
     - `Opaque`: Background `#000000`, Foreground `#FFFFFF`.
2. **Loading Element Line Ratios** (`editor_link_box.cpp:L48-L51`):
   - Title placeholder ratio: $0.45$.
   - Content line width ratios: `[0.32, 0.58, 0.86]`.
   - Glare animation duration: $1000\text{ms}$, timeout: $1000\text{ms}$.
3. **Opaque Bounds Alpha Scanning** (`editor_message_render.cpp:L35-L78`):
   - Computes tightest bounding rectangle $R = [x_{\min}, y_{\min}, w, h]$ containing non-zero alpha pixels by scanning scanlines row-by-row and column-by-column.

### 2.6 Editor Stylesheet & Metric System (`editor.style:L16-L400`)

The GPUI `EditorStyleSheet` embodies all metric tokens from `editor.style`:
- Controls Bar Height: `photoEditorControlsHeight: 146px`
- Bottom Skip: `photoEditorControlsBottomSkip: 20px`, Center Skip: `photoEditorControlsCenterSkip: 6px`
- Button Bar Height: `photoEditorButtonBarHeight: 48px`, Width: `422px`
- Button Padding: Horizontal $2\text{px}$; Text Button Padding: Horizontal $22\text{px}$
- Color Button Size: `24px`, Border: `1px`, Palette Item: `20px`, Gap: `6px`, Selection Width: `2px`
- Tool Button Size: `20px`, Gap: `18px`, Hover Duration: `350ms`, Select Duration: `120ms`
- Brush Control Height: `280px`, Collapsed Width: `6px`, Top Inset: `1px`, Hit Padding: `24px`
- Crop Point Size: `10px`, Min Size: `20px`, Item Handle Size: `10px`, Sticky Trigger: `12px`
- Audio Disc Size: `48px`, Skip: `12px`, Cover Size: `30px`, Ring: `2px`
- Audio Timeline Bar Width: `2px`, Skip: `3px`, Min: `1px`, Ratio: `0.6`, Opacity: `0.4`
- Video Timeline Strip Height: `54px`, Label Height: `18px`, Trim Strip Height: `32px`
- Video Timeline Dot Size: `5px`, Active: `9px`, Radius: `6px`, Handle Width: `11px`
- Video Quality Slider Dot Size: `7px`, Active: `11px`, Line Width: `2px`, Height: `32px`
