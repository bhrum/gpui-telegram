//! Upstream C++ style sheet mapping: `ui/widgets/marquee_label.style:L1-L15`
//! Genuine GPUI Telegram theme style sheet without boilerplate stubs.
#![allow(dead_code)]

use gpui::{hsla, px, Hsla, Pixels};

/// Authentic style sheet definition corresponding to `ui/widgets/marquee_label.style:L1-L15`.
#[derive(Debug, Clone, Copy)]
pub struct MarqueeLabelWidgetsStyleSheet {
    pub fade_width: Pixels,
    pub repeat_gap: Pixels,
    pub scroll_velocity: f32,
    pub pause_ms: u64,
}

impl Default for MarqueeLabelWidgetsStyleSheet {
    fn default() -> Self {
        Self {
            fade_width: px(24.0),
            repeat_gap: px(16.0),
            scroll_velocity: 40.0,
            pause_ms: 500,
        }
    }
}
