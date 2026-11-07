# Module 09 Specification: Editor Scene Graph, Controllers & Video Timeline (`editor/scene`, `editor/controllers`, `editor/video`)

## Overview

Module 09 Slice 6 delivers the complete interactive graphics scene graph, affine transformations, sticky snapping guidelines, vector drawing canvas, text editing controller, stickers/emoji integration, undo/redo stack, and video timeline editing subsystem for Telegram Desktop reconstructed in pure, idiomatic Rust.

This specification is authored in strict compliance with:
- **Mandate #15 (Zero Tolerance for Boilerplate)**: All generic boilerplate stubs (`{ id, label, value }` Patterns A, B, C, D) are completely eliminated across `crates/gpui_telegram_media/src/editor/scene/`, `crates/gpui_telegram_media/src/editor/controllers/`, and `crates/gpui_telegram_media/src/editor/video/`.
- **Mandate #17 (Dynamic Target File Architecture)**: Upstream C++ files serve as the reading checklist. The target architecture provides fine-grained, authentic Rust modules and state machines with clean mathematical abstractions.
- **Mandate #18 (Anti-Bluffing Protocol)**: Every component, transform algorithm, and timeline state machine cites exact upstream C++ implementation file line ranges (`.cpp:L<start>-L<end>`) and header definitions (`.h:L<start>-L<end>`), extracting anonymous-namespace `k...` constants, geometric equations, and edge-case behaviors.
- **Mandate #19 (Zero-Omitted-File Reading Guarantee)**: 100% of the 51 assigned upstream C++ source and header files are explicitly present, documented, and accounted for in the Per-File Reading Manifest below (`omitted_from_design_doc == 0`).
- **Mandate #20 (Stream Hygiene)**: Written in modular chunks under ~150-200 lines to prevent timeout.

---

## 1. 100% Per-File Reading Manifest (All 51 Assigned Upstream Sources)

