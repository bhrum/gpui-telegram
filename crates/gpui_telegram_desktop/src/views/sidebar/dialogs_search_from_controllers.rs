//! Sender filter controller for group message search (`from:user`).
//!
//! Upstream: `Telegram/SourceFiles/dialogs/dialogs_search_from_controllers.h:L15-35`
//! Subsystem: dialogs, Feature: TCG-N03

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchFromPeer {
    pub peer_id: u64,
    pub name: String,
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DialogsSearchFromControllers {
    pub selected_peer: Option<SearchFromPeer>,
    pub candidate_peers: Vec<SearchFromPeer>,
}

impl DialogsSearchFromControllers {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_from_peer(&mut self, peer: SearchFromPeer) {
        self.selected_peer = Some(peer);
    }

    pub fn clear(&mut self) {
        self.selected_peer = None;
    }
}
