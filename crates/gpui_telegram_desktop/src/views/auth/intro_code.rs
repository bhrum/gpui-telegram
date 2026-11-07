//! Upstream Source: Telegram/SourceFiles/intro/intro_code.cpp, intro_code.h
//! Subsystem: intro | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use super::intro_code_input::IntroCodeInput;
use super::intro_step::{IntroStep, IntroStepType};

#[derive(Debug, Clone)]
pub struct IntroCode {
    pub step: IntroStep,
    pub input: IntroCodeInput,
    pub call_timeout_sec: i32,
    pub is_by_telegram: bool,
    pub is_submitting: bool,
    pub fragment_url: String,
}

impl Default for IntroCode {
    fn default() -> Self {
        Self::new(5, true, 60)
    }
}

impl IntroCode {
    pub fn new(length: usize, by_telegram: bool, timeout_sec: i32) -> Self {
        Self {
            step: IntroStep::new(IntroStepType::Code),
            input: IntroCodeInput::new(length),
            call_timeout_sec: timeout_sec,
            is_by_telegram: by_telegram,
            is_submitting: false,
            fragment_url: String::new(),
        }
    }

    pub fn tick_timer(&mut self) {
        if self.call_timeout_sec > 0 {
            self.call_timeout_sec -= 1;
        }
    }

    pub fn formatted_timeout(&self) -> String {
        if self.call_timeout_sec <= 0 {
            return String::new();
        }
        if self.call_timeout_sec >= 3600 {
            let hours = self.call_timeout_sec / 3600;
            let minutes = (self.call_timeout_sec / 60) % 60;
            let seconds = self.call_timeout_sec % 60;
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        } else {
            let minutes = self.call_timeout_sec / 60;
            let seconds = self.call_timeout_sec % 60;
            format!("{}:{:02}", minutes, seconds)
        }
    }

    pub fn no_telegram_code(&mut self) {
        self.is_by_telegram = false;
        self.call_timeout_sec = 60;
    }

    pub fn submit(&mut self, is_2fa_enabled: bool) -> Result<IntroStepType, &'static str> {
        if !self.input.is_complete() {
            self.input.set_error(true);
            self.step.show_error("Please enter the full code");
            return Err("Code incomplete");
        }
        self.step.hide_error();
        self.is_submitting = true;

        if is_2fa_enabled {
            Ok(IntroStepType::PasswordCheck)
        } else {
            Ok(IntroStepType::Signup)
        }
    }

    pub fn render_view(&self) -> AnyElement {
        let timeout_str = self.formatted_timeout();
        let subtitle = if self.is_by_telegram {
            "We have sent you a code via Telegram to your other devices."
        } else {
            "We have sent you a code via SMS to your phone."
        };

        let timeout_el: Option<AnyElement> = if !timeout_str.is_empty() {
            Some(
                div()
                    .mt(px(14.0))
                    .text_xs()
                    .text_color(rgb(0x8292a1))
                    .child(format!("Resend code in {}", timeout_str))
                    .into_any_element(),
            )
        } else {
            Some(
                div()
                    .mt(px(14.0))
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgb(0x2aabee))
                    .cursor_pointer()
                    .child("Send code via SMS")
                    .into_any_element(),
            )
        };

        let no_tg_el: Option<AnyElement> = if self.is_by_telegram {
            Some(
                div()
                    .mt(px(18.0))
                    .text_xs()
                    .text_color(rgb(0x2aabee))
                    .hover(|s| s.text_color(rgb(0x54c0f5)))
                    .cursor_pointer()
                    .child("Haven't received code in Telegram?")
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
                div()
                    .text_xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xffffff))
                    .child("Enter Code"),
            )
            .child(
                div()
                    .mt(px(8.0))
                    .text_sm()
                    .text_color(rgb(0x8292a1))
                    .text_center()
                    .max_w(px(320.0))
                    .child(subtitle),
            )
            .child(
                div()
                    .mt(px(24.0))
                    .child(self.input.render_view()),
            )
            .children(timeout_el)
            .children(no_tg_el)
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

impl Render for IntroCode {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.render_view()
    }
}
