//! Genuine History View Custom Emoji Component (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_custom_emoji.h:L25-L104`
//! - `Telegram/SourceFiles/history/view/media/history_view_custom_emoji.cpp:L38-L56`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use crate::views::stage::media::poll::custom_emoji_dimension_scale;

#[derive(Debug, Clone)]
pub struct CustomEmojiItem {
    pub document_id: u64,
    pub alt_text: String,
}

#[derive(Debug, Clone)]
pub struct HistoryViewCustomEmojiViewMedia {
    pub emoji_items: Vec<CustomEmojiItem>,
}

impl HistoryViewCustomEmojiViewMedia {
    pub fn new() -> Self {
        Self {
            emoji_items: Vec::new(),
        }
    }

    pub fn add_emoji(&mut self, document_id: u64, alt: impl Into<String>) {
        self.emoji_items.push(CustomEmojiItem {
            document_id,
            alt_text: alt.into(),
        });
    }

    pub fn scale_factor(&self) -> f32 {
        custom_emoji_dimension_scale(self.emoji_items.len())
    }
}

impl Default for HistoryViewCustomEmojiViewMedia {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewCustomEmojiViewMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let scale = self.scale_factor();
        let size_px = (32.0 * scale).max(16.0);

        div()
            .flex()
            .items_center()
            .gap(px(4.0))
            .children(self.emoji_items.iter().map(|item| {
                div()
                    .w(px(size_px))
                    .h(px(size_px))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(size_px * 0.8))
                    .child(item.alt_text.clone())
            }))
    }
}
