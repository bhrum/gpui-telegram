//! History View List Widget View Component (`history_view_list_widget`)
//!
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_list_widget.h` (L1-L1128)
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_list_widget.cpp` (L1-L6833)

use crate::views::stage::list_widget::{
    hides_dates, supports_read_metrics, HistoryListContext, HistoryViewListWidgetState,
    VirtualListGeometry, VirtualMessageItem, K_CLEAR_USERPICS_AFTER,
    K_PRELOADED_SCREENS_COUNT, K_PRELOAD_IF_LESS_THAN_SCREENS,
};

/// GPUI view component for primary chat message list
#[derive(Debug, Clone)]
pub struct HistoryViewListWidgetView {
    pub inner: HistoryViewListWidgetState,
    pub is_visible: bool,
    pub is_wide_layout: bool,
    pub peer_id: i64,
}

impl HistoryViewListWidgetView {
    pub fn new(peer_id: i64, context: HistoryListContext, viewport_height: f32) -> Self {
        Self {
            inner: HistoryViewListWidgetState::new(context, viewport_height),
            is_visible: true,
            is_wide_layout: true,
            peer_id,
        }
    }

    pub fn set_items(&mut self, items: Vec<VirtualMessageItem>) {
        self.inner.set_items(items);
    }

    pub fn visible_item_range(&self) -> (usize, usize) {
        self.inner.geometry.calculate_visible_range(&self.inner.items)
    }

    pub fn preloaded_item_range(&self) -> (usize, usize) {
        self.inner.geometry.calculate_preloaded_range(&self.inner.items)
    }

    pub fn scroll_to_item(&mut self, msg_id: i64) -> bool {
        self.inner.scroll_to_message(msg_id)
    }

    pub fn toggle_selection(&mut self, msg_id: i64) -> bool {
        self.inner.selection.toggle_selection(msg_id)
    }

    pub fn clear_selection(&mut self) {
        self.inner.selection.clear();
    }

    pub fn selected_count(&self) -> usize {
        self.inner.selection.count()
    }

    pub fn should_hide_dates(&self) -> bool {
        hides_dates(self.inner.context)
    }

    pub fn has_read_metrics(&self) -> bool {
        supports_read_metrics(self.inner.context)
    }

    pub fn preloaded_screens_count(&self) -> usize {
        K_PRELOADED_SCREENS_COUNT
    }

    pub fn preload_trigger_threshold(&self) -> usize {
        K_PRELOAD_IF_LESS_THAN_SCREENS
    }

    pub fn clear_userpics_limit(&self) -> usize {
        K_CLEAR_USERPICS_AFTER
    }
}
