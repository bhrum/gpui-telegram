//! WindowThemePreviewThemes: Live chat preview mockup showcasing bubble, wallpaper, and accent colors
//! Upstream: Telegram/SourceFiles/window/themes/window_theme_preview.h, window_theme_preview.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct WindowThemePreviewThemes {
    pub bg_color: u32,
    pub bubble_in_color: u32,
    pub bubble_out_color: u32,
    pub text_in_color: u32,
    pub text_out_color: u32,
}

impl WindowThemePreviewThemes {
    pub fn new() -> Self {
        Self {
            bg_color: 0x0E1621,
            bubble_in_color: 0x182533,
            bubble_out_color: 0x2B5278,
            text_in_color: 0xFFFFFF,
            text_out_color: 0xFFFFFF,
        }
    }

    pub fn set_palette(&mut self, bg: u32, b_in: u32, b_out: u32, t_in: u32, t_out: u32) {
        self.bg_color = bg;
        self.bubble_in_color = b_in;
        self.bubble_out_color = b_out;
        self.text_in_color = t_in;
        self.text_out_color = t_out;
    }
}

impl Default for WindowThemePreviewThemes {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowThemePreviewThemes {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(12.0))
            .p(px(16.0))
            .bg(rgb(self.bg_color))
            .rounded(px(8.0))
            .child(
                div()
                    .p(px(8.0))
                    .bg(rgb(self.bubble_in_color))
                    .text_color(rgb(self.text_in_color))
                    .rounded(px(6.0))
                    .text_xs()
                    .child("Hello! How does this theme look?")
            )
            .child(
                div()
                    .p(px(8.0))
                    .bg(rgb(self.bubble_out_color))
                    .text_color(rgb(self.text_out_color))
                    .rounded(px(6.0))
                    .text_xs()
                    .child("It looks authentic and beautifully tuned!")
            )
    }
}
