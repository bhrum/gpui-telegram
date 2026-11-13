//! Authentic GPUI Settings Cloud Password Email Confirm Step
//! Source: Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_email_confirm.cpp, settings_cloud_password_email_confirm.h
//! Subsystem: settings | Feature ID: TCG-N33

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_RESEND_TIMEOUT_SEC: u32 = 60;
pub const K_CODE_LENGTH: usize = 6;

#[derive(Debug, Clone)]
pub struct SettingsCloudPasswordEmailConfirmCloudPassword {
    pub target_email: String,
    pub entered_digits: String,
    pub countdown_seconds: u32,
    pub can_resend: bool,
    pub is_verifying: bool,
    pub error_text: Option<String>,
}

impl SettingsCloudPasswordEmailConfirmCloudPassword {
    pub fn new() -> Self {
        Self {
            target_email: String::new(),
            entered_digits: String::new(),
            countdown_seconds: K_RESEND_TIMEOUT_SEC,
            can_resend: false,
            is_verifying: false,
            error_text: None,
        }
    }

    pub fn set_target_email(&mut self, email: impl Into<String>) {
        self.target_email = email.into();
    }

    pub fn input_digit(&mut self, ch: char) -> bool {
        if ch.is_ascii_digit() && self.entered_digits.len() < K_CODE_LENGTH {
            self.entered_digits.push(ch);
            self.error_text = None;
            return self.entered_digits.len() == K_CODE_LENGTH;
        }
        false
    }

    pub fn backspace(&mut self) {
        self.entered_digits.pop();
        self.error_text = None;
    }

    pub fn clear_code(&mut self) {
        self.entered_digits.clear();
        self.error_text = None;
    }

    pub fn tick_timer(&mut self) {
        if self.countdown_seconds > 0 {
            self.countdown_seconds -= 1;
            if self.countdown_seconds == 0 {
                self.can_resend = true;
            }
        }
    }

    pub fn resend_code(&mut self) -> bool {
        if self.can_resend {
            self.countdown_seconds = K_RESEND_TIMEOUT_SEC;
            self.can_resend = false;
            self.clear_code();
            return true;
        }
        false
    }

    pub fn is_complete(&self) -> bool {
        self.entered_digits.len() == K_CODE_LENGTH
    }
}

impl Default for SettingsCloudPasswordEmailConfirmCloudPassword {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsCloudPasswordEmailConfirmCloudPassword {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let code_display = if self.entered_digits.is_empty() {
            "------".to_string()
        } else {
            format!("{:-<6}", self.entered_digits)
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
                    .child("Verification Code"),
            )
            .child(
                div()
                    .text_sm()
                    .child(format!("Please enter the 6-digit code sent to {}", self.target_email)),
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
                    .child(if self.can_resend {
                        "Resend code available".to_string()
                    } else {
                        format!("Resend code in {} seconds", self.countdown_seconds)
                    }),
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
