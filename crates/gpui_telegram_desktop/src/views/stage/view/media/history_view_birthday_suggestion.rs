//! Genuine History View Birthday Suggestion Component (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_birthday_suggestion.h:L20-L70`
//! - `Telegram/SourceFiles/history/view/media/history_view_birthday_suggestion.cpp:L43-L60`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;

#[derive(Debug, Clone)]
pub struct HistoryViewBirthdaySuggestionViewMedia {
    pub user_name: String,
    pub day: u8,
    pub month: u8,
}

impl HistoryViewBirthdaySuggestionViewMedia {
    pub fn new(user_name: impl Into<String>, day: u8, month: u8) -> Self {
        Self {
            user_name: user_name.into(),
            day,
            month,
        }
    }
}

impl Default for HistoryViewBirthdaySuggestionViewMedia {
    fn default() -> Self {
        Self::new("Bob", 15, 8)
    }
}

impl Render for HistoryViewBirthdaySuggestionViewMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w(px(280.0))
            .p(px(12.0))
            .rounded(px(10.0))
            .bg(rgb(0x182533))
            .gap(px(8.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(div().text_size(px(24.0)).child("🎂"))
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child(format!("Today is {}'s Birthday!", self.user_name)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .py(px(6.0))
                    .rounded(px(6.0))
                    .bg(rgb(0x2AABEE))
                    .text_size(px(12.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xFFFFFF))
                    .child("🎁 Send a Gift"),
            )
    }
}
