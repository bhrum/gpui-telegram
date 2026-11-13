//! Edit Participants Tabbed Manager Dialog Box (Members, Admins, Banned, Restricted)
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/edit_participants_box.h:L1-L200`
//! - `Telegram/SourceFiles/boxes/peers/edit_participants_box.cpp:L1-L3058`
//! - `edit_participants_box.cpp:L100-L250` (Member tab classification)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

/// Participant tab category.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticipantsTab {
    Members,
    Admins,
    Banned,
    Restricted,
}

/// Target symbol in ledger: `EditParticipantsBoxPeers`.
#[derive(Debug, Clone)]
pub struct EditParticipantsBoxPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub chat_id: u64,
    pub active_tab: ParticipantsTab,
    pub member_count: usize,
    pub admin_count: usize,
    pub banned_count: usize,
    pub restricted_count: usize,
}

impl Default for EditParticipantsBoxPeers {
    fn default() -> Self {
        Self::new(0)
    }
}

impl EditParticipantsBoxPeers {
    pub fn new(chat_id: u64) -> Self {
        Self {
            is_visible: true,
            width: px(420.0),
            height: px(540.0),
            chat_id,
            active_tab: ParticipantsTab::Members,
            member_count: 0,
            admin_count: 0,
            banned_count: 0,
            restricted_count: 0,
        }
    }

    pub fn set_tab(&mut self, tab: ParticipantsTab) {
        self.active_tab = tab;
    }
}
