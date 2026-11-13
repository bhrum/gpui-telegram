//! WindowHistoryHider: Overlay mask preventing visual artifacts and content flashing during rapid transitions
//! Upstream: Telegram/SourceFiles/window/window_history_hider.h, window_history_hider.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct WindowHistoryHider {
    pub is_active: bool,
    pub opacity: f32,
    pub duration_ms: f32,
}

impl WindowHistoryHider {
    pub fn new() -> Self {
        Self {
            is_active: false,
            opacity: 0.0,
            duration_ms: 150.0,
        }
    }

    pub fn cover(&mut self) {
        self.is_active = true;
        self.opacity = 1.0;
    }

    pub fn unhide(&mut self) {
        self.is_active = false;
        self.opacity = 0.0;
    }

    pub fn is_visible(&self) -> bool {
        self.is_active && self.opacity > 0.0
    }
}

impl Default for WindowHistoryHider {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowHistoryHider {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .absolute()
            .inset_0()
            .bg(rgba(0x0E1621FF))
    }
}
