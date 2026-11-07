//! Telegram Desktop Statistics Theme Stylesheet
//! Subsystem: statistics, Feature: TCG-N12

use gpui::{px, rgb, Pixels, Rgba};

#[derive(Debug, Clone)]
pub struct StatisticsStyleSheet {
    pub height: Pixels,
    pub padding: Pixels,
    pub margin: Pixels,
    pub border_radius: Pixels,
    pub bg_color: Rgba,
    pub text_color: Rgba,
    pub accent_color: Rgba,
    pub border_color: Rgba,
    pub statistics_layer_overview_margins_top: Pixels,
    pub statistics_layer_overview_margins_bottom: Pixels,
    pub statistics_layer_margins_horizontal: Pixels,
    pub statistics_chart_height: Pixels,
    pub statistics_chart_entry_padding_top: Pixels,
    pub statistics_chart_entry_padding_bottom: Pixels,
    pub statistics_details_arrow_shift: Pixels,
    pub statistics_details_arrow_stroke: f32,
    pub statistics_details_popup_padding: Pixels,
    pub statistics_details_popup_mid_line_space: Pixels,
    pub statistics_details_dot_radius: Pixels,
    pub statistics_chart_line_width: Pixels,
    pub statistics_chart_footer_skip: Pixels,
    pub statistics_chart_footer_height: Pixels,
    pub statistics_chart_footer_between_side: Pixels,
    pub statistics_chart_footer_side_width: Pixels,
    pub statistics_chart_footer_arrow_height: Pixels,
    pub statistics_chart_footer_side_radius: Pixels,
    pub statistics_chart_ruler_caption_skip: Pixels,
    pub statistics_chart_bottom_caption_height: Pixels,
    pub statistics_chart_bottom_caption_skip: Pixels,
    pub statistics_chart_header_height: Pixels,
    pub statistics_overview_mid_skip: Pixels,
    pub statistics_overview_right_skip: Pixels,
    pub statistics_recent_post_height: Pixels,
}

impl Default for StatisticsStyleSheet {
    fn default() -> Self {
        Self {
            height: px(200.0),
            padding: px(8.0),
            margin: px(4.0),
            border_radius: px(6.0),
            bg_color: rgb(0xFFFFFF),
            text_color: rgb(0x222222),
            accent_color: rgb(0x2AABEE),
            border_color: rgb(0xE0E0E0),
            statistics_layer_overview_margins_top: px(17.0),
            statistics_layer_overview_margins_bottom: px(9.0),
            statistics_layer_margins_horizontal: px(20.0),
            statistics_chart_height: px(200.0),
            statistics_chart_entry_padding_top: px(13.0),
            statistics_chart_entry_padding_bottom: px(2.0),
            statistics_details_arrow_shift: px(3.0),
            statistics_details_arrow_stroke: 1.5,
            statistics_details_popup_padding: px(6.0),
            statistics_details_popup_mid_line_space: px(4.0),
            statistics_details_dot_radius: px(5.0),
            statistics_chart_line_width: px(2.0),
            statistics_chart_footer_skip: px(11.0),
            statistics_chart_footer_height: px(42.0),
            statistics_chart_footer_between_side: px(5.0),
            statistics_chart_footer_side_width: px(10.0),
            statistics_chart_footer_arrow_height: px(10.0),
            statistics_chart_footer_side_radius: px(6.0),
            statistics_chart_ruler_caption_skip: px(4.0),
            statistics_chart_bottom_caption_height: px(15.0),
            statistics_chart_bottom_caption_skip: px(6.0),
            statistics_chart_header_height: px(36.0),
            statistics_overview_mid_skip: px(50.0),
            statistics_overview_right_skip: px(14.0),
            statistics_recent_post_height: px(56.0),
        }
    }
}

impl StatisticsStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn chart_height(&self) -> Pixels {
        self.statistics_chart_height
    }
    pub fn footer_height(&self) -> Pixels {
        self.statistics_chart_footer_height
    }
}
