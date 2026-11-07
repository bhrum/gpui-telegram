//! IME Safety State Machine for Telegram Multi-line Composer (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.4).
//! Prevents premature message submission when composing with Chinese/Japanese/Korean Input Method Editors.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImeState {
    /// Idle: No active IME preedit composition. Enter key sends message (or Shift+Enter newlines).
    Idle,
    /// Composing: User is actively typing IME phonetic/radical keys (e.g. Pinyin or Romaji).
    /// Enter key selects/confirms current preedit candidate and MUST NOT submit the message.
    Composing {
        preedit_text: String,
        cursor_pos: usize,
    },
    /// Committed: The selected IME candidate has been inserted into the buffer.
    Committed {
        committed_text: String,
    },
}

impl Default for ImeState {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyAction {
    /// Send the composed message buffer
    SendMessage,
    /// Insert a newline into multi-line buffer
    InsertNewline,
    /// Confirm IME composition candidate without sending
    ImeCandidateChosen,
    /// Normal character insertion / edit
    DefaultEdit,
    /// Ignored
    None,
}

pub struct ImeSafetyMachine {
    pub state: ImeState,
    pub text_buffer: String,
    pub cursor_index: usize,
}

impl ImeSafetyMachine {
    pub fn new() -> Self {
        Self {
            state: ImeState::Idle,
            text_buffer: String::new(),
            cursor_index: 0,
        }
    }

    pub fn with_text(text: impl Into<String>) -> Self {
        let text_buffer = text.into();
        let cursor_index = text_buffer.len();
        Self {
            state: ImeState::Idle,
            text_buffer,
            cursor_index,
        }
    }

    /// Called when OS IME begins composition
    pub fn on_ime_start(&mut self) {
        self.state = ImeState::Composing {
            preedit_text: String::new(),
            cursor_pos: 0,
        };
    }

    /// Called as user types phonetic/radical keystrokes in IME preedit window
    pub fn on_ime_update(&mut self, preedit: &str, cursor_pos: usize) {
        self.state = ImeState::Composing {
            preedit_text: preedit.to_string(),
            cursor_pos,
        };
    }

    /// Called when user commits a candidate from IME into the real text buffer
    pub fn on_ime_commit(&mut self, committed: &str) {
        self.insert_text(committed);
        self.state = ImeState::Committed {
            committed_text: committed.to_string(),
        };
        // Reset back to Idle for subsequent inputs
        self.state = ImeState::Idle;
    }

    /// Called when IME composition is canceled without committing
    pub fn on_ime_cancel(&mut self) {
        self.state = ImeState::Idle;
    }

    /// Process Enter key press with IME safety guard
    pub fn handle_enter_key(&mut self, is_shift: bool) -> KeyAction {
        match &self.state {
            ImeState::Composing { preedit_text, .. } => {
                // If IME is composing, Enter selects the preedit text
                let committed = preedit_text.clone();
                self.on_ime_commit(&committed);
                KeyAction::ImeCandidateChosen
            }
            ImeState::Idle | ImeState::Committed { .. } => {
                if is_shift {
                    self.insert_text("\n");
                    KeyAction::InsertNewline
                } else if !self.text_buffer.trim().is_empty() {
                    KeyAction::SendMessage
                } else {
                    KeyAction::None
                }
            }
        }
    }

    pub fn insert_text(&mut self, text: &str) {
        if self.cursor_index >= self.text_buffer.len() {
            self.text_buffer.push_str(text);
            self.cursor_index = self.text_buffer.len();
        } else {
            self.text_buffer.insert_str(self.cursor_index, text);
            self.cursor_index += text.len();
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor_index > 0 && !self.text_buffer.is_empty() {
            let mut prev_char_boundary = self.cursor_index - 1;
            while !self.text_buffer.is_char_boundary(prev_char_boundary) && prev_char_boundary > 0 {
                prev_char_boundary -= 1;
            }
            self.text_buffer.remove(prev_char_boundary);
            self.cursor_index = prev_char_boundary;
        }
    }

    pub fn clear(&mut self) {
        self.text_buffer.clear();
        self.cursor_index = 0;
        self.state = ImeState::Idle;
    }

    pub fn text(&self) -> &str {
        &self.text_buffer
    }

    pub fn is_composing(&self) -> bool {
        matches!(self.state, ImeState::Composing { .. })
    }

    pub fn preedit_text(&self) -> Option<&str> {
        match &self.state {
            ImeState::Composing { preedit_text, .. } => Some(preedit_text.as_str()),
            _ => None,
        }
    }
}

impl Default for ImeSafetyMachine {
    fn default() -> Self {
        Self::new()
    }
}
