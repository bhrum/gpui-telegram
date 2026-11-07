//! Authentic GPUI implementation of Telegram Desktop's Edit Filter Chats List Box.
//! Upstream reference: `Telegram/SourceFiles/boxes/filters/edit_filter_chats_list.h:L29-L90`
//! Upstream implementation: `Telegram/SourceFiles/boxes/filters/edit_filter_chats_list.cpp:L30-L60`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::sync::Arc;

pub const CHAT_FLAG_NEW_CHATS: u32 = 1 << 0;
pub const CHAT_FLAG_EXISTING_CHATS: u32 = 1 << 1;
pub const CHAT_FLAG_CONTACTS: u32 = 1 << 2;
pub const CHAT_FLAG_NON_CONTACTS: u32 = 1 << 3;
pub const CHAT_FLAG_GROUPS: u32 = 1 << 4;
pub const CHAT_FLAG_CHANNELS: u32 = 1 << 5;
pub const CHAT_FLAG_BOTS: u32 = 1 << 6;
pub const CHAT_FLAG_NO_MUTED: u32 = 1 << 7;
pub const CHAT_FLAG_NO_READ: u32 = 1 << 8;
pub const CHAT_FLAG_NO_ARCHIVED: u32 = 1 << 9;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatTypeOption {
    pub flag: u32,
    pub name: String,
    pub icon_emoji: String,
    pub is_selected: bool,
}

#[derive(Clone)]
pub struct EditFilterChatsListFilters {
    pub title: String,
    pub is_exclude: bool,
    pub type_options: Vec<ChatTypeOption>,
    pub selected_peer_ids: Vec<u64>,
    pub is_closed: bool,
    pub on_save: Option<Arc<dyn Fn(u32, Vec<u64>) + Send + Sync>>,
    pub on_close: Option<Arc<dyn Fn() + Send + Sync>>,
}

pub type EditFilterChatsList = EditFilterChatsListFilters;

impl EditFilterChatsListFilters {
    pub fn new(is_exclude: bool) -> Self {
        let title = if is_exclude { "Exclude Chats" } else { "Include Chats" };
        Self {
            title: title.into(),
            is_exclude,
            type_options: vec![
                ChatTypeOption { flag: CHAT_FLAG_CONTACTS, name: "Contacts".into(), icon_emoji: "👤".into(), is_selected: false },
                ChatTypeOption { flag: CHAT_FLAG_NON_CONTACTS, name: "Non-Contacts".into(), icon_emoji: "👥".into(), is_selected: false },
                ChatTypeOption { flag: CHAT_FLAG_GROUPS, name: "Groups".into(), icon_emoji: "💬".into(), is_selected: true },
                ChatTypeOption { flag: CHAT_FLAG_CHANNELS, name: "Channels".into(), icon_emoji: "📢".into(), is_selected: true },
                ChatTypeOption { flag: CHAT_FLAG_BOTS, name: "Bots".into(), icon_emoji: "🤖".into(), is_selected: false },
                ChatTypeOption { flag: CHAT_FLAG_NO_MUTED, name: "Exclude Muted".into(), icon_emoji: "🔕".into(), is_selected: false },
                ChatTypeOption { flag: CHAT_FLAG_NO_READ, name: "Exclude Read".into(), icon_emoji: "📖".into(), is_selected: false },
                ChatTypeOption { flag: CHAT_FLAG_NO_ARCHIVED, name: "Exclude Archived".into(), icon_emoji: "📦".into(), is_selected: true },
            ],
            selected_peer_ids: vec![101, 102],
            is_closed: false,
            on_save: None,
            on_close: None,
        }
    }

    pub fn toggle_flag(&mut self, flag: u32) {
        if let Some(opt) = self.type_options.iter_mut().find(|o| o.flag == flag) {
            opt.is_selected = !opt.is_selected;
        }
    }

    pub fn compute_flags(&self) -> u32 {
        let mut flags = 0;
        for opt in &self.type_options {
            if opt.is_selected {
                flags |= opt.flag;
            }
        }
        flags
    }

    pub fn close(&mut self) {
        self.is_closed = true;
        if let Some(ref cb) = self.on_close {
            cb();
        }
    }
}

impl Default for EditFilterChatsListFilters {
    fn default() -> Self {
        Self::new(false)
    }
}

impl Render for EditFilterChatsListFilters {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        div()
            .id("edit_filter_chats_list")
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
                            .child(self.title.clone()),
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
                    .gap(px(4.0))
                    .children(self.type_options.iter().map(|opt| {
                        div()
                            .flex()
                            .justify_between()
                            .items_center()
                            .p(px(8.0))
                            .rounded(px(6.0))
                            .bg(if opt.is_selected { rgb(0xEEF8FF) } else { rgb(0xFAFAFA) })
                            .cursor_pointer()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(8.0))
                                    .child(div().text_size(px(18.0)).child(opt.icon_emoji.clone()))
                                    .child(div().text_size(px(13.0)).font_weight(FontWeight::MEDIUM).text_color(rgb(0x222222)).child(opt.name.clone())),
                            )
                            .child(
                                div()
                                    .size(px(18.0))
                                    .rounded(px(4.0))
                                    .border_1()
                                    .border_color(if opt.is_selected { rgb(0x2AABEE) } else { rgb(0xCCCCCC) })
                                    .bg(if opt.is_selected { rgb(0x2AABEE) } else { rgb(0xFFFFFF) })
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(if opt.is_selected { div().text_size(px(12.0)).text_color(rgb(0xFFFFFF)).child("✓") } else { div() }),
                            )
                    })),
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
                            .child("Done"),
                    ),
            )
            .into_any_element()
    }
}
