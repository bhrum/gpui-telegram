//! Upstream C++ style sheet mapping: `Telegram/SourceFiles/media/view/media_view.style:L1-L1295`
//! Target Symbol: `MediaViewViewStyleSheet`
//! Subsystem: `media` | Feature: `TCG-N08`
#![allow(dead_code)]

use gpui::{px, rgb, Pixels, Rgba};

/// Authentic GPUI style metrics corresponding to `media_view.style:L18-L120`.
#[derive(Debug, Clone, Copy)]
pub struct MediaViewViewStyleSheet {
    pub overlay_animation_duration_ms: u32,
    pub controller_width: Pixels,
    pub controller_height: Pixels,
    pub controller_radius: Pixels,
    pub playback_slider_height: Pixels,
    pub playback_top: Pixels,
    pub play_progress_top: Pixels,
    pub play_button_size: Pixels,
    pub play_button_icon_offset: Pixels,
    pub pip_button_size: Pixels,
    pub pip_button_icon_offset: Pixels,
    pub volume_slider_width: Pixels,
    pub timestamp_top: Pixels,
    pub timestamp_height: Pixels,
    pub bg_color: Rgba,
    pub playback_active_fg: Rgba,
    pub playback_inactive_fg: Rgba,
    pub icon_fg: Rgba,
    pub icon_fg_over: Rgba,
}

impl Default for MediaViewViewStyleSheet {
    fn default() -> Self {
        Self {
            overlay_animation_duration_ms: 150,
            controller_width: px(480.0),
            controller_height: px(72.0),
            controller_radius: px(9.0),
            playback_slider_height: px(3.0),
            playback_top: px(49.0),
            play_progress_top: px(46.0),
            play_button_size: px(40.0),
            play_button_icon_offset: px(8.0),
            pip_button_size: px(32.0),
            pip_button_icon_offset: px(4.0),
            volume_slider_width: px(75.0),
            timestamp_top: px(63.0),
            timestamp_height: px(10.0),
            bg_color: rgb(0x1E1E1E),
            playback_active_fg: rgb(0xFFFFFF),
            playback_inactive_fg: rgb(0x555555),
            icon_fg: rgb(0xFFFFFF),
            icon_fg_over: rgb(0xCCCCCC),
        }
    }
}

impl MediaViewViewStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }
}
