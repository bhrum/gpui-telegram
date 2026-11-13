// Source: Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_start.cpp
// Subsystem: settings | Feature ID: TCG-N02
// Upstream: Start::title, tr::lng_settings_cloud_password_start_title, tr::lng_settings_cloud_password_password_subtitle

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone)]
pub struct SettingsCloudPasswordStartCloudPassword {
    pub headline: String,
    pub description: String,
    pub has_existing_password: bool,
    pub is_starting: bool,
}

impl SettingsCloudPasswordStartCloudPassword {
    pub fn new() -> Self {
        Self {
            headline: "Two-Step Verification".into(),
            description: "You can set a password that will be required when you log in on a new device in addition to the code you get in the SMS or Telegram message.".into(),
            has_existing_password: false,
            is_starting: false,
        }
    }

    pub fn start_setup(&mut self) {
        self.is_starting = true;
    }

    pub fn reset(&mut self) {
        self.is_starting = false;
    }

    pub fn is_in_progress(&self) -> bool {
        self.is_starting
    }
}

impl Default for SettingsCloudPasswordStartCloudPassword {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsCloudPasswordStartCloudPassword {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .size_full()
            .p(px(24.0))
            .gap(px(16.0))
            .child(
                div()
                    .w(px(80.0))
                    .h(px(80.0))
                    .rounded_full()
                    .bg(rgb(0xe3f2fd))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_2xl()
                    .child("🔒")
            )
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_xl()
                    .child(self.headline.clone())
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x707579))
                    .max_w(px(360.0))
                    .child(self.description.clone())
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .max_w(px(360.0))
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x8a8a8e))
                            .child("✓ Protects account against SIM swapping")
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x8a8a8e))
                            .child("✓ Requires secondary confirmation on new logins")
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x8a8a8e))
                            .child("✓ Can be restored via verified recovery email")
                    )
            )
            .child(
                div()
                    .mt(px(12.0))
                    .px(px(24.0))
                    .py(px(10.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x2481cc))
                    .text_color(rgb(0xffffff))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("Set Additional Password")
            )
    }
}
