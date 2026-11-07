# Subsystem Shard: `layout`

**Total Source Files**: 12  
**Total Source Lines**: 1,009  
**Primary Feature IDs**: TCG-N04  
**JSON Shard Path**: `docs/specs/ledger/shards/layout.json`  

## Component Inventory & Parity Mapping

| Source Path | Feature ID | Target Symbol | Target Rust / GPUI Path | Lines | Test ID |
|---|---|---|---|---|---|
| `Telegram/SourceFiles/layout/abstract_layout_item.cpp` | `TCG-N04` | `AbstractLayoutItem` | `crates/gpui_telegram_desktop/src/views/stage/layout/abstract_layout_item.rs` | 57 | `tests/test_tcg_n04_abstract_layout_item.rs` |
| `Telegram/SourceFiles/layout/abstract_layout_item.h` | `TCG-N04` | `AbstractLayoutItem` | `crates/gpui_telegram_desktop/src/views/stage/layout/abstract_layout_item.rs` | 61 | `tests/test_tcg_n04_abstract_layout_item.rs` |
| `Telegram/SourceFiles/layout/layout_document_generic_preview.cpp` | `TCG-N04` | `LayoutDocumentGenericPreview` | `crates/gpui_telegram_desktop/src/views/stage/layout/layout_document_generic_preview.rs` | 144 | `tests/test_tcg_n04_layout_document_generic_preview.rs` |
| `Telegram/SourceFiles/layout/layout_document_generic_preview.h` | `TCG-N04` | `LayoutDocumentGenericPreview` | `crates/gpui_telegram_desktop/src/views/stage/layout/layout_document_generic_preview.rs` | 26 | `tests/test_tcg_n04_layout_document_generic_preview.rs` |
| `Telegram/SourceFiles/layout/layout_item_base.cpp` | `TCG-N04` | `LayoutItemBase` | `crates/gpui_telegram_desktop/src/views/stage/layout/layout_item_base.rs` | 22 | `tests/test_tcg_n04_layout_item_base.rs` |
| `Telegram/SourceFiles/layout/layout_item_base.h` | `TCG-N04` | `LayoutItemBase` | `crates/gpui_telegram_desktop/src/views/stage/layout/layout_item_base.rs` | 33 | `tests/test_tcg_n04_layout_item_base.rs` |
| `Telegram/SourceFiles/layout/layout_mosaic.cpp` | `TCG-N04` | `LayoutMosaic` | `crates/gpui_telegram_desktop/src/views/stage/layout/layout_mosaic.rs` | 379 | `tests/test_tcg_n04_layout_mosaic.rs` |
| `Telegram/SourceFiles/layout/layout_mosaic.h` | `TCG-N04` | `LayoutMosaic` | `crates/gpui_telegram_desktop/src/views/stage/layout/layout_mosaic.rs` | 167 | `tests/test_tcg_n04_layout_mosaic.rs` |
| `Telegram/SourceFiles/layout/layout_position.cpp` | `TCG-N04` | `LayoutPosition` | `crates/gpui_telegram_desktop/src/views/stage/layout/layout_position.rs` | 31 | `tests/test_tcg_n04_layout_position.rs` |
| `Telegram/SourceFiles/layout/layout_position.h` | `TCG-N04` | `LayoutPosition` | `crates/gpui_telegram_desktop/src/views/stage/layout/layout_position.rs` | 21 | `tests/test_tcg_n04_layout_position.rs` |
| `Telegram/SourceFiles/layout/layout_selection.cpp` | `TCG-N04` | `LayoutSelection` | `crates/gpui_telegram_desktop/src/views/stage/layout/layout_selection.rs` | 43 | `tests/test_tcg_n04_layout_selection.rs` |
| `Telegram/SourceFiles/layout/layout_selection.h` | `TCG-N04` | `LayoutSelection` | `crates/gpui_telegram_desktop/src/views/stage/layout/layout_selection.rs` | 25 | `tests/test_tcg_n04_layout_selection.rs` |

## Deep Source Reading Evidence

### `Telegram/SourceFiles/layout/abstract_layout_item.h`
- **Classes & Structs**: PaintContextBase, AbstractLayoutItem
- **Core Methods & Signals**: selecting, AbstractLayoutItem, resizeGetHeight, setPosition, setShiftX, setShiftY
- **Target**: `AbstractLayoutItem` in `crates/gpui_telegram_desktop/src/views/stage/layout/abstract_layout_item.rs`

### `Telegram/SourceFiles/layout/layout_document_generic_preview.h`
- **Classes & Structs**: DocumentGenericPreview
- **Core Methods & Signals**: Create
- **Target**: `LayoutDocumentGenericPreview` in `crates/gpui_telegram_desktop/src/views/stage/layout/layout_document_generic_preview.rs`

### `Telegram/SourceFiles/layout/layout_item_base.h`
- **Classes & Structs**: TextState, StateRequest, LayoutItemBase
- **Core Methods & Signals**: initDimensions
- **Target**: `LayoutItemBase` in `crates/gpui_telegram_desktop/src/views/stage/layout/layout_item_base.rs`

### `Telegram/SourceFiles/layout/layout_mosaic.h`
- **Classes & Structs**: FoundItem, AbstractMosaicLayout, Row, MosaicLayout, Parent
- **Core Methods & Signals**: AbstractMosaicLayout::addItems, Parent::addItems, Parent::itemAt, Parent::maybeItemAt, Parent::forEach, Parent::paint
- **Target**: `LayoutMosaic` in `crates/gpui_telegram_desktop/src/views/stage/layout/layout_mosaic.rs`

### `Telegram/SourceFiles/layout/layout_position.h`
- **Classes & Structs**: Position
- **Target**: `LayoutPosition` in `crates/gpui_telegram_desktop/src/views/stage/layout/layout_position.rs`

### `Telegram/SourceFiles/layout/layout_selection.h`
- **Style Tokens**: asset:layout_selection.h
- **Target**: `LayoutSelection` in `crates/gpui_telegram_desktop/src/views/stage/layout/layout_selection.rs`

