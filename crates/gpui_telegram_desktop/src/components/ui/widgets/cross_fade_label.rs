//! Authentic GPUI Cross Fade Animated Transition Label.
//! Reference: Telegram/SourceFiles/ui/widgets/cross_fade_label.cpp:L15-L50
//! Reference: Telegram/SourceFiles/ui/widgets/cross_fade_label.h:L18-L44

#![allow(dead_code, unused_imports, unused_variables)]

/// Cross-fade transition animation duration: 200ms.
/// Reference: `cross_fade_label.cpp:L15`
pub const K_CROSS_FADE_DURATION_MS: u64 = 200;

/// Target symbol for animated text label fading and sliding between previous and current values.
#[derive(Debug, Clone)]
pub struct CrossFadeLabelWidgets {
    pub current_text: String,
    pub previous_text: String,
    pub current_width: f32,
    pub previous_width: f32,
    pub direction: i32,
    pub is_animating: bool,
    pub progress: f32,
    pub line_height: f32,
}

impl Default for CrossFadeLabelWidgets {
    fn default() -> Self {
        Self::new("Label")
    }
}

impl CrossFadeLabelWidgets {
    pub fn new(text: impl Into<String>) -> Self {
        let t = text.into();
        let w = t.len() as f32 * 8.0;
        Self {
            current_text: t,
            previous_text: String::new(),
            current_width: w,
            previous_width: 0.0,
            direction: 1,
            is_animating: false,
            progress: 1.0,
            line_height: 18.0,
        }
    }

    /// Sets text with animated cross-fade or instant switch.
    /// Reference: `cross_fade_label.cpp:L26-L50`
    pub fn set_text(&mut self, text: impl Into<String>, animated: bool) {
        let new_text = text.into();
        if self.current_text == new_text {
            return;
        }
        self.previous_text = std::mem::replace(&mut self.current_text, new_text);
        self.previous_width = self.current_width;
        self.current_width = self.current_text.len() as f32 * 8.0;

        if animated {
            self.is_animating = true;
            self.progress = 0.0;
        } else {
            self.is_animating = false;
            self.progress = 1.0;
        }
    }

    /// Advances ease-out cubic animation progress.
    /// Reference: `cross_fade_label.cpp:L45-L50`
    pub fn step_animation(&mut self, delta_progress: f32) -> bool {
        if self.is_animating {
            self.progress = (self.progress + delta_progress).min(1.0);
            if self.progress >= 1.0 {
                self.is_animating = false;
            }
            true
        } else {
            false
        }
    }

    /// Evaluates ease-out cubic easing factor: 1 - (1 - t)^3.
    pub fn ease_out_factor(&self) -> f32 {
        let inv = 1.0 - self.progress;
        1.0 - (inv * inv * inv)
    }

    /// Vertical slide displacement in pixels.
    /// Reference: `cross_fade_label.cpp:L60`
    pub fn current_y_offset(&self) -> f32 {
        let factor = self.ease_out_factor();
        (1.0 - factor) * self.line_height * (self.direction as f32)
    }
}
