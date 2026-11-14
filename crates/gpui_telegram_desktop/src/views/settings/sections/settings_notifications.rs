// Source: Telegram/SourceFiles/settings/sections/settings_notifications.cpp

use crate::i18n::tr_str;
use crate::theme::assets::IconName;
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::settings::settings_theme_ui::{
    settings_action_row, settings_card, settings_container, settings_section_header,
    settings_subsection_title, settings_toggle_row,
};
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
    pub palette: TdesktopThemePalette,
    pub language_id: String,
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
            palette: TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x5288C1, 14, 12),
            language_id: "en".into(),
        }
    }

    pub fn new_live() -> Self {
        let mut s = Self::new();
        s.exceptions_count = 0;
        s
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) { self.palette = palette; }
    pub fn set_language(&mut self, lang: impl Into<String>) { self.language_id = lang.into(); }
    pub fn toggle_private_chats(&mut self) { self.private_chats_enabled = !self.private_chats_enabled; }
    pub fn toggle_groups(&mut self) { self.groups_enabled = !self.groups_enabled; }
    pub fn toggle_channels(&mut self) { self.channels_enabled = !self.channels_enabled; }
    pub fn toggle_badge_include_muted(&mut self) { self.include_muted_in_badge = !self.include_muted_in_badge; }
    pub fn set_badge_count_type(&mut self, count_type: BadgeCountType) { self.badge_count_type = count_type; }
    pub fn toggle_flash_taskbar(&mut self) { self.flash_taskbar_icon = !self.flash_taskbar_icon; }
    pub fn toggle_sound(&mut self) { self.sound_enabled = !self.sound_enabled; }
}

impl Default for SettingsNotificationsSections {
    fn default() -> Self { Self::new() }
}

impl Render for SettingsNotificationsSections {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette;
        let lang = &self.language_id;
        let badge_label = match self.badge_count_type {
            BadgeCountType::CountMessages => tr_str(lang, "未读消息数", "Unread Messages"),
            BadgeCountType::CountChats => tr_str(lang, "未读对话数", "Unread Chats"),
        };

        settings_container(pal)
            .child(settings_card(pal)
                .child(settings_section_header(IconName::Speaker, tr_str(lang, "通知与声音", "Notifications and Sounds"), None, pal))
                .child(settings_toggle_row(IconName::ContactsUsers, tr_str(lang, "私聊通知", "Private Chats"), None, self.private_chats_enabled, pal, Self::toggle_private_chats, cx))
                .child(settings_toggle_row(IconName::GroupUsers, tr_str(lang, "群组通知", "Groups"), None, self.groups_enabled, pal, Self::toggle_groups, cx))
                .child(settings_toggle_row(IconName::ChannelMegaphone, tr_str(lang, "频道通知", "Channels"), None, self.channels_enabled, pal, Self::toggle_channels, cx))
                .child(settings_toggle_row(IconName::Speaker, tr_str(lang, "提示音", "Play Sound"), None, self.sound_enabled, pal, Self::toggle_sound, cx)))
            .child(settings_card(pal)
                .child(settings_subsection_title(tr_str(lang, "角标与闪烁", "Badge Counter"), pal))
                .child(settings_toggle_row(IconName::Mute, tr_str(lang, "包含已静音对话", "Include Muted Chats"), None, self.include_muted_in_badge, pal, Self::toggle_badge_include_muted, cx))
                .child(settings_action_row(IconName::ArchivedChats, tr_str(lang, "角标计数方式", "Count Unread"), badge_label, pal, false, |this| {
                    this.badge_count_type = match this.badge_count_type {
                        BadgeCountType::CountMessages => BadgeCountType::CountChats,
                        BadgeCountType::CountChats => BadgeCountType::CountMessages,
                    };
                }, cx))
                .child(settings_toggle_row(IconName::SettingsGear, tr_str(lang, "任务栏闪烁提醒", "Flash Taskbar Icon"), None, self.flash_taskbar_icon, pal, Self::toggle_flash_taskbar, cx)))
            .into_any_element()
    }
}
