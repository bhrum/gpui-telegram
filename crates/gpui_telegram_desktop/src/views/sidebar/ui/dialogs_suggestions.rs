//! Top bar suggestion banner view component.
//!
//! Based on upstream Telegram Desktop:
//! - `Telegram/SourceFiles/dialogs/ui/dialogs_suggestions.h:L1-395`
//! - `Telegram/SourceFiles/dialogs/ui/dialogs_suggestions.cpp:L1-200`
//!
//! Subsystem: dialogs, Feature: TCG-N03

use crate::views::sidebar::suggestions::suggestion_manager::{SuggestionItem, SuggestionManager};

pub const K_COLLAPSED_CHANNELS_COUNT: usize = 3;
pub const K_SEARCH_QUERY_DELAY_MS: u64 = 400;
pub const K_SEARCH_PER_PAGE: usize = 50;

/// Banner view state for active top bar suggestions.
#[derive(Debug, Clone, Default)]
pub struct DialogsSuggestionsUi {
    pub current_item: Option<SuggestionItem>,
    pub is_visible: bool,
}

impl DialogsSuggestionsUi {
    pub fn new() -> Self {
        Self {
            current_item: None,
            is_visible: false,
        }
    }

    pub fn update_from_manager(&mut self, manager: &SuggestionManager) {
        if let Some(item) = manager.current_suggestion() {
            self.current_item = Some(item.clone());
            self.is_visible = true;
        } else {
            self.current_item = None;
            self.is_visible = false;
        }
    }

    pub fn dismiss_current(&mut self, manager: &mut SuggestionManager) {
        if let Some(ref item) = self.current_item {
            manager.dismiss(&item.key);
            self.update_from_manager(manager);
        }
    }
}
