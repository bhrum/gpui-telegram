//! Common dialog structures, badges, and sort modes.
//!
//! Upstream:
//! - `Telegram/SourceFiles/dialogs/dialogs_common.h:L1-L171`
//!
//! Core Data Types:
//! - `Dialogs::SortMode` (`dialogs_common.h:L26-30`): Date, Name, Add
//! - `Dialogs::PositionChange` (`dialogs_common.h:L32-36`): from, to, height
//! - `Dialogs::UnreadState` (`dialogs_common.h:L38-80`): Full vector of unread message/chat/mark/reaction/mention counters
//! - `Dialogs::BadgesState` (`dialogs_common.h:L111-132`): Visual badge flags and counter value
//! - `Dialogs::CountInBadge` (`dialogs_common.h:L134-138`): Default, Chats, Messages
//! - `Dialogs::IncludeInBadge` (`dialogs_common.h:L140-145`): Default, Unmuted, All, UnmutedOrAll

pub use gpui_telegram_client::dialogs::entry::{
    BadgesState, CountInBadge, IncludeInBadge, UnreadState,
};
pub use gpui_telegram_client::dialogs::list::{PositionChange, SortMode};
use serde::{Deserialize, Serialize};

/// Common dialog state aggregator for sidebar view components.
/// Upstream: `dialogs_common.h:L22-170`
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialogsCommon {
    pub sort_mode: SortMode,
    pub unread_state: UnreadState,
    pub badges_state: BadgesState,
}

impl DialogsCommon {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_sort_mode(mut self, sort_mode: SortMode) -> Self {
        self.sort_mode = sort_mode;
        self
    }

    pub fn with_unread_state(mut self, unread_state: UnreadState) -> Self {
        self.unread_state = unread_state;
        self
    }
}
