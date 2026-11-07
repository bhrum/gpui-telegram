//! Authentic GPUI Sent Verification Code Input Field Component.
//! Reference: Telegram/SourceFiles/ui/widgets/sent_code_field.cpp:L20-L65
//! Reference: Telegram/SourceFiles/ui/widgets/sent_code_field.h:L18-L74

#![allow(dead_code, unused_imports, unused_variables)]

/// Target symbol for verification code input field with digit sanitization and auto-submit.
#[derive(Debug, Clone)]
pub struct SentCodeFieldWidgets {
    pub raw_text: String,
    pub auto_submit_length: usize,
    pub is_submitted: bool,
    pub has_changed: bool,
    pub cursor_position: usize,
}

impl Default for SentCodeFieldWidgets {
    fn default() -> Self {
        Self::new(5)
    }
}

impl SentCodeFieldWidgets {
    pub fn new(auto_submit_length: usize) -> Self {
        Self {
            raw_text: String::new(),
            auto_submit_length,
            is_submitted: false,
            has_changed: false,
            cursor_position: 0,
        }
    }

    pub fn set_auto_submit_length(&mut self, length: usize) {
        self.auto_submit_length = length;
    }

    /// Filters and returns only numeric digits from input:
    /// Reference: `sent_code_field.cpp:L37-L41`
    pub fn get_digits_only(&self) -> String {
        self.raw_text.chars().filter(|c| c.is_ascii_digit()).collect()
    }

    /// Sanitizes input, updates cursor, and checks auto-submit threshold.
    /// Reference: `sent_code_field.cpp:L43-L65`
    pub fn on_text_changed(&mut self, text: impl Into<String>) -> bool {
        self.raw_text = text.into();
        let digits = self.get_digits_only();
        self.cursor_position = digits.len();
        self.has_changed = true;

        if self.auto_submit_length > 0 && digits.len() >= self.auto_submit_length {
            self.is_submitted = true;
            true
        } else {
            self.is_submitted = false;
            false
        }
    }

    pub fn reset(&mut self) {
        self.raw_text.clear();
        self.is_submitted = false;
        self.has_changed = false;
        self.cursor_position = 0;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SentCodeCallState {
    Waiting,
    Calling,
    Called,
}

#[derive(Debug, Clone)]
pub struct SentCodeCall {
    pub timeout_seconds: u32,
    pub state: SentCodeCallState,
}

impl SentCodeCall {
    pub fn new(timeout_seconds: u32) -> Self {
        Self {
            timeout_seconds,
            state: SentCodeCallState::Waiting,
        }
    }

    pub fn tick_second(&mut self) -> bool {
        if self.timeout_seconds > 0 {
            self.timeout_seconds -= 1;
            if self.timeout_seconds == 0 {
                self.state = SentCodeCallState::Calling;
                return true;
            }
        }
        false
    }

    pub fn formatted_text(&self) -> String {
        match self.state {
            SentCodeCallState::Waiting => {
                let m = self.timeout_seconds / 60;
                let s = self.timeout_seconds % 60;
                if m >= 60 {
                    let h = m / 60;
                    let m_rem = m % 60;
                    format!("{h}:{m_rem:02}:{s:02}")
                } else {
                    format!("{m}:{s:02}")
                }
            }
            SentCodeCallState::Calling => "Calling...".to_string(),
            SentCodeCallState::Called => "Called".to_string(),
        }
    }
}
