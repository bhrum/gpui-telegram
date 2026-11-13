//! Birthday setup prompt suggestion.
//!
//! Upstream: `Telegram/SourceFiles/dialogs/suggestions/suggestion_birthday_setup.cpp:L20-86`
//! Subsystem: dialogs, Feature: TCG-N03

use super::suggestion_manager::{SuggestionItem, SuggestionPriority};

#[derive(Debug, Clone, Default)]
pub struct SuggestionBirthdaySetupSuggestions {
    pub is_set: bool,
}

impl SuggestionBirthdaySetupSuggestions {
    pub const KEY: &'static str = "BIRTHDAY_SETUP";

    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_item(&self) -> SuggestionItem {
        SuggestionItem {
            priority: SuggestionPriority::BirthdaySetup,
            key: Self::KEY.to_string(),
            title: "Add Your Birthday".to_string(),
            description: "Receive birthday wishes and gifts from friends on Telegram.".to_string(),
            button_text: Some("Add".to_string()),
            action_url: Some("internal:edit_birthday:add_privacy".to_string()),
            can_close: true,
        }
    }
}
