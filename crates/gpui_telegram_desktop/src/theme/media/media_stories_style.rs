//! Upstream C++ style sheet mapping: `Telegram/SourceFiles/media/stories/media_stories.style:L1-L81`
//! Target Symbol: `MediaStoriesStoriesStyleSheet`
//! Subsystem: `media` | Feature: `TCG-N38`
#![allow(dead_code)]

use gpui::{px, rgb, Pixels, Rgba};

/// Authentic GPUI style metrics corresponding to `media_stories.style:L26-L80`.
#[derive(Debug, Clone, Copy)]
pub struct MediaStoriesStoriesStyleSheet {
    pub button_height: Pixels,
    pub button_padding: Pixels,
    pub button_text_top: Pixels,
    pub margin_top: Pixels,
    pub margin_bottom: Pixels,
    pub close_button_size: Pixels,
    pub close_ripple_size: Pixels,
    pub close_ripple_pos: Pixels,
    pub feature_title_height: Pixels,
    pub bg_color: Rgba,
    pub text_fg: Rgba,
    pub sub_text_fg: Rgba,
    pub active_bg: Rgba,
    pub active_fg: Rgba,
}

impl Default for MediaStoriesStoriesStyleSheet {
    fn default() -> Self {
        Self {
            button_height: px(42.0),
            button_padding: px(10.0),
            button_text_top: px(12.0),
            margin_top: px(56.0),
            margin_bottom: px(10.0),
            close_button_size: px(34.0),
            close_ripple_size: px(40.0),
            close_ripple_pos: px(4.0),
            feature_title_height: px(20.0),
            bg_color: rgb(0xFFFFFF),
            text_fg: rgb(0x222222),
            sub_text_fg: rgb(0x707579),
            active_bg: rgb(0x2AABEE),
            active_fg: rgb(0xFFFFFF),
        }
    }
}

impl MediaStoriesStoriesStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }
}
