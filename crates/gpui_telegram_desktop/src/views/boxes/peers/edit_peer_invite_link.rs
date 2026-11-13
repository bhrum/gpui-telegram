//! Create and Edit Chat Invite Link Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/edit_peer_invite_link.h:L1-L100`
//! - `Telegram/SourceFiles/boxes/peers/edit_peer_invite_link.cpp:L1-L1679`
//! - `edit_peer_invite_link.cpp:L50-L150` (Invite link configuration fields)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

/// Target symbol in ledger: `EditPeerInviteLinkPeers`.
#[derive(Debug, Clone)]
pub struct EditPeerInviteLinkPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub peer_id: u64,
    pub title: String,
    pub expire_date_sec: Option<i64>,
    pub usage_limit: Option<u32>,
    pub request_approval: bool,
    pub is_submitting: bool,
}

impl Default for EditPeerInviteLinkPeers {
    fn default() -> Self {
        Self::new(0)
    }
}

impl EditPeerInviteLinkPeers {
    pub fn new(peer_id: u64) -> Self {
        Self {
            is_visible: true,
            width: px(400.0),
            height: px(480.0),
            peer_id,
            title: String::new(),
            expire_date_sec: None,
            usage_limit: None,
            request_approval: false,
            is_submitting: false,
        }
    }

    pub fn set_limits(&mut self, expire: Option<i64>, usage: Option<u32>, approval: bool) {
        self.expire_date_sec = expire;
        self.usage_limit = usage;
        self.request_approval = approval;
    }
}
