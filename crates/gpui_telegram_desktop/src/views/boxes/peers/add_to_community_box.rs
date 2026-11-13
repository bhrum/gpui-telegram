//! Add Channel or Group to Community Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/add_to_community_box.h:L1-L26`
//! - `Telegram/SourceFiles/boxes/peers/add_to_community_box.cpp:L1-L492`
//! - `Telegram/SourceFiles/boxes/peers/add_to_community_box.cpp:L25-L100` (`kCommunityRequestCreated`, `kCommunityPeersTooMuch`)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

/// Target symbol in ledger: `AddToCommunityBoxPeers`.
#[derive(Debug, Clone)]
pub struct AddToCommunityBoxPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub community_id: u64,
    pub selected_channel_ids: Vec<u64>,
    pub max_channels: usize,
    pub is_submitting: bool,
}

impl Default for AddToCommunityBoxPeers {
    fn default() -> Self {
        Self::new(0)
    }
}

impl AddToCommunityBoxPeers {
    pub fn new(community_id: u64) -> Self {
        Self {
            is_visible: true,
            width: px(380.0),
            height: px(460.0),
            community_id,
            selected_channel_ids: Vec::new(),
            max_channels: 50,
            is_submitting: false,
        }
    }

    pub fn toggle_channel(&mut self, channel_id: u64) -> bool {
        if let Some(pos) = self.selected_channel_ids.iter().position(|&id| id == channel_id) {
            self.selected_channel_ids.remove(pos);
            true
        } else if self.selected_channel_ids.len() < self.max_channels {
            self.selected_channel_ids.push(channel_id);
            true
        } else {
            false
        }
    }
}
