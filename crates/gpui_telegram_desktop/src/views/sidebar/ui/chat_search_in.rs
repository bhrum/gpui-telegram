//! Chat search scope picker ('in this chat', 'in channel', 'all chats').
//!
//! Upstream: `Telegram/SourceFiles/dialogs/ui/chat_search_in.cpp:L1-477`
//! Subsystem: dialogs, Feature: TCG-N03

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatSearchTab {
    ThisChat,
    MyMessages,
    PublicPosts,
    Media,
    Links,
    Files,
    Music,
    Voice,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatSearchInUi {
    pub active_tab: ChatSearchTab,
    pub peer_id: Option<u64>,
    pub peer_name: Option<String>,
}

impl Default for ChatSearchInUi {
    fn default() -> Self {
        Self {
            active_tab: ChatSearchTab::ThisChat,
            peer_id: None,
            peer_name: None,
        }
    }
}

impl ChatSearchInUi {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_scope(&mut self, peer_id: u64, peer_name: String) {
        self.peer_id = Some(peer_id);
        self.peer_name = Some(peer_name);
    }

    pub fn clear_scope(&mut self) {
        self.peer_id = None;
        self.peer_name = None;
        self.active_tab = ChatSearchTab::ThisChat;
    }
}
