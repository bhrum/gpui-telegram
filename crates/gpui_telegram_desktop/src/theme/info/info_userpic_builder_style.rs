//! Userpic Builder Theme Style Sheet
//! Upstream: Telegram/SourceFiles/info/userpic/info_userpic_builder.style
//! Exact metrics and sizes for custom emoji & background userpic builder modal layers.

use gpui::{px, rgb, Pixels, Point, Rgba};

#[derive(Debug, Clone)]
pub struct InfoUserpicBuilderUserpicStyleSheet {
    pub emoji_subtitle_min_width: Pixels,
    pub emoji_bubble_palette_width: Pixels,
    pub emoji_selector_left: Pixels,
    pub emoji_back_position: Point<Pixels>,
    pub emoji_save_position: Point<Pixels>,
    pub emoji_accent_color_size: Pixels,
    pub emoji_layer_min_height: Pixels,
    pub emoji_selector_min_height: Pixels,
    pub emoji_selector_toggle_pos: Point<Pixels>,
    pub bg_color: Rgba,
    pub text_fg: Rgba,
    pub sub_text_fg: Rgba,
    pub icon_fg: Rgba,
}

impl Default for InfoUserpicBuilderUserpicStyleSheet {
    fn default() -> Self {
        Self {
            emoji_subtitle_min_width: px(356.0),
            emoji_bubble_palette_width: px(356.0),
            emoji_selector_left: px(5.0),
            emoji_back_position: Point { x: px(8.0), y: px(8.0) },
            emoji_save_position: Point { x: px(7.0), y: px(12.0) },
            emoji_accent_color_size: px(30.0),
            emoji_layer_min_height: px(496.0),
            emoji_selector_min_height: px(216.0),
            emoji_selector_toggle_pos: Point { x: px(6.0), y: px(6.0) },
            bg_color: rgb(0xFFFFFF),
            text_fg: rgb(0x222222),
            sub_text_fg: rgb(0x707579),
            icon_fg: rgb(0x707579),
        }
    }
}

impl InfoUserpicBuilderUserpicStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_userpic_builder_style_metrics() {
        let style = InfoUserpicBuilderUserpicStyleSheet::new();
        assert_eq!(style.emoji_subtitle_min_width, px(356.0));
        assert_eq!(style.emoji_bubble_palette_width, px(356.0));
        assert_eq!(style.emoji_accent_color_size, px(30.0));
        assert_eq!(style.emoji_layer_min_height, px(496.0));
        assert_eq!(style.emoji_selector_min_height, px(216.0));
    }
}
