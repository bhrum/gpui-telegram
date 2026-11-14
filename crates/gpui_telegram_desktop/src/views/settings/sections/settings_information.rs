//! Upstream Source: Telegram/SourceFiles/settings/sections/settings_information.cpp

use crate::i18n::tr_str;
use crate::theme::assets::IconName;
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::settings::settings_theme_ui::{
    settings_action_row, settings_card, settings_container, settings_info_row,
    settings_section_header, settings_subsection_title,
};
use gpui::*;

pub const K_SAVE_BIO_TIMEOUT_MS: u64 = 1000;
pub const K_STANDARD_BIO_LIMIT: usize = 70;
pub const K_PREMIUM_BIO_LIMIT: usize = 140;

#[derive(Debug, Clone)]
pub struct AccountEntry {
    pub account_id: u64,
    pub name: String,
    pub phone: String,
    pub unread_count: usize,
    pub is_active: bool,
}

#[derive(Debug, Clone)]
pub struct SettingsInformationSections {
    pub first_name: String,
    pub last_name: String,
    pub bio: String,
    pub username: String,
    pub phone: String,
    pub birthday: Option<String>,
    pub is_premium: bool,
    pub accounts: Vec<AccountEntry>,
    pub save_pending: bool,
    pub is_visible: bool,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
}

impl SettingsInformationSections {
    pub fn new() -> Self {
        Self {
            first_name: "Pavel".into(),
            last_name: "Durov".into(),
            bio: "Exploring GPUI and high-performance Rust interfaces".into(),
            username: "durov".into(),
            phone: "+1 234 567 8900".into(),
            birthday: Some("October 10".into()),
            is_premium: true,
            accounts: vec![
                AccountEntry { account_id: 1, name: "Pavel Durov".into(), phone: "+1 234 567 8900".into(), unread_count: 0, is_active: true },
                AccountEntry { account_id: 2, name: "Work Account".into(), phone: "+1 415 555 0138".into(), unread_count: 4, is_active: false },
            ],
            save_pending: false,
            is_visible: true,
            palette: TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x5288C1, 14, 12),
            language_id: "en".into(),
        }
    }

    pub fn new_live() -> Self {
        let mut s = Self::new();
        s.first_name.clear();
        s.last_name.clear();
        s.bio.clear();
        s.username.clear();
        s.phone.clear();
        s.birthday = None;
        s.is_premium = false;
        s.accounts.clear();
        s
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) {
        self.palette = palette;
    }

    pub fn set_language(&mut self, lang: impl Into<String>) {
        self.language_id = lang.into();
    }

    pub fn bio_limit(&self) -> usize {
        if self.is_premium { K_PREMIUM_BIO_LIMIT } else { K_STANDARD_BIO_LIMIT }
    }

    pub fn update_bio(&mut self, bio: &str) -> bool {
        if bio.len() > self.bio_limit() { false } else { self.set_bio(bio); true }
    }

    pub fn set_bio(&mut self, bio: impl Into<String>) {
        let b = bio.into();
        let limit = self.bio_limit();
        self.bio = b.chars().take(limit).collect();
        self.save_pending = true;
    }

    pub fn set_names(&mut self, first: impl Into<String>, last: impl Into<String>) {
        self.first_name = first.into();
        self.last_name = last.into();
        self.save_pending = true;
    }

    pub fn set_username(&mut self, username: impl Into<String>) {
        self.username = username.into().chars().filter(|c| c.is_alphanumeric() || *c == '_').collect();
        self.save_pending = true;
    }

    pub fn switch_account(&mut self, account_id: u64) {
        for acc in &mut self.accounts {
            acc.is_active = acc.account_id == account_id;
        }
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }
}

impl Default for SettingsInformationSections {
    fn default() -> Self { Self::new() }
}

impl Render for SettingsInformationSections {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible { return div().into_any_element(); }
        let pal = self.palette;
        let lang = &self.language_id;
        let full_name = format!("{} {}", self.first_name, self.last_name).trim().to_string();

        settings_container(pal)
            .child(settings_card(pal)
                .child(settings_section_header(IconName::ContactsUsers, tr_str(lang, "编辑个人资料", "Edit Profile"), None, pal))
                .child(settings_info_row(tr_str(lang, "姓名", "Name"), if full_name.is_empty() { "-" } else { &full_name }, pal))
                .child(settings_info_row(tr_str(lang, "用户名", "Username"), if self.username.is_empty() { "-" } else { &self.username }, pal))
                .child(settings_info_row(tr_str(lang, "手机号", "Phone"), if self.phone.is_empty() { "-" } else { &self.phone }, pal))
                .child(settings_info_row(&format!("Bio ({}/{})", self.bio.len(), self.bio_limit()), if self.bio.is_empty() { "-" } else { &self.bio }, pal)))
            .child(settings_card(pal)
                .child(settings_subsection_title(tr_str(lang, "已登录账号", "Accounts"), pal))
                .children(self.accounts.clone().into_iter().map(|acc| {
                    let id = acc.account_id;
                    settings_action_row(IconName::ContactsUsers, &acc.name, if acc.is_active { "Active" } else { &acc.phone }, pal, false, move |this| {
                        this.switch_account(id);
                    }, cx)
                })))
            .into_any_element()
    }
}
