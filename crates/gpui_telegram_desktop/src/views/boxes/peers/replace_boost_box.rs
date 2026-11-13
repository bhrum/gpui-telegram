//! Channel Boost Replacement and Slot Transfer Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/replace_boost_box.h:L1-L98`
//! - `Telegram/SourceFiles/boxes/peers/replace_boost_box.cpp:L1-L1116`
//! - `replace_boost_box.cpp:L50-L180` (Boost slot reallocation)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

/// Single existing boost slot assignment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoostSlotItem {
    pub slot_id: i32,
    pub channel_id: u64,
    pub channel_title: String,
    pub expires_at_sec: i64,
    pub is_selected: bool,
}

/// Target symbol in ledger: `ReplaceBoostBoxPeers`.
#[derive(Debug, Clone)]
pub struct ReplaceBoostBoxPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub target_channel_id: u64,
    pub target_channel_title: String,
    pub available_slots: Vec<BoostSlotItem>,
    pub is_submitting: bool,
}

impl Default for ReplaceBoostBoxPeers {
    fn default() -> Self {
        Self::new(0, "Channel".to_string())
    }
}

impl ReplaceBoostBoxPeers {
    pub fn new(target_channel_id: u64, target_channel_title: String) -> Self {
        Self {
            is_visible: true,
            width: px(400.0),
            height: px(500.0),
            target_channel_id,
            target_channel_title,
            available_slots: Vec::new(),
            is_submitting: false,
        }
    }

    pub fn select_slot(&mut self, slot_id: i32) {
        for slot in &mut self.available_slots {
            slot.is_selected = slot.slot_id == slot_id;
        }
    }

    pub fn selected_slot(&self) -> Option<&BoostSlotItem> {
        self.available_slots.iter().find(|s| s.is_selected)
    }
}
