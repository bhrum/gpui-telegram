//! Upstream Source: Telegram/SourceFiles/settings/sections/settings_premium.style
//! Subsystem: settings | Feature ID: TCG-N31 | Disposition: rust_domain_or_server_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone)]
pub struct SettingsPremiumSectionsStyleSheet {
    pub is_active: bool,
}

impl SettingsPremiumSectionsStyleSheet {
    pub fn new() -> Self {
        Self { is_active: true }
    }

    pub fn settings_icon_chat() -> f32 {
        16.0
    }

    pub fn settings_icon_stickers() -> f32 {
        16.0
    }

    pub fn settings_icon_emoji() -> f32 {
        16.0
    }

    pub fn settings_premium_icon_wallpapers() -> f32 {
        16.0
    }

    pub fn settings_premium_icon_stories() -> f32 {
        16.0
    }

    pub fn settings_premium_icon_channels_off() -> f32 {
        16.0
    }

    pub fn settings_premium_icon_status() -> f32 {
        16.0
    }

    pub fn settings_premium_icon_like() -> f32 {
        16.0
    }

}

impl Default for SettingsPremiumSectionsStyleSheet {
    fn default() -> Self {
        Self::new()
    }
}
