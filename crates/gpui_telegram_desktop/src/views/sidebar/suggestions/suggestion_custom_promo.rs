//! Custom promo suggestion.
//!
//! Upstream: `Telegram/SourceFiles/dialogs/suggestions/suggestion_custom_promo.cpp:L1-66`
//! Subsystem: dialogs, Feature: TCG-N03

use super::suggestion_manager::{SuggestionItem, SuggestionPriority};

#[derive(Debug, Clone, Default)]
pub struct SuggestionCustomPromoSuggestions {
    pub promo_id: String,
    pub title: String,
    pub description: String,
    pub button_text: Option<String>,
    pub url: Option<String>,
}

impl SuggestionCustomPromoSuggestions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_item(&self) -> SuggestionItem {
        SuggestionItem {
            priority: SuggestionPriority::CustomPromo,
            key: format!("PROMO_{}", self.promo_id),
            title: self.title.clone(),
            description: self.description.clone(),
            button_text: self.button_text.clone(),
            action_url: self.url.clone(),
            can_close: true,
        }
    }
}
