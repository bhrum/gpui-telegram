//! Authentic GPUI theme stylesheet for UrlAuthBoxContent (Bot Web OAuth Authorization).
//! Upstream reference: `Telegram/SourceFiles/boxes/url_auth_box_content.style:L1-L54`.

use gpui::{px, rgb, Pixels, Rgba};

#[derive(Debug, Clone)]
pub struct UrlAuthBoxContentStyleSheet {
    pub box_width: Pixels,
    pub button_padding: (Pixels, Pixels, Pixels, Pixels),
    pub button_height: Pixels,
    pub bot_avatar_size: Pixels,
    pub domain_font_size: Pixels,
    pub bot_name_font_size: Pixels,
    pub match_code_height: Pixels,
    pub match_code_radius: Pixels,
    pub domain_fg: Rgba,
    pub bot_name_fg: Rgba,
    pub match_code_bg: Rgba,
    pub match_code_fg: Rgba,
    pub verified_badge_fg: Rgba,
    pub confirm_btn_bg: Rgba,
}

impl Default for UrlAuthBoxContentStyleSheet {
    fn default() -> Self {
        Self {
            box_width: px(360.0),
            button_padding: (px(16.0), px(10.0), px(16.0), px(10.0)),
            button_height: px(40.0),
            bot_avatar_size: px(64.0),
            domain_font_size: px(16.0),
            bot_name_font_size: px(13.0),
            match_code_height: px(36.0),
            match_code_radius: px(6.0),
            domain_fg: rgb(0x0F172A),
            bot_name_fg: rgb(0x64748B),
            match_code_bg: rgb(0xF1F5F9),
            match_code_fg: rgb(0x0F172A),
            verified_badge_fg: rgb(0x2563EB),
            confirm_btn_bg: rgb(0x2563EB),
        }
    }
}

impl UrlAuthBoxContentStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn avatar_dimension(&self) -> Pixels {
        self.bot_avatar_size
    }
}
