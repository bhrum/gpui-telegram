//! Composer Message Field Controller and Link Parsing
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/chat_helpers/message_field.h:L1-L241`
//! - `Telegram/SourceFiles/chat_helpers/message_field.cpp:L1-L1665`
//! - `Telegram/SourceFiles/chat_helpers/message_field.cpp:L76-L84` (`kParseLinksTimeout`, `kLinkProtocols`)
//!
//! Subsystem: chat_helpers, Feature: TCG-N11

use super::input_field::{FormattingTag, InputField};
use gpui::{px, Pixels};
use std::ops::Range;
use std::time::Duration;

pub const K_PARSE_LINKS_TIMEOUT: Duration = Duration::from_millis(1000);
pub const K_TYPES_DURATION: Duration = Duration::from_millis(4000);
pub const K_CODE_LANGUAGE_LIMIT: usize = 32;
pub const K_LINK_PROTOCOLS: &[&str] = &["http://", "https://", "tonsite://"];

/// Extracted link range within composer text
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageLinkRange {
    pub range: Range<usize>,
    pub url: String,
    pub is_webpage_preview_candidate: bool,
}

/// Message Field controller managing text editing, link preview extraction,
/// draft persistence, and edit-previous-message triggers.
#[derive(Debug, Clone)]
pub struct MessageField {
    pub input: InputField,
    pub draft_text: String,
    pub parsed_links: Vec<MessageLinkRange>,
    pub webpage_preview_url: Option<String>,
    pub webpage_preview_disabled: bool,
    pub is_editing_existing_message: bool,
    pub editing_message_id: Option<i64>,

    // Typing activity indicator state
    pub last_typed_at: Option<std::time::Instant>,
    pub is_sending: bool,
}

impl Default for MessageField {
    fn default() -> Self {
        Self {
            input: InputField::default(),
            draft_text: String::new(),
            parsed_links: Vec::new(),
            webpage_preview_url: None,
            webpage_preview_disabled: false,
            is_editing_existing_message: false,
            editing_message_id: None,
            last_typed_at: None,
            is_sending: false,
        }
    }
}

impl MessageField {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(&self) -> &str {
        &self.input.text
    }

    pub fn set_text(&mut self, text: String) {
        self.input.set_text(text);
        self.parse_links();
        self.last_typed_at = Some(std::time::Instant::now());
    }

    pub fn clear(&mut self) {
        self.input.set_text(String::new());
        self.parsed_links.clear();
        self.webpage_preview_url = None;
        self.is_editing_existing_message = false;
        self.editing_message_id = None;
    }

    /// Extract links matching `kLinkProtocols` per `message_field.cpp:L80-L84`.
    pub fn parse_links(&mut self) {
        self.parsed_links.clear();
        let text = &self.input.text;

        for (word_idx, word) in text.split_whitespace().enumerate() {
            for proto in K_LINK_PROTOCOLS {
                if word.starts_with(proto) && word.len() > proto.len() {
                    if let Some(start) = text.find(word) {
                        let end = start + word.len();
                        self.parsed_links.push(MessageLinkRange {
                            range: start..end,
                            url: word.to_string(),
                            is_webpage_preview_candidate: word_idx == 0,
                        });
                    }
                    break;
                }
            }
        }

        if !self.webpage_preview_disabled {
            self.webpage_preview_url = self.parsed_links
                .first()
                .map(|l| l.url.clone());
        }
    }

    /// Determine if typing notification is active (valid for `kTypesDuration` = 4s)
    pub fn is_typing_active(&self) -> bool {
        if let Some(typed) = self.last_typed_at {
            typed.elapsed() < K_TYPES_DURATION
        } else {
            false
        }
    }

    /// Format current selection with bold, italic, code, spoiler, etc.
    pub fn format_selection(&mut self, tag: FormattingTag) {
        self.input.apply_formatting(tag);
    }

    /// Handle edit previous message shortcut (UpArrow on empty composer)
    pub fn can_trigger_edit_last_message(&self) -> bool {
        self.input.text.is_empty() && !self.is_editing_existing_message
    }

    pub fn start_editing_message(&mut self, message_id: i64, original_text: String) {
        self.is_editing_existing_message = true;
        self.editing_message_id = Some(message_id);
        self.input.set_text(original_text);
        self.parse_links();
    }

    pub fn cancel_editing(&mut self) {
        self.is_editing_existing_message = false;
        self.editing_message_id = None;
        self.input.set_text(std::mem::take(&mut self.draft_text));
        self.parse_links();
    }
}
