//! Authentic GPUI Settings Cloud Password Login Email Confirm Step
//! Source: Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_login_email_confirm.cpp, settings_cloud_password_login_email_confirm.h
//! Subsystem: settings | Feature ID: TCG-N33

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_LOGIN_EMAIL_RESEND_TIMEOUT_SEC: u32 = 60;
pub const K_LOGIN_CODE_LENGTH: usize = 6;

#[derive(Debug, Clone)]
pub struct SettingsCloudPasswordLoginEmailConfirmCloudPassword {
    pub email: String,
    pub code_input: String,
    pub countdown: u32,
    pub resend_enabled: bool,
    pub error_message: Option<String>,
}

impl SettingsCloudPasswordLoginEmailConfirmCloudPassword {
    pub fn new() -> Self {
        Self {
            email: String::new(),
            code_input: String::new(),
            countdown: K_LOGIN_EMAIL_RESEND_TIMEOUT_SEC,
            resend_enabled: false,
            error_message: None,
        }
    }

    pub fn set_email(&mut self, email: impl Into<String>) {
        self.email = email.into();
    }

    pub fn push_char(&mut self, c: char) -> bool {
        if c.is_ascii_digit() && self.code_input.len() < K_LOGIN_CODE_LENGTH {
            self.code_input.push(c);
            self.error_message = None;
            return self.code_input.len() == K_LOGIN_CODE_LENGTH;
        }
        false
    }

    pub fn pop_char(&mut self) {
        self.code_input.pop();
        self.error_message = None;
    }

    pub fn tick(&mut self) {
        if self.countdown > 0 {
            self.countdown -= 1;
            if self.countdown == 0 {
                self.resend_enabled = true;
            }
        }
    }

    pub fn resend(&mut self) -> bool {
        if self.resend_enabled {
            self.countdown = K_LOGIN_EMAIL_RESEND_TIMEOUT_SEC;
            self.resend_enabled = false;
            self.code_input.clear();
            return true;
        }
        false
    }
}

impl Default for SettingsCloudPasswordLoginEmailConfirmCloudPassword {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsCloudPasswordLoginEmailConfirmCloudPassword {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let code_display = if self.code_input.is_empty() {
            "------".to_string()
        } else {
            format!("{:-<6}", self.code_input)
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
                    .child("Confirm Login Email"),
            )
            .child(
                div()
                    .text_sm()
                    .child(format!("Enter the 6-digit code sent to {}", self.email)),
            )
            .child(
                div()
                    .flex()
                    .justify_center()
                    .p(px(16.0))
                    .rounded_md()
                    .bg(rgb(0x21252b))
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .child(code_display),
            )
            .child(
                div()
                    .text_xs()
                    .child(if self.resend_enabled {
                        "Resend code available".to_string()
                    } else {
                        format!("Resend code in {} seconds", self.countdown)
                    }),
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
