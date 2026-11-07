//! Forum topics row layout and expansion view.
//!
//! Upstream: `Telegram/SourceFiles/dialogs/ui/dialogs_topics_view.cpp:L1-442`
//! Subsystem: dialogs, Feature: TCG-N03

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForumTopicEntry {
    pub topic_id: u64,
    pub title: String,
    pub icon_emoji: String,
    pub unread_count: u32,
    pub is_closed: bool,
    pub is_hidden: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialogsTopicsViewUi {
    pub forum_id: u64,
    pub topics: Vec<ForumTopicEntry>,
    pub is_expanded: bool,
}

impl DialogsTopicsViewUi {
    pub const TOPICS_ROW_HEIGHT: u32 = 80;

    pub fn new(forum_id: u64) -> Self {
        Self {
            forum_id,
            topics: Vec::new(),
            is_expanded: false,
        }
    }
}
