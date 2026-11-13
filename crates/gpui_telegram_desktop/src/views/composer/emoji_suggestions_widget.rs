//! Colon-Triggered Inline Emoji Suggestions Engine
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/chat_helpers/emoji_suggestions_widget.h:L1-L120`
//! - `Telegram/SourceFiles/chat_helpers/emoji_suggestions_widget.cpp:L1-L1153`
//! - `Telegram/SourceFiles/chat_helpers/emoji_suggestions_widget.cpp:L40-L41` (`kShowExactDelay`, `kMaxNonScrolledEmoji`)
//! - `Telegram/SourceFiles/chat_helpers/emoji_suggestions_widget.cpp:L956-L971` (Colon parser & replacement math)
//!
//! Subsystem: chat_helpers, Feature: TCG-N11

use std::time::Duration;

pub const K_SHOW_EXACT_DELAY: Duration = Duration::from_millis(300);
pub const K_MAX_NON_SCROLLED_EMOJI: usize = 7;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmojiSuggestionItem {
    pub emoji: String,
    pub keyword: String,
    pub custom_document_id: Option<u64>,
}

/// Emoji suggestions dropdown controller triggered by colon syntax (e.g. `:smile`).
/// Target symbol in ledger: `EmojiSuggestionsWidget`.
#[derive(Debug, Clone)]
pub struct EmojiSuggestionsWidget {
    pub is_visible: bool,
    pub query: String,
    pub query_start_position: usize,
    pub query_length: usize,
    pub selected_index: i32,
    pub suggestions: Vec<EmojiSuggestionItem>,
    pub max_visible_items: usize,
}

impl Default for EmojiSuggestionsWidget {
    fn default() -> Self {
        Self {
            is_visible: false,
            query: String::new(),
            query_start_position: 0,
            query_length: 0,
            selected_index: 0,
            suggestions: Vec::new(),
            max_visible_items: K_MAX_NON_SCROLLED_EMOJI,
        }
    }
}

impl EmojiSuggestionsWidget {
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse text before cursor to check for colon emoji trigger.
    /// Derived from `emoji_suggestions_widget.cpp:L956-L971`.
    pub fn parse_query(&mut self, text: &str, cursor_pos: usize) -> Option<String> {
        if text.is_empty() || cursor_pos == 0 || cursor_pos > text.len() {
            self.hide();
            return None;
        }

        let slice = &text[..cursor_pos];
        let chars: Vec<char> = slice.chars().collect();
        let len = chars.len();

        for i in (0..len).rev() {
            if chars[i] == ':' {
                // Check preceding char (must NOT be letter or digit)
                if i > 0 && (chars[i - 1].is_alphanumeric()) {
                    self.hide();
                    return None;
                }
                // Check following char (cannot be space or empty)
                if i + 1 == len || chars[i + 1].is_whitespace() {
                    self.hide();
                    return None;
                }

                let q: String = chars[i + 1..len].iter().collect();
                self.query = q.clone();
                self.query_start_position = i;
                self.query_length = len - i;
                self.is_visible = true;
                return Some(q);
            }
        }

        self.hide();
        None
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
        self.query.clear();
        self.query_length = 0;
        self.selected_index = 0;
    }

    /// Calculate replacement range in the editor for the selected emoji.
    /// `from = query_start_position, to = query_start_position + query_length`.
    pub fn replacement_range(&self) -> std::ops::Range<usize> {
        self.query_start_position..(self.query_start_position + self.query_length)
    }

    pub fn move_selection(&mut self, delta: i32) -> bool {
        if self.suggestions.is_empty() {
            return false;
        }
        let count = self.suggestions.len() as i32;
        let next = self.selected_index + delta;
        self.selected_index = if next < 0 {
            count - 1
        } else if next >= count {
            0
        } else {
            next
        };
        true
    }

    pub fn selected_item(&self) -> Option<&EmojiSuggestionItem> {
        if self.selected_index >= 0 && (self.selected_index as usize) < self.suggestions.len() {
            Some(&self.suggestions[self.selected_index as usize])
        } else {
            None
        }
    }
}
