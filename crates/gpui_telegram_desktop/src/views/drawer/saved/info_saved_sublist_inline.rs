//! Authentic GPUI implementation of Telegram Saved Messages Inline Sublist.
//! Source: `Telegram/SourceFiles/info/saved/info_saved_sublist_inline.cpp:L1-L623`, `info_saved_sublist_inline.h:L1-L43`
//! Subsystem: info | Feature ID: TCG-N38 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedSublistMessageItem {
    pub message_id: u64,
    pub original_peer_id: u64,
    pub sender_name: String,
    pub text_preview: String,
    pub date: i64,
    pub tag: Option<String>,
}

#[derive(Debug, Clone)]
pub struct InfoSavedSublistInlineSaved {
    pub peer_id: u64,
    pub items: Vec<SavedSublistMessageItem>,
    pub selected_tag: Option<String>,
    pub active_message_id: Option<u64>,
}

impl InfoSavedSublistInlineSaved {
    pub fn new() -> Self {
        Self {
            peer_id: 0,
            items: Vec::new(),
            selected_tag: None,
            active_message_id: None,
        }
    }

    pub fn with_items(items: Vec<SavedSublistMessageItem>) -> Self {
        Self {
            peer_id: 0,
            items,
            selected_tag: None,
            active_message_id: None,
        }
    }

    pub fn filter_by_tag(&mut self, tag: Option<String>) {
        self.selected_tag = tag;
    }

    pub fn jump_to_message(&mut self, message_id: u64) {
        self.active_message_id = Some(message_id);
    }

    pub fn filtered_items(&self) -> Vec<&SavedSublistMessageItem> {
        if let Some(ref tag) = self.selected_tag {
            self.items.iter().filter(|i| i.tag.as_ref() == Some(tag)).collect()
        } else {
            self.items.iter().collect()
        }
    }
}

impl Default for InfoSavedSublistInlineSaved {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoSavedSublistInlineSaved {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let items = self.filtered_items();
        let is_empty = items.is_empty();

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(8.0))
            .when(is_empty, |this| {
                this.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .py(px(24.0))
                        .text_xs()
                        .text_color(rgb(0x707579))
                        .child("No saved messages in this sublist")
                )
            })
            .when(!is_empty, |this| {
                let mut col = div().flex().flex_col().gap(px(4.0));
                for item in items {
                    let is_active = self.active_message_id == Some(item.message_id);
                    col = col.child(
                        div()
                            .flex()
                            .flex_col()
                            .p(px(8.0))
                            .rounded(px(6.0))
                            .bg(if is_active { rgb(0x242f3d) } else { rgb(0x17212b) })
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x5288c1))
                                            .child(item.sender_name.clone())
                                    )
                                    .when(item.tag.is_some(), |h| {
                                        let tag_str = item.tag.clone().unwrap();
                                        h.child(
                                            div()
                                                .px(px(6.0))
                                                .py(px(2.0))
                                                .rounded(px(8.0))
                                                .bg(rgb(0x2b5278))
                                                .text_xs()
                                                .child(tag_str)
                                        )
                                    })
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0xffffff))
                                    .child(item.text_preview.clone())
                            )
                    );
                }
                this.child(col)
            })
    }
}
