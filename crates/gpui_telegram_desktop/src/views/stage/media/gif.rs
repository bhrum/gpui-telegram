//! Authentic GPUI GIF & Video Message View.
//!
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_gif.h`
//! - `Telegram/SourceFiles/history/view/media/history_view_gif.cpp`
//! - `Telegram/SourceFiles/history/view/media/history_view_video_status.h`
//! - `Telegram/SourceFiles/history/view/media/history_view_video_status.cpp`

use gpui::*;
use gpui::prelude::FluentBuilder;
use super::MediaBubbleRounding;

/// Constants from `history_view_gif.cpp:L75-L79`
pub const MAX_INLINE_AREA: u64 = 1920 * 1080;
pub const MAX_INSTANT_VIEW_INLINE_AREA: u64 = 1920 * 1920;
pub const SEEK_PREVIEW_INTERVAL_MS: u64 = 100;
pub const MAX_GIF_FORWARDED_BAR_LINES: usize = 4;
pub const USE_NON_BLURRED_THRESHOLD: u32 = 240;

pub const K_MAX_INLINE_AREA: u64 = MAX_INLINE_AREA;
pub const K_MAX_INSTANT_VIEW_INLINE_AREA: u64 = MAX_INSTANT_VIEW_INLINE_AREA;
pub const K_SEEK_PREVIEW_INTERVAL_MS: u64 = SEEK_PREVIEW_INTERVAL_MS;
pub const K_MAX_GIF_FORWARDED_BAR_LINES: usize = MAX_GIF_FORWARDED_BAR_LINES;
pub const K_USE_NON_BLURRED_THRESHOLD: u32 = USE_NON_BLURRED_THRESHOLD;

#[derive(Debug, Clone)]
pub struct VideoCornerStatusInfo {
    pub text: String,
    pub download_size: Option<String>,
    pub is_downloading: bool,
    pub is_muted: bool,
}

impl Default for VideoCornerStatusInfo {
    fn default() -> Self {
        Self {
            text: "GIF".to_string(),
            download_size: None,
            is_downloading: false,
            is_muted: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GifMediaState {
    pub document_id: u64,
    pub width: u32,
    pub height: u32,
    pub duration_seconds: u32,
    pub is_looping: bool,
    pub is_playing: bool,
    pub is_spoiler: bool,
    pub is_revealed: bool,
    pub is_video_message_round: bool,
    pub playback_progress: f32, // 0.0 .. 1.0
    pub corner_status: VideoCornerStatusInfo,
    pub caption: Option<String>,
    pub rounding: MediaBubbleRounding,
    pub is_outgoing: bool,
}

impl GifMediaState {
    pub fn new(document_id: u64, width: u32, height: u32, duration: u32) -> Self {
        let is_gif = duration <= 1;
        let status_text = if is_gif {
            "GIF".to_string()
        } else {
            format!("{:02}:{:02}", duration / 60, duration % 60)
        };
        Self {
            document_id,
            width,
            height,
            duration_seconds: duration,
            is_looping: is_gif,
            is_playing: true,
            is_spoiler: false,
            is_revealed: false,
            is_video_message_round: false,
            playback_progress: 0.0,
            corner_status: VideoCornerStatusInfo {
                text: status_text,
                download_size: None,
                is_downloading: false,
                is_muted: true,
            },
            caption: None,
            rounding: MediaBubbleRounding::default(),
            is_outgoing: false,
        }
    }

    pub fn compute_render_size(&self, max_w: f32) -> (f32, f32) {
        if self.is_video_message_round {
            let s = 240.0f32;
            return (s, s);
        }
        let area = (self.width as u64) * (self.height as u64);
        let (eff_w, eff_h) = if area > MAX_INLINE_AREA {
            let scale = (MAX_INLINE_AREA as f64 / area as f64).sqrt();
            (
                (self.width as f64 * scale).round() as f32,
                (self.height as f64 * scale).round() as f32,
            )
        } else {
            (self.width as f32, self.height as f32)
        };
        let ratio = if eff_h > 0.0 { eff_w / eff_h } else { 1.33 };
        let w = max_w.clamp(160.0, 420.0);
        let h = (w / ratio).clamp(120.0, 380.0);
        (w, h)
    }
}

pub struct GifMediaView;

impl GifMediaView {
    pub fn render(state: &GifMediaState) -> impl IntoElement {
        let (render_w, render_h) = state.compute_render_size(360.0);
        let show_spoiler = state.is_spoiler && !state.is_revealed;
        let is_round = state.is_video_message_round;
        let bg_color = if state.is_outgoing {
            rgb(0x2B5278)
        } else {
            rgb(0x182533)
        };

        div()
            .flex()
            .flex_col()
            .w(px(render_w))
            .when(!is_round, |el| el.rounded(px(state.rounding.top_left)))
            .when(is_round, |el| el.rounded_full())
            .bg(bg_color)
            .overflow_hidden()
            // Viewport
            .child(
                div()
                    .relative()
                    .w(px(render_w))
                    .h(px(render_h))
                    .bg(rgb(0x0C141C))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_size(px(36.0))
                            .child(if is_round { "⏺️" } else { "🎬" }),
                    )
                    // Bottom-left corner duration / GIF status pill
                    .child(
                        div()
                            .absolute()
                            .bottom(px(8.0))
                            .left(px(8.0))
                            .px(px(6.0))
                            .py(px(2.0))
                            .rounded(px(4.0))
                            .bg(rgba(0x00000088))
                            .text_size(px(11.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child(state.corner_status.text.clone()),
                    )
                    // Bottom-right sound mute indicator
                    .when(state.corner_status.is_muted && !state.is_looping, |el| {
                        el.child(
                            div()
                                .absolute()
                                .bottom(px(8.0))
                                .right(px(8.0))
                                .w(px(20.0))
                                .h(px(20.0))
                                .rounded_full()
                                .bg(rgba(0x00000088))
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_size(px(10.0))
                                .child("🔇"),
                        )
                    })
                    // Circular Scrubber Track for Round Video
                    .when(is_round, |el| {
                        el.child(
                            div()
                                .absolute()
                                .inset_0()
                                .rounded_full()
                                .border_2()
                                .border_color(rgba(0x5288C188)),
                        )
                    })
                    // Spoiler Cover
                    .when(show_spoiler, |el| {
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
                                        .child("✨ Tap to reveal"),
                                ),
                        )
                    }),
            )
            // Optional Caption
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
