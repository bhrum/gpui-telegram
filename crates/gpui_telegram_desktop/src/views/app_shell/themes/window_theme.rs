//! WindowThemeThemes: Master theme engine manager, palette applier, and wallpaper background coordinator
//! Upstream: Telegram/SourceFiles/window/themes/window_theme.h, window_theme.cpp

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmbeddedThemeType {
    DayBlue,
    DefaultClassic,
    Night,
    NightGreen,
}

#[derive(Debug, Clone)]
pub struct WindowThemeThemes {
    pub active_theme: EmbeddedThemeType,
    pub is_night_mode: bool,
    pub accent_color: u32,
    pub background_tiled: bool,
    pub wallpaper_id: Option<u64>,
}

impl WindowThemeThemes {
    pub fn new() -> Self {
        Self {
            active_theme: EmbeddedThemeType::Night,
            is_night_mode: true,
            accent_color: 0x5288C1,
            background_tiled: false,
            wallpaper_id: None,
        }
    }

    pub fn toggle_night_mode(&mut self) -> bool {
        self.is_night_mode = !self.is_night_mode;
        self.active_theme = if self.is_night_mode {
            EmbeddedThemeType::Night
        } else {
            EmbeddedThemeType::DayBlue
        };
        self.accent_color = if self.is_night_mode { 0x5288C1 } else { 0x2AABEE };
        self.is_night_mode
    }

    pub fn set_theme(&mut self, theme: EmbeddedThemeType) {
        self.active_theme = theme;
        self.is_night_mode = matches!(theme, EmbeddedThemeType::Night | EmbeddedThemeType::NightGreen);
        self.accent_color = match theme {
            EmbeddedThemeType::DayBlue => 0x2AABEE,
            EmbeddedThemeType::DefaultClassic => 0x3390EC,
            EmbeddedThemeType::Night => 0x5288C1,
            EmbeddedThemeType::NightGreen => 0x4FA882,
        };
    }

    pub fn set_accent_color(&mut self, color: u32) {
        self.accent_color = color;
    }

    pub fn set_wallpaper(&mut self, wallpaper_id: Option<u64>, tiled: bool) {
        self.wallpaper_id = wallpaper_id;
        self.background_tiled = tiled;
    }
}

impl Default for WindowThemeThemes {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowThemeThemes {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0x82868A))
            .child(format!(
                "ThemeManager: theme={:?}, night={}, accent=#{:06X}",
                self.active_theme, self.is_night_mode, self.accent_color
            ))
    }
}
