//! Chat Preview Popover (Long-press Peek & Context Actions)
//! Faithful upstream C++ extraction:
//! - `history_view_chat_preview.cpp:L30-L100`
//! - `history_view_chat_preview.h:L20-L40`

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatPreviewActionType {
    OpenItem(u64),
    OpenInfo,
    MarkRead,
    MarkUnread,
    Cancel,
}

#[derive(Debug, Clone)]
pub struct HistoryViewChatPreviewView {
    pub peer_id: u64,
    pub title: String,
    pub preview_messages: Vec<(String, String)>, // (sender, text)
    pub is_unread: bool,
    pub is_visible: bool,
}

impl HistoryViewChatPreviewView {
    pub fn new(peer_id: u64, title: String) -> Self {
        Self {
            peer_id,
            title,
            preview_messages: Vec::new(),
            is_unread: false,
            is_visible: true,
        }
    }

    pub fn add_preview_message(&mut self, sender: String, text: String) {
        self.preview_messages.push((sender, text));
    }
}

impl Default for HistoryViewChatPreviewView {
    fn default() -> Self {
        Self::new(0, "Chat Preview".into())
    }
}

impl Render for HistoryViewChatPreviewView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }

        let title = &self.title;
        let msgs = &self.preview_messages;

        div()
            .flex()
            .flex_col()
            .w(px(340.0))
            .max_h(px(480.0))
            .p(px(12.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(16.0))
            .shadow_xl()
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_size(px(15.0))
                    .text_color(rgb(0x222222))
                    .pb(px(8.0))
                    .border_b_1()
                    .border_color(rgb(0xF0F0F0))
                    .child(title.clone())
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .py(px(8.0))
                    .children(msgs.iter().map(|(sender, text)| {
                        div()
                            .flex()
                            .flex_col()
                            .p(px(6.0))
                            .bg(rgba(0x00000005))
                            .rounded(px(8.0))
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0x2AABEE))
                                    .child(sender.clone())
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(rgb(0x333333))
                                    .child(text.clone())
                            )
                    }))
            )
            .child(
                div()
                    .flex()
                    .justify_between()
                    .pt(px(8.0))
                    .border_t_1()
                    .border_color(rgb(0xF0F0F0))
                    .text_size(px(13.0))
                    .text_color(rgb(0x2AABEE))
                    .child(div().child("Open Chat"))
                    .child(div().child(if self.is_unread { "Mark Read" } else { "Mark Unread" }))
            )
            .into_any_element()
    }
}
