//! Account controller managing networking, auth keys, AppConfig, and active session
//! Upstream: Telegram/SourceFiles/main/main_account.h, main_account.cpp
//! Subsystem: main | Feature ID: TCG-N46 | Disposition: gpui_ui_1to1

use super::main_app_config::MainAppConfig;
use super::main_session::MainSession;
use super::main_session_settings::MainSessionSettings;
use serde::{Deserialize, Serialize};

/// Account lifecycle and protocol controller bound to a storage index and subfolder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainAccount {
    pub index: usize,
    pub data_name: String,
    pub main_dc_id: i32,
    pub session_user_id: Option<u64>,
    pub auth_key: Option<Vec<u8>>,
    pub app_config: MainAppConfig,
    pub session: Option<MainSession>,
    pub is_logging_out: bool,
    pub login_code_callback: Option<String>,
}

impl Default for MainAccount {
    fn default() -> Self {
        Self::new(0, "default".into())
    }
}

impl MainAccount {
    /// Constructs a new account with slot index and data storage subfolder name.
    pub fn new(index: usize, data_name: String) -> Self {
        Self {
            index,
            data_name,
            main_dc_id: 2, // Default Telegram DC 2 (Production)
            session_user_id: None,
            auth_key: None,
            app_config: MainAppConfig::new(),
            session: None,
            is_logging_out: false,
            login_code_callback: None,
        }
    }

    /// Slot index of the account (0..5).
    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }

    /// Data storage subfolder name.
    #[inline]
    pub fn data_name(&self) -> &str {
        &self.data_name
    }

    /// Main data center identifier (1..5).
    #[inline]
    pub fn main_dc_id(&self) -> i32 {
        self.main_dc_id
    }

    /// Updates main DC ID.
    pub fn set_main_dc_id(&mut self, dc_id: i32) {
        self.main_dc_id = dc_id;
    }

    /// Prepares MTProto configuration and associates auth key.
    pub fn prepare_to_start(&mut self, key: Vec<u8>) {
        self.auth_key = Some(key);
    }

    /// Creates and attaches an authenticated session.
    pub fn create_session(
        &mut self,
        user_id: u64,
        username: Option<String>,
        settings: Option<MainSessionSettings>,
    ) {
        self.session_user_id = Some(user_id);
        let mut s = MainSession::new(user_id, false);
        s.username = username;
        if let Some(cfg) = settings {
            s.settings = cfg;
        }
        self.session = Some(s);
        self.is_logging_out = false;
    }

    /// Sets session state restored from persistent storage.
    pub fn set_session_from_storage(
        &mut self,
        user_id: u64,
        settings: MainSessionSettings,
    ) {
        self.create_session(user_id, None, Some(settings));
    }

    /// Returns true if an authenticated session exists.
    #[inline]
    pub fn session_exists(&self) -> bool {
        self.session.is_some()
    }

    /// Returns immutable reference to the active session.
    pub fn session(&self) -> Option<&MainSession> {
        self.session.as_ref()
    }

    /// Returns mutable reference to the active session.
    pub fn session_mut(&mut self) -> Option<&mut MainSession> {
        self.session.as_mut()
    }

    /// Initiates graceful logout sequence.
    pub fn log_out(&mut self) {
        self.is_logging_out = true;
        if let Some(ref mut s) = self.session {
            s.finish_logout();
        }
        self.session = None;
        self.session_user_id = None;
        self.auth_key = None;
    }

    /// Forces immediate termination of active session.
    pub fn forced_log_out(&mut self) {
        self.log_out();
    }

    /// Returns true if logout is currently in progress.
    #[inline]
    pub fn logging_out(&self) -> bool {
        self.is_logging_out
    }

    /// Returns immutable reference to dynamic AppConfig.
    #[inline]
    pub fn app_config(&self) -> &MainAppConfig {
        &self.app_config
    }

    /// Returns mutable reference to dynamic AppConfig.
    #[inline]
    pub fn app_config_mut(&mut self) -> &mut MainAppConfig {
        &mut self.app_config
    }

    /// Dispatches a received login confirmation code.
    pub fn handle_login_code(&mut self, code: String) {
        self.login_code_callback = Some(code);
    }

    /// Returns the last handled login code.
    pub fn login_code(&self) -> Option<&str> {
        self.login_code_callback.as_deref()
    }

    /// Destroys stale authorization keys.
    pub fn destroy_stale_authorization_keys(&mut self) {
        if self.session.is_none() {
            self.auth_key = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_account_session_lifecycle() {
        let mut account = MainAccount::new(0, "account_0".into());
        assert!(!account.session_exists());

        account.create_session(987654, Some("alice".into()), None);
        assert!(account.session_exists());
        assert_eq!(account.session().unwrap().user_id(), 987654);

        account.log_out();
        assert!(!account.session_exists());
        assert!(account.logging_out());
    }
}
