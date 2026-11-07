//! Upstream: `Telegram/SourceFiles/ui/effects/peer_qr_colors_panel.cpp:L1-L625`
//! Upstream: `Telegram/SourceFiles/ui/effects/peer_qr_colors_panel.h:L20-L96`
//!
//! QR Color theme palette picker for personal and channel QR codes.
//! Displays horizontal scroll of gradient swatches with selection indicator.

use gpui::*;
use gpui::prelude::FluentBuilder;

#[derive(Debug, Clone)]
pub struct QrColorTheme {
    pub id: u32,
    pub top_color: u32,
    pub bottom_color: u32,
}

#[derive(Debug, Clone)]
pub struct PeerQrColorsPanelEffects {
    themes: Vec<QrColorTheme>,
    selected_theme_id: u32,
}

impl PeerQrColorsPanelEffects {
    pub fn new() -> Self {
        let themes = vec![
            QrColorTheme { id: 1, top_color: 0x2AABEE, bottom_color: 0x229ED9 },
            QrColorTheme { id: 2, top_color: 0xE8BC2C, bottom_color: 0xFE8C27 },
            QrColorTheme { id: 3, top_color: 0xD0049E, bottom_color: 0x5723FD },
            QrColorTheme { id: 4, top_color: 0x6CB859, bottom_color: 0x2AABEE },
        ];
        Self {
            themes,
            selected_theme_id: 1,
        }
    }

    pub fn select_theme(&mut self, id: u32) {
        if self.themes.iter().any(|t| t.id == id) {
            self.selected_theme_id = id;
        }
    }

    pub fn selected_theme(&self) -> Option<&QrColorTheme> {
        self.themes.iter().find(|t| t.id == self.selected_theme_id)
    }

    pub fn themes(&self) -> &[QrColorTheme] {
        &self.themes
    }
}

impl Default for PeerQrColorsPanelEffects {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for PeerQrColorsPanelEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let selected_id = self.selected_theme_id;
        div()
            .flex()
            .items_center()
            .gap(px(10.0))
            .children(self.themes.iter().map(|t| {
                let is_sel = t.id == selected_id;
                div()
                    .size(px(32.0))
                    .rounded(px(16.0))
                    .bg(rgb(t.top_color))
                    .when(is_sel, |el| el.border_2().border_color(rgb(0xFFFFFF)))
            }))
    }
}
