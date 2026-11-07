//! Window Section Container for Tabbed Selector
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/chat_helpers/tabbed_section.h:L1-L45`
//! - `Telegram/SourceFiles/chat_helpers/tabbed_section.cpp:L1-L81`
//!
//! Subsystem: chat_helpers, Feature: TCG-N11

use super::tabbed_selector::TabbedSelector;
use gpui::{px, Pixels};

/// Section container embedding a TabbedSelector inside a standalone window section.
/// Target symbol in ledger: `TabbedSection`.
#[derive(Debug, Clone)]
pub struct TabbedSection {
    pub selector: TabbedSelector,
    pub is_active: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub focus_in_selector: bool,
}

impl Default for TabbedSection {
    fn default() -> Self {
        Self {
            selector: TabbedSelector::default(),
            is_active: false,
            width: px(345.0),
            height: px(500.0),
            focus_in_selector: false,
        }
    }
}

impl TabbedSection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn before_hiding(&mut self) {
        self.is_active = false;
    }

    pub fn after_shown(&mut self) {
        self.is_active = true;
    }

    pub fn set_geometry(&mut self, width: Pixels, height: Pixels) {
        self.width = width;
        self.height = height;
    }
}
