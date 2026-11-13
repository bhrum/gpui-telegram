//! Tag Details and Tagged Messages Filter Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/tag_info_box.h:L1-L27`
//! - `Telegram/SourceFiles/boxes/peers/tag_info_box.cpp:L1-L531`
//! - `tag_info_box.cpp:L40-L150` (Tag details and message count)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

/// Target symbol in ledger: `TagInfoBoxPeers`.
#[derive(Debug, Clone)]
pub struct TagInfoBoxPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub tag: String,
    pub emoji_id: Option<u64>,
    pub message_count: usize,
}

impl Default for TagInfoBoxPeers {
    fn default() -> Self {
        Self::new(String::new())
    }
}

impl TagInfoBoxPeers {
    pub fn new(tag: String) -> Self {
        Self {
            is_visible: true,
            width: px(360.0),
            height: px(320.0),
            tag,
            emoji_id: None,
            message_count: 0,
        }
    }

    pub fn set_count(&mut self, count: usize) {
        self.message_count = count;
    }
}
