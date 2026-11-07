//! Upstream Source: Telegram/SourceFiles/intro/intro_step.cpp, intro_step.h
//! Subsystem: intro | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IntroStepType {
    #[default]
    Start,
    Qr,
    Phone,
    Code,
    Signup,
    PasswordCheck,
    Email,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IntroCallStatus {
    Waiting,
    Calling,
    Called,
    #[default]
    Disabled,
}

#[derive(Debug, Clone, Default)]
pub struct IntroStepData {
    pub country: String,
    pub phone: String,
    pub phone_hash: Vec<u8>,
    pub call_status: IntroCallStatus,
    pub call_timeout: i32,
    pub code_length: usize,
    pub code_by_telegram: bool,
    pub code_by_fragment_url: String,
    pub email_pattern_login: String,
    pub pwd_hint: String,
    pub error_text: Option<String>,
}

#[derive(Debug, Clone)]
pub struct IntroStep {
    pub step_type: IntroStepType,
    pub title: String,
    pub description: String,
    pub has_cover: bool,
    pub data: IntroStepData,
    pub is_animating: bool,
}

impl Default for IntroStep {
    fn default() -> Self {
        Self::new(IntroStepType::Start)
    }
}

impl IntroStep {
    pub fn new(step_type: IntroStepType) -> Self {
        let (title, description, has_cover) = match step_type {
            IntroStepType::Start => (
                "Telegram Desktop".to_string(),
                "The world's fastest messaging app. Free and secure.".to_string(),
                true,
            ),
            IntroStepType::Qr => (
                "Log in to Telegram by QR Code".to_string(),
                "Scan from Telegram on your phone".to_string(),
                false,
            ),
            IntroStepType::Phone => (
                "Your Phone Number".to_string(),
                "Please confirm your country code and enter your phone number.".to_string(),
                false,
            ),
            IntroStepType::Code => (
                "Enter Code".to_string(),
                "We've sent the code to your other device or SMS.".to_string(),
                false,
            ),
            IntroStepType::Signup => (
                "Your Name".to_string(),
                "Enter your name and add a profile photo.".to_string(),
                false,
            ),
            IntroStepType::PasswordCheck => (
                "Two-Step Verification".to_string(),
                "Your account is protected with an additional password.".to_string(),
                false,
            ),
            IntroStepType::Email => (
                "Login Email".to_string(),
                "Enter the verification code sent to your email.".to_string(),
                false,
            ),
        };

        Self {
            step_type,
            title,
            description,
            has_cover,
            data: IntroStepData {
                code_length: 5,
                ..Default::default()
            },
            is_animating: false,
        }
    }

    pub fn has_back(&self) -> bool {
        self.step_type != IntroStepType::Start
    }

    pub fn next_button_text(&self) -> &'static str {
        match self.step_type {
            IntroStepType::Start => "Start Messaging",
            IntroStepType::Qr => "Log in by phone number",
            IntroStepType::Phone => "Next",
            IntroStepType::Code => "Next",
            IntroStepType::Signup => "Sign up",
            IntroStepType::PasswordCheck => "Next",
            IntroStepType::Email => "Verify",
        }
    }

    pub fn show_error(&mut self, text: impl Into<String>) {
        self.data.error_text = Some(text.into());
    }

    pub fn hide_error(&mut self) {
        self.data.error_text = None;
    }

    pub fn fill_sent_code_data(&mut self, code_len: usize, is_telegram: bool, timeout: i32, fragment_url: Option<String>) {
        self.data.code_length = code_len;
        self.data.code_by_telegram = is_telegram;
        self.data.call_timeout = timeout;
        self.data.code_by_fragment_url = fragment_url.unwrap_or_default();
    }
}

impl Render for IntroStep {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let err_el = self.data.error_text.as_ref().map(|err| {
            div()
                .text_sm()
                .text_color(rgb(0xee5655))
                .py(px(4.0))
                .child(err.clone())
        });

        div()
            .flex()
            .flex_col()
            .items_center()
            .w_full()
            .max_w(px(380.0))
            .child(
                div()
                    .text_xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xffffff))
                    .child(self.title.clone()),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x8292a1))
                    .py(px(8.0))
                    .text_center()
                    .child(self.description.clone()),
            )
            .children(err_el)
    }
}
