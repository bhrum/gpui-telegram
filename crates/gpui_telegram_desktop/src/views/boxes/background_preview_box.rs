//! Authentic GPUI implementation of Telegram Desktop's Background Preview Box.
//! Upstream reference: `Telegram/SourceFiles/boxes/background_preview_box.h:L41-L150`
//! Upstream implementation: `Telegram/SourceFiles/boxes/background_preview_box.cpp:L58-L60`, `L400-L560`, and `background_preview_box.style:L10-L57`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::sync::Arc;

pub const K_MAX_WALL_PAPER_SLUG_LENGTH: usize = 255;

#[derive(Clone)]
pub struct BackgroundPreviewBox {
    pub wallpaper_id: u64,
    pub title: String,
    pub is_blurred: bool,
    pub dimming_percent: u32,
    pub apply_for_both: bool,
    pub peer_name: Option<String>,
    pub is_closed: bool,
    pub on_apply: Option<Arc<dyn Fn(bool, u32, bool) + Send + Sync>>,
    pub on_close: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl BackgroundPreviewBox {
    pub fn new(wallpaper_id: u64, title: impl Into<String>, peer_name: Option<String>) -> Self {
        Self {
            wallpaper_id,
            title: title.into(),
            is_blurred: false,
            dimming_percent: 20,
            apply_for_both: false,
            peer_name,
            is_closed: false,
            on_apply: None,
            on_close: None,
        }
    }

    pub fn toggle_blur(&mut self) {
        self.is_blurred = !self.is_blurred;
    }

    pub fn set_dimming(&mut self, dimming: u32) {
        self.dimming_percent = dimming.min(100);
    }

    pub fn toggle_apply_for_both(&mut self) {
        self.apply_for_both = !self.apply_for_both;
    }

    pub fn apply(&mut self) {
        if let Some(ref cb) = self.on_apply {
            cb(self.is_blurred, self.dimming_percent, self.apply_for_both);
        }
        self.close();
    }

    pub fn close(&mut self) {
        self.is_closed = true;
        if let Some(ref cb) = self.on_close {
            cb();
        }
    }
}

impl Default for BackgroundPreviewBox {
    fn default() -> Self {
        Self::new(1, "Classic Doodles", Some("Alice".into()))
    }
}

impl Render for BackgroundPreviewBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        let has_peer = self.peer_name.is_some();

        div()
            .id("background_preview_box")
            .w(px(380.0))
            .p(px(20.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(12.0))
            .shadow_lg()
            .flex()
            .flex_col()
            .gap(px(14.0))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x222222))
                            .child("Wallpaper Preview"),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .text_size(px(14.0))
                            .text_color(rgb(0x888888))
                            .child("✕"),
                    ),
            )
            .child(
                div()
                    .h(px(180.0))
                    .w_full()
                    .rounded(px(10.0))
                    .bg(rgb(0x1B2A4A))
                    .p(px(12.0))
                    .flex()
                    .flex_col()
                    .justify_end()
                    .gap(px(8.0))
                    .child(
                        div()
                            .max_w(px(240.0))
                            .p(px(8.0))
                            .rounded(px(8.0))
                            .bg(rgb(0xFFFFFF))
                            .text_size(px(12.0))
                            .text_color(rgb(0x222222))
                            .child("Hello! How does this wallpaper look?"),
                    )
                    .child(
                        div()
                            .flex()
                            .justify_end()
                            .child(
                                div()
                                    .max_w(px(240.0))
                                    .p(px(8.0))
                                    .rounded(px(8.0))
                                    .bg(rgb(0xEEF8FF))
                                    .text_size(px(12.0))
                                    .text_color(rgb(0x222222))
                                    .child("Looks awesome! Very clear and legible."),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .cursor_pointer()
                            .child(
                                div()
                                    .size(px(16.0))
                                    .rounded(px(4.0))
                                    .border_1()
                                    .border_color(rgb(0x2AABEE))
                                    .bg(if self.is_blurred { rgb(0x2AABEE) } else { rgb(0xFFFFFF) })
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(if self.is_blurred { div().text_size(px(11.0)).text_color(rgb(0xFFFFFF)).child("✓") } else { div() }),
                            )
                            .child(div().text_size(px(13.0)).text_color(rgb(0x444444)).child("Blurred")),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(0x888888))
                            .child(format!("Dimming: {}%", self.dimming_percent)),
                    ),
            )
            .child(
                if has_peer {
                    let label = format!("Also set for {}", self.peer_name.as_ref().unwrap());
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .cursor_pointer()
                        .child(
                            div()
                                .size(px(16.0))
                                .rounded(px(4.0))
                                .border_1()
                                .border_color(rgb(0x2AABEE))
                                .bg(if self.apply_for_both { rgb(0x2AABEE) } else { rgb(0xFFFFFF) })
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(if self.apply_for_both { div().text_size(px(11.0)).text_color(rgb(0xFFFFFF)).child("✓") } else { div() }),
                        )
                        .child(div().text_size(px(12.0)).text_color(rgb(0x444444)).child(label))
                } else {
                    div()
                },
            )
            .child(
                div()
                    .w_full()
                    .py(px(10.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x2AABEE))
                    .cursor_pointer()
                    .flex()
                    .justify_center()
                    .items_center()
                    .child(
                        div()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child("Set Wallpaper"),
                    ),
            )
            .into_any_element()
    }
}
