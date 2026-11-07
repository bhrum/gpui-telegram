//! Authentic GPUI Settings Cloud Password Email Setup Step
//! Source: Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_email.cpp, settings_cloud_password_email.h
//! Subsystem: settings | Feature ID: TCG-N33

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone)]
pub struct SettingsCloudPasswordEmailCloudPassword {
    pub email: String,
    pub is_valid: bool,
    pub is_submitting: bool,
    pub error_message: Option<String>,
    pub skip_warning_shown: bool,
}

impl SettingsCloudPasswordEmailCloudPassword {
    pub fn new() -> Self {
        Self {
            email: String::new(),
            is_valid: false,
            is_submitting: false,
            error_message: None,
            skip_warning_shown: false,
        }
    }

    pub fn set_email(&mut self, email: impl Into<String>) {
        self.email = email.into().trim().to_string();
        self.is_valid = self.validate_email(&self.email);
        self.error_message = None;
    }

    pub fn validate_email(&self, email: &str) -> bool {
        if email.is_empty() {
            return false;
        }
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 {
            return false;
        }
        let (local, domain) = (parts[0], parts[1]);
        !local.is_empty() && domain.contains('.') && domain.len() >= 3
    }

    pub fn submit_email(&mut self) -> bool {
        if !self.is_valid {
            self.error_message = Some("Please enter a valid email address".into());
            return false;
        }
        self.is_submitting = true;
        self.error_message = None;
        true
    }

    pub fn trigger_skip(&mut self) {
        self.skip_warning_shown = true;
    }
}

impl Default for SettingsCloudPasswordEmailCloudPassword {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsCloudPasswordEmailCloudPassword {
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
                    .child("Recovery Email"),
            )
            .child(
                div()
                    .text_sm()
                    .child("Please add a recovery email. This is the only way to recover a forgotten password."),
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
                            .child(format!("Current Input: {}", if self.email.is_empty() { "None" } else { &self.email })),
                    )
                    .child(
                        div()
                            .text_xs()
                            .child(format!("Valid: {}", if self.is_valid { "Yes" } else { "No" })),
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
