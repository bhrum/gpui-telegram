// Source: Telegram/SourceFiles/settings/sections/settings_notifications.cpp, Telegram/SourceFiles/settings/sections/settings_notifications.h
// Subsystem: settings | Feature ID: TCG-N31
// Upstream: tr::lng_notification_private_chats, tr::lng_notification_exceptions_view, NotificationsCount::NotificationsCount

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgeCountType {
    CountMessages,
    CountChats,
}

#[derive(Debug, Clone)]
pub struct SettingsNotificationsSections {
    pub private_chats_enabled: bool,
    pub groups_enabled: bool,
    pub channels_enabled: bool,
    pub include_muted_in_badge: bool,
    pub badge_count_type: BadgeCountType,
    pub flash_taskbar_icon: bool,
    pub sound_enabled: bool,
    pub exceptions_count: usize,
}

impl SettingsNotificationsSections {
    pub fn new() -> Self {
        Self {
            private_chats_enabled: true,
            groups_enabled: true,
            channels_enabled: false,
            include_muted_in_badge: false,
            badge_count_type: BadgeCountType::CountMessages,
            flash_taskbar_icon: true,
            sound_enabled: true,
            exceptions_count: 5,
        }
    }

    pub fn toggle_private_chats(&mut self) {
        self.private_chats_enabled = !self.private_chats_enabled;
    }

    pub fn toggle_groups(&mut self) {
        self.groups_enabled = !self.groups_enabled;
    }

    pub fn toggle_channels(&mut self) {
        self.channels_enabled = !self.channels_enabled;
    }

    pub fn toggle_badge_include_muted(&mut self) {
        self.include_muted_in_badge = !self.include_muted_in_badge;
    }

    pub fn set_badge_count_type(&mut self, count_type: BadgeCountType) {
        self.badge_count_type = count_type;
    }

    pub fn toggle_flash_taskbar(&mut self) {
        self.flash_taskbar_icon = !self.flash_taskbar_icon;
    }

    pub fn toggle_sound(&mut self) {
        self.sound_enabled = !self.sound_enabled;
    }
}

impl Default for SettingsNotificationsSections {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsNotificationsSections {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let badge_label = match self.badge_count_type {
            BadgeCountType::CountMessages => "Unread Messages",
            BadgeCountType::CountChats => "Unread Chats",
        };

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
                    .child("Notifications and Sounds")
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
                            .child(div().child("Private Chats"))
                            .child(div().text_color(if self.private_chats_enabled { rgb(0x4caf50) } else { rgb(0x707579) }).child(if self.private_chats_enabled { "On" } else { "Off" }))
                    )
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .child(div().child("Groups"))
                            .child(div().text_color(if self.groups_enabled { rgb(0x4caf50) } else { rgb(0x707579) }).child(if self.groups_enabled { "On" } else { "Off" }))
                    )
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .child(div().child("Channels"))
                            .child(div().text_color(if self.channels_enabled { rgb(0x4caf50) } else { rgb(0x707579) }).child(if self.channels_enabled { "On" } else { "Off" }))
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
                            .child(div().child("Badge Counter Mode"))
                            .child(div().text_color(rgb(0x2481cc)).child(badge_label))
                    )
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .child(div().child("Include Muted Chats in Badge"))
                            .child(div().text_color(if self.include_muted_in_badge { rgb(0x4caf50) } else { rgb(0x707579) }).child(if self.include_muted_in_badge { "On" } else { "Off" }))
                    )
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .child(div().child("Flash Taskbar Icon"))
                            .child(div().text_color(if self.flash_taskbar_icon { rgb(0x4caf50) } else { rgb(0x707579) }).child(if self.flash_taskbar_icon { "On" } else { "Off" }))
                    )
            )
            .child(
                div()
                    .flex()
                    .justify_between()
                    .p(px(12.0))
                    .bg(rgb(0xf4f4f5))
                    .rounded(px(8.0))
                    .child(div().child("Custom Exceptions"))
                    .child(div().text_color(rgb(0x707579)).child(format!("{} chats", self.exceptions_count)))
            )
    }
}
