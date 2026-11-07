//! Canonical Bot Keyboard Card (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.4.11).

use gpui::*;
use super::models::{InlineButtonType, InlineKeyboardButton};

pub struct InlineBotKeyboardView;

impl InlineBotKeyboardView {
    pub fn render(rows: &[Vec<InlineKeyboardButton>]) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .pt(px(4.0))
            .w_full()
            .children(rows.iter().map(|row| {
                div()
                    .flex()
                    .items_center()
                    .gap(px(4.0))
                    .w_full()
                    .children(row.iter().map(|btn| {
                        let icon = match &btn.button_type {
                            InlineButtonType::Callback(_) => "",
                            InlineButtonType::Url(_) => " ↗",
                            InlineButtonType::WebApp(_) => " 🌐",
                            InlineButtonType::SwitchInline(_) => " 🔍",
                        };

                        div()
                            .flex_1()
                            .h(px(32.0))
                            .rounded(px(6.0))
                            .bg(rgba(0x00000033))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(0x5288C1)) // Button text accent
                            .child(format!("{}{}", btn.text, icon))
                    }))
            }))
    }
}
