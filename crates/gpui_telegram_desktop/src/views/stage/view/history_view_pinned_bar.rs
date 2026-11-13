//! History View Pinned Bar View Component (`history_view_pinned_bar`)
//!
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_pinned_bar.h` (L1-L67)
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_pinned_bar.cpp` (L1-L409)

use crate::views::stage::pinned_bar::{
    format_pinned_title, HistoryViewPinnedBarState, K_FULL_LOADED, K_NOT_LOADED,
    K_SOME_LOADED,
};

/// GPUI view component for pinned message banner
#[derive(Debug, Clone)]
pub struct HistoryViewPinnedBarView {
    pub inner: HistoryViewPinnedBarState,
    pub height: f32,
}

impl HistoryViewPinnedBarView {
    pub fn new(is_channel: bool) -> Self {
        Self {
            inner: HistoryViewPinnedBarState::new(is_channel),
            height: 44.0,
        }
    }

    pub fn set_pinned_messages(&mut self, ids: Vec<i64>) {
        let count = ids.len();
        self.inner.tracker.set_pinned_ids(ids);
        self.inner.is_visible = count > 0;
        self.inner.title = format_pinned_title(self.inner.tracker.current_index, count);
    }

    pub fn on_bar_clicked(&mut self) -> Option<i64> {
        self.inner.on_bar_clicked()
    }

    pub fn current_pinned_id(&self) -> Option<i64> {
        self.inner.tracker.current_msg_id()
    }

    pub fn is_fully_loaded(&self) -> bool {
        self.inner.load_level == K_FULL_LOADED
    }

    pub fn update_preview(&mut self, text: String, has_media: bool, spoiler: bool) {
        self.inner.update_content(text, has_media, spoiler);
    }
}
