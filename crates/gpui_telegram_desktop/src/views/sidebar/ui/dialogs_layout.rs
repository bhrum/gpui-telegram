//! Dialog row layout measurements and hit-testing context.
//!
//! Upstream: `Telegram/SourceFiles/dialogs/ui/dialogs_layout.cpp:L1-1458`
//! Subsystem: dialogs, Feature: TCG-N03

use gpui::{px, Pixels, Point};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialogsLayoutUi {
    pub width: u32,
    pub height: u32,
    pub is_narrow: bool,
    pub has_unread: bool,
}

impl Default for DialogsLayoutUi {
    fn default() -> Self {
        Self {
            width: 288,
            height: 62,
            is_narrow: false,
            has_unread: false,
        }
    }
}

impl DialogsLayoutUi {
    pub const AVATAR_SIZE: Pixels = px(46.0);
    pub const AVATAR_ORIGIN: Point<Pixels> = Point { x: px(10.0), y: px(8.0) };
    pub const NAME_LEFT: Pixels = px(68.0);
    pub const NAME_TOP: Pixels = px(10.0);
    pub const TEXT_LEFT: Pixels = px(68.0);
    pub const TEXT_TOP: Pixels = px(34.0);

    pub fn new() -> Self {
        Self::default()
    }
}
