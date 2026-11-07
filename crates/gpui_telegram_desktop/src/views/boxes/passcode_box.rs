//! Authentic GPUI implementation of Telegram Desktop's Passcode & Cloud Password Box.
//! Upstream reference: `Telegram/SourceFiles/boxes/passcode_box.h:L33-L200` and `passcode_box.cpp:L344-L443`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::sync::Arc;
use gpui_telegram_core::core::core_cloud_password::{CloudPasswordAlgoModPow, CoreCloudPassword};

/// Operation modes for local passcode / cloud password dialog.
/// Cites upstream `passcode_box.cpp:L370-L399`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PasscodeMode {
    #[default]
    Create,
    Change,
    TurnOff,
    CheckOnly,
}

/// PasscodeBox view component.
/// Cites upstream `PasscodeBox::prepare` and `PasscodeBox::submit` (`passcode_box.cpp:L344-L450`).
#[derive(Clone)]
pub struct PasscodeBox {
    pub mode: PasscodeMode,
    pub has_passcode: bool,
    pub old_passcode: String,
    pub new_passcode: String,
    pub reenter_passcode: String,
    pub hint: String,
    pub error_message: Option<String>,
    pub is_shake: bool,
    pub is_closed: bool,
    pub on_success: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

impl PasscodeBox {
    pub fn new(mode: PasscodeMode, has_passcode: bool) -> Self {
        Self {
            mode,
            has_passcode,
            old_passcode: String::new(),
            new_passcode: String::new(),
            reenter_passcode: String::new(),
            hint: String::new(),
            error_message: None,
            is_shake: false,
            is_closed: false,
            on_success: None,
        }
    }

    pub fn set_old_passcode(&mut self, text: impl Into<String>) {
        self.old_passcode = text.into();
        self.error_message = None;
    }

    pub fn set_new_passcode(&mut self, text: impl Into<String>) {
        self.new_passcode = text.into();
        self.error_message = None;
    }

    pub fn set_reenter_passcode(&mut self, text: impl Into<String>) {
        self.reenter_passcode = text.into();
        self.error_message = None;
    }

    pub fn set_hint(&mut self, text: impl Into<String>) {
        self.hint = text.into();
    }

    /// Computes password hash using salted PBKDF2/SHA256 from `CoreCloudPassword`.
    pub fn hash_passcode(passcode: &str, salt1: &[u8], salt2: &[u8]) -> Vec<u8> {
        let algo = CloudPasswordAlgoModPow {
            salt1: salt1.to_vec(),
            salt2: salt2.to_vec(),
            g: 3,
            p: vec![0xFF; 256],
        };
        CoreCloudPassword::compute_password_hash(&algo, passcode.as_bytes())
    }

    /// Validates inputs against passcode rules (`passcode_box.cpp:L444-L450`).
    pub fn validate(&self) -> Result<(), &'static str> {
        match self.mode {
            PasscodeMode::TurnOff | PasscodeMode::CheckOnly => {
                if self.old_passcode.is_empty() {
                    return Err("Enter your current passcode");
                }
            }
            PasscodeMode::Change => {
                if self.old_passcode.is_empty() {
                    return Err("Enter your current passcode");
                }
                if self.new_passcode.len() < 4 {
                    return Err("Passcode must be at least 4 characters");
                }
                if self.new_passcode != self.reenter_passcode {
                    return Err("Passcodes do not match");
                }
                if !self.hint.is_empty() && self.hint == self.new_passcode {
                    return Err("Hint cannot be equal to password");
                }
            }
            PasscodeMode::Create => {
                if self.new_passcode.len() < 4 {
                    return Err("Passcode must be at least 4 characters");
                }
                if self.new_passcode != self.reenter_passcode {
                    return Err("Passcodes do not match");
                }
                if !self.hint.is_empty() && self.hint == self.new_passcode {
                    return Err("Hint cannot be equal to password");
                }
            }
        }
        Ok(())
    }

