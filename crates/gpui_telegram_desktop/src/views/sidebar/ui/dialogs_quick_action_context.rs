//! Quick dialog action execution context.
//!
//! Upstream: `Telegram/SourceFiles/dialogs/ui/dialogs_quick_action_context.h:L1-48`
//! Subsystem: dialogs, Feature: TCG-N03

use super::dialogs_quick_action::QuickDialogActionType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialogsQuickActionContextUi {
    pub peer_id: u64,
    pub action: QuickDialogActionType,
}

impl DialogsQuickActionContextUi {
    pub fn new(peer_id: u64, action: QuickDialogActionType) -> Self {
        Self { peer_id, action }
    }
}