| # | Upstream C++ Source Path | Exact Line Range Read | Upstream Symbols & Classes | Extracted Constants & Logic | Dynamic Rust Target Module & Disposition |
|---|---|---|---|---|---|
| 1 | `Telegram/SourceFiles/editor/scene/scene.cpp` | `L1-L1262` | `Editor::Scene`, `ItemAction`, `ItemEraser`, `StickyGuidesItem`, `CapturedPlacement`, `PendingShape` | Scene graph lifecycle, undo/redo stack (`_items`, `_itemNumber`), sticky guidelines rendering with animation (`kStickyDuration = 150ms`), shape placement, text edit dispatch, video clip selection synchronization | `crates/gpui_telegram_media/src/editor/scene/scene.rs` (`SceneScene`, `gpui_ui_1to1`) |
| 2 | `Telegram/SourceFiles/editor/scene/scene.h` | `L1-L217` | `Editor::Scene`, `PendingShape`, `CapturedPlacement` | Scene class interface: `setCanvasRect`, `applyBrush`, `setTextDefaults`, `addItem`, `removeItem`, `canEqualizeDurations`, `equalizeDurations`, `hasUndo`, `hasRedo`, `performUndo`, `performRedo` | `crates/gpui_telegram_media/src/editor/scene/scene.rs` (`SceneScene`, `gpui_ui_1to1`) |
| 3 | `Telegram/SourceFiles/editor/scene/scene_emoji_document.cpp` | `L1-L263` | `EmojiDocument`, `ReplaceEmoji`, `ReplaceEmojiInRange`, `SanitizeRange`, `RecoverTextFromDocument`, `RecoverWrappedTextFromDocument` | Custom emoji loading via resource loader `emoji://` URI scheme; device pixel ratio scaling; text document entity serialization and deserialization with line-wrapping recovery | `crates/gpui_telegram_media/src/editor/scene/scene_emoji_document.rs` (`SceneEmojiDocumentScene`, `gpui_ui_1to1`) |
| 4 | `Telegram/SourceFiles/editor/scene/scene_emoji_document.h` | `L1-L29` | `EmojiDocument`, `ReplaceEmoji`, `SanitizeRange` | Rich text emoji document definition and emoji parsing utility function prototypes | `crates/gpui_telegram_media/src/editor/scene/scene_emoji_document.rs` (`SceneEmojiDocumentScene`, `gpui_ui_1to1`) |
| 5 | `Telegram/SourceFiles/editor/scene/scene_item_animated.cpp` | `L1-L108` | `ItemAnimated`, `entityKind`, `entityRect`, `visibleRect`, `paintFrame` | Base class for animated media scene elements (stickers, video, animated messages); keying cache, loop duration tracking, frame playback and mirror flipping | `crates/gpui_telegram_media/src/editor/scene/scene_item_animated.rs` (`SceneItemAnimatedScene`, `gpui_ui_1to1`) |
| 6 | `Telegram/SourceFiles/editor/scene/scene_item_animated.h` | `L1-L53` | `ItemAnimated` | Abstract interface for animated scene items: `animated()`, `hasContent()`, `content()`, `loopDuration()`, `trim()`, `releasePlayers()`, `animatedEntity()` | `crates/gpui_telegram_media/src/editor/scene/scene_item_animated.rs` (`SceneItemAnimatedScene`, `gpui_ui_1to1`) |
| 7 | `Telegram/SourceFiles/editor/scene/scene_item_base.cpp` | `L1-L872` | `NumberedItem`, `ItemBase`, `kSnapAngle = 45.`, `kMinSizeRatio = 0.05`, `kStickyDuration = 150ms`, `kStickyLines = 5` | Affine transformation interaction: left/right handles, rotation angle calculation `atan2(dy, dx)*180/PI`, 45-degree angle snapping (`kSnapAngle`), size clamping, 5-line sticky snapping (`kStickyLines`), cubic ease-out animation | `crates/gpui_telegram_media/src/editor/scene/scene_item_base.rs` (`SceneItemBaseScene`, `gpui_ui_1to1`) |
| 8 | `Telegram/SourceFiles/editor/scene/scene_item_base.h` | `L1-L267` | `NumberedItem`, `ItemBase`, `Placement`, `Sticky`, `StickyAxis`, `HandleType` | Comprehensive base item definitions: placement state `{ position, rotation, scale, zValue, size, aspectRatio, bend, fontSize, flipped }`, handle hit-testing, context menu, duplication | `crates/gpui_telegram_media/src/editor/scene/scene_item_base.rs` (`SceneItemBaseScene`, `gpui_ui_1to1`) |
| 9 | `Telegram/SourceFiles/editor/scene/scene_item_canvas.cpp` | `L1-L518` | `ItemCanvas`, `kMinPointDistanceBase = 2.0`, `kMaxPointDistance = 15.0`, `kSmoothingStrength = 0.5`, `kSegmentOverlap = 3` | Freehand drawing canvas stroke smoothing algorithm: distance threshold, quadratic Bezier curve interpolation, blur tool offscreen clipping, eraser blending | `crates/gpui_telegram_media/src/editor/scene/scene_item_canvas.rs` (`SceneItemCanvasScene`, `gpui_ui_1to1`) |
| 10 | `Telegram/SourceFiles/editor/scene/scene_item_canvas.h` | `L1-L110` | `ItemCanvas`, `Content`, `StrokePoint` | Freehand drawing canvas graphics item interface: `applyBrush`, `setCanvasRect`, `drawableRect`, mouse stroke capturing stream | `crates/gpui_telegram_media/src/editor/scene/scene_item_canvas.rs` (`SceneItemCanvasScene`, `gpui_ui_1to1`) |
| 11 | `Telegram/SourceFiles/editor/scene/scene_item_image.cpp` | `L1-L48` | `ItemImage`, `paint`, `performFlip`, `duplicate` | Static raster image layer item: aspect ratio preservation, horizontal reflection, duplicate offset by `size / 3` | `crates/gpui_telegram_media/src/editor/scene/scene_item_image.rs` (`SceneItemImageScene`, `gpui_ui_1to1`) |
| 12 | `Telegram/SourceFiles/editor/scene/scene_item_image.h` | `L1-L32` | `ItemImage` | Header definition for static image scene items | `crates/gpui_telegram_media/src/editor/scene/scene_item_image.rs` (`SceneItemImageScene`, `gpui_ui_1to1`) |
| 13 | `Telegram/SourceFiles/editor/scene/scene_item_line.cpp` | `L1-L93` | `ItemLine`, `applyEraser`, `setPixmap` | Persistent painted vector brush stroke line item; eraser application via mask subtraction; undo state preservation | `crates/gpui_telegram_media/src/editor/scene/scene_item_line.rs` (`SceneItemLineScene`, `gpui_ui_1to1`) |
| 14 | `Telegram/SourceFiles/editor/scene/scene_item_line.h` | `L1-L47` | `ItemLine` | Interface definition for completed brush stroke lines | `crates/gpui_telegram_media/src/editor/scene/scene_item_line.rs` (`SceneItemLineScene`, `gpui_ui_1to1`) |
| 15 | `Telegram/SourceFiles/editor/scene/scene_item_link.cpp` | `L1-L181` | `ItemLink`, `kMaxWidthRatio = 0.9`, `NextStyle`, `PreviousStyle` | Story link preview pill widget: 3 cycling styles (`Framed`, `SemiTransparent`, `Opaque`), density computation from image width, URL editing callback | `crates/gpui_telegram_media/src/editor/scene/scene_item_link.rs` (`SceneItemLinkScene`, `gpui_ui_1to1`) |
| 16 | `Telegram/SourceFiles/editor/scene/scene_item_link.h` | `L1-L59` | `ItemLink` | Header declaration for story link pills | `crates/gpui_telegram_media/src/editor/scene/scene_item_link.rs` (`SceneItemLinkScene`, `gpui_ui_1to1`) |
| 17 | `Telegram/SourceFiles/editor/scene/scene_item_message.cpp` | `L1-L499` | `ItemMessage`, `kMaxRatio = 8`, `kPlaybackFrameSide = 1024`, `CoverRect` | Forwarded message quote sticker item: video playback integration, dynamic ratio scaling, cover rect calculation, theme color styling | `crates/gpui_telegram_media/src/editor/scene/scene_item_message.rs` (`SceneItemMessageScene`, `gpui_ui_1to1`) |
| 18 | `Telegram/SourceFiles/editor/scene/scene_item_message.h` | `L1-L121` | `ItemMessage`, `MessageVideoOptions` | Forwarded quote message item interface: video playback options, message renderer binding | `crates/gpui_telegram_media/src/editor/scene/scene_item_message.rs` (`SceneItemMessageScene`, `gpui_ui_1to1`) |
| 19 | `Telegram/SourceFiles/editor/scene/scene_item_shape.cpp` | `L1-L463` | `ItemShape`, `kStarInnerRatio = 0.4316`, `kRoundingRatio = 0.234`, `kBubbleBodyRatio = 2./2.25`, `kBubbleTailRatio = 0.75`, `kArrowHeadRatio = 0.16`, `kArrowHeadStrokeFactor = 2.5`, `kArrowHeadAngle = 30 deg`, `kArrowMaxBendRatio = 0.35`, `kStarBoxWidth = 1.902113`, `kStarBoxHeight = 1.809017` | Vector geometric shapes: Rectangle, Circle, Star (golden ratio 5-point math), Speech Bubble with tail, Arrow with quadratic curved shaft and triangular arrowhead; live bend control point dragging | `crates/gpui_telegram_media/src/editor/scene/scene_item_shape.rs` (`SceneItemShapeScene`, `gpui_ui_1to1`) |
| 20 | `Telegram/SourceFiles/editor/scene/scene_item_shape.h` | `L1-L93` | `ItemShape`, `ArrowPoints` | Vector shape graphics item interface: color, stroke width, fill, live draft frame sizing, and bend ratio | `crates/gpui_telegram_media/src/editor/scene/scene_item_shape.rs` (`SceneItemShapeScene`, `gpui_ui_1to1`) |
| 21 | `Telegram/SourceFiles/editor/scene/scene_item_sticker.cpp` | `L1-L236` | `ItemSticker`, `updateThumbnail`, `createPlayer` | Static and animated sticker items: Lottie and WebM player initialization, high-resolution thumbnail fallback, aspect ratio preservation, flipping | `crates/gpui_telegram_media/src/editor/scene/scene_item_sticker.rs` (`SceneItemStickerScene`, `gpui_ui_1to1`) |
| 22 | `Telegram/SourceFiles/editor/scene/scene_item_sticker.h` | `L1-L73` | `ItemSticker` | Header interface for sticker graphics scene elements | `crates/gpui_telegram_media/src/editor/scene/scene_item_sticker.rs` (`SceneItemStickerScene`, `gpui_ui_1to1`) |
| 23 | `Telegram/SourceFiles/editor/scene/scene_item_text.cpp` | `L1-L1015` | `ItemText`, `kPaddingFactor = 0.4`, `kMaxWidthFactor = 0.8`, `kMinContentWidth = 20`, `kBrightnessFramedThreshold = 0.721`, `kBrightnessSemiTransparentThreshold = 0.25`, `kSemiTransparentAlpha = 0x99`, `kCornerRadiusFactor = 1./3.`, `kLinePadHFactor = 1./3.`, `kMergeRadiusFactor = 1.5`, `kCondensedStretch = 78` | Text formatting & layout: 4 typefaces, multi-line background bubbles, bubble corner radius rounding and vertical line merging, luminance contrast calculations, alignment adjustments | `crates/gpui_telegram_media/src/editor/scene/scene_item_text.rs` (`SceneItemTextScene`, `gpui_ui_1to1`) |
| 24 | `Telegram/SourceFiles/editor/scene/scene_item_text.h` | `L1-L136` | `ItemText`, `TextLayoutSpec`, `TextBackgroundLine`, `BuildTextBackgroundPath` | Text scene item interface and layout specification data structures | `crates/gpui_telegram_media/src/editor/scene/scene_item_text.rs` (`SceneItemTextScene`, `gpui_ui_1to1`) |
| 25 | `Telegram/SourceFiles/editor/scene/scene_item_video.cpp` | `L1-L141` | `ItemVideo`, `kPlaybackFrameSide = 512`, `FrameSizeFor` | Embedded video clip item: playback thumbnail downscaling to 512px box, video clip player binding, looping trim window synchronization | `crates/gpui_telegram_media/src/editor/scene/scene_item_video.rs` (`SceneItemVideoScene`, `gpui_ui_1to1`) |
| 26 | `Telegram/SourceFiles/editor/scene/scene_item_video.h` | `L1-L54` | `ItemVideo` | Video item scene element interface | `crates/gpui_telegram_media/src/editor/scene/scene_item_video.rs` (`SceneItemVideoScene`, `gpui_ui_1to1`) |
| 27 | `Telegram/SourceFiles/editor/scene/scene_text_editing.cpp` | `L1-L708` | `TextEditController`, `TextEditProxy`, `kMinWidthFactor = 0.16`, `kIdealWidthExtra = 2`, `kScaleThreshold = 0.01`, `kPlaceholderOpacity = 0.38`, `kAutoShrinkHeightFactor = 1./3.`, `kAutoShrinkStep = 0.9`, `kMinFontSizeFactor = 1./50.` | Inline text editor controller: text proxy placement over scene item, auto-shrink algorithm when text overflows canvas height, placeholder rendering, color change dispatch | `crates/gpui_telegram_media/src/editor/scene/scene_text_editing.rs` (`SceneTextEditingScene`, `gpui_ui_1to1`) |
| 28 | `Telegram/SourceFiles/editor/scene/scene_text_editing.h` | `L1-L90` | `TextEditController` | Interface for managing active scene text editing sessions | `crates/gpui_telegram_media/src/editor/scene/scene_text_editing.rs` (`SceneTextEditingScene`, `gpui_ui_1to1`) |
| 29 | `Telegram/SourceFiles/editor/controllers/controllers.h` | `L1-L43` | `Editor::Controllers` | Aggregator container bundling stickers panel controller, undo controller, and UI show layers | `crates/gpui_telegram_media/src/editor/controllers/controllers.rs` (`ControllersControllers`, `gpui_ui_1to1`) |
| 30 | `Telegram/SourceFiles/editor/controllers/stickers_panel_controller.cpp` | `L1-L113` | `StickersPanelController`, `ShowRequest`, `setShowRequestChanges`, `setMoveRequestChanges` | Stickers/GIF/media drawer integration: positioning below toolbar button, animation requests (`ToggleAnimated`, `ShowAnimated`, `HideAnimated`, `HideFast`), document selection dispatch | `crates/gpui_telegram_media/src/editor/controllers/stickers_panel_controller.rs` (`StickersPanelControllerControllers`, `gpui_ui_1to1`) |
| 31 | `Telegram/SourceFiles/editor/controllers/stickers_panel_controller.h` | `L1-L53` | `StickersPanelController`, `ShowRequest` | Header definition for stickers and emoji panel controller | `crates/gpui_telegram_media/src/editor/controllers/stickers_panel_controller.rs` (`StickersPanelControllerControllers`, `gpui_ui_1to1`) |
| 32 | `Telegram/SourceFiles/editor/controllers/undo_controller.cpp` | `L1-L39` | `UndoController`, `Undo`, `EnableRequest` | Undo/redo request stream controller: binds can-perform state and triggers undo/redo command streams | `crates/gpui_telegram_media/src/editor/controllers/undo_controller.rs` (`UndoControllerControllers`, `gpui_ui_1to1`) |
| 33 | `Telegram/SourceFiles/editor/controllers/undo_controller.h` | `L1-L41` | `UndoController`, `Undo`, `EnableRequest` | Interface for tracking and executing editor undo/redo operations | `crates/gpui_telegram_media/src/editor/controllers/undo_controller.rs` (`UndoControllerControllers`, `gpui_ui_1to1`) |
| 34 | `Telegram/SourceFiles/editor/video/video_clip.cpp` | `L1-L147` | `VideoClip`, `VideoClipSource`, `setTrim`, `loopDuration` | Single video clip media manager: segment player binding, trim window clamping `[from, till]`, audio track mute/volume control, thumbnail generation | `crates/gpui_telegram_media/src/editor/video/video_clip.rs` (`VideoClipVideo`, `gpui_ui_1to1`) |
| 35 | `Telegram/SourceFiles/editor/video/video_clip.h` | `L1-L71` | `VideoClip`, `VideoClipSource`, `State` | Video clip class declaration with playback state and frame extraction interface | `crates/gpui_telegram_media/src/editor/video/video_clip.rs` (`VideoClipVideo`, `gpui_ui_1to1`) |
| 36 | `Telegram/SourceFiles/editor/video/video_editor.cpp` | `L1-L999` | `VideoEditor`, `ControlsBar`, `BarTextButton`, `kBubbleMinVisible = 600ms`, `kBubbleDuration = 500ms`, `kCaptureDuration = 420ms`, `kMaxGifSize = 10MB` | Full-screen video editing layer widget: video streaming pipeline, trim timeline, quality slider, GIF conversion mode, animated play badge, size estimation | `crates/gpui_telegram_media/src/editor/video/video_editor.rs` (`VideoEditorVideo`, `gpui_ui_1to1`) |
| 37 | `Telegram/SourceFiles/editor/video/video_editor.h` | `L1-L145` | `VideoEditor`, `VideoEditorDescriptor` | Video editor widget class interface with done/cancel event producers | `crates/gpui_telegram_media/src/editor/video/video_editor.rs` (`VideoEditorVideo`, `gpui_ui_1to1`) |
| 38 | `Telegram/SourceFiles/editor/video/video_editor_common.cpp` | `L1-L116` | `EditedFrameSize`, `VideoEdited`, `ComposeVideoSource`, `ExtractCoverImage` | Video modification validation: crop rectangle intersection, rotation transposing, audio track difference detection, transcode flag determination | `crates/gpui_telegram_media/src/editor/video/video_editor_common.rs` (`VideoEditorCommonVideo`, `gpui_ui_1to1`) |
| 39 | `Telegram/SourceFiles/editor/video/video_editor_common.h` | `L1-L65` | `VideoModifications`, `VideoEditorData` | Modification descriptors: geometry, trim bounds (`from`, `till`, `cover`), quality step, GIF flag, FPS limits | `crates/gpui_telegram_media/src/editor/video/video_editor_common.rs` (`VideoEditorCommonVideo`, `gpui_ui_1to1`) |
| 40 | `Telegram/SourceFiles/editor/video/video_editor_layer.cpp` | `L1-L318` | `ProfileVideoEditorData`, `ShowVideoEditorLayer`, `PrepareProfileVideo`, `kMaxSideRatio = 10`, `AcceptableDimensions` | Layer integration for avatar/profile video editing: 800x800 square aspect ratio constraint, 9.6s max duration, 1s min duration, 30fps transcode | `crates/gpui_telegram_media/src/editor/video/video_editor_layer.rs` (`VideoEditorLayerVideo`, `gpui_ui_1to1`) |
| 41 | `Telegram/SourceFiles/editor/video/video_editor_layer.h` | `L1-L68` | `kProfileVideoSide = 800`, `kProfileVideoMaxDuration = 9600ms`, `kProfileVideoMinDuration = 1000ms`, `kProfileVideoFps = 30.`, `ProfileMedia` | Profile video editor dimension and duration constants and layer factory functions | `crates/gpui_telegram_media/src/editor/video/video_editor_layer.rs` (`VideoEditorLayerVideo`, `gpui_ui_1to1`) |
| 42 | `Telegram/SourceFiles/editor/video/video_editor_quality.cpp` | `L1-L46` | `kLevels = { 360, 480, 720, 1080 }`, `VideoQualityLevels`, `VideoQualityLabel` | Quality ladder generation: downscaled resolutions preserving aspect ratio for shorter side < original shorter side, plus original resolution notch | `crates/gpui_telegram_media/src/editor/video/video_editor_quality.rs` (`VideoEditorQualityVideo`, `gpui_ui_1to1`) |
| 43 | `Telegram/SourceFiles/editor/video/video_editor_quality.h` | `L1-L26` | `VideoQualityLevel`, `VideoQualityLevels`, `VideoQualityLabel` | Quality level structure `{ shorterSide, resolution }` and label formatting prototypes | `crates/gpui_telegram_media/src/editor/video/video_editor_quality.rs` (`VideoEditorQualityVideo`, `gpui_ui_1to1`) |
| 44 | `Telegram/SourceFiles/editor/video/video_quality_slider.cpp` | `L1-L225` | `VideoQualitySlider`, `kShiftDuration = 120ms`, `LabelsMinSkip`, `dotX`, `indexAt` | Discrete step quality slider: animated notch selector, label rendering, snap-to-nearest notch during touch/mouse dragging | `crates/gpui_telegram_media/src/editor/video/video_quality_slider.rs` (`VideoQualitySliderVideo`, `gpui_ui_1to1`) |
| 45 | `Telegram/SourceFiles/editor/video/video_quality_slider.h` | `L1-L57` | `VideoQualitySlider` | Discrete step video quality slider widget declaration | `crates/gpui_telegram_media/src/editor/video/video_quality_slider.rs` (`VideoQualitySliderVideo`, `gpui_ui_1to1`) |
| 46 | `Telegram/SourceFiles/editor/video/video_segment_player.cpp` | `L1-L259` | `SegmentPlayer`, `FrameRequestFor`, `start`, `setSegment`, `restart`, `frame` | Video segment looping playback engine: streaming player integration, loop boundaries `[from, till]`, seeking throttle, pause/resume | `crates/gpui_telegram_media/src/editor/video/video_segment_player.rs` (`VideoSegmentPlayerVideo`, `gpui_ui_1to1`) |
| 47 | `Telegram/SourceFiles/editor/video/video_segment_player.h` | `L1-L98` | `SegmentPlayer`, `SegmentPlayerOptions` | Looping segment player interface with frame delivery and position updates | `crates/gpui_telegram_media/src/editor/video/video_segment_player.rs` (`VideoSegmentPlayerVideo`, `gpui_ui_1to1`) |
| 48 | `Telegram/SourceFiles/editor/video/video_timeline.cpp` | `L1-L287` | `VideoTimeline`, `VideoTimelineFramesCache`, `kMaxFrames = 24`, `kMaxCachedFrameSets = 8`, `kDotDuration = 500ms`, `kReloadDelay = 150ms`, `kFrameWidthTolerance = 0.25` | Video thumbnail strip timeline: async frame generator, 24 thumbnail slots, cache eviction (>8 sets), cover dot scrubber, trim handle synchronization | `crates/gpui_telegram_media/src/editor/video/video_timeline.rs` (`VideoTimelineVideo`, `gpui_ui_1to1`) |
| 49 | `Telegram/SourceFiles/editor/video/video_timeline.h` | `L1-L92` | `VideoTimeline`, `VideoTimelineFrames`, `VideoTimelineFramesCache`, `VideoTimelineDescriptor` | Timeline thumbnail frame cache and interactive trimmer interface | `crates/gpui_telegram_media/src/editor/video/video_timeline.rs` (`VideoTimelineVideo`, `gpui_ui_1to1`) |
| 50 | `Telegram/SourceFiles/editor/video/video_timeline_seeker.cpp` | `L1-L86` | `TimelineSeeker`, `kSeekThrottle = 120ms`, `seek`, `finishDragging` | Timeline scrubbing throttler: 120ms seek timer, scrubbing vs playback state coordination, boundary looping synchronization | `crates/gpui_telegram_media/src/editor/video/video_timeline_seeker.rs` (`VideoTimelineSeekerVideo`, `gpui_ui_1to1`) |
| 51 | `Telegram/SourceFiles/editor/video/video_timeline_seeker.h` | `L1-L39` | `TimelineSeeker` | Timeline scrubbing coordinator linking trim timeline with video segment player | `crates/gpui_telegram_media/src/editor/video/video_timeline_seeker.rs` (`VideoTimelineSeekerVideo`, `gpui_ui_1to1`) |

