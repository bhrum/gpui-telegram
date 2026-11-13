//! WindowThemesEmbeddedThemes: Built-in embedded theme schemes (DayBlue, Default, Night, NightGreen) and accent recipes
//! Upstream: Telegram/SourceFiles/window/themes/window_themes_embedded.h, window_themes_embedded.cpp

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuiltinTheme {
    DayBlue,
    DefaultClassic,
    Night,
    NightGreen,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddedPaletteScheme {
    pub theme: BuiltinTheme,
    pub background: u32,
    pub sidebar_bg: u32,
    pub bubble_in: u32,
    pub bubble_out: u32,
    pub accent: u32,
}

#[derive(Debug, Clone)]
pub struct WindowThemesEmbeddedThemes {
    pub active_builtin: BuiltinTheme,
    pub custom_accent: Option<u32>,
}

impl WindowThemesEmbeddedThemes {
    pub fn new() -> Self {
        Self {
            active_builtin: BuiltinTheme::Night,
            custom_accent: None,
        }
    }

    pub fn scheme_for(theme: BuiltinTheme) -> EmbeddedPaletteScheme {
        match theme {
            BuiltinTheme::DayBlue => EmbeddedPaletteScheme {
                theme,
                background: 0xFFFFFF,
                sidebar_bg: 0xF1F1F1,
                bubble_in: 0xFFFFFF,
                bubble_out: 0xEEFFDE,
                accent: 0x2AABEE,
            },
            BuiltinTheme::DefaultClassic => EmbeddedPaletteScheme {
                theme,
                background: 0xF5F5F5,
                sidebar_bg: 0xFFFFFF,
                bubble_in: 0xFFFFFF,
                bubble_out: 0xEFFDDE,
                accent: 0x3390EC,
            },
            BuiltinTheme::Night => EmbeddedPaletteScheme {
                theme,
                background: 0x18222D,
                sidebar_bg: 0x0E1621,
                bubble_in: 0x182533,
                bubble_out: 0x2B5278,
                accent: 0x5288C1,
            },
            BuiltinTheme::NightGreen => EmbeddedPaletteScheme {
                theme,
                background: 0x17212B,
                sidebar_bg: 0x0E1621,
                bubble_in: 0x1E2C3A,
                bubble_out: 0x26473E,
                accent: 0x4FA882,
            },
        }
    }

    pub fn current_scheme(&self) -> EmbeddedPaletteScheme {
        let mut scheme = Self::scheme_for(self.active_builtin);
        if let Some(accent) = self.custom_accent {
            scheme.accent = accent;
        }
        scheme
    }

    pub fn set_custom_accent(&mut self, accent: Option<u32>) {
        self.custom_accent = accent;
    }
}

impl Default for WindowThemesEmbeddedThemes {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowThemesEmbeddedThemes {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let scheme = self.current_scheme();
        div()
            .text_xs()
            .text_color(rgb(0x82868A))
            .child(format!("EmbeddedScheme: {:?}, accent=#{:06X}", scheme.theme, scheme.accent))
    }
}
