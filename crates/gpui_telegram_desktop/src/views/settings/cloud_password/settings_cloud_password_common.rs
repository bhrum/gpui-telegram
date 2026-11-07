//! Authentic GPUI Settings Cloud Password Common Module
//! Source: Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_common.cpp, settings_cloud_password_common.h
//! Subsystem: settings | Feature ID: TCG-N33

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_TIMER_CHECK_MS: u64 = 500;
pub const K_AUTO_CLOSE_TIMEOUT_MS: u64 = 60000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CloudPasswordStage {
    Start,
    SetPassword,
    ConfirmPassword,
    PasswordHint,
    RecoveryEmail,
    EmailConfirm,
    LoginEmail,
    LoginEmailConfirm,
    Manage,
}

#[derive(Debug, Clone)]
pub struct PasswordFields {
    pub password_hash: Vec<u8>,
    pub salt: Vec<u8>,
    pub hint: String,
    pub email: String,
    pub has_recovery: bool,
}

impl Default for PasswordFields {
    fn default() -> Self {
        Self {
            password_hash: Vec::new(),
            salt: Vec::new(),
            hint: String::new(),
            email: String::new(),
            has_recovery: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SettingsCloudPasswordCommonCloudPassword {
    pub current_stage: CloudPasswordStage,
    pub fields: PasswordFields,
    pub is_loading: bool,
    pub error_text: Option<String>,
    pub timer_active: bool,
    pub auto_close_remaining_ms: u64,
}

impl SettingsCloudPasswordCommonCloudPassword {
    pub fn new() -> Self {
        Self {
            current_stage: CloudPasswordStage::Start,
            fields: PasswordFields::default(),
            is_loading: false,
            error_text: None,
            timer_active: false,
            auto_close_remaining_ms: K_AUTO_CLOSE_TIMEOUT_MS,
        }
    }

    pub fn set_stage(&mut self, stage: CloudPasswordStage) {
        self.current_stage = stage;
        self.error_text = None;
    }

    pub fn update_hint(&mut self, hint: impl Into<String>) {
        self.fields.hint = hint.into();
    }

    pub fn update_email(&mut self, email: impl Into<String>) {
        self.fields.email = email.into();
        self.fields.has_recovery = !self.fields.email.is_empty();
    }

    pub fn set_error(&mut self, err: impl Into<String>) {
        self.error_text = Some(err.into());
    }

    pub fn clear_error(&mut self) {
        self.error_text = None;
    }

    pub fn reset_fields(&mut self) {
        self.fields = PasswordFields::default();
        self.error_text = None;
        self.is_loading = false;
    }

    pub fn is_active_2fa(&self) -> bool {
        self.current_stage == CloudPasswordStage::Manage
    }
}

impl Default for SettingsCloudPasswordCommonCloudPassword {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsCloudPasswordCommonCloudPassword {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let stage_str = format!("{:?}", self.current_stage);
        let mut root = div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .gap(px(12.0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_lg()
                    .child("Two-Step Verification Common State"),
            )
            .child(
                div()
                    .text_sm()
                    .child(format!("Current Stage: {}", stage_str)),
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
