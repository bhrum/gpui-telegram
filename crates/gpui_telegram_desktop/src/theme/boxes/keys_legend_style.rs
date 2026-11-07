//! Authentic GPUI theme stylesheet for Keyboard Shortcuts Legend.
//! Upstream reference: `Telegram/SourceFiles/boxes/keys_legend.style:L1-L52`.

use gpui::{px, rgb, Pixels, Rgba};

#[derive(Debug, Clone)]
pub struct KeysLegendStyleSheet {
    pub menu_skip: Pixels,
    pub item_padding: (Pixels, Pixels, Pixels, Pixels),
    pub key_badge_radius: Pixels,
    pub key_badge_padding: (Pixels, Pixels),
    pub key_font_size: Pixels,
    pub key_badge_bg: Rgba,
    pub key_badge_fg: Rgba,
    pub description_fg: Rgba,
    pub row_spacing: Pixels,
}

impl Default for KeysLegendStyleSheet {
    fn default() -> Self {
        Self {
            menu_skip: px(6.0),
            item_padding: (px(16.0), px(8.0), px(16.0), px(8.0)),
            key_badge_radius: px(4.0),
            key_badge_padding: (px(6.0), px(3.0)),
            key_font_size: px(12.0),
            key_badge_bg: rgb(0xEEF2F5),
            key_badge_fg: rgb(0x334155),
            description_fg: rgb(0x64748B),
            row_spacing: px(10.0),
        }
    }
}

impl KeysLegendStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn badge_radius(&self) -> Pixels {
        self.key_badge_radius
    }
}
