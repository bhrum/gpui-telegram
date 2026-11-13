//! Create Managed Bot Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/create_managed_bot_box.h:L1-L35`
//! - `Telegram/SourceFiles/boxes/peers/create_managed_bot_box.cpp:L1-L522`
//! - `create_managed_bot_box.cpp:L40-L150` (Bot creation flow)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

/// Dialog box for initializing and attaching a managed business or agent bot.
/// Target symbol in ledger: `CreateManagedBotBoxPeers`.
#[derive(Debug, Clone)]
pub struct CreateManagedBotBoxPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub bot_name: String,
    pub bot_username: String,
    pub bot_token: String,
    pub is_business_bot: bool,
    pub is_submitting: bool,
}

impl Default for CreateManagedBotBoxPeers {
    fn default() -> Self {
        Self::new()
    }
}

impl CreateManagedBotBoxPeers {
    pub fn new() -> Self {
        Self {
            is_visible: true,
            width: px(380.0),
            height: px(400.0),
            bot_name: String::new(),
            bot_username: String::new(),
            bot_token: String::new(),
            is_business_bot: true,
            is_submitting: false,
        }
    }

    pub fn set_credentials(&mut self, name: &str, username: &str, token: &str) {
        self.bot_name = name.trim().to_string();
        self.bot_username = username.trim().to_string();
        self.bot_token = token.trim().to_string();
    }

    pub fn is_valid(&self) -> bool {
        !self.bot_name.is_empty()
            && self.bot_username.ends_with("bot")
            && self.bot_username.len() >= 5
    }
}
