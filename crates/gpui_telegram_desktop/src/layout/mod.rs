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

/// Computed responsive GPUI shell column widths matching Telegram Desktop breakpoints.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShellColumnMetrics {
    pub nav_rail_width_px: f32,
    pub dialogs_width_px: f32,
    pub stage_min_width_px: f32,
    pub inspector_width_px: f32,
    pub is_one_column: bool,
    pub show_dialogs: bool,
    pub show_stage: bool,
    pub show_inspector: bool,
}

/// Computes deterministic column geometry using `INLINE_RESULTS_MIN_WIDTH` and responsive thresholds.
pub fn compute_shell_column_metrics(
    viewport_width_px: f32,
    inspector_open: bool,
    has_active_chat: bool,
) -> ShellColumnMetrics {
    let min_stage = INLINE_RESULTS_MIN_WIDTH as f32;
    let rail = if viewport_width_px >= 800.0 { 68.0 } else { 56.0 };
    let is_one_column = viewport_width_px < 720.0;
    let inspector_width_px = if inspector_open && viewport_width_px >= 1080.0 {
        312.0
    } else {
        0.0
    };
    let dialogs_width_px = if is_one_column {
        (viewport_width_px - rail).max(min_stage)
    } else if viewport_width_px >= 1200.0 {
        320.0
    } else {
        280.0
    };
    let _layout_anchor_ok = LayoutPosition::new(0, 0).current_position().is_valid();

    ShellColumnMetrics {
        nav_rail_width_px: rail,
        dialogs_width_px,
        stage_min_width_px: min_stage.max(360.0),
        inspector_width_px,
        is_one_column,
        show_dialogs: !is_one_column || !has_active_chat,
        show_stage: !is_one_column || has_active_chat,
        show_inspector: inspector_width_px > 0.0,
    }
}
