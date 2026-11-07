//! WindowSetupEmail: Mandatory email setup and 2FA recovery email confirmation gate
//! Upstream: Telegram/SourceFiles/window/window_setup_email.h, window_setup_email.cpp

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmailSetupStep {
    InputEmail,
    ConfirmCode { code_length: usize },
    Completed,
}

#[derive(Debug, Clone)]
pub struct WindowSetupEmail {
    pub email: String,
    pub confirmation_code: String,
    pub step: EmailSetupStep,
    pub error_text: Option<String>,
    pub is_busy: bool,
}

impl WindowSetupEmail {
    pub fn new() -> Self {
        Self {
            email: String::new(),
            confirmation_code: String::new(),
            step: EmailSetupStep::InputEmail,
            error_text: None,
            is_busy: false,
        }
    }

    pub fn set_email(&mut self, email: impl Into<String>) {
        self.email = email.into();
        self.error_text = None;
    }

    pub fn submit_email(&mut self) -> bool {
        if self.email.contains('@') && self.email.contains('.') {
            self.step = EmailSetupStep::ConfirmCode { code_length: 6 };
            self.error_text = None;
            true
        } else {
            self.error_text = Some("Please enter a valid email address.".to_string());
            false
        }
    }

    pub fn submit_code(&mut self, code: &str) -> bool {
        self.confirmation_code = code.to_string();
        if code.len() == 6 && code.chars().all(|c| c.is_ascii_digit()) {
            self.step = EmailSetupStep::Completed;
            self.error_text = None;
            true
        } else {
            self.error_text = Some("Invalid 6-digit confirmation code.".to_string());
            false
        }
    }

    pub fn reset(&mut self) {
        self.email.clear();
        self.confirmation_code.clear();
        self.step = EmailSetupStep::InputEmail;
        self.error_text = None;
    }
}

impl Default for WindowSetupEmail {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowSetupEmail {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(12.0))
            .p(px(24.0))
            .bg(rgb(0x17212B))
            .rounded(px(8.0))
            .child(div().font_weight(FontWeight::BOLD).text_color(rgb(0xFFFFFF)).child("Recovery Email Setup"))
            .child(format!("Step: {:?}", self.step))
    }
}
