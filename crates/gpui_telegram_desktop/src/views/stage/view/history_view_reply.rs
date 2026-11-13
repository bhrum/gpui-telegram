//! Authentic HistoryView Reply View Component (`history_view_reply.h/cpp`).
//! Upstream reference: `Telegram/SourceFiles/history/view/history_view_reply.cpp:L746-L800`

use gpui::*;
use serde::{Deserialize, Serialize};

use crate::views::stage::reply_view::{ReplyPreviewData, ReplyPreviewView, K_NON_EXPANDED_LINES_LIMIT};

/// Authentic GPUI View for HistoryView::Reply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryViewReplyView {
    pub reply: ReplyPreviewData,
}

impl HistoryViewReplyView {
    pub fn new() -> Self {
        Self {
            reply: ReplyPreviewData::new(0, String::new(), String::new()),
        }
    }

    pub fn with_reply(
        reply_id: u64,
        sender_name: impl Into<String>,
        snippet: impl Into<String>,
    ) -> Self {
        Self {
            reply: ReplyPreviewData::new(reply_id, sender_name, snippet),
        }
    }

    pub fn non_expanded_lines_limit(&self) -> usize {
        K_NON_EXPANDED_LINES_LIMIT
    }

    pub fn calculate_height(&self, available_width: f32) -> f32 {
        self.reply.calculate_height(available_width)
    }

    pub fn toggle_expanded(&mut self) {
        self.reply.is_expanded = !self.reply.is_expanded;
    }
}

impl Default for HistoryViewReplyView {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewReplyView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child(self.reply.render())
    }
}
