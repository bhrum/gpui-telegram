//! Genuine History View Similar Channels Component (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_similar_channels.h:L19-L102`
//! - `Telegram/SourceFiles/history/view/media/history_view_similar_channels.cpp:L79-L105`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;

#[derive(Debug, Clone)]
pub struct SimilarChannelItem {
    pub channel_id: u64,
    pub title: String,
    pub subscribers_count: u32,
    pub username: Option<String>,
}

#[derive(Debug, Clone)]
pub struct HistoryViewSimilarChannelsViewMedia {
    pub channels: Vec<SimilarChannelItem>,
    pub is_toggled: bool,
}

impl HistoryViewSimilarChannelsViewMedia {
    pub fn new() -> Self {
        Self {
            channels: Vec::new(),
            is_toggled: true,
        }
    }

    pub fn add_channel(&mut self, channel_id: u64, title: impl Into<String>, subscribers: u32) {
        self.channels.push(SimilarChannelItem {
            channel_id,
            title: title.into(),
            subscribers_count: subscribers,
            username: None,
        });
    }

    pub fn toggle(&mut self) {
        self.is_toggled = !self.is_toggled;
    }
}

impl Default for HistoryViewSimilarChannelsViewMedia {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewSimilarChannelsViewMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w(px(340.0))
            .p(px(10.0))
            .rounded(px(10.0))
            .bg(rgb(0x182533))
            .gap(px(6.0))
            .child(
                div()
                    .text_size(px(13.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x2AABEE))
                    .child("Similar Channels"),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .children(self.channels.iter().map(|ch| {
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(rgb(0xFFFFFF))
                                    .child(ch.title.clone()),
                            )
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(rgb(0x8A9AA8))
                                    .child(format!("{} subs", ch.subscribers_count)),
                            )
                    })),
            )
    }
}
