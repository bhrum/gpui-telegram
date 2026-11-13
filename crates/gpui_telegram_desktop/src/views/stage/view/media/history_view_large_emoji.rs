//! Genuine History View Large Emoji Component (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_large_emoji.h:L25-L67`
//! - `Telegram/SourceFiles/history/view/media/history_view_large_emoji.cpp:L30-L51`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;

pub const K_ISOLATED_EMOJI_LIMIT: usize = 3;

#[derive(Debug, Clone)]
pub struct HistoryViewLargeEmojiViewMedia {
    pub emoji_list: Vec<String>,
}

impl HistoryViewLargeEmojiViewMedia {
    pub fn new(emoji: impl Into<String>) -> Self {
        Self {
            emoji_list: vec![emoji.into()],
        }
    }

    pub fn with_emojis(emojis: Vec<String>) -> Self {
        let mut list = emojis;
        list.truncate(K_ISOLATED_EMOJI_LIMIT);
        Self { emoji_list: list }
    }
}

impl Default for HistoryViewLargeEmojiViewMedia {
    fn default() -> Self {
        Self::new("👍")
    }
}

impl Render for HistoryViewLargeEmojiViewMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let size_px = match self.emoji_list.len() {
            1 => 48.0,
            2 => 38.0,
            _ => 32.0,
        };

        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .p(px(4.0))
            .children(self.emoji_list.iter().map(|e| {
                div()
                    .w(px(size_px))
                    .h(px(size_px))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(size_px * 0.85))
                    .child(e.clone())
            }))
    }
}
