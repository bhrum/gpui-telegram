//! Upstream Source: Telegram/SourceFiles/settings/sections/settings_active_sessions.style
//! Subsystem: settings | Feature ID: TCG-N02 | Disposition: rust_domain_or_server_1to1
//! Inherited Rust Path: inherited/node-agent-coordinator/src/tdrp/settings_cloud_passkeys.rs

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone)]
pub struct SettingsActiveSessionsSectionsStyleSheet {
    pub is_active: bool,
}

impl SettingsActiveSessionsSectionsStyleSheet {
    pub fn new() -> Self {
        Self { is_active: true }
    }

    pub fn session_location_top() -> f32 {
        54.0
    }

    pub fn session_current_skip() -> f32 {
        8.0
    }

    pub fn session_icon_windows() -> f32 {
        16.0
    }

    pub fn session_icon_mac() -> f32 {
        16.0
    }

    pub fn session_icon_ubuntu() -> f32 {
        16.0
    }

    pub fn session_icon_linux() -> f32 {
        16.0
    }

    pub fn session_iconi_pad() -> f32 {
        16.0
    }

    pub fn session_icon_web() -> f32 {
        16.0
    }

}

impl Default for SettingsActiveSessionsSectionsStyleSheet {
    fn default() -> Self {
        Self::new()
    }
}
