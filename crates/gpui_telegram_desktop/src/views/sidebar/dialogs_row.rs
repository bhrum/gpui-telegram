//! Visual row model for dialog entries with positioning and badges.
//!
//! Upstream:
//! - `Telegram/SourceFiles/dialogs/dialogs_row.h:L1-L276`
//! - `Telegram/SourceFiles/dialogs/dialogs_row.cpp:L1-L1002`
//!
//! Anonymous Constants & Style Metrics:
//! - `kTopLayer = 3`, `kHiddenLayer = 2`, `kBottomLayer = 1`, `kNoneLayer = 0` (`dialogs_row.cpp:L43-46`)
//! - `kBlurRadius = 24` (`dialogs_row.cpp:L47`)
//! - `Row::ComputeSt`: `forumDialogRow` (80px), `taggedForumDialogRow` (96px), `taggedDialogRow` (72px), `defaultDialogRow` (62px) (`dialogs_row.cpp:L406-422`)
//! - `Row::recountHeight` (`dialogs_row.cpp:L424-432`): Height interpolation with narrow ratio

pub use gpui_telegram_client::dialogs::list::DialogRowItem;
use crate::views::sidebar::dialogs_entry::DialogsEntry;

pub const K_TOP_LAYER: i32 = 3;
pub const K_HIDDEN_LAYER: i32 = 2;
pub const K_BOTTOM_LAYER: i32 = 1;
pub const K_NONE_LAYER: i32 = 0;
pub const K_BLUR_RADIUS: i32 = 24;

/// Visual row component for a conversation entry in GPUI.
/// Upstream: `Dialogs::Row` (`dialogs_row.h:L84-227`)
#[derive(Debug, Clone)]
pub struct DialogsRow {
    pub entry: DialogsEntry,
    pub top: i32,
    pub height: i32,
    pub index: usize,
    pub is_selected: bool,
    pub is_pressed: bool,
    pub sort_key: u64,
}

impl DialogsRow {
    pub fn new(entry: DialogsEntry, index: usize, top: i32) -> Self {
        let sort_key = entry.compute_sort_position(false);
        let mut row = Self {
            entry,
            top,
            height: 62,
            index,
            is_selected: false,
            is_pressed: false,
            sort_key,
        };
        row.recount_height(0.0);
        row
    }

    /// Recount row height given sidebar narrow ratio.
    /// Upstream: `dialogs_row.cpp:L424-432`
    pub fn recount_height(&mut self, narrow_ratio: f64) {
        let base_height = if self.entry.data.is_forum {
            if self.entry.data.has_filter_tags { 96 } else { 80 }
        } else if self.entry.data.has_filter_tags {
            72
        } else {
            62
        };

        self.height = if base_height == 62 {
            62
        } else {
            let diff = (62 - base_height) as f64 * narrow_ratio;
            (base_height as f64 + diff).round() as i32
        };
    }

    pub fn set_top(&mut self, top: i32) {
        self.top = top;
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.is_selected = selected;
        self.entry.set_hovered(selected);
    }
}
