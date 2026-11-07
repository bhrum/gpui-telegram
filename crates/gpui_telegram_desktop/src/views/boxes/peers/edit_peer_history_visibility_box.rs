//! Edit Chat History Visibility for New Members Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/edit_peer_history_visibility_box.h:L1-L30`
//! - `Telegram/SourceFiles/boxes/peers/edit_peer_history_visibility_box.cpp:L1-L107`
//! - `edit_peer_history_visibility_box.cpp:L25-L95` (Visible vs Hidden selection)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

/// Target symbol in ledger: `EditPeerHistoryVisibilityBoxPeers`.
#[derive(Debug, Clone)]
pub struct EditPeerHistoryVisibilityBoxPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub chat_id: u64,
    pub history_visible_for_new_members: bool,
}

impl Default for EditPeerHistoryVisibilityBoxPeers {
    fn default() -> Self {
        Self::new(0)
    }
}

impl EditPeerHistoryVisibilityBoxPeers {
    pub fn new(chat_id: u64) -> Self {
        Self {
            is_visible: true,
            width: px(380.0),
            height: px(320.0),
            chat_id,
            history_visible_for_new_members: true,
        }
    }

    pub fn set_history_visible(&mut self, visible: bool) {
        self.history_visible_for_new_members = visible;
    }
}
