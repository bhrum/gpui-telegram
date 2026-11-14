//! Upstream Source: Telegram/SourceFiles/settings/sections/settings_main.cpp

use crate::i18n::tr_str;
use crate::theme::assets::{IconName, tg_icon};
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::settings::settings_theme_ui::{
    settings_action_row, settings_card, settings_container, settings_section_header,
    settings_subsection_title,
};
use crate::views::settings::settings_type::SettingsSectionType;
use gpui::prelude::FluentBuilder;
use gpui::*;

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
    pub palette: TdesktopThemePalette,
    pub language_id: String,
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
            palette: TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x5288C1, 14, 12),
            language_id: "en".into(),
        }
    }

    pub fn new_live() -> Self {
        let mut s = Self::new();
        s.profile = SettingsMainProfile {
            name: String::new(),
            phone: String::new(),
            username: String::new(),
            bio: String::new(),
            is_verified: false,
            is_premium: false,
        };
        s
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) {
        self.palette = palette;
    }

    pub fn set_language(&mut self, lang: impl Into<String>) {
        self.language_id = lang.into();
    }

    pub fn sync_profile(&mut self, name: String, phone: String, username: String, bio: String, is_verified: bool, is_premium: bool) {
        self.profile = SettingsMainProfile { name, phone, username, bio, is_verified, is_premium };
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
        let pal = self.palette;
        let lang = &self.language_id;
        let title = if self.profile.name.is_empty() {
            tr_str(lang, "我的账号", "My Account")
        } else {
            &self.profile.name
        };

        let links = [
            (IconName::ContactsUsers, tr_str(lang, "编辑资料", "Edit Profile"), SettingsSectionType::Information),
            (IconName::Speaker, tr_str(lang, "通知与声音", "Notifications and Sounds"), SettingsSectionType::Notifications),
            (IconName::Lock, tr_str(lang, "隐私与安全", "Privacy and Security"), SettingsSectionType::PrivacySecurity),
            (IconName::NightMoon, tr_str(lang, "聊天设置", "Chat Settings"), SettingsSectionType::Chat),
            (IconName::FolderAll, tr_str(lang, "聊天分组", "Folders"), SettingsSectionType::Folders),
            (IconName::SettingsGear, tr_str(lang, "高级设置", "Advanced"), SettingsSectionType::Advanced),
            (IconName::PhoneCall, tr_str(lang, "音视频通话", "Speakers and Camera"), SettingsSectionType::Calls),
            (IconName::PremiumStar, "Telegram Premium", SettingsSectionType::Premium),
            (IconName::WalletStars, "Telegram Stars", SettingsSectionType::Credits),
            (IconName::Translate, tr_str(lang, "语言", "Language"), SettingsSectionType::Language),
        ];

        settings_container(pal)
            .child(settings_card(pal)
                .child(settings_section_header(IconName::ContactsUsers, title, Some(&self.profile.bio), pal))
                .when(!self.profile.phone.is_empty(), |el| {
                    el.child(settings_action_row(IconName::PhoneCall, &self.profile.phone, tr_str(lang, "复制", "Copy"), pal, false, |this| {
                        this.copy_phone();
                    }, cx))
                })
                .when(!self.profile.username.is_empty(), |el| {
                    el.child(settings_action_row(IconName::Copy, &format!("@{}", self.profile.username), tr_str(lang, "复制", "Copy"), pal, false, |this| {
                        this.copy_username();
                    }, cx))
                })
                .when_some(self.toast_message.clone(), |el, msg| {
                    el.child(div().text_size(px(12.0)).text_color(rgb(pal.success)).child(msg))
                }))
            .child(settings_card(pal)
                .child(settings_subsection_title(tr_str(lang, "设置分类", "Sections"), pal))
                .children(links.into_iter().map(|(icon, label, sec)| {
                    settings_action_row(icon, label, "", pal, false, move |this| {
                        this.navigate_to(sec);
                    }, cx)
                })))
            .child(settings_card(pal)
                .child(div().flex().items_center().justify_between()
                    .child(div().flex().items_center().gap(px(8.0)).child(tg_icon(IconName::SettingsGear, 15.0, pal.icon_fg))
                        .child(div().text_size(px(13.0)).text_color(rgb(pal.text_primary)).child(tr_str(lang, "界面缩放", "Interface Scale"))))
                    .child(div().flex().gap(px(6.0)).children([100u32, 125, 150, 200].into_iter().map(|pct| {
                        let act = self.interface_scale_pct == pct;
                        div().cursor_pointer().px(px(8.0)).py(px(4.0)).rounded(px(6.0))
                            .bg(rgb(if act { pal.accent } else { pal.input_bg })).text_size(px(11.5))
                            .text_color(rgb(if act { pal.active_button_fg } else { pal.text_primary }))
                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _e, _w, cx| {
                                this.set_interface_scale(pct);
                                cx.notify();
                            }))
                            .child(format!("{}%", pct))
                    })))))
            .into_any_element()
    }
}
