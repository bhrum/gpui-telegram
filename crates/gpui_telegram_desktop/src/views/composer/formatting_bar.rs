//! Rich Text Formatting Bar (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.4).
//! Upstream reference: `tdesktop/Telegram/SourceFiles/chat_helpers/message_field.cpp`.

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextStyleKind {
    Bold,
    Italic,
    Monospace,
    Strikethrough,
    Underline,
    Spoiler,
    Link,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormatRange {
    pub start: usize,
    pub end: usize,
    pub kind: TextStyleKind,
    pub link_url: Option<String>,
}

pub struct FormattingBar {
    pub is_visible: bool,
    pub active_styles: Vec<TextStyleKind>,
}

impl FormattingBar {
    pub fn new() -> Self {
        Self {
            is_visible: false,
            active_styles: Vec::new(),
        }
    }

    pub fn toggle_style(&mut self, style: TextStyleKind) {
        if let Some(pos) = self.active_styles.iter().position(|s| *s == style) {
            self.active_styles.remove(pos);
        } else {
            self.active_styles.push(style);
        }
    }

    pub fn has_style(&self, style: TextStyleKind) -> bool {
        self.active_styles.contains(&style)
    }

    pub fn render(&self) -> impl IntoElement {
        let buttons = [
            (TextStyleKind::Bold, "B", "Bold (Cmd+B)"),
            (TextStyleKind::Italic, "I", "Italic (Cmd+I)"),
            (TextStyleKind::Monospace, "<>", "Monospace (Cmd+Shift+M)"),
            (TextStyleKind::Strikethrough, "S̶", "Strikethrough"),
            (TextStyleKind::Underline, "U̲", "Underline"),
            (TextStyleKind::Spoiler, "👁", "Spoiler"),
            (TextStyleKind::Link, "🔗", "Add Link"),
        ];

        div()
            .flex()
            .items_center()
            .gap(px(2.0))
            .px(px(6.0))
            .py(px(3.0))
            .rounded(px(6.0))
            .bg(rgb(0x1E2B38))
            .border_1()
            .border_color(rgb(0x2B5278))
            .children(buttons.iter().map(|&(kind, label, _tooltip)| {
                let is_active = self.has_style(kind);
                let btn_bg = if is_active {
                    rgb(0x2AABEE)
                } else {
                    rgba(0x00000000)
                };
                let text_color = if is_active {
                    rgb(0xFFFFFF)
                } else {
                    rgb(0x8A9AA8)
                };

                div()
                    .w(px(26.0))
                    .h(px(26.0))
                    .rounded(px(4.0))
                    .bg(btn_bg)
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(12.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(text_color)
                    .child(label)
            }))
    }
}

impl Default for FormattingBar {
    fn default() -> Self {
        Self::new()
    }
}
