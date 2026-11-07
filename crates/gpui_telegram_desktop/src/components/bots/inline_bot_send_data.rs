//! InlineBotSendData: Message payload creation and restriction checking for chosen inline bot results.
//!
//! Upstream reference:
//! - `Telegram/SourceFiles/inline_bots/inline_bot_send_data.h:L36-L180`
//! - `Telegram/SourceFiles/inline_bots/inline_bot_send_data.cpp:L25-L90`
#![allow(dead_code)]

use gpui::*;

/// Outgoing inline message payload variants (upstream: `InlineBots::internal::SendData`).
#[derive(Debug, Clone, PartialEq)]
pub enum InlineBotPayload {
    Text {
        text: String,
        has_webpage: bool,
    },
    RichPage {
        summary: String,
    },
    Geo {
        latitude: f64,
        longitude: f64,
        period_seconds: Option<i32>,
    },
    Contact {
        phone_number: String,
        first_name: String,
        last_name: String,
    },
    Game {
        short_name: String,
    },
    Photo {
        photo_id: u64,
    },
    File {
        document_id: u64,
    },
}

/// Send restriction validation error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InlineSendError {
    ChatRestricted,
    EmptyContent,
    BotNotAllowed,
}

/// Authentic InlineBotSendData component state.
pub struct InlineBotSendData {
    pub result_id: String,
    pub payload: InlineBotPayload,
    pub reply_to_msg_id: Option<i32>,
    pub updated_at: i64,
}

impl InlineBotSendData {
    pub fn new_text(result_id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            result_id: result_id.into(),
            payload: InlineBotPayload::Text {
                text: text.into(),
                has_webpage: true,
            },
            reply_to_msg_id: None,
            updated_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn new_geo(result_id: impl Into<String>, lat: f64, lon: f64) -> Self {
        Self {
            result_id: result_id.into(),
            payload: InlineBotPayload::Geo {
                latitude: lat,
                longitude: lon,
                period_seconds: None,
            },
            reply_to_msg_id: None,
            updated_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn validate_send(&self, is_chat_restricted: bool) -> Result<(), InlineSendError> {
        if is_chat_restricted {
            return Err(InlineSendError::ChatRestricted);
        }
        match &self.payload {
            InlineBotPayload::Text { text, .. } if text.trim().is_empty() => {
                Err(InlineSendError::EmptyContent)
            }
            _ => Ok(()),
        }
    }

    pub fn has_location(&self) -> bool {
        matches!(self.payload, InlineBotPayload::Geo { .. })
    }
}

impl Default for InlineBotSendData {
    fn default() -> Self {
        Self::new_text("default_result", "Sample inline bot message")
    }
}

impl Render for InlineBotSendData {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .p(px(8.0))
            .bg(rgb(0xFFFFFF))
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x444444))
                    .child(format!("SendData: result {}", self.result_id))
            )
    }
}
