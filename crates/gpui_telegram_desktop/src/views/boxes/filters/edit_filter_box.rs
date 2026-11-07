//! Authentic GPUI implementation of Telegram Desktop's Edit Filter Box.
//! Upstream reference: `Telegram/SourceFiles/boxes/filters/edit_filter_box.h:L22-L32`
//! Upstream implementation: `Telegram/SourceFiles/boxes/filters/edit_filter_box.cpp:L65-L120`, `L400-L550`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::sync::Arc;

pub const K_MAX_FILTER_TITLE_LENGTH: usize = 12;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatFilterModel {
    pub id: i32,
    pub title: String,
    pub icon_emoji: String,
    pub color_index: u8,
    pub flags: u32,
    pub included_peers_count: usize,
    pub excluded_peers_count: usize,
}

#[derive(Clone)]
pub struct EditFilterBoxFilters {
    pub filter: ChatFilterModel,
    pub is_new: bool,
    pub is_closed: bool,
    pub on_save: Option<Arc<dyn Fn(ChatFilterModel) + Send + Sync>>,
    pub on_delete: Option<Arc<dyn Fn(i32) + Send + Sync>>,
    pub on_close: Option<Arc<dyn Fn() + Send + Sync>>,
}

pub type EditFilterBox = EditFilterBoxFilters;

impl EditFilterBoxFilters {
    pub fn new(filter: ChatFilterModel, is_new: bool) -> Self {
        Self {
            filter,
            is_new,
            is_closed: false,
            on_save: None,
            on_delete: None,
            on_close: None,
        }
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        let mut t = title.into();
        t.truncate(K_MAX_FILTER_TITLE_LENGTH);
        self.filter.title = t;
    }

    pub fn save(&mut self) {
        if let Some(ref cb) = self.on_save {
            cb(self.filter.clone());
        }
        self.close();
    }

    pub fn close(&mut self) {
        self.is_closed = true;
        if let Some(ref cb) = self.on_close {
            cb();
        }
    }
}

impl Default for EditFilterBoxFilters {
    fn default() -> Self {
        Self::new(ChatFilterModel {
            id: 1,
            title: "Work".into(),
            icon_emoji: "💼".into(),
            color_index: 0,
            flags: 0b0000010100, // e.g. Groups + Channels
            included_peers_count: 14,
            excluded_peers_count: 2,
        }, false)
    }
}

impl Render for EditFilterBoxFilters {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        let title_header = if self.is_new { "New Folder" } else { "Edit Folder" };

        div()
            .id("edit_filter_box")
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
                            .child(title_header),
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
                    .items_center()
                    .gap(px(10.0))
                    .p(px(8.0))
                    .bg(rgb(0xF1F3F5))
                    .rounded(px(8.0))
                    .child(div().text_size(px(24.0)).child(self.filter.icon_emoji.clone()))
                    .child(
                        div()
                            .flex_1()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(0x222222))
                            .child(self.filter.title.clone()),
                    ),
            )
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .p(px(10.0))
                    .rounded(px(8.0))
                    .bg(rgb(0xFAFAFA))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(div().text_size(px(13.0)).font_weight(FontWeight::MEDIUM).text_color(rgb(0x222222)).child("Included Chats"))
                            .child(div().text_size(px(11.0)).text_color(rgb(0x888888)).child(format!("{} chats selected", self.filter.included_peers_count))),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(0x2AABEE))
                            .child("+ Add"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .p(px(10.0))
                    .rounded(px(8.0))
                    .bg(rgb(0xFAFAFA))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(div().text_size(px(13.0)).font_weight(FontWeight::MEDIUM).text_color(rgb(0x222222)).child("Excluded Chats"))
                            .child(div().text_size(px(11.0)).text_color(rgb(0x888888)).child(format!("{} chats excluded", self.filter.excluded_peers_count))),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(0x2AABEE))
                            .child("+ Add"),
                    ),
            )
            .child(
                div()
                    .w_full()
                    .py(px(10.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x2AABEE))
                    .cursor_pointer()
                    .flex()
                    .justify_center()
                    .items_center()
                    .child(
                        div()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child("Save"),
                    ),
            )
            .into_any_element()
    }
}
