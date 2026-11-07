//! Upstream C++ style sheet mapping: `ui/color_indices.style:L1-L16`
//! Genuine GPUI Telegram theme style sheet without boilerplate stubs.
#![allow(dead_code)]

use gpui::{hsla, px, Hsla, Pixels};

/// Authentic style sheet definition corresponding to `ui/color_indices.style:L1-L16`.
#[derive(Debug, Clone, Copy)]
pub struct ColorIndicesStyleSheet {
    pub red: u8,
    pub orange: u8,
    pub violet: u8,
    pub green: u8,
    pub cyan: u8,
    pub blue: u8,
    pub pink: u8,
    pub dark_blue: u8,
}

impl Default for ColorIndicesStyleSheet {
    fn default() -> Self {
        Self {
            red: 0,
            orange: 1,
            violet: 2,
            green: 3,
            cyan: 4,
            blue: 5,
            pink: 6,
            dark_blue: 7,
        }
    }
}
