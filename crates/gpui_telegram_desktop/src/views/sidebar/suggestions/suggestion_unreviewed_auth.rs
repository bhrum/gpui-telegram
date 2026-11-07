//! Unreviewed authorization warning suggestion.
//!
//! Upstream: `Telegram/SourceFiles/dialogs/suggestions/suggestion_unreviewed_auth.cpp:L25-144`
//! Subsystem: dialogs, Feature: TCG-N03

use super::suggestion_manager::{SuggestionItem, SuggestionPriority, UnreviewedAuthDetails};

#[derive(Debug, Clone, Default)]
pub struct SuggestionUnreviewedAuthSuggestions {
    pub auth_details: Option<UnreviewedAuthDetails>,
}

impl SuggestionUnreviewedAuthSuggestions {
    pub const KEY: &'static str = "UNREVIEWED_AUTH";

    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_item(&self) -> SuggestionItem {
        let desc = if let Some(ref auth) = self.auth_details {
            format!("New login from {}. Confirm if this was you.", auth.format_auth_info())
        } else {
            "New unconfirmed login session detected.".to_string()
        };

        SuggestionItem {
            priority: SuggestionPriority::UnreviewedAuth,
            key: Self::KEY.to_string(),
            title: "Security Notice".to_string(),
            description: desc,
            button_text: Some("Review".to_string()),
            action_url: Some("internal:settings:sessions".to_string()),
            can_close: true,
        }
    }
}
