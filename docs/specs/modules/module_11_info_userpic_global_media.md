# Module 11: Info & Settings — Userpic & Global Media Specification (Gap Closure)

## Executive Summary
This design document establishes the authoritative C++ to Rust/GPUI architectural mapping for **Module 11 (`info & settings`) Gap Closure**, encompassing exactly 24 upstream Telegram Desktop source files (and 12 distinct target Rust files) across two major subsystems:
1. **Global Media Viewer & Provider (`info/global_media/`)**:
   - Cross-dialog shared media browser, pagination controller, global media search provider, and scroll/selection management.
   - Files: `info_global_media_inner_widget.{h,cpp}`, `info_global_media_provider.{h,cpp}`, `info_global_media_widget.{h,cpp}`.
2. **Userpic Emoji Builder (`info/userpic/`)**:
   - Dynamic avatar and forum icon generator combining custom emojis, animated stickers, and customizable gradient palettes.
   - Files: `info_userpic_bubble_wrap.{h,cpp}`, `info_userpic_color_circle_button.{h,cpp}`, `info_userpic_colors_editor.{h,cpp}`, `info_userpic_emoji_builder.{h,cpp}`, `info_userpic_emoji_builder_common.{h,cpp}`, `info_userpic_emoji_builder_layer.{h,cpp}`, `info_userpic_emoji_builder_menu_item.{h,cpp}`, `info_userpic_emoji_builder_preview.{h,cpp}`, `info_userpic_emoji_builder_widget.{h,cpp}`.

---

## 100% Upstream Source Reading Manifest (24 / 24 Files)

