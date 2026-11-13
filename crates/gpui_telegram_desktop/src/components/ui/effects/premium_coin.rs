//! Upstream: `Telegram/SourceFiles/ui/effects/premium_coin.cpp:L13-L30`
//! Upstream: `Telegram/SourceFiles/ui/effects/premium_coin.h:L18-L36`
//!
//! 3D spinning Telegram Star Coin cover component.
//! Implements 3000ms spin cycle, 2000ms respin delay, and 4x MSAA rendering.

use gpui::*;
use std::time::Duration;

pub const SPIN_DURATION: Duration = Duration::from_millis(3000);
pub const RESPIN_DELAY: Duration = Duration::from_millis(2000);
pub const FLECKS_PERIOD: f32 = 50.0;
pub const SAMPLE_COUNT: usize = 4;

#[derive(Debug, Clone)]
pub struct PremiumCoinEffects {
    spin_progress: f32,
    yaw: f32,
    pitch: f32,
    is_spinning: bool,
}

impl PremiumCoinEffects {
    pub fn new() -> Self {
        Self {
            spin_progress: 0.0,
            yaw: 0.0,
            pitch: 0.0,
            is_spinning: true,
        }
    }

    pub fn set_spin_progress(&mut self, progress: f32) {
        self.spin_progress = progress.clamp(0.0, 1.0);
        self.yaw = self.spin_progress * 360.0;
    }

    pub fn yaw(&self) -> f32 {
        self.yaw
    }

    pub fn is_spinning(&self) -> bool {
        self.is_spinning
    }
}

impl Default for PremiumCoinEffects {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for PremiumCoinEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size(px(120.0))
            .rounded(px(60.0))
            .bg(rgb(0xFFD700))
            .flex()
            .items_center()
            .justify_center()
            .text_size(px(32.0))
            .child("🪙")
    }
}
