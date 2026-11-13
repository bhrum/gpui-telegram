//! Public posts global search intro message view.
//!
//! Upstream: `Telegram/SourceFiles/dialogs/ui/posts_search_intro.cpp:L1-252`
//! Subsystem: dialogs, Feature: TCG-N03

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostsSearchIntroUi {
    pub title: String,
    pub description: String,
}

impl PostsSearchIntroUi {
    pub fn new() -> Self {
        Self {
            title: "Public Channel Posts".to_string(),
            description: "Find messages and posts across public Telegram channels.".to_string(),
        }
    }
}
