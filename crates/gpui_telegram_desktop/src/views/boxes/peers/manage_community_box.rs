//! Manage Community Settings and Linked Channels Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/manage_community_box.h:L1-L18`
//! - `Telegram/SourceFiles/boxes/peers/manage_community_box.cpp:L1-L354`
//! - `Telegram/SourceFiles/boxes/peers/manage_community_box.cpp:L40-L120` (Admin controls)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

/// Target symbol in ledger: `ManageCommunityBoxPeers`.
#[derive(Debug, Clone)]
pub struct ManageCommunityBoxPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub community_id: u64,
    pub can_edit_info: bool,
    pub can_invite_channels: bool,
    pub can_remove_channels: bool,
}

impl Default for ManageCommunityBoxPeers {
    fn default() -> Self {
        Self::new(0)
    }
}

impl ManageCommunityBoxPeers {
    pub fn new(community_id: u64) -> Self {
        Self {
            is_visible: true,
            width: px(400.0),
            height: px(480.0),
            community_id,
            can_edit_info: true,
            can_invite_channels: true,
            can_remove_channels: true,
        }
    }
}
