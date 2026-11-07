//! Content layout for top bar suggestion banner.
//!
//! Upstream: `Telegram/SourceFiles/dialogs/ui/dialogs_top_bar_suggestion_content.h:L1-176`
//! Subsystem: dialogs, Feature: TCG-N03

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SuggestionRightIcon {
    None,
    Close,
    Arrow,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialogsTopBarSuggestionContentUi {
    pub title: String,
    pub description: String,
    pub right_icon: Option<SuggestionRightIcon>,
    pub button_text: Option<String>,
}

impl DialogsTopBarSuggestionContentUi {
    pub fn new(title: String, description: String) -> Self {
        Self {
            title,
            description,
            right_icon: Some(SuggestionRightIcon::Close),
            button_text: None,
        }
    }
}
