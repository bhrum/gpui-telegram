//! Window lock widgets style sheet
//! Source: Telegram/SourceFiles/window/window_lock_widgets.style
//! Subsystem: window | Feature ID: TCG-N46 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone)]
pub struct WindowLockWidgetsStyleSheet {
    pub is_active: bool,
}

impl WindowLockWidgetsStyleSheet {
    pub fn new() -> Self {
        Self { is_active: true }
    }

    pub fn passcode_header_font() -> f32 {
        16.0
    }

    pub fn passcode_header_height() -> f32 {
        80.0
    }

    pub fn passcode_input() -> f32 {
        16.0
    }

    pub fn text_margins() -> f32 {
        16.0
    }

    pub fn passcode_submit() -> f32 {
        16.0
    }

    pub fn width() -> f32 {
        225.0
    }

    pub fn passcode_submit_skip() -> f32 {
        40.0
    }

    pub fn passcode_system_unlock() -> f32 {
        16.0
    }

}

impl Default for WindowLockWidgetsStyleSheet {
    fn default() -> Self {
        Self::new()
    }
}
