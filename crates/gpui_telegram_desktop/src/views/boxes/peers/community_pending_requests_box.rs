//! Community Pending Join Requests Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/community_pending_requests_box.h:L1-L20`
//! - `Telegram/SourceFiles/boxes/peers/community_pending_requests_box.cpp:L1-L22`
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

/// Target symbol in ledger: `CommunityPendingRequestsBoxPeers`.
#[derive(Debug, Clone)]
pub struct CommunityPendingRequestsBoxPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub community_id: u64,
    pub pending_count: usize,
    pub auto_approve: bool,
}

impl Default for CommunityPendingRequestsBoxPeers {
    fn default() -> Self {
        Self::new(0)
    }
}

impl CommunityPendingRequestsBoxPeers {
    pub fn new(community_id: u64) -> Self {
        Self {
            is_visible: true,
            width: px(380.0),
            height: px(460.0),
            community_id,
            pending_count: 0,
            auto_approve: false,
        }
    }

    pub fn set_pending_count(&mut self, count: usize) {
        self.pending_count = count;
    }
}
