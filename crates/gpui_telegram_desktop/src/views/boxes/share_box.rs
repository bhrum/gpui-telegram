//! Authentic GPUI implementation of Telegram Desktop's Share & Forward Box.
//! Upstream reference: `Telegram/SourceFiles/boxes/share_box.h:L63-L140` and `share_box.cpp:L74-L200`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::sync::Arc;

/// Recipient peer item for multi-share list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShareRecipient {
    pub peer_id: i64,
    pub name: String,
    pub username: Option<String>,
    pub is_selected: bool,
}

/// Forward options mirroring upstream `Data::ForwardOptions` (`share_box.h:L49-L51`, `L134-L138`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ShareForwardOptions {
    pub hide_sender_names: bool,
    pub hide_captions: bool,
}

/// ShareBoxView view component (also aliased as ShareBox).
/// Cites upstream `ShareBox::Descriptor` (`share_box.h:L123-L140`).
#[derive(Clone)]
pub struct ShareBoxView {
    pub share_link: String,
    pub share_text: Option<String>,
    pub comment: String,
    pub recipients: Vec<ShareRecipient>,
    pub forward_options: ShareForwardOptions,
    pub is_closed: bool,
    pub on_submit: Option<Arc<dyn Fn(Vec<i64>, &str) + Send + Sync>>,
}

pub type ShareBox = ShareBoxView;

impl ShareBoxView {
    pub fn new(share_link: String) -> Self {
        Self {
            share_link,
            share_text: None,
            comment: String::new(),
            recipients: Vec::new(),
            forward_options: ShareForwardOptions::default(),
            is_closed: false,
            on_submit: None,
        }
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.share_text = Some(text.into());
        self
    }

    pub fn with_recipients(mut self, recipients: Vec<ShareRecipient>) -> Self {
        self.recipients = recipients;
        self
    }

    pub fn toggle_recipient(&mut self, peer_id: i64) {
        if let Some(r) = self.recipients.iter_mut().find(|r| r.peer_id == peer_id) {
            r.is_selected = !r.is_selected;
        }
    }

    pub fn selected_peer_ids(&self) -> Vec<i64> {
        self.recipients
            .iter()
            .filter(|r| r.is_selected)
            .map(|r| r.peer_id)
            .collect()
    }

    pub fn selected_count(&self) -> usize {
        self.recipients.iter().filter(|r| r.is_selected).count()
    }

    pub fn set_comment(&mut self, text: impl Into<String>) {
        self.comment = text.into();
    }

    pub fn copy_link(&self) -> String {
        self.share_link.clone()
    }

    pub fn submit(&mut self) {
        if self.is_closed {
            return;
        }
        self.is_closed = true;
        let selected = self.selected_peer_ids();
        if let Some(ref cb) = self.on_submit {
            cb(selected, &self.comment);
        }
    }
}

impl Default for ShareBoxView {
    fn default() -> Self {
        Self::new("https://t.me/example".to_string())
    }
}

impl Render for ShareBoxView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        let mut card = div()
            .id("share_box_card")
            .w(px(380.0))
            .p(px(24.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(10.0))
            .shadow_lg()
            .flex()
            .flex_col()
            .gap(px(14.0))
            .child(
                div()
                    .text_size(px(18.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x222222))
                    .child("Share"),
            );

        // Link copy preview
        if !self.share_link.is_empty() {
            card = card.child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .p(px(8.0))
                    .bg(rgb(0xF4F4F5))
                    .rounded(px(6.0))
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(rgb(0x5288C1))
                            .child(self.share_link.clone()),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x5288C1))
                            .cursor_pointer()
                            .child("Copy"),
                    ),
            );
        }

        // Recipients list
        let mut list = div().flex().flex_col().gap(px(4.0));
        for r in &self.recipients {
            let row = div()
                .flex()
                .items_center()
                .gap(px(10.0))
                .p(px(6.0))
                .rounded(px(4.0))
                .cursor_pointer()
                .hover(|s| s.bg(rgba(0x0000000A)))
                .child(
                    div()
                        .size(px(16.0))
                        .rounded(px(3.0))
                        .border_1()
                        .border_color(if r.is_selected {
                            rgb(0x5288C1)
                        } else {
                            rgb(0xCCCCCC)
                        })
                        .bg(if r.is_selected {
                            rgb(0x5288C1)
                        } else {
                            rgb(0xFFFFFF)
                        })
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(if r.is_selected {
                            div().text_size(px(10.0)).text_color(rgb(0xFFFFFF)).child("✓")
                        } else {
                            div()
                        }),
                )
                .child(
                    div()
                        .text_size(px(14.0))
                        .text_color(rgb(0x333333))
                        .child(r.name.clone()),
                );

            list = list.child(row);
        }
        card = card.child(list);

        // Footer buttons: Cancel and Share/Send
        let selected = self.selected_count();
        let send_text = if selected > 0 {
            format!("Send ({})", selected)
        } else {
            "Send".to_string()
        };

        let btn_row = div()
            .flex()
            .justify_end()
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
                    .text_color(rgb(0x5288C1))
                    .child(send_text),
            );

        card.child(btn_row).into_any_element()
    }
}
