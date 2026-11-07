//! Authentic GPUI Generic Confirmation & Information Modal Component.
//! Reference: Telegram/SourceFiles/ui/boxes/confirm_box.cpp:L25-L159
//! Reference: Telegram/SourceFiles/ui/boxes/confirm_box.h:L18-L70

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmBoxResult {
    Confirmed,
    Cancelled,
    Dismissed,
}

/// Target symbol for confirmation and alert modal box.
/// Reference: `confirm_box.h:L20-L50`
#[derive(Debug, Clone)]
pub struct ConfirmBoxBoxes {
    pub title: Option<String>,
    pub text: String,
    pub confirm_text: String,
    pub cancel_text: String,
    pub is_inform: bool,
    pub is_strict_cancel: bool,
    pub result: Option<ConfirmBoxResult>,
    pub is_open: bool,
}

impl Default for ConfirmBoxBoxes {
    fn default() -> Self {
        Self::new("Are you sure?", "Do you want to proceed with this action?")
    }
}

impl ConfirmBoxBoxes {
    pub fn new(title: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            title: Some(title.into()),
            text: text.into(),
            confirm_text: "OK".to_string(),
            cancel_text: "Cancel".to_string(),
            is_inform: false,
            is_strict_cancel: false,
            result: None,
            is_open: true,
        }
    }

    pub fn inform(text: impl Into<String>) -> Self {
        let mut b = Self::new("Information", text);
        b.is_inform = true;
        b
    }

    /// User confirmed the modal action.
    /// Reference: `confirm_box.cpp:L60-L75`
    pub fn confirm(&mut self) {
        self.result = Some(ConfirmBoxResult::Confirmed);
        self.is_open = false;
    }

    /// User explicitly clicked cancel button.
    /// Reference: `confirm_box.cpp:L78-L90`
    pub fn cancel(&mut self) {
        self.result = Some(ConfirmBoxResult::Cancelled);
        self.is_open = false;
    }

    /// User dismissed dialog via backdrop or ESC key.
    /// Under strict cancel semantics, cancel callback is skipped.
    /// Reference: `confirm_box.h:L48-L50`
    pub fn dismiss(&mut self) {
        if !self.is_strict_cancel {
            self.result = Some(ConfirmBoxResult::Cancelled);
        } else {
            self.result = Some(ConfirmBoxResult::Dismissed);
        }
        self.is_open = false;
    }
}
