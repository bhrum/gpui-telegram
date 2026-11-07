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
pub mod session;

pub use main::Main;
pub use main_account::MainAccount;
pub use main_app_config::MainAppConfig;
pub use main_app_config_values::MainAppConfigValues;
pub use main_domain::MainDomain;
pub use main_session::MainSession;
pub use main_session_settings::MainSessionSettings;
pub use mainwidget::Mainwidget;
pub use mainwindow::Mainwindow;

pub struct TelegramApp;
impl TelegramApp {
    pub fn new() -> Self { Self }
}
impl Default for TelegramApp {
    fn default() -> Self { Self::new() }
}
