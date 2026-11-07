//! GPUI Telegram History Item Reply Markup View Component
//!
//! Citations:
//! - `Telegram/SourceFiles/history/history_item_reply_markup.h:L80-L199`
//! - `Telegram/SourceFiles/history/history_item_reply_markup.cpp:L27-L50` (kButtonTypeIcons)
//! - `Telegram/SourceFiles/history/history_item_reply_markup.cpp:L438-L442` (OnlyBuyButton)

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui_telegram_client::history::item_reply_markup::{
    HistoryMessageMarkupButton, HistoryMessageMarkupButtonColor,
    HistoryMessageMarkupButtonIcon, HistoryMessageMarkupButtonType,
    HistoryMessageMarkupData, ReplyMarkupFlags,
};

/// Upstream `history_item_reply_markup.h:L181-L199`: Interactive Reply Markup Keyboard View
#[derive(Debug, Clone, Default)]
pub struct HistoryItemReplyMarkup {
    pub data: HistoryMessageMarkupData,
    pub activated_button_id: Option<i64>,
}

impl HistoryItemReplyMarkup {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_data(mut self, data: HistoryMessageMarkupData) -> Self {
        self.data = data;
        self
    }

    pub fn activate_button(&mut self, row: usize, col: usize) -> Option<HistoryMessageMarkupButton> {
        if row < self.data.rows.len() && col < self.data.rows[row].len() {
            let button = self.data.rows[row][col].clone();
            self.activated_button_id = Some(button.button_id);
            Some(button)
        } else {
            None
        }
    }

    pub fn render_view(&self) -> impl IntoElement {
        if self.data.rows.is_empty() {
            return div();
        }

        let mut keyboard = div().flex().flex_col().w_full().gap(px(4.0)).mt(px(4.0));

        for row in &self.data.rows {
            let mut row_div = div().flex().flex_row().w_full().gap(px(4.0));
            for button in row {
                let (bg_color, text_color) = match button.visual.color {
                    HistoryMessageMarkupButtonColor::Primary => (rgb(0x2AABEE), rgb(0xFFFFFF)),
                    HistoryMessageMarkupButtonColor::Danger => (rgb(0xE53935), rgb(0xFFFFFF)),
                    HistoryMessageMarkupButtonColor::Success => (rgb(0x4CAF50), rgb(0xFFFFFF)),
                    HistoryMessageMarkupButtonColor::Normal => (rgb(0xF0F2F5), rgb(0x222222)),
                };

                let icon_str = match button.icon() {
                    HistoryMessageMarkupButtonIcon::Url => " ↗",
                    HistoryMessageMarkupButtonIcon::Payment => " 💳",
                    HistoryMessageMarkupButtonIcon::SwitchPm => " 💬",
                    HistoryMessageMarkupButtonIcon::Webview => " 🌐",
                    HistoryMessageMarkupButtonIcon::Copy => " 📋",
                    HistoryMessageMarkupButtonIcon::None => "",
                };

                row_div = row_div.child(
                    div()
                        .flex_1()
                        .flex()
                        .items_center()
                        .justify_center()
                        .py(px(6.0))
                        .px(px(8.0))
                        .bg(bg_color)
                        .rounded(px(6.0))
                        .cursor_pointer()
                        .child(
                            div()
                                .text_size(px(13.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(text_color)
                                .child(format!("{}{}", button.text, icon_str)),
                        ),
                );
            }
            keyboard = keyboard.child(row_div);
        }

        keyboard
    }
}

impl Render for HistoryItemReplyMarkup {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.data.rows.is_empty() {
            return div();
        }

        let mut keyboard = div().flex().flex_col().w_full().gap(px(4.0)).mt(px(4.0));

        for row in &self.data.rows {
            let mut row_div = div().flex().flex_row().w_full().gap(px(4.0));
            for button in row {
                let (bg_color, text_color) = match button.visual.color {
                    HistoryMessageMarkupButtonColor::Primary => (rgb(0x2AABEE), rgb(0xFFFFFF)),
                    HistoryMessageMarkupButtonColor::Danger => (rgb(0xE53935), rgb(0xFFFFFF)),
                    HistoryMessageMarkupButtonColor::Success => (rgb(0x4CAF50), rgb(0xFFFFFF)),
                    HistoryMessageMarkupButtonColor::Normal => (rgb(0xF0F2F5), rgb(0x222222)),
                };

                let icon_str = match button.icon() {
                    HistoryMessageMarkupButtonIcon::Url => " ↗",
                    HistoryMessageMarkupButtonIcon::Payment => " 💳",
                    HistoryMessageMarkupButtonIcon::SwitchPm => " 💬",
                    HistoryMessageMarkupButtonIcon::Webview => " 🌐",
                    HistoryMessageMarkupButtonIcon::Copy => " 📋",
                    HistoryMessageMarkupButtonIcon::None => "",
                };

                row_div = row_div.child(
                    div()
                        .flex_1()
                        .flex()
                        .items_center()
                        .justify_center()
                        .py(px(6.0))
                        .px(px(8.0))
                        .bg(bg_color)
                        .rounded(px(6.0))
                        .cursor_pointer()
                        .child(
                            div()
                                .text_size(px(13.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(text_color)
                                .child(format!("{}{}", button.text, icon_str)),
                        ),
                );
            }
            keyboard = keyboard.child(row_div);
        }

        keyboard
    }
}
