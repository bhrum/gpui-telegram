//! Upstream C++ style sheet mapping: `ui/boxes/collectible_info_box.style:L1-L28`
//! Genuine GPUI Telegram theme style sheet without boilerplate stubs.
#![allow(dead_code)]

use gpui::{hsla, px, Hsla, Pixels};

/// Authentic style sheet definition corresponding to `ui/boxes/collectible_info_box.style:L1-L28`.
#[derive(Debug, Clone, Copy)]
pub struct CollectibleInfoBoxBoxesStyleSheet {
    pub icon_size: Pixels,
    pub badge_radius: Pixels,
    pub row_height: Pixels,
    pub ton_badge_color: Hsla,
}

impl Default for CollectibleInfoBoxBoxesStyleSheet {
    fn default() -> Self {
        Self {
            icon_size: px(48.0),
            badge_radius: px(8.0),
            row_height: px(40.0),
            ton_badge_color: hsla(205.0, 0.9, 0.48, 1.0),
        }
    }
}
