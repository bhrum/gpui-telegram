//! WindowThemeWarningThemes: Warning dialog for unsupported theme versions or large assets
//! Upstream: Telegram/SourceFiles/window/themes/window_theme_warning.h, window_theme_warning.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct WindowThemeWarningThemes {
    pub warning_text: String,
    pub is_open: bool,
}

impl WindowThemeWarningThemes {
    pub fn new() -> Self {
        Self {
            warning_text: String::new(),
            is_open: false,
        }
    }

    pub fn warn(&mut self, text: impl Into<String>) {
        self.warning_text = text.into();
        self.is_open = true;
    }

    pub fn dismiss(&mut self) {
        self.is_open = false;
    }
}

impl Default for WindowThemeWarningThemes {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowThemeWarningThemes {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0xE53935))
            .child(format!("ThemeWarning: open={}", self.is_open))
    }
}
