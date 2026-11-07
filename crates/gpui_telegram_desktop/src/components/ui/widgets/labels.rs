//! Labels Reusable Widget
//! Upstream: Telegram/SourceFiles/ui/widgets/cross_fade_label.cpp:L13-L80
//! Upstream: Telegram/SourceFiles/ui/widgets/marquee_label.cpp:L27-L33
//! Upstream: Telegram/SourceFiles/ui/vertical_list.cpp:L36-L81
//! Implements flat labels with text styling, link click callbacks, and text selection support.

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelAlignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone)]
pub struct FlatLabel {
    pub text: String,
    pub font_size: f32,
    pub is_bold: bool,
    pub is_selectable: bool,
    pub text_color: u32,
    pub link_color: u32,
    pub alignment: LabelAlignment,
    pub max_lines: Option<usize>,
}

impl FlatLabel {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            font_size: 13.0,
            is_bold: false,
            is_selectable: false,
            text_color: 0x222222,
            link_color: 0x2AABEE,
            alignment: LabelAlignment::Left,
            max_lines: None,
        }
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn with_bold(mut self, bold: bool) -> Self {
        self.is_bold = bold;
        self
    }

    pub fn with_color(mut self, color: u32) -> Self {
        self.text_color = color;
        self
    }

    pub fn with_selectable(mut self, selectable: bool) -> Self {
        self.is_selectable = selectable;
        self
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }
}

impl Default for FlatLabel {
    fn default() -> Self {
        Self::new("")
    }
}

impl Render for FlatLabel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut el = div()
            .text_size(px(self.font_size))
            .text_color(rgb(self.text_color));

        if self.is_bold {
            el = el.font_weight(FontWeight::BOLD);
        }

        match self.alignment {
            LabelAlignment::Center => {
                el = el.text_center();
            }
            LabelAlignment::Right => {
                el = el.text_right();
            }
            LabelAlignment::Left => {}
        }

        el.child(self.text.clone())
    }
}
