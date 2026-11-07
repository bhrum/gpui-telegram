//! Edit Tag and Reaction Category Control Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/edit_tag_control.h:L1-L50`
//! - `Telegram/SourceFiles/boxes/peers/edit_tag_control.cpp:L1-L417`
//! - `edit_tag_control.cpp:L30-L120` (Tag emoji and title binding)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

/// Target symbol in ledger: `EditTagControlPeers`.
#[derive(Debug, Clone)]
pub struct EditTagControlPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub tag_text: String,
    pub emoji_id: Option<u64>,
}

impl Default for EditTagControlPeers {
    fn default() -> Self {
        Self::new()
    }
}

impl EditTagControlPeers {
    pub fn new() -> Self {
        Self {
            is_visible: true,
            width: px(340.0),
            height: px(260.0),
            tag_text: String::new(),
            emoji_id: None,
        }
    }

    pub fn set_tag(&mut self, text: &str, emoji: Option<u64>) {
        self.tag_text = text.trim().to_string();
        self.emoji_id = emoji;
    }
}
