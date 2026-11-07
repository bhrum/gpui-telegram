//! Authentic GPUI Reciprocal Privacy vs Premium Upsell Modal Component.
//! Reference: Telegram/SourceFiles/ui/boxes/show_or_premium_box.cpp:L25-L218
//! Reference: Telegram/SourceFiles/ui/boxes/show_or_premium_box.h:L15-L32

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Line separator opacity factor between options: 0.3.
/// Reference: `show_or_premium_box.cpp:L30`
pub const K_SHOW_OR_LINE_OPACITY: f32 = 0.3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShowOrPremium {
    LastSeen,
    ReadTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShowOrPremiumChoice {
    JustShow,
    ToPremium,
}

/// Target symbol for reciprocal privacy vs premium subscription modal box.
/// Reference: `show_or_premium_box.h:L15-L32`
#[derive(Debug, Clone)]
pub struct ShowOrPremiumBoxBoxes {
    pub mode: ShowOrPremium,
    pub short_name: String,
    pub choice: Option<ShowOrPremiumChoice>,
    pub is_open: bool,
}

impl Default for ShowOrPremiumBoxBoxes {
    fn default() -> Self {
        Self::new(ShowOrPremium::LastSeen, "Alex")
    }
}

impl ShowOrPremiumBoxBoxes {
    pub fn new(mode: ShowOrPremium, short_name: impl Into<String>) -> Self {
        Self {
            mode,
            short_name: short_name.into(),
            choice: None,
            is_open: true,
        }
    }

    /// User opted to reveal own last seen or read time to view the other peer's.
    /// Reference: `show_or_premium_box.cpp:L65-L80`
    pub fn choose_just_show(&mut self) {
        self.choice = Some(ShowOrPremiumChoice::JustShow);
        self.is_open = false;
    }

    /// User opted to subscribe to Telegram Premium for one-way visibility.
    /// Reference: `show_or_premium_box.cpp:L85-L100`
    pub fn choose_to_premium(&mut self) {
        self.choice = Some(ShowOrPremiumChoice::ToPremium);
        self.is_open = false;
    }
}
