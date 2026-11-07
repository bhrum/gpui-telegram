//! Profile Top Bar Theme Style Sheet
//! Upstream: Telegram/SourceFiles/info/profile/info_profile_top_bar.style
//! Exact metrics and offsets for profile header cover, action icon buttons and balance pills.

use gpui::{px, rgb, Pixels, Rgba};

#[derive(Debug, Clone)]
pub struct InfoProfileTopBarProfileStyleSheet {
    pub title_top: Pixels,
    pub status_top: Pixels,
    pub photo_top: Pixels,
    pub photo_bg_shift: Pixels,
    pub photo_bg_no_actions_shift: Pixels,
    pub photo_size: Pixels,
    pub gift_size: Pixels,
    pub action_button_space: Pixels,
    pub action_menu_skip: Pixels,
    pub topic_status_radius: Pixels,
    pub balance_skip: Pixels,
    pub balance_bottom_skip: Pixels,
    pub balance_pill_height: Pixels,
    pub balance_pill_icon_size: Pixels,
    pub balance_pill_colored_bg_opacity: f32,
    pub text_fg: Rgba,
    pub bold_fg: Rgba,
    pub active_fg: Rgba,
    pub toast_fg: Rgba,
}

impl Default for InfoProfileTopBarProfileStyleSheet {
    fn default() -> Self {
        Self {
            title_top: px(113.0),
            status_top: px(134.0),
            photo_top: px(24.0),
            photo_bg_shift: px(55.0),
            photo_bg_no_actions_shift: px(30.0),
            photo_size: px(80.0),
            gift_size: px(20.0),
            action_button_space: px(10.0),
            action_menu_skip: px(10.0),
            topic_status_radius: px(4.0),
            balance_skip: px(22.0),
            balance_bottom_skip: px(4.0),
            balance_pill_height: px(16.0),
            balance_pill_icon_size: px(12.0),
            balance_pill_colored_bg_opacity: 0.2,
            text_fg: rgb(0x222222),
            bold_fg: rgb(0x000000),
            active_fg: rgb(0x2AABEE),
            toast_fg: rgb(0xFFFFFF),
        }
    }
}

impl InfoProfileTopBarProfileStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_top_bar_style_metrics() {
        let style = InfoProfileTopBarProfileStyleSheet::new();
        assert_eq!(style.title_top, px(113.0));
        assert_eq!(style.status_top, px(134.0));
        assert_eq!(style.photo_top, px(24.0));
        assert_eq!(style.photo_size, px(80.0));
        assert_eq!(style.balance_pill_height, px(16.0));
    }
}
