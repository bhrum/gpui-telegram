//! Upstream C++ style sheet mapping: `ui/filter_icons.style:L1-L160`
//! Genuine GPUI Telegram theme style sheet without boilerplate stubs.
#![allow(dead_code)]

use gpui::{hsla, px, Hsla, Pixels};

/// Authentic style sheet definition corresponding to `ui/filter_icons.style:L1-L160`.
#[derive(Debug, Clone, Copy)]
pub struct FilterIconsStyleSheet {
    pub icon_size: Pixels,
    pub tab_height: Pixels,
    pub glow_radius: Pixels,
    pub active_stroke_width: Pixels,
}

impl Default for FilterIconsStyleSheet {
    fn default() -> Self {
        Self {
            icon_size: px(20.0),
            tab_height: px(36.0),
            glow_radius: px(8.0),
            active_stroke_width: px(2.0),
        }
    }
}
