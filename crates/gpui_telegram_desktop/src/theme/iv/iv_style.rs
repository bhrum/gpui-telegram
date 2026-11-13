//! Instant View Theme & Editor Style Sheet (`iv.style`)
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::{px, rgb, rgba, Pixels, Rgba, Size};

/// Complete visual styling tokens for Instant View and IV Article Editor.
#[derive(Debug, Clone)]
pub struct IvStyleSheet {
    // Window dimensions
    pub min_window_width: Pixels,
    pub min_window_height: Pixels,
    pub default_window_width: Pixels,
    pub default_window_height: Pixels,

    // Layout paddings
    pub height: Pixels,
    pub padding: Pixels,
    pub margin: Pixels,
    pub border_radius: Pixels,
    pub body_padding_top: Pixels,
    pub body_padding_bottom: Pixels,
    pub body_padding_left: Pixels,
    pub body_padding_right: Pixels,

    // Toolbar metrics
    pub toolbar_button_size: Pixels,
    pub toolbar_padding: Pixels,
    pub toolbar_groups_skip: Pixels,
    pub pill_padding: Pixels,
    pub pill_button_skip: Pixels,
    pub bottom_circle_size: Pixels,

    // Suggestions popup metrics
    pub suggestions_radius: Pixels,
    pub suggestions_query_radius: Pixels,
    pub suggestions_max_rows: usize,
    pub suggestions_skip: Pixels,

    // Header & Zoom controls
    pub iv_menu_toggle: Pixels,
    pub width: Pixels,
    pub icon: Pixels,
    pub icon_over: Pixels,
    pub ripple_area_position: Pixels,
    pub ripple_area_size: Pixels,
    pub ripple: Pixels,
    pub iv_menu_position: Pixels,
    pub iv_zoom_dropdown_position: Pixels,
    pub iv_back_icon: Pixels,
    pub iv_back: Pixels,
    pub zoom_buttons_size: Pixels,

    // Color palette
    pub bg_color: Rgba,
    pub text_color: Rgba,
    pub accent_color: Rgba,
    pub border_color: Rgba,
    pub toolbar_bg: Rgba,
    pub toolbar_button_active_bg: Rgba,
    pub formula_preview_fg: Rgba,
    pub premium_star_color: Rgba,
}

impl Default for IvStyleSheet {
    fn default() -> Self {
        Self {
            min_window_width: px(384.0),
            min_window_height: px(480.0),
            default_window_width: px(800.0),
            default_window_height: px(600.0),

            height: px(40.0),
            padding: px(8.0),
            margin: px(4.0),
            border_radius: px(6.0),
            body_padding_top: px(8.0),
            body_padding_bottom: px(8.0),
            body_padding_left: px(0.0),
            body_padding_right: px(0.0),

            toolbar_button_size: px(36.0),
            toolbar_padding: px(10.0),
            toolbar_groups_skip: px(6.0),
            pill_padding: px(2.0),
            pill_button_skip: px(6.0),
            bottom_circle_size: px(40.0),

            suggestions_radius: px(8.0),
            suggestions_query_radius: px(4.0),
            suggestions_max_rows: 5,
            suggestions_skip: px(4.0),

            iv_menu_toggle: px(48.0),
            width: px(8.0),
            icon: px(24.0),
            icon_over: px(24.0),
            ripple_area_position: px(6.0),
            ripple_area_size: px(36.0),
            ripple: px(8.0),
            iv_menu_position: px(40.0),
            iv_zoom_dropdown_position: px(0.0),
            iv_back_icon: px(24.0),
            iv_back: px(60.0),
            zoom_buttons_size: px(26.0),

            bg_color: rgb(0xFFFFFF),
            text_color: rgb(0x222222),
            accent_color: rgb(0x2AABEE),
            border_color: rgb(0xE0E0E0),
            toolbar_bg: rgba(0xF5F5F5FF),
            toolbar_button_active_bg: rgba(0x2AABEE22),
            formula_preview_fg: rgb(0x111111),
            premium_star_color: rgb(0x8B5CF6),
        }
    }
}

impl IvStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn min_window_size(&self) -> Size<Pixels> {
        Size {
            width: self.min_window_width,
            height: self.min_window_height,
        }
    }

    pub fn default_window_size(&self) -> Size<Pixels> {
        Size {
            width: self.default_window_width,
            height: self.default_window_height,
        }
    }

    pub fn point(&self) -> Pixels {
        self.padding
    }

    pub fn margins(&self) -> Pixels {
        self.margin
    }
}
