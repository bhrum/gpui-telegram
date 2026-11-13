//! History View Top Peers Selector View Component (`history_view_top_peers_selector`)
//!
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_top_peers_selector.h` (L1-L25)
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_top_peers_selector.cpp` (L1-L263)

/// Maximum peers limit from `history_view_top_peers_selector.cpp:L39`
pub const K_MAX_TOP_PEERS: usize = 5;
pub const K_TOP_PEER_SHIFT: f32 = 0.15;

/// Top peer candidate representation
#[derive(Debug, Clone, PartialEq)]
pub struct TopPeerCandidate {
    pub peer_id: i64,
    pub name: String,
    pub is_user: bool,
    pub is_bot: bool,
    pub is_self: bool,
}

/// GPUI view component for quick contact suggestions row
#[derive(Debug, Clone)]
pub struct HistoryViewTopPeersSelectorView {
    pub self_user_id: i64,
    pub candidates: Vec<TopPeerCandidate>,
    pub selected_peer_id: Option<i64>,
}

impl HistoryViewTopPeersSelectorView {
    pub fn new(self_user_id: i64) -> Self {
        Self {
            self_user_id,
            candidates: Vec::new(),
            selected_peer_id: None,
        }
    }

    /// Upstream filter: adds self first, then non-bot accessible users up to `K_MAX_TOP_PEERS`
    pub fn collect_peers(&mut self, peers: &[TopPeerCandidate]) {
        let mut result = Vec::with_capacity(K_MAX_TOP_PEERS);
        // Add self first
        result.push(TopPeerCandidate {
            peer_id: self.self_user_id,
            name: "Saved Messages".to_string(),
            is_user: true,
            is_bot: false,
            is_self: true,
        });

        for p in peers {
            if result.len() >= K_MAX_TOP_PEERS {
                break;
            }
            if p.peer_id != self.self_user_id && p.is_user && !p.is_bot {
                if !result.iter().any(|existing| existing.peer_id == p.peer_id) {
                    result.push(p.clone());
                }
            }
        }
        self.candidates = result;
    }

    pub fn max_peers_limit(&self) -> usize {
        K_MAX_TOP_PEERS
    }

    pub fn peer_shift_factor(&self) -> f32 {
        K_TOP_PEER_SHIFT
    }
}
