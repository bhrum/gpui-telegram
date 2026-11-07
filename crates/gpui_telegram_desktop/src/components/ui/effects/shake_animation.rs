//! Upstream: `Telegram/SourceFiles/ui/effects/shake_animation.cpp:L14-L37`
//! Upstream: `Telegram/SourceFiles/ui/effects/shake_animation.h:L10-L15`
//!
//! Error shake physics animation callback. Computes 5-segment alternating horizontal
//! oscillation for wrong passcode or validation error feedback.

use gpui::*;

pub const SHIFT_PROGRESS: f32 = 6.0;
pub const SEGMENTS_COUNT: i32 = 5;

#[derive(Debug, Clone)]
pub struct ShakeAnimationEffects {
    progress: f32,
    shake_magnitude: f32,
    is_shaking: bool,
}

impl ShakeAnimationEffects {
    pub fn new(shake_magnitude: f32) -> Self {
        Self {
            progress: 1.0,
            shake_magnitude: if shake_magnitude > 0.0 { shake_magnitude } else { 8.0 },
            is_shaking: false,
        }
    }

    pub fn start(&mut self) {
        self.progress = 0.0;
        self.is_shaking = true;
    }

    pub fn set_progress(&mut self, progress: f32) {
        self.progress = progress.clamp(0.0, 1.0);
        if self.progress >= 1.0 {
            self.is_shaking = false;
        }
    }

    /// C++: `shake_animation.cpp:L17-L36`:
    /// Computes normalized `shift` in `[-1.0, 1.0]` across 5 segments
    pub fn compute_shift(&self) -> f32 {
        let full_progress = self.progress * SHIFT_PROGRESS;
        let segment = (full_progress.floor() as i32).clamp(0, SEGMENTS_COUNT);
        let part = full_progress - (segment as f32);

        let from = if segment == 0 {
            0.0
        } else if segment == 1 || segment == 3 || segment == 5 {
            1.0
        } else {
            -1.0
        };

        let to = if segment == 0 || segment == 2 || segment == 4 {
            1.0
        } else if segment == 1 || segment == 3 {
            -1.0
        } else {
            0.0
        };

        from * (1.0 - part) + to * part
    }

    pub fn current_pixel_offset(&self) -> f32 {
        self.compute_shift() * self.shake_magnitude
    }

    pub fn is_shaking(&self) -> bool {
        self.is_shaking
    }
}

impl Default for ShakeAnimationEffects {
    fn default() -> Self {
        Self::new(8.0)
    }
}

impl Render for ShakeAnimationEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let offset = self.current_pixel_offset();
        div()
            .relative()
            .left(px(offset))
    }
}
