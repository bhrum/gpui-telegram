//! Edit Collectible and Standard Usernames List Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/edit_peer_usernames_list.h:L1-L57`
//! - `Telegram/SourceFiles/boxes/peers/edit_peer_usernames_list.cpp:L1-L442`
//! - `edit_peer_usernames_list.cpp:L40-L150` (Username activation & reordering)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

/// Single username entry with active status and collectible flag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsernameEntry {
    pub username: String,
    pub is_active: bool,
    pub is_collectible: bool,
}

/// Target symbol in ledger: `EditPeerUsernamesListPeers`.
#[derive(Debug, Clone)]
pub struct EditPeerUsernamesListPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub peer_id: u64,
    pub usernames: Vec<UsernameEntry>,
}

impl Default for EditPeerUsernamesListPeers {
    fn default() -> Self {
        Self::new(0)
    }
}

impl EditPeerUsernamesListPeers {
    pub fn new(peer_id: u64) -> Self {
        Self {
            is_visible: true,
            width: px(400.0),
            height: px(460.0),
            peer_id,
            usernames: Vec::new(),
        }
    }

    pub fn toggle_username_active(&mut self, username: &str) -> bool {
        if let Some(entry) = self.usernames.iter_mut().find(|u| u.username == username) {
            entry.is_active = !entry.is_active;
            true
        } else {
            false
        }
    }

    pub fn primary_username(&self) -> Option<&str> {
        self.usernames
            .iter()
            .find(|u| u.is_active)
            .map(|u| u.username.as_str())
    }
}
