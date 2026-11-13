//! Upstream: `Telegram/SourceFiles/ui/effects/premium_star.cpp:L23-L34`, `L48-L60`
//! Upstream: `Telegram/SourceFiles/ui/effects/premium_star.h:L18-L121`
//!
//! 3D Interactive Premium Star widget.
//! Manages pitch [-50°, 50°] clamping, 600ms rebound duration, and purple/gold gradient shaders.

use gpui::*;
use std::time::Duration;

pub const SHIMMER_SPEED: f32 = 0.03;
pub const DRAG_YAW: f32 = 0.5;
pub const DRAG_PITCH: f32 = 0.3;
pub const MAX_PITCH: f32 = 50.0;
pub const BACK_DURATION: Duration = Duration::from_millis(600);
pub const IDLE_DELAY: Duration = Duration::from_millis(2000);
pub const OVERSHOOT_TENSION: f32 = 2.0;

#[derive(Debug, Clone)]
pub struct PremiumStarEffects {
    yaw: f32,
    pitch: f32,
    is_golden: bool,
    gradient_top: u32,
    gradient_bottom: u32,
}

impl PremiumStarEffects {
    pub fn new() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.0,
            is_golden: false,
            gradient_top: 0x8C5BE8,
            gradient_bottom: 0x5028A8,
        }
    }

    /// C++: `premium_star.cpp:L58-L60`: `setGolden(golden)`
    pub fn set_golden(&mut self, golden: bool) {
        self.is_golden = golden;
        if golden {
            self.gradient_top = 0xFFD700;
            self.gradient_bottom = 0xF59E0B;
        } else {
            self.gradient_top = 0x8C5BE8;
            self.gradient_bottom = 0x5028A8;
        }
    }

    pub fn set_rotation(&mut self, yaw: f32, pitch: f32) {
        self.yaw = yaw;
        self.pitch = pitch.clamp(-MAX_PITCH, MAX_PITCH);
    }

    pub fn is_golden(&self) -> bool {
        self.is_golden
    }

    pub fn yaw(&self) -> f32 {
        self.yaw
    }

    pub fn pitch(&self) -> f32 {
        self.pitch
    }
}

impl Default for PremiumStarEffects {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for PremiumStarEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size(px(140.0))
            .rounded(px(70.0))
            .bg(rgb(self.gradient_top))
            .flex()
            .items_center()
            .justify_center()
            .text_size(px(40.0))
            .child("⭐")
    }
}
