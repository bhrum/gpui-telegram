//! Compose AI action button with 640ms sparkle cycle and premium star overlay.
//! Upstream reference: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_compose_ai_button.cpp`
//! Upstream header: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_compose_ai_button.h`

use gpui::*;
use serde::{Deserialize, Serialize};

pub const AI_ANIMATION_DURATION_MS: u64 = 640;

/// Calculate star opacities according to 4-phase sparkle cycle
pub fn calc_sparkle_opacities(progress: f32) -> (f32, f32) {
    let p = progress.clamp(0.0, 1.0);
    if p < 0.25 {
        (1.0 - (p / 0.25), 1.0)
    } else if p < 0.50 {
        (0.0, 1.0 - ((p - 0.25) / 0.25))
    } else if p < 0.75 {
        ((p - 0.50) / 0.25, 0.0)
    } else {
        (1.0, (p - 0.75) / 0.25)
    }
}

pub struct HistoryViewComposeAiButtonViewControls {
    pub progress: f32,
    pub is_premium: bool,
    pub is_visible: bool,
    pub is_active: bool,
}

impl HistoryViewComposeAiButtonViewControls {
    pub fn new() -> Self {
        Self {
            progress: 0.0,
            is_premium: false,
            is_visible: true,
            is_active: false,
        }
    }

    pub fn tick(&mut self, elapsed_ms: u64) {
        let delta = elapsed_ms as f32 / AI_ANIMATION_DURATION_MS as f32;
        self.progress = (self.progress + delta) % 1.0;
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }
}

impl Default for HistoryViewComposeAiButtonViewControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewComposeAiButtonViewControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let (s1, s2) = calc_sparkle_opacities(self.progress);

        div()
            .flex()
            .items_center()
            .justify_center()
            .w_8()
            .h_8()
            .rounded_full()
            .bg(rgb(0x1e2c3a))
            .hover(|s| s.bg(rgb(0x2481cc)))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_0p5()
                    .text_xs()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xffffff))
                    .child("AI")
                    .child(div().opacity(s1).text_color(rgb(0xffd700)).child("*"))
                    .child(div().opacity(s2).text_color(rgb(0xffd700)).child("*")),
            )
    }
}