---

## 2. Subsystem Architectural Details & Algorithms

### 2.1 Affine Transformation Math & Snapping (`scene_item_base.cpp:L27-L61`, `L203-L230`)

In Telegram Desktop's editor scene, every interactive element (`ItemBase`) supports full 2D affine transformation (translation, scaling, rotation, and horizontal flipping) with rotational angle snapping and sticky alignment guidelines.

1. **Rotation Angle Calculation & Snap**:
   - Given the mouse position in item local coordinates `(mx, my)` mapped to scene coordinates `pos`, and the center of the item `origin`:
     $$\Delta x = \text{pos}.x - \text{origin}.x, \quad \Delta y = \text{pos}.y - \text{origin}.y$$
     $$\theta = \text{Normalized}\left((\text{isLeft} ? 180^\circ : 0^\circ) + \text{atan2}(\Delta y, \Delta x) \cdot \frac{180^\circ}{\pi}\right)$$
   - Angle normalization wraps any value outside $[-360^\circ, 360^\circ]$:
     $$\text{Normalized}(\theta) = \theta + (|\theta| < 360^\circ ? 0 : -360^\circ \cdot \text{sgn}(\theta))$$
   - When the Shift key is held, rotation snaps to multiples of `kSnapAngle = 45.0` degrees:
     $$\theta_{\text{snapped}} = \text{round}\left(\frac{\theta}{45.0^\circ}\right) \cdot 45.0^\circ$$

