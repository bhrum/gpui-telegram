//! Authentic GPUI Phone Confirmation & SMS Verification Dialog.
//! Reference: Telegram/SourceFiles/ui/boxes/confirm_phone_box.cpp:L30-L187
//! Reference: Telegram/SourceFiles/ui/boxes/confirm_phone_box.h:L20-L70

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Target symbol for phone confirmation code dialog.
/// Reference: `confirm_phone_box.h:L25-L65`
#[derive(Debug, Clone)]
pub struct ConfirmPhoneBoxBoxes {
    pub phone: String,
    pub sent_code_length: usize,
    pub entered_code: String,
    pub is_waiting_check: bool,
    pub call_timeout_seconds: Option<i32>,
    pub error_text: Option<String>,
    pub check_requested: bool,
    pub resend_requested: bool,
}

impl Default for ConfirmPhoneBoxBoxes {
    fn default() -> Self {
        Self::new("+1234567890", 5)
    }
}

impl ConfirmPhoneBoxBoxes {
    pub fn new(phone: impl Into<String>, code_length: usize) -> Self {
        Self {
            phone: phone.into(),
            sent_code_length: code_length,
            entered_code: String::new(),
            is_waiting_check: false,
            call_timeout_seconds: Some(60),
            error_text: None,
            check_requested: false,
            resend_requested: false,
        }
    }

    /// Handles digit input and checks if code length matches for auto-submit.
    /// Reference: `confirm_phone_box.cpp:L65-L80`
    pub fn enter_digit(&mut self, ch: char) -> bool {
        if ch.is_ascii_digit() && self.entered_code.len() < self.sent_code_length {
            self.entered_code.push(ch);
            self.error_text = None;
            if self.entered_code.len() == self.sent_code_length {
                self.is_waiting_check = true;
                self.check_requested = true;
                return true; // Auto-submitted
            }
        }
        false
    }

    pub fn request_resend(&mut self) {
        self.resend_requested = true;
    }

    pub fn set_server_error(&mut self, err: impl Into<String>) {
        self.error_text = Some(err.into());
        self.is_waiting_check = false;
        self.check_requested = false;
    }
}
