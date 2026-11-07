//! Create and Edit Forum Topic Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/edit_forum_topic_box.h:L1-L40`
//! - `Telegram/SourceFiles/boxes/peers/edit_forum_topic_box.cpp:L1-L608`
//! - `edit_forum_topic_box.cpp:L50-L150` (Topic title, icon emoji ID, close/pin flags)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

pub const K_MAX_FORUM_TOPIC_TITLE: usize = 128;

/// Target symbol in ledger: `EditForumTopicBoxPeers`.
#[derive(Debug, Clone)]
pub struct EditForumTopicBoxPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub chat_id: u64,
    pub topic_id: Option<i32>,
    pub title: String,
    pub icon_emoji_id: Option<u64>,
    pub is_closed: bool,
    pub is_pinned: bool,
    pub is_submitting: bool,
}

impl Default for EditForumTopicBoxPeers {
    fn default() -> Self {
        Self::new(0, None)
    }
}

impl EditForumTopicBoxPeers {
    pub fn new(chat_id: u64, topic_id: Option<i32>) -> Self {
        Self {
            is_visible: true,
            width: px(380.0),
            height: px(380.0),
            chat_id,
            topic_id,
            title: String::new(),
            icon_emoji_id: None,
            is_closed: false,
            is_pinned: false,
            is_submitting: false,
        }
    }

    pub fn set_title(&mut self, text: &str) {
        self.title = text.chars().take(K_MAX_FORUM_TOPIC_TITLE).collect();
    }

    pub fn can_submit(&self) -> bool {
        !self.title.trim().is_empty() && !self.is_submitting
    }
}
