use gpui::*;
use gpui::prelude::FluentBuilder;
use super::models::{MediaType, MessageItemModel};
use super::text_card::{render_bubble_tail, BubbleRounding, MSG_MAX_WIDTH, MSG_MIN_WIDTH};

pub struct MediaMessageView;

impl MediaMessageView {
    pub fn render_default(
        message: &MessageItemModel,
        media_type: MediaType,
        url: &str,
        width: u32,
        height: u32,
        duration_secs: Option<u32>,
        file_size_bytes: u64,
        caption: Option<&str>,
        is_streaming: bool,
    ) -> impl IntoElement {
        Self::render(
            message,
            media_type,
            url,
            width,
            height,
            duration_secs,
            file_size_bytes,
            caption,
            is_streaming,
            BubbleRounding::default(),
        )
    }

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
        rounding: BubbleRounding,
    ) -> impl IntoElement {
        let is_outgoing = message.is_outgoing();
        let bg_color = if is_outgoing { rgb(0x2B5278) } else { rgb(0x182533) };
        let aspect = if height > 0 { width as f32 / height as f32 } else { 1.33 };
        let card_w = 320.0f32;
        let card_h = (card_w / aspect).clamp(180.0, 360.0);

        div()
            .relative()
            .flex()
            .flex_col()
            .min_w(px(MSG_MIN_WIDTH))
            .max_w(px(MSG_MAX_WIDTH))
            .rounded_tl(px(rounding.top_left))
            .rounded_tr(px(rounding.top_right))
            .rounded_bl(px(rounding.bottom_left))
            .rounded_br(px(rounding.bottom_right))
            .bg(bg_color)
            .p(px(4.0))
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
            .when(rounding.has_tail, |el| {
                el.child(render_bubble_tail(message.direction, bg_color))
            })
    }
}
