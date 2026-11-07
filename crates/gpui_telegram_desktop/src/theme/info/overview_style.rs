//! Media Overview Theme Style Sheet
//! Upstream: Telegram/SourceFiles/overview/overview.style
//! Exact metrics, checkboxes, song/file layouts and link thumbnails for media tabs.

use gpui::{px, rgb, rgba, Pixels, Rgba};

#[derive(Debug, Clone)]
pub struct OverviewStyleSheet {
    pub check_pressed_size: f32,
    pub check_size: Pixels,
    pub check_skip: Pixels,
    pub photo_min_size: Pixels,
    pub file_layout_max_width: Pixels,
    pub song_thumb_size: Pixels,
    pub song_name_top: Pixels,
    pub song_status_top: Pixels,
    pub file_thumb_size: Pixels,
    pub file_name_top: Pixels,
    pub file_status_top: Pixels,
    pub file_date_top: Pixels,
    pub links_max_width: Pixels,
    pub links_photo_size: Pixels,
    pub links_photo_padding: Pixels,
    pub video_radial_size: Pixels,
    pub video_status_radius: Pixels,
    pub bg_color: Rgba,
    pub text_fg: Rgba,
    pub active_fg: Rgba,
    pub check_bg: Rgba,
    pub check_bg_active: Rgba,
    pub border_fg: Rgba,
}

impl Default for OverviewStyleSheet {
    fn default() -> Self {
        Self {
            check_pressed_size: 0.8,
            check_size: px(29.0),
            check_skip: px(5.0),
            photo_min_size: px(90.0),
            file_layout_max_width: px(520.0),
            song_thumb_size: px(36.0),
            song_name_top: px(7.0),
            song_status_top: px(25.0),
            file_thumb_size: px(70.0),
            file_name_top: px(7.0),
            file_status_top: px(24.0),
            file_date_top: px(49.0),
            links_max_width: px(520.0),
            links_photo_size: px(46.0),
            links_photo_padding: px(12.0),
            video_radial_size: px(36.0),
            video_status_radius: px(4.0),
            bg_color: rgb(0xFFFFFF),
            text_fg: rgb(0x222222),
            active_fg: rgb(0x2AABEE),
            check_bg: rgba(0x0000004C),
            check_bg_active: rgb(0x2AABEE),
            border_fg: rgba(0xFFFFFFB2),
        }
    }
}

impl OverviewStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overview_style_sheet_metrics() {
        let style = OverviewStyleSheet::new();
        assert_eq!(style.check_pressed_size, 0.8);
        assert_eq!(style.check_size, px(29.0));
        assert_eq!(style.check_skip, px(5.0));
        assert_eq!(style.photo_min_size, px(90.0));
        assert_eq!(style.file_layout_max_width, px(520.0));
        assert_eq!(style.links_photo_size, px(46.0));
        assert_eq!(style.video_radial_size, px(36.0));
    }
}
