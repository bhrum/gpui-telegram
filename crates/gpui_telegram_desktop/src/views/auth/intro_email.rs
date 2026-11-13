//! Upstream Source: Telegram/SourceFiles/intro/intro_email.cpp, intro_email.h
//! Subsystem: intro | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use super::intro_step::{IntroStep, IntroStepType};

#[derive(Debug, Clone)]
pub struct IntroEmail {
    pub step: IntroStep,
    pub email: String,
    pub verification_code: String,
    pub is_code_sent: bool,
    pub is_submitting: bool,
    pub cooldown_seconds: u32,
}

impl Default for IntroEmail {
    fn default() -> Self {
        Self::new()
    }
}

impl IntroEmail {
    pub fn new() -> Self {
        Self {
            step: IntroStep::new(IntroStepType::Email),
            email: String::new(),
            verification_code: String::new(),
            is_code_sent: false,
            is_submitting: false,
            cooldown_seconds: 0,
        }
    }

    pub fn set_email(&mut self, email: impl Into<String>) {
        self.email = email.into();
    }

    pub fn set_code(&mut self, code: impl Into<String>) {
        self.verification_code = code.into();
    }

    pub fn tick_cooldown(&mut self) {
        if self.cooldown_seconds > 0 {
            self.cooldown_seconds -= 1;
        }
    }

    pub fn send_code(&mut self) -> Result<(), &'static str> {
        if self.email.is_empty() || !self.email.contains('@') {
            self.step.show_error("Please enter a valid email address");
            return Err("Invalid email");
        }
        if self.cooldown_seconds > 0 {
            return Err("Cooldown active");
        }
        self.step.hide_error();
        self.is_code_sent = true;
        self.cooldown_seconds = 60;
        Ok(())
    }

    pub fn submit(&mut self) -> Result<(), &'static str> {
        if self.verification_code.trim().is_empty() {
            self.step.show_error("Please enter the email verification code");
            return Err("Code empty");
        }
        self.step.hide_error();
        self.is_submitting = true;
        Ok(())
    }

    pub fn render_view(&self) -> AnyElement {
        let email_text = if self.email.is_empty() {
            "Your email address".to_string()
        } else {
            self.email.clone()
        };

        let code_input_el: Option<AnyElement> = if self.is_code_sent {
            let code_text = if self.verification_code.is_empty() {
                "Verification code".to_string()
            } else {
                self.verification_code.clone()
            };
            Some(
                div()
                    .mt(px(12.0))
                    .w(px(300.0))
                    .h(px(42.0))
                    .px(px(12.0))
                    .rounded(px(6.0))
                    .bg(rgb(0x17212b))
                    .border_1()
                    .border_color(rgb(0x242f3d))
                    .flex()
                    .items_center()
                    .text_sm()
                    .text_color(rgb(0xffffff))
                    .child(code_text)
                    .into_any_element(),
            )
        } else {
            None
        };

        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .size_full()
            .p(px(24.0))
            .child(
                // Email Lottie / Icon placeholder
                div()
                    .size(px(72.0))
                    .rounded_full()
                    .bg(rgb(0x2aabee))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_2xl()
                    .text_color(rgb(0xffffff))
                    .child("✉"),
            )
            .child(
                div()
                    .mt(px(20.0))
                    .text_xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xffffff))
                    .child("Recovery Email"),
            )
            .child(
                div()
                    .mt(px(8.0))
                    .text_sm()
                    .text_color(rgb(0x8292a1))
                    .text_center()
                    .max_w(px(320.0))
                    .child("Add an email to protect your Telegram account with 2-Step Verification."),
            )
            .child(
                // Email input field
                div()
                    .mt(px(24.0))
                    .w(px(300.0))
                    .h(px(42.0))
                    .px(px(12.0))
                    .rounded(px(6.0))
                    .bg(rgb(0x17212b))
                    .border_1()
                    .border_color(rgb(0x242f3d))
                    .flex()
                    .items_center()
                    .text_sm()
                    .text_color(rgb(0xffffff))
                    .child(email_text),
            )
            .children(code_input_el)
            .child(
                // Next / Submit Button
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
                            .child(if self.is_code_sent { "VERIFY" } else { "SEND CODE" }),
                    ),
            )
            .into_any_element()
    }
}

impl Render for IntroEmail {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.render_view()
    }
}
