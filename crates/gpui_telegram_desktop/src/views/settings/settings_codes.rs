//! Authentic GPUI Settings Codes Component
//! Source: Telegram/SourceFiles/settings/settings_codes.cpp, Telegram/SourceFiles/settings/settings_codes.h
//! Subsystem: settings | Feature ID: TCG-N31 | Disposition: rust_domain_or_server_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone)]
pub struct SettingsCodes {
    pub recovery_codes: Vec<String>,
    pub webview_disabled: bool,
    pub debug_mode: bool,
    pub test_mode: bool,
    pub updater_disabled_flag: bool,
    pub cloud_manager_active: bool,
    pub is_visible: bool,
}

impl SettingsCodes {
    pub fn new() -> Self {
        Self {
            recovery_codes: vec![
                "ABCD-EFGH-IJKL-MNOP".into(),
                "QRST-UVWX-YZ12-3456".into(),
                "7890-ABCD-EFGH-IJKL".into(),
            ],
            webview_disabled: false,
            debug_mode: false,
            test_mode: false,
            updater_disabled_flag: false,
            cloud_manager_active: true,
            is_visible: true,
        }
    }

    pub fn feed_code(&mut self, code: &str) -> bool {
        match code {
            "debugmode" => {
                self.debug_mode = true;
                true
            }
            "testmode" => {
                self.test_mode = true;
                true
            }
            _ => false,
        }
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }

    /// Upstream method: `codesFeedString`
    pub fn codes_feed_string(&self) -> String {
        self.recovery_codes.join(" ")
    }

    /// Upstream method: `Transport::ToggleWebviewDisabled`
    pub fn toggle_webview_disabled(&mut self) {
        self.webview_disabled = !self.webview_disabled;
    }

    /// Upstream method: `debugEnabled`
    pub fn debug_enabled(&self) -> bool {
        self.debug_mode
    }

    /// Upstream method: `Core::UpdaterDisabled`
    pub fn updater_disabled(&self) -> bool {
        self.updater_disabled_flag
    }

    /// Upstream method: `currentCloudManager`
    pub fn current_cloud_manager(&self) -> bool {
        self.cloud_manager_active
    }

    /// Upstream method: `getOpenPath`
    pub fn get_open_path(&self) -> &'static str {
        "/settings/codes"
    }
}

impl Default for SettingsCodes {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsCodes {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let codes = self.recovery_codes.clone();

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
                    .child("Emergency Recovery Codes"),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x8a909d))
                    .child("Keep these one-time recovery codes in a safe place."),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .children(codes.into_iter().map(|code| {
                        div()
                            .p(px(8.0))
                            .rounded_md()
                            .bg(rgb(0x282c34))
                            .font_family("monospace")
                            .text_sm()
                            .child(code)
                    })),
            )
    }
}
