//! Upstream Source: Telegram/SourceFiles/settings/settings.style
//! Subsystem: settings | Feature ID: TCG-N31 | Disposition: rust_domain_or_server_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone)]
pub struct SettingsStyleSheet {
    pub is_active: bool,
}

impl SettingsStyleSheet {
    pub fn new() -> Self {
        Self { is_active: true }
    }

    pub fn settings_button() -> f32 {
        16.0
    }

    pub fn style() -> f32 {
        16.0
    }

    pub fn padding() -> f32 {
        16.0
    }

    pub fn icon_left() -> f32 {
        20.0
    }

    pub fn settings_button_right_label_spoiler() -> f32 {
        16.0
    }

    pub fn right_label() -> f32 {
        16.0
    }

    pub fn palette() -> gpui::Rgba {
        gpui::rgb(0x2aabee)
    }

    pub fn spoiler_fg() -> f32 {
        16.0
    }

}

impl Default for SettingsStyleSheet {
    fn default() -> Self {
        Self::new()
    }
}
