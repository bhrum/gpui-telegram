//! Edit Peer Allowed Message Reactions Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/edit_peer_reactions.h:L1-L42`
//! - `Telegram/SourceFiles/boxes/peers/edit_peer_reactions.cpp:L1-L1037`
//! - `edit_peer_reactions.cpp:L50-L150` (Reactions modes: All, Some, None)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};
use std::collections::HashSet;

/// Reactions enablement mode for chat.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReactionsMode {
    All,
    Some,
    None,
}

/// Target symbol in ledger: `EditPeerReactionsPeers`.
#[derive(Debug, Clone)]
pub struct EditPeerReactionsPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub peer_id: u64,
    pub mode: ReactionsMode,
    pub allowed_emojis: HashSet<String>,
    pub allow_custom_emojis: bool,
    pub max_reactions_per_message: u32,
}

impl Default for EditPeerReactionsPeers {
    fn default() -> Self {
        Self::new(0)
    }
}

impl EditPeerReactionsPeers {
    pub fn new(peer_id: u64) -> Self {
        Self {
            is_visible: true,
            width: px(400.0),
            height: px(520.0),
            peer_id,
            mode: ReactionsMode::All,
            allowed_emojis: HashSet::new(),
            allow_custom_emojis: false,
            max_reactions_per_message: 11,
        }
    }

    pub fn set_mode(&mut self, mode: ReactionsMode) {
        self.mode = mode;
    }

    pub fn toggle_emoji(&mut self, emoji: &str) {
        if self.allowed_emojis.contains(emoji) {
            self.allowed_emojis.remove(emoji);
        } else {
            self.allowed_emojis.insert(emoji.to_string());
        }
    }
}
