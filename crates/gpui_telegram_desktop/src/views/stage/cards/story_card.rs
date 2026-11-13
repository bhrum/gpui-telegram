//! Canonical Story Message Card (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.4.9).

use gpui::*;
use super::models::MessageItemModel;

pub struct StoryMessageView;

impl StoryMessageView {
    pub fn render(
        message: &MessageItemModel,
        user_name: &str,
        avatar_initials: &str,
        caption: &str,
        preview_color: u32,
        expires_in: &str,
    ) -> impl IntoElement {
        let is_outgoing = message.is_outgoing();
        let bg_color = if is_outgoing { rgb(0x2B5278) } else { rgb(0x182533) };

        div()
            .flex()
            .flex_col()
            .w(px(260.0))
            .rounded(px(14.0))
            .bg(bg_color)
            .p(px(8.0))
            .gap(px(6.0))
            // Story author header
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    // Gradient Story Ring around Avatar
                    .child(
                        div()
                            .w(px(32.0))
                            .h(px(32.0))
                            .rounded_full()
                            .border_2()
                            .border_color(rgb(0x9B59B6)) // Telegram Story Purple/Blue gradient tone
                            .p(px(1.5))
                            .child(
                                div()
                                    .size_full()
                                    .rounded_full()
                                    .bg(rgb(0x2AABEE))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .child(avatar_initials.to_string()),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .child(user_name.to_string()),
                            )
                            .child(
                                div()
                                    .text_size(px(10.0))
                                    .text_color(rgb(0x8A9AA8))
                                    .child(format!("Story expires in {}", expires_in)),
                            ),
                    ),
            )
            // Story preview card (height 120px)
            .child(
                div()
                    .relative()
                    .w_full()
                    .h(px(120.0))
                    .rounded(px(8.0))
                    .bg(rgb(preview_color))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_size(px(28.0))
                            .child("✨"),
                    )
                    // View Story Pill
                    .child(
                        div()
                            .absolute()
                            .bottom(px(6.0))
                            .px(px(8.0))
                            .py(px(3.0))
                            .rounded_full()
                            .bg(rgba(0x00000088))
                            .text_size(px(11.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child("View Story"),
                    ),
            )
            // Caption & Timestamp
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_end()
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(0xFFFFFF))
                            .child(caption.to_string()),
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
