//! Authentic GPUI implementation of Telegram Desktop's Message Delete Confirmation Box.
//! Upstream reference: `Telegram/SourceFiles/boxes/delete_messages_box.h:L26-L89` and `delete_messages_box.cpp:L43-L240`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::sync::Arc;

/// Lightweight message deletion confirmation dialog.
/// Cites upstream `delete_messages_box.cpp:L181-L218` (revoke checkbox & attention button).
#[derive(Clone)]
pub struct DeleteBoxView {
    pub message_count: usize,
    pub peer_name: Option<String>,
    pub can_delete_for_all: bool,
    pub delete_for_everyone: bool,
    pub is_wipe_history: bool,
    pub is_closed: bool,
    pub on_confirm: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    pub on_cancel: Option<Arc<dyn Fn() + Send + Sync>>,
}

pub type DeleteBox = DeleteBoxView;

impl DeleteBoxView {
    pub fn new(message_count: usize, can_delete_for_all: bool) -> Self {
        Self {
            message_count,
            peer_name: None,
            can_delete_for_all,
            delete_for_everyone: can_delete_for_all,
            is_wipe_history: false,
            is_closed: false,
            on_confirm: None,
            on_cancel: None,
        }
    }

    pub fn with_peer_name(mut self, name: impl Into<String>) -> Self {
        self.peer_name = Some(name.into());
        self
    }

    pub fn with_wipe_history(mut self, wipe: bool) -> Self {
        self.is_wipe_history = wipe;
        self
    }

    pub fn toggle_delete_for_everyone(&mut self) {
        if self.can_delete_for_all {
            self.delete_for_everyone = !self.delete_for_everyone;
        }
    }

    pub fn confirm(&mut self) {
        if self.is_closed {
            return;
        }
        self.is_closed = true;
        if let Some(ref cb) = self.on_confirm {
            cb(self.delete_for_everyone);
        }
    }

    pub fn cancel(&mut self) {
        if self.is_closed {
            return;
        }
        self.is_closed = true;
        if let Some(ref cb) = self.on_cancel {
            cb();
        }
    }
}

impl Render for DeleteBoxView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        let title = if self.is_wipe_history {
            "Clear History?".to_string()
        } else if self.message_count == 1 {
            "Delete Message?".to_string()
        } else {
            format!("Delete {} messages?", self.message_count)
        };

        let body = if self.is_wipe_history {
            "Are you sure you want to clear chat history? This action cannot be undone.".to_string()
        } else if self.message_count == 1 {
            "Are you sure you want to delete this message?".to_string()
        } else {
            format!("Are you sure you want to delete {} messages?", self.message_count)
        };

        let mut card = div()
            .id("delete_box_card")
            .w(px(340.0))
            .p(px(24.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(10.0))
            .shadow_lg()
            .flex()
            .flex_col()
            .gap(px(16.0));

        // Title
        card = card.child(
            div()
                .text_size(px(18.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(0x222222))
                .child(title),
        );

        // Body prompt
        card = card.child(
            div()
                .text_size(px(14.0))
                .text_color(rgb(0x555555))
                .child(body),
        );

        // Revoke checkbox ("Also delete for {PeerName}"): `delete_messages_box.cpp:L197-L218`
        if self.can_delete_for_all {
            let label = if let Some(ref peer) = self.peer_name {
                format!("Also delete for {}", peer)
            } else {
                "Delete for everyone".to_string()
            };

            card = card.child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .cursor_pointer()
                    .child(
                        div()
                            .size(px(18.0))
                            .rounded(px(4.0))
                            .border_2()
                            .border_color(if self.delete_for_everyone {
                                rgb(0x5288C1)
                            } else {
                                rgb(0xCCCCCC)
                            })
                            .bg(if self.delete_for_everyone {
                                rgb(0x5288C1)
                            } else {
                                rgb(0xFFFFFF)
                            })
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(if self.delete_for_everyone {
                                div().text_size(px(12.0)).text_color(rgb(0xFFFFFF)).child("✓")
                            } else {
                                div()
                            }),
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(rgb(0x333333))
                            .child(label),
                    ),
            );
        }

        // Action buttons: Cancel and Delete (attention red button)
        let btn_row = div()
            .flex()
            .justify_end()
            .items_center()
            .gap(px(8.0))
            .child(
                div()
                    .px(px(16.0))
                    .py(px(8.0))
                    .rounded(px(6.0))
                    .cursor_pointer()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgb(0x707579))
                    .child("Cancel"),
            )
            .child(
                div()
                    .px(px(16.0))
                    .py(px(8.0))
                    .rounded(px(6.0))
                    .cursor_pointer()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(0xE53935))
                    .child("Delete"),
            );

        card.child(btn_row).into_any_element()
    }
}
