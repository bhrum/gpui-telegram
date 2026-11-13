//! Edit Group or Channel Primary Information Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/edit_peer_info_box.h:L1-L200`
//! - `Telegram/SourceFiles/boxes/peers/edit_peer_info_box.cpp:L1-L3279`
//! - `edit_peer_info_box.cpp:L50-L150` (Profile editor fields)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};
use super::edit_peer_common::{EditPeerCommonPeers, K_MAX_CHANNEL_DESCRIPTION};

/// Target symbol in ledger: `EditPeerInfoBoxPeers`.
#[derive(Debug, Clone)]
pub struct EditPeerInfoBoxPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub peer_id: u64,
    pub is_channel: bool,
    pub title: String,
    pub description: String,
    pub has_custom_photo: bool,
    pub is_submitting: bool,
}

impl Default for EditPeerInfoBoxPeers {
    fn default() -> Self {
        Self::new(0, false)
    }
}

impl EditPeerInfoBoxPeers {
    pub fn new(peer_id: u64, is_channel: bool) -> Self {
        Self {
            is_visible: true,
            width: px(400.0),
            height: px(500.0),
            peer_id,
            is_channel,
            title: String::new(),
            description: String::new(),
            has_custom_photo: false,
            is_submitting: false,
        }
    }

    pub fn set_title(&mut self, title: &str) -> Result<(), &'static str> {
        let valid = EditPeerCommonPeers::validate_title(title)?;
        self.title = valid;
        Ok(())
    }

    pub fn set_description(&mut self, text: &str) {
        self.description = text.chars().take(K_MAX_CHANNEL_DESCRIPTION).collect();
    }

    pub fn can_save(&self) -> bool {
        !self.title.trim().is_empty() && !self.is_submitting
    }
}
