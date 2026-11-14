//! Upstream Source: Telegram/SourceFiles/intro/intro_code_input.cpp, intro_code_input.h
//! Subsystem: intro | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use crate::theme::TdesktopThemePalette;
use gpui::*;

pub const MAX_DIGITS_COUNT: usize = 32;

#[derive(Debug, Clone)]
pub struct IntroCodeInput {
    pub digits_count: usize,
    pub digits: Vec<Option<char>>,
    pub focused_index: usize,
    pub is_error: bool,
    pub digit_height_px: f32,
    pub digit_skip_px: f32,
    pub font_size_px: f32,
}

impl Default for IntroCodeInput {
    fn default() -> Self {
        Self::new(5)
    }
}

impl IntroCodeInput {
    pub fn new(count: usize) -> Self {
        let count = count.clamp(1, MAX_DIGITS_COUNT);
        Self {
            digits_count: count,
            digits: vec![None; count],
            focused_index: 0,
            is_error: false,
            digit_height_px: 50.0,
            digit_skip_px: 10.0,
            font_size_px: 20.0,
        }
    }

    pub fn set_digits_count(&mut self, count: usize) {
        let count = count.clamp(1, MAX_DIGITS_COUNT);
        self.digits_count = count;
        self.digits.resize(count, None);
        if self.focused_index >= count {
            self.focused_index = count - 1;
        }
    }

    pub fn push_digit(&mut self, c: char) -> bool {
        if !c.is_ascii_digit() {
            return false;
        }
        self.is_error = false;
        if let Some(pos) = self.digits.iter().position(|d| d.is_none()) {
            self.digits[pos] = Some(c);
            self.focused_index = (pos + 1).min(self.digits_count - 1);
            true
        } else {
            let last = self.digits_count - 1;
            self.digits[last] = Some(c);
            false
        }
    }

    pub fn pop_digit(&mut self) -> bool {
        self.is_error = false;
        if let Some(last_pos) = self.digits.iter().rposition(|d| d.is_some()) {
            self.digits[last_pos] = None;
            self.focused_index = last_pos;
            true
        } else {
            false
        }
    }

    pub fn set_code(&mut self, code_str: &str) {
        self.clear();
        for c in code_str.chars() {
            if c.is_ascii_digit() && !self.push_digit(c) {
                break;
            }
        }
    }

    pub fn clear(&mut self) {
        for d in &mut self.digits {
            *d = None;
        }
        self.focused_index = 0;
        self.is_error = false;
    }

    pub fn code(&self) -> String {
        self.digits.iter().flatten().collect()
    }

    pub fn is_complete(&self) -> bool {
        self.digits.iter().all(|d| d.is_some())
    }

    pub fn set_error(&mut self, err: bool) {
        self.is_error = err;
    }

    pub fn render_view(&self) -> AnyElement {
        self.render_themed(&TdesktopThemePalette::default())
    }

    pub fn render_themed(&self, palette: &TdesktopThemePalette) -> AnyElement {
        let height = self.digit_height_px;
        let skip = self.digit_skip_px;
        let font_sz = self.font_size_px;
        let active_idx = self.digits.iter().position(|d| d.is_none()).unwrap_or(self.digits_count - 1);

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .gap(px(skip))
            .children(self.digits.iter().enumerate().map(|(i, digit)| {
                let is_focused = i == active_idx;
                let border_col = if self.is_error {
                    rgb(palette.danger)
                } else if is_focused {
                    rgb(palette.accent)
                } else {
                    rgb(palette.border_fg)
                };

                let char_str = digit.map(|c| c.to_string()).unwrap_or_default();

                div()
                    .w(px(38.0))
                    .h(px(height))
                    .rounded(px(8.0))
                    .bg(rgb(palette.input_bg))
                    .border_b_2()
                    .border_color(border_col)
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(font_sz))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(palette.text_primary))
                    .child(char_str)
            }))
            .into_any_element()
    }
}

impl Render for IntroCodeInput {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.render_view()
    }
}