| Source Path | Line Range | Core Types & Symbols Extracted | Constants & Formulas | Target Rust / GPUI Path | Target Symbol |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `Telegram/SourceFiles/info/global_media/info_global_media_inner_widget.h` | L1-L86 | `InnerWidget`, `Memento`, `EmptyWidget`, `ScrollToRequest`, `SelectedItems`, `SelectionAction` | Inner layout geometry, list vs empty visibility condition (`listHeight > 0`) | `crates/gpui_telegram_desktop/src/views/drawer/global_media/info_global_media_inner_widget.rs` | `InfoGlobalMediaInnerWidgetGlobalMedia` |
| `Telegram/SourceFiles/info/global_media/info_global_media_inner_widget.cpp` | L1-L150 | `InnerWidget::setupList`, `InnerWidget::recountHeight`, `InnerWidget::showInternal`, `visibleTopBottomUpdated` | Scroll offset mapping (`widget->y() + to`), height recalculation with dynamic margin | `crates/gpui_telegram_desktop/src/views/drawer/global_media/info_global_media_inner_widget.rs` | `InfoGlobalMediaInnerWidgetGlobalMedia` |
| `Telegram/SourceFiles/info/global_media/info_global_media_provider.h` | L1-L258 | `GlobalMediaKey`, `GlobalMediaSliceSnapshot`, `GlobalMediaSlice`, `Provider`, `FillResult`, `SliceUpdate`, `RequestCursor` | `kMinimalIdsLimit = 16`, preloading threshold, floating header predicate | `crates/gpui_telegram_desktop/src/views/drawer/global_media/info_global_media_provider.rs` | `InfoGlobalMediaProviderGlobalMedia` |
| `Telegram/SourceFiles/info/global_media/info_global_media_provider.cpp` | L1-L811 | `GlobalMediaSlice::nearest`, `Provider::checkPreload`, `Provider::fillRequest`, `Provider::requestMore` | `kPreloadedScreensCount = 4`, `kPreloadedScreensCountFull = 9`, `preloadIdsLimitMin = (preloadedCount / 2) + 1` | `crates/gpui_telegram_desktop/src/views/drawer/global_media/info_global_media_provider.rs` | `InfoGlobalMediaProviderGlobalMedia` |
| `Telegram/SourceFiles/info/global_media/info_global_media_widget.h` | L1-L78 | `Memento`, `Widget`, `ContentMemento`, `ContentWidget`, `Make` | State memento serialization, top bar menu action dispatch | `crates/gpui_telegram_desktop/src/views/drawer/global_media/info_global_media_widget.rs` | `InfoGlobalMediaWidgetGlobalMedia` |
| `Telegram/SourceFiles/info/global_media/info_global_media_widget.cpp` | L1-L143 | `Widget::setInternalState`, `Widget::fillTopBarMenu`, `Widget::title`, `Make` | Top bar title `lng_profile_shared_media`, confirmation box for delete all files | `crates/gpui_telegram_desktop/src/views/drawer/global_media/info_global_media_widget.rs` | `InfoGlobalMediaWidgetGlobalMedia` |
| `Telegram/SourceFiles/info/userpic/info_userpic_bubble_wrap.h` | L1-L23 | `BubbleWrapInnerRect`, `AddBubbleWrap`, `VerticalLayout` | Bubble padding reduction formula: `r - st::userpicBuilderEmojiBubblePadding` | `crates/gpui_telegram_desktop/src/views/drawer/userpic/info_userpic_bubble_wrap.rs` | `InfoUserpicBubbleWrapUserpic` |
| `Telegram/SourceFiles/info/userpic/info_userpic_bubble_wrap.cpp` | L1-L102 | `PaintExcludeTopShadow`, `BubbleWrapInnerRect`, `AddBubbleWrap` | Shadow offsets `hOffset = 1.0`, `vOffset = 2.0`, opacity steps `0.2` and `0.4`, `st::bubbleRadiusSmall` | `crates/gpui_telegram_desktop/src/views/drawer/userpic/info_userpic_bubble_wrap.rs` | `InfoUserpicBubbleWrapUserpic` |
| `Telegram/SourceFiles/info/userpic/info_userpic_color_circle_button.h` | L1-L33 | `CircleButton`, `AbstractButton`, `setIndex`, `setSelectedProgress`, `setBrush` | Selected progress animation `0.0..=1.0`, circle index | `crates/gpui_telegram_desktop/src/views/drawer/userpic/info_userpic_color_circle_button.rs` | `InfoUserpicColorCircleButtonUserpic` |
| `Telegram/SourceFiles/info/userpic/info_userpic_color_circle_button.cpp` | L1-L43 | `CircleButton::paintEvent`, `CircleButton::setSelectedProgress`, `PaintRoundColorButton` | Circular button rendering with inner highlight ring and animated selection | `crates/gpui_telegram_desktop/src/views/drawer/userpic/info_userpic_color_circle_button.rs` | `InfoUserpicColorCircleButtonUserpic` |
| `Telegram/SourceFiles/info/userpic/info_userpic_colors_editor.h` | L1-L31 | `CreateGradientEditor`, `BothWayCommunication`, `DocumentData` | Multi-stop gradient color editor contract | `crates/gpui_telegram_desktop/src/views/drawer/userpic/info_userpic_colors_editor.rs` | `InfoUserpicColorsEditorUserpic` |
| `Telegram/SourceFiles/info/userpic/info_userpic_colors_editor.cpp` | L1-L317 | `RandomColor`, `ColorsLine`, `ColorsLine::calculatePositionFor`, `CreateGradientEditor` | `kMaxColors = 4`, HSL randomizer: `(rand0 % 100) / 100.`, `(rand1 % 50) / 100. + 0.5`, `c.lightnessF()` | `crates/gpui_telegram_desktop/src/views/drawer/userpic/info_userpic_colors_editor.rs` | `InfoUserpicColorsEditorUserpic` |
| `Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder.h` | L1-L25 | `ShowLayer`, `StartData`, `Result` | Userpic emoji builder modal launcher entry point | `crates/gpui_telegram_desktop/src/views/drawer/userpic/info_userpic_emoji_builder.rs` | `InfoUserpicEmojiBuilderUserpic` |
| `Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder.cpp` | L1-L78 | `ShowLayer`, `CreateUserpicBuilder`, `RoundButton`, `IconButton` | Back position `(8, 8)`, Save position `(7, 12)`, `LayerOption::KeepOther` | `crates/gpui_telegram_desktop/src/views/drawer/userpic/info_userpic_emoji_builder.rs` | `InfoUserpicEmojiBuilderUserpic` |
| `Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder_common.h` | L1-L39 | `Result`, `GenerateGradient`, `StartData`, `BothWayCommunication` | `kRotation = 45 deg`, forum userpic radius multiplier | `crates/gpui_telegram_desktop/src/views/drawer/userpic/info_userpic_emoji_builder_common.rs` | `InfoUserpicEmojiBuilderCommonUserpic` |
| `Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder_common.cpp` | L1-L54 | `GenerateGradient`, `Circle`, `Round`, `CornersMask`, `ForumUserpicRadiusMultiplier` | Corner radius `min(w, h) * ForumMultiplier`, 45-degree linear diagonal gradient | `crates/gpui_telegram_desktop/src/views/drawer/userpic/info_userpic_emoji_builder_common.rs` | `InfoUserpicEmojiBuilderCommonUserpic` |
| `Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder_layer.h` | L1-L40 | `LayerWidget`, `CornersPixmaps`, `closeByOutsideClick`, `parentResized` | Modal overlay sheet container for emoji avatar builder | `crates/gpui_telegram_desktop/src/views/drawer/userpic/info_userpic_emoji_builder_layer.rs` | `InfoUserpicEmojiBuilderLayerUserpic` |
| `Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder_layer.cpp` | L1-L57 | `LayerWidget::parentResized`, `LayerWidget::resizeGetHeight`, `LayerWidget::paintEvent` | Centering formula `(parent - child) / 2`, `closeByOutsideClick = false`, `st::userpicBuilderEmojiLayerMinHeight = 496px` | `crates/gpui_telegram_desktop/src/views/drawer/userpic/info_userpic_emoji_builder_layer.rs` | `InfoUserpicEmojiBuilderLayerUserpic` |
| `Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder_menu_item.h` | L1-L30 | `AddEmojiBuilderAction`, `PopupMenu`, `Result` | Context menu entry for "Set Emoji / Sticker Avatar" | `crates/gpui_telegram_desktop/src/views/drawer/userpic/info_userpic_emoji_builder_menu_item.rs` | `InfoUserpicEmojiBuilderMenuItemUserpic` |
| `Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder_menu_item.cpp` | L1-L217 | `StickerProvider`, `AddEmojiBuilderAction`, `processDocumentIndex`, `lookupSticker` | Cycling timeout `kTimeout = 1500ms`, RNG shuffling `std::mt19937`, icon centering `(remove - icon) / 2` | `crates/gpui_telegram_desktop/src/views/drawer/userpic/info_userpic_emoji_builder_menu_item.rs` | `InfoUserpicEmojiBuilderMenuItemUserpic` |
| `Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder_preview.h` | L1-L91 | `PreviewPainter`, `EmojiUserpic`, `StickerPlayer`, `Result` | Interactive preview widget and bitmap generator | `crates/gpui_telegram_desktop/src/views/drawer/userpic/info_userpic_emoji_builder_preview.rs` | `InfoUserpicEmojiBuilderPreviewUserpic` |
| `Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder_preview.cpp` | L1-L251 | `PreviewPainter::PreviewPainter`, `EmojiUserpic::setGradientColors`, `EmojiUserpic::result` | Emoji size formula `SafeRound(size / sqrt(2))`, frame radius `25%`, `kAttemptsToDrawFirstFrame = 3000` | `crates/gpui_telegram_desktop/src/views/drawer/userpic/info_userpic_emoji_builder_preview.rs` | `InfoUserpicEmojiBuilderPreviewUserpic` |
| `Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder_widget.h` | L1-L41 | `CreateUserpicBuilder`, `CreateEmojiUserpic`, `StartData`, `BothWayCommunication` | Userpic builder main container assembly & emoji userpic factory | `crates/gpui_telegram_desktop/src/views/drawer/userpic/info_userpic_emoji_builder_widget.rs` | `InfoUserpicEmojiBuilderWidgetUserpic` |
| `Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder_widget.cpp` | L1-L578 | `AlignChildren`, `GenerateSpecial`, `PaletteGradients`, `EmojiSelector`, `CreateUserpicBuilder` | Palette count `kColorsCount = 7`, special button (gradient editor), children spacing `(full - widgets) / (count - 1)` | `crates/gpui_telegram_desktop/src/views/drawer/userpic/info_userpic_emoji_builder_widget.rs` | `InfoUserpicEmojiBuilderWidgetUserpic` |

