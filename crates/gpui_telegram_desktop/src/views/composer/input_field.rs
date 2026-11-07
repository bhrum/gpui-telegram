//! Multi-Line Rich Input Field with Formatting and Auto-Expansion
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/chat_helpers/message_field.cpp:L704-L712`
//! - `Telegram/SourceFiles/chat_helpers/input_field.h:L1-L180`
//!
//! Subsystem: chat_helpers, Feature: TCG-N11

use gpui::{px, Pixels};
use std::ops::Range;

/// Formatting style applied to a span of text in the composer
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormattingTag {
    Bold,
    Italic,
    Underline,
    Strike,
    Monospace,
    Spoiler,
    CustomEmoji(u64), // custom emoji document id
    Link(String),     // url
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormattedSpan {
    pub range: Range<usize>,
    pub tag: FormattingTag,
}

/// Rich Input Field state managing multi-line text, formatting spans, undo stack, and height limits.
#[derive(Debug, Clone)]
pub struct InputField {
    pub text: String,
    pub cursor: usize,
    pub selection: Option<Range<usize>>,
    pub formatting: Vec<FormattedSpan>,
    pub undo_stack: Vec<(String, usize)>,
    pub redo_stack: Vec<(String, usize)>,

    // Layout metrics (message_field.cpp:L704-L712)
    pub min_height: Pixels,
    pub max_height: Pixels,
    pub document_margin: Pixels,
    pub line_height: Pixels,
}

impl Default for InputField {
    fn default() -> Self {
        Self {
            text: String::new(),
            cursor: 0,
            selection: None,
            formatting: Vec::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            min_height: px(38.0),
            max_height: px(224.0),
            document_margin: px(4.0),
            line_height: px(20.0),
        }
    }
}

impl InputField {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.set_text(text.into());
        self
    }

    pub fn set_text(&mut self, text: String) {
        if self.text != text {
            self.undo_stack.push((self.text.clone(), self.cursor));
            self.text = text;
            self.cursor = self.text.chars().count();
            self.selection = None;
            self.redo_stack.clear();
        }
    }

    pub fn insert_text(&mut self, insert: &str) {
        self.undo_stack.push((self.text.clone(), self.cursor));
        if let Some(sel) = self.selection.take() {
            let start = sel.start.min(self.text.len());
            let end = sel.end.min(self.text.len());
            self.text.replace_range(start..end, insert);
            self.cursor = start + insert.len();
        } else {
            let idx = self.cursor.min(self.text.len());
            self.text.insert_str(idx, insert);
            self.cursor += insert.len();
        }
        self.redo_stack.clear();
    }

    pub fn apply_formatting(&mut self, tag: FormattingTag) {
        if let Some(sel) = &self.selection {
            if sel.start < sel.end && sel.end <= self.text.len() {
                self.formatting.push(FormattedSpan {
                    range: sel.clone(),
                    tag,
                });
            }
        }
    }

    pub fn clear_formatting_in_range(&mut self, range: &Range<usize>) {
        self.formatting.retain(|span| {
            !(span.range.start >= range.start && span.range.end <= range.end)
        });
    }

    pub fn undo(&mut self) -> bool {
        if let Some((prev_text, prev_cursor)) = self.undo_stack.pop() {
            self.redo_stack.push((self.text.clone(), self.cursor));
            self.text = prev_text;
            self.cursor = prev_cursor;
            self.selection = None;
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self) -> bool {
        if let Some((next_text, next_cursor)) = self.redo_stack.pop() {
            self.undo_stack.push((self.text.clone(), self.cursor));
            self.text = next_text;
            self.cursor = next_cursor;
            self.selection = None;
            true
        } else {
            false
        }
    }

    /// Calculate auto-expanded content height based on newline count and margins.
    pub fn calculate_content_height(&self) -> Pixels {
        let line_count = self.text.split('\n').count().max(1);
        let wanted = f32::from(self.document_margin) * 2.0 + (f32::from(self.line_height) * line_count as f32);
        px(wanted.clamp(f32::from(self.min_height), f32::from(self.max_height)))
    }
}
