//! Upstream Source: Telegram/SourceFiles/intro/intro_widget.cpp, intro_widget.h
//! Subsystem: intro | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use super::intro_code::IntroCode;
use super::intro_email::IntroEmail;
use super::intro_password_check::IntroPasswordCheck;
use super::intro_phone::IntroPhone;
use super::intro_qr::IntroQr;
use super::intro_signup::IntroSignup;
use super::intro_start::IntroStart;
use super::intro_step::{IntroCallStatus, IntroStepData, IntroStepType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IntroEnterPoint {
    #[default]
    Start,
    Phone,
    Qr,
}

#[derive(Debug, Clone)]
pub struct IntroWidget {
    pub current_step: IntroStepType,
    pub history: Vec<IntroStepType>,
    pub data: IntroStepData,
    pub language_code: String,
    pub is_settings_open: bool,

    // Step views
    pub start_view: IntroStart,
    pub qr_view: IntroQr,
    pub phone_view: IntroPhone,
    pub code_view: IntroCode,
    pub signup_view: IntroSignup,
    pub password_check_view: IntroPasswordCheck,
    pub email_view: IntroEmail,
}

impl Default for IntroWidget {
    fn default() -> Self {
        Self::new(IntroEnterPoint::Start)
    }
}

impl IntroWidget {
    pub fn new(point: IntroEnterPoint) -> Self {
        let initial_step = match point {
            IntroEnterPoint::Start => IntroStepType::Start,
            IntroEnterPoint::Phone => IntroStepType::Phone,
            IntroEnterPoint::Qr => IntroStepType::Qr,
        };

        Self {
            current_step: initial_step,
            history: Vec::new(),
            data: IntroStepData::default(),
            language_code: "en".to_string(),
            is_settings_open: false,

            start_view: IntroStart::new(),
            qr_view: IntroQr::new(),
            phone_view: IntroPhone::new(),
            code_view: IntroCode::default(),
            signup_view: IntroSignup::default(),
            password_check_view: IntroPasswordCheck::default(),
            email_view: IntroEmail::default(),
        }
    }

    pub fn can_go_back(&self) -> bool {
        !self.history.is_empty() && self.current_step != IntroStepType::Start
    }

    pub fn show_step(&mut self, next: IntroStepType) {
        if self.current_step != next {
            self.history.push(self.current_step);
            self.current_step = next;
        }
    }

    pub fn step_back(&mut self) -> bool {
        if let Some(prev) = self.history.pop() {
            self.current_step = prev;
            true
        } else {
            false
        }
    }

    pub fn switch_language(&mut self, lang: impl Into<String>) {
        self.language_code = lang.into();
    }

    pub fn toggle_settings(&mut self) {
        self.is_settings_open = !self.is_settings_open;
    }
}

impl Render for IntroWidget {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let back_button = if self.can_go_back() {
            div()
                .size(px(40.0))
                .rounded_full()
                .hover(|s| s.bg(rgb(0x17212b)))
                .cursor_pointer()
                .flex()
                .items_center()
                .justify_center()
                .text_lg()
                .text_color(rgb(0x8292a1))
                .child("←")
        } else {
            div().size(px(40.0))
        };

        let step_content: AnyElement = match self.current_step {
            IntroStepType::Start => self.start_view.render_view(),
            IntroStepType::Qr => self.qr_view.render_view(),
            IntroStepType::Phone => self.phone_view.render_view(),
            IntroStepType::Code => self.code_view.render_view(),
            IntroStepType::Signup => self.signup_view.render_view(),
            IntroStepType::PasswordCheck => self.password_check_view.render_view(),
            IntroStepType::Email => self.email_view.render_view(),
        };

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x0e1621))
            .child(
                // TopBar with Back button and Language Switcher
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .w_full()
                    .px(px(16.0))
                    .py(px(12.0))
                    .child(back_button)
                    .child(
                        // Language switcher button
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(0x2aabee))
                            .hover(|s| s.text_color(rgb(0x54c0f5)))
                            .cursor_pointer()
                            .child(format!("Language ({})", self.language_code.to_uppercase())),
                    ),
            )
            .child(
                // Centered step container (width: 380px)
                div()
                    .flex()
                    .flex_1()
                    .items_center()
                    .justify_center()
                    .w_full()
                    .child(
                        div()
                            .w(px(380.0))
                            .flex()
                            .flex_col()
                            .items_center()
                            .child(step_content),
                    ),
            )
    }
}
