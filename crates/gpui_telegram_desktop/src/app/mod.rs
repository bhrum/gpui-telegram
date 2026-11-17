//! Module definitions for `crates/gpui_telegram_desktop/src/app`

pub mod main;
pub mod main_account;
pub mod main_app_config;
pub mod main_app_config_values;
pub mod main_domain;
pub mod main_session;
pub mod main_session_settings;
pub mod mainwidget;
pub mod mainwindow;
pub mod preferences;
pub mod session;
pub mod updater;

pub use main::Main;
pub use main_account::MainAccount;
pub use main_app_config::MainAppConfig;
pub use main_app_config_values::MainAppConfigValues;
pub use main_domain::MainDomain;
pub use main_session::MainSession;
pub use main_session_settings::MainSessionSettings;
pub use mainwidget::Mainwidget;
pub use mainwindow::Mainwindow;
pub use preferences::{
    AppPreferences, AutoDownloadPreferences, ChatAppearancePreferences, LocalizationPreferences,
    NotificationPreferences, ThemeMode,
};
pub use updater::{
    compare_semver, is_newer_version, parse_github_release_json, parse_release_manifest,
    AvailableUpdateInfo, DesktopUpdateSnapshot, DesktopUpdateState, DesktopUpdater,
    PlatformAssetEntry, UpdateReleaseManifest, CURRENT_APP_VERSION, DEFAULT_GITHUB_LATEST_API_URL,
    DEFAULT_GITHUB_REPO, DEFAULT_MANIFEST_URL,
};


use crate::domain::DesktopDomainBridge;
use crate::layout::{compute_shell_column_metrics, ShellColumnMetrics};
use crate::ui::UiShellToolkit;
use crate::window::WindowSubsystemRuntime;

/// Top-level GPUI Telegram desktop application coordinator tying domain, preferences, layout, and window controllers together.
#[derive(Debug, Clone)]
pub struct TelegramApp {
    pub domain: MainDomain,
    pub domain_bridge: DesktopDomainBridge,
    pub config: MainAppConfig,
    pub preferences: AppPreferences,
    pub mainwidget: Mainwidget,
    pub mainwindow: Mainwindow,
    pub window_runtime: WindowSubsystemRuntime,
    pub ui_toolkit: UiShellToolkit,
}

impl TelegramApp {
    pub fn new() -> Self {
        let width = 1280;
        let height = 820;
        Self {
            domain: MainDomain::new("default".to_string()),
            domain_bridge: DesktopDomainBridge::new(),
            config: MainAppConfig::new(),
            preferences: AppPreferences::default(),
            mainwidget: Mainwidget::new(width, height),
            mainwindow: Mainwindow::new(width, height),
            window_runtime: WindowSubsystemRuntime::new(),
            ui_toolkit: UiShellToolkit::new(),
        }
    }

    /// Propagates window resize across `Mainwidget`, `Mainwindow`, and `WindowAdaptive`.
    pub fn sync_viewport_size(&mut self, width_px: f32, height_px: f32) {
        let w = width_px.max(360.0) as i32;
        let h = height_px.max(480.0) as i32;
        self.mainwidget.resize(w, h);
        self.mainwindow.resize(w, h);
        self.window_runtime.apply_viewport_width(width_px);
    }

    /// Computes responsive column widths for NavRail, ChatSidebar, Stage, and ContextInspector.
    pub fn column_metrics(&self, viewport_width_px: f32, inspector_open: bool, has_active_chat: bool) -> ShellColumnMetrics {
        compute_shell_column_metrics(viewport_width_px, inspector_open, has_active_chat)
    }

    /// Binds an authenticated user session into the domain and window controllers.
    pub fn bind_authenticated_session(
        &mut self,
        user_id: u64,
        full_name: impl Into<String>,
        phone: impl Into<String>,
    ) {
        let name = full_name.into();
        self.domain_bridge.bind_actor(user_id);
        self.domain.is_quitting = false;
        self.domain.accounts.clear();
        self.domain.active_index = None;
        if self.domain.add_activated(false).is_ok() {
            if let Some(acc) = self.domain.active_account_mut() {
                acc.create_session(user_id, Some(name.clone()), None);
            }
        }
        self.domain.started = true;
        self.mainwindow.setup_main();
        self.window_runtime.bind_session_user(user_id, name, phone);
    }

    /// Clears session-scoped desktop state upon logout or account switch.
    pub fn reset_session(&mut self) {
        self.domain_bridge.reset_session();
        self.domain.finish();
        self.domain.accounts.clear();
        self.domain.active_index = None;
        self.domain.is_quitting = false;
        self.mainwindow.setup_intro();
        self.window_runtime.clear_session_user();
    }
}

impl Default for TelegramApp {
    fn default() -> Self {
        Self::new()
    }
}
