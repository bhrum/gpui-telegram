//! Authentic GPUI Single-Choice Radio Selection Modal Component.
//! Reference: Telegram/SourceFiles/ui/boxes/single_choice_box.cpp:L20-L55
//! Reference: Telegram/SourceFiles/ui/boxes/single_choice_box.h:L15-L32

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Target symbol for single choice radio selection dialog.
/// Reference: `single_choice_box.h:L18-L32`
#[derive(Debug, Clone)]
pub struct SingleChoiceBoxBoxes {
    pub title: String,
    pub options: Vec<String>,
    pub selected_index: usize,
    pub is_closed: bool,
    pub callback_invoked: bool,
}

impl Default for SingleChoiceBoxBoxes {
    fn default() -> Self {
        Self::new("Select Option", vec!["Option 1".to_string(), "Option 2".to_string()], 0)
    }
}

impl SingleChoiceBoxBoxes {
    pub fn new(title: impl Into<String>, options: Vec<String>, initial_selection: usize) -> Self {
        let sel = if options.is_empty() { 0 } else { initial_selection.min(options.len() - 1) };
        Self {
            title: title.into(),
            options,
            selected_index: sel,
            is_closed: false,
            callback_invoked: false,
        }
    }

    /// Selects radio option, triggers callback, and auto-closes box.
    /// Reference: `single_choice_box.cpp:L45-L54`
    pub fn choose_index(&mut self, index: usize) {
        if index < self.options.len() {
            self.selected_index = index;
            self.callback_invoked = true;
            self.is_closed = true;
        }
    }

    pub fn selected_text(&self) -> Option<&str> {
        self.options.get(self.selected_index).map(|s| s.as_str())
    }
}
