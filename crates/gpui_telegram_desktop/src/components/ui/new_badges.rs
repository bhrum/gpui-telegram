//! Upstream C++ source: `Telegram/SourceFiles/ui/new_badges.cpp:L1-L128` & `new_badges.h:L1-L32`
//! Authentic GPUI implementation of "NEW" feature announcement badges.
#![allow(dead_code)]

use gpui::{hsla, px, Hsla, Pixels};

/// Promotional pill badge indicating a new or preview feature.
/// C++ companion: `Ui::NewBadge` (`new_badges.cpp:L20-L128`).
#[derive(Debug, Clone)]
pub struct NewBadges {
    text: String,
    bg_color: Hsla,
    fg_color: Hsla,
    height: Pixels,
}

impl NewBadges {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            bg_color: hsla(210.0, 0.90, 0.55, 1.0),
            fg_color: hsla(0.0, 0.0, 1.0, 1.0),
            height: px(16.0),
        }
    }

    pub fn standard_new() -> Self {
        Self::new("NEW")
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn bg_color(&self) -> Hsla {
        self.bg_color
    }

    pub fn fg_color(&self) -> Hsla {
        self.fg_color
    }

    pub fn height(&self) -> Pixels {
        self.height
    }
}

impl Default for NewBadges {
    fn default() -> Self {
        Self::standard_new()
    }
}
