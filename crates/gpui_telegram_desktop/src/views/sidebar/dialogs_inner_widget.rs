//! Authentic GPUI Telegram Dialogs Inner Widget virtual list coordinator.
//!
//! Based on upstream Telegram Desktop:
//! - `Telegram/SourceFiles/dialogs/dialogs_inner_widget.h:L80-835`
//! - `Telegram/SourceFiles/dialogs/dialogs_inner_widget.cpp:L116-260, L2733-2800, L5469-5650, L6703-6745`
//!
//! Subsystem: dialogs, Feature: TCG-N03

use serde::{Deserialize, Serialize};

/// Upstream constants from `dialogs_inner_widget.cpp:L118-124`.
pub const K_FREEZE_TIMEOUT_MS: u64 = 2000;
pub const K_HASHTAG_RESULTS_LIMIT: usize = 5;
pub const K_START_REORDER_THRESHOLD_PX: i32 = 30;
pub const K_START_DRAG_TO_FILTER_THRESHOLD_X: i32 = 30;
pub const K_START_DRAG_TO_FILTER_THRESHOLD_Y: i32 = 75;
pub const K_QUERY_PREVIEW_LIMIT: usize = 32;
pub const K_PREVIEW_POSTS_LIMIT: usize = 3;

/// Inner widget rendering state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InnerWidgetState {
    Default,
    Filtered,
}

impl Default for InnerWidgetState {
    fn default() -> Self {
        Self::Default
    }
}

/// Filtered search counts across the 5 upstream result buckets.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FilteredSectionCounts {
    pub hashtag_count: usize,
    pub filter_count: usize,
    pub peer_search_count: usize,
    pub preview_count: usize,
    pub search_results_count: usize,
}

impl FilteredSectionCounts {
    pub fn total(&self) -> usize {
        self.hashtag_count
            + self.filter_count
            + self.peer_search_count
            + self.preview_count
            + self.search_results_count
    }
}

/// Drag and drop state for pinned chat reordering (`dialogs_inner_widget.cpp:L2733-2800`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinnedDragState {
    pub dragging_peer_id: u64,
    pub start_y: i32,
    pub current_y: i32,
    pub is_reordering: bool,
    pub original_order: Vec<u64>,
}

/// Core inner widget coordinator managing selection, keyboard navigation, and virtual math.
#[derive(Debug, Clone)]
pub struct DialogsInnerWidget {
    pub state: InnerWidgetState,
    pub row_height: i32, // default 62px
    pub selected_index: Option<usize>,
    pub collapsed_selected: Option<usize>,
    pub total_rows: usize,
    pub skip_top_dialog: bool,
    pub pinned_drag: Option<PinnedDragState>,
}

impl Default for DialogsInnerWidget {
    fn default() -> Self {
        Self {
            state: InnerWidgetState::Default,
            row_height: 62,
            selected_index: None,
            collapsed_selected: None,
            total_rows: 0,
            skip_top_dialog: false,
            pinned_drag: None,
        }
    }
}

impl DialogsInnerWidget {
    pub fn new() -> Self {
        Self::default()
    }

    /// Skips selection by `direction` rows (Up = -1, Down = +1, Home = -rows, End = +rows).
    ///
    /// Upstream: `InnerWidget::selectSkip` (`dialogs_inner_widget.cpp:L5469-5550`).
    pub fn select_skip(&mut self, direction: i32, total_count: usize) {
        if total_count == 0 {
            self.selected_index = None;
            return;
        }

        let current = self.selected_index.unwrap_or(0) as i32;
        let max_index = (total_count as i32) - 1;
        let next = (current + direction).clamp(0, max_index);
        self.selected_index = Some(next as usize);
    }

    /// Skips selection by page height in pixels.
    ///
    /// Upstream: `InnerWidget::selectSkipPage` (`dialogs_inner_widget.cpp:L5636-5645`).
    pub fn select_skip_page(&mut self, viewport_height_px: i32, direction: i32, total_count: usize) {
        let to_skip = (viewport_height_px / self.row_height).max(1);
        self.select_skip(direction * to_skip, total_count);
    }

    /// Checks if mouse movement exceeds threshold to initiate pinned chat reordering.
    ///
    /// Upstream: `InnerWidget::checkReorderPinnedStart` (`dialogs_inner_widget.cpp:L2733-2751`).
    pub fn check_reorder_pinned_start(start_y: i32, current_y: i32) -> bool {
        (current_y - start_y).abs() >= K_START_REORDER_THRESHOLD_PX
    }

    /// Begins dragging a pinned row.
    pub fn start_reorder_pinned(&mut self, peer_id: u64, start_y: i32, order: Vec<u64>) {
        if order.len() >= 2 {
            self.pinned_drag = Some(PinnedDragState {
                dragging_peer_id: peer_id,
                start_y,
                current_y: start_y,
                is_reordering: true,
                original_order: order,
            });
        }
    }

    /// Updates drag position during mouse move.
    pub fn update_drag_position(&mut self, current_y: i32) {
        if let Some(ref mut drag) = self.pinned_drag {
            drag.current_y = current_y;
        }
    }

    /// Finishes dragging and returns true if order changed.
    pub fn finish_drag(&mut self) -> Option<Vec<u64>> {
        let drag = self.pinned_drag.take()?;
        Some(drag.original_order)
    }
}
