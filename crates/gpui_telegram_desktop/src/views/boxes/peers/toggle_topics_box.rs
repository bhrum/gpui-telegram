//! Toggle Forum Topics Mode Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/toggle_topics_box.h:L1-L20`
//! - `Telegram/SourceFiles/boxes/peers/toggle_topics_box.cpp:L1-L228`
//! - `toggle_topics_box.cpp:L30-L100` (Enable/disable forum threads)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

/// Target symbol in ledger: `ToggleTopicsBoxPeers`.
#[derive(Debug, Clone)]
pub struct ToggleTopicsBoxPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub chat_id: u64,
    pub forum_enabled: bool,
}

impl Default for ToggleTopicsBoxPeers {
    fn default() -> Self {
        Self::new(0, false)
    }
}

impl ToggleTopicsBoxPeers {
    pub fn new(chat_id: u64, current_enabled: bool) -> Self {
        Self {
            is_visible: true,
            width: px(380.0),
            height: px(340.0),
            chat_id,
            forum_enabled: current_enabled,
        }
    }

    pub fn toggle(&mut self) {
        self.forum_enabled = !self.forum_enabled;
    }
}
