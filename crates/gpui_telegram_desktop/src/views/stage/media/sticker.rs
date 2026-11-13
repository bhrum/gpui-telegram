//! Authentic GPUI Sticker Message View.
//!
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_sticker.h`
//! - `Telegram/SourceFiles/history/view/media/history_view_sticker.cpp`
//! - `Telegram/SourceFiles/history/view/media/history_view_sticker_player.h`
//! - `Telegram/SourceFiles/history/view/media/history_view_sticker_player_abstract.h`

use gpui::*;
use gpui::prelude::FluentBuilder;

/// Constants from `history_view_sticker.cpp:L44-L49`
pub const MAX_SIZE_FIXED: u32 = 512;
pub const MAX_EMOJI_SIZE_FIXED: u32 = 256;
pub const PREMIUM_MULTIPLIER: f32 = 1.49; // 1.0 + 0.245 * 2
pub const EMOJI_MULTIPLIER: u32 = 3;
pub const MESSAGE_EFFECT_MULTIPLIER: u32 = 2;

pub const K_MAX_SIZE_FIXED: u32 = MAX_SIZE_FIXED;
pub const K_MAX_EMOJI_SIZE_FIXED: u32 = MAX_EMOJI_SIZE_FIXED;
pub const K_PREMIUM_MULTIPLIER: f32 = PREMIUM_MULTIPLIER;
pub const K_EMOJI_MULTIPLIER: u32 = EMOJI_MULTIPLIER;
pub const K_MESSAGE_EFFECT_MULTIPLIER: u32 = MESSAGE_EFFECT_MULTIPLIER;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StickerFormat {
    #[default]
    StaticWebp,
    LottieTgs,
    VideoWebm,
}

#[derive(Debug, Clone)]
pub struct StickerPlaybackState {
    pub document_id: u64,
    pub set_id: Option<u64>,
    pub set_short_name: Option<String>,
    pub format: StickerFormat,
    pub frame_index: i32,
    pub frames_count: i32,
    pub is_playing: bool,
    pub play_once: bool,
    pub stop_on_last_frame: bool,
    pub custom_size: Option<f32>,
    pub dice_info: Option<(String, i32)>, // (emoji, value)
    pub is_premium: bool,
    pub alt_emoji: Option<String>,
}

impl StickerPlaybackState {
    pub fn new(document_id: u64, format: StickerFormat) -> Self {
        Self {
            document_id,
            set_id: None,
            set_short_name: None,
            format,
            frame_index: 0,
            frames_count: if format == StickerFormat::StaticWebp { 1 } else { 60 },
            is_playing: format != StickerFormat::StaticWebp,
            play_once: false,
            stop_on_last_frame: false,
            custom_size: None,
            dice_info: None,
            is_premium: false,
            alt_emoji: None,
        }
    }

    /// Check if sticker playback reached the final frame.
    /// Upstream: `Sticker::atTheEnd()` (`history_view_sticker.h:L76-L78`)
    pub fn at_the_end(&self) -> bool {
        self.frame_index >= 0 && (self.frame_index + 1 >= self.frames_count)
    }

    pub fn advance_frame(&mut self) {
        if !self.is_playing {
            return;
        }
        if self.at_the_end() {
            if self.stop_on_last_frame || self.play_once {
                self.is_playing = false;
            } else {
                self.frame_index = 0;
            }
        } else {
            self.frame_index += 1;
        }
    }

    pub fn compute_size(&self) -> f32 {
        if let Some(sz) = self.custom_size {
            return sz;
        }
        let base_size = 180.0f32;
        if self.is_premium {
            base_size * PREMIUM_MULTIPLIER
        } else {
            base_size
        }
    }
}

pub struct StickerMediaView;

impl StickerMediaView {
    pub fn render(state: &StickerPlaybackState) -> impl IntoElement {
        let size = state.compute_size();
        let display_char = if let Some((ref emoji, val)) = state.dice_info {
            format!("{} [{}]", emoji, val)
        } else if let Some(ref alt) = state.alt_emoji {
            alt.clone()
        } else {
            "🧸".to_string()
        };

        div()
            .relative()
            .w(px(size))
            .h(px(size))
            .flex()
            .items_center()
            .justify_center()
            // Unwrapped - no message bubble background!
            .child(
                div()
                    .text_size(px(size * 0.65))
                    .child(display_char),
            )
            // Premium star badge indicator if premium sticker
            .when(state.is_premium, |el| {
                el.child(
                    div()
                        .absolute()
                        .bottom(px(4.0))
                        .right(px(4.0))
                        .w(px(18.0))
                        .h(px(18.0))
                        .rounded_full()
                        .bg(rgba(0xE0A030EE))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(10.0))
                        .child("⭐"),
                )
            })
    }
}
