//! Genuine History View Community Added Component (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_community_added.h:L15-L25`
//! - `Telegram/SourceFiles/history/view/media/history_view_community_added.cpp:L34-L60`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;

#[derive(Debug, Clone)]
pub struct HistoryViewCommunityAddedViewMedia {
    pub community_title: String,
    pub members_count: u32,
    pub has_photo: bool,
}

impl HistoryViewCommunityAddedViewMedia {
    pub fn new(title: impl Into<String>, members_count: u32) -> Self {
        Self {
            community_title: title.into(),
            members_count,
            has_photo: false,
        }
    }
}

impl Default for HistoryViewCommunityAddedViewMedia {
    fn default() -> Self {
        Self::new("Rust Community", 450)
    }
}

impl Render for HistoryViewCommunityAddedViewMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_between()
            .w(px(280.0))
            .p(px(10.0))
            .rounded(px(10.0))
            .bg(rgb(0x182533))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child(self.community_title.clone()),
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(0x8A9AA8))
                            .child(format!("{} members", self.members_count)),
                    ),
            )
            .child(
                div()
                    .px(px(8.0))
                    .py(px(4.0))
                    .rounded(px(6.0))
                    .bg(rgb(0x2AABEE))
                    .text_size(px(11.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xFFFFFF))
                    .child("Join"),
            )
    }
}
