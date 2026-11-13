//! Add Participants to Chat or Channel Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/add_participants_box.h:L1-L231`
//! - `Telegram/SourceFiles/boxes/peers/add_participants_box.cpp:L1-L2096`
//! - `Telegram/SourceFiles/boxes/peers/add_participants_box.cpp:L45-L55` (`kParticipantsFirstPageCount`, `kParticipantsPerPage`, `kUserpicsLimit`)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};
use std::collections::HashSet;

pub const K_PARTICIPANTS_FIRST_PAGE_COUNT: usize = 50;
pub const K_PARTICIPANTS_PER_PAGE: usize = 50;
pub const K_USERPICS_LIMIT: usize = 5;

/// Dialog box for searching and adding new members or subscribers to a chat.
/// Target symbol in ledger: `AddParticipantsBoxPeers`.
#[derive(Debug, Clone)]
pub struct AddParticipantsBoxPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub peer_id: u64,
    pub is_channel: bool,
    pub selected_peer_ids: HashSet<u64>,
    pub invite_link: Option<String>,
    pub max_select_count: usize,
    pub is_submitting: bool,
}

impl Default for AddParticipantsBoxPeers {
    fn default() -> Self {
        Self::new(0, false)
    }
}

impl AddParticipantsBoxPeers {
    pub fn new(peer_id: u64, is_channel: bool) -> Self {
        Self {
            is_visible: true,
            width: px(400.0),
            height: px(520.0),
            peer_id,
            is_channel,
            selected_peer_ids: HashSet::new(),
            invite_link: None,
            max_select_count: if is_channel { 200 } else { 100 },
            is_submitting: false,
        }
    }

    pub fn toggle_selection(&mut self, peer_id: u64) -> bool {
        if self.selected_peer_ids.contains(&peer_id) {
            self.selected_peer_ids.remove(&peer_id);
            true
        } else if self.selected_peer_ids.len() < self.max_select_count {
            self.selected_peer_ids.insert(peer_id);
            true
        } else {
            false
        }
    }

    pub fn selected_count(&self) -> usize {
        self.selected_peer_ids.len()
    }

    pub fn can_submit(&self) -> bool {
        !self.selected_peer_ids.is_empty() && !self.is_submitting
    }
}
