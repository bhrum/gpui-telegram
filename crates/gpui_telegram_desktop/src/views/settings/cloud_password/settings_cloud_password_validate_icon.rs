// Source: Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_validate_icon.cpp
// Subsystem: settings | Feature ID: TCG-N02
// Upstream: CreateValidateGoodIcon, Emoji::Find

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationBadgeState {
    Idle,
    Checking,
    Valid,
    Invalid,
}

#[derive(Debug, Clone)]
pub struct SettingsCloudPasswordValidateIconCloudPassword {
    pub state: ValidationBadgeState,
    pub message: Option<String>,
}

impl SettingsCloudPasswordValidateIconCloudPassword {
    pub fn new(state: ValidationBadgeState) -> Self {
        Self {
            state,
            message: None,
        }
    }

    pub fn set_valid(&mut self) {
        self.state = ValidationBadgeState::Valid;
        self.message = Some("Input verified".to_string());
    }

    pub fn set_invalid(&mut self, reason: impl Into<String>) {
        self.state = ValidationBadgeState::Invalid;
        self.message = Some(reason.into());
    }

    pub fn set_checking(&mut self) {
        self.state = ValidationBadgeState::Checking;
        self.message = Some("Verifying...".to_string());
    }

    pub fn reset(&mut self) {
        self.state = ValidationBadgeState::Idle;
        self.message = None;
    }

    pub fn is_valid(&self) -> bool {
        self.state == ValidationBadgeState::Valid
    }
}

impl Default for SettingsCloudPasswordValidateIconCloudPassword {
    fn default() -> Self {
        Self::new(ValidationBadgeState::Idle)
    }
}

impl Render for SettingsCloudPasswordValidateIconCloudPassword {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let (icon_symbol, bg_color, text_color) = match self.state {
            ValidationBadgeState::Idle => ("•", rgb(0xf0f0f0), rgb(0x9e9e9e)),
            ValidationBadgeState::Checking => ("⏳", rgb(0xfff3e0), rgb(0xff9800)),
            ValidationBadgeState::Valid => ("✓", rgb(0xe8f5e9), rgb(0x4caf50)),
            ValidationBadgeState::Invalid => ("✕", rgb(0xffebee), rgb(0xf44336)),
        };

        let mut root = div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(
                div()
                    .w(px(24.0))
                    .h(px(24.0))
                    .rounded_full()
                    .bg(bg_color)
                    .text_color(text_color)
                    .flex()
                    .items_center()
                    .justify_center()
                    .font_weight(FontWeight::BOLD)
                    .text_xs()
                    .child(icon_symbol)
            );
        if let Some(msg) = self.message.as_ref() {
            root = root.child(
                div()
                    .text_xs()
                    .text_color(text_color)
                    .child(msg.clone())
            );
        }
        root
    }
}