---

## Detailed Architectural Analysis by Cluster

### Cluster 1: Global Media Viewer & Provider Subsystem
The Global Media subsystem enables browsing and searching media files across conversations or in global searches:
1. **`Info::GlobalMedia::InnerWidget`**:
   - Manages the internal layout of the global media tab.
   - Embeds `Media::ListWidget` for the media grid/list items and `Media::EmptyWidget` for empty or zero-result states.
   - Coordinates dynamic height calculation: when `_list->heightNoMargins() > 0`, the empty illustration is hidden; otherwise, the empty widget is displayed.
   - Proxies scroll requests from child item positions (`widget->y() + to`) to the outer scroll area.
   - Streams multi-selection updates (`SelectedItems`) and routes batch selection actions (`SelectionAction`).
2. **`Info::GlobalMedia::Provider`**:
   - Subclasses `Media::ListProvider` and implements `Media::ListSectionDelegate`.
   - Maintains `GlobalMediaSlice` indexed by message position (`aroundId`) and handles windowed pagination.
   - Computes preloading thresholds based on viewport height and item min height:
     `kPreloadedScreensCount = 4`, `kPreloadedScreensCountFull = 9`, `preloadIdsLimitMin = (preloadedCount / 2) + 1`.
   - Determines section headers: floating headers are enabled for `Type::File` and `Type::Link`, while disabled for photos, videos, GIFs, audio, and voice notes.
   - Integrates with API requests via `requestGlobalMedia` using pagination cursors (`RequestCursor`).
3. **`Info::GlobalMedia::Widget` & `Memento`**:
   - Extends `ContentWidget` and `ContentMemento` for the info panel drawer.
   - Serializes scroll position and media state across navigation pushes/pops.
   - Populates top bar context menu with "Delete All Downloaded Files" (`lng_context_delete_all_files`) with confirmation dialog.
   - Binds section title to `tr::lng_profile_shared_media()`.

