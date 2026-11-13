//! Edit Peer Name and Profile Accent Color Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/edit_peer_color_box.h:L1-L150`
//! - `Telegram/SourceFiles/boxes/peers/edit_peer_color_box.cpp:L1-L2834`
//! - `edit_peer_color_box.cpp:L50-L150` (Color palette indices & boost requirements)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

/// Target symbol in ledger: `EditPeerColorBoxPeers`.
#[derive(Debug, Clone)]
pub struct EditPeerColorBoxPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub peer_id: u64,
    pub color_index: u8,
    pub background_emoji_id: Option<u64>,
    pub required_boost_level: u8,
    pub current_boost_level: u8,
}

impl Default for EditPeerColorBoxPeers {
    fn default() -> Self {
        Self::new(0)
    }
}

impl EditPeerColorBoxPeers {
    pub fn new(peer_id: u64) -> Self {
        Self {
            is_visible: true,
            width: px(400.0),
            height: px(520.0),
            peer_id,
            color_index: 0,
            background_emoji_id: None,
            required_boost_level: 1,
            current_boost_level: 0,
        }
    }

    pub fn set_color_index(&mut self, index: u8) {
        self.color_index = index % 16;
        self.required_boost_level = if self.color_index >= 7 { 2 } else { 1 };
    }

    pub fn can_apply(&self) -> bool {
        self.current_boost_level >= self.required_boost_level
    }
}
