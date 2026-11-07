//! Info Profile Free Codes component
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_free_codes.cpp:L1-L59`
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_free_codes.h:L1-L37`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FreeCodeEntry {
    pub code: String,
    pub expires_at: i64,
    pub is_used: bool,
}

/// Manages giveaway activation codes and temporary promotional gift links.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoProfileFreeCodesProfile {
    pub codes: Vec<FreeCodeEntry>,
    pub is_loading: bool,
}

impl InfoProfileFreeCodesProfile {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_code(&mut self, code: impl Into<String>, expires_at: i64) {
        self.codes.push(FreeCodeEntry {
            code: code.into(),
            expires_at,
            is_used: false,
        });
    }

    pub fn mark_used(&mut self, code_str: &str) {
        if let Some(entry) = self.codes.iter_mut().find(|c| c.code == code_str) {
            entry.is_used = true;
        }
    }
}
