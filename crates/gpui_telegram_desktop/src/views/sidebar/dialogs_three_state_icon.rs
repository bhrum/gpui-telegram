//! Three-state icon representation (normal, hovered/over, active/pressed).
//!
//! Upstream:
//! - `Telegram/SourceFiles/dialogs/dialogs_three_state_icon.h:L1-L22`
//!
//! Subsystem: dialogs, Feature: TCG-N03
//!
//! Algorithm:
//! - `active ? icons.active : over ? icons.over : icons.icon` (L17-18)

pub use gpui_telegram_client::dialogs::quick_action::ThreeStateIcon;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum IconState {
    #[default]
    Normal,
    Over,
    Active,
}

/// Three-state icon representation for sidebar rows.
/// Upstream: `Dialogs::ThreeStateIcon` (`dialogs_three_state_icon.h:L14-19`)
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DialogsThreeStateIcon {
    pub normal: String,
    pub over: String,
    pub active: String,
    pub state: IconState,
}

impl DialogsThreeStateIcon {
    pub fn new(normal: impl Into<String>, over: impl Into<String>, active: impl Into<String>) -> Self {
        Self {
            normal: normal.into(),
            over: over.into(),
            active: active.into(),
            state: IconState::Normal,
        }
    }

    /// Select current icon asset key based on active and over states.
    /// Upstream: `dialogs_three_state_icon.h:L17-18`
    pub fn select(&self, active: bool, over: bool) -> &str {
        if active {
            &self.active
        } else if over {
            &self.over
        } else {
            &self.normal
        }
    }

    pub fn current_icon(&self) -> &str {
        match self.state {
            IconState::Active => &self.active,
            IconState::Over => &self.over,
            IconState::Normal => &self.normal,
        }
    }
}
