//! Upstream Source: Telegram/SourceFiles/settings/sections/settings_websites.cpp

use crate::i18n::tr_str;
use crate::theme::assets::IconName;
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::settings::settings_theme_ui::{
    settings_action_row, settings_card, settings_container, settings_empty_state,
    settings_section_header,
};
use gpui::prelude::FluentBuilder;
use gpui::*;

pub const K_SHORT_POLL_TIMEOUT_MS: u64 = 60_000;

#[derive(Debug, Clone)]
pub struct ConnectedWebsite {
    pub id: u64,
    pub domain: String,
    pub bot_name: String,
    pub browser: String,
    pub ip: String,
    pub location: String,
}

#[derive(Debug, Clone)]
pub struct SettingsWebsitesSections {
    pub websites: Vec<ConnectedWebsite>,
    pub poll_timeout_ms: u64,
    pub has_divider: bool,
    pub is_visible: bool,
    pub pending_sync: bool,
    pub status_message: Option<String>,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
}

impl SettingsWebsitesSections {
    pub fn new() -> Self {
        Self {
            websites: vec![
                ConnectedWebsite { id: 501, domain: "fragment.com".into(), bot_name: "Fragment Auth Bot".into(), browser: "Safari / macOS".into(), ip: "192.168.1.50".into(), location: "United States".into() },
                ConnectedWebsite { id: 502, domain: "telegram.org".into(), bot_name: "Telegram Login".into(), browser: "Chrome / Windows".into(), ip: "10.0.0.5".into(), location: "United Kingdom".into() },
            ],
            poll_timeout_ms: K_SHORT_POLL_TIMEOUT_MS,
            has_divider: true,
            is_visible: true,
            pending_sync: false,
            status_message: None,
            palette: TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x5288C1, 14, 12),
            language_id: "en".into(),
        }
    }

    pub fn new_live() -> Self {
        let mut s = Self::new();
        s.websites.clear();
        s
    }

    pub fn sync_from_domains(&mut self, domains: &[String]) {
        self.websites = domains
            .iter()
            .enumerate()
            .map(|(idx, domain)| ConnectedWebsite {
                id: 501 + idx as u64,
                domain: domain.clone(),
                bot_name: "Telegram Login".into(),
                browser: "GPUI Desktop / macOS".into(),
                ip: "127.0.0.1".into(),
                location: "Local".into(),
            })
            .collect();
    }

    pub fn connected_domains(&self) -> Vec<String> {
        self.websites.iter().map(|w| w.domain.clone()).collect()
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) { self.palette = palette; }
    pub fn set_language(&mut self, lang: impl Into<String>) { self.language_id = lang.into(); }
    pub fn websites_id(&self) -> &'static str { "settings_websites" }
    pub fn toggle_visibility(&mut self) { self.is_visible = !self.is_visible; }
    pub fn margins(&self) -> (f32, f32, f32, f32) { (22.0, 10.0, 20.0, 5.0) }

    pub fn disconnect_all(&mut self) -> usize {
        let count = self.websites.len();
        self.websites.clear();
        if count > 0 {
            self.pending_sync = true;
            self.status_message = Some(
                tr_str(&self.language_id, "已断开所有网站登录授权", "Disconnected all websites")
                    .to_string(),
            );
        }
        count
    }

    pub fn disconnect(&mut self, id: u64) -> bool {
        let initial_len = self.websites.len();
        self.websites.retain(|w| w.id != id);
        let removed = self.websites.len() < initial_len;
        if removed {
            self.pending_sync = true;
            self.status_message = Some(
                tr_str(&self.language_id, "已断开网站授权", "Website disconnected").to_string(),
            );
        }
        removed
    }

    pub fn remove(&mut self, id: u64) { self.disconnect(id); }

    pub fn update(&mut self, site: ConnectedWebsite) {
        if let Some(existing) = self.websites.iter_mut().find(|w| w.id == site.id) {
            *existing = site;
        } else { self.websites.push(site); }
        self.pending_sync = true;
    }

    pub fn authorize_domain(&mut self, domain: &str) {
        let trimmed = domain.trim();
        if trimmed.is_empty() || self.websites.iter().any(|w| w.domain == trimmed) {
            return;
        }
        let next_id = self.websites.iter().map(|w| w.id).max().unwrap_or(500) + 1;
        self.websites.push(ConnectedWebsite {
            id: next_id,
            domain: trimmed.to_string(),
            bot_name: "Telegram Login".into(),
            browser: "GPUI Desktop / macOS".into(),
            ip: "127.0.0.1".into(),
            location: "Local".into(),
        });
        self.pending_sync = true;
        self.status_message = Some(if self.language_id.starts_with("zh") {
            format!("已授权登录网站: {}", trimmed)
        } else {
            format!("Authorized website: {}", trimmed)
        });
    }

    pub fn add_divider(&mut self) { self.has_divider = true; }
    pub fn lng_sessions_ip(&self, ip: &str) -> String { format!("IP: {}", ip) }
    pub fn lng_settings_disconnect(&mut self, id: u64) { self.disconnect(id); }
}

impl Default for SettingsWebsitesSections {
    fn default() -> Self { Self::new() }
}

impl Render for SettingsWebsitesSections {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible { return div().into_any_element(); }
        let pal = self.palette;
        let lang = &self.language_id;

        settings_container(pal)
            .child(settings_card(pal)
                .child(settings_section_header(IconName::BotRobot, tr_str(lang, "已授权网站", "Connected Websites"), Some(tr_str(lang, "管理通过 Telegram 登录小组件授权的第三方网站", "Manage websites logged in via Telegram Login Widget")), pal))
                .child(settings_action_row(IconName::Verified, tr_str(lang, "添加 telegram.org 登录授权", "Authorize telegram.org Login"), "+", pal, false, |this| {
                    if this.websites.iter().any(|w| w.domain == "telegram.org") {
                        this.authorize_domain("fragment.com");
                    } else {
                        this.authorize_domain("telegram.org");
                    }
                }, cx))
                .when(!self.websites.is_empty(), |el| {
                    el.child(settings_action_row(IconName::DeleteTrash, tr_str(lang, "全部断开授权", "Disconnect All"), "", pal, true, |this| {
                        this.disconnect_all();
                    }, cx))
                })
                .when_some(self.status_message.clone(), |el, msg| {
                    el.child(div().text_size(px(pal.scaled_px(12.0))).text_color(rgb(pal.success)).child(msg))
                })
                .map(|el| {
                    if self.websites.is_empty() {
                        el.child(settings_empty_state(IconName::BotRobot, tr_str(lang, "暂无已登录网站", "No connected websites"), pal))
                    } else {
                        el.children(self.websites.clone().into_iter().map(|w| {
                            let wid = w.id;
                            settings_action_row(IconName::BotRobot, &format!("{} ({})", w.domain, w.browser), tr_str(lang, "断开", "Disconnect"), pal, true, move |this| {
                                this.disconnect(wid);
                            }, cx)
                        }))
                    }
                }))
            .into_any_element()
    }
}
