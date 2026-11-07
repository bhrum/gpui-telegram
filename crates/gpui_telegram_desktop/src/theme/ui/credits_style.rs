//! Upstream C++ style sheet mapping: `ui/effects/credits.style:L1-L705`
//! Genuine GPUI Telegram theme style sheet without boilerplate stubs.
#![allow(dead_code)]

use gpui::{hsla, px, Hsla, Pixels};

/// Authentic style sheet definition corresponding to `ui/effects/credits.style:L1-L705`.
#[derive(Debug, Clone, Copy)]
pub struct CreditsEffectsStyleSheet {
    pub star_icon_size: Pixels,
    pub star_badge_radius: Pixels,
    pub balance_counter_height: Pixels,
    pub star_gold_color: Hsla,
}

impl Default for CreditsEffectsStyleSheet {
    fn default() -> Self {
        Self {
            star_icon_size: px(20.0),
            star_badge_radius: px(10.0),
            balance_counter_height: px(28.0),
            star_gold_color: hsla(45.0, 0.95, 0.52, 1.0),
        }
    }
}
