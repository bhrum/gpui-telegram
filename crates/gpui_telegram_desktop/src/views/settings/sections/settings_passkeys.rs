//! Upstream Source: Telegram/SourceFiles/settings/sections/settings_passkeys.cpp

use crate::i18n::tr_str;
use crate::theme::assets::IconName;
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::settings::settings_theme_ui::{
    settings_action_row, settings_card, settings_container, settings_empty_state,
    settings_section_header, settings_subsection_title,
};
use gpui::prelude::FluentBuilder;
use gpui::*;

#[derive(Debug, Clone)]
pub struct PasskeyEntry {
    pub id: u64,
    pub name: String,
    pub software_emoji_id: Option<u64>,
    pub created_at: u64,
    pub last_used_at: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct SettingsPasskeysSections {
    pub passkeys: Vec<PasskeyEntry>,
    pub is_registering: bool,
    pub can_register: bool,
    pub is_visible: bool,
    next_id: u64,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
}

impl SettingsPasskeysSections {
    pub fn new() -> Self {
        Self {
            passkeys: vec![
                PasskeyEntry { id: 1, name: "MacBook Touch ID".into(), software_emoji_id: Some(0x1F4BB), created_at: 1700000000, last_used_at: Some(1705000000) },
                PasskeyEntry { id: 2, name: "YubiKey 5C NFC".into(), software_emoji_id: Some(0x1F511), created_at: 1701000000, last_used_at: None },
            ],
            is_registering: false,
            can_register: true,
            is_visible: true,
            next_id: 3,
            palette: TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x5288C1, 14, 12),
            language_id: "en".into(),
        }
    }

    pub fn new_live() -> Self {
        let mut s = Self::new();
        s.passkeys.clear();
        s.next_id = 1;
        s
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) { self.palette = palette; }
    pub fn set_language(&mut self, lang: impl Into<String>) { self.language_id = lang.into(); }
    pub fn passkeys_id(&self) -> &'static str { "settings_passkeys" }
    pub fn can_register(&self) -> bool { self.can_register }
    pub fn request_list(&self) -> &[PasskeyEntry] { &self.passkeys }

    pub fn add_passkey(&mut self, name: &str, emoji_id: Option<u64>) -> PasskeyEntry {
        let entry = PasskeyEntry {
            id: self.next_id,
            name: if name.trim().is_empty() { "New Passkey".into() } else { name.trim().into() },
            software_emoji_id: emoji_id,
            created_at: 1709000000,
            last_used_at: None,
        };
        self.next_id += 1;
        self.passkeys.push(entry.clone());
        entry
    }

    pub fn delete_passkey(&mut self, id: u64) -> bool {
        let initial_len = self.passkeys.len();
        self.passkeys.retain(|p| p.id != id);
        self.passkeys.len() < initial_len
    }

    pub fn rename_passkey(&mut self, id: u64, new_name: &str) -> bool {
        if let Some(entry) = self.passkeys.iter_mut().find(|p| p.id == id) {
            entry.name = new_name.trim().to_string();
            true
        } else { false }
    }

    pub fn toggle_visibility(&mut self) { self.is_visible = !self.is_visible; }
}

impl Default for SettingsPasskeysSections {
    fn default() -> Self { Self::new() }
}

impl Render for SettingsPasskeysSections {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible { return div().into_any_element(); }
        let pal = self.palette;
        let lang = &self.language_id;

        settings_container(pal)
            .child(settings_card(pal)
                .child(settings_section_header(IconName::Lock, tr_str(lang, "通行密钥 (Passkeys)", "Passkeys"), None, pal))
                .child(settings_action_row(IconName::Lock, tr_str(lang, "注册新通行密钥", "Add Passkey"), "+", pal, false, |this| {
                    this.is_registering = true;
                    this.add_passkey("Desktop Passkey", None);
                    this.is_registering = false;
                }, cx)))
            .child(settings_card(pal)
                .child(settings_subsection_title(tr_str(lang, "已绑定密钥", "Registered Passkeys"), pal))
                .map(|el| {
                    if self.passkeys.is_empty() {
                        el.child(settings_empty_state(IconName::Lock, tr_str(lang, "暂无通行密钥", "No passkeys registered"), pal))
                    } else {
                        el.children(self.passkeys.clone().into_iter().map(|pk| {
                            let id = pk.id;
                            settings_action_row(IconName::Verified, &pk.name, tr_str(lang, "删除", "Remove"), pal, true, move |this| {
                                this.delete_passkey(id);
                            }, cx)
                        }))
                    }
                }))
            .into_any_element()
    }
}
