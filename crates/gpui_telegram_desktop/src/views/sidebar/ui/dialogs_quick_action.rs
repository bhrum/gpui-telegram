//! Quick dialog action button / trigger.
//!
//! Upstream: `Telegram/SourceFiles/dialogs/ui/dialogs_quick_action.h:L1-23`
//! Subsystem: dialogs, Feature: TCG-N03

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuickDialogActionType {
    Disabled,
    Pin,
    Mute,
    Archive,
    Delete,
    MarkRead,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialogsQuickActionUi {
    pub action_type: QuickDialogActionType,
    pub is_hovered: bool,
}

impl Default for DialogsQuickActionUi {
    fn default() -> Self {
        Self {
            action_type: QuickDialogActionType::Disabled,
            is_hovered: false,
        }
    }
}

impl DialogsQuickActionUi {
    pub fn new() -> Self {
        Self::default()
    }
}
