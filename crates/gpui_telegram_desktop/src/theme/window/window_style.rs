//! Window layout dimensions and style metrics
//! Source: Telegram/SourceFiles/window/window.style
//! Subsystem: window | Feature ID: TCG-N46 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone)]
pub struct WindowStyleSheet {
    pub is_active: bool,
}

impl WindowStyleSheet {
    pub fn new() -> Self {
        Self { is_active: true }
    }

    pub fn window_min_width() -> f32 {
        380.0
    }

    pub fn window_min_height() -> f32 {
        480.0
    }

    pub fn window_default_width() -> f32 {
        800.0
    }

    pub fn window_default_height() -> f32 {
        600.0
    }

    pub fn window_big_default_width() -> f32 {
        1024.0
    }

    pub fn window_big_default_height() -> f32 {
        768.0
    }

    pub fn window_shell_min_width() -> f32 {
        240.0
    }

    pub fn window_shell_min_height() -> f32 {
        160.0
    }

}

impl Default for WindowStyleSheet {
    fn default() -> Self {
        Self::new()
    }
}
