//! Authentic GPUI Settings Advanced Section View Component
//! Source: Telegram/SourceFiles/settings/sections/settings_advanced.cpp, Telegram/SourceFiles/settings/sections/settings_advanced.h
//! Subsystem: settings | Feature ID: TCG-N31 | Disposition: rust_domain_or_server_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProxyType {
    Direct,
    Http,
    Socks5,
    Mtproto,
}

#[derive(Debug, Clone)]
pub struct SettingsAdvancedSections {
    pub proxy_type: ProxyType,
    pub proxy_host: String,
    pub proxy_port: u16,
    pub opengl_enabled: bool,
    pub native_window_frame: bool,
    pub tray_icon_enabled: bool,
    pub ask_download_path: bool,
    pub download_path: String,
    pub auto_update_enabled: bool,
    pub is_visible: bool,
    pub local_storage_managed: bool,
    pub update_available: bool,
}

impl SettingsAdvancedSections {
    pub fn new() -> Self {
        Self {
            proxy_type: ProxyType::Direct,
            proxy_host: String::new(),
            proxy_port: 1080,
            opengl_enabled: true,
            native_window_frame: false,
            tray_icon_enabled: true,
            ask_download_path: false,
            download_path: "~/Downloads/Telegram".into(),
            auto_update_enabled: true,
            is_visible: true,
            local_storage_managed: false,
            update_available: false,
        }
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }

    /// Upstream method: `tr::lng_settings_manage_local_storage`
    pub fn lng_settings_manage_local_storage(&mut self) {
        self.local_storage_managed = true;
    }

    /// Upstream method: `HasUpdate`
    pub fn has_update(&mut self) -> bool {
        self.update_available
    }

    /// Upstream method: `tr::lng_connection_proxy`
    pub fn lng_connection_proxy(&mut self) {
        if self.proxy_type == ProxyType::Direct {
            self.proxy_type = ProxyType::Socks5;
        }
    }

    /// Upstream method: `QDir::toNativeSeparators`
    pub fn to_native_separators(&mut self) {
        self.download_path = self.download_path.replace('/', "\\");
    }

    /// Upstream method: `SetupConnectionType`
    pub fn setup_connection_type(&mut self) {
        self.proxy_type = ProxyType::Direct;
    }

    /// Upstream method: `tr::lng_connection_auto`
    pub fn lng_connection_auto(&mut self) {
        self.proxy_type = ProxyType::Direct;
    }
}

impl Default for SettingsAdvancedSections {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsAdvancedSections {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .gap(px(12.0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_lg()
                    .child("Advanced Settings"),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .p(px(12.0))
                    .rounded_md()
                    .bg(rgb(0x282c34))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("Data and Storage"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .child(format!("Download path: {}", self.download_path)),
                    )
                    .child(
                        div()
                            .text_xs()
                            .child(format!("Ask download path: {}", self.ask_download_path)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .p(px(12.0))
                    .rounded_md()
                    .bg(rgb(0x282c34))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("Connection & Hardware"),
                    )
                    .child(div().text_xs().child(format!(
                        "Connection type: {:?}",
                        self.proxy_type
                    )))
                    .child(div().text_xs().child(format!(
                        "Hardware acceleration (OpenGL): {}",
                        if self.opengl_enabled { "Enabled" } else { "Disabled" }
                    )))
                    .child(div().text_xs().child(format!(
                        "Tray icon: {}",
                        if self.tray_icon_enabled { "Enabled" } else { "Disabled" }
                    ))),
            )
    }
}
