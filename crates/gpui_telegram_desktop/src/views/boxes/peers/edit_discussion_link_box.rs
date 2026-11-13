//! Link or Unlink Discussion Group Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/edit_discussion_link_box.h:L1-L35`
//! - `Telegram/SourceFiles/boxes/peers/edit_discussion_link_box.cpp:L1-L363`
//! - `edit_discussion_link_box.cpp:L40-L120` (Discussion supergroup link binding)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

/// Target symbol in ledger: `EditDiscussionLinkBoxPeers`.
#[derive(Debug, Clone)]
pub struct EditDiscussionLinkBoxPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub channel_id: u64,
    pub current_discussion_group_id: Option<u64>,
    pub selected_group_id: Option<u64>,
}

impl Default for EditDiscussionLinkBoxPeers {
    fn default() -> Self {
        Self::new(0)
    }
}

impl EditDiscussionLinkBoxPeers {
    pub fn new(channel_id: u64) -> Self {
        Self {
            is_visible: true,
            width: px(380.0),
            height: px(460.0),
            channel_id,
            current_discussion_group_id: None,
            selected_group_id: None,
        }
    }

    pub fn set_current_group(&mut self, group_id: Option<u64>) {
        self.current_discussion_group_id = group_id;
    }

    pub fn select_group(&mut self, group_id: u64) {
        self.selected_group_id = Some(group_id);
    }

    pub fn is_unlinking(&self) -> bool {
        self.selected_group_id.is_none() && self.current_discussion_group_id.is_some()
    }
}
