//! Add Bot to Chat or Channel Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/add_bot_to_chat_box.h:L1-L85`
//! - `Telegram/SourceFiles/boxes/peers/add_bot_to_chat_box.cpp:L1-L424`
//! - `Telegram/SourceFiles/boxes/peers/add_bot_to_chat_box.cpp:L25-L100` (Bot rights assignment)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

/// Target symbol in ledger: `AddBotToChatBoxPeers`.
#[derive(Debug, Clone)]
pub struct AddBotToChatBoxPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub bot_id: u64,
    pub bot_name: String,
    pub target_chat_id: Option<u64>,
    pub grant_admin_rights: bool,
    pub is_submitting: bool,
}

impl Default for AddBotToChatBoxPeers {
    fn default() -> Self {
        Self::new(0, "Bot".to_string())
    }
}

impl AddBotToChatBoxPeers {
    pub fn new(bot_id: u64, bot_name: String) -> Self {
        Self {
            is_visible: true,
            width: px(380.0),
            height: px(440.0),
            bot_id,
            bot_name,
            target_chat_id: None,
            grant_admin_rights: false,
            is_submitting: false,
        }
    }

    pub fn select_chat(&mut self, chat_id: u64) {
        self.target_chat_id = Some(chat_id);
    }

    pub fn set_grant_admin(&mut self, grant: bool) {
        self.grant_admin_rights = grant;
    }

    pub fn can_submit(&self) -> bool {
        self.target_chat_id.is_some() && !self.is_submitting
    }
}
