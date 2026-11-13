//! Choose Peer for Bot WebApp RequestPeer Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/choose_peer_box.h:L1-L38`
//! - `Telegram/SourceFiles/boxes/peers/choose_peer_box.cpp:L1-L582`
//! - `choose_peer_box.cpp:L40-L150` (RequestPeer query filter options)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

/// Target peer type filter for bot RequestPeer query.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChoosePeerType {
    Users,
    Chats,
    Channels,
}

/// Target symbol in ledger: `ChoosePeerBoxPeers`.
#[derive(Debug, Clone)]
pub struct ChoosePeerBoxPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub query_id: i32,
    pub peer_type: ChoosePeerType,
    pub require_admin: bool,
    pub selected_peer_id: Option<u64>,
}

impl Default for ChoosePeerBoxPeers {
    fn default() -> Self {
        Self::new(0, ChoosePeerType::Users)
    }
}

impl ChoosePeerBoxPeers {
    pub fn new(query_id: i32, peer_type: ChoosePeerType) -> Self {
        Self {
            is_visible: true,
            width: px(380.0),
            height: px(460.0),
            query_id,
            peer_type,
            require_admin: false,
            selected_peer_id: None,
        }
    }

    pub fn select_peer(&mut self, peer_id: u64) {
        self.selected_peer_id = Some(peer_id);
    }
}
