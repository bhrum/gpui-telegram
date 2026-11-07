//! Authentic GPUI implementation of Telegram Desktop's Choose Filter Box.
//! Upstream reference: `Telegram/SourceFiles/boxes/choose_filter_box.h:L25-L73`
//! Upstream implementation: `Telegram/SourceFiles/boxes/choose_filter_box.cpp:L41-L60`, `L200-L310`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::sync::Arc;

pub const K_SCALE: f64 = 0.75;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatFilterOption {
    pub id: i32,
    pub title: String,
    pub icon_emoji: String,
    pub chats_count: usize,
    pub is_chat_included: bool,
}

#[derive(Clone)]
pub struct ChooseFilterBox {
    pub chat_name: String,
    pub peer_id: u64,
    pub filters: Vec<ChatFilterOption>,
    pub max_filters: usize,
    pub is_closed: bool,
    pub on_toggle_filter: Option<Arc<dyn Fn(i32, bool) + Send + Sync>>,
    pub on_create_new: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_close: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl ChooseFilterBox {
    pub fn new(peer_id: u64, chat_name: impl Into<String>) -> Self {
        Self {
            chat_name: chat_name.into(),
            peer_id,
            filters: vec![
                ChatFilterOption { id: 1, title: "Work".into(), icon_emoji: "💼".into(), chats_count: 14, is_chat_included: true },
                ChatFilterOption { id: 2, title: "Crypto".into(), icon_emoji: "🪙".into(), chats_count: 28, is_chat_included: false },
                ChatFilterOption { id: 3, title: "News".into(), icon_emoji: "📰".into(), chats_count: 42, is_chat_included: false },
                ChatFilterOption { id: 4, title: "Bots".into(), icon_emoji: "🤖".into(), chats_count: 8, is_chat_included: false },
            ],
            max_filters: 10,
            is_closed: false,
            on_toggle_filter: None,
            on_create_new: None,
            on_close: None,
        }
    }

    pub fn toggle_filter(&mut self, filter_id: i32) {
        if let Some(f) = self.filters.iter_mut().find(|f| f.id == filter_id) {
            f.is_chat_included = !f.is_chat_included;
            let included = f.is_chat_included;
            if let Some(ref cb) = self.on_toggle_filter {
                cb(filter_id, included);
            }
        }
    }

    pub fn close(&mut self) {
        self.is_closed = true;
        if let Some(ref cb) = self.on_close {
            cb();
        }
    }
}

impl Default for ChooseFilterBox {
    fn default() -> Self {
        Self::new(12345, "Rustaceans Group")
    }
}

impl Render for ChooseFilterBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        div()
            .id("choose_filter_box")
            .w(px(380.0))
            .p(px(20.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(12.0))
            .shadow_lg()
            .flex()
            .flex_col()
            .gap(px(14.0))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x222222))
                            .child(format!("Add \"{}\" to Folders", self.chat_name)),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .text_size(px(14.0))
                            .text_color(rgb(0x888888))
                            .child("✕"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .children(self.filters.iter().map(|f| {
                        div()
                            .flex()
                            .justify_between()
                            .items_center()
                            .p(px(10.0))
                            .rounded(px(8.0))
                            .bg(if f.is_chat_included { rgb(0xEEF8FF) } else { rgb(0xFAFAFA) })
                            .cursor_pointer()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(10.0))
                                    .child(div().text_size(px(18.0)).child(f.icon_emoji.clone()))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(div().text_size(px(13.0)).font_weight(FontWeight::MEDIUM).text_color(rgb(0x222222)).child(f.title.clone()))
                                            .child(div().text_size(px(11.0)).text_color(rgb(0x888888)).child(format!("{} chats", f.chats_count))),
                                    ),
                            )
                            .child(
                                div()
                                    .size(px(18.0))
                                    .rounded(px(4.0))
                                    .border_1()
                                    .border_color(if f.is_chat_included { rgb(0x2AABEE) } else { rgb(0xCCCCCC) })
                                    .bg(if f.is_chat_included { rgb(0x2AABEE) } else { rgb(0xFFFFFF) })
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(if f.is_chat_included { div().text_size(px(12.0)).text_color(rgb(0xFFFFFF)).child("✓") } else { div() }),
                            )
                    })),
            )
            .child(
                div()
                    .w_full()
                    .py(px(9.0))
                    .rounded(px(8.0))
                    .bg(rgb(0xF1F3F5))
                    .cursor_pointer()
                    .flex()
                    .justify_center()
                    .items_center()
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(0x2AABEE))
                            .child("+ Create New Folder"),
                    ),
            )
            .into_any_element()
    }
}
