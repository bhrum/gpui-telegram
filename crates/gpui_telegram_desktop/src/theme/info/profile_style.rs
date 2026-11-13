//! Profile Theme Style Sheet
//! Upstream: Telegram/SourceFiles/profile/profile.style
//! Exact metrics and tokens for profile blocks and cover drag & drop upload target areas.

use gpui::{px, rgb, Pixels, Point, Rgba};

#[derive(Debug, Clone)]
pub struct ProfileStyleSheet {
    pub drop_area_title_top: Pixels,
    pub drop_area_subtitle_top: Pixels,
    pub drop_area_border_width: Pixels,
    pub drop_area_duration: u32,
    pub block_margin_top: Pixels,
    pub block_title_height: Pixels,
    pub block_title_pos: Point<Pixels>,
    pub bg_color: Rgba,
    pub text_fg: Rgba,
    pub drop_area_fg: Rgba,
    pub bold_fg: Rgba,
}

impl Default for ProfileStyleSheet {
    fn default() -> Self {
        Self {
            drop_area_title_top: px(30.0),
            drop_area_subtitle_top: px(68.0),
            drop_area_border_width: px(3.0),
            drop_area_duration: 200,
            block_margin_top: px(14.0),
            block_title_height: px(24.0),
            block_title_pos: Point { x: px(24.0), y: px(0.0) },
            bg_color: rgb(0xFFFFFF),
            text_fg: rgb(0x222222),
            drop_area_fg: rgb(0x2AABEE),
            bold_fg: rgb(0x000000),
        }
    }
}

impl ProfileStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_style_sheet_metrics() {
        let style = ProfileStyleSheet::new();
        assert_eq!(style.drop_area_title_top, px(30.0));
        assert_eq!(style.drop_area_subtitle_top, px(68.0));
        assert_eq!(style.drop_area_border_width, px(3.0));
        assert_eq!(style.drop_area_duration, 200);
        assert_eq!(style.block_margin_top, px(14.0));
        assert_eq!(style.block_title_height, px(24.0));
    }
}
