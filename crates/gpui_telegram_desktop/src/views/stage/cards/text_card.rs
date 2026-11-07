//! Canonical Text Message Card (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.4.1).

use gpui::*;
use gpui::prelude::FluentBuilder;
use super::models::{DeliveryStatus, MessageItemModel, ReactionItem, ReplyReference};

pub struct TextMessageView;

impl TextMessageView {
    pub fn render(
        message: &MessageItemModel,
        text: &str,
        is_edited: bool,
    ) -> impl IntoElement {
        let is_outgoing = message.is_outgoing();
        let bg_color = if is_outgoing {
            rgb(0x2B5278) // Upstream msgInBg / msgOutBg
        } else {
            rgb(0x182533)
        };

        div()
            .flex()
            .flex_col()
            .max_w(px(520.0))
            .rounded(px(14.0))
            .bg(bg_color)
            .px(px(12.0))
            .py(px(8.0))
            .gap(px(4.0))
            // Reply quote strip if replying to a message
            .when_some(message.reply_to.as_ref(), |el, reply| {
                el.child(Self::render_reply_quote(reply))
            })
            // Sender name (for incoming messages in group chats)
            .when(!is_outgoing, |el| {
                el.child(
                    div()
                        .text_size(px(12.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(message.avatar_color))
                        .child(message.sender_name.clone()),
                )
            })
            // Text body with timestamp inline / row
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .items_end()
                    .justify_between()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_size(px(14.0))
                            .line_height(px(19.0))
                            .text_color(rgb(0xFFFFFF))
                            .child(text.to_string()),
                    )
                    // Metadata footer: edited + time + delivery status
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(4.0))
                            .when(is_edited, |el| {
                                el.child(
                                    div()
                                        .text_size(px(11.0))
                                        .text_color(rgb(0x8A9AA8))
                                        .child("edited"),
                                )
                            })
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(rgb(0x8A9AA8))
                                    .child(message.timestamp.clone()),
                            )
                            .when(is_outgoing, |el| {
                                el.child(Self::render_status(message.delivery_status))
                            }),
                    ),
            )
            // Reactions pill strip
            .when(!message.reactions.is_empty(), |el| {
                el.child(Self::render_reactions(&message.reactions))
            })
    }

    fn render_reply_quote(reply: &ReplyReference) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap(px(6.0))
            .px(px(8.0))
            .py(px(4.0))
            .rounded(px(6.0))
            .bg(rgba(0x00000033))
            .border_l_2()
            .border_color(rgb(0x5288C1))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .text_size(px(11.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x5288C1))
                            .child(reply.sender_name.clone()),
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(0x8A9AA8))
                            .child(reply.snippet.clone()),
                    ),
            )
    }

    fn render_status(status: DeliveryStatus) -> impl IntoElement {
        match status {
            DeliveryStatus::Sending => div()
                .text_size(px(11.0))
                .text_color(rgb(0x8A9AA8))
                .child("🕒"),
            DeliveryStatus::Sent => div()
                .text_size(px(11.0))
                .text_color(rgb(0x8A9AA8))
                .child("✓"),
            DeliveryStatus::Read => div()
                .text_size(px(11.0))
                .text_color(rgb(0x2AABEE))
                .child("✓✓"),
        }
    }

    pub fn render_reactions(reactions: &[ReactionItem]) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap(px(4.0))
            .pt(px(2.0))
            .children(reactions.iter().map(|r| {
                let pill_bg = if r.user_reacted {
                    rgba(0x2AABEE44)
                } else {
                    rgba(0x00000033)
                };
                div()
                    .flex()
                    .items_center()
                    .gap(px(3.0))
                    .px(px(6.0))
                    .py(px(2.0))
                    .rounded_full()
                    .bg(pill_bg)
                    .child(div().text_size(px(12.0)).child(r.emoji.clone()))
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(0xFFFFFF))
                            .child(r.count.to_string()),
                    )
            }))
    }
}
