//! History View Message Layout & Bubble Rendering (`history_view_message.h/cpp`).
//! Upstream reference: `Telegram/SourceFiles/history/view/history_view_message.cpp:L6650-L6720`
//! Mandate #18 Citations:
//! - Content width calculation: `newWidth - st::msgMargin.left() - wideSkip` (`history_view_message.cpp:L6687-L6689`)
//! - Clamping to `msgMaxWidth = 430px` (`history_view_message.cpp:L6699`, `ui/chat/chat.style:L14`)
//! - Minimum width `msgMinWidth = 160px` (`history_view_message.cpp:L6653`, `ui/chat/chat.style:L22`)
//! - Bubble padding: Left 11px, Top 8px, Right 11px, Bottom 8px (`ui/chat/chat.style:L25`)
//! - Bubble rounding: `bubbleRadiusLarge = 16px`, `bubbleRadiusSmall = 4px` (`history_view_message.cpp:L6646`)

use gpui::*;
use serde::{Deserialize, Serialize};

use super::bottom_info::{BottomInfoData, BottomInfoFlags};
use super::history_element::{
    BubblePadding, HistoryElementBase, MSG_MAX_WIDTH, MSG_MIN_WIDTH, MSG_PHOTO_SIZE,
    MSG_PHOTO_SKIP,
};
use super::reply_view::ReplyPreviewData;

/// Delivery status for outgoing messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum MessageDeliveryStatus {
    #[default]
    Pending,
    Sent,
    Read,
}

/// Core message view element for ConversationStage.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageBubbleData {
    pub base: HistoryElementBase,
    pub text: String,
    pub author_name: Option<String>,
    pub author_rank: Option<String>,
    pub reply: Option<ReplyPreviewData>,
    pub bottom_info: BottomInfoData,
    pub delivery_status: MessageDeliveryStatus,
    pub has_avatar: bool,
    pub is_selected: bool,
}

impl MessageBubbleData {
    pub fn new(
        item_id: u64,
        sender_id: u64,
        sender_name: String,
        text: String,
        date: i64,
        is_outgoing: bool,
    ) -> Self {
        let mut base = HistoryElementBase::new(item_id, sender_id, date, is_outgoing);
        base.sender_name = sender_name.clone();

        let mut bottom_flags = BottomInfoFlags::new();
        if is_outgoing {
            bottom_flags.insert(BottomInfoFlags::OUT_LAYOUT);
        }

        let bottom_info = BottomInfoData {
            date,
            edited_date: None,
            author: None,
            flags: bottom_flags,
            views: None,
            replies: None,
            forwards_count: None,
        };

        Self {
            base,
            text,
            author_name: if is_outgoing { None } else { Some(sender_name) },
            author_rank: None,
            reply: None,
            bottom_info,
            delivery_status: if is_outgoing {
                MessageDeliveryStatus::Sent
            } else {
                MessageDeliveryStatus::Read
            },
            has_avatar: !is_outgoing,
            is_selected: false,
        }
    }

    /// Resize and layout geometry matching `Message::resizeContentGetHeight`
    /// (`history_view_message.cpp:L6650-L6720`).
    pub fn calculate_layout(&mut self, available_width: f32) -> (f32, f32) {
        let margins = self.base.margins;
        let padding = BubblePadding::default();

        let mut content_width = available_width - margins.left - margins.right;
        if self.has_avatar && !self.base.is_outgoing {
            content_width -= MSG_PHOTO_SKIP;
        }

        content_width = content_width.clamp(MSG_MIN_WIDTH, MSG_MAX_WIDTH);

        let mut inner_height = padding.top;

        if self.author_name.is_some() {
            inner_height += 18.0;
        }

        if let Some(ref reply) = self.reply {
            inner_height += reply.calculate_height(content_width - padding.left - padding.right);
        }

        let text_lines = if self.text.is_empty() {
            0
        } else {
            let chars_per_line = ((content_width - padding.left - padding.right) / 7.5).max(10.0) as usize;
            (self.text.len() / chars_per_line).max(1)
        };
        inner_height += (text_lines as f32) * 19.0;

        inner_height += 16.0;
        inner_height += padding.bottom;

        let total_width = content_width;
        let total_height = inner_height + margins.top + margins.bottom;

        self.base.width = total_width;
        self.base.height = total_height;

        (total_width, total_height)
    }