2. **Handle Sizing & Interactive Dragging**:
   - Handle width scales inversely with view zoom: `scaledHandleSize = photoEditorItemHandleSize / zoom`.
   - Right handle bounds: `QRectF(horizontalSize/2 - handleSize/2, -handleSize/2, handleSize, handleSize)`.
   - Left handle bounds: `QRectF(-horizontalSize/2 - handleSize/2, -handleSize/2, handleSize, handleSize)`.
   - Resizing adjusts `horizontalSize = clamp(max(2 * px, 2 * py), minSize, maxSize)` and recalculates `verticalSize = horizontalSize * aspectRatio`.

3. **Sticky Alignment Guidelines (`kStickyLines = 5`, `kStickyDuration = 150ms`)**:
   - Upstream evaluates 5 equidistant guidelines across the canvas:
     $$\text{StickyLine}(i) = \text{Start} + \text{Length} \cdot \frac{i}{kStickyLines - 1}, \quad i \in \{0, 1, 2, 3, 4\}$$
   - Proximity detection triggers within `scaledStickyTrigger = 6.0 / zoom`.
   - Animated snapping smoothly translates item coordinates with `easeOutCubic` over 150ms.

### 2.2 Vector Canvas Stroke Smoothing & Eraser Algorithm (`scene_item_canvas.cpp:L19-L36`)

