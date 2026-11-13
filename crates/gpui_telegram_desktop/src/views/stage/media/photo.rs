//! Authentic GPUI Photo Message View.
//!
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_photo.h`
//! - `Telegram/SourceFiles/history/view/media/history_view_photo.cpp`

use gpui::*;
use gpui::prelude::FluentBuilder;
use super::MediaBubbleRounding;

/// Constants from `history_view_photo.cpp:L53-L54`
pub const STORY_WIDTH: u32 = 720;
pub const STORY_HEIGHT: u32 = 1280;
pub const K_STORY_WIDTH: u32 = STORY_WIDTH;
pub const K_STORY_HEIGHT: u32 = STORY_HEIGHT;
pub const PHOTO_MIN_SIZE: f32 = 100.0;
pub const PHOTO_MAX_SIZE: f32 = 420.0;
pub const PHOTO_BUBBLE_MIN_WIDTH: f32 = 160.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhotoAspectRatio {
    pub width: u32,
    pub height: u32,
}

impl Default for PhotoAspectRatio {
    fn default() -> Self {
        Self { width: 4, height: 3 }
    }
}

impl PhotoAspectRatio {
    pub const STORY: Self = Self {
        width: STORY_WIDTH,
        height: STORY_HEIGHT,
    };

    pub fn ratio(&self) -> f32 {
        if self.height > 0 {
            self.width as f32 / self.height as f32
        } else {
            1.0
        }
    }

    pub fn is_story(&self) -> bool {
        self.width == STORY_WIDTH && self.height == STORY_HEIGHT
    }
}

#[derive(Debug, Clone)]
pub struct PhotoMediaState {
    pub photo_id: u64,
    pub dimensions: PhotoAspectRatio,
    pub caption: Option<String>,
    pub is_spoiler: bool,
    pub is_revealed: bool,
    pub is_sensitive: bool,
    pub paid_stars: Option<u64>,
    pub ttl_seconds: Option<u32>,
    pub rounding: MediaBubbleRounding,
    pub is_enlarged: bool,
    pub is_outgoing: bool,
}

impl PhotoMediaState {
    pub fn new(photo_id: u64, width: u32, height: u32) -> Self {
        Self {
            photo_id,
            dimensions: PhotoAspectRatio { width, height },
            caption: None,
            is_spoiler: false,
            is_revealed: false,
            is_sensitive: false,
            paid_stars: None,
            ttl_seconds: None,
            rounding: MediaBubbleRounding::default(),
            is_enlarged: false,
            is_outgoing: false,
        }
    }

    pub fn compute_render_size(&self, max_container_width: f32) -> (f32, f32) {
        let max_w = max_container_width.clamp(PHOTO_MIN_SIZE, PHOTO_MAX_SIZE);
        let ratio = self.dimensions.ratio();
        if self.dimensions.is_story() {
            let h = (max_w / ratio).clamp(240.0, 480.0);
            let w = h * ratio;
            (w, h)
        } else {
            let w = max_w;
            let h = (w / ratio).clamp(120.0, 380.0);
            (w, h)
        }
    }
}

pub struct PhotoMediaView;

impl PhotoMediaView {
    pub fn render(state: &PhotoMediaState) -> impl IntoElement {
        let (render_w, render_h) = state.compute_render_size(360.0);
        let show_spoiler_blur = state.is_spoiler && !state.is_revealed;
        let bg_color = if state.is_outgoing {
            rgb(0x2B5278)
        } else {
            rgb(0x182533)
        };

        div()
            .flex()
            .flex_col()
            .w(px(render_w))
            .rounded(px(state.rounding.top_left))
            .bg(bg_color)
            .overflow_hidden()
            // Photo viewport container
            .child(
                div()
                    .relative()
                    .w(px(render_w))
                    .h(px(render_h))
                    .bg(rgb(0x101921))
                    .flex()
                    .items_center()
                    .justify_center()
                    // Inner image or graphic simulation
                    .child(
                        div()
                            .text_size(px(40.0))
                            .text_color(rgb(0x3B5066))
                            .child(if state.dimensions.is_story() { "📱" } else { "🖼️" }),
                    )
                    // Spoiler overlay layer
                    .when(show_spoiler_blur, |el| {
                        el.child(
                            div()
                                .absolute()
                                .inset_0()
                                .bg(rgba(0x000000B0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(
                                    div()
                                        .px(px(10.0))
                                        .py(px(4.0))
                                        .rounded_full()
                                        .bg(rgba(0x2B5278DD))
                                        .text_size(px(12.0))
                                        .text_color(rgb(0xFFFFFF))
                                        .child(if state.is_sensitive {
                                            "👁️ Sensitive Content".to_string()
                                        } else if let Some(stars) = state.paid_stars {
                                            format!("⭐ {} Stars", stars)
                                        } else {
                                            "✨ Tap to reveal".to_string()
                                        }),
                                ),
                        )
                    })
                    // TTL self-destruct indicator
                    .when_some(state.ttl_seconds, |el, ttl| {
                        el.child(
                            div()
                                .absolute()
                                .top(px(8.0))
                                .right(px(8.0))
                                .px(px(6.0))
                                .py(px(2.0))
                                .rounded(px(6.0))
                                .bg(rgba(0xE05545CC))
                                .text_size(px(10.0))
                                .text_color(rgb(0xFFFFFF))
                                .child(format!("⏱ {}s", ttl)),
                        )
                    }),
            )
            // Optional Caption below image
            .when_some(state.caption.as_ref(), |el, caption| {
                el.child(
                    div()
                        .p(px(8.0))
                        .text_size(px(13.0))
                        .text_color(rgb(0xFFFFFF))
                        .child(caption.clone()),
                )
            })
    }
}
