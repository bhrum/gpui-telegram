//! Chat Join Requests Moderation Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/edit_peer_requests_box.h:L1-L149`
//! - `Telegram/SourceFiles/boxes/peers/edit_peer_requests_box.cpp:L1-L768`
//! - `edit_peer_requests_box.cpp:L50-L150` (Join request review and batch approval)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

/// Single pending request item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JoinRequestEntry {
    pub user_id: u64,
    pub name: String,
    pub requested_at_sec: i64,
    pub bio: Option<String>,
}

/// Target symbol in ledger: `EditPeerRequestsBoxPeers`.
#[derive(Debug, Clone)]
pub struct EditPeerRequestsBoxPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub peer_id: u64,
    pub requests: Vec<JoinRequestEntry>,
}

impl Default for EditPeerRequestsBoxPeers {
    fn default() -> Self {
        Self::new(0)
    }
}

impl EditPeerRequestsBoxPeers {
    pub fn new(peer_id: u64) -> Self {
        Self {
            is_visible: true,
            width: px(400.0),
            height: px(500.0),
            peer_id,
            requests: Vec::new(),
        }
    }

    pub fn approve(&mut self, user_id: u64) {
        self.requests.retain(|r| r.user_id != user_id);
    }

    pub fn dismiss(&mut self, user_id: u64) {
        self.requests.retain(|r| r.user_id != user_id);
    }

    pub fn approve_all(&mut self) {
        self.requests.clear();
    }
}
