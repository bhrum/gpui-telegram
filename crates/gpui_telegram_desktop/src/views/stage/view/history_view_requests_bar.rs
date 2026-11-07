//! History View Requests Bar View Component (`history_view_requests_bar`)
//!
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_requests_bar.h` (L1-L25)
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_requests_bar.cpp` (L1-L209)

/// Single pending join request item
#[derive(Debug, Clone, PartialEq)]
pub struct PendingJoinUser {
    pub user_id: i64,
    pub name: String,
    pub date: i64,
}

/// GPUI view component for join requests bar
#[derive(Debug, Clone)]
pub struct HistoryViewRequestsBarView {
    pub peer_id: i64,
    pub is_group: bool, // !peer->isBroadcast()
    pub pending_users: Vec<PendingJoinUser>,
    pub is_visible: bool,
}

impl HistoryViewRequestsBarView {
    pub fn new(peer_id: i64, is_group: bool) -> Self {
        Self {
            peer_id,
            is_group,
            pending_users: Vec::new(),
            is_visible: false,
        }
    }

    pub fn set_pending_users(&mut self, users: Vec<PendingJoinUser>) {
        self.is_visible = !users.is_empty();
        self.pending_users = users;
    }

    pub fn count(&self) -> usize {
        self.pending_users.len()
    }

    pub fn dismiss_request(&mut self, user_id: i64) -> Option<PendingJoinUser> {
        if let Some(idx) = self.pending_users.iter().position(|u| u.user_id == user_id) {
            let u = self.pending_users.remove(idx);
            if self.pending_users.is_empty() {
                self.is_visible = false;
            }
            Some(u)
        } else {
            None
        }
    }
}
