//! History View Pinned Section View Component (`history_view_pinned_section`)
//!
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_pinned_section.h` (L1-L246)
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_pinned_section.cpp` (L1-L853)

use crate::views::stage::pinned_bar::PinnedSectionMemento;

/// GPUI view component for full pinned messages section
#[derive(Debug, Clone)]
pub struct HistoryViewPinnedSectionView {
    pub memento: PinnedSectionMemento,
    pub is_active: bool,
    pub pinned_messages_count: usize,
    pub selected_unpin_id: Option<i64>,
}

impl HistoryViewPinnedSectionView {
    pub fn new(peer_id: i64, topic_id: Option<i64>, highlight_msg_id: Option<i64>) -> Self {
        Self {
            memento: PinnedSectionMemento::new(peer_id, topic_id, highlight_msg_id),
            is_active: true,
            pinned_messages_count: 0,
            selected_unpin_id: None,
        }
    }

    pub fn set_pinned_count(&mut self, count: usize) {
        self.pinned_messages_count = count;
    }

    pub fn request_unpin(&mut self, msg_id: i64) {
        self.selected_unpin_id = Some(msg_id);
    }

    pub fn clear_unpin_request(&mut self) {
        self.selected_unpin_id = None;
    }
}
