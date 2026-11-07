//! Profile Action Buttons & Info Row Tokens
//! Upstream: Telegram/SourceFiles/info/profile/info_profile_actions.style
//! Exact metrics, paddings and icon offsets for profile contact/channel action buttons.

use gpui::{px, rgb, Pixels, Point, Rgba};

#[derive(Debug, Clone)]
pub struct InfoProfileActionsProfileStyleSheet {
    pub labeled_padding_left: Pixels,
    pub labeled_padding_top: Pixels,
    pub personal_channel_padding_left: Pixels,
    pub channel_members_icon_pos: Point<Pixels>,
    pub channel_admins_icon_pos: Point<Pixels>,
    pub earn_credits_icon_pos: Point<Pixels>,
    pub earn_currency_icon_pos: Point<Pixels>,
    pub open_app_margin_horizontal: Pixels,
    pub open_app_margin_vertical: Pixels,
    pub block_button_skip: Pixels,
    pub hours_outer_margin_horizontal: Pixels,
    pub hours_day_skip: Pixels,
    pub hours_arrow_size: Pixels,
    pub text_fg: Rgba,
    pub sub_text_fg: Rgba,
    pub active_fg: Rgba,
    pub attention_fg: Rgba,
}

impl Default for InfoProfileActionsProfileStyleSheet {
    fn default() -> Self {
        Self {
            labeled_padding_left: px(23.0),
            labeled_padding_top: px(9.0),
            personal_channel_padding_left: px(79.0),
            channel_members_icon_pos: Point { x: px(20.0), y: px(4.0) },
            channel_admins_icon_pos: Point { x: px(24.0), y: px(7.0) },
            earn_credits_icon_pos: Point { x: px(24.0), y: px(7.0) },
            earn_currency_icon_pos: Point { x: px(20.0), y: px(3.0) },
            open_app_margin_horizontal: px(16.0),
            open_app_margin_vertical: px(12.0),
            block_button_skip: px(8.0),
            hours_outer_margin_horizontal: px(8.0),
            hours_day_skip: px(6.0),
            hours_arrow_size: px(4.0),
            text_fg: rgb(0x222222),
            sub_text_fg: rgb(0x707579),
            active_fg: rgb(0x2AABEE),
            attention_fg: rgb(0xE04F54),
        }
    }
}

impl InfoProfileActionsProfileStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_actions_style_metrics() {
        let style = InfoProfileActionsProfileStyleSheet::new();
        assert_eq!(style.labeled_padding_left, px(23.0));
        assert_eq!(style.personal_channel_padding_left, px(79.0));
        assert_eq!(style.open_app_margin_horizontal, px(16.0));
        assert_eq!(style.block_button_skip, px(8.0));
    }
}
