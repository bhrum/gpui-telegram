//! Canonical Album Message Card (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.4.4).

use gpui::*;
use gpui::prelude::FluentBuilder;
use super::models::{AlbumItem, MediaType, MessageItemModel};

pub struct AlbumMessageView;

impl AlbumMessageView {
    pub fn render(
        message: &MessageItemModel,
        items: &[AlbumItem],
        caption: Option<&str>,
    ) -> impl IntoElement {
        let is_outgoing = message.is_outgoing();
        let bg_color = if is_outgoing { rgb(0x2B5278) } else { rgb(0x182533) };
        let count = items.len();

        div()
            .flex()
            .flex_col()
            .max_w(px(340.0))
            .rounded(px(14.0))
            .bg(bg_color)
            .p(px(4.0))
            .overflow_hidden()
            // Album Grid Mosaic
            .child(
                div()
                    .w(px(332.0))
                    .flex()
                    .flex_wrap()
                    .gap(px(2.0))
                    .children(items.iter().take(4).enumerate().map(|(idx, item)| {
                        // Dynamic 2x2 sizing for 2-4 items, or hero layout
                        let tile_w = if count == 1 { 332.0 } else { 164.0 };
                        let tile_h = if count <= 2 { 180.0 } else { 120.0 };
                        let is_last_tile = idx == 3 && count > 4;

                        div()
                            .relative()
                            .w(px(tile_w))
                            .h(px(tile_h))
                            .rounded(px(6.0))
                            .bg(rgb(item.preview_color))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .text_size(px(24.0))
                                    .child(if item.media_type == MediaType::Video { "🎬" } else { "🖼️" }),
                            )
                            // Overflow overlay (+N more items)
                            .when(is_last_tile, |el| {
                                el.child(
                                    div()
                                        .absolute()
                                        .size_full()
                                        .rounded(px(6.0))
                                        .bg(rgba(0x000000AA))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0xFFFFFF))
                                        .child(format!("+{}", count - 3)),
                                )
                            })
                    })),
            )
            // Caption or Timestamp
            .child(
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
                            .child(caption.unwrap_or("").to_string()),
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
