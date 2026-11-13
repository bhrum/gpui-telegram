//! Info Theme Style Sheet
//! Upstream: Telegram/SourceFiles/info/info.style
//! Exact metrics, dimensions, paddings and color tokens for the info drawer subsystem.

use gpui::{px, rgb, rgba, Pixels, Rgba};

#[derive(Debug, Clone)]
pub struct InfoStyleSheet {
    pub top_bar_height: Pixels,
    pub layer_top_bar_height: Pixels,
    pub minimal_width: Pixels,
    pub desired_width: Pixels,
    pub minimal_layer_margin: Pixels,
    pub profile_cover_height: Pixels,
    pub profile_photo_inner_size: Pixels,
    pub profile_photo_left: Pixels,
    pub profile_photo_top: Pixels,
    pub profile_name_left: Pixels,
    pub profile_name_top: Pixels,
    pub profile_status_left: Pixels,
    pub profile_status_top: Pixels,
    pub profile_top_bar_height_max: Pixels,
    pub profile_top_bar_step1: Pixels,
    pub profile_top_bar_step2: Pixels,
    pub profile_action_button_size: Pixels,
    pub profile_action_buttons_height: Pixels,
    pub media_header_height: Pixels,
    pub media_min_grid_size: Pixels,
    pub media_skip: Pixels,
    pub media_left: Pixels,
    pub slide_duration: u32,
    pub highlight_duration: u32,
    pub bg_color: Rgba,
    pub text_fg: Rgba,
    pub sub_text_fg: Rgba,
    pub active_fg: Rgba,
}

impl Default for InfoStyleSheet {
    fn default() -> Self {
        let top_bar_height = px(54.0);
        let layer_top_bar_height = px(56.0);
        let minimal_width = px(324.0);
        let desired_width = px(392.0);
        let minimal_layer_margin = px(48.0);
        let profile_action_button_size = px(52.0);
        let profile_action_buttons_height = px(68.0);
        let profile_top_bar_height_max = px(180.0) + layer_top_bar_height;
        let profile_top_bar_step2 = profile_top_bar_height_max - layer_top_bar_height;
        let profile_top_bar_step1 = profile_top_bar_step2 - profile_action_buttons_height;

        Self {
            top_bar_height,
            layer_top_bar_height,
            minimal_width,
            desired_width,
            minimal_layer_margin,
            profile_cover_height: px(108.0),
            profile_photo_inner_size: px(72.0),
            profile_photo_left: px(19.0),
            profile_photo_top: px(18.0),
            profile_name_left: px(109.0),
            profile_name_top: px(32.0),
            profile_status_left: px(109.0),
            profile_status_top: px(58.0),
            profile_top_bar_height_max,
            profile_top_bar_step1,
            profile_top_bar_step2,
            profile_action_button_size,
            profile_action_buttons_height,
            media_header_height: px(28.0),
            media_min_grid_size: px(82.0),
            media_skip: px(2.0),
            media_left: px(3.0),
            slide_duration: 0,
            highlight_duration: 240,
            bg_color: rgb(0xFFFFFF),
            text_fg: rgb(0x222222),
            sub_text_fg: rgb(0x707579),
            active_fg: rgb(0x2AABEE),
        }
    }
}

impl InfoStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn minimal_supported_layer_width(&self) -> Pixels {
        self.minimal_width + self.minimal_layer_margin * 2.0
    }

    pub fn clamped_layer_width(&self, parent_width: Pixels) -> Pixels {
        let candidate = parent_width - self.minimal_layer_margin * 2.0;
        if candidate < self.desired_width {
            candidate
        } else {
            self.desired_width
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_info_style_sheet_metrics() {
        let style = InfoStyleSheet::new();
        assert_eq!(style.top_bar_height, px(54.0));
        assert_eq!(style.layer_top_bar_height, px(56.0));
        assert_eq!(style.minimal_width, px(324.0));
        assert_eq!(style.desired_width, px(392.0));
        assert_eq!(style.minimal_layer_margin, px(48.0));
        assert_eq!(style.profile_cover_height, px(108.0));
        assert_eq!(style.profile_photo_inner_size, px(72.0));
        assert_eq!(style.profile_top_bar_height_max, px(236.0));
        assert_eq!(style.minimal_supported_layer_width(), px(420.0));
        assert_eq!(style.clamped_layer_width(px(600.0)), px(392.0));
        assert_eq!(style.clamped_layer_width(px(450.0)), px(354.0));
    }
}
