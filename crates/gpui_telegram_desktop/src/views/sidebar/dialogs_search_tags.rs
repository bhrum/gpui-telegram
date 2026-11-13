//! Search reaction tag pills beneath search header.
//!
//! Based on upstream Telegram Desktop:
//! - `Telegram/SourceFiles/dialogs/dialogs_search_tags.h:L26-92`
//! - `Telegram/SourceFiles/dialogs/dialogs_search_tags.cpp:L33-100`
//!
//! Subsystem: dialogs, Feature: TCG-N03

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchTagItem {
    pub tag_id: String,
    pub title: String,
    pub count: u32,
    pub is_selected: bool,
    pub is_premium: bool,
}

#[derive(Debug, Clone, Default)]
pub struct DialogsSearchTags {
    pub tags: Vec<SearchTagItem>,
    pub selected_tags: Vec<String>,
}

impl DialogsSearchTags {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn toggle_tag(&mut self, tag_id: &str) {
        if let Some(pos) = self.selected_tags.iter().position(|t| t == tag_id) {
            self.selected_tags.remove(pos);
        } else {
            self.selected_tags.push(tag_id.to_string());
        }

        for tag in &mut self.tags {
            if tag.tag_id == tag_id {
                tag.is_selected = !tag.is_selected;
            }
        }
    }

    pub fn clear_selection(&mut self) {
        self.selected_tags.clear();
        for tag in &mut self.tags {
            tag.is_selected = false;
        }
    }
}
