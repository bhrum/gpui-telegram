//! Authentic GPUI implementation of Telegram Desktop's Filter Chats Preview.
//! Upstream reference: `Telegram/SourceFiles/boxes/filters/edit_filter_chats_preview.h:L20-L65`
//! Upstream implementation: `Telegram/SourceFiles/boxes/filters/edit_filter_chats_preview.cpp:L20-L60`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilterChipItem {
    pub id: u64,
    pub title: String,
    pub is_type_flag: bool,
    pub icon_emoji: String,
}

#[derive(Clone)]
pub struct EditFilterChatsPreviewFilters {
    pub chips: Vec<FilterChipItem>,
    pub is_closed: bool,
    pub on_remove: Option<Arc<dyn Fn(u64, bool) + Send + Sync>>,
}

pub type EditFilterChatsPreview = EditFilterChatsPreviewFilters;

impl EditFilterChatsPreviewFilters {
    pub fn new() -> Self {
        Self {
            chips: vec![
                FilterChipItem { id: 1, title: "Groups".into(), is_type_flag: true, icon_emoji: "💬".into() },
                FilterChipItem { id: 2, title: "Channels".into(), is_type_flag: true, icon_emoji: "📢".into() },
                FilterChipItem { id: 101, title: "Telegram News".into(), is_type_flag: false, icon_emoji: "📰".into() },
            ],
            is_closed: false,
            on_remove: None,
        }
    }

    pub fn remove_chip(&mut self, id: u64) {
        if let Some(pos) = self.chips.iter().position(|c| c.id == id) {
            let removed = self.chips.remove(pos);
            if let Some(ref cb) = self.on_remove {
                cb(removed.id, removed.is_type_flag);
            }
        }
    }
}

impl Default for EditFilterChatsPreviewFilters {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for EditFilterChatsPreviewFilters {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        div()
            .id("edit_filter_chats_preview")
            .flex()
            .flex_wrap()
            .gap(px(6.0))
            .children(self.chips.iter().map(|chip| {
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .px(px(8.0))
                    .py(px(4.0))
                    .rounded(px(14.0))
                    .bg(rgb(0xEEF3F8))
                    .child(div().text_size(px(12.0)).child(chip.icon_emoji.clone()))
                    .child(div().text_size(px(12.0)).font_weight(FontWeight::MEDIUM).text_color(rgb(0x222222)).child(chip.title.clone()))
                    .child(
                        div()
                            .cursor_pointer()
                            .text_size(px(11.0))
                            .text_color(rgb(0x888888))
                            .child("✕"),
                    )
            }))
            .into_any_element()
    }
}
