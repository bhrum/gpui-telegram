//! InputFields Reusable Widget
//! Upstream: Telegram/SourceFiles/ui/widgets/fields/special_fields.cpp:L18-L100, L315-L343
//! Upstream: Telegram/SourceFiles/ui/widgets/sent_code_field.cpp:L21-L98
//! Implements standard reactive GPUI single-line and multi-line text input fields.

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_INPUT_CURSOR_BLINK_INTERVAL_MS: u64 = 500;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputFieldMode {
    SingleLine,
    MultiLine,
    Password,
}

#[derive(Debug, Clone)]
pub struct InputField {
    pub text: String,
    pub placeholder: String,
    pub mode: InputFieldMode,
    pub is_focused: bool,
    pub is_disabled: bool,
    pub cursor_pos: usize,
    pub selection_range: Option<(usize, usize)>,
    pub max_length: Option<usize>,
    pub border_color: u32,
    pub border_active_color: u32,
    pub bg_color: u32,
    pub text_color: u32,
    pub placeholder_color: u32,
}

impl InputField {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            placeholder: String::new(),
            mode: InputFieldMode::SingleLine,
            is_focused: false,
            is_disabled: false,
            cursor_pos: 0,
            selection_range: None,
            max_length: None,
            border_color: 0xDFE1E5,
            border_active_color: 0x2AABEE,
            bg_color: 0xFFFFFF,
            text_color: 0x222222,
            placeholder_color: 0x707579,
        }
    }

    pub fn with_placeholder(mut self, ph: impl Into<String>) -> Self {
        self.placeholder = ph.into();
        self
    }

    pub fn with_mode(mut self, mode: InputFieldMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_max_length(mut self, max: usize) -> Self {
        self.max_length = Some(max);
        self
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        let mut t = text.into();
        if let Some(max) = self.max_length {
            if t.len() > max {
                t.truncate(max);
            }
        }
        self.text = t;
        self.cursor_pos = self.text.len();
        self.selection_range = None;
    }

    pub fn insert_char(&mut self, ch: char) {
        if self.is_disabled {
            return;
        }
        if let Some(max) = self.max_length {
            if self.text.len() >= max {
                return;
            }
        }
        self.text.insert(self.cursor_pos, ch);
        self.cursor_pos += 1;
    }

    pub fn backspace(&mut self) -> bool {
        if self.is_disabled || self.cursor_pos == 0 {
            return false;
        }
        self.cursor_pos -= 1;
        self.text.remove(self.cursor_pos);
        true
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor_pos = 0;
        self.selection_range = None;
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.is_focused = focused;
    }

    pub fn masked_text(&self) -> String {
        if self.mode == InputFieldMode::Password {
            "•".repeat(self.text.len())
        } else {
            self.text.clone()
        }
    }
}

impl Default for InputField {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InputField {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let border_col = if self.is_focused {
            rgb(self.border_active_color)
        } else {
            rgb(self.border_color)
        };

        let displayed = self.masked_text();
        let is_empty = displayed.is_empty();

        div()
            .flex()
            .items_center()
            .w_full()
            .h(px(38.0))
            .px(px(10.0))
            .bg(rgb(self.bg_color))
            .border_1()
            .border_color(border_col)
            .rounded(px(6.0))
            .child(
                div()
                    .flex_1()
                    .text_size(px(14.0))
                    .text_color(if is_empty {
                        rgb(self.placeholder_color)
                    } else {
                        rgb(self.text_color)
                    })
                    .child(if is_empty {
                        self.placeholder.clone()
                    } else {
                        displayed
                    }),
            )
    }
}
