//! Public channel posts search controller.
//!
//! Upstream: `Telegram/SourceFiles/dialogs/dialogs_search_posts.h:L1-80`
//! Subsystem: dialogs, Feature: TCG-N03

use serde::{Deserialize, Serialize};

pub const K_QUERY_DELAY_MS: u64 = 400;
pub const K_POSTS_PER_PAGE: usize = 20;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelPostMatch {
    pub channel_id: u64,
    pub channel_title: String,
    pub message_id: u64,
    pub snippet: String,
    pub date: String,
}

#[derive(Debug, Clone, Default)]
pub struct DialogsSearchPosts {
    pub posts: Vec<ChannelPostMatch>,
    pub is_loading: bool,
    pub has_more: bool,
}

impl DialogsSearchPosts {
    pub fn new() -> Self {
        Self::default()
    }
}
