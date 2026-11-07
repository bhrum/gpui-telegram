//! History View Context Menu View Component (`history_view_context_menu`)
//!
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_context_menu.h` (L1-L183)
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_context_menu.cpp` (L1-L3118)

use std::time::Duration;

/// Anonymous namespace constants from `history_view_context_menu.cpp:L127-L130`
pub const K_RESCHEDULE_LIMIT: usize = 20;
pub const K_TAG_NAME_LIMIT: usize = 12;
pub const K_PUBLIC_POST_LINK_TOAST_DURATION: Duration = Duration::from_millis(4000);
pub const K_MIN_VIEWS_COUNT: usize = 10;

/// Action entries available in history item context menu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HistoryMenuAction {
    Reply,
    Edit,
    Pin,
    Unpin,
    CopyText,
    CopyLink,
    Forward,
    Delete,
    Select,
    Reschedule,
    Factcheck,
    Translate,
    RateTranscribe,
    ViewSeenBy,
}

/// GPUI view component for message context menu
#[derive(Debug, Clone)]
pub struct HistoryViewContextMenuView {
    pub message_id: i64,
    pub peer_id: i64,
    pub is_outgoing: bool,
    pub can_edit: bool,
    pub can_delete: bool,
    pub can_pin: bool,
    pub available_actions: Vec<HistoryMenuAction>,
    pub is_open: bool,
}

impl HistoryViewContextMenuView {
    pub fn new(message_id: i64, peer_id: i64, is_outgoing: bool) -> Self {
        let mut menu = Self {
            message_id,
            peer_id,
            is_outgoing,
            can_edit: is_outgoing,
            can_delete: true,
            can_pin: true,
            available_actions: Vec::new(),
            is_open: false,
        };
        menu.rebuild_actions();
        menu
    }

    pub fn rebuild_actions(&mut self) {
        let mut actions = vec![HistoryMenuAction::Reply];
        if self.can_edit && self.is_outgoing {
            actions.push(HistoryMenuAction::Edit);
        }
        if self.can_pin {
            actions.push(HistoryMenuAction::Pin);
        }
        actions.push(HistoryMenuAction::CopyText);
        actions.push(HistoryMenuAction::CopyLink);
        actions.push(HistoryMenuAction::Forward);
        actions.push(HistoryMenuAction::Select);
        if self.can_delete {
            actions.push(HistoryMenuAction::Delete);
        }
        self.available_actions = actions;
    }

    pub fn has_action(&self, action: HistoryMenuAction) -> bool {
        self.available_actions.contains(&action)
    }

    pub fn reschedule_limit(&self) -> usize {
        K_RESCHEDULE_LIMIT
    }

    pub fn tag_name_limit(&self) -> usize {
        K_TAG_NAME_LIMIT
    }

    pub fn toast_duration(&self) -> Duration {
        K_PUBLIC_POST_LINK_TOAST_DURATION
    }

    pub fn min_views_count(&self) -> usize {
        K_MIN_VIEWS_COUNT
    }
}
