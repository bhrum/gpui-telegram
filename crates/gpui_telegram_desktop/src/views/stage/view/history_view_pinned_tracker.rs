//! History View Pinned Tracker View Component (`history_view_pinned_tracker`)
//!
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_pinned_tracker.h` (L1-L63)
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_pinned_tracker.cpp` (L1-L186)

use crate::views::stage::pinned_bar::{
    project_universal_msg_id, PinnedTracker, K_CHANGE_VIEWER_LIMIT, K_LOADED_LIMIT,
    SERVER_MAX_MSG_ID,
};

/// GPUI view component for multi-pinned cycling tracker
#[derive(Debug, Clone)]
pub struct HistoryViewPinnedTrackerView {
    pub tracker: PinnedTracker,
    pub loaded_limit: usize,
    pub change_viewer_limit: usize,
}

impl HistoryViewPinnedTrackerView {
    pub fn new(is_channel: bool) -> Self {
        Self {
            tracker: PinnedTracker::new(is_channel),
            loaded_limit: K_LOADED_LIMIT,
            change_viewer_limit: K_CHANGE_VIEWER_LIMIT,
        }
    }

    pub fn set_ids(&mut self, ids: Vec<i64>) {
        self.tracker.set_pinned_ids(ids);
    }

    pub fn next(&mut self) -> Option<i64> {
        self.tracker.next_pinned_id()
    }

    pub fn current(&self) -> Option<i64> {
        self.tracker.current_msg_id()
    }

    pub fn project_id(&self, msg_id: i64) -> i64 {
        project_universal_msg_id(self.tracker.is_channel, msg_id)
    }

    pub fn server_max_msg_boundary(&self) -> i64 {
        SERVER_MAX_MSG_ID
    }
}
