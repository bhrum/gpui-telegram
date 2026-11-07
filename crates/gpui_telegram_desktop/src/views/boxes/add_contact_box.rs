//! Add Contact and Peer Profile Creation Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/add_contact_box.h:L1-L282`
//! - `Telegram/SourceFiles/boxes/add_contact_box.cpp:L1-L1694`
//! - `Telegram/SourceFiles/boxes/add_contact_box.cpp:L574-L592` (`kMaxGroupChannelTitle`, `kMaxChannelDescription`)
//! - `Telegram/SourceFiles/boxes/add_contact_box.cpp:L1345-L1375` (`kMinUsernameLength`, `kUsernameCheckTimeout`)
//! - `Telegram/SourceFiles/boxes/add_contact_box.cpp:L1570-L1575` (`kMaxUserFirstLastName`)
//!
//! Subsystem: boxes, Feature: TCG-N03

use gpui::{px, Pixels};
use std::time::Duration;

pub const K_MAX_USER_FIRST_LAST_NAME: usize = 64;
pub const K_MAX_GROUP_CHANNEL_TITLE: usize = 128;
pub const K_MAX_CHANNEL_DESCRIPTION: usize = 255;
pub const K_MIN_USERNAME_LENGTH: usize = 5;
pub const K_USERNAME_CHECK_TIMEOUT: Duration = Duration::from_millis(200);

/// Validation status for new contact entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContactValidationError {
    EmptyFirstName,
    FirstNameTooLong,
    LastNameTooLong,
    InvalidPhoneNumber,
}

/// Modal dialog box for adding a new contact or editing an existing contact profile.
/// Target symbol in ledger: `AddContactBox`.
#[derive(Debug, Clone)]
pub struct AddContactBox {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub is_submitting: bool,
    pub error: Option<ContactValidationError>,
}

impl Default for AddContactBox {
    fn default() -> Self {
        Self::new()
    }
}

impl AddContactBox {
    pub fn new() -> Self {
        Self {
            is_visible: true,
            width: px(360.0),
            height: px(320.0),
            first_name: String::new(),
            last_name: String::new(),
            phone: String::new(),
            is_submitting: false,
            error: None,
        }
    }

    /// Sets contact fields with length boundary checks (add_contact_box.cpp:L1574-L1575).
    pub fn set_fields(&mut self, first_name: &str, last_name: &str, phone: &str) {
        self.first_name = first_name.chars().take(K_MAX_USER_FIRST_LAST_NAME).collect();
        self.last_name = last_name.chars().take(K_MAX_USER_FIRST_LAST_NAME).collect();
        self.phone = phone.trim().to_string();
        self.validate();
    }

    /// Validates required names and phone format (add_contact_box.cpp:L420-L450).
    pub fn validate(&mut self) -> bool {
        if self.first_name.trim().is_empty() {
            self.error = Some(ContactValidationError::EmptyFirstName);
            return false;
        }
        if self.first_name.len() > K_MAX_USER_FIRST_LAST_NAME {
            self.error = Some(ContactValidationError::FirstNameTooLong);
            return false;
        }
        if self.last_name.len() > K_MAX_USER_FIRST_LAST_NAME {
            self.error = Some(ContactValidationError::LastNameTooLong);
            return false;
        }
        let clean_phone: String = self.phone.chars().filter(|c| c.is_ascii_digit()).collect();
        if clean_phone.len() < 7 || clean_phone.len() > 15 {
            self.error = Some(ContactValidationError::InvalidPhoneNumber);
            return false;
        }
        self.error = None;
        true
    }

    pub fn can_submit(&self) -> bool {
        self.error.is_none() && !self.first_name.trim().is_empty() && !self.is_submitting
    }
}
