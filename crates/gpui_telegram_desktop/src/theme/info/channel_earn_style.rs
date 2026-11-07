//! Channel & Bot Earn Theme Style Sheet
//! Upstream: Telegram/SourceFiles/info/channel_statistics/earn/channel_earn.style
//! Exact metrics, typography scales and component paddings for Telegram Stars revenue & monetization screens.

use gpui::{px, rgb, rgba, Pixels, Rgba};

#[derive(Debug, Clone)]
pub struct ChannelEarnChannelStatisticsEarnStyleSheet {
    pub overview_title_skip: Pixels,
    pub overview_minor_label_skip: Pixels,
    pub balance_minor_label_skip: Pixels,
    pub history_recipient_button_width: Pixels,
    pub history_recipient_button_height: Pixels,
    pub history_recipient_button_radius: Pixels,
    pub learn_desc_min_width: Pixels,
    pub bot_earn_input_width: Pixels,
    pub bot_earn_input_height_max: Pixels,
    pub bg_color: Rgba,
    pub text_fg: Rgba,
    pub sub_text_fg: Rgba,
    pub active_fg: Rgba,
    pub premium_fg: Rgba,
}

impl Default for ChannelEarnChannelStatisticsEarnStyleSheet {
    fn default() -> Self {
        Self {
            overview_title_skip: px(11.0),
            overview_minor_label_skip: px(3.0),
            balance_minor_label_skip: px(6.0),
            history_recipient_button_width: px(190.0),
            history_recipient_button_height: px(34.0),
            history_recipient_button_radius: px(8.0),
            learn_desc_min_width: px(264.0),
            bot_earn_input_width: px(100.0),
            bot_earn_input_height_max: px(55.0),
            bg_color: rgb(0xFFFFFF),
            text_fg: rgb(0x222222),
            sub_text_fg: rgb(0x707579),
            active_fg: rgb(0x2AABEE),
            premium_fg: rgb(0x8E52EC),
        }
    }
}

impl ChannelEarnChannelStatisticsEarnStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_earn_style_sheet_metrics() {
        let style = ChannelEarnChannelStatisticsEarnStyleSheet::new();
        assert_eq!(style.overview_title_skip, px(11.0));
        assert_eq!(style.overview_minor_label_skip, px(3.0));
        assert_eq!(style.balance_minor_label_skip, px(6.0));
        assert_eq!(style.history_recipient_button_width, px(190.0));
        assert_eq!(style.history_recipient_button_height, px(34.0));
        assert_eq!(style.bot_earn_input_width, px(100.0));
    }
}
