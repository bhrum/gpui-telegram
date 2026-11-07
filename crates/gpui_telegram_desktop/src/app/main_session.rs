//! Authenticated Telegram session controller
//! Upstream: Telegram/SourceFiles/main/main_session.h, main_session.cpp
//! Subsystem: main | Feature ID: TCG-N02 | Disposition: gpui_ui_1to1

use super::main_session_settings::MainSessionSettings;
use super::session::send_as_peers::SendAsPeersSession;
use serde::{Deserialize, Serialize};

/// Account freeze info indicating temporary restriction details and appeal URL.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FreezeInfo {
    pub since: i64,
    pub until: i64,
    pub appeal_url: String,
}

impl FreezeInfo {
    #[inline]
    pub fn is_frozen(&self) -> bool {
        self.since != 0
    }
}

/// Terms of Service agreement lock.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TermsLock {
    pub id: String,
    pub text: String,
    pub min_age: i32,
    pub popup: bool,
}

/// Authenticated user session controller coordinating data, settings, and UI windows.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainSession {
    pub user_id: u64,
    pub test_mode: bool,
    pub username: Option<String>,
    pub is_premium: bool,
    pub freeze_info: FreezeInfo,
    pub terms_lock: Option<TermsLock>,
    pub tmp_password: Option<(Vec<u8>, i64)>, // (password_bytes, valid_until)
    pub settings: MainSessionSettings,
    pub send_as_peers: SendAsPeersSession,
    pub active_windows_count: usize,
    pub uploads_in_progress: usize,
}

impl Default for MainSession {
    fn default() -> Self {
        Self::new(0, false)
    }
}

impl MainSession {
    /// Test DC shift constant for isolating unique IDs in test environments.
    pub const TEST_DC_SHIFT: u64 = 0x4000_0000_0000_0000;

    /// Creates a new authenticated session for a user.
    pub fn new(user_id: u64, test_mode: bool) -> Self {
        Self {
            user_id,
            test_mode,
            username: None,
            is_premium: false,
            freeze_info: FreezeInfo::default(),
            terms_lock: None,
            tmp_password: None,
            settings: MainSessionSettings::new(),
            send_as_peers: SendAsPeersSession::new(user_id),
            active_windows_count: 1,
            uploads_in_progress: 0,
        }
    }

    /// Returns the Telegram user ID.
    #[inline]
    pub fn user_id(&self) -> u64 {
        self.user_id
    }

    /// Returns the unique session ID (shifted by test DC offset in test mode).
    #[inline]
    pub fn unique_id(&self) -> u64 {
        if self.test_mode {
            self.user_id | Self::TEST_DC_SHIFT
        } else {
            self.user_id
        }
    }

    /// Validates whether the active session matches a given user ID.
    pub fn validate_self(&self, id: u64) -> bool {
        self.user_id == id
    }

    /// Returns whether the session operates against Telegram test DCs.
    #[inline]
    pub fn is_test_mode(&self) -> bool {
        self.test_mode
    }

    /// Returns true if the account has an active Telegram Premium subscription.
    #[inline]
    pub fn is_premium(&self) -> bool {
        self.is_premium
    }

    /// Updates Telegram Premium status.
    pub fn set_premium(&mut self, premium: bool) {
        self.is_premium = premium;
    }

    /// Sets or updates account freeze info.
    pub fn set_freeze_info(&mut self, freeze: FreezeInfo) {
        self.freeze_info = freeze;
    }

    /// Returns true if the account is currently frozen.
    #[inline]
    pub fn is_frozen(&self) -> bool {
        self.freeze_info.is_frozen()
    }

    /// Locks the session behind a Terms of Service agreement.
    pub fn lock_by_terms(&mut self, lock: TermsLock) {
        self.terms_lock = Some(lock);
    }

    /// Unlocks terms after acceptance.
    pub fn unlock_terms(&mut self) {
        self.terms_lock = None;
    }

    /// Returns true if the session is locked awaiting Terms acceptance.
    #[inline]
    pub fn is_terms_locked(&self) -> bool {
        self.terms_lock.is_some()
    }

    /// Stores a temporary password with expiration timestamp.
    pub fn set_tmp_password(&mut self, password: Vec<u8>, valid_until: i64) {
        self.tmp_password = Some((password, valid_until));
    }

    /// Clears any cached temporary password.
    pub fn drop_tmp_password(&mut self) {
        self.tmp_password = None;
    }

    /// Retrieves the cached temporary password if it has not expired.
    pub fn valid_tmp_password(&self) -> Option<&[u8]> {
        let now = chrono::Utc::now().timestamp();
        if let Some((ref pwd, valid_until)) = self.tmp_password {
            if valid_until > now {
                return Some(pwd.as_slice());
            }
        }
        None
    }

    /// Registers a newly opened desktop window for this session.
    pub fn add_window(&mut self) {
        self.active_windows_count = self.active_windows_count.saturating_add(1);
    }

    /// Unregisters a closed window.
    pub fn remove_window(&mut self) {
        self.active_windows_count = self.active_windows_count.saturating_sub(1);
    }

    /// Returns the number of active windows bound to this session.
    #[inline]
    pub fn windows_count(&self) -> usize {
        self.active_windows_count
    }

    /// Increments the active upload count.
    pub fn start_upload(&mut self) {
        self.uploads_in_progress = self.uploads_in_progress.saturating_add(1);
    }

    /// Decrements the active upload count.
    pub fn finish_upload(&mut self) {
        self.uploads_in_progress = self.uploads_in_progress.saturating_sub(1);
    }

    /// Returns true if file or media uploads are currently in progress.
    #[inline]
    pub fn uploads_in_progress(&self) -> bool {
        self.uploads_in_progress > 0
    }

    /// Prepares session for graceful logout.
    pub fn finish_logout(&mut self) {
        self.drop_tmp_password();
        self.uploads_in_progress = 0;
        self.active_windows_count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_session_lifecycle() {
        let mut session = MainSession::new(12345678, false);
        assert_eq!(session.user_id(), 12345678);
        assert_eq!(session.unique_id(), 12345678);
        assert!(session.validate_self(12345678));
        assert!(!session.is_frozen());

        // Test mode shift
        let test_session = MainSession::new(12345678, true);
        assert_eq!(test_session.unique_id(), 12345678 | MainSession::TEST_DC_SHIFT);

        // Temporary password expiration
        let future = chrono::Utc::now().timestamp() + 3600;
        session.set_tmp_password(b"secret".to_vec(), future);
        assert_eq!(session.valid_tmp_password(), Some(b"secret".as_slice()));

        let past = chrono::Utc::now().timestamp() - 10;
        session.set_tmp_password(b"secret".to_vec(), past);
        assert_eq!(session.valid_tmp_password(), None);
    }
}
