//! History View Service Message Layout & Pill Rendering (`history_view_service_message.h/cpp`).
//! Upstream reference: `Telegram/SourceFiles/history/view/history_view_service_message.cpp:L500-L560`
//! Mandate #18 Citations:
//! - Service chat width in wide mode: `WideChatWidth() = msgMaxWidth + 2 * msgPhotoSkip + 2 * msgMargin.left() = 542px` (`history_view_service_message.cpp:L214-L216`)
//! - Service message padding: Left 12px, Top 3px, Right 12px, Bottom 4px (`ui/chat/chat.style:L95`)
//! - Service message margins: Left 10px, Top 10px, Right 10px, Bottom 2px (`ui/chat/chat.style:L96`)
//! - Service bubble corner rounding: pill shaped (`history_view_service_message.cpp:L68-L100`)

use gpui::*;
use serde::{Deserialize, Serialize};

use super::history_element::{MSG_MAX_WIDTH, MSG_PHOTO_SKIP};

/// Wide chat width limit constant (`history_view_service_message.cpp:L214-L216`).
/// `WideChatWidth = 430 + 2 * 40 + 2 * 16 = 542px`.
pub const WIDE_CHAT_WIDTH: f32 = MSG_MAX_WIDTH + 2.0 * MSG_PHOTO_SKIP + 32.0;

/// Service message pill padding (`ui/chat/chat.style:L95`).
pub const MSG_SERVICE_PADDING_LEFT: f32 = 12.0;
pub const MSG_SERVICE_PADDING_TOP: f32 = 3.0;
pub const MSG_SERVICE_PADDING_RIGHT: f32 = 12.0;
pub const MSG_SERVICE_PADDING_BOTTOM: f32 = 4.0;

/// Service message margins (`ui/chat/chat.style:L96`).
pub const MSG_SERVICE_MARGIN_TOP: f32 = 10.0;
pub const MSG_SERVICE_MARGIN_BOTTOM: f32 = 2.0;

/// Categorization of service message notification content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceMessageKind {
    DateSeparator(String),
    UserJoined(String),
    PinnedMessage { sender: String, snippet: String },
    CallEnded { duration: String },
    ChannelCreated(String),
    ActionText(String),
}

/// Data payload for a service message bubble.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceMessageData {
    pub item_id: u64,
    pub date: i64,
    pub kind: ServiceMessageKind,
}

impl ServiceMessageData {
    pub fn new_date(date: i64, formatted: impl Into<String>) -> Self {
        Self {
            item_id: 0,
            date,
            kind: ServiceMessageKind::DateSeparator(formatted.into()),
        }
    }

    pub fn new_action(item_id: u64, date: i64, text: impl Into<String>) -> Self {
        Self {
            item_id,
            date,
            kind: ServiceMessageKind::ActionText(text.into()),
        }
    }

    pub fn new_pin(item_id: u64, date: i64, sender: impl Into<String>, snippet: impl Into<String>) -> Self {
        Self {
            item_id,
            date,
            kind: ServiceMessageKind::PinnedMessage {
                sender: sender.into(),
                snippet: snippet.into(),
            },
        }
    }

    pub fn display_text(&self) -> String {
        match &self.kind {
            ServiceMessageKind::DateSeparator(date) => date.clone(),
            ServiceMessageKind::UserJoined(user) => format!("{} joined the group", user),
            ServiceMessageKind::PinnedMessage { sender, snippet } => {
                format!("{} pinned \"{}\"", sender, snippet)
            }
            ServiceMessageKind::CallEnded { duration } => format!("Call ended ({})", duration),
            ServiceMessageKind::ChannelCreated(name) => format!("Channel \"{}\" created", name),
            ServiceMessageKind::ActionText(text) => text.clone(),
        }
    }

    pub fn render(&self) -> impl IntoElement {
        let text = self.display_text();

        div()
            .flex()
            .flex_row()
            .justify_center()
            .w_full()
            .mt(px(MSG_SERVICE_MARGIN_TOP))
            .mb(px(MSG_SERVICE_MARGIN_BOTTOM))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .max_w(px(WIDE_CHAT_WIDTH))
                    .pl(px(MSG_SERVICE_PADDING_LEFT))
                    .pr(px(MSG_SERVICE_PADDING_RIGHT))
                    .pt(px(MSG_SERVICE_PADDING_TOP))
                    .pb(px(MSG_SERVICE_PADDING_BOTTOM))
                    .bg(rgb(0x7D8B99))
                    .rounded_full()
                    .text_size(px(13.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(0xFFFFFF))
                    .child(text),
            )
    }
}

/// GPUI View Element for centered pill-shaped service messages.
pub struct ServiceMessageBubbleView {
    pub data: ServiceMessageData,
}

impl ServiceMessageBubbleView {
    pub fn new(data: ServiceMessageData) -> Self {
        Self { data }
    }

    pub fn render(&self) -> impl IntoElement {
        self.data.render()
    }
}

impl Render for ServiceMessageBubbleView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.data.render()
    }
}
