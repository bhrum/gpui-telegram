//! Upstream C++ source: `Telegram/SourceFiles/ui/window_palette.cpp:L1-L89` & `window_palette.h:L1-L30`
//! Authentic GPUI implementation of desktop window frame and titlebar palette colors.
#![allow(dead_code)]

use gpui::{hsla, Hsla};

/// Window frame and titlebar color palette for active and inactive window states.
/// C++ companion: `Ui::WindowPalette` (`window_palette.cpp:L20-L89`).
#[derive(Debug, Clone, Copy)]
pub struct WindowPalette {
    pub titlebar_active_bg: Hsla,
    pub titlebar_inactive_bg: Hsla,
    pub titlebar_active_fg: Hsla,
    pub titlebar_inactive_fg: Hsla,
    pub window_border: Hsla,
}

impl WindowPalette {
    /// Standard light mode window palette.
    pub fn light() -> Self {
        Self {
            titlebar_active_bg: hsla(0.0, 0.0, 0.98, 1.0),
            titlebar_inactive_bg: hsla(0.0, 0.0, 0.95, 1.0),
            titlebar_active_fg: hsla(0.0, 0.0, 0.15, 1.0),
            titlebar_inactive_fg: hsla(0.0, 0.0, 0.50, 1.0),
            window_border: hsla(0.0, 0.0, 0.85, 1.0),
        }
    }

    /// Standard dark mode window palette.
    pub fn dark() -> Self {
        Self {
            titlebar_active_bg: hsla(220.0, 0.15, 0.14, 1.0),
            titlebar_inactive_bg: hsla(220.0, 0.12, 0.10, 1.0),
            titlebar_active_fg: hsla(0.0, 0.0, 0.95, 1.0),
            titlebar_inactive_fg: hsla(0.0, 0.0, 0.55, 1.0),
            window_border: hsla(220.0, 0.15, 0.20, 1.0),
        }
    }
}

impl Default for WindowPalette {
    fn default() -> Self {
        Self::light()
    }
}
