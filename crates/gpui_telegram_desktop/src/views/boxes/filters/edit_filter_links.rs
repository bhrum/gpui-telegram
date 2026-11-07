//! Authentic GPUI implementation of Telegram Desktop's Edit Filter Links Box.
//! Upstream reference: `Telegram/SourceFiles/boxes/filters/edit_filter_links.h:L29-L64`
//! Upstream implementation: `Telegram/SourceFiles/boxes/filters/edit_filter_links.cpp:L45-L60`, `L320-L450`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::sync::Arc;

pub const K_MAX_LINK_TITLE_LENGTH: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilterShareLinkItem {
    pub url: String,
    pub title: String,
    pub chat_count: usize,
}

#[derive(Clone)]
pub struct EditFilterLinksFilters {
    pub filter_id: i32,
    pub filter_name: String,
    pub links: Vec<FilterShareLinkItem>,
    pub is_closed: bool,
    pub on_create_link: Option<Arc<dyn Fn(String) + Send + Sync>>,
    pub on_close: Option<Arc<dyn Fn() + Send + Sync>>,
}

pub type EditFilterLinks = EditFilterLinksFilters;

impl EditFilterLinksFilters {
    pub fn new(filter_id: i32, filter_name: impl Into<String>) -> Self {
        Self {
            filter_id,
            filter_name: filter_name.into(),
            links: vec![
                FilterShareLinkItem {
                    url: "t.me/addlist/abCDeFg123".into(),
                    title: "Public Share Link".into(),
                    chat_count: 8,
                },
            ],
            is_closed: false,
            on_create_link: None,
            on_close: None,
        }
    }

    pub fn close(&mut self) {
        self.is_closed = true;
        if let Some(ref cb) = self.on_close {
            cb();
        }
    }
}

impl Default for EditFilterLinksFilters {
    fn default() -> Self {
        Self::new(1, "Crypto News")
    }
}

impl Render for EditFilterLinksFilters {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        div()
            .id("edit_filter_links")
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
                            .child(format!("Share Folder: {}", self.filter_name)),
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
                    .text_size(px(13.0))
                    .text_color(rgb(0x666666))
                    .child("Anyone with this link can easily join all public channels and groups in this folder."),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .children(self.links.iter().map(|link| {
                        div()
                            .p(px(10.0))
                            .rounded(px(8.0))
                            .bg(rgb(0xF8F9FA))
                            .border_1()
                            .border_color(rgb(0xEEEEEE))
                            .flex()
                            .flex_col()
                            .gap(px(4.0))
                            .child(
                                div()
                                    .flex()
                                    .justify_between()
                                    .child(div().text_size(px(13.0)).font_weight(FontWeight::MEDIUM).text_color(rgb(0x222222)).child(link.title.clone()))
                                    .child(div().text_size(px(11.0)).text_color(rgb(0x888888)).child(format!("{} chats", link.chat_count))),
                            )
                            .child(div().text_size(px(12.0)).text_color(rgb(0x2AABEE)).child(link.url.clone()))
                    })),
            )
            .child(
                div()
                    .w_full()
                    .py(px(9.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x2AABEE))
                    .cursor_pointer()
                    .flex()
                    .justify_center()
                    .items_center()
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child("Create an Invite Link"),
                    ),
            )
            .into_any_element()
    }
}