Freehand drawing uses a high-fidelity quadratic Bezier curve interpolation algorithm with dynamic distance thresholding:
- Minimum point distance: `kMinPointDistanceBase = 2.0`
- Maximum point distance: `kMaxPointDistance = 15.0`
- Smoothing strength: `kSmoothingStrength = 0.5`
- Half strength: `kHalfStrength = 0.25`, Inverse: `kInvStrength = 0.5`
- Interpolated point formula:
  $$P_{\text{smooth}} = P_{\text{prev}} \cdot kHalfStrength + P_{\text{curr}} \cdot kInvStrength + P_{\text{next}} \cdot kHalfStrength$$
- Eraser tool applies mask subtraction to previously painted stroke pixmaps without modifying static background image data.

### 2.3 Geometric Shapes Math & Vector Curves (`scene_item_shape.cpp:L21-L32`)

Vector shapes implement parametric formulas:
1. **Star**: 5-point star with golden ratio:
   - Outer-to-inner radius ratio: `kStarInnerRatio = 0.4316`
   - Bounding box aspect ratio: `kStarBoxHeight / kStarBoxWidth = 1.809017 / 1.902113 \approx 0.9510565`
2. **Speech Bubble**:
   - Body to height ratio: `kBubbleBodyRatio = 2.0 / 2.25 \approx 0.8889`
   - Tail offset: `kBubbleTailRatio = 0.75`
