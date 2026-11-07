//! Genuine History View Story Mention Component (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_story_mention.h:L20-L69`
//! - `Telegram/SourceFiles/history/view/media/history_view_story_mention.cpp:L44-L60`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use crate::views::stage::media::gift::K_READ_OUTLINE_ALPHA;

#[derive(Debug, Clone)]
pub struct HistoryViewStoryMentionViewMedia {
    pub story_id: u64,
    pub author_name: String,
    pub is_unread: bool,
}

impl HistoryViewStoryMentionViewMedia {
    pub fn new(story_id: u64, author: impl Into<String>, is_unread: bool) -> Self {
        Self {
            story_id,
            author_name: author.into(),
            is_unread,
        }
    }
}

impl Default for HistoryViewStoryMentionViewMedia {
    fn default() -> Self {
        Self::new(101, "Alice", true)
    }
}

impl Render for HistoryViewStoryMentionViewMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap(px(10.0))
            .w(px(260.0))
            .p(px(8.0))
            .rounded(px(10.0))
            .bg(rgb(0x182533))
            // Story Ring
            .child(
                div()
                    .w(px(40.0))
                    .h(px(40.0))
                    .rounded_full()
                    .border_2()
                    .border_color(if self.is_unread { rgb(0x2AABEE) } else { rgba(0xFFFFFF33) })
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(14.0))
                    .child("📖"),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child("Mentioned in Story"),
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(0x8A9AA8))
                            .child(format!("by {}", self.author_name)),
                    ),
            )
    }
}
