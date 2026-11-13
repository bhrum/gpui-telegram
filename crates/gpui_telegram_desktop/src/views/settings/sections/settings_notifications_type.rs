// Source: Telegram/SourceFiles/settings/sections/settings_notifications_type.cpp, Telegram/SourceFiles/settings/sections/settings_notifications_type.h
// Subsystem: settings | Feature ID: TCG-N31
// Upstream: ExceptionsController::session, ExceptionsController::prepare, rpl::NotificationsEnabledForTypeValue

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerNotificationException {
    pub peer_id: u64,
    pub name: String,
    pub is_muted: bool,
    pub custom_sound: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SettingsNotificationsTypeSections {
    pub chat_type_name: String,
    pub alerts_enabled: bool,
    pub message_preview: bool,
    pub sound_enabled: bool,
    pub exceptions: Vec<PeerNotificationException>,
}

impl SettingsNotificationsTypeSections {
    pub fn new(chat_type_name: impl Into<String>) -> Self {
        Self {
            chat_type_name: chat_type_name.into(),
            alerts_enabled: true,
            message_preview: true,
            sound_enabled: true,
            exceptions: Vec::new(),
        }
    }

    pub fn toggle_alerts(&mut self) {
        self.alerts_enabled = !self.alerts_enabled;
    }

    pub fn toggle_preview(&mut self) {
        self.message_preview = !self.message_preview;
    }

    pub fn toggle_sound(&mut self) {
        self.sound_enabled = !self.sound_enabled;
    }

    pub fn add_exception(
        &mut self,
        peer_id: u64,
        name: impl Into<String>,
        is_muted: bool,
        custom_sound: Option<String>,
    ) {
        self.exceptions.retain(|e| e.peer_id != peer_id);
        self.exceptions.push(PeerNotificationException {
            peer_id,
            name: name.into(),
            is_muted,
            custom_sound,
        });
    }

    pub fn remove_exception(&mut self, peer_id: u64) -> bool {
        let initial_len = self.exceptions.len();
        self.exceptions.retain(|e| e.peer_id != peer_id);
        self.exceptions.len() < initial_len
    }
}

impl Default for SettingsNotificationsTypeSections {
    fn default() -> Self {
        let mut s = Self::new("Private Chats");
        s.add_exception(2001, "Alice", true, None);
        s.add_exception(2002, "Bob", false, Some("Chime".into()));
        s
    }
}

impl Render for SettingsNotificationsTypeSections {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.exceptions.len();

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
                    .child(format!("Notifications for {}", self.chat_type_name))
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
                            .child(div().child("Show Notifications"))
                            .child(div().text_color(if self.alerts_enabled { rgb(0x4caf50) } else { rgb(0x707579) }).child(if self.alerts_enabled { "On" } else { "Off" }))
                    )
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .child(div().child("Message Preview"))
                            .child(div().text_color(if self.message_preview { rgb(0x4caf50) } else { rgb(0x707579) }).child(if self.message_preview { "On" } else { "Off" }))
                    )
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .child(div().child("Sound"))
                            .child(div().text_color(if self.sound_enabled { rgb(0x4caf50) } else { rgb(0x707579) }).child(if self.sound_enabled { "On" } else { "Off" }))
                    )
            )
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(div().font_weight(FontWeight::SEMIBOLD).child("Exceptions"))
                    .child(div().text_xs().text_color(rgb(0x2481cc)).child("+ Add Exception"))
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .children(self.exceptions.iter().map(|exc| {
                        div()
                            .flex()
                            .justify_between()
                            .p(px(10.0))
                            .rounded(px(6.0))
                            .bg(rgb(0xfafafa))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(div().font_weight(FontWeight::MEDIUM).child(exc.name.clone()))
                                    .child(div().text_xs().text_color(rgb(0x8e8e93)).child(if exc.is_muted { "Muted" } else { "Always unmuted" }))
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0xcc2424))
                                    .child("Delete")
                            )
                    }))
            )
    }
}
