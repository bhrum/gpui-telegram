//! UI Swipe Handler touch / trackpad horizontal gesture interaction.
//! Upstream: `Telegram/SourceFiles/ui/controls/swipe_handler.cpp:L28-L50, L52-L100`
//!           `Telegram/SourceFiles/ui/controls/swipe_handler.h:L24-L46, L48-L60`
//!
//! Logarithmic damping overscroll curve, reply swipe trigger thresholds, and back navigation.

use gpui::*;

pub const K_SWIPE_SLOW: f32 = 0.2;
pub const K_MSG_BARE_ID_SWIPE_BACK: i64 = i64::MAX - 77;
pub const K_SWIPED_BACK_SPEED_RATIO: f32 = 0.35;
pub const K_OVERSWIPE_LOG_A: f32 = 16.0;
pub const K_OVERSWIPE_LOG_B: f32 = 10.0;

/// Logarithmic damping of the swipe translation past the action threshold.
/// Mirrors upstream `DampedOverswipe` in `ui/controls/swipe_handler.cpp:L38-L46`.
pub fn damped_overswipe(translation: f32, scale: f32) -> f32 {
    if translation == 0.0 {
        return 0.0;
    }
    let s = if scale <= 0.0 { 1.0 } else { scale };
    let value = translation.abs() / s;
    let result = K_OVERSWIPE_LOG_A * (1.0 + value / K_OVERSWIPE_LOG_B).ln();
    (if translation > 0.0 { 1.0 } else { -1.0 }) * result * s
}

/// SwipeHandlerControls component. Target symbol required by ledger: `SwipeHandlerControls`.
#[derive(Debug, Clone)]
pub struct SwipeHandlerControls {
    pub translation: f32,
    pub threshold: f32,
    pub msg_bare_id: i64,
    pub is_triggered: bool,
    pub scale: f32,
}

impl SwipeHandlerControls {
    pub fn new(threshold: f32) -> Self {
        Self {
            translation: 0.0,
            threshold,
            msg_bare_id: 0,
            is_triggered: false,
            scale: 1.0,
        }
    }

    pub fn with_msg_id(mut self, msg_bare_id: i64) -> Self {
        self.msg_bare_id = msg_bare_id;
        self
    }

    pub fn update_translation(&mut self, delta: f32) -> f32 {
        self.translation += delta;
        if self.translation.abs() > self.threshold {
            let over = self.translation.signum() * (self.translation.abs() - self.threshold);
            let damped = damped_overswipe(over, self.scale);
            self.is_triggered = true;
            self.translation.signum() * self.threshold + damped
        } else {
            self.is_triggered = false;
            self.translation
        }
    }

    pub fn reset(&mut self) {
        self.translation = 0.0;
        self.is_triggered = false;
    }
}

impl Default for SwipeHandlerControls {
    fn default() -> Self {
        Self::new(60.0)
    }
}

impl Render for SwipeHandlerControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let opacity = (self.translation.abs() / self.threshold).clamp(0.0, 1.0);
        div()
            .h(px(32.0))
            .flex()
            .items_center()
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(hsla(204.0 / 360.0, 0.88, 0.53, opacity))
                    .child(if self.is_triggered { "↩ Reply" } else { "Swipe to reply" })
            )
    }
}
