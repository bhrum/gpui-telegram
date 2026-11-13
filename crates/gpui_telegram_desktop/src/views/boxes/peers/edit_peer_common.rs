//! Common Utilities and Constants for Peer Editing Dialogs
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/edit_peer_common.h:L1-L77`
//! - `edit_peer_common.h:L20-L75` (Constants and validation thresholds)
//!
//! Subsystem: boxes, Feature: TCG-N11

use std::time::Duration;

pub const K_MAX_GROUP_CHANNEL_TITLE: usize = 128;
pub const K_MAX_CHANNEL_DESCRIPTION: usize = 255;
pub const K_MIN_USERNAME_LENGTH: usize = 5;
pub const K_MAX_USERNAME_LENGTH: usize = 32;
pub const K_USERNAME_CHECK_TIMEOUT: Duration = Duration::from_millis(200);

/// Common validation and helper service for peer settings.
/// Target symbol in ledger: `EditPeerCommonPeers`.
#[derive(Debug, Clone, Default)]
pub struct EditPeerCommonPeers;

impl EditPeerCommonPeers {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_title(title: &str) -> Result<String, &'static str> {
        let trimmed = title.trim();
        if trimmed.is_empty() {
            Err("Title cannot be empty")
        } else if trimmed.chars().count() > K_MAX_GROUP_CHANNEL_TITLE {
            Err("Title is too long")
        } else {
            Ok(trimmed.to_string())
        }
    }

    pub fn validate_username(username: &str) -> Result<String, &'static str> {
        let trimmed = username.trim().trim_start_matches('@');
        let count = trimmed.chars().count();
        if count < K_MIN_USERNAME_LENGTH {
            Err("Username must be at least 5 characters")
        } else if count > K_MAX_USERNAME_LENGTH {
            Err("Username must not exceed 32 characters")
        } else if !trimmed.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            Err("Username may only contain a-z, 0-9 and underscores")
        } else {
            Ok(trimmed.to_string())
        }
    }
}
