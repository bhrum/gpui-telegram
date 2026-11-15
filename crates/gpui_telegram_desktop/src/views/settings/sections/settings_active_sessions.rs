//! Upstream Source: Telegram/SourceFiles/settings/sections/settings_active_sessions.cpp

use crate::i18n::tr_str;
use crate::theme::assets::IconName;
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::settings::settings_theme_ui::{
    settings_action_row, settings_card, settings_container, settings_empty_state,
    settings_section_header, settings_subsection_title,
};
use gpui::prelude::FluentBuilder;
use gpui::*;

pub const K_SHORT_POLL_TIMEOUT_MS: u64 = 60_000;
pub const K_MAX_DEVICE_MODEL_LENGTH: usize = 32;

#[derive(Debug, Clone)]
pub struct SessionRowData {
    pub hash: u64,
    pub app_name: String,
    pub device_model: String,
    pub platform: String,
    pub ip: String,
    pub location: String,
    pub is_current: bool,
}

#[derive(Debug, Clone)]
pub struct SettingsActiveSessionsSections {
    pub sessions: Vec<SessionRowData>,
    pub session_ids_by_hash: std::collections::HashMap<u64, String>,
    pub pending_terminate_session_id: Option<String>,
    pub pending_terminate_all_other: bool,
    pub poll_timeout_ms: u64,
    pub is_visible: bool,
    pub has_divider: bool,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
}

impl SettingsActiveSessionsSections {
    pub fn new() -> Self {
        Self {
            sessions: vec![
                SessionRowData {
                    hash: 1001,
                    app_name: "AI Chat Desktop 1.0".into(),
                    device_model: "MacBook Pro".into(),
                    platform: "macOS".into(),
                    ip: "192.168.1.100".into(),
                    location: "Current Session".into(),
                    is_current: true,
                },
                SessionRowData {
                    hash: 1002,
                    app_name: "Mobile Client 1.0".into(),
                    device_model: "iPhone 15 Pro".into(),
                    platform: "iOS".into(),
                    ip: "10.0.0.12".into(),
                    location: "San Francisco, USA".into(),
                    is_current: false,
                },
            ],
            session_ids_by_hash: std::collections::HashMap::new(),
            pending_terminate_session_id: None,
            pending_terminate_all_other: false,
            poll_timeout_ms: K_SHORT_POLL_TIMEOUT_MS,
            is_visible: true,
            has_divider: true,
            palette: TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x5288C1, 14, 12),
            language_id: "en".into(),
        }
    }

    pub fn new_live() -> Self {
        let mut s = Self::new();
        s.sessions.clear();
        s
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) { self.palette = palette; }
    pub fn set_language(&mut self, lang: impl Into<String>) { self.language_id = lang.into(); }
    pub fn toggle_visibility(&mut self) { self.is_visible = !self.is_visible; }
    pub fn margins(&self) -> (f32, f32, f32, f32) { (22.0, 10.0, 20.0, 5.0) }

    pub fn make_icon(&self, platform: &str) -> &'static str {
        match platform {
            "macOS" => "device_desktop_mac",
            "Windows" => "device_desktop_win",
            "Ubuntu" | "Linux" => "device_linux",
            "iOS" => "device_tablet_ios",
            _ => "device_other",
        }
    }

    pub fn add_session_info_row(&mut self, session: SessionRowData) {
        let mut truncated = session;
        if truncated.device_model.len() > K_MAX_DEVICE_MODEL_LENGTH {
            truncated.device_model.truncate(K_MAX_DEVICE_MODEL_LENGTH);
        }
        self.sessions.push(truncated);
    }

    pub fn terminate_session(&mut self, hash: u64) -> bool {
        let before = self.sessions.len();
        self.sessions.retain(|s| s.is_current || s.hash != hash);
        let removed = self.sessions.len() < before;
        if removed {
            if let Some(sid) = self.session_ids_by_hash.remove(&hash) {
                self.pending_terminate_session_id = Some(sid);
            }
        }
        removed
    }

    pub fn terminate_all_other(&mut self) {
        self.sessions.retain(|s| s.is_current);
        self.pending_terminate_all_other = true;
    }

    pub fn lng_settings_rename_device_title(&self) -> &'static str { "Rename Device" }
    pub fn add_divider(&mut self) { self.has_divider = true; }
    pub fn lng_settings_save(&mut self) {}
}

impl Default for SettingsActiveSessionsSections {
    fn default() -> Self { Self::new() }
}

impl Render for SettingsActiveSessionsSections {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible { return div().into_any_element(); }
        let pal = self.palette;
        let lang = &self.language_id;
        let has_others = self.sessions.iter().any(|s| !s.is_current);

        settings_container(pal)
            .child(settings_card(pal)
                .child(settings_section_header(IconName::ComputerDesktop, tr_str(lang, "活跃设备与会话", "Active Sessions"), Some(tr_str(lang, "当前登录的设备列表", "Devices logged into your account")), pal))
                .when(has_others, |el| {
                    el.child(settings_action_row(IconName::DeleteTrash, tr_str(lang, "注销所有其他设备", "Terminate All Other Sessions"), "", pal, true, Self::terminate_all_other, cx))
                }))
            .child(settings_card(pal)
                .child(settings_subsection_title(tr_str(lang, "设备清单", "Devices"), pal))
                .map(|el| {
                    if self.sessions.is_empty() {
                        el.child(settings_empty_state(IconName::ComputerDesktop, tr_str(lang, "暂无活跃设备记录", "No active sessions synced"), pal))
                    } else {
                        el.children(self.sessions.clone().into_iter().map(|s| {
                            let h = s.hash;
                            let title = format!("{} — {}", s.device_model, s.app_name);
                            let val = if s.is_current { tr_str(lang, "当前设备", "Current") } else { tr_str(lang, "注销", "Terminate") };
                            settings_action_row(IconName::ComputerDesktop, &title, val, pal, !s.is_current, move |this| {
                                if !s.is_current {
                                    this.terminate_session(h);
                                }
                            }, cx)
                        }))
                    }
                }))
            .into_any_element()
    }
}
