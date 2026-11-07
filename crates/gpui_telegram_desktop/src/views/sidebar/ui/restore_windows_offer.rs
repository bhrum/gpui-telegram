//! Restore closed windows banner offer.
//!
//! Upstream: `Telegram/SourceFiles/dialogs/ui/restore_windows_offer.cpp:L1-209`
//! Subsystem: dialogs, Feature: TCG-N03

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreWindowsOfferUi {
    pub is_available: bool,
    pub closed_window_count: usize,
}

impl RestoreWindowsOfferUi {
    pub fn new() -> Self {
        Self::default()
    }
}
