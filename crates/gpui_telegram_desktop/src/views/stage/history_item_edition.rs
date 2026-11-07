//! GPUI Telegram History Item Edition View Component
//!
//! Citations:
//! - `Telegram/SourceFiles/history/history_item_edition.h:L26-L56` (HistoryMessageEdition)
//! - `Telegram/SourceFiles/history/history_item_edition.cpp:L14-L51`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui_telegram_client::history::item::HistoryMessageEdition as ClientHistoryMessageEdition;

/// Upstream `history_item_edition.h:L26-L56`: Message Edition State & Diff Tracker
#[derive(Debug, Clone, Default)]
pub struct HistoryItemEdition {
    pub inner: ClientHistoryMessageEdition,
    pub is_editing: bool,
    pub draft_text: String,
}

impl HistoryItemEdition {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_edit_date(mut self, edit_date: i64) -> Self {
        self.inner.edit_date = edit_date;
        self
    }

    pub fn start_editing(&mut self, current_text: &str) {
        self.is_editing = true;
        self.draft_text = current_text.to_string();
    }

    pub fn finish_editing(&mut self, new_text: &str, now: i64) {
        self.is_editing = false;
        self.inner.text = new_text.to_string();
        self.inner.edit_date = now;
    }

    pub fn cancel_editing(&mut self) {
        self.is_editing = false;
        self.draft_text.clear();
    }
}

impl Render for HistoryItemEdition {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_editing {
            return div();
        }

        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(8.0))
            .bg(rgb(0xFAFAFA))
            .rounded(px(6.0))
            .border_1()
            .border_color(rgb(0x2AABEE))
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x2AABEE))
                    .child("Editing Message..."),
            )
            .child(
                div()
                    .mt(px(4.0))
                    .text_size(px(14.0))
                    .child(self.draft_text.clone()),
            )
    }
}
