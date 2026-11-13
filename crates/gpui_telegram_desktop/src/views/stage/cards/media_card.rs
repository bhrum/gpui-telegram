//! Canonical Media Message Card (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.4.2).

use gpui::*;
use gpui::prelude::FluentBuilder;
use super::models::{MediaType, MessageItemModel};

pub struct MediaMessageView;

impl MediaMessageView {
    pub fn render(
        message: &MessageItemModel,
        media_type: MediaType,
        _url: &str,
        width: u32,
        height: u32,
        duration_secs: Option<u32>,
        _file_size_bytes: u64,
        caption: Option<&str>,
        is_streaming: bool,
    ) -> impl IntoElement {
        let is_outgoing = message.is_outgoing();
        let bg_color = if is_outgoing { rgb(0x2B5278) } else { rgb(0x182533) };
        let aspect = if height > 0 { width as f32 / height as f32 } else { 1.33 };
        let card_w = 320.0f32;
        let card_h = (card_w / aspect).clamp(180.0, 360.0);

        div()
            .flex()
            .flex_col()
            .max_w(px(340.0))
            .rounded(px(14.0))
            .bg(bg_color)
            .p(px(4.0))
            .overflow_hidden()
            // Media Preview Thumbnail Area
            .child(
                div()
                    .relative()
                    .w(px(card_w))
                    .h(px(card_h))
                    .rounded(px(10.0))
                    .bg(rgb(0x101921))
                    .flex()
                    .items_center()
                    .justify_center()
                    // Background placeholder image pattern or mock gradient
                    .child(
                        div()
                            .text_size(px(32.0))
                            .text_color(rgb(0x4F6578))
                            .child(if media_type == MediaType::Video { "🎬" } else { "🖼️" }),
                    )
                    // Video Play Button Overlay
                    .when(media_type == MediaType::Video, |el| {
                        el.child(
                            div()
                                .w(px(48.0))
                                .h(px(48.0))
                                .rounded_full()
                                .bg(rgba(0x00000088))
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_color(rgb(0xFFFFFF))
                                .text_size(px(20.0))
                                .child("▶"),
                        )
                    })
                    // Video Duration Pill Badge (bottom-left)
                    .when_some(duration_secs, |el, dur| {
                        let mins = dur / 60;
                        let secs = dur % 60;
                        el.child(
                            div()
                                .absolute()
                                .bottom(px(8.0))
                                .left(px(8.0))
                                .px(px(6.0))
                                .py(px(2.0))
                                .rounded(px(4.0))
                                .bg(rgba(0x00000088))
                                .text_size(px(11.0))
                                .text_color(rgb(0xFFFFFF))
                                .child(format!("{:02}:{:02}", mins, secs)),
                        )
                    })
                    // Streaming live badge
                    .when(is_streaming, |el| {
                        el.child(
                            div()
                                .absolute()
                                .top(px(8.0))
                                .left(px(8.0))
                                .px(px(6.0))
                                .py(px(2.0))
                                .rounded(px(4.0))
                                .bg(rgb(0xE53935))
                                .text_size(px(10.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xFFFFFF))
                                .child("LIVE"),
                        )
                    })
                    // Timestamp badge (bottom-right of media)
                    .when(caption.is_none(), |el| {
                        el.child(
                            div()
                                .absolute()
                                .bottom(px(8.0))
                                .right(px(8.0))
                                .px(px(6.0))
                                .py(px(2.0))
                                .rounded(px(4.0))
                                .bg(rgba(0x00000088))
                                .text_size(px(11.0))
                                .text_color(rgb(0xFFFFFF))
                                .child(message.timestamp.clone()),
                        )
                    }),
            )
            // Optional Caption below media
            .when_some(caption, |el, cap| {
                el.child(
                    div()
                        .px(px(8.0))
                        .py(px(6.0))
                        .flex()
                        .justify_between()
                        .items_end()
                        .child(
                            div()
                                .text_size(px(13.0))
                                .text_color(rgb(0xFFFFFF))
                                .child(cap.to_string()),
                        )
                        .child(
                            div()
                                .text_size(px(11.0))
                                .text_color(rgb(0x8A9AA8))
                                .child(message.timestamp.clone()),
                        ),
                )
            })
    }
}
