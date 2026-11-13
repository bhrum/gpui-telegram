//! Window main menu style sheet
//! Source: Telegram/SourceFiles/window/window_main_menu.style
//! Subsystem: window | Feature ID: TCG-N46 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone)]
pub struct WindowMainMenuStyleSheet {
    pub is_active: bool,
}

impl WindowMainMenuStyleSheet {
    pub fn new() -> Self {
        Self { is_active: true }
    }

    pub fn main_menu_width() -> f32 {
        274.0
    }

    pub fn main_menu_cover_height() -> f32 {
        134.0
    }

    pub fn main_menu_userpic_left() -> f32 {
        24.0
    }

    pub fn main_menu_userpic_top() -> f32 {
        20.0
    }

    pub fn main_menu_cloud_button() -> f32 {
        16.0
    }

    pub fn width() -> f32 {
        48.0
    }

    pub fn height() -> f32 {
        48.0
    }

    pub fn icon() -> f32 {
        16.0
    }

}

impl Default for WindowMainMenuStyleSheet {
    fn default() -> Self {
        Self::new()
    }
}
