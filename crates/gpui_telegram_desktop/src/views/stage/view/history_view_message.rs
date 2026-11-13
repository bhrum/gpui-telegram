//! Authentic HistoryView Message View Component (`history_view_message.h/cpp`).
//! Upstream reference: `Telegram/SourceFiles/history/view/history_view_message.cpp:L6650-L6720`

use gpui::*;
use serde::{Deserialize, Serialize};

use crate::views::stage::history_message::{MessageBubbleData, MessageDeliveryStatus};

/// Authentic GPUI View for HistoryView::Message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoryViewMessageView {
    pub bubble: MessageBubbleData,
}

impl HistoryViewMessageView {
    pub fn new() -> Self {
        Self {
            bubble: MessageBubbleData::new(0, 0, String::new(), String::new(), 0, false),
        }
    }

    pub fn with_message(
        item_id: u64,
        sender_id: u64,
        sender_name: impl Into<String>,
        text: impl Into<String>,
        date: i64,
        is_outgoing: bool,
    ) -> Self {
        Self {
            bubble: MessageBubbleData::new(
                item_id,
                sender_id,
                sender_name.into(),
                text.into(),
                date,
                is_outgoing,
            ),
        }
    }

    pub fn calculate_layout(&mut self, available_width: f32) -> (f32, f32) {
        self.bubble.calculate_layout(available_width)
    }

    pub fn set_delivery_status(&mut self, status: MessageDeliveryStatus) {
        self.bubble.delivery_status = status;
    }
}

impl Default for HistoryViewMessageView {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewMessageView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child(self.bubble.render())
    }
}
