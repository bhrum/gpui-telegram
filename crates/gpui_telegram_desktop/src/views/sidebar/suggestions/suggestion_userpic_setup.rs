//! Set profile picture prompt suggestion.
//!
//! Upstream: `Telegram/SourceFiles/dialogs/suggestions/suggestion_userpic_setup.cpp:L20-102`
//! Subsystem: dialogs, Feature: TCG-N03

use super::suggestion_manager::{SuggestionItem, SuggestionPriority};

#[derive(Debug, Clone, Default)]
pub struct SuggestionUserpicSetupSuggestions {
    pub has_userpic: bool,
}

impl SuggestionUserpicSetupSuggestions {
    pub const KEY: &'static str = "USERPIC_SETUP";

    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_item(&self) -> SuggestionItem {
        SuggestionItem {
            priority: SuggestionPriority::UserpicSetup,
            key: Self::KEY.to_string(),
            title: "Add a Profile Photo".to_string(),
            description: "Upload a photo or choose an animated avatar to personalize your account.".to_string(),
            button_text: Some("Set Photo".to_string()),
            action_url: Some("internal:edit_userpic".to_string()),
            can_close: true,
        }
    }
}