3. **Curved Arrow**:
   - Arrowhead length ratio: `kArrowHeadRatio = 0.16`
   - Arrowhead angle: `kArrowHeadAngle = 30^\circ` ($30 \cdot \pi / 180 \approx 0.5236$ rad)
   - Maximum curvature bend ratio: `kArrowMaxBendRatio = 0.35`
   - Arrowhead stroke factor: `kArrowHeadStrokeFactor = 2.5`

### 2.4 Text Rendering, Bubble Merging & Auto-Shrink (`scene_item_text.cpp:L34-L43`, `scene_text_editing.cpp:L27-L38`)

Text items support 4 Telegram font typefaces (Default, Outlined, Framed, SemiTransparent):
- Multi-line bubble rendering:
  - `kPaddingFactor = 0.4`, `kMaxWidthFactor = 0.8`, `kMinContentWidth = 20`
  - `kBrightnessFramedThreshold = 0.721`, `kBrightnessSemiTransparentThreshold = 0.25`
  - `kSemiTransparentAlpha = 0x99` (60% opacity)
  - Corner rounding factor: `kCornerRadiusFactor = 1.0 / 3.0`
  - Adjacent line merge radius: `kMergeRadiusFactor = 1.5`
- Auto-shrink controller:
  - If text document height exceeds available editor bounds, font size scales by `kAutoShrinkStep = 0.9` iteratively down to `kMinFontSizeFactor = 1.0 / 50.0`.

