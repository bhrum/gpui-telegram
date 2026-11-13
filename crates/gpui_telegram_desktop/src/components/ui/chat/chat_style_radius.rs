//! UI Chat Style Radius computation.
//! Upstream: `Telegram/SourceFiles/ui/chat/chat_style_radius.cpp:L15-L45`
//!           `Telegram/SourceFiles/ui/chat/chat_style_radius.h:L10-L24`
//!
//! Computes responsive corner radii for chat bubbles depending on settings and DPI scale.

use gpui::*;

/// ChatStyleRadiusChat component. Target symbol required by ledger: `ChatStyleRadiusChat`.
#[derive(Debug, Clone)]
pub struct ChatStyleRadiusChat {
    pub base_radius: f32,
    pub max_radius: f32,
    pub min_radius: f32,
    pub dpr: f32,
}

impl ChatStyleRadiusChat {
    pub fn new() -> Self {
        Self {
            base_radius: 12.0,
            max_radius: 20.0,
            min_radius: 4.0,
            dpr: 1.0,
        }
    }

    pub fn with_base_radius(mut self, r: f32) -> Self {
        self.base_radius = r.clamp(self.min_radius, self.max_radius);
        self
    }

    pub fn bubble_radius(&self) -> f32 {
        self.base_radius * self.dpr
    }

    pub fn small_radius(&self) -> f32 {
        (self.base_radius / 3.0).max(self.min_radius) * self.dpr
    }
}

impl Default for ChatStyleRadiusChat {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for ChatStyleRadiusChat {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .rounded(px(self.bubble_radius()))
            .p(px(8.0))
    }
}
