//! Upstream: `Telegram/SourceFiles/ui/effects/skeleton_animation.cpp:L16-L20`, `L90-L100`
//! Upstream: `Telegram/SourceFiles/ui/effects/skeleton_animation.h:L18-L38`
//!
//! Shimmer placeholder skeleton animation for loading text labels.
//! Implements a 2000ms cycle with 1000ms active sliding highlight followed by 1000ms wait.

use gpui::*;
use std::time::Duration;

pub const SLIDE_DURATION: Duration = Duration::from_millis(1000);
pub const WAIT_DURATION: Duration = Duration::from_millis(1000);
pub const FULL_DURATION: Duration = Duration::from_millis(2000);
pub const BASE_ALPHA: f32 = 0.5;
pub const GRADIENT_ALPHA: f32 = 0.2;

#[derive(Debug, Clone)]
pub struct SkeletonAnimationEffects {
    is_active: bool,
    line_widths: Vec<f32>,
    font_height: f32,
    elapsed_ms: u64,
}

impl SkeletonAnimationEffects {
    pub fn new(font_height: f32) -> Self {
        Self {
            is_active: false,
            line_widths: vec![120.0, 80.0],
            font_height: if font_height > 0.0 { font_height } else { 14.0 },
            elapsed_ms: 0,
        }
    }

    pub fn start(&mut self) {
        self.is_active = true;
        self.elapsed_ms = 0;
    }

    pub fn stop(&mut self) {
        self.is_active = false;
    }

    pub fn set_line_widths(&mut self, widths: Vec<f32>) {
        self.line_widths = widths;
    }

    pub fn tick(&mut self, delta_ms: u64) {
        self.elapsed_ms = (self.elapsed_ms + delta_ms) % (FULL_DURATION.as_millis() as u64);
    }

    /// C++: `skeleton_animation.cpp:L90-L93`:
    /// `period = now % kFullDuration; gradientActive = (period < kSlideDuration)`
    pub fn is_gradient_active(&self) -> bool {
        let period = self.elapsed_ms % (FULL_DURATION.as_millis() as u64);
        period < (SLIDE_DURATION.as_millis() as u64)
    }

    /// C++: `skeleton_animation.cpp:L96-L98`: `progress = period / float64(kSlideDuration)`
    pub fn slide_progress(&self) -> f32 {
        let period = self.elapsed_ms % (FULL_DURATION.as_millis() as u64);
        let slide_ms = SLIDE_DURATION.as_millis() as f32;
        if period < (slide_ms as u64) {
            (period as f32) / slide_ms
        } else {
            1.0
        }
    }

    /// C++: `skeleton_animation.cpp:L80-L81`: `thickness = font->height / 2; radius = thickness / 2.`
    pub fn line_metrics(&self) -> (f32, f32) {
        let thickness = self.font_height / 2.0;
        let radius = thickness / 2.0;
        (thickness, radius)
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

impl Default for SkeletonAnimationEffects {
    fn default() -> Self {
        Self::new(14.0)
    }
}

impl Render for SkeletonAnimationEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let (thickness, radius) = self.line_metrics();
        let active = self.is_gradient_active();
        let bg_color = if active { rgb(0x2B5278) } else { rgb(0x182533) };

        div()
            .flex()
            .flex_col()
            .gap(px(6.0))
            .children(self.line_widths.iter().map(|w| {
                div()
                    .w(px(*w))
                    .h(px(thickness))
                    .rounded(px(radius))
                    .bg(bg_color)
            }))
    }
}
