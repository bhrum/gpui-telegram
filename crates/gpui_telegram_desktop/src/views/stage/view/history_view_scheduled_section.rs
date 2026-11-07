//! History View Scheduled Section View Component (`history_view_scheduled_section`)
//!
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_scheduled_section.h` (L1-L362)
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_scheduled_section.cpp` (L1-L1882)

use crate::views::stage::scheduled_section::{
    HistoryViewScheduledSectionState, ScheduledMessageItem,
    K_VIDEO_PROCESSING_INFO_DURATION,
};

/// GPUI view component for scheduled messages section
#[derive(Debug, Clone)]
pub struct HistoryViewScheduledSectionView {
    pub inner: HistoryViewScheduledSectionState,
    pub is_focused: bool,
}

impl HistoryViewScheduledSectionView {
    pub fn new(peer_id: i64, topic_id: Option<i64>) -> Self {
        Self {
            inner: HistoryViewScheduledSectionState::new(peer_id, topic_id),
            is_focused: true,
        }
    }

    pub fn set_items(&mut self, items: Vec<ScheduledMessageItem>) {
        self.inner.items = items;
    }

    pub fn send_now(&mut self, id: i64) -> Option<ScheduledMessageItem> {
        self.inner.send_now(id)
    }

    pub fn reschedule(&mut self, id: i64, timestamp: i64) -> bool {
        self.inner.reschedule(id, timestamp)
    }

    pub fn count(&self) -> usize {
        self.inner.items.len()
    }

    pub fn video_processing_duration(&self) -> std::time::Duration {
        K_VIDEO_PROCESSING_INFO_DURATION
    }
}
