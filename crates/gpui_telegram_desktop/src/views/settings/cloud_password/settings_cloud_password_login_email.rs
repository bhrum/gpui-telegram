//! Authentic GPUI Settings Cloud Password Login Email Step
//! Source: Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_login_email.cpp, settings_cloud_password_login_email.h
//! Subsystem: settings | Feature ID: TCG-N33

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone)]
pub struct SettingsCloudPasswordLoginEmailCloudPassword {
    pub login_email: String,
    pub is_valid: bool,
    pub error_text: Option<String>,
}

impl SettingsCloudPasswordLoginEmailCloudPassword {
    pub fn new() -> Self {
        Self {
            login_email: String::new(),
            is_valid: false,
            error_text: None,
        }
    }

    pub fn set_email(&mut self, text: impl Into<String>) {
        self.login_email = text.into().trim().to_string();
        self.is_valid = self.validate(&self.login_email);
        self.error_text = None;
    }

    pub fn validate(&self, email: &str) -> bool {
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

    pub fn submit(&mut self) -> bool {
        if !self.is_valid {
            self.error_text = Some("Invalid login email address.".into());
            false
        } else {
            self.error_text = None;
            true
        }
    }
}

impl Default for SettingsCloudPasswordLoginEmailCloudPassword {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsCloudPasswordLoginEmailCloudPassword {
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
                    .child("Login Email"),
            )
            .child(
                div()
                    .text_sm()
                    .child("Add a login email to sign in to your Telegram account using Apple, Google, or email confirmation codes."),
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
                            .child(format!("Login Email: {}", if self.login_email.is_empty() { "Not set" } else { &self.login_email })),
                    )
                    .child(
                        div()
                            .text_xs()
                            .child(format!("Validated: {}", if self.is_valid { "Yes" } else { "No" })),
                    ),
            );
        if let Some(err) = self.error_text.as_ref() {
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
