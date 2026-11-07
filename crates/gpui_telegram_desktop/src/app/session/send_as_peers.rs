//! Send-As peer identity coordinator for posting as channels or groups
//! Upstream: Telegram/SourceFiles/main/session/send_as_peers.h, send_as_peers.cpp
//! Subsystem: main | Feature ID: TCG-N02 | Disposition: gpui_ui_1to1

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type of activity being performed on behalf of an alternative peer identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum SendAsType {
    #[default]
    Message,
    PaidReaction,
    VideoStream,
}

/// Lookup key combining target conversation peer and activity type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SendAsKey {
    pub peer_id: u64,
    pub send_type: SendAsType,
}

impl SendAsKey {
    pub fn new(peer_id: u64, send_type: SendAsType) -> Self {
        Self { peer_id, send_type }
    }

    pub fn message(peer_id: u64) -> Self {
        Self::new(peer_id, SendAsType::Message)
    }
}

/// Permitted alternative sender identity (e.g., owned channel or group).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SendAsPeer {
    pub peer_id: u64,
    pub title: String,
    pub premium_required: bool,
}

impl SendAsPeer {
    pub fn new(peer_id: u64, title: String, premium_required: bool) -> Self {
        Self {
            peer_id,
            title,
            premium_required,
        }
    }
}

/// Session controller managing alternative sending identities per peer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SendAsPeersSession {
    pub user_peer_id: u64,
    pub lists: HashMap<SendAsKey, Vec<SendAsPeer>>,
    pub chosen: HashMap<u64, u64>,
}

impl Default for SendAsPeersSession {
    fn default() -> Self {
        Self::new(0)
    }
}

impl SendAsPeersSession {
    /// Constructs a new send-as controller bound to the current user.
    pub fn new(user_peer_id: u64) -> Self {
        Self {
            user_peer_id,
            lists: HashMap::new(),
            chosen: HashMap::new(),
        }
    }

    /// Sets the list of permitted send-as peers for a key.
    pub fn set_list(&mut self, key: SendAsKey, peers: Vec<SendAsPeer>) {
        self.lists.insert(key, peers);
    }

    /// Retrieves the list of available send-as peers for a key.
    pub fn list(&self, key: &SendAsKey) -> &[SendAsPeer] {
        self.lists.get(key).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Returns true if the user has multiple identities to choose from for this key.
    pub fn should_choose(&self, key: &SendAsKey) -> bool {
        self.list(key).len() > 1
    }

    /// Saves the selected sending peer identity for a chat.
    pub fn set_chosen(&mut self, peer_id: u64, chosen_id: u64) {
        self.chosen.insert(peer_id, chosen_id);
    }

    /// Returns the chosen sending peer identity if set.
    pub fn chosen(&self, peer_id: u64) -> Option<u64> {
        self.chosen.get(&peer_id).copied()
    }

    /// Resolves the effective sending peer identity.
    /// If a previously chosen peer is still in the permitted list, returns it;
    /// otherwise falls back to the first available option or the user's own peer ID.
    pub fn resolve_chosen(&self, key: &SendAsKey) -> u64 {
        let available = self.list(key);
        if available.is_empty() {
            return self.user_peer_id;
        }

        if let Some(chosen_id) = self.chosen(key.peer_id) {
            if available.iter().any(|p| p.peer_id == chosen_id) {
                return chosen_id;
            }
        }

        available.first().map(|p| p.peer_id).unwrap_or(self.user_peer_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_as_peers_resolution() {
        let user_id = 1000;
        let mut session = SendAsPeersSession::new(user_id);
        let chat_id = 2000;
        let key = SendAsKey::message(chat_id);

        assert_eq!(session.resolve_chosen(&key), user_id);
        assert!(!session.should_choose(&key));

        let peers = vec![
            SendAsPeer::new(user_id, "Me".into(), false),
            SendAsPeer::new(3000, "My Channel".into(), false),
        ];
        session.set_list(key, peers);
        assert!(session.should_choose(&key));

        session.set_chosen(chat_id, 3000);
        assert_eq!(session.resolve_chosen(&key), 3000);

        // Revoking channel permission falls back
        session.set_list(key, vec![SendAsPeer::new(user_id, "Me".into(), false)]);
        assert_eq!(session.resolve_chosen(&key), user_id);
    }
}
