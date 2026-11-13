//! Top-Bar Suggestions facade and spec registry.
//!
//! Upstream: `Telegram/SourceFiles/dialogs/suggestions/suggestion.h:L20-76`
//! Subsystem: dialogs, Feature: TCG-N03

use super::suggestion_manager::{SuggestionItem, SuggestionManager};

/// Suggestion spec and collection facade.
#[derive(Debug, Clone, Default)]
pub struct SuggestionSuggestions {
    pub manager: SuggestionManager,
}

impl SuggestionSuggestions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn current(&self) -> Option<&SuggestionItem> {
        self.manager.current_suggestion()
    }

    pub fn dismiss(&mut self, key: &str) {
        self.manager.dismiss(key);
    }
}
