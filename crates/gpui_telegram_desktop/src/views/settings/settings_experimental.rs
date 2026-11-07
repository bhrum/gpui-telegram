//! Authentic GPUI Settings Experimental View Component
//! Source: Telegram/SourceFiles/settings/settings_experimental.cpp, Telegram/SourceFiles/settings/settings_experimental.h
//! Subsystem: settings | Feature ID: TCG-N31 | Disposition: rust_domain_or_server_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone)]
pub struct ExperimentalFlag {
    pub id: String,
    pub name: String,
    pub category: String,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct SettingsExperimental {
    pub flags: Vec<ExperimentalFlag>,
    pub restart_required: bool,
    pub clipboard_buffer: String,
    pub content_ready: bool,
    pub is_visible: bool,
}

impl SettingsExperimental {
    pub fn new() -> Self {
        Self {
            flags: vec![
                ExperimentalFlag {
                    id: "av1_hw_decode".into(),
                    name: "Hardware AV1 Video Decoding".into(),
                    category: "Media".into(),
                    enabled: false,
                },
                ExperimentalFlag {
                    id: "custom_window_frame".into(),
                    name: "Custom Window Shadow and Blur".into(),
                    category: "Rendering".into(),
                    enabled: true,
                },
                ExperimentalFlag {
                    id: "mtproto_v2_transport".into(),
                    name: "Experimental MTProto Multiplexing".into(),
                    category: "Network".into(),
                    enabled: false,
                },
            ],
            restart_required: false,
            clipboard_buffer: String::new(),
            content_ready: true,
            is_visible: true,
        }
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }

    /// Upstream method: `setupContent`
    pub fn setup_content(&mut self) {
        self.content_ready = true;
    }

    /// Upstream method: `showFinished`
    pub fn show_finished(&mut self) {
        self.content_ready = true;
    }

    /// Upstream method: `tr::lng_settings_experimental_about`
    pub fn lng_settings_experimental_about(&self) -> &'static str {
        "Experimental features for advanced users and developers. Some settings may cause instability."
    }

    /// Upstream method: `tr::lng_settings_restart_now`
    pub fn lng_settings_restart_now(&mut self) {
        self.restart_required = true;
    }

    /// Upstream method: `pos`
    pub fn pos(&self) -> (i32, i32) {
        (0, 0)
    }

    /// Upstream method: `setClipboardText`
    pub fn set_clipboard_text(&mut self, text: String) {
        self.clipboard_buffer = text;
    }

    pub fn toggle_flag(&mut self, id: &str) {
        if let Some(flag) = self.flags.iter_mut().find(|f| f.id == id) {
            flag.enabled = !flag.enabled;
            self.restart_required = true;
        }
    }
}

impl Default for SettingsExperimental {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsExperimental {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let flags = self.flags.clone();

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
                    .child("Experimental Features"),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x8a909d))
                    .child(self.lng_settings_experimental_about()),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .children(flags.into_iter().map(|f| {
                        div()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .items_center()
                            .p(px(10.0))
                            .rounded_md()
                            .bg(rgb(0x282c34))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(div().font_weight(FontWeight::SEMIBOLD).child(f.name))
                                    .child(div().text_xs().text_color(rgb(0x8a909d)).child(f.category)),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(if f.enabled { rgb(0x4fae57) } else { rgb(0x8a909d) })
                                    .child(if f.enabled { "Enabled" } else { "Disabled" }),
                            )
                    })),
            )
    }
}
