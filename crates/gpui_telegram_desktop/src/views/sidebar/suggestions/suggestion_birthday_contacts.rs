//! Birthday contacts reminder suggestion.
//!
//! Upstream: `Telegram/SourceFiles/dialogs/suggestions/suggestion_birthday_contacts.cpp:L1-151`
//! Subsystem: dialogs, Feature: TCG-N03

use super::suggestion_manager::{SuggestionItem, SuggestionPriority};

#[derive(Debug, Clone, Default)]
pub struct SuggestionBirthdayContactsSuggestions {
    pub contact_names: Vec<String>,
}

impl SuggestionBirthdayContactsSuggestions {
    pub const KEY: &'static str = "BIRTHDAY_CONTACTS";

    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_item(&self) -> SuggestionItem {
        let desc = if self.contact_names.is_empty() {
            "Some of your contacts are celebrating their birthday today.".to_string()
        } else {
            format!("Wish {} a happy birthday!", self.contact_names.join(", "))
        };

        SuggestionItem {
            priority: SuggestionPriority::BirthdayContacts,
            key: Self::KEY.to_string(),
            title: "Today's Birthdays".to_string(),
            description: desc,
            button_text: Some("Send Wishes".to_string()),
            action_url: Some("internal:birthdays".to_string()),
            can_close: true,
        }
    }
}
