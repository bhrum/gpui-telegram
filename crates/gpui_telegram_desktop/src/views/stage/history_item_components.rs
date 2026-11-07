//! GPUI Telegram History Item Components View Component
//!
//! Citations:
//! - `Telegram/SourceFiles/history/history_item_components.h:L86-L1029`
//! - `Telegram/SourceFiles/history/history_item_components.cpp:L1-L1685`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui_telegram_client::history::item_components::{
    HistoryMessageFactcheck, HistoryMessageForwarded, HistoryMessageReply,
    HistoryMessageSuggestion, HistoryMessageVia, HistoryMessageViews,
    K_MAX_RECENT_REPLIERS,
};

/// Upstream `history_item_components.h:L86-L1029`: Dynamic Runtime Components Container
#[derive(Debug, Clone, Default)]
pub struct HistoryItemComponents {
    pub via: Option<HistoryMessageVia>,
    pub views: Option<HistoryMessageViews>,
    pub reply: Option<HistoryMessageReply>,
    pub forwarded: Option<HistoryMessageForwarded>,
    pub factcheck: Option<HistoryMessageFactcheck>,
    pub suggestion: Option<HistoryMessageSuggestion>,
}

impl HistoryItemComponents {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_via(mut self, bot_id: i64, username: impl Into<String>) -> Self {
        self.via = Some(HistoryMessageVia {
            bot_id,
            username: username.into(),
            link_url: None,
        });
        self
    }

    pub fn with_views(mut self, views: i32, replies: i32) -> Self {
        self.views = Some(HistoryMessageViews::new(views, replies));
        self
    }

    pub fn with_reply(mut self, reply_to_msg_id: i64, quote_text: Option<String>) -> Self {
        self.reply = Some(HistoryMessageReply {
            reply_to_msg_id,
            reply_to_top_id: None,
            reply_to_story_id: None,
            quote_text,
            quote_offset: None,
            is_forum_post: false,
        });
        self
    }

    pub fn with_forwarded(mut self, original_name: impl Into<String>, date: i64) -> Self {
        self.forwarded = Some(HistoryMessageForwarded {
            original_sender_id: None,
            original_name: Some(original_name.into()),
            original_date: date,
            original_id: None,
            psa_type: None,
        });
        self
    }

    pub fn has_components(&self) -> bool {
        self.via.is_some()
            || self.views.is_some()
            || self.reply.is_some()
            || self.forwarded.is_some()
            || self.factcheck.is_some()
            || self.suggestion.is_some()
    }

    pub fn render_view(&self) -> impl IntoElement {
        let mut container = div().flex().flex_col().w_full().gap(px(4.0));

        if let Some(forwarded) = &self.forwarded {
            container = container.child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x2AABEE))
                    .child(format!(
                        "Forwarded from {}",
                        forwarded.original_name.as_deref().unwrap_or("Unknown")
                    )),
            );
        }

        if let Some(reply) = &self.reply {
            container = container.child(
                div()
                    .flex()
                    .flex_col()
                    .p(px(6.0))
                    .bg(rgb(0xF5F5F5))
                    .rounded(px(4.0))
                    .border_l_2()
                    .border_color(rgb(0x2AABEE))
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(0x888888))
                            .child(format!("Reply to #{}", reply.reply_to_msg_id)),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(0x333333))
                            .child(reply.quote_text.clone().unwrap_or_default()),
                    ),
            );
        }

        if let Some(factcheck) = &self.factcheck {
            container = container.child(
                div()
                    .p(px(6.0))
                    .bg(rgb(0xFFF8E7))
                    .rounded(px(4.0))
                    .border_1()
                    .border_color(rgb(0xF5A623))
                    .text_size(px(12.0))
                    .text_color(rgb(0x8B572A))
                    .child(format!("Fact check: {}", factcheck.text)),
            );
        }

        container
    }
}

impl Render for HistoryItemComponents {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut container = div().flex().flex_col().w_full().gap(px(4.0));

        if let Some(forwarded) = &self.forwarded {
            container = container.child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x2AABEE))
                    .child(format!(
                        "Forwarded from {}",
                        forwarded.original_name.as_deref().unwrap_or("Unknown")
                    )),
            );
        }

        if let Some(reply) = &self.reply {
            container = container.child(
                div()
                    .flex()
                    .flex_col()
                    .p(px(6.0))
                    .bg(rgb(0xF5F5F5))
                    .rounded(px(4.0))
                    .border_l_2()
                    .border_color(rgb(0x2AABEE))
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(0x888888))
                            .child(format!("Reply to #{}", reply.reply_to_msg_id)),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(0x333333))
                            .child(reply.quote_text.clone().unwrap_or_default()),
                    ),
            );
        }

        if let Some(factcheck) = &self.factcheck {
            container = container.child(
                div()
                    .p(px(6.0))
                    .bg(rgb(0xFFF8E7))
                    .rounded(px(4.0))
                    .border_1()
                    .border_color(rgb(0xF5A623))
                    .text_size(px(12.0))
                    .text_color(rgb(0x8B572A))
                    .child(format!("Fact check: {}", factcheck.text)),
            );
        }

        container
    }
}
