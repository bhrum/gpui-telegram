// Source: Telegram/SourceFiles/settings/sections/settings_privacy_security.cpp

use crate::i18n::tr_str;
use crate::theme::assets::IconName;
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::settings::settings_theme_ui::{
    settings_action_row, settings_card, settings_container, settings_info_row,
    settings_section_header, settings_toggle_row,
};
use gpui::*;

#[derive(Debug, Clone)]
pub struct SettingsPrivacySecuritySections {
    pub blocked_count: usize,
    pub passcode_enabled: bool,
    pub two_factor_enabled: bool,
    pub active_sessions_count: usize,
    pub ttl_months: u32,
    pub sensitive_content_enabled: bool,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
}

impl SettingsPrivacySecuritySections {
    pub fn new() -> Self {
        Self {
            blocked_count: 0,
            passcode_enabled: false,
            two_factor_enabled: true,
            active_sessions_count: 2,
            ttl_months: 6,
            sensitive_content_enabled: false,
            palette: TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x5288C1, 14, 12),
            language_id: "en".into(),
        }
    }

    pub fn new_live() -> Self {
        let mut s = Self::new();
        s.two_factor_enabled = false;
        s.active_sessions_count = 0;
        s
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) { self.palette = palette; }
    pub fn set_language(&mut self, lang: impl Into<String>) { self.language_id = lang.into(); }
    pub fn set_blocked_count(&mut self, count: usize) { self.blocked_count = count; }
    pub fn set_passcode_enabled(&mut self, enabled: bool) { self.passcode_enabled = enabled; }
    pub fn set_two_factor_enabled(&mut self, enabled: bool) { self.two_factor_enabled = enabled; }

    pub fn set_ttl_months(&mut self, months: u32) {
        self.ttl_months = match months {
            1 | 3 | 6 | 12 | 18 | 24 => months,
            _ => 6,
        };
    }

    pub fn toggle_sensitive_content(&mut self) {
        self.sensitive_content_enabled = !self.sensitive_content_enabled;
    }

    pub fn ttl_description(&self) -> String {
        match self.ttl_months {
            1 => "1 month".into(),
            3 => "3 months".into(),
            6 => "6 months".into(),
            12 => "1 year".into(),
            18 => "18 months".into(),
            24 => "2 years".into(),
            n => format!("{} months", n),
        }
    }
}

impl Default for SettingsPrivacySecuritySections {
    fn default() -> Self { Self::new() }
}

impl Render for SettingsPrivacySecuritySections {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette;
        let lang = &self.language_id;

        settings_container(pal)
            .child(settings_card(pal)
                .child(settings_section_header(IconName::Lock, tr_str(lang, "隐私与安全", "Privacy and Security"), None, pal))
                .child(settings_info_row(tr_str(lang, "已屏蔽用户", "Blocked Users"), &format!("{}", self.blocked_count), pal))
                .child(settings_toggle_row(IconName::Lock, tr_str(lang, "本地密码锁", "Local Passcode"), None, self.passcode_enabled, pal, |this| {
                    this.passcode_enabled = !this.passcode_enabled;
                }, cx))
                .child(settings_toggle_row(IconName::Verified, tr_str(lang, "两步验证密码", "Two-Step Verification"), None, self.two_factor_enabled, pal, |this| {
                    this.two_factor_enabled = !this.two_factor_enabled;
                }, cx))
                .child(settings_info_row(tr_str(lang, "活跃设备数量", "Active Sessions"), &format!("{}", self.active_sessions_count), pal))
                .child(settings_action_row(IconName::ClockSending, tr_str(lang, "账号自毁期限", "If Away For"), &self.ttl_description(), pal, false, |this| {
                    let next = match this.ttl_months { 1 => 3, 3 => 6, 6 => 12, 12 => 18, 18 => 24, _ => 1 };
                    this.set_ttl_months(next);
                }, cx))
                .child(settings_toggle_row(IconName::Close, tr_str(lang, "关闭敏感内容过滤", "Disable Filtering"), None, self.sensitive_content_enabled, pal, Self::toggle_sensitive_content, cx)))
            .into_any_element()
    }
}
