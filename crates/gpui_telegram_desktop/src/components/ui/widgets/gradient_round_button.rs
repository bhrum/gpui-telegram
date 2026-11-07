//! Authentic GPUI Gradient Round Button with Glare Sweep Animation.
//! Reference: Telegram/SourceFiles/ui/widgets/gradient_round_button.cpp:L17-L70
//! Reference: Telegram/SourceFiles/ui/widgets/gradient_round_button.h:L17-L41

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::{hsla, rgba, Hsla, Rgba};

/// Standard ripple color: QColor(0, 0, 0, 36).
/// Reference: `gradient_round_button.cpp:L45`
pub fn gradient_button_ripple_color() -> Rgba {
    rgba(0x00000024)
}

#[derive(Debug, Clone, PartialEq)]
pub struct GradientStop {
    pub position: f32,
    pub color: Hsla,
}

/// Target symbol for rounded buttons styled with dynamic color gradients and glare reflections.
#[derive(Debug, Clone)]
pub struct GradientRoundButtonWidgets {
    pub text: String,
    pub stops: Vec<GradientStop>,
    pub full_radius: bool,
    pub corner_radius: f32,
    pub is_hovered: bool,
    pub is_pressed: bool,
    pub glare_active: bool,
    pub glare_progress: f32,
    pub glare_width: f32,
}

impl Default for GradientRoundButtonWidgets {
    fn default() -> Self {
        Self::new("Action")
    }
}

impl GradientRoundButtonWidgets {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            stops: vec![
                GradientStop { position: 0.0, color: hsla(0.55, 0.8, 0.5, 1.0) },
                GradientStop { position: 1.0, color: hsla(0.65, 0.8, 0.5, 1.0) },
            ],
            full_radius: true,
            corner_radius: 8.0,
            is_hovered: false,
            is_pressed: false,
            glare_active: false,
            glare_progress: 0.0,
            glare_width: 32.0,
        }
    }

    pub fn set_stops(&mut self, stops: Vec<GradientStop>) {
        self.stops = stops;
    }

    pub fn set_full_radius(&mut self, full: bool) {
        self.full_radius = full;
    }

    pub fn start_glare(&mut self) {
        self.glare_active = true;
        self.glare_progress = 0.0;
    }

    /// Advances glare shine pass across the button face.
    /// Reference: `gradient_round_button.cpp:L50-L60`
    pub fn step_glare(&mut self, delta: f32) -> bool {
        if self.glare_active {
            self.glare_progress += delta;
            if self.glare_progress >= 1.0 {
                self.glare_active = false;
                self.glare_progress = 1.0;
            }
            true
        } else {
            false
        }
    }

    /// Computes glare coordinate across the button width.
    /// Reference: `gradient_round_button.cpp:L54`
    pub fn compute_glare_x(&self, button_width: f32) -> f32 {
        (-self.glare_width) + (button_width + self.glare_width * 2.0) * self.glare_progress
    }
}
