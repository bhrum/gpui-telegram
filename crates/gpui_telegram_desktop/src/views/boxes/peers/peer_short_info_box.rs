//! Peer Short Profile Info Popup Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/peer_short_info_box.h:L1-L213`
//! - `Telegram/SourceFiles/boxes/peers/peer_short_info_box.cpp:L1-L935`
//! - `peer_short_info_box.cpp:L50-L150` (Profile card layout & action buttons)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

/// Target symbol in ledger: `PeerShortInfoBoxPeers`.
#[derive(Debug, Clone)]
pub struct PeerShortInfoBoxPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub peer_id: u64,
    pub title: String,
    pub username: Option<String>,
    pub bio: Option<String>,
    pub is_contact: bool,
    pub is_blocked: bool,
}

impl Default for PeerShortInfoBoxPeers {
    fn default() -> Self {
        Self::new(0, "User".to_string())
    }
}

impl PeerShortInfoBoxPeers {
    pub fn new(peer_id: u64, title: String) -> Self {
        Self {
            is_visible: true,
            width: px(340.0),
            height: px(380.0),
            peer_id,
            title,
            username: None,
            bio: None,
            is_contact: false,
            is_blocked: false,
        }
    }

    pub fn set_profile(&mut self, username: Option<String>, bio: Option<String>, contact: bool) {
        self.username = username;
        self.bio = bio;
        self.is_contact = contact;
    }
}
