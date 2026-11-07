//! Instant View Editor Auto-Pair Delimiters (`iv_editor_auto_pair.h/.cpp`)
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

/// Delimiter helper functions matching Telegram C++ implementation.
pub fn closing_for(opening: char) -> Option<char> {
    match opening {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '"' => Some('"'),
        _ => None,
    }
}

pub fn is_closing(ch: char) -> bool {
    matches!(ch, ')' | ']' | '}' | '"')
}

pub fn allowed_before(next: Option<char>) -> bool {
    match next {
        None => true,
        Some(c) if c.is_whitespace() || is_closing(c) || ".,;:!?".contains(c) => true,
        _ => false,
    }
}

pub fn allowed_after(typed: char, previous: Option<char>) -> bool {
    match previous {
        None => true,
        Some(p) => {
            if typed == '"' {
                !p.is_alphanumeric()
            } else if p.is_whitespace() || p.is_alphanumeric() {
                true
            } else {
                closing_for(p).is_some() || is_closing(p)
            }
        }
    }
}

/// Simulated text buffer with cursor and selection for auto-pairing.
#[derive(Debug, Clone, Default)]
pub struct AutoPairBuffer {
    pub text: String,
    pub cursor: usize,
    pub selection: Option<(usize, usize)>,
}

impl AutoPairBuffer {
    pub fn new(text: impl Into<String>) -> Self {
        let text = text.into();
        let cursor = text.len();
        Self {
            text,
            cursor,
            selection: None,
        }
    }

    pub fn char_before(&self) -> Option<char> {
        let pos = self.selection.map(|(s, _)| s).unwrap_or(self.cursor);
        if pos == 0 {
            None
        } else {
            self.text[..pos].chars().last()
        }
    }

    pub fn char_after(&self) -> Option<char> {
        let pos = self.selection.map(|(_, e)| e).unwrap_or(self.cursor);
        if pos >= self.text.len() {
            None
        } else {
            self.text[pos..].chars().next()
        }
    }

    pub fn remove_pair(&mut self) -> bool {
        if self.cursor > 0 && self.cursor < self.text.len() {
            self.text.remove(self.cursor);
            self.cursor -= 1;
            self.text.remove(self.cursor);
            true
        } else {
            false
        }
    }

    pub fn skip_closing(&mut self) -> bool {
        if self.cursor < self.text.len() {
            self.cursor += 1;
            true
        } else {
            false
        }
    }

    pub fn wrap_selection(&mut self, opening: char, closing: char) -> bool {
        if let Some((from, till)) = self.selection {
            let start = from.min(till);
            let end = from.max(till);
            let selected = &self.text[start..end];
            let wrapped = format!("{}{}{}", opening, selected, closing);
            self.text.replace_range(start..end, &wrapped);
            self.selection = Some((start + 1, end + 1));
            self.cursor = end + 1;
            true
        } else {
            false
        }
    }

    pub fn insert_pair(&mut self, opening: char, closing: char) -> bool {
        let pair = format!("{}{}", opening, closing);
        self.text.insert_str(self.cursor, &pair);
        self.cursor += 1;
        true
    }

    pub fn handle_char(&mut self, typed: char) -> bool {
        let next = self.char_after();
        let previous = self.char_before();

        if self.selection.is_none() && next == Some(typed) && is_closing(typed) {
            return self.skip_closing();
        }

        if let Some(closing) = closing_for(typed) {
            if self.selection.is_some() {
                return self.wrap_selection(typed, closing);
            }
            if allowed_after(typed, previous) && allowed_before(next) {
                return self.insert_pair(typed, closing);
            }
        }
        false
    }

    pub fn handle_backspace(&mut self) -> bool {
        if self.selection.is_none() {
            let prev = self.char_before();
            let next = self.char_after();
            if let (Some(p), Some(n)) = (prev, next) {
                if closing_for(p) == Some(n) {
                    return self.remove_pair();
                }
            }
        }
        false
    }
}

/// Auto-pairing editor component.
#[derive(Debug, Clone)]
pub struct IvEditorAutoPairEditor {
    pub is_active: bool,
    pub title: String,
    pub buffer: AutoPairBuffer,
    pub auto_pairs_triggered: usize,
}

impl Default for IvEditorAutoPairEditor {
    fn default() -> Self {
        Self {
            is_active: false,
            title: "Auto-Pair Delimiters Editor".to_string(),
            buffer: AutoPairBuffer::default(),
            auto_pairs_triggered: 0,
        }
    }
}

impl IvEditorAutoPairEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn handle_char(&mut self, ch: char) -> bool {
        let res = self.buffer.handle_char(ch);
        if res {
            self.auto_pairs_triggered += 1;
        }
        res
    }

    pub fn handle_backspace(&mut self) -> bool {
        let res = self.buffer.handle_backspace();
        if res {
            self.auto_pairs_triggered += 1;
        }
        res
    }
}

impl Render for IvEditorAutoPairEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let bg = if self.is_active {
            rgb(0x2AABEE)
        } else {
            rgb(0xFFFFFF)
        };
        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(12.0))
            .bg(bg)
            .rounded(px(8.0))
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x222222))
                    .child(self.title.clone())
            )
            .child(
                div()
                    .text_size(px(13.0))
                    .text_color(rgb(0x666666))
                    .child(format!(
                        "Buffer: \"{}\" | Cursor: {} | Auto-pairs: {}",
                        self.buffer.text, self.buffer.cursor, self.auto_pairs_triggered
                    ))
            )
    }
}
