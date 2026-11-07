//! Desktop Layout Subsystem re-exports and integration
//! Upstream: Telegram/SourceFiles/layout/

pub use crate::views::stage::layout::abstract_layout_item::{AbstractLayoutItem, PaintContextBase};
pub use crate::views::stage::layout::layout_document_generic_preview::{
    DocumentColorIndex, LayoutDocumentGenericPreview,
};
pub use crate::views::stage::layout::layout_item_base::{
    LayoutItemBase, StateRequest, TextSelectType, TextState,
};
pub use crate::views::stage::layout::layout_mosaic::{
    FoundItem, LayoutMosaic, MosaicRow, INLINE_ITEMS_MAX_PER_ROW, INLINE_RESULTS_MIN_WIDTH,
};
pub use crate::views::stage::layout::layout_position::{
    LayoutPosition, Position, MATRIX_ROW_SHIFT,
};
pub use crate::views::stage::layout::layout_selection::{
    LayoutSelection, TextSelection, FULL_SELECTION, MAX_GROUP_SELECTION_ITEMS,
};
