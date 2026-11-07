//! Upstream Source: Telegram/SourceFiles/intro/intro_code_input.cpp, intro_code_input.h
//! Subsystem: intro | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

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
            self.focused_index = count.saturating_sub(1);
        }
    }

    pub fn push_digit(&mut self, c: char) -> bool {
        if !c.is_ascii_digit() {
            return false;
        }
        self.is_error = false;
        if self.focused_index < self.digits_count {
            self.digits[self.focused_index] = Some(c);
            if self.focused_index + 1 < self.digits_count {
                self.focused_index += 1;
            }
            true
        } else {
            false
        }
    }

    pub fn pop_digit(&mut self) -> bool {
        self.is_error = false;
        if self.digits[self.focused_index].is_some() {
            self.digits[self.focused_index] = None;
            true
        } else if self.focused_index > 0 {
            self.focused_index -= 1;
            self.digits[self.focused_index] = None;
            true
        } else {
            false
        }
    }

    pub fn set_code(&mut self, code_str: &str) {
        self.clear();
        for c in code_str.chars().filter(|c| c.is_ascii_digit()) {
            if !self.push_digit(c) {
                break;
            }
        }
    }

    pub fn clear(&mut self) {
        for slot in &mut self.digits {
            *slot = None;
        }
        self.focused_index = 0;
        self.is_error = false;
    }

    pub fn code(&self) -> String {
        self.digits.iter().filter_map(|&d| d).collect()
    }

    pub fn is_complete(&self) -> bool {
        self.digits.iter().all(|d| d.is_some())
    }

    pub fn set_error(&mut self, err: bool) {
        self.is_error = err;
    }

    pub fn render_view(&self) -> AnyElement {
        let digit_width = (300.0 - (self.digits_count as f32 - 1.0) * self.digit_skip_px) / self.digits_count as f32;

        div()
            .flex()
            .flex_row()
            .justify_center()
            .items_center()
            .gap(px(self.digit_skip_px))
            .w_full()
            .max_w(px(320.0))
            .py(px(8.0))
            .children((0..self.digits_count).map(|idx| {
                let digit_char = self.digits.get(idx).copied().flatten();
                let is_focused = idx == self.focused_index;

                let border_color = if self.is_error {
                    rgb(0xee5655)
                } else if is_focused {
                    rgb(0x2aabee)
                } else {
                    rgb(0x242f3d)
                };

                div()
                    .w(px(digit_width))
                    .h(px(self.digit_height_px))
                    .rounded(px(6.0))
                    .bg(rgb(0x17212b))
                    .border_2()
                    .border_color(border_color)
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_size(px(self.font_size_px))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .child(digit_char.map(|c| c.to_string()).unwrap_or_default()),
                    )
            }))
            .into_any_element()
    }
}

impl Render for IntroCodeInput {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.render_view()
    }
}
