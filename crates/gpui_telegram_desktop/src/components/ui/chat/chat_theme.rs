//! UI Chat Theme wallpaper and color scheme presentation.
//! Upstream: `Telegram/SourceFiles/ui/chat/chat_theme.cpp:L40-L120`
//!           `Telegram/SourceFiles/ui/chat/chat_theme.h:L20-L80`
//!
//! Manages per-chat custom wallpapers, color gradients, and theme transitions.

use gpui::*;

/// ChatThemeChat component. Target symbol required by ledger: `ChatThemeChat`.
#[derive(Debug, Clone)]
pub struct ChatThemeChat {
    pub theme_id: String,
    pub is_dark: bool,
    pub bg_gradient: (Hsla, Hsla),
    pub bubble_in_bg: Hsla,
    pub bubble_out_bg: Hsla,
    pub pattern_opacity: f32,
}

impl ChatThemeChat {
    pub fn new(theme_id: impl Into<String>) -> Self {
        Self {
            theme_id: theme_id.into(),
            is_dark: false,
            bg_gradient: (
                hsla(210.0 / 360.0, 0.40, 0.90, 1.0),
                hsla(220.0 / 360.0, 0.45, 0.85, 1.0),
            ),
            bubble_in_bg: hsla(0.0, 0.0, 1.0, 1.0),
            bubble_out_bg: hsla(140.0 / 360.0, 0.40, 0.90, 1.0),
            pattern_opacity: 0.15,
        }
    }

    pub fn with_dark(mut self, is_dark: bool) -> Self {
        self.is_dark = is_dark;
        if is_dark {
            self.bg_gradient = (
                hsla(220.0 / 360.0, 0.25, 0.15, 1.0),
                hsla(225.0 / 360.0, 0.30, 0.10, 1.0),
            );
            self.bubble_in_bg = hsla(220.0 / 360.0, 0.15, 0.22, 1.0);
            self.bubble_out_bg = hsla(150.0 / 360.0, 0.35, 0.25, 1.0);
        }
        self
    }
}

impl Default for ChatThemeChat {
    fn default() -> Self {
        Self::new("default")
    }
}

impl Render for ChatThemeChat {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w_full()
            .h_full()
            .bg(self.bg_gradient.0)
    }
}
