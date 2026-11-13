//! Quick action handler for swipe / hover actions on dialog rows.
//!
//! Upstream:
//! - `Telegram/SourceFiles/dialogs/dialogs_quick_action.h:L1-L58`
//! - `Telegram/SourceFiles/dialogs/dialogs_quick_action.cpp:L1-L320`
//!
//! Subsystem: dialogs, Feature: TCG-N03
//!
//! Key Logic:
//! - Dynamic action label resolution based on thread status (`ResolveQuickDialogLabel` L192-231)
//! - Lottie animation key mapping (`ResolveQuickDialogLottieIconName` L165-190)
//! - Background color resolution (`ResolveQuickActionBg` L260-280)
//! - Font size reduction loop (`SwipeActionFont` L34-68, `kNormalFontSize = 13`, `kMinFontSize = 5`)

pub use gpui_telegram_client::dialogs::quick_action::{
    fit_swipe_action_font_size, resolve_quick_action_bg_hex, resolve_quick_dialog_label,
    resolve_quick_dialog_lottie_icon_name, QuickDialogAction, QuickDialogActionLabel,
    K_MIN_FONT_SIZE, K_NORMAL_FONT_SIZE,
};
use serde::{Deserialize, Serialize};

/// Quick swipe action component for a sidebar row.
/// Upstream: `Dialogs::QuickAction` (`dialogs_quick_action.h:L28-56`)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialogsQuickAction {
    pub peer_id: i64,
    pub action: QuickDialogAction,
    pub label: QuickDialogActionLabel,
    pub icon_lottie: String,
    pub bg_hex: String,
    pub is_triggered: bool,
}

impl DialogsQuickAction {
    pub fn new(peer_id: i64, action: QuickDialogAction) -> Self {
        let label = resolve_quick_dialog_label(
            action,
            false,
            false,
            false,
            false,
            false,
            true,
            false,
        );
        let icon_lottie = resolve_quick_dialog_lottie_icon_name(label).to_string();
        let bg_hex = resolve_quick_action_bg_hex(label).to_string();

        Self {
            peer_id,
            action,
            label,
            icon_lottie,
            bg_hex,
            is_triggered: false,
        }
    }

    pub fn update_state(
        &mut self,
        is_self: bool,
        is_muted: bool,
        is_pinned: bool,
        has_unread: bool,
        is_archived: bool,
        can_archive: bool,
        is_community_ungroup: bool,
    ) {
        self.label = resolve_quick_dialog_label(
            self.action,
            is_self,
            is_muted,
            is_pinned,
            has_unread,
            is_archived,
            can_archive,
            is_community_ungroup,
        );
        self.icon_lottie = resolve_quick_dialog_lottie_icon_name(self.label).to_string();
        self.bg_hex = resolve_quick_action_bg_hex(self.label).to_string();
    }
}
