//! Authentic GPUI Settings Cloud Password Input Step
//! Source: Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_input.cpp, settings_cloud_password_input.h
//! Subsystem: settings | Feature ID: TCG-N33

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasswordStrength {
    TooShort,
    Weak,
    Fair,
    Strong,
}

#[derive(Debug, Clone)]
pub struct SettingsCloudPasswordInputCloudPassword {
    pub password: String,
    pub confirm_password: String,
    pub is_confirming: bool,
    pub strength: PasswordStrength,
    pub error_message: Option<String>,
    pub show_password: bool,
}

impl SettingsCloudPasswordInputCloudPassword {
    pub fn new() -> Self {
        Self {
            password: String::new(),
            confirm_password: String::new(),
            is_confirming: false,
            strength: PasswordStrength::TooShort,
            error_message: None,
            show_password: false,
        }
    }

    pub fn set_password(&mut self, text: impl Into<String>) {
        self.password = text.into();
        self.strength = self.compute_strength(&self.password);
        self.error_message = None;
    }

    pub fn set_confirm_password(&mut self, text: impl Into<String>) {
        self.confirm_password = text.into();
        self.error_message = None;
    }

    pub fn compute_strength(&self, pwd: &str) -> PasswordStrength {
        let len = pwd.len();
        if len < 6 {
            return PasswordStrength::TooShort;
        }
        let has_upper = pwd.chars().any(|c| c.is_uppercase());
        let has_lower = pwd.chars().any(|c| c.is_lowercase());
        let has_digit = pwd.chars().any(|c| c.is_ascii_digit());
        let has_special = pwd.chars().any(|c| !c.is_alphanumeric());

        let score = (has_upper as u8) + (has_lower as u8) + (has_digit as u8) + (has_special as u8);
        if len >= 10 && score >= 3 {
            PasswordStrength::Strong
        } else if len >= 8 && score >= 2 {
            PasswordStrength::Fair
        } else {
            PasswordStrength::Weak
        }
    }

    pub fn check_match(&mut self) -> bool {
        if self.password != self.confirm_password {
            self.error_message = Some("Passwords do not match.".into());
            false
        } else {
            self.error_message = None;
            true
        }
    }

    pub fn toggle_show_password(&mut self) {
        self.show_password = !self.show_password;
    }
}

impl Default for SettingsCloudPasswordInputCloudPassword {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsCloudPasswordInputCloudPassword {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let strength_label = match self.strength {
            PasswordStrength::TooShort => "Too Short (min 6)",
            PasswordStrength::Weak => "Weak",
            PasswordStrength::Fair => "Fair",
            PasswordStrength::Strong => "Strong",
        };

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
                    .child(if self.is_confirming { "Confirm Cloud Password" } else { "Enter Cloud Password" }),
            )
            .child(
                div()
                    .text_sm()
                    .child("This password will be requested when you log in on a new device in addition to the code sent via SMS."),
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
                            .child(format!("Password length: {}", self.password.len())),
                    )
                    .child(
                        div()
                            .text_xs()
                            .child(format!("Strength: {}", strength_label)),
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
