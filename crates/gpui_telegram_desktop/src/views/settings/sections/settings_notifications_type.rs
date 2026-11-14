// Source: Telegram/SourceFiles/settings/sections/settings_notifications_type.cpp

use crate::i18n::tr_str;
use crate::theme::assets::IconName;
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::settings::settings_theme_ui::{
    settings_action_row, settings_card, settings_container, settings_empty_state,
    settings_section_header, settings_subsection_title, settings_toggle_row,
};
use gpui::prelude::FluentBuilder;
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
    pub palette: TdesktopThemePalette,
    pub language_id: String,
}

impl SettingsNotificationsTypeSections {
    pub fn new(chat_type_name: impl Into<String>) -> Self {
        Self {
            chat_type_name: chat_type_name.into(),
            alerts_enabled: true,
            message_preview: true,
            sound_enabled: true,
            exceptions: Vec::new(),
            palette: TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x5288C1, 14, 12),
            language_id: "en".into(),
        }
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) { self.palette = palette; }
    pub fn set_language(&mut self, lang: impl Into<String>) { self.language_id = lang.into(); }
    pub fn toggle_alerts(&mut self) { self.alerts_enabled = !self.alerts_enabled; }
    pub fn toggle_preview(&mut self) { self.message_preview = !self.message_preview; }
    pub fn toggle_sound(&mut self) { self.sound_enabled = !self.sound_enabled; }

    pub fn add_exception(&mut self, peer_id: u64, name: impl Into<String>, is_muted: bool, custom_sound: Option<String>) {
        self.exceptions.retain(|e| e.peer_id != peer_id);
        self.exceptions.push(PeerNotificationException { peer_id, name: name.into(), is_muted, custom_sound });
    }

    pub fn remove_exception(&mut self, peer_id: u64) -> bool {
        let initial_len = self.exceptions.len();
        self.exceptions.retain(|e| e.peer_id != peer_id);
        self.exceptions.len() < initial_len
    }
}

impl Default for SettingsNotificationsTypeSections {
    fn default() -> Self {
        Self::new("Private Chats")
    }
}

impl Render for SettingsNotificationsTypeSections {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette;
        let lang = &self.language_id;

        settings_container(pal)
            .child(settings_card(pal)
                .child(settings_section_header(IconName::Speaker, &self.chat_type_name, Some(tr_str(lang, "对话类型通知设置", "Chat type notifications")), pal))
                .child(settings_toggle_row(IconName::Speaker, tr_str(lang, "弹出通知提醒", "Show Notifications"), None, self.alerts_enabled, pal, Self::toggle_alerts, cx))
                .child(settings_toggle_row(IconName::FileDoc, tr_str(lang, "显示消息预览", "Message Preview"), None, self.message_preview, pal, Self::toggle_preview, cx))
                .child(settings_toggle_row(IconName::Speaker, tr_str(lang, "播放提示音", "Play Sound"), None, self.sound_enabled, pal, Self::toggle_sound, cx)))
            .child(settings_card(pal)
                .child(settings_subsection_title(&format!("{} ({})", tr_str(lang, "例外列表", "Exceptions"), self.exceptions.len()), pal))
                .map(|el| {
                    if self.exceptions.is_empty() {
                        el.child(settings_empty_state(IconName::Mute, tr_str(lang, "暂无例外对话", "No notification exceptions"), pal))
                    } else {
                        el.children(self.exceptions.clone().into_iter().map(|ex| {
                            let pid = ex.peer_id;
                            let status = if ex.is_muted { "Muted" } else { "Custom" };
                            settings_action_row(IconName::ContactsUsers, &ex.name, status, pal, false, move |this| {
                                this.remove_exception(pid);
                            }, cx)
                        }))
                    }
                }))
            .into_any_element()
    }
}
