//! Organization Verification Badge Request Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/verify_peers_box.h:L1-L36`
//! - `Telegram/SourceFiles/boxes/peers/verify_peers_box.cpp:L1-L344`
//! - `verify_peers_box.cpp:L40-L120` (Custom verification badge assignment)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

/// Target symbol in ledger: `VerifyPeersBoxPeers`.
#[derive(Debug, Clone)]
pub struct VerifyPeersBoxPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub peer_id: u64,
    pub custom_description: String,
    pub icon_emoji_id: Option<u64>,
    pub is_submitting: bool,
}

impl Default for VerifyPeersBoxPeers {
    fn default() -> Self {
        Self::new(0)
    }
}

impl VerifyPeersBoxPeers {
    pub fn new(peer_id: u64) -> Self {
        Self {
            is_visible: true,
            width: px(380.0),
            height: px(400.0),
            peer_id,
            custom_description: String::new(),
            icon_emoji_id: None,
            is_submitting: false,
        }
    }

    pub fn set_verification(&mut self, desc: &str, icon_emoji_id: Option<u64>) {
        self.custom_description = desc.trim().to_string();
        self.icon_emoji_id = icon_emoji_id;
    }
}
