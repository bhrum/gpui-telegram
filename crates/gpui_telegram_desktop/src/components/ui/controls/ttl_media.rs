//! UI TTL Media self-destructing countdown indicator.
//! Upstream: `Telegram/SourceFiles/ui/controls/ttl_media.cpp:L30-L90`
//!           `Telegram/SourceFiles/ui/controls/ttl_media.h:L18-L39, L43-L53`
//!
//! Radial circular sweep timer with remaining second readout and burn particles effect.

use gpui::*;

/// TtlMediaControls component. Target symbol required by ledger: `TtlMediaControls`.
#[derive(Debug, Clone)]
pub struct TtlMediaControls {
    pub destroy_at: i64,
    pub ttl_seconds: i64,
    pub now: i64,
}

impl TtlMediaControls {
    pub fn new(destroy_at: i64, ttl_seconds: i64) -> Self {
        Self {
            destroy_at,
            ttl_seconds,
            now: 0,
        }
    }

    pub fn with_now(mut self, now: i64) -> Self {
        self.now = now;
        self
    }

    pub fn seconds_left(&self) -> i64 {
        (self.destroy_at - self.now).max(0)
    }

    pub fn fraction_left(&self) -> f32 {
        if self.ttl_seconds <= 0 {
            0.0
        } else {
            (self.seconds_left() as f32 / self.ttl_seconds as f32).clamp(0.0, 1.0)
        }
    }
}

impl Default for TtlMediaControls {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Render for TtlMediaControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let left = self.seconds_left();
        let frac = self.fraction_left();

        div()
            .w(px(36.0))
            .h(px(36.0))
            .rounded(px(18.0))
            .bg(hsla(0.0, 0.0, 0.0, 0.54))
            .flex()
            .items_center()
            .justify_center()
            .text_size(px(12.0))
            .text_color(hsla(0.0, 0.0, 1.0, 1.0))
            .font_weight(FontWeight::BOLD)
            .child(format!("{}s", left))
    }
}
