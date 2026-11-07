//! Upstream Source: Telegram/SourceFiles/settings/sections/settings_main.cpp, Telegram/SourceFiles/settings/sections/settings_main.h
//! Subsystem: settings | Feature ID: TCG-N31 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui::prelude::FluentBuilder;
use crate::views::settings::settings_type::SettingsSectionType;

#[derive(Debug, Clone)]
pub struct SettingsMainProfile {
    pub name: String,
    pub phone: String,
    pub username: String,
    pub bio: String,
    pub is_verified: bool,
    pub is_premium: bool,
}

#[derive(Debug, Clone)]
pub struct SettingsMainSections {
    pub profile: SettingsMainProfile,
    pub interface_scale_pct: u32,
    pub active_section: Option<SettingsSectionType>,
    pub toast_message: Option<String>,
    pub is_visible: bool,
}

impl SettingsMainSections {
    pub fn new() -> Self {
        Self {
            profile: SettingsMainProfile {
                name: "Pavel Durov".into(),
                phone: "+1 234 567 8900".into(),
                username: "durov".into(),
                bio: "Exploring GPUI and high-performance Rust interfaces".into(),
                is_verified: true,
                is_premium: true,
            },
            interface_scale_pct: 100,
            active_section: None,
            toast_message: None,
            is_visible: true,
        }
    }

    pub fn copy_phone(&mut self) -> String {
        let msg = "Phone copied to clipboard".to_string();
        self.toast_message = Some(msg.clone());
        msg
    }

    pub fn copy_username(&mut self) -> String {
        let msg = format!("Username copied: @{}", self.profile.username);
        self.toast_message = Some(msg.clone());
        msg
    }

    pub fn set_interface_scale(&mut self, scale_pct: u32) {
        self.interface_scale_pct = scale_pct.clamp(100, 300);
    }

    pub fn navigate_to(&mut self, section: SettingsSectionType) {
        self.active_section = Some(section);
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }
}

impl Default for SettingsMainSections {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsMainSections {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }

        let phone = self.profile.phone.clone();
        let username = self.profile.username.clone();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x0e1621))
            .child(
                // Profile Cover Card
                div()
                    .flex()
                    .flex_col()
                    .p(px(20.0))
                    .bg(rgb(0x17212b))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(16.0))
                            .child(
                                // Avatar circle with initials
                                div()
                                    .size(px(64.0))
                                    .rounded(px(32.0))
                                    .bg(rgb(0x5288c1))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_xl()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xffffff))
                                    .child("AV"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(2.0))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .gap(px(6.0))
                                            .child(
                                                div()
                                                    .text_base()
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(0xffffff))
                                                    .child(self.profile.name.clone()),
                                            )
                                            .when(self.profile.is_premium, |this| {
                                                this.child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(rgb(0x5288c1))
                                                        .child("⭐"),
                                                )
                                            }),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x8292a1))
                                            .hover(|s| s.text_color(rgb(0x5288c1)))
                                            .cursor_pointer()
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                                this.copy_phone();
                                                cx.notify();
                                            }))
                                            .child(phone),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x5288c1))
                                            .hover(|s| s.text_color(rgb(0x73a9e3)))
                                            .cursor_pointer()
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                                this.copy_username();
                                                cx.notify();
                                            }))
                                            .child(format!("@{}", username)),
                                    ),
                            ),
                    ),
            )
            .when_some(self.toast_message.clone(), |this, msg| {
                this.child(
                    div()
                        .mx(px(16.0))
                        .my(px(8.0))
                        .p(px(8.0))
                        .rounded(px(6.0))
                        .bg(rgb(0x232e3c))
                        .text_xs()
                        .text_color(rgb(0x5288c1))
                        .child(msg),
                )
            })
            .child(
                // Settings Section Menu Rows
                div()
                    .flex()
                    .flex_col()
                    .gap(px(1.0))
                    .pt(px(12.0))
                    .children(vec![
                        (SettingsSectionType::Information, "👤 Edit Profile", "Name, bio, username"),
                        (SettingsSectionType::Notifications, "🔔 Notifications & Sounds", "Alerts, preview, badge"),
                        (SettingsSectionType::PrivacySecurity, "🔒 Privacy & Security", "2FA, passcode, passkeys"),
                        (SettingsSectionType::Chat, "💬 Chat Settings", "Themes, stickers, emoji"),
                        (SettingsSectionType::Folders, "📁 Chat Folders", "Organize dialogs"),
                        (SettingsSectionType::Advanced, "⚙️ Advanced", "Data storage, connection"),
                        (SettingsSectionType::Calls, "📞 Calls", "Mic, speaker, camera devices"),
                        (SettingsSectionType::Credits, "⭐ Telegram Stars", "Balance & digital gifts"),
                        (SettingsSectionType::Premium, "👑 Telegram Premium", "Subscription & features"),
                        (SettingsSectionType::Shortcuts, "⌨️ Shortcuts", "Keyboard keybindings"),
                    ].into_iter().map(|(sec, title, sub)| {
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .px(px(16.0))
                            .py(px(12.0))
                            .bg(rgb(0x17212b))
                            .hover(|s| s.bg(rgb(0x232e3c)))
                            .cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _window, cx| {
                                this.navigate_to(sec);
                                cx.notify();
                            }))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(rgb(0xffffff))
                                            .child(title),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x6c7883))
                                            .child(sub),
                                    ),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0x6c7883))
                                    .child("›"),
                            )
                    })),
            )
            .into_any_element()
    }
}
