//! Upstream Source: Telegram/SourceFiles/intro/intro_password_check.cpp, intro_password_check.h
//! Subsystem: intro | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use super::intro_step::{IntroStep, IntroStepType};

#[derive(Debug, Clone)]
pub struct IntroPasswordCheck {
    pub step: IntroStep,
    pub password: String,
    pub hint: String,
    pub is_password_visible: bool,
    pub is_recovering: bool,
    pub recovery_email_pattern: String,
    pub is_submitting: bool,
}

impl Default for IntroPasswordCheck {
    fn default() -> Self {
        Self::new("My usual secret")
    }
}

impl IntroPasswordCheck {
    pub fn new(hint: impl Into<String>) -> Self {
        Self {
            step: IntroStep::new(IntroStepType::PasswordCheck),
            password: String::new(),
            hint: hint.into(),
            is_password_visible: false,
            is_recovering: false,
            recovery_email_pattern: "a***@gmail.com".to_string(),
            is_submitting: false,
        }
    }

    pub fn set_password(&mut self, pwd: impl Into<String>) {
        self.password = pwd.into();
    }

    pub fn toggle_visibility(&mut self) {
        self.is_password_visible = !self.is_password_visible;
    }

    pub fn to_recover(&mut self) {
        self.is_recovering = true;
    }

    pub fn to_password(&mut self) {
        self.is_recovering = false;
    }

    pub fn submit(&mut self) -> Result<(), &'static str> {
        if self.password.is_empty() {
            self.step.show_error("Please enter your cloud password");
            return Err("Password empty");
        }
        self.step.hide_error();
        self.is_submitting = true;
        Ok(())
    }

    pub fn render_view(&self) -> AnyElement {
        let masked = if self.is_password_visible {
            self.password.clone()
        } else {
            "•".repeat(self.password.len())
        };

        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .size_full()
            .p(px(24.0))
            .child(
                div()
                    .text_xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xffffff))
                    .child("Two-Step Verification"),
            )
            .child(
                div()
                    .mt(px(8.0))
                    .text_sm()
                    .text_color(rgb(0x8292a1))
                    .text_center()
                    .max_w(px(320.0))
                    .child("Your account is protected with an additional password."),
            )
            .child(
                // Password input field (width: 300px, height: 42px) with eye toggle
                div()
                    .mt(px(28.0))
                    .w(px(300.0))
                    .h(px(42.0))
                    .px(px(12.0))
                    .rounded(px(6.0))
                    .bg(rgb(0x17212b))
                    .border_1()
                    .border_color(rgb(0x242f3d))
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xffffff))
                            .child(if self.password.is_empty() {
                                "Password".to_string()
                            } else {
                                masked
                            }),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .text_xs()
                            .text_color(rgb(0x8292a1))
                            .child(if self.is_password_visible { "HIDE" } else { "SHOW" }),
                    ),
            )
            .child(
                if !self.hint.is_empty() {
                    div()
                        .mt(px(10.0))
                        .text_xs()
                        .text_color(rgb(0x8292a1))
                        .child(format!("Hint: {}", self.hint))
                } else {
                    div()
                },
            )
            .child(
                // Forgot password? Link
                div()
                    .mt(px(20.0))
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgb(0x2aabee))
                    .hover(|s| s.text_color(rgb(0x54c0f5)))
                    .cursor_pointer()
                    .child("Forgot password?"),
            )
            .child(
                // Next Button
                div()
                    .mt(px(28.0))
                    .w(px(300.0))
                    .h(px(42.0))
                    .rounded(px(6.0))
                    .bg(rgb(0x2aabee))
                    .hover(|s| s.bg(rgb(0x229ed9)))
                    .cursor_pointer()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_base()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0xffffff))
                            .child("NEXT"),
                    ),
            )
            .into_any_element()
    }
}

impl Render for IntroPasswordCheck {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.render_view()
    }
}