### 2.5 Undo/Redo State Machine & Transaction Log (`undo_controller.cpp:L15-L39`, `scene.cpp:L29-L76`)

The scene graph tracks reversible operations via `NumberedItem` and `ItemAction`:
- Operations increment monotonically increasing sequence numbers `_itemNumber`.
- Each action records `.save(SaveState::Keep)` and `.save(SaveState::Apply)` snapshots.
- Undo/redo streams broadcast `EnableRequest { command: Undo::Undo | Undo::Redo, enable: bool }`.
- Deleting, adding, modifying transforms, or erasing strokes can be unrolled and re-applied without data loss.

### 2.6 Video Clip Trim, Scrubbing & Timeline FSM (`video_clip.cpp:L54-L60`, `video_timeline.cpp:L18-L24`, `video_timeline_seeker.cpp:L16-L30`)

Video editing provides non-destructive trimming and playback scrubbing:
- Trim boundaries: `from = clamp(from, 0, duration)` and `till = (till > 0) ? clamp(till, from, duration) : 0`.
- Thumbnail strip: `kMaxFrames = 24`, cache retention `kMaxCachedFrameSets = 8`, reload debounce `kReloadDelay = 150ms`.
- Cover frame scrubber dot animation: `kDotDuration = 500ms`.
- Scrubbing throttler: `kSeekThrottle = 120ms` decouples high-frequency mouse moves from video decoder restarts.

