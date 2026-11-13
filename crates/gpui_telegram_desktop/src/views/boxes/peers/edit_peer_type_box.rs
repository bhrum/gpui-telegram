//! Edit Chat or Channel Type (Public vs Private) Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/edit_peer_type_box.h:L1-L80`
//! - `Telegram/SourceFiles/boxes/peers/edit_peer_type_box.cpp:L1-L963`
//! - `edit_peer_type_box.cpp:L50-L150` (Public/Private mode & username routing)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};
use super::edit_peer_common::EditPeerCommonPeers;

/// Target symbol in ledger: `EditPeerTypeBoxPeers`.
#[derive(Debug, Clone)]
pub struct EditPeerTypeBoxPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub peer_id: u64,
    pub is_channel: bool,
    pub is_public: bool,
    pub public_username: String,
    pub invite_link: Option<String>,
}

impl Default for EditPeerTypeBoxPeers {
    fn default() -> Self {
        Self::new(0, false)
    }
}

impl EditPeerTypeBoxPeers {
    pub fn new(peer_id: u64, is_channel: bool) -> Self {
        Self {
            is_visible: true,
            width: px(400.0),
            height: px(480.0),
            peer_id,
            is_channel,
            is_public: false,
            public_username: String::new(),
            invite_link: None,
        }
    }

    pub fn set_public(&mut self, is_public: bool) {
        self.is_public = is_public;
    }

    pub fn set_username(&mut self, username: &str) -> Result<(), &'static str> {
        let valid = EditPeerCommonPeers::validate_username(username)?;
        self.public_username = valid;
        Ok(())
    }
}
