//! Pill badge component for folder filters and unread badges.
//!
//! Upstream: `Telegram/SourceFiles/dialogs/ui/dialogs_pill.cpp:L1-80`
//! Subsystem: dialogs, Feature: TCG-N03

use gpui::{px, rgb, Pixels, Rgba};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialogsPillUi {
    pub label: String,
    pub is_active: bool,
    pub is_muted: bool,
    pub count: u32,
}

impl DialogsPillUi {
    pub const HEIGHT: Pixels = px(19.0);
    pub const PADDING: Pixels = px(5.0);

    pub fn new(label: String, count: u32) -> Self {
        Self {
            label,
            is_active: false,
            is_muted: false,
            count,
        }
    }

    pub fn bg_color(&self) -> Rgba {
        if self.is_muted {
            rgb(0xA2ABB2)
        } else {
            rgb(0x40A7E3)
        }
    }
}
