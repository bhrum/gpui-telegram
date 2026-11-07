//! Stories content container model.
//!
//! Upstream: `Telegram/SourceFiles/dialogs/ui/dialogs_stories_content.h:L1-38`
//! Subsystem: dialogs, Feature: TCG-N03

use super::dialogs_stories_list::StoryElement;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialogsStoriesContentUi {
    pub elements: Vec<StoryElement>,
    pub total: usize,
    pub unread_total: usize,
}

impl DialogsStoriesContentUi {
    pub fn new(elements: Vec<StoryElement>) -> Self {
        let total = elements.len();
        let unread_total = elements.iter().map(|e| e.unread_count as usize).sum();
        Self {
            elements,
            total,
            unread_total,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}
