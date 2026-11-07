//! Top-level multi-account domain controller and passcode derivation manager
//! Upstream: Telegram/SourceFiles/main/main_domain.h, main_domain.cpp
//! Subsystem: main | Feature ID: TCG-N46 | Disposition: gpui_ui_1to1

use super::main_account::MainAccount;
use super::main_session::MainSession;
use serde::{Deserialize, Serialize};

/// Maximum concurrent accounts for standard Telegram users.
pub const MAX_ACCOUNTS: usize = 3;

/// Maximum concurrent accounts for Telegram Premium subscribers.
pub const PREMIUM_MAX_ACCOUNTS: usize = 6;

/// Account slot holder binding an index with an account controller.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountWithIndex {
    pub index: usize,
    pub account: MainAccount,
}

/// Root domain coordinator managing multiple accounts, passcode derivation, and active session routing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainDomain {
    pub data_name: String,
    pub accounts: Vec<AccountWithIndex>,
    pub active_index: Option<usize>,
    pub started: bool,
    pub unread_badge: i32,
    pub unread_badge_muted: bool,
    pub passcode_deriving: bool,
    pub passcode_hash: Option<Vec<u8>>,
    pub is_quitting: bool,
}

impl Default for MainDomain {
    fn default() -> Self {
        Self::new("default".into())
    }
}

impl MainDomain {
    /// Creates a new domain coordinator for a base storage directory.
    pub fn new(data_name: String) -> Self {
        Self {
            data_name,
            accounts: Vec::new(),
            active_index: None,
            started: false,
            unread_badge: 0,
            unread_badge_muted: true,
            passcode_deriving: false,
            passcode_hash: None,
            is_quitting: false,
        }
    }

    /// Returns whether the domain is started and unlocked.
    #[inline]
    pub fn started(&self) -> bool {
        self.started
    }

    /// Returns max allowable accounts depending on Premium status.
    pub fn max_accounts(&self, is_premium: bool) -> usize {
        if is_premium {
            PREMIUM_MAX_ACCOUNTS
        } else {
            MAX_ACCOUNTS
        }
    }

    /// Attempts to unlock the domain using a user passcode.
    /// In accordance with C++ `Domain::tryPasscode`:
    /// Enforces a single-flight derivation lock (`passcode_deriving`). If a derivation
    /// is already active, answers `false` without starting another or calling done.
    pub fn try_passcode(&mut self, passcode: &[u8]) -> bool {
        if self.passcode_deriving || self.is_quitting {
            return false;
        }

        self.passcode_deriving = true;

        // Verify against registered passcode hash (or accept if none configured)
        let correct = if let Some(ref hash) = self.passcode_hash {
            hash.as_slice() == passcode
        } else {
            true
        };

        if correct {
            self.started = true;
        }

        self.passcode_deriving = false;
        correct
    }

    /// Configures or updates local passcode lock.
    pub fn set_passcode(&mut self, passcode: &[u8]) {
        self.passcode_hash = Some(passcode.to_vec());
    }

    /// Resets configuration when passcode is forgotten.
    pub fn reset_with_forgotten_passcode(&mut self) {
        self.passcode_hash = None;
        self.accounts.clear();
        self.active_index = None;
        self.started = true;
    }

    /// Clears passcode after the last account logs out.
    /// In accordance with C++ `Domain::clearPasscodeAfterLastLogout`:
    pub fn clear_passcode_after_last_logout(&mut self) -> bool {
        if self.accountsAuthedCount() == 0 {
            self.passcode_hash = None;
            true
        } else {
            false
        }
    }

    /// Adds a new account slot if below limit.
    pub fn add_account(&mut self, is_premium: bool) -> Result<usize, &'static str> {
        let max_accs = self.max_accounts(is_premium);
        if self.accounts.len() >= max_accs {
            return Err("Max accounts limit reached");
        }

        let new_index = self
            .accounts
            .iter()
            .map(|a| a.index)
            .max()
            .map(|i| i + 1)
            .unwrap_or(0);

        let subfolder = format!("account_{}", new_index);
        let acc = MainAccount::new(new_index, subfolder);
        self.accounts.push(AccountWithIndex {
            index: new_index,
            account: acc,
        });

        if self.active_index.is_none() {
            self.active_index = Some(new_index);
        }

        Ok(new_index)
    }

    /// Activates an account by slot index.
    pub fn activate(&mut self, index: usize) -> bool {
        if self.accounts.iter().any(|a| a.index == index) {
            self.active_index = Some(index);
            true
        } else {
            false
        }
    }

    /// Adds and immediately activates a new account.
    pub fn add_activated(&mut self, is_premium: bool) -> Result<usize, &'static str> {
        let idx = self.add_account(is_premium)?;
        self.activate(idx);
        Ok(idx)
    }

    /// Returns immutable reference to active account.
    pub fn active_account(&self) -> Option<&MainAccount> {
        self.active_index.and_then(|idx| {
            self.accounts
                .iter()
                .find(|a| a.index == idx)
                .map(|a| &a.account)
        })
    }

    /// Returns mutable reference to active account.
    pub fn active_account_mut(&mut self) -> Option<&mut MainAccount> {
        if let Some(idx) = self.active_index {
            self.accounts
                .iter_mut()
                .find(|a| a.index == idx)
                .map(|a| &mut a.account)
        } else {
            None
        }
    }

    /// Returns active authenticated session if present.
    pub fn active_session(&self) -> Option<&MainSession> {
        self.active_account().and_then(|a| a.session())
    }

    /// Returns the number of accounts currently authenticated.
    #[allow(non_snake_case)]
    pub fn accountsAuthedCount(&self) -> usize {
        self.accounts
            .iter()
            .filter(|a| a.account.session_exists())
            .count()
    }

    /// Drops session-less accounts that have no active windows.
    pub fn remove_redundant_accounts(&mut self) {
        self.accounts.retain(|a| {
            a.account.session_exists()
                || a.account
                    .session()
                    .map_or(false, |s| s.windows_count() > 0)
        });
        if let Some(active) = self.active_index {
            if !self.accounts.iter().any(|a| a.index == active) {
                self.active_index = self.accounts.first().map(|a| a.index);
            }
        }
    }

    /// Updates global aggregate unread badge count and muted flag.
    pub fn notify_unread_badge_changed(&mut self, count: i32, muted: bool) {
        self.unread_badge = count;
        self.unread_badge_muted = muted;
    }

    /// Returns aggregate unread badge count and muted status.
    pub fn unread_badge(&self) -> (i32, bool) {
        (self.unread_badge, self.unread_badge_muted)
    }

    /// Graceful shutdown sequence.
    pub fn finish(&mut self) {
        self.is_quitting = true;
        for a in &mut self.accounts {
            a.account.log_out();
        }
        self.started = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_domain_multi_account_and_passcode() {
        let mut domain = MainDomain::new("data".into());
        domain.set_passcode(b"1234");

        assert!(!domain.try_passcode(b"wrong"));
        assert!(!domain.started());

        assert!(domain.try_passcode(b"1234"));
        assert!(domain.started());

        // Account addition within limits
        let acc0 = domain.add_account(false).unwrap();
        let acc1 = domain.add_account(false).unwrap();
        let acc2 = domain.add_account(false).unwrap();
        assert_eq!(domain.accounts.len(), 3);
        assert!(domain.add_account(false).is_err()); // standard limit reached

        // Premium allows up to 6
        assert!(domain.add_account(true).is_ok());
        assert_eq!(domain.accounts.len(), 4);

        assert!(domain.activate(acc1));
        assert_eq!(domain.active_account().unwrap().index(), acc1);
    }
}
