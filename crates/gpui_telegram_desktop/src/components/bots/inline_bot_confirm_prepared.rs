//! InlineBotConfirmPrepared: Confirmation modal dialog for prepared inline messages.
//!
//! Upstream reference:
//! - `Telegram/SourceFiles/inline_bots/inline_bot_confirm_prepared.h:L24-L30`
//! - `Telegram/SourceFiles/inline_bots/inline_bot_confirm_prepared.cpp:L30-L80`
#![allow(dead_code)]

use gpui::*;

/// Prepared inline message presentation model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedMessageInfo {
    pub bot_name: String,
    pub preview_text: String,
    pub has_media: bool,
    pub target_peer_id: u64,
    pub target_thread_id: Option<i32>,
}

/// Authentic InlineBotConfirmPrepared component state.
pub struct InlineBotConfirmPrepared {
    pub message_info: Option<PreparedMessageInfo>,
    pub is_presented: bool,
    pub confirmed: bool,
    pub updated_at: i64,
}

impl InlineBotConfirmPrepared {
    pub fn new() -> Self {
        Self {
            message_info: None,
            is_presented: false,
            confirmed: false,
            updated_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn present(&mut self, info: PreparedMessageInfo) {
        self.message_info = Some(info);
        self.is_presented = true;
        self.confirmed = false;
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn confirm(&mut self) -> Option<PreparedMessageInfo> {
        self.confirmed = true;
        self.is_presented = false;
        self.updated_at = chrono::Utc::now().timestamp();
        self.message_info.take()
    }

    pub fn cancel(&mut self) {
        self.is_presented = false;
        self.message_info = None;
        self.updated_at = chrono::Utc::now().timestamp();
    }
}

impl Default for InlineBotConfirmPrepared {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InlineBotConfirmPrepared {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_presented {
            return div();
        }

        let info = self.message_info.as_ref();
        let bot_name = info.map(|i| i.bot_name.as_str()).unwrap_or("Bot");
        let preview = info.map(|i| i.preview_text.as_str()).unwrap_or("");

        div()
            .flex()
            .flex_col()
            .p(px(16.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(8.0))
            .border_1()
            .border_color(rgb(0xDDDDDD))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_size(px(14.0))
                    .child(format!("Send message from {}?", bot_name))
            )
            .child(
                div()
                    .mt(px(8.0))
                    .p(px(8.0))
                    .bg(rgb(0xF8F8F8))
                    .rounded(px(4.0))
                    .text_size(px(12.0))
                    .text_color(rgb(0x444444))
                    .child(preview.to_string())
            )
            .child(
                div()
                    .flex()
                    .justify_end()
                    .gap(px(8.0))
                    .mt(px(12.0))
                    .child(
                        div()
                            .px(px(12.0))
                            .py(px(6.0))
                            .rounded(px(4.0))
                            .bg(rgb(0x2AABEE))
                            .text_color(rgb(0xFFFFFF))
                            .text_size(px(12.0))
                            .child("Send")
                    )
            )
    }
}
