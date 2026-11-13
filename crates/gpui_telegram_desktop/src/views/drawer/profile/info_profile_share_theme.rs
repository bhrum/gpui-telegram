//! Info Profile Share Theme component
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_share_theme.cpp:L1-L76`
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_share_theme.h:L1-L32`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct QrColorway {
    pub id: u8,
    pub top_color: u32,
    pub bottom_color: u32,
}

/// Theme palette selector allowing users to choose background gradient colors for QR codes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoProfileShareThemeProfile {
    pub selected_palette_index: usize,
    pub available_palettes: Vec<QrColorway>,
}

impl InfoProfileShareThemeProfile {
    pub fn new() -> Self {
        let palettes = vec![
            QrColorway { id: 0, top_color: 0x2ea6ff, bottom_color: 0x1f78b4 },
            QrColorway { id: 1, top_color: 0x9b59b6, bottom_color: 0x8e44ad },
            QrColorway { id: 2, top_color: 0x2ecc71, bottom_color: 0x27ae60 },
            QrColorway { id: 3, top_color: 0xe67e22, bottom_color: 0xd35400 },
            QrColorway { id: 4, top_color: 0xe74c3c, bottom_color: 0xc0392b },
            QrColorway { id: 5, top_color: 0x34495e, bottom_color: 0x2c3e50 },
            QrColorway { id: 6, top_color: 0xf1c40f, bottom_color: 0xf39c12 },
        ];
        Self {
            selected_palette_index: 0,
            available_palettes: palettes,
        }
    }

    pub fn select_palette(&mut self, index: usize) {
        if index < self.available_palettes.len() {
            self.selected_palette_index = index;
        }
    }

    pub fn current_colorway(&self) -> QrColorway {
        self.available_palettes[self.selected_palette_index]
    }
}

impl Default for InfoProfileShareThemeProfile {
    fn default() -> Self {
        Self::new()
    }
}
