// Source: Telegram/SourceFiles/settings/sections/settings_notifications_reactions.cpp, Telegram/SourceFiles/settings/sections/settings_notifications_reactions.h
// Subsystem: settings | Feature ID: TCG-N14
// Upstream: tr::lng_notification_reactions_notify_about, tr::lng_notification_reactions_from_all, tr::lng_notification_reactions_from_contacts

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReactionsNotifySource {
    All,
    Contacts,
}

#[derive(Debug, Clone)]
pub struct SettingsNotificationsReactionsSections {
    pub notify_reactions: bool,
    pub notify_stories: bool,
    pub source: ReactionsNotifySource,
    pub sound_enabled: bool,
    pub sound_name: String,
}

impl SettingsNotificationsReactionsSections {
    pub fn new() -> Self {
        Self {
            notify_reactions: true,
            notify_stories: true,
            source: ReactionsNotifySource::All,
            sound_enabled: true,
            sound_name: "Default".into(),
        }
    }

    pub fn toggle_notify_reactions(&mut self) {
        self.notify_reactions = !self.notify_reactions;
    }

    pub fn toggle_notify_stories(&mut self) {
        self.notify_stories = !self.notify_stories;
    }

    pub fn set_source(&mut self, source: ReactionsNotifySource) {
        self.source = source;
    }

    pub fn toggle_sound(&mut self) {
        self.sound_enabled = !self.sound_enabled;
    }

    pub fn set_sound_name(&mut self, name: impl Into<String>) {
        self.sound_name = name.into();
    }
}

impl Default for SettingsNotificationsReactionsSections {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsNotificationsReactionsSections {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let source_label = match self.source {
            ReactionsNotifySource::All => "Everyone",
            ReactionsNotifySource::Contacts => "Contacts Only",
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
                    .child("Reactions Notifications")
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
                            .child(div().child("Notify About Reactions"))
                            .child(div().text_color(if self.notify_reactions { rgb(0x4caf50) } else { rgb(0x707579) }).child(if self.notify_reactions { "On" } else { "Off" }))
                    )
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .child(div().child("Notify About Stories Reactions"))
                            .child(div().text_color(if self.notify_stories { rgb(0x4caf50) } else { rgb(0x707579) }).child(if self.notify_stories { "On" } else { "Off" }))
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
                            .child(div().child("Reactions From"))
                            .child(div().text_color(rgb(0x2481cc)).child(source_label))
                    )
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .child(div().child("Reaction Sound"))
                            .child(div().text_color(rgb(0x2481cc)).child(if self.sound_enabled { self.sound_name.clone() } else { "None".to_string() }))
                    )
            )
    }
}
