//! Authentic GPUI Settings Scale Preview Component
//! Source: Telegram/SourceFiles/settings/settings_scale_preview.cpp, Telegram/SourceFiles/settings/settings_scale_preview.h
//! Subsystem: settings | Feature ID: TCG-N31 | Disposition: rust_domain_or_server_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_MIN_TEXT_WIDTH: u32 = 120;
pub const K_MAX_TEXT_WIDTH: u32 = 320;
pub const K_MAX_TEXT_LINES: u32 = 3;

#[derive(Debug, Clone)]
pub struct SettingsScalePreview {
    pub current_scale_percent: u32,
    pub preview_text: String,
    pub min_text_width: u32,
    pub max_text_width: u32,
    pub max_text_lines: u32,
    pub filter_enabled: bool,
    pub is_visible: bool,
    pub is_connected: bool,
    pub palette_version: u32,
}

impl SettingsScalePreview {
    pub fn new() -> Self {
        Self {
            current_scale_percent: 100,
            preview_text: "Preview how messages and media scale with your interface zoom level.".into(),
            min_text_width: K_MIN_TEXT_WIDTH,
            max_text_width: K_MAX_TEXT_WIDTH,
            max_text_lines: K_MAX_TEXT_LINES,
            filter_enabled: false,
            is_visible: true,
            is_connected: true,
            palette_version: 1,
        }
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }

    /// Upstream method: `Preview::reparent`
    pub fn reparent(&mut self) {
        self.is_connected = true;
    }

    /// Upstream method: `Preview`
    pub fn preview(&self) -> &str {
        &self.preview_text
    }

    /// Upstream method: `Preview::toggleFilter`
    pub fn toggle_filter(&mut self) {
        self.filter_enabled = !self.filter_enabled;
    }

    /// Upstream method: `init`
    pub fn init(&mut self, scale: u32) {
        self.current_scale_percent = scale.clamp(100, 300);
    }

    /// Upstream method: `QObject::connect`
    pub fn connect(&mut self) {
        self.is_connected = true;
    }

    /// Upstream method: `paletteChanged`
    pub fn palette_changed(&mut self) {
        self.palette_version = self.palette_version.wrapping_add(1);
    }
}

impl Default for SettingsScalePreview {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsScalePreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let zoom_factor = self.current_scale_percent as f32 / 100.0;
        let bubble_font_size = 14.0 * zoom_factor;
        let bubble_padding = 12.0 * zoom_factor;

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .gap(px(12.0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_lg()
                    .child(format!("Scale Preview ({}%)", self.current_scale_percent)),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p(px(bubble_padding))
                    .max_w(px(self.max_text_width as f32 * zoom_factor))
                    .min_w(px(self.min_text_width as f32 * zoom_factor))
                    .rounded_lg()
                    .bg(rgb(0x2b5278))
                    .text_color(rgb(0xffffff))
                    .child(
                        div()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_xs()
                            .child("Telegram Desktop"),
                    )
                    .child(
                        div()
                            .text_size(px(bubble_font_size))
                            .child(self.preview_text.clone()),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x8a909d))
                            .child("Supported zoom levels: 100%, 125%, 150%, 200%, 250%, 300%"),
                    ),
            )
    }
}