### 2.7 Video Quality Ladder & Profile Video Constraints (`video_editor_quality.cpp:L16-L38`, `video_editor_layer.h:L28-L32`)

- Quality resolution rungs: `kLevels = [360, 480, 720, 1080]`.
- Output ladder generates rungs where `shorterSide < min(width, height)`, followed by original `shorterSide = 0`.
- Profile video constraints:
  - `kProfileVideoSide = 800` (1:1 square)
  - `kProfileVideoMinDuration = 1000ms` (1.0s)
  - `kProfileVideoMaxDuration = 9600ms` (9.6s)
  - `kProfileVideoFps = 30.0`
  - Audio is muted (`removeAudio = true`), forced transcode (`transcodeAlways = true`).

---

## 3. Dynamic Rust Architecture & Implementation Plan

The Rust implementation replaces all Pattern D boilerplate across `crates/gpui_telegram_media/src/editor/` with high-cohesion, authentic structures:

1. **`editor/scene/scene_item_base.rs`**: Core `ItemPlacement`, `AffineTransform2D`, handle hit-testing (`kSnapAngle = 45.0`, `kStickyLines = 5`), and sticky alignment math.
2. **`editor/scene/scene_item_shape.rs`**: Geometric shapes (`ShapeType`: Rectangle, Circle, Star, SpeechBubble, Arrow) with exact radius ratios and bend control points.
3. **`editor/scene/scene_item_canvas.rs`**: Stroke smoothing engine with distance thresholds and eraser mask blending.
4. **`editor/scene/scene_item_text.rs` & `scene_text_editing.rs`**: Text styling, multi-line bubble geometry, and auto-shrink engine.
5. **`editor/scene/scene.rs`**: Scene graph manager with item z-ordering, selection, and undo stack.
6. **`editor/scene/scene_emoji_document.rs`**, `scene_item_animated.rs`, `scene_item_image.rs`, `scene_item_line.rs`, `scene_item_link.rs`, `scene_item_message.rs`, `scene_item_sticker.rs`, `scene_item_video.rs`: Dedicated authentic elements.
7. **`editor/controllers/`**: `UndoController`, `StickersPanelController`, and `Controllers` aggregator.
8. **`editor/video/`**: `VideoClip`, `VideoTimeline`, `TimelineSeeker`, `VideoQualitySlider`, `VideoEditorQuality`, `VideoSegmentPlayer`, `VideoEditorCommon`, `VideoEditorLayer`, and `VideoEditor`.

---

## 4. Verification & Test Plan

A comprehensive subsystem test suite is authored at `crates/gpui_telegram_media/tests/editor_scene_video_subsystem_tests.rs` containing >= 10 tests verifying:
- Affine transformation angle calculation and 45-degree snap quantization.
- 5-line sticky guideline coordinate generation and snap trigger logic.
- Star, speech bubble, and arrow parametric geometry ratios.
- Vector stroke smoothing algorithm and distance thresholds.
- Multi-line text bubble margin and auto-shrink scale calculation.
- Undo/redo controller event streaming and transaction reversal.
- Video trim clamping and duration normalization.
- Video quality ladder downscaling and original fallback.
- Profile video dimension and duration constraint enforcement.
- Video timeline scrubbing throttler state machine.
