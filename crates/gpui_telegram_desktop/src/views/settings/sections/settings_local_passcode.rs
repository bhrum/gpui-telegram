//! Upstream Source: Telegram/SourceFiles/settings/sections/settings_local_passcode.cpp

use crate::i18n::tr_str;
use crate::theme::assets::IconName;
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::boxes::passcode_box::PasscodeBox;
use crate::views::settings::settings_theme_ui::{
    settings_action_row, settings_card, settings_container, settings_info_row,
    settings_section_header, settings_subsection_title,
};
use gpui::prelude::FluentBuilder;
use gpui::*;

pub const K_PASSCODE_SECTION_TIMEOUT_MS: u64 = 60_000;
pub const K_PASSCODE_COUNTDOWN_TICK_MS: u64 = 1_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoLockTimeout {
    Disabled,
    In1Min,
    In5Mins,
    In1Hour,
    In5Hours,
}

impl AutoLockTimeout {
    pub fn as_seconds(&self) -> u64 {
        match self {
            Self::Disabled => 0,
            Self::In1Min => 60,
            Self::In5Mins => 300,
            Self::In1Hour => 3600,
            Self::In5Hours => 18000,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Disabled => "Disabled",
            Self::In1Min => "In 1 minute",
            Self::In5Mins => "In 5 minutes",
            Self::In1Hour => "In 1 hour",
            Self::In5Hours => "In 5 hours",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PasscodeCountdown {
    pub remaining_seconds: u64,
    pub paused: bool,
}

impl PasscodeCountdown {
    pub fn new() -> Self { Self { remaining_seconds: K_PASSCODE_SECTION_TIMEOUT_MS / 1000, paused: false } }
    pub fn restart(&mut self) { self.remaining_seconds = K_PASSCODE_SECTION_TIMEOUT_MS / 1000; self.paused = false; }
    pub fn pause(&mut self) { self.paused = true; }
    pub fn tick(&mut self) -> bool {
        if self.paused || self.remaining_seconds == 0 { return false; }
        self.remaining_seconds = self.remaining_seconds.saturating_sub(1);
        self.remaining_seconds == 0
    }
    pub fn formatted_text(&self) -> String { format!("{}:{:02}", self.remaining_seconds / 60, self.remaining_seconds % 60) }
}

#[derive(Debug, Clone)]
pub struct SettingsLocalPasscodeSections {
    pub has_passcode: bool,
    pub passcode_hash: Option<String>,
    pub auto_lock: AutoLockTimeout,
    pub countdown: PasscodeCountdown,
    pub bad_tries: u32,
    pub is_locked: bool,
    pub is_visible: bool,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
}

impl SettingsLocalPasscodeSections {
    fn hash_passcode_digest(pass: &str) -> String {
        let digest = PasscodeBox::hash_passcode(pass, b"tg-local-pass-salt-1", b"tg-local-pass-salt-2");
        digest.iter().map(|b| format!("{:02x}", b)).collect()
    }

    pub fn new() -> Self {
        Self {
            has_passcode: true,
            passcode_hash: Some(Self::hash_passcode_digest("correct")),
            auto_lock: AutoLockTimeout::In1Hour,
            countdown: PasscodeCountdown::new(),
            bad_tries: 0,
            is_locked: false,
            is_visible: true,
            palette: TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x5288C1, 14, 12),
            language_id: "en".into(),
        }
    }

    pub fn new_live() -> Self {
        let mut s = Self::new();
        s.has_passcode = false;
        s.passcode_hash = None;
        s
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) { self.palette = palette; }
    pub fn set_language(&mut self, lang: impl Into<String>) { self.language_id = lang.into(); }
    pub fn local_passcode_create_id(&self) -> &'static str { "settings_local_passcode_create" }
    pub fn local_passcode_check_id(&self) -> &'static str { "settings_local_passcode_check" }
    pub fn local_passcode_manage_id(&self) -> &'static str { "settings_local_passcode_manage" }

    pub fn set_passcode(&mut self, new_pass: &str) -> bool {
        if new_pass.trim().is_empty() { return false; }
        self.passcode_hash = Some(Self::hash_passcode_digest(new_pass));
        self.has_passcode = true;
        self.bad_tries = 0;
        self.countdown.restart();
        true
    }

    pub fn verify_passcode(&mut self, pass: &str) -> bool {
        if let Some(ref current) = self.passcode_hash {
            if current == &Self::hash_passcode_digest(pass) {
                self.bad_tries = 0;
                self.is_locked = false;
                self.countdown.restart();
                return true;
            }
        }
        self.bad_tries += 1;
        false
    }

    pub fn remove_passcode(&mut self, pass: &str) -> bool {
        if self.verify_passcode(pass) {
            self.has_passcode = false;
            self.passcode_hash = None;
            self.countdown.pause();
            true
        } else { false }
    }

    pub fn set_auto_lock(&mut self, timeout: AutoLockTimeout) { self.auto_lock = timeout; }
    pub fn toggle_visibility(&mut self) { self.is_visible = !self.is_visible; }
}

impl Default for SettingsLocalPasscodeSections {
    fn default() -> Self { Self::new() }
}

impl Render for SettingsLocalPasscodeSections {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible { return div().into_any_element(); }
        let pal = self.palette;
        let lang = &self.language_id;

        settings_container(pal)
            .child(settings_card(pal)
                .child(settings_section_header(IconName::Lock, tr_str(lang, "本地锁定密码", "Passcode Lock"), None, pal))
                .child(settings_info_row(tr_str(lang, "密码状态", "Status"), if self.has_passcode { "Enabled" } else { "Disabled" }, pal))
                .child(settings_action_row(IconName::ClockSending, tr_str(lang, "自动锁定超时", "Auto-Lock"), self.auto_lock.display_name(), pal, false, |this| {
                    this.auto_lock = match this.auto_lock {
                        AutoLockTimeout::Disabled => AutoLockTimeout::In1Min,
                        AutoLockTimeout::In1Min => AutoLockTimeout::In5Mins,
                        AutoLockTimeout::In5Mins => AutoLockTimeout::In1Hour,
                        AutoLockTimeout::In1Hour => AutoLockTimeout::In5Hours,
                        AutoLockTimeout::In5Hours => AutoLockTimeout::Disabled,
                    };
                }, cx))
                .when(self.has_passcode, |el| {
                    el.child(settings_subsection_title(tr_str(lang, "锁屏控制", "Controls"), pal))
                        .child(settings_action_row(IconName::Lock, tr_str(lang, "立即锁定", "Lock Telegram"), &self.countdown.formatted_text(), pal, false, |this| {
                            this.is_locked = !this.is_locked;
                        }, cx))
                }))
            .into_any_element()
    }
}
