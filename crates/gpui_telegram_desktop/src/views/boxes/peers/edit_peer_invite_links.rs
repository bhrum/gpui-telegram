//! Active and Revoked Chat Invite Links List Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/edit_peer_invite_links.h:L1-L100`
//! - `Telegram/SourceFiles/boxes/peers/edit_peer_invite_links.cpp:L1-L1085`
//! - `edit_peer_invite_links.cpp:L50-L150` (Active & revoked invite links overview)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

/// Single invite link record with stats.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InviteLinkItem {
    pub link: String,
    pub title: String,
    pub is_primary: bool,
    pub is_revoked: bool,
    pub usage_count: u32,
    pub usage_limit: Option<u32>,
}

/// Target symbol in ledger: `EditPeerInviteLinksPeers`.
#[derive(Debug, Clone)]
pub struct EditPeerInviteLinksPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub peer_id: u64,
    pub links: Vec<InviteLinkItem>,
}

impl Default for EditPeerInviteLinksPeers {
    fn default() -> Self {
        Self::new(0)
    }
}

impl EditPeerInviteLinksPeers {
    pub fn new(peer_id: u64) -> Self {
        Self {
            is_visible: true,
            width: px(420.0),
            height: px(520.0),
            peer_id,
            links: Vec::new(),
        }
    }

    pub fn primary_link(&self) -> Option<&InviteLinkItem> {
        self.links.iter().find(|l| l.is_primary && !l.is_revoked)
    }
}
