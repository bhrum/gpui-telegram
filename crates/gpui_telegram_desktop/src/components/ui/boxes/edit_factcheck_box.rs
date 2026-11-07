//! Authentic GPUI Channel Fact-Check Editor Component.
//! Reference: Telegram/SourceFiles/ui/boxes/edit_factcheck_box.cpp:L20-L89
//! Reference: Telegram/SourceFiles/ui/boxes/edit_factcheck_box.h:L15-L21

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FactcheckButtonState {
    Initial,
    Changed,
    Removed,
}

/// Target symbol for message fact-check editor box.
/// Reference: `edit_factcheck_box.h:L15-L21`
#[derive(Debug, Clone)]
pub struct EditFactcheckBoxBoxes {
    pub initial_text: String,
    pub current_text: String,
    pub limit: usize,
    pub state: FactcheckButtonState,
    pub is_saved: bool,
    pub is_removed: bool,
}

impl Default for EditFactcheckBoxBoxes {
    fn default() -> Self {
        Self::new("", 1024)
    }
}

impl EditFactcheckBoxBoxes {
    pub fn new(initial: impl Into<String>, limit: usize) -> Self {
        let text = initial.into();
        Self {
            initial_text: text.clone(),
            current_text: text,
            limit,
            state: FactcheckButtonState::Initial,
            is_saved: false,
            is_removed: false,
        }
    }

    /// Updates current text and re-evaluates 3-state button transitions.
    /// Reference: `edit_factcheck_box.cpp:L38-L55`
    pub fn update_text(&mut self, text: &str) {
        self.current_text = text.to_string();
        let trimmed = text.trim();
        if !trimmed.is_empty() {
            self.state = FactcheckButtonState::Changed;
        } else if self.initial_text.is_empty() {
            self.state = FactcheckButtonState::Initial;
        } else {
            self.state = FactcheckButtonState::Removed;
        }
    }

    pub fn remaining_chars(&self) -> i32 {
        self.limit as i32 - self.current_text.chars().count() as i32
    }

    pub fn save(&mut self) -> Option<String> {
        self.is_saved = true;
        if self.state == FactcheckButtonState::Removed {
            self.is_removed = true;
            None
        } else {
            Some(self.current_text.clone())
        }
    }
}