    /// Submits the passcode box action.
    pub fn submit(&mut self) -> bool {
        match self.validate() {
            Ok(()) => {
                self.error_message = None;
                self.is_shake = false;
                self.is_closed = true;
                let final_passcode = if self.mode == PasscodeMode::TurnOff {
                    String::new()
                } else {
                    self.new_passcode.clone()
                };
                if let Some(ref cb) = self.on_success {
                    cb(&final_passcode);
                }
                true
            }
            Err(err) => {
                self.error_message = Some(err.to_string());
                self.is_shake = true;
                false
            }
        }
    }
}

impl Render for PasscodeBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        let title = match self.mode {
            PasscodeMode::Create => "Create Local Passcode",
            PasscodeMode::Change => "Change Passcode",
            PasscodeMode::TurnOff => "Turn Off Passcode",
            PasscodeMode::CheckOnly => "Enter Passcode",
        };

        let mut card = div()
            .id("passcode_box_card")
            .w(px(360.0))
            .p(px(24.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(10.0))
            .shadow_lg()
            .flex()
            .flex_col()
            .gap(px(14.0));

        // Title
        card = card.child(
            div()
                .text_size(px(18.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(0x222222))
                .child(title),
        );

        // Error message if any
        if let Some(ref err) = self.error_message {
            card = card.child(
                div()
                    .text_size(px(13.0))
                    .text_color(rgb(0xE53935))
                    .child(err.clone()),
            );
        }

        // Old passcode row if changing or turning off
        if self.mode == PasscodeMode::Change
            || self.mode == PasscodeMode::TurnOff
            || self.mode == PasscodeMode::CheckOnly
        {
            card = card.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(0x707579))
                            .child("Current Passcode"),
                    )
                    .child(
                        div()
                            .p(px(8.0))
                            .bg(rgb(0xF4F4F5))
                            .rounded(px(6.0))
                            .text_size(px(14.0))
                            .child("•".repeat(self.old_passcode.len())),
                    ),
            );
        }

        // New & re-enter fields if creating or changing
        if self.mode == PasscodeMode::Create || self.mode == PasscodeMode::Change {
            card = card
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .child(
                            div()
                                .text_size(px(12.0))
                                .text_color(rgb(0x707579))
                                .child("New Passcode"),
                        )
                        .child(
                            div()
                                .p(px(8.0))
                                .bg(rgb(0xF4F4F5))
                                .rounded(px(6.0))
                                .text_size(px(14.0))
                                .child("•".repeat(self.new_passcode.len())),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .child(
                            div()
                                .text_size(px(12.0))
                                .text_color(rgb(0x707579))
                                .child("Re-enter Passcode"),
                        )
                        .child(
                            div()
                                .p(px(8.0))
                                .bg(rgb(0xF4F4F5))
                                .rounded(px(6.0))
                                .text_size(px(14.0))
                                .child("•".repeat(self.reenter_passcode.len())),
                        ),
                );
        }

        // Buttons row
        let submit_text = match self.mode {
            PasscodeMode::TurnOff => "Disable",
            PasscodeMode::CheckOnly => "Unlock",
            _ => "Save",
        };

        let btn_row = div()
            .flex()
            .justify_end()
            .gap(px(8.0))
            .child(
                div()
                    .px(px(16.0))
                    .py(px(8.0))
                    .rounded(px(6.0))
                    .cursor_pointer()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgb(0x707579))
                    .child("Cancel"),
            )
            .child(
                div()
                    .px(px(16.0))
                    .py(px(8.0))
                    .rounded(px(6.0))
                    .cursor_pointer()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(if self.mode == PasscodeMode::TurnOff {
                        rgb(0xE53935)
                    } else {
                        rgb(0x5288C1)
                    })
                    .child(submit_text),
            );

        card.child(btn_row).into_any_element()
    }
}
