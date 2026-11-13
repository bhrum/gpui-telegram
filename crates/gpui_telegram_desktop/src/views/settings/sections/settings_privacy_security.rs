// Source: Telegram/SourceFiles/settings/sections/settings_privacy_security.cpp, Telegram/SourceFiles/settings/sections/settings_privacy_security.h
// Subsystem: settings | Feature ID: TCG-N33
// Upstream: CloudPasswordAppOutdatedBox, EditCloudPasswordBox, tr::lng_edit_privacy_contacts

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone)]
pub struct SettingsPrivacySecuritySections {
    pub blocked_count: usize,
    pub passcode_enabled: bool,
    pub two_factor_enabled: bool,
    pub active_sessions_count: usize,
    pub ttl_months: u32,
    pub sensitive_content_enabled: bool,
}

impl SettingsPrivacySecuritySections {
    pub fn new() -> Self {
        Self {
            blocked_count: 0,
            passcode_enabled: false,
            two_factor_enabled: true,
            active_sessions_count: 2,
            ttl_months: 6,
            sensitive_content_enabled: false,
        }
    }

    pub fn set_blocked_count(&mut self, count: usize) {
        self.blocked_count = count;
    }

    pub fn set_passcode_enabled(&mut self, enabled: bool) {
        self.passcode_enabled = enabled;
    }

    pub fn set_two_factor_enabled(&mut self, enabled: bool) {
        self.two_factor_enabled = enabled;
    }

    pub fn set_ttl_months(&mut self, months: u32) {
        self.ttl_months = match months {
            1 | 3 | 6 | 12 | 18 | 24 => months,
            _ => 6,
        };
    }

    pub fn toggle_sensitive_content(&mut self) {
        self.sensitive_content_enabled = !self.sensitive_content_enabled;
    }

    pub fn ttl_description(&self) -> String {
        match self.ttl_months {
            1 => "1 month".to_string(),
            3 => "3 months".to_string(),
            6 => "6 months".to_string(),
            12 => "1 year".to_string(),
            18 => "18 months".to_string(),
            24 => "2 years".to_string(),
            n => format!("{} months", n),
        }
    }
}

impl Default for SettingsPrivacySecuritySections {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsPrivacySecuritySections {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(20.0))
            .gap(px(16.0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_xl()
                    .child("Privacy and Security")
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .p(px(12.0))
                    .bg(rgb(0xf4f4f5))
                    .rounded(px(8.0))
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .child(div().child("Blocked Users"))
                            .child(div().text_color(rgb(0x707579)).child(format!("{}", self.blocked_count)))
                    )
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .child(div().child("Phone Number"))
                            .child(div().text_color(rgb(0x2481cc)).child("My Contacts"))
                    )
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .child(div().child("Last Seen & Online"))
                            .child(div().text_color(rgb(0x2481cc)).child("Everybody"))
                    )
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .child(div().child("Profile Photos"))
                            .child(div().text_color(rgb(0x2481cc)).child("Everybody"))
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .p(px(12.0))
                    .bg(rgb(0xf4f4f5))
                    .rounded(px(8.0))
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .child(div().child("Passcode Lock"))
                            .child(div().text_color(if self.passcode_enabled { rgb(0x4caf50) } else { rgb(0x707579) }).child(if self.passcode_enabled { "On" } else { "Off" }))
                    )
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .child(div().child("Two-Step Verification"))
                            .child(div().text_color(if self.two_factor_enabled { rgb(0x4caf50) } else { rgb(0x707579) }).child(if self.two_factor_enabled { "On" } else { "Off" }))
                    )
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .child(div().child("Active Sessions"))
                            .child(div().text_color(rgb(0x707579)).child(format!("{}", self.active_sessions_count)))
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .p(px(12.0))
                    .bg(rgb(0xf4f4f5))
                    .rounded(px(8.0))
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .child(div().child("Delete my account if away for"))
                            .child(div().text_color(rgb(0x2481cc)).child(self.ttl_description()))
                    )
            )
    }
}