### Cluster 2: Userpic Emoji Builder Subsystem
The Userpic Emoji Builder empowers users to create custom profile pictures or forum group icons using emojis and gradient backgrounds:
1. **`UserpicBuilder::BubbleWrap`**:
   - Wraps sections inside a styled bubble container with rounded corners and subtle drop shadows.
   - Subtracts padding margin: `BubbleWrapInnerRect(r) = r - st::userpicBuilderEmojiBubblePadding`.
   - Renders 3-step layered shadow excluding top shadow with opacities `0.2` and `0.4` and vertical offset `vOffset = 2.0`.
2. **`UserpicBuilder::CircleButton`**:
   - Palette color button rendering a circular preview with animated selection indicator.
   - Interpolates selected progress `_selectedProgress: 0.0..=1.0` to render concentric highlight rings.
3. **`UserpicBuilder::ColorsEditor` (`CreateGradientEditor`)**:
   - Interactive gradient color editor supporting up to `kMaxColors = 4` gradient stops.
   - Includes Minus button (disabled when `<= 1` stop) and Plus button (disabled when `>= 4` stops).
   - Generates random colors using HSL formula: Hue in `0..100%`, Saturation in `50..100%`, and preserving lightness.
   - Positions buttons using `(fullWidth - colorsWidth) / 2` with smooth animated transitions.
4. **`UserpicBuilder::PreviewPainter` & `EmojiUserpic`**:
   - Real-time compositing widget and export engine.
   - Calculates emoji bounding square using geometric formula: `emojiSize = SafeRound(size / sqrt(2))`!
   - Applies frame corner mask with `25%` relative radius.
   - Generates diagonal 45-degree linear gradients across color stops.
   - Supports circular masks for standard profile pictures and rounded rectangle masks with `ForumUserpicRadiusMultiplier` for forum topics.
5. **`UserpicBuilder::MenuItem` & `StickerProvider`**:
   - Context menu item that dynamically cycles through available stickers every `kTimeout = 1500ms`.
   - Shuffles document IDs using `std::mt19937` and displays thumbnail centered in menu action icon slot.
6. **`UserpicBuilder::LayerWidget` & `EmojiBuilderWidget`**:
   - Modal dialog layer centered in parent viewport: `(parent - child) / 2`.
   - Tabbed emoji and sticker selector with scrollable grid.
   - Built-in 7-color preset palette (`PaletteGradients`) with 180-degree diagonal rotation plus 8th "special" custom gradient button.
   - Emits finished avatar bitmap (`Editor::kProfilePhotoSize`) on Save click.

---

## GPUI Architectural Mapping & Rust Models

The 12 target files cleanly implement these behaviors using GPUI reactive state models:
1. `InfoGlobalMediaInnerWidgetGlobalMedia`: View model tracking media items, empty state queries, scroll requests, and selection.
2. `InfoGlobalMediaProviderGlobalMedia`: Provider state machine managing slice windows, query pagination, preloading offsets, and header modes.
3. `InfoGlobalMediaWidgetGlobalMedia`: Content widget holding inner widget, top-bar menu actions, and memento state.
4. `InfoUserpicBubbleWrapUserpic`: Bubble container with custom shadow rendering and inner content layout.
5. `InfoUserpicColorCircleButtonUserpic`: Interactive color circle button with selection progress animations.
6. `InfoUserpicColorsEditorUserpic`: Multi-stop gradient color editor with add/remove buttons and HSL randomizer.
7. `InfoUserpicEmojiBuilderUserpic`: Modal launcher coordinating builder lifecycle and callback resolution.
8. `InfoUserpicEmojiBuilderCommonUserpic`: Mathematical models for gradient generation (45-degree angle), forum corner masks, and start data.
9. `InfoUserpicEmojiBuilderLayerUserpic`: Overlay layer handling center alignment, geometry constraints, and corner rendering.
10. `InfoUserpicEmojiBuilderMenuItemUserpic`: Dynamic menu action cycling stickers with 1500ms timer.
11. `InfoUserpicEmojiBuilderPreviewUserpic`: Preview canvas implementing `emoji_size = size / sqrt(2)` sizing, cross-fades, and final bitmap generation.
12. `InfoUserpicEmojiBuilderWidgetUserpic`: Master builder view combining preview, preset palette (7 presets + custom), and emoji/sticker selector.

---

## Verification Test Plan & Matrix

Each target file contains comprehensive unit tests verifying:
- **Global Media**: Slice indexing, distance calculation, nearest position lookup, preloading threshold logic, floating header decisions, and empty vs list toggling.
- **Userpic Builder**: Bubble rect math, color circle selection progress, gradient color stops (`1..=4`), HSL color generation, 45-degree gradient specs, sqrt(2) scaling, centering geometry, and preset palettes.
