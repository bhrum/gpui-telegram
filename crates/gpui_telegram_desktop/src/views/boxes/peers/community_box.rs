//! Community Hub and Channel Aggregator Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/community_box.h:L1-L70`
//! - `Telegram/SourceFiles/boxes/peers/community_box.cpp:L1-L825`
//! - `community_box.cpp:L50-L150` (Community management structure)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

/// Target symbol in ledger: `CommunityBoxPeers`.
#[derive(Debug, Clone)]
pub struct CommunityBoxPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub community_id: u64,
    pub name: String,
    pub description: String,
    pub member_chat_ids: Vec<u64>,
    pub is_open_community: bool,
}

impl Default for CommunityBoxPeers {
    fn default() -> Self {
        Self::new(0, "Community".to_string())
    }
}

impl CommunityBoxPeers {
    pub fn new(community_id: u64, name: String) -> Self {
        Self {
            is_visible: true,
            width: px(420.0),
            height: px(500.0),
            community_id,
            name,
            description: String::new(),
            member_chat_ids: Vec::new(),
            is_open_community: true,
        }
    }

    pub fn add_chat(&mut self, chat_id: u64) {
        if !self.member_chat_ids.contains(&chat_id) {
            self.member_chat_ids.push(chat_id);
        }
    }

    pub fn remove_chat(&mut self, chat_id: u64) {
        self.member_chat_ids.retain(|&id| id != chat_id);
    }
}
