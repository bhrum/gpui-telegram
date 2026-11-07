//! Authentic GPUI Settings Cloud Password Hint Step
//! Source: Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_hint.cpp, settings_cloud_password_hint.h
//! Subsystem: settings | Feature ID: TCG-N33

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_MAX_HINT_LENGTH: usize = 64;

#[derive(Debug, Clone)]
pub struct SettingsCloudPasswordHintCloudPassword {
    pub hint: String,
    pub password_hash_probe: String,
    pub is_valid: bool,
    pub error_message: Option<String>,
}

impl SettingsCloudPasswordHintCloudPassword {
    pub fn new() -> Self {
        Self {
            hint: String::new(),
            password_hash_probe: String::new(),
            is_valid: true,
            error_message: None,
        }
    }

    pub fn set_password_probe(&mut self, probe: impl Into<String>) {
        self.password_hash_probe = probe.into();
    }

    pub fn set_hint(&mut self, text: impl Into<String>) {
        let val = text.into();
        if val.chars().count() <= K_MAX_HINT_LENGTH {
            self.hint = val;
            self.validate();
        }
    }

    pub fn validate(&mut self) -> bool {
        if !self.hint.is_empty() && !self.password_hash_probe.is_empty() && self.hint == self.password_hash_probe {
            self.is_valid = false;
            self.error_message = Some("Hint cannot be the same as your password.".into());
            return false;
        }
        self.is_valid = true;
        self.error_message = None;
        true
    }

    pub fn chars_remaining(&self) -> usize {
        K_MAX_HINT_LENGTH.saturating_sub(self.hint.chars().count())
    }
}

impl Default for SettingsCloudPasswordHintCloudPassword {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsCloudPasswordHintCloudPassword {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut root = div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(24.0))
            .gap(px(16.0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_xl()
                    .child("Password Hint"),
            )
            .child(
                div()
                    .text_sm()
                    .child("Create a hint to help remember your password. Anyone with access to your device can see this hint."),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .p(px(12.0))
                    .rounded_md()
                    .bg(rgb(0x21252b))
                    .child(
                        div()
                            .text_xs()
                            .child(format!("Hint: {}", if self.hint.is_empty() { "None" } else { &self.hint })),
                    )
                    .child(
                        div()
                            .text_xs()
                            .child(format!("Characters remaining: {}", self.chars_remaining())),
                    ),
            );
        if let Some(err) = self.error_message.as_ref() {
            root = root.child(
                div()
                    .text_xs()
                    .text_color(rgb(0xe06c75))
                    .child(err.clone()),
            );
        }
        root
    }
}
