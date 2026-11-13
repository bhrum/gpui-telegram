//! Authentic HistoryView Service Message View Component (`history_view_service_message.h/cpp`).
//! Upstream reference: `Telegram/SourceFiles/history/view/history_view_service_message.cpp:L500-L560`

use gpui::*;
use serde::{Deserialize, Serialize};

use crate::views::stage::history_service::{
    ServiceMessageBubbleView, ServiceMessageData, ServiceMessageKind, WIDE_CHAT_WIDTH,
};

/// Authentic GPUI View for HistoryView::Service.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryViewServiceMessageView {
    pub service: ServiceMessageData,
}

impl HistoryViewServiceMessageView {
    pub fn new() -> Self {
        Self {
            service: ServiceMessageData::new_action(0, 0, String::new()),
        }
    }

    pub fn with_date(date: i64, formatted: impl Into<String>) -> Self {
        Self {
            service: ServiceMessageData::new_date(date, formatted),
        }
    }

    pub fn with_action(item_id: u64, date: i64, text: impl Into<String>) -> Self {
        Self {
            service: ServiceMessageData::new_action(item_id, date, text),
        }
    }

    pub fn wide_chat_width(&self) -> f32 {
        WIDE_CHAT_WIDTH
    }
}

impl Default for HistoryViewServiceMessageView {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewServiceMessageView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child(self.service.render())
    }
}
