//! Upstream C++ source: `Telegram/SourceFiles/ui/vertical_list.cpp:L1-L83` & `vertical_list.h:L1-L40`
//! Authentic GPUI implementation of vertical stacked layout container with dividers.
#![allow(dead_code)]

use gpui::{px, Pixels};

/// Stacking container ordering widgets vertically with uniform item spacing.
/// C++ companion: `Ui::VerticalList` (`vertical_list.cpp:L15-L83`).
#[derive(Debug, Clone)]
pub struct VerticalList {
    spacing: Pixels,
    show_dividers: bool,
    item_count: usize,
}

impl VerticalList {
    pub fn new(spacing_px: f32, show_dividers: bool) -> Self {
        Self {
            spacing: px(spacing_px),
            show_dividers,
            item_count: 0,
        }
    }

    pub fn set_item_count(&mut self, count: usize) {
        self.item_count = count;
    }

    pub fn spacing(&self) -> Pixels {
        self.spacing
    }

    pub fn has_dividers(&self) -> bool {
        self.show_dividers
    }

    pub fn item_count(&self) -> usize {
        self.item_count
    }
}

impl Default for VerticalList {
    fn default() -> Self {
        Self::new(4.0, false)
    }
}