    pub fn render(&self) -> impl IntoElement {
        let is_outgoing = self.base.is_outgoing;
        let rounding = self.base.bubble_rounding;
        let padding = BubblePadding::default();
        let margins = self.base.margins;

        let bubble_bg = if is_outgoing {
            rgb(0xEEFFDE)
        } else {
            rgb(0xFFFFFF)
        };

        let bubble_border = if is_outgoing {
            rgb(0xD2E8C4)
        } else {
            rgb(0xE0E0E0)
        };

        let author_color = rgb(0x2481CC);
        let text_color = rgb(0x111111);

        let mut bubble_col = div()
            .flex()
            .flex_col()
            .max_w(px(MSG_MAX_WIDTH))
            .min_w(px(MSG_MIN_WIDTH))
            .bg(bubble_bg)
            .border_1()
            .border_color(bubble_border)
            .rounded_tl(px(rounding.top_left))
            .rounded_tr(px(rounding.top_right))
            .rounded_bl(px(rounding.bottom_left))
            .rounded_br(px(rounding.bottom_right))
            .pl(px(padding.left))
            .pr(px(padding.right))
            .pt(px(padding.top))
            .pb(px(padding.bottom));

        if let Some(ref author) = self.author_name {
            bubble_col = bubble_col.child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .mb(px(2.0))
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(author_color)
                            .child(author.clone()),
                    ),
            );
        }

        if let Some(ref reply) = self.reply {
            bubble_col = bubble_col.child(
                div()
                    .flex()
                    .flex_row()
                    .mb(px(4.0))
                    .pl(px(8.0))
                    .border_l_2()
                    .border_color(author_color)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(author_color)
                                    .child(reply.sender_name.clone()),
                            )
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(rgb(0x555555))
                                    .child(reply.snippet.clone()),
                            ),
                    ),
            );
        }

        if !self.text.is_empty() {
            bubble_col = bubble_col.child(
                div()
                    .text_size(px(14.0))
                    .line_height(px(19.0))
                    .text_color(text_color)
                    .child(self.text.clone()),
            );
        }

        bubble_col = bubble_col.child(
            div()
                .flex()
                .flex_row()
                .justify_end()
                .mt(px(4.0))
                .child(self.bottom_info.render()),
        );

        let mut outer_row = div()
            .flex()
            .flex_row()
            .w_full()
            .mt(px(margins.top))
            .mb(px(margins.bottom))
            .pl(px(margins.left))
            .pr(px(margins.right));

        if is_outgoing {
            outer_row = outer_row.justify_end().child(bubble_col);
        } else {
            if self.has_avatar {
                let avatar = div()
                    .w(px(MSG_PHOTO_SIZE))
                    .h(px(MSG_PHOTO_SIZE))
                    .rounded_full()
                    .bg(rgb(0x40A7E3))
                    .mr(px(MSG_PHOTO_SKIP - MSG_PHOTO_SIZE))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(12.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xFFFFFF))
                    .child(
                        self.author_name
                            .as_ref()
                            .and_then(|n| n.chars().next())
                            .unwrap_or('U')
                            .to_string(),
                    );
                outer_row = outer_row.justify_start().child(avatar).child(bubble_col);
            } else {
                outer_row = outer_row
                    .justify_start()
                    .pl(px(MSG_PHOTO_SKIP))
                    .child(bubble_col);
            }
        }

        outer_row
    }
}

/// GPUI View Element representing an authentic Telegram message bubble.
pub struct MessageBubbleView {
    pub data: MessageBubbleData,
}

impl MessageBubbleView {
    pub fn new(data: MessageBubbleData) -> Self {
        Self { data }
    }

    pub fn render(&self) -> impl IntoElement {
        self.data.render()
    }
}

impl Render for MessageBubbleView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.data.render()
    }
}
