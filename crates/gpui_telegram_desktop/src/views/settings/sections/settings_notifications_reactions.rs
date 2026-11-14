// Source: Telegram/SourceFiles/settings/sections/settings_notifications_reactions.cpp

use crate::i18n::tr_str;
use crate::theme::assets::IconName;
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::settings::settings_theme_ui::{
    settings_action_row, settings_card, settings_container, settings_info_row,
    settings_section_header, settings_toggle_row,
};
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
    pub palette: TdesktopThemePalette,
    pub language_id: String,
}

impl SettingsNotificationsReactionsSections {
    pub fn new() -> Self {
        Self {
            notify_reactions: true,
            notify_stories: true,
            source: ReactionsNotifySource::All,
            sound_enabled: true,
            sound_name: "Default".into(),
            palette: TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x5288C1, 14, 12),
            language_id: "en".into(),
        }
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) { self.palette = palette; }
    pub fn set_language(&mut self, lang: impl Into<String>) { self.language_id = lang.into(); }
    pub fn toggle_notify_reactions(&mut self) { self.notify_reactions = !self.notify_reactions; }
    pub fn toggle_notify_stories(&mut self) { self.notify_stories = !self.notify_stories; }
    pub fn set_source(&mut self, source: ReactionsNotifySource) { self.source = source; }
    pub fn toggle_sound(&mut self) { self.sound_enabled = !self.sound_enabled; }
    pub fn set_sound_name(&mut self, name: impl Into<String>) { self.sound_name = name.into(); }
}

impl Default for SettingsNotificationsReactionsSections {
    fn default() -> Self { Self::new() }
}

impl Render for SettingsNotificationsReactionsSections {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette;
        let lang = &self.language_id;
        let src_label = match self.source {
            ReactionsNotifySource::All => tr_str(lang, "所有人", "Everyone"),
            ReactionsNotifySource::Contacts => tr_str(lang, "仅联系人", "Contacts Only"),
        };

        settings_container(pal)
            .child(settings_card(pal)
                .child(settings_section_header(IconName::HeartReaction, tr_str(lang, "回应通知", "Reactions Notifications"), None, pal))
                .child(settings_toggle_row(IconName::HeartReaction, tr_str(lang, "消息回应通知", "Notify About Reactions"), None, self.notify_reactions, pal, Self::toggle_notify_reactions, cx))
                .child(settings_toggle_row(IconName::Camera, tr_str(lang, "动态回应通知", "Notify About Stories Reactions"), None, self.notify_stories, pal, Self::toggle_notify_stories, cx))
                .child(settings_action_row(IconName::ContactsUsers, tr_str(lang, "接收范围", "From"), src_label, pal, false, |this| {
                    this.source = match this.source {
                        ReactionsNotifySource::All => ReactionsNotifySource::Contacts,
                        ReactionsNotifySource::Contacts => ReactionsNotifySource::All,
                    };
                }, cx))
                .child(settings_toggle_row(IconName::Speaker, tr_str(lang, "回应提示音", "Sound"), None, self.sound_enabled, pal, Self::toggle_sound, cx))
                .child(settings_info_row(tr_str(lang, "提示音名称", "Tone"), &self.sound_name, pal)))
            .into_any_element()
    }
}
