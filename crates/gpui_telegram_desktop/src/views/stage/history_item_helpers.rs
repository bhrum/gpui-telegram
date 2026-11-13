//! GPUI Telegram History Item Helpers Component
//!
//! Citations:
//! - `Telegram/SourceFiles/history/history_item_helpers.h:L48-L100` (PreparedServiceText, MediaCheckResult)
//! - `Telegram/SourceFiles/history/history_item_helpers.cpp:L949-L991` (FlagsFromMTP bitmask mapping)

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui_telegram_client::history::item::{flags_from_mtp, MessageFlags};

/// Upstream `history_item_helpers.h:L64-L70`: Media Check Results
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MediaCheckResult {
    #[default]
    Good,
    Unsupported,
    Empty,
    HasExpiredMediaTimeToLive,
    HasStoryMention,
}

/// Upstream `history_item_helpers.h:L48-L100`: Helper Utilities for Message Rendering
#[derive(Debug, Clone, Default)]
pub struct HistoryItemHelpers {
    pub last_checked_media_result: MediaCheckResult,
}

impl HistoryItemHelpers {
    pub fn new() -> Self {
        Self::default()
    }

    /// Upstream `history_item_helpers.cpp:L949-L991`: FlagsFromMTP wrapper
    pub fn compute_flags_from_mtp(&self, id: i64, mtp_flags: u64, local_flags: u64) -> MessageFlags {
        flags_from_mtp(id, mtp_flags, local_flags)
    }

    /// Upstream `history_item_helpers.h:L71-L72`: Check message media viability
    pub fn check_media_expiration(&mut self, is_outgoing: bool, has_unread_flag: bool, ttl_seconds: Option<i32>) -> MediaCheckResult {
        if !is_outgoing && !has_unread_flag && ttl_seconds.map(|s| s > 0).unwrap_or(false) {
            self.last_checked_media_result = MediaCheckResult::HasExpiredMediaTimeToLive;
        } else {
            self.last_checked_media_result = MediaCheckResult::Good;
        }
        self.last_checked_media_result
    }
}

impl Render for HistoryItemHelpers {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}
