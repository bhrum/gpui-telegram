//! Chat search empty / no-results view.
//!
//! Upstream: `Telegram/SourceFiles/dialogs/ui/chat_search_empty.cpp:L1-83`
//! Subsystem: dialogs, Feature: TCG-N03

use serde::{Deserialize, Serialize};

pub const K_QUERY_PREVIEW_LIMIT: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchEmptyIcon {
    Search,
    NoResults,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatSearchEmptyUi {
    pub icon: SearchEmptyIcon,
    pub title: String,
    pub query_preview: String,
    pub show_try_all_chats_link: bool,
}

impl Default for ChatSearchEmptyUi {
    fn default() -> Self {
        Self {
            icon: SearchEmptyIcon::Search,
            title: "Search for messages".to_string(),
            query_preview: String::new(),
            show_try_all_chats_link: false,
        }
    }
}

impl ChatSearchEmptyUi {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn for_no_results(query: &str, suggest_all_chats: bool) -> Self {
        let trimmed = query.trim();
        let preview = if trimmed.len() > K_QUERY_PREVIEW_LIMIT {
            format!("{}...", &trimmed[..K_QUERY_PREVIEW_LIMIT])
        } else {
            trimmed.to_string()
        };

        Self {
            icon: SearchEmptyIcon::NoResults,
            title: "No results".to_string(),
            query_preview: preview,
            show_try_all_chats_link: suggest_all_chats,
        }
    }
}
