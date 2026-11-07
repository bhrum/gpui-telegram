// Source: Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_manage.cpp
// Subsystem: settings | Feature ID: TCG-N02
// Upstream: CloudPasswordManageId, Manage::createPinnedToBottom, tr::lng_settings_cloud_password_manage_about1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManageAction {
    ChangePassword,
    DisablePassword,
    SetRecoveryEmail,
}

#[derive(Debug, Clone)]
pub struct SettingsCloudPasswordManageCloudPassword {
    pub has_recovery_email: bool,
    pub hint: String,
    pub pending_action: Option<ManageAction>,
    pub confirm_disable_dialog_open: bool,
    pub status_message: Option<String>,
}

impl SettingsCloudPasswordManageCloudPassword {
    pub fn new(has_recovery_email: bool, hint: impl Into<String>) -> Self {
        Self {
            has_recovery_email,
            hint: hint.into(),
            pending_action: None,
            confirm_disable_dialog_open: false,
            status_message: None,
        }
    }

    pub fn change_password(&mut self) {
        self.pending_action = Some(ManageAction::ChangePassword);
        self.status_message = Some("Navigating to change password...".to_string());
    }

    pub fn request_disable_password(&mut self) {
        self.confirm_disable_dialog_open = true;
        self.pending_action = Some(ManageAction::DisablePassword);
    }

    pub fn confirm_disable_password(&mut self) -> bool {
        if self.confirm_disable_dialog_open {
            self.confirm_disable_dialog_open = false;
            self.pending_action = None;
            self.status_message = Some("Two-step verification has been disabled.".to_string());
            true
        } else {
            false
        }
    }

    pub fn cancel_disable(&mut self) {
        self.confirm_disable_dialog_open = false;
        self.pending_action = None;
    }

    pub fn update_recovery_email(&mut self) {
        self.pending_action = Some(ManageAction::SetRecoveryEmail);
        self.status_message = Some("Navigating to recovery email settings...".to_string());
    }

    pub fn has_password(&self) -> bool {
        true
    }
}

impl Default for SettingsCloudPasswordManageCloudPassword {
    fn default() -> Self {
        Self::new(true, "First pet name")
    }
}

impl Render for SettingsCloudPasswordManageCloudPassword {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
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
                    .child("Two-Step Verification Active")
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x707579))
                    .child(format!(
                        "Password hint: {}. You will need to enter your password when logging in on a new device.",
                        if self.hint.is_empty() { "None" } else { &self.hint }
                    ))
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .p(px(12.0))
                    .rounded(px(8.0))
                    .bg(rgb(0xf4f4f5))
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .child(div().child("Change Password"))
                            .child(div().text_color(rgb(0x2481cc)).child("Edit"))
                    )
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .child(div().child("Turn Off Password"))
                            .child(div().text_color(rgb(0xcc2424)).child("Disable"))
                    )
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .child(div().child(if self.has_recovery_email { "Change Recovery Email" } else { "Set Recovery Email" }))
                            .child(div().text_color(rgb(0x2481cc)).child(if self.has_recovery_email { "Configured" } else { "Set" }))
                    )
            );
        if self.confirm_disable_dialog_open {
            root = root.child(
                div()
                    .p(px(12.0))
                    .rounded(px(8.0))
                    .bg(rgb(0xffebee))
                    .border_1()
                    .border_color(rgb(0xef5350))
                    .child("Are you sure you want to disable your cloud password? Anyone who gets your SMS code can access your account.")
            );
        }
        if let Some(msg) = self.status_message.as_ref() {
            root = root.child(
                div()
                    .text_xs()
                    .text_color(rgb(0x4caf50))
                    .child(msg.clone())
            );
        }
        root
    }
}
