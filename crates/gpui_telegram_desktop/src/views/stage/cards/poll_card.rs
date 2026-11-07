//! Canonical Poll Message Card (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.4.6).

use gpui::*;
use gpui::prelude::FluentBuilder;
use super::models::{MessageItemModel, PollOption, PollType};

pub struct PollMessageView;

impl PollMessageView {
    pub fn render(
        message: &MessageItemModel,
        question: &str,
        poll_type: PollType,
        options: &[PollOption],
        total_votes: u32,
        is_closed: bool,
        selected_option_id: Option<u32>,
    ) -> impl IntoElement {
        let is_outgoing = message.is_outgoing();
        let bg_color = if is_outgoing { rgb(0x2B5278) } else { rgb(0x182533) };

        let type_label = match poll_type {
            PollType::Regular => "Poll",
            PollType::Quiz => "Quiz",
            PollType::Anonymous => "Anonymous Poll",
        };

        div()
            .flex()
            .flex_col()
            .w(px(340.0))
            .rounded(px(14.0))
            .bg(bg_color)
            .p(px(14.0))
            .gap(px(10.0))
            // Header: Poll Type & Closed Badge
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x5288C1))
                            .child(type_label),
                    )
                    .when(is_closed, |el| {
                        el.child(
                            div()
                                .px(px(6.0))
                                .py(px(2.0))
                                .rounded(px(4.0))
                                .bg(rgba(0x00000044))
                                .text_size(px(10.0))
                                .text_color(rgb(0x8A9AA8))
                                .child("Final Results"),
                        )
                    }),
            )
            // Question Title
            .child(
                div()
                    .text_size(px(15.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xFFFFFF))
                    .line_height(px(20.0))
                    .child(question.to_string()),
            )
            // Poll Options with Percentage Bars
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .children(options.iter().map(|opt| {
                        let has_voted = selected_option_id.is_some() || is_closed;
                        let is_my_choice = selected_option_id == Some(opt.id);

                        div()
                            .flex()
                            .flex_col()
                            .gap(px(4.0))
                            // Option row: Radio circle + Text + Percentage
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(8.0))
                                            // Radio Button / Checkmark
                                            .child(
                                                div()
                                                    .w(px(18.0))
                                                    .h(px(18.0))
                                                    .rounded_full()
                                                    .border_2()
                                                    .border_color(if is_my_choice { rgb(0x2AABEE) } else { rgb(0x6B808D) })
                                                    .bg(if is_my_choice { rgb(0x2AABEE) } else { rgba(0x00000000) })
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .text_size(px(10.0))
                                                    .text_color(rgb(0xFFFFFF))
                                                    .child(if is_my_choice { "✓" } else { "" }),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(13.0))
                                                    .text_color(rgb(0xFFFFFF))
                                                    .child(opt.text.clone()),
                                            ),
                                    )
                                    .when(has_voted, |el| {
                                        el.child(
                                            div()
                                                .text_size(px(12.0))
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(rgb(0x8A9AA8))
                                                .child(format!("{:.0}%", opt.percentage)),
                                        )
                                    }),
                            )
                            // Progress bar
                            .when(has_voted, |el| {
                                let bar_w = ((opt.percentage / 100.0) * 312.0).clamp(4.0, 312.0);
                                let bar_color = if poll_type == PollType::Quiz && opt.is_correct {
                                    rgb(0x4CAF50) // Green for correct quiz answer
                                } else if is_my_choice {
                                    rgb(0x2AABEE) // Accent for user selection
                                } else {
                                    rgb(0x3B4E61)
                                };

                                el.child(
                                    div()
                                        .w_full()
                                        .h(px(4.0))
                                        .rounded_full()
                                        .bg(rgb(0x1B2734))
                                        .child(
                                            div()
                                                .w(px(bar_w))
                                                .h(px(4.0))
                                                .rounded_full()
                                                .bg(bar_color),
                                        ),
                                )
                            })
                    })),
            )
            // Footer: Total votes + Message timestamp
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .pt(px(4.0))
                    .border_t_1()
                    .border_color(rgba(0x00000022))
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(0x8A9AA8))
                            .child(format!("{} votes", total_votes)),
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(0x8A9AA8))
                            .child(message.timestamp.clone()),
                    ),
            )
    }
}
