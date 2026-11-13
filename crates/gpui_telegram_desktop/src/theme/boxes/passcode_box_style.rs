//! Authentic GPUI theme stylesheet for Passcode & Local Lock Box.
//! Upstream reference: `Telegram/SourceFiles/boxes/passcode_box.style:L1-L39`.

use gpui::{px, rgb, Pixels, Rgba};

#[derive(Debug, Clone)]
pub struct PasscodeBoxStyleSheet {
    pub padding: (Pixels, Pixels, Pixels, Pixels),
    pub text_line_height: Pixels,
    pub little_skip: Pixels,
    pub about_skip: Pixels,
    pub dot_radius: Pixels,
    pub input_field_width: Pixels,
    pub error_color: Rgba,
    pub input_bg: Rgba,
    pub text_fg: Rgba,
    pub dot_color: Rgba,
}

impl Default for PasscodeBoxStyleSheet {
    fn default() -> Self {
        Self {
            padding: (px(24.0), px(16.0), px(24.0), px(16.0)),
            text_line_height: px(28.0),
            little_skip: px(5.0),
            about_skip: px(7.0),
            dot_radius: px(6.0),
            input_field_width: px(260.0),
            error_color: rgb(0xDF3F40),
            input_bg: rgb(0xF4F4F5),
            text_fg: rgb(0x1F2937),
            dot_color: rgb(0x4B5563),
        }
    }
}

impl PasscodeBoxStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn input_width(&self) -> Pixels {
        self.input_field_width
    }
}
