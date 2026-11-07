//! Top-bar suggestion presenter controller.
//!
//! Upstream: `Telegram/SourceFiles/dialogs/dialogs_top_bar_suggestion.h:L1-37`
//! Subsystem: dialogs, Feature: TCG-N03

use super::suggestions::suggestion_manager::SuggestionManager;

#[derive(Debug, Clone, Default)]
pub struct DialogsTopBarSuggestion {
    pub is_active: bool,
}

impl DialogsTopBarSuggestion {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn should_display(&self, manager: &SuggestionManager) -> bool {
        manager.current_suggestion().is_some()
    }
}
