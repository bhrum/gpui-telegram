//! Canonical Location Message Card (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.4.5).

use gpui::*;
use super::models::MessageItemModel;

pub struct LocationMessageView;

impl LocationMessageView {
    pub fn render(
        message: &MessageItemModel,
        latitude: f64,
        longitude: f64,
        title: &str,
        address: &str,
    ) -> impl IntoElement {
        let is_outgoing = message.is_outgoing();
        let bg_color = if is_outgoing { rgb(0x2B5278) } else { rgb(0x182533) };

        div()
            .flex()
            .flex_col()
            .w(px(300.0))
            .rounded(px(14.0))
            .bg(bg_color)
            .p(px(4.0))
            .overflow_hidden()
            // Map Preview Canvas Area (height 140px)
            .child(
                div()
                    .relative()
                    .w(px(292.0))
                    .h(px(140.0))
                    .rounded(px(10.0))
                    .bg(rgb(0x243447)) // Map background tone
                    .flex()
                    .items_center()
                    .justify_center()
                    // Map Grid lines decoration
                    .child(
                        div()
                            .text_size(px(36.0))
                            .child("📍"),
                    )
                    // Coordinates pill (bottom-left)
                    .child(
                        div()
                            .absolute()
                            .bottom(px(6.0))
                            .left(px(6.0))
                            .px(px(6.0))
                            .py(px(2.0))
                            .rounded(px(4.0))
                            .bg(rgba(0x00000088))
                            .text_size(px(10.0))
                            .text_color(rgb(0xFFFFFF))
                            .child(format!("{:.4}, {:.4}", latitude, longitude)),
                    ),
            )
            // Venue Details & Open Action
            .child(
                div()
                    .px(px(8.0))
                    .py(px(8.0))
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .child(
                        div()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child(title.to_string()),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(0x8A9AA8))
                            .child(address.to_string()),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .pt(px(4.0))
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0x2AABEE))
                                    .child("Open in Maps →"),
                            )
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(rgb(0x8A9AA8))
                                    .child(message.timestamp.clone()),
                            ),
                    ),
            )
    }
}
