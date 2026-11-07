//! Target Symbol: ExportViewStyleSheet
//! Telegram Desktop 1:1 Theme Specification: `Telegram/SourceFiles/export/view/export.style`
//! Subsystem: `export`, Feature ID: `TCG-N43`
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::{px, rgb, Pixels, Rgba, Size};

/// Complete theme tokens and pixel-precise layout metrics extracted from `export.style`
#[derive(Debug, Clone)]
pub struct ExportViewStyleSheet {
    pub panel_width: Pixels,
    pub panel_height: Pixels,
    pub setting_padding_top: Pixels,
    pub setting_padding_bottom: Pixels,
    pub setting_padding_horizontal: Pixels,
    pub sub_setting_padding_left: Pixels,
    pub header_padding_top: Pixels,
    pub header_padding_bottom: Pixels,
    pub header_padding_horizontal: Pixels,
    pub file_size_slider_height: Pixels,
    pub file_size_slider_seek_size: Pixels,
    pub file_size_label_bottom: Pixels,
    pub location_label_max_height: Pixels,
    pub progress_padding_horizontal: Pixels,
    pub progress_padding_vertical: Pixels,
    pub done_padding_horizontal: Pixels,
    pub done_padding_vertical: Pixels,
    pub bg_color: Rgba,
    pub text_color: Rgba,
    pub secondary_text_color: Rgba,
    pub accent_color: Rgba,
    pub border_color: Rgba,
    pub track_color: Rgba,
}

impl Default for ExportViewStyleSheet {
    fn default() -> Self {
        Self {
            panel_width: px(380.0),
            panel_height: px(460.0),
            setting_padding_top: px(8.0),
            setting_padding_bottom: px(8.0),
            setting_padding_horizontal: px(22.0),
            sub_setting_padding_left: px(46.0),
            header_padding_top: px(16.0),
            header_padding_bottom: px(6.0),
            header_padding_horizontal: px(22.0),
            file_size_slider_height: px(24.0),
            file_size_slider_seek_size: px(16.0),
            file_size_label_bottom: px(18.0),
            location_label_max_height: px(21.0),
            progress_padding_horizontal: px(22.0),
            progress_padding_vertical: px(16.0),
            done_padding_horizontal: px(22.0),
            done_padding_vertical: px(16.0),
            bg_color: rgb(0xFFFFFF),
            text_color: rgb(0x222222),
            secondary_text_color: rgb(0x707579),
            accent_color: rgb(0x2AABEE),
            border_color: rgb(0xDADCE0),
            track_color: rgb(0xE4E6E8),
        }
    }
}

impl ExportViewStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn panel_size(&self) -> Size<Pixels> {
        Size {
            width: self.panel_width,
            height: self.panel_height,
        }
    }

    pub fn setting_horizontal_padding(&self) -> Pixels {
        self.setting_padding_horizontal
    }

    pub fn sub_setting_indent(&self) -> Pixels {
        self.sub_setting_padding_left
    }
}
