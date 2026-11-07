//! Genuine History View Userpic Suggestion Component (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_userpic_suggestion.h:L20-L58`
//! - `Telegram/SourceFiles/history/view/media/history_view_userpic_suggestion.cpp:L39-L60`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use crate::views::stage::media::location::K_TOAST_DURATION_MS;

#[derive(Debug, Clone)]
pub struct HistoryViewUserpicSuggestionViewMedia {
    pub suggested_by_name: String,
    pub has_video: bool,
    pub is_accepted: bool,
}

impl HistoryViewUserpicSuggestionViewMedia {
    pub fn new(suggested_by: impl Into<String>, has_video: bool) -> Self {
        Self {
            suggested_by_name: suggested_by.into(),
            has_video,
            is_accepted: false,
        }
    }

    pub fn accept(&mut self) {
        self.is_accepted = true;
    }
}

impl Default for HistoryViewUserpicSuggestionViewMedia {
    fn default() -> Self {
        Self::new("Alice", false)
    }
}

impl Render for HistoryViewUserpicSuggestionViewMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let text = if self.has_video {
            format!("{} suggested a profile video for you", self.suggested_by_name)
        } else {
            format!("{} suggested a profile photo for you", self.suggested_by_name)
        };

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
                    .text_size(px(13.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(0xFFFFFF))
                    .child(text),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div()
                            .flex_1()
                            .py(px(6.0))
                            .rounded(px(6.0))
                            .bg(if self.is_accepted { rgb(0x4CAF50) } else { rgb(0x2AABEE) })
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(if self.is_accepted { "Photo Set" } else { "Set as Photo" }),
                    ),
            )
    }
}
