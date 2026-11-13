//! Upstream C++ style sheet mapping: `Telegram/SourceFiles/media/player/media_player.style:L1-L352`
//! Target Symbol: `MediaPlayerPlayerStyleSheet`
//! Subsystem: `media` | Feature: `TCG-N08`
#![allow(dead_code)]

use gpui::{px, rgb, Pixels, Rgba};

/// Authentic GPUI style metrics corresponding to `media_player.style:L83-L200`.
#[derive(Debug, Clone, Copy)]
pub struct MediaPlayerPlayerStyleSheet {
    pub wide_width: Pixels,
    pub height: Pixels,
    pub padding: Pixels,
    pub name_top: Pixels,
    pub play_left: Pixels,
    pub play_skip: Pixels,
    pub play_top: Pixels,
    pub close_right: Pixels,
    pub repeat_width: Pixels,
    pub repeat_height: Pixels,
    pub repeat_icon_x: Pixels,
    pub repeat_icon_y: Pixels,
    pub repeat_ripple_size: Pixels,
    pub play_icon_size: Pixels,
    pub play_icon_x: Pixels,
    pub play_icon_y: Pixels,
    pub play_icon_width: Pixels,
    pub play_icon_height: Pixels,
    pub play_icon_radius: f32,
    pub pause_icon_x: Pixels,
    pub pause_icon_y: Pixels,
    pub pause_bar_width: Pixels,
    pub pause_radius: f32,
    pub animation_duration_ms: u32,
    pub bg_color: Rgba,
    pub active_fg: Rgba,
    pub inactive_fg: Rgba,
}

impl Default for MediaPlayerPlayerStyleSheet {
    fn default() -> Self {
        Self {
            wide_width: px(460.0),
            height: px(35.0),
            padding: px(8.0),
            name_top: px(22.0),
            play_left: px(9.0),
            play_skip: px(1.0),
            play_top: px(0.0),
            close_right: px(0.0),
            repeat_width: px(30.0),
            repeat_height: px(30.0),
            repeat_icon_x: px(2.0),
            repeat_icon_y: px(5.0),
            repeat_ripple_size: px(24.0),
            play_icon_size: px(24.0),
            play_icon_x: px(7.0),
            play_icon_y: px(4.0),
            play_icon_width: px(12.0),
            play_icon_height: px(16.0),
            play_icon_radius: 1.5,
            pause_icon_x: px(7.0),
            pause_icon_y: px(6.0),
            pause_bar_width: px(4.0),
            pause_radius: 1.0,
            animation_duration_ms: 200,
            bg_color: rgb(0xFFFFFF),
            active_fg: rgb(0x2AABEE),
            inactive_fg: rgb(0x888888),
        }
    }
}

impl MediaPlayerPlayerStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }
}
