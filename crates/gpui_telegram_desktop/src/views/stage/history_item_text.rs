//! GPUI Telegram History Item Text View Component
//!
//! Citations:
//! - `Telegram/SourceFiles/history/history_item_text.h:L19-L46`
//! - `Telegram/SourceFiles/history/history_item_text.cpp:L30-L120` (kSelectedCopyReplyPreviewLimit = 64)

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui_telegram_client::history::selection::K_SELECTED_COPY_REPLY_PREVIEW_LIMIT;

/// Upstream `history_item_text.h:L19-L46`: Text Rendering & Formatting Component
#[derive(Debug, Clone, Default)]
pub struct HistoryItemText {
    pub raw_text: String,
    pub formatted_text: String,
    pub is_empty: bool,
}

impl HistoryItemText {
    pub fn new(text: impl Into<String>) -> Self {
        let raw = text.into();
        let is_empty = raw.is_empty();
        Self {
            raw_text: raw.clone(),
            formatted_text: raw,
            is_empty,
        }
    }

    /// Upstream `history_item_text.cpp:L32`: Truncates reply preview text to 64 chars
    pub fn reply_preview(&self) -> String {
        if self.raw_text.len() <= K_SELECTED_COPY_REPLY_PREVIEW_LIMIT {
            self.raw_text.clone()
        } else {
            let mut s = self.raw_text[..K_SELECTED_COPY_REPLY_PREVIEW_LIMIT].to_string();
            s.push('…');
            s
        }
    }
}

impl Render for HistoryItemText {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_size(px(14.0))
            .line_height(relative(1.3))
            .text_color(rgb(0x111111))
            .child(self.formatted_text.clone())
    }
}
