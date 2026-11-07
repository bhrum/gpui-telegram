//! Community requestable join chats list view.
//!
//! Upstream:
//! - `Telegram/SourceFiles/dialogs/dialogs_community_requestable_list.h:L1-L45`
//! - `Telegram/SourceFiles/dialogs/dialogs_community_requestable_list.cpp:L1-L159`
//!
//! Subsystem: dialogs, Feature: TCG-N11
//!
//! Responsibilities:
//! - Present chats where join requests can be submitted or approved
//! - Custom status displaying member vs subscriber count (`MakeCommunityChatRow` L25-38)

pub use gpui_telegram_client::dialogs::community::CommunityChatEntry;
use serde::{Deserialize, Serialize};

/// Community requestable join chats list view model.
/// Upstream: `Dialogs::CommunityRequestableList` (`dialogs_community_requestable_list.h:L24-42`)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialogsCommunityRequestableList {
    pub community_id: i64,
    pub pending_requests_count: usize,
    pub requestable_chats: Vec<CommunityChatEntry>,
}

impl DialogsCommunityRequestableList {
    pub fn new(community_id: i64) -> Self {
        Self {
            community_id,
            pending_requests_count: 0,
            requestable_chats: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.requestable_chats.len()
    }

    pub fn is_empty(&self) -> bool {
        self.requestable_chats.is_empty()
    }

    pub fn set_chats(&mut self, chats: Vec<CommunityChatEntry>) {
        self.pending_requests_count = chats.len();
        self.requestable_chats = chats;
    }
}
