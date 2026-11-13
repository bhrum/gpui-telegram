//! Community member chats list view.
//!
//! Upstream:
//! - `Telegram/SourceFiles/dialogs/dialogs_community_chats_list.h:L1-L78`
//! - `Telegram/SourceFiles/dialogs/dialogs_community_chats_list.cpp:L1-L264`
//!
//! Subsystem: dialogs, Feature: TCG-N11
//!
//! Key Behavior:
//! - Descending sort for joined chats by `(sortKeyInChatList, chatListTimeId)` (L117-122)
//! - Style selection mapping to `communityInfo*` variants (L26-37)

pub use gpui_telegram_client::dialogs::community::{
    CommunityChatEntry, CommunityChatsKind, CommunityChatsList as ClientCommunityChatsList,
    CommunityRowsView,
};
use serde::{Deserialize, Serialize};

/// Community chats list widget view model for the sidebar.
/// Upstream: `Dialogs::CommunityChatsList` (`dialogs_community_chats_list.h:L37-75`)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialogsCommunityChatsList {
    pub community_id: i64,
    pub kind: CommunityChatsKind,
    pub items: Vec<CommunityChatEntry>,
}

impl DialogsCommunityChatsList {
    pub fn new(community_id: i64, kind: CommunityChatsKind) -> Self {
        Self {
            community_id,
            kind,
            items: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn rebuild(&mut self, mut entries: Vec<CommunityChatEntry>) {
        if self.kind == CommunityChatsKind::Joined {
            entries.retain(|e| e.is_joined);
            entries.sort_by(|a, b| {
                match b.sort_key.cmp(&a.sort_key) {
                    std::cmp::Ordering::Equal => b.time_id.cmp(&a.time_id),
                    ord => ord,
                }
            });
        } else {
            entries.retain(|e| !e.is_joined);
        }
        self.items = entries;
    }
}
