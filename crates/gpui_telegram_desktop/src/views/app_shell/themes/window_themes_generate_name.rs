//! WindowThemesGenerateNameThemes: Natural language name generator for customized themes
//! Upstream: Telegram/SourceFiles/window/themes/window_themes_generate_name.h, window_themes_generate_name.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct WindowThemesGenerateNameThemes;

impl WindowThemesGenerateNameThemes {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_name(accent_hex: u32, is_dark: bool) -> String {
        let r = ((accent_hex >> 16) & 0xFF) as f32;
        let g = ((accent_hex >> 8) & 0xFF) as f32;
        let b = (accent_hex & 0xFF) as f32;

        let tone = if is_dark { "Midnight" } else { "Sunny" };
        let color_name = if r > 200.0 && g < 100.0 && b < 100.0 {
            "Crimson"
        } else if b > 180.0 && r < 120.0 {
            "Sapphire"
        } else if g > 180.0 && r < 150.0 {
            "Emerald"
        } else if r > 180.0 && g > 150.0 && b < 100.0 {
            "Amber"
        } else if r > 150.0 && b > 150.0 {
            "Amethyst"
        } else {
            "Classic"
        };

        format!("{} {}", tone, color_name)
    }
}

impl Default for WindowThemesGenerateNameThemes {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowThemesGenerateNameThemes {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0x82868A))
            .child("ThemeNameGenerator")
    }
}
