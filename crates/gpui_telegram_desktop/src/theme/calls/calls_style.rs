//! Calls Theme Style Sheet
//! Upstream: Telegram/SourceFiles/calls/calls.style:L17-L200
//! Exact metrics and tokens for 1-on-1 and group call UI panels and windows.
#![allow(dead_code)]

use gpui::{px, rgb, rgba, Pixels, Rgba};

#[derive(Debug, Clone)]
pub struct CallsStyleSheet {
    pub call_width: Pixels,
    pub call_height: Pixels,
    pub call_width_min: Pixels,
    pub call_height_min: Pixels,
    pub call_bottom_controls_height: Pixels,
    pub photo_size: Pixels,
    pub photo_top: Pixels,
    pub name_top: Pixels,
    pub status_top: Pixels,
    pub participants_top: Pixels,
    pub mute_stroke: Pixels,
    pub mute_size: Pixels,
    pub preview_height: Pixels,
    pub preview_photo_size: Pixels,
    pub preview_photo_top: Pixels,
    pub preview_name_top: Pixels,
    pub preview_status_top: Pixels,
    pub button_diameter: Pixels,
    pub button_box_width: Pixels,
    pub button_box_height: Pixels,
    pub signal_bar_width: Pixels,
    pub signal_bar_radius: Pixels,
    pub signal_bar_skip: Pixels,
    pub signal_bar_min: Pixels,
    pub signal_bar_max: Pixels,
    pub signal_bar_inactive_opacity: f32,
    pub fingerprint_skip: Pixels,
    pub fingerprint_top: Pixels,
    pub bg_color: Rgba,
    pub name_fg: Rgba,
    pub status_fg: Rgba,
    pub answer_bg: Rgba,
    pub hangup_bg: Rgba,
    pub icon_bg: Rgba,
    pub icon_bg_active: Rgba,
    pub bar_active_fg: Rgba,
}

impl Default for CallsStyleSheet {
    fn default() -> Self {
        Self {
            call_width: px(720.0),
            call_height: px(540.0),
            call_width_min: px(380.0),
            call_height_min: px(520.0),
            call_bottom_controls_height: px(87.0),
            photo_size: px(160.0),
            photo_top: px(21.0),
            name_top: px(221.0),
            status_top: px(254.0),
            participants_top: px(294.0),
            mute_stroke: px(3.0),
            mute_size: px(36.0),
            preview_height: px(185.0),
            preview_photo_size: px(100.0),
            preview_photo_top: px(21.0),
            preview_name_top: px(132.0),
            preview_status_top: px(163.0),
            button_diameter: px(44.0),
            button_box_width: px(68.0),
            button_box_height: px(79.0),
            signal_bar_width: px(2.0),
            signal_bar_radius: px(1.0),
            signal_bar_skip: px(2.0),
            signal_bar_min: px(4.0),
            signal_bar_max: px(10.0),
            signal_bar_inactive_opacity: 0.5,
            fingerprint_skip: px(4.0),
            fingerprint_top: px(8.0),
            bg_color: rgba(0x18222DFF),
            name_fg: rgb(0xFFFFFF),
            status_fg: rgba(0xFFFFFFB2),
            answer_bg: rgb(0x45C85A),
            hangup_bg: rgb(0xE04F54),
            icon_bg: rgba(0xFFFFFF28),
            icon_bg_active: rgb(0xFFFFFF),
            bar_active_fg: rgb(0x2AABEE),
        }
    }
}

impl CallsStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_wide_layout(&self, window_width: Pixels) -> bool {
        window_width >= self.call_width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calls_style_sheet_metrics() {
        let style = CallsStyleSheet::new();
        assert_eq!(style.call_width, px(720.0));
        assert_eq!(style.call_height, px(540.0));
        assert_eq!(style.call_width_min, px(380.0));
        assert_eq!(style.call_height_min, px(520.0));
        assert_eq!(style.button_diameter, px(44.0));
        assert_eq!(style.signal_bar_width, px(2.0));
        assert!(style.is_wide_layout(px(800.0)));
        assert!(!style.is_wide_layout(px(600.0)));
    }
}
