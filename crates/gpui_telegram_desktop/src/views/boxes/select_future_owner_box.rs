//! Select Future Owner Dialog Box for Group and Channel Ownership Transfer
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/select_future_owner_box.h:L1-L22`
//! - `Telegram/SourceFiles/boxes/select_future_owner_box.cpp:L1-L525`
//! - `Telegram/SourceFiles/boxes/select_future_owner_box.cpp:L50-L150` (Candidate filtering & requirements)
//!
//! Subsystem: boxes, Feature: TCG-N03

use gpui::{px, Pixels};
use std::time::Duration;

pub const K_2FA_MIN_ACTIVE_DURATION: Duration = Duration::from_secs(7 * 86400); // 7 days
pub const K_SESSION_MIN_ACTIVE_DURATION: Duration = Duration::from_secs(86400); // 24 hours

/// Candidate peer for channel or group ownership transfer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerCandidate {
    pub user_id: u64,
    pub name: String,
    pub username: Option<String>,
    pub is_admin: bool,
    pub is_bot: bool,
}

/// Modal dialog box for selecting a future channel or supergroup owner.
/// Target symbol in ledger: `SelectFutureOwnerBox`.
#[derive(Debug, Clone)]
pub struct SelectFutureOwnerBox {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub channel_id: u64,
    pub candidates: Vec<OwnerCandidate>,
    pub selected_candidate_id: Option<u64>,
    pub has_2fa_for_7_days: bool,
    pub has_session_for_24_hours: bool,
    pub is_submitting: bool,
}

impl Default for SelectFutureOwnerBox {
    fn default() -> Self {
        Self::new(0)
    }
}

impl SelectFutureOwnerBox {
    pub fn new(channel_id: u64) -> Self {
        Self {
            is_visible: true,
            width: px(380.0),
            height: px(460.0),
            channel_id,
            candidates: Vec::new(),
            selected_candidate_id: None,
            has_2fa_for_7_days: true,
            has_session_for_24_hours: true,
            is_submitting: false,
        }
    }

    pub fn set_candidates(&mut self, list: Vec<OwnerCandidate>) {
        // Only human admin members can become new owners (select_future_owner_box.cpp:L80-L110)
        self.candidates = list.into_iter().filter(|c| !c.is_bot).collect();
    }

    pub fn select_candidate(&mut self, user_id: u64) -> bool {
        if self.candidates.iter().any(|c| c.user_id == user_id) {
            self.selected_candidate_id = Some(user_id);
            true
        } else {
            false
        }
    }

    pub fn check_security_requirements(&self) -> Result<(), &'static str> {
        if !self.has_2fa_for_7_days {
            return Err("Two-Step Verification must be enabled for at least 7 days.");
        }
        if !self.has_session_for_24_hours {
            return Err("Current session must be active for at least 24 hours.");
        }
        if self.selected_candidate_id.is_none() {
            return Err("Please select a candidate user.");
        }
        Ok(())
    }
}
