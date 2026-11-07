//! WindowSlideAnimation: Interpolated slide transitions and easing curves for view switches
//! Upstream: Telegram/SourceFiles/window/window_slide_animation.h, window_slide_animation.cpp

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlideDirection {
    FromLeft,
    FromRight,
    FromTop,
    FromBottom,
}

#[derive(Debug, Clone)]
pub struct WindowSlideAnimation {
    pub direction: SlideDirection,
    pub progress: f32,
    pub duration_ms: f32,
    pub elapsed_ms: f32,
    pub is_active: bool,
}

impl WindowSlideAnimation {
    pub fn new() -> Self {
        Self {
            direction: SlideDirection::FromRight,
            progress: 1.0,
            duration_ms: 250.0,
            elapsed_ms: 250.0,
            is_active: false,
        }
    }

    pub fn start(&mut self, direction: SlideDirection, duration_ms: f32) {
        self.direction = direction;
        self.duration_ms = duration_ms.max(50.0);
        self.elapsed_ms = 0.0;
        self.progress = 0.0;
        self.is_active = true;
    }

    pub fn tick(&mut self, delta_ms: f32) {
        if !self.is_active {
            return;
        }
        self.elapsed_ms += delta_ms;
        let t = (self.elapsed_ms / self.duration_ms).min(1.0);
        // Easing: easeOutCirc -> sqrt(1 - (t - 1)^2)
        let factor = (1.0 - (t - 1.0) * (t - 1.0)).max(0.0).sqrt();
        self.progress = factor;
        if t >= 1.0 {
            self.progress = 1.0;
            self.is_active = false;
        }
    }

    pub fn is_finished(&self) -> bool {
        !self.is_active
    }

    pub fn current_offset_percent(&self) -> f32 {
        let remaining = 1.0 - self.progress;
        match self.direction {
            SlideDirection::FromRight => remaining * 100.0,
            SlideDirection::FromLeft => -remaining * 100.0,
            SlideDirection::FromTop => -remaining * 100.0,
            SlideDirection::FromBottom => remaining * 100.0,
        }
    }
}

impl Default for WindowSlideAnimation {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowSlideAnimation {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0x82868A))
            .child(format!(
                "SlideAnimation: {:?}, progress={:.2}, active={}",
                self.direction, self.progress, self.is_active
            ))
    }
}
