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
    pub pending_sync: bool,
    pub status_message: Option<String>,
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
            pending_sync: false,
            status_message: None,
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

    pub fn sync_from_names(&mut self, names: &[String]) {
        self.passkeys.clear();
        self.next_id = 1;
        for name in names {
            let entry = PasskeyEntry {
                id: self.next_id,
                name: name.clone(),
                software_emoji_id: Some(0x1F511),
                created_at: 1709000000,
                last_used_at: None,
            };
            self.next_id += 1;
            self.passkeys.push(entry);
        }
    }

    pub fn passkey_names(&self) -> Vec<String> {
        self.passkeys.iter().map(|p| p.name.clone()).collect()
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) { self.palette = palette; }
    pub fn set_language(&mut self, lang: impl Into<String>) { self.language_id = lang.into(); }
    pub fn passkeys_id(&self) -> &'static str { "settings_passkeys" }
    pub fn can_register(&self) -> bool { self.can_register }
    pub fn request_list(&self) -> &[PasskeyEntry] { &self.passkeys }

    pub fn add_passkey(&mut self, name: &str, emoji_id: Option<u64>) -> PasskeyEntry {
        let resolved_name = if name.trim().is_empty() {
            format!("Passkey #{}", self.next_id)
        } else {
            name.trim().to_string()
        };
        let entry = PasskeyEntry {
            id: self.next_id,
            name: resolved_name.clone(),
            software_emoji_id: emoji_id,
            created_at: 1709000000,
            last_used_at: None,
        };
        self.next_id += 1;
        self.passkeys.push(entry.clone());
        self.pending_sync = true;
        self.status_message = Some(if self.language_id.starts_with("zh") {
            format!("已注册通行密钥: {}", resolved_name)
        } else {
            format!("Registered passkey: {}", resolved_name)
        });
        entry
    }

    pub fn delete_passkey(&mut self, id: u64) -> bool {
        let initial_len = self.passkeys.len();
        self.passkeys.retain(|p| p.id != id);
        let removed = self.passkeys.len() < initial_len;
        if removed {
            self.pending_sync = true;
            self.status_message = Some(
                tr_str(&self.language_id, "已移除通行密钥", "Passkey removed").to_string(),
            );
        }
        removed
    }

    pub fn rename_passkey(&mut self, id: u64, new_name: &str) -> bool {
        if let Some(entry) = self.passkeys.iter_mut().find(|p| p.id == id) {
            entry.name = new_name.trim().to_string();
            self.pending_sync = true;
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
                .child(settings_section_header(IconName::Lock, tr_str(lang, "通行密钥 (Passkeys)", "Passkeys"), Some(tr_str(lang, "使用系统生物识别或硬件安全密钥无密码登录", "Sign in securely without passwords using biometrics or security keys")), pal))
                .child(settings_action_row(IconName::Lock, tr_str(lang, "注册新通行密钥", "Add Passkey"), "+", pal, false, |this| {
                    this.is_registering = true;
                    let idx = this.next_id;
                    let label = if this.language_id.starts_with("zh") {
                        format!("本机通行密钥 #{}", idx)
                    } else {
                        format!("Desktop Passkey #{}", idx)
                    };
                    this.add_passkey(&label, None);
                    this.is_registering = false;
                }, cx))
                .when_some(self.status_message.clone(), |el, msg| {
                    el.child(div().text_size(px(pal.scaled_px(12.0))).text_color(rgb(pal.success)).child(msg))
                }))
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
