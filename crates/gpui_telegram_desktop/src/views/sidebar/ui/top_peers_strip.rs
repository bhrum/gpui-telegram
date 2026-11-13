//! Top frequently contacted peers carousel strip.
//!
//! Based on upstream Telegram Desktop:
//! - `Telegram/SourceFiles/dialogs/ui/top_peers_strip.h:L24-60`
//! - `Telegram/SourceFiles/dialogs/ui/top_peers_strip.cpp:L30-95`
//!
//! Subsystem: dialogs, Feature: TCG-N03

use serde::{Deserialize, Serialize};

/// Peer entry in the top frequent contacts strip (`top_peers_strip.h:L24-32`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopPeerItem {
    pub id: u64,
    pub name: String,
    pub avatar_initials: String,
    pub avatar_color: u32,
    pub badge: u32,
    pub is_unread: bool,
    pub is_muted: bool,
    pub is_online: bool,
}

/// Top peers strip component coordinator.
#[derive(Debug, Clone, Default)]
pub struct TopPeersStripUi {
    pub entries: Vec<TopPeerItem>,
    pub is_expanded: bool,
    pub selected_peer_id: Option<u64>,
}

impl TopPeersStripUi {
    pub fn new() -> Self {
        Self::default()
    }

    /// Evaluates whether the number of entries exceeds single-line capacity (`top_peers_strip.cpp:L91`).
    pub fn can_expand(count: usize, item_width: f32, available_width: f32) -> bool {
        (count as f32 * item_width) > (available_width + (item_width * 2.0) / 3.0)
    }

    pub fn set_entries(&mut self, list: Vec<TopPeerItem>) {
        self.entries = list;
    }

    pub fn toggle_expanded(&mut self) {
        self.is_expanded = !self.is_expanded;
    }

    pub fn remove_peer(&mut self, peer_id: u64) {
        self.entries.retain(|e| e.id != peer_id);
    }
}
