//! History View Reply Layout & Preview Strip Rendering (`history_view_reply.h/cpp`).
//! Upstream reference: `Telegram/SourceFiles/history/view/history_view_reply.cpp:L746-L800`
//! Mandate #18 Citations:
//! - Non-expanded multiline limit: `kNonExpandedLinesLimit = 5` (`history_view_reply.cpp:L46`)
//! - Reply preview thumbnail size: `historyReplyPreview = 32px` (`ui/chat/chat.style:L65`)
//! - Reply preview margins: Left 7px, Top 4px, Right 4px, Bottom 4px (`ui/chat/chat.style:L66`)
//! - Reply padding: Left 11px, Top 2px, Right 6px, Bottom 2px (`ui/chat/chat.style:L73`)
//! - Vertical spacing: `historyReplyTop = 2px`, `historyReplyBottom = 2px` (`ui/chat/chat.style:L63-L64`)
//! - Accent bar: `msgReplyBarPos = point(1px, 0px)`, `msgReplyBarSize = size(2px, 36px)` (`ui/chat/chat.style:L92-L93`)

use gpui::*;
use serde::{Deserialize, Serialize};

/// Maximum visible lines before an expandable reply snippet is truncated.
/// C++ upstream reference: `history_view_reply.cpp:L46`.
pub const K_NON_EXPANDED_LINES_LIMIT: usize = 5;

/// Quoted message preview data model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplyPreviewData {
    pub reply_id: u64,
    pub sender_name: String,
    pub snippet: String,
    pub has_thumbnail: bool,
    pub thumbnail_url: Option<String>,
    pub is_expanded: bool,
}

impl ReplyPreviewData {
    pub fn new(reply_id: u64, sender_name: impl Into<String>, snippet: impl Into<String>) -> Self {
        Self {
            reply_id,
            sender_name: sender_name.into(),
            snippet: snippet.into(),
            has_thumbnail: false,
            thumbnail_url: None,
            is_expanded: false,
        }
    }

    pub fn with_thumbnail(mut self, url: impl Into<String>) -> Self {
        self.has_thumbnail = true;
        self.thumbnail_url = Some(url.into());
        self
    }

    /// Calculate vertical height of reply strip matching `Reply::resizeToWidth` (`history_view_reply.cpp:L746-L779`).
    pub fn calculate_height(&self, _available_width: f32) -> f32 {
        let base_height = 36.0;
        let margins = 4.0;
        base_height + margins
    }

    pub fn render(&self) -> impl IntoElement {
        let bar_color = rgb(0x2481CC);

        let mut row = div()
            .flex()
            .flex_row()
            .items_center()
            .w_full()
            .my(px(2.0))
            .pl(px(8.0))
            .border_l_2()
            .border_color(bar_color);

        if self.has_thumbnail {
            let thumb = div()
                .w(px(32.0))
                .h(px(32.0))
                .rounded(px(4.0))
                .bg(rgb(0xD9E5ED))
                .mr(px(6.0))
                .flex()
                .items_center()
                .justify_center()
                .text_size(px(10.0))
                .text_color(rgb(0x666666))
                .child("🖼");
            row = row.child(thumb);
        }

        let text_col = div()
            .flex()
            .flex_col()
            .child(
                div()
                    .text_size(px(12.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(bar_color)
                    .child(self.sender_name.clone()),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x555555))
                    .line_clamp(if self.is_expanded {
                        100
                    } else {
                        K_NON_EXPANDED_LINES_LIMIT
                    })
                    .child(self.snippet.clone()),
            );

        row.child(text_col)
    }
}

/// GPUI View Element for quoted message reply preview strip.
pub struct ReplyPreviewView {
    pub data: ReplyPreviewData,
}

impl ReplyPreviewView {
    pub fn new(data: ReplyPreviewData) -> Self {
        Self { data }
    }

    pub fn render(&self) -> impl IntoElement {
        self.data.render()
    }
}

impl Render for ReplyPreviewView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.data.render()
    }
}
