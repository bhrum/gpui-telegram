//! Authentic GPUI theme stylesheet for Telegram general dialog boxes.
//! Upstream reference: `Telegram/SourceFiles/boxes/boxes.style:L1-L773`.

use gpui::{px, rgb, Pixels, Rgba};

#[derive(Debug, Clone)]
pub struct BoxesStyleSheet {
    pub box_width: Pixels,
    pub box_wide_width: Pixels,
    pub box_radius: Pixels,
    pub row_padding: (Pixels, Pixels, Pixels, Pixels),
    pub button_padding: (Pixels, Pixels, Pixels, Pixels),
    pub button_height: Pixels,
    pub title_font_size: Pixels,
    pub title_font_weight_bold: bool,
    pub divider_width: Pixels,
    pub bg_color: Rgba,
    pub title_fg: Rgba,
    pub text_fg: Rgba,
    pub accent_fg: Rgba,
    pub border_color: Rgba,
}

impl Default for BoxesStyleSheet {
    fn default() -> Self {
        Self {
            box_width: px(380.0),
            box_wide_width: px(420.0),
            box_radius: px(8.0),
            row_padding: (px(24.0), px(16.0), px(24.0), px(16.0)),
            button_padding: (px(12.0), px(8.0), px(12.0), px(8.0)),
            button_height: px(36.0),
            title_font_size: px(16.0),
            title_font_weight_bold: true,
            divider_width: px(1.0),
            bg_color: rgb(0xFFFFFF),
            title_fg: rgb(0x222222),
            text_fg: rgb(0x707579),
            accent_fg: rgb(0x2AABEE),
            border_color: rgb(0xE0E0E0),
        }
    }
}

impl BoxesStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn wide_width(&self) -> Pixels {
        self.box_wide_width
    }

    pub fn standard_width(&self) -> Pixels {
        self.box_width
    }

    pub fn action_button_height(&self) -> Pixels {
        self.button_height
    }
}
