//! Suggestion Manager: Consolidated priority queue banner engine for Telegram suggestions.
//!
//! Based on upstream Telegram Desktop:
//! - `Telegram/SourceFiles/dialogs/suggestions/suggestion.h:L43-53`
//! - `Telegram/SourceFiles/dialogs/suggestions/suggestion.cpp:L12-25`
//! - `Telegram/SourceFiles/dialogs/suggestions/suggestion_unreviewed_auth.cpp:L28-44`
//! - `Telegram/SourceFiles/dialogs/suggestions/suggestion_premium_grace.cpp:L21-58`
//! - `Telegram/SourceFiles/dialogs/suggestions/suggestion_birthday_setup.cpp:L25-60`
//!
//! Subsystem: dialogs, Feature: TCG-N03

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Suggestion priority hierarchy (1 is highest priority, 9 is lowest).
/// Upstream: `Dialogs::TopBarSuggestions::Priority` (`suggestion.h:L43-53`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum SuggestionPriority {
    UserpicSetup = 1,
    PremiumOffer = 2,
    BirthdaySetup = 3,
    BirthdayContacts = 4,
    LowCreditsSubs = 5,
    PremiumGrace = 6,
    CustomPromo = 7,
    GiftAuctions = 8,
    UnreviewedAuth = 9,
}

/// Dynamic payload data for active suggestions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuggestionItem {
    pub priority: SuggestionPriority,
    pub key: String,
    pub title: String,
    pub description: String,
    pub button_text: Option<String>,
    pub action_url: Option<String>,
    pub can_close: bool,
}

/// Unreviewed login auth details (`suggestion_unreviewed_auth.cpp:L28-44`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnreviewedAuthDetails {
    pub location: String,
    pub device: String,
    pub date: String,
}

impl UnreviewedAuthDetails {
    /// Formats unreviewed auth string matching upstream C++: `🌍 Location (📱 Device)`.
    pub fn format_auth_info(&self) -> String {
        let loc = if self.location.is_empty() {
            String::new()
        } else {
            format!("🌍 {}", self.location)
        };
        let dev = if self.device.is_empty() {
            String::new()
        } else {
            format!("📱 {}", self.device)
        };

        if !loc.is_empty() && !dev.is_empty() {
            format!("{} ({})", loc, dev)
        } else if !loc.is_empty() {
            loc
        } else if !dev.is_empty() {
            dev
        } else {
            String::new()
        }
    }
}

/// Consolidated Suggestion Manager coordinating active banners and user dismissals.
#[derive(Debug, Clone, Default)]
pub struct SuggestionManager {
    dismissed_keys: HashSet<String>,
    active_suggestions: Vec<SuggestionItem>,
}

impl SuggestionManager {
    pub const KEY_PREMIUM_GRACE: &'static str = "PREMIUM_GRACE";
    pub const KEY_BIRTHDAY_SETUP: &'static str = "BIRTHDAY_SETUP";
    pub const KEY_USERPIC_SETUP: &'static str = "USERPIC_SETUP";
    pub const KEY_BIRTHDAY_CONTACTS: &'static str = "BIRTHDAY_CONTACTS";
    pub const KEY_LOW_CREDITS_SUBS: &'static str = "LOW_CREDITS_SUBS";

    pub fn new() -> Self {
        Self::default()
    }

    /// Registers or updates an active suggestion candidate.
    pub fn add_suggestion(&mut self, item: SuggestionItem) {
        if !self.dismissed_keys.contains(&item.key) {
            self.active_suggestions.retain(|s| s.key != item.key);
            self.active_suggestions.push(item);
            // Sort by priority ascending (1 = highest priority)
            self.active_suggestions.sort_by_key(|s| s.priority);
        }
    }

    /// Dismisses a suggestion by key.
    pub fn dismiss(&mut self, key: &str) {
        self.dismissed_keys.insert(key.to_string());
        self.active_suggestions.retain(|s| s.key != key);
    }

    /// Returns the current highest-priority suggestion to display in the banner.
    pub fn current_suggestion(&self) -> Option<&SuggestionItem> {
        self.active_suggestions.first()
    }

    /// Checks if a suggestion key is currently dismissed.
    pub fn is_dismissed(&self, key: &str) -> bool {
        self.dismissed_keys.contains(key)
    }

    /// Total count of pending active suggestions.
    pub fn active_count(&self) -> usize {
        self.active_suggestions.len()
    }
}
