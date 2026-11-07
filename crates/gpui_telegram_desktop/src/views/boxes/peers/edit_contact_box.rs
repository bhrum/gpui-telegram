//! Edit Existing Contact Profile Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/edit_contact_box.h:L1-L60`
//! - `Telegram/SourceFiles/boxes/peers/edit_contact_box.cpp:L1-L969`
//! - `edit_contact_box.cpp:L50-L150` (First/last name editing & deletion prompt)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

/// Target symbol in ledger: `EditContactBoxPeers`.
#[derive(Debug, Clone)]
pub struct EditContactBoxPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub user_id: u64,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub is_submitting: bool,
}

impl Default for EditContactBoxPeers {
    fn default() -> Self {
        Self::new(0)
    }
}

impl EditContactBoxPeers {
    pub fn new(user_id: u64) -> Self {
        Self {
            is_visible: true,
            width: px(360.0),
            height: px(340.0),
            user_id,
            first_name: String::new(),
            last_name: String::new(),
            phone: String::new(),
            is_submitting: false,
        }
    }

    pub fn set_names(&mut self, first: &str, last: &str) {
        self.first_name = first.trim().to_string();
        self.last_name = last.trim().to_string();
    }

    pub fn can_save(&self) -> bool {
        !self.first_name.is_empty() && !self.is_submitting
    }
}
