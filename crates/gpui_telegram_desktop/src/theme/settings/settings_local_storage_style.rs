//! Upstream Source: Telegram/SourceFiles/settings/sections/settings_local_storage.style
//! Subsystem: settings | Feature ID: TCG-N34 | Disposition: rust_domain_or_server_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone)]
pub struct SettingsLocalStorageSectionsStyleSheet {
    pub is_active: bool,
}

impl SettingsLocalStorageSectionsStyleSheet {
    pub fn new() -> Self {
        Self { is_active: true }
    }

    pub fn local_storage_limit_label() -> f32 {
        16.0
    }

    pub fn font() -> f32 {
        16.0
    }

    pub fn local_storage_limit_value() -> f32 {
        16.0
    }

    pub fn text_fg() -> f32 {
        16.0
    }

    pub fn local_storage_limit_label_margin() -> f32 {
        16.0
    }

    pub fn local_storage_chart_height() -> f32 {
        200.0
    }

    pub fn local_storage_chart_diameter() -> f32 {
        172.0
    }

    pub fn local_storage_chart_thickness() -> f32 {
        38.0
    }

}

impl Default for SettingsLocalStorageSectionsStyleSheet {
    fn default() -> Self {
        Self::new()
    }
}
