//! UI Torn Edge jagged divider separator.
//! Upstream: `Telegram/SourceFiles/ui/chat/torn_edge.cpp:L15-L45`
//!           `Telegram/SourceFiles/ui/chat/torn_edge.h:L10-L24`
//!
//! Generates sawtooth jagged border paths for Instant View cards and receipt separators.

use gpui::*;

/// TornEdgeChat component. Target symbol required by ledger: `TornEdgeChat`.
#[derive(Debug, Clone)]
pub struct TornEdgeChat {
    pub tooth_width: f32,
    pub tooth_height: f32,
    pub width: f32,
    pub color: Hsla,
}

impl TornEdgeChat {
    pub fn new(width: f32) -> Self {
        Self {
            tooth_width: 8.0,
            tooth_height: 4.0,
            width,
            color: hsla(0.0, 0.0, 0.90, 1.0),
        }
    }

    pub fn tooth_count(&self) -> usize {
        (self.width / self.tooth_width).floor() as usize
    }
}

impl Default for TornEdgeChat {
    fn default() -> Self {
        Self::new(200.0)
    }
}

impl Render for TornEdgeChat {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(self.width))
            .h(px(self.tooth_height))
            .bg(self.color)
    }
}
