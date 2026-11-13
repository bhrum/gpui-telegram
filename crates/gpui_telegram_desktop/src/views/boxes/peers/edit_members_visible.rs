//! Edit Members Visibility (Hide Members List) Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/edit_members_visible.h:L1-L25`
//! - `Telegram/SourceFiles/boxes/peers/edit_members_visible.cpp:L1-L82`
//! - `edit_members_visible.cpp:L20-L80` (100+ member threshold requirement)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

pub const K_MIN_MEMBERS_FOR_HIDE_LIST: usize = 100;

/// Target symbol in ledger: `EditMembersVisiblePeers`.
#[derive(Debug, Clone)]
pub struct EditMembersVisiblePeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub chat_id: u64,
    pub member_count: usize,
    pub hide_members: bool,
}

impl Default for EditMembersVisiblePeers {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl EditMembersVisiblePeers {
    pub fn new(chat_id: u64, member_count: usize) -> Self {
        Self {
            is_visible: true,
            width: px(380.0),
            height: px(300.0),
            chat_id,
            member_count,
            hide_members: false,
        }
    }

    pub fn can_hide_members(&self) -> bool {
        self.member_count >= K_MIN_MEMBERS_FOR_HIDE_LIST
    }

    pub fn toggle_hide_members(&mut self) -> bool {
        if self.can_hide_members() {
            self.hide_members = !self.hide_members;
            true
        } else {
            false
        }
    }
}
