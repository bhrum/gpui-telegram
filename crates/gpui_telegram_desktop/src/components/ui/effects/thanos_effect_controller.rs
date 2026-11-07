//! Upstream: `Telegram/SourceFiles/ui/effects/thanos_effect_controller.cpp:L22-L25`, `L35-L60`
//! Upstream: `Telegram/SourceFiles/ui/effects/thanos_effect_controller.h:L25-L140`
//!
//! Thanos effect controller for message deletion animation.
//! Batches deletion items and scales effect duration proportionally to item height `min(400 + 0.15 * h, 600)`.

use gpui::*;
use std::time::Duration;

pub const BASE_DURATION: Duration = Duration::from_millis(400);
pub const PER_PIXEL_DURATION: f32 = 0.15;
pub const MAX_DURATION: Duration = Duration::from_millis(600);
pub const PENDING_ATTEMPTS: usize = 4;

#[derive(Debug, Clone)]
pub struct DisintegratingItem {
    pub item_id: i64,
    pub height: f32,
    pub duration_ms: f32,
    pub elapsed_ms: f32,
}

#[derive(Debug, Clone)]
pub struct ThanosEffectControllerEffects {
    active_items: Vec<DisintegratingItem>,
}

impl ThanosEffectControllerEffects {
    pub fn new() -> Self {
        Self {
            active_items: Vec::new(),
        }
    }

    /// C++: `thanos_effect_controller.cpp:L22-L24`:
    /// Computes duration in ms: `min(400 + 0.15 * height, 600)`
    pub fn compute_duration(height: f32) -> f32 {
        let base = BASE_DURATION.as_millis() as f32;
        let max = MAX_DURATION.as_millis() as f32;
        (base + PER_PIXEL_DURATION * height).min(max)
    }

    pub fn capture_item(&mut self, item_id: i64, height: f32) {
        let duration_ms = Self::compute_duration(height);
        self.active_items.push(DisintegratingItem {
            item_id,
            height,
            duration_ms,
            elapsed_ms: 0.0,
        });
    }

    pub fn update(&mut self, dt_ms: f32) {
        self.active_items.retain_mut(|item| {
            item.elapsed_ms += dt_ms;
            item.elapsed_ms < item.duration_ms
        });
    }

    pub fn active_count(&self) -> usize {
        self.active_items.len()
    }
}

impl Default for ThanosEffectControllerEffects {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for ThanosEffectControllerEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_0()
            .invisible()
    }
}
