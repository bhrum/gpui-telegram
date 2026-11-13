//! Authentic GPUI theme stylesheet for BackgroundPreviewBox.
//! Upstream reference: `Telegram/SourceFiles/boxes/background_preview_box.style:L1-L56`.

use gpui::{px, rgb, Pixels, Rgba};

#[derive(Debug, Clone)]
pub struct BackgroundPreviewBoxStyleSheet {
    pub preview_width: Pixels,
    pub preview_height: Pixels,
    pub preview_radius: Pixels,
    pub storage_limit_margin: (Pixels, Pixels, Pixels, Pixels),
    pub checkbox_size: Pixels,
    pub pattern_opacity: f32,
    pub dim_opacity: f32,
    pub blur_radius: Pixels,
    pub overlay_bg: Rgba,
    pub text_fg: Rgba,
    pub text_active_fg: Rgba,
}

impl Default for BackgroundPreviewBoxStyleSheet {
    fn default() -> Self {
        Self {
            preview_width: px(320.0),
            preview_height: px(480.0),
            preview_radius: px(8.0),
            storage_limit_margin: (px(24.0), px(16.0), px(24.0), px(16.0)),
            checkbox_size: px(22.0),
            pattern_opacity: 0.5,
            dim_opacity: 0.3,
            blur_radius: px(12.0),
            overlay_bg: rgb(0x000000),
            text_fg: rgb(0x707579),
            text_active_fg: rgb(0x2B2B2B),
        }
    }
}

impl BackgroundPreviewBoxStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn preview_dimensions(&self) -> (Pixels, Pixels) {
        (self.preview_width, self.preview_height)
    }
}
