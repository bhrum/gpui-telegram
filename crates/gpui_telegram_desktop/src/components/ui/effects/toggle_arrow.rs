//! Upstream: `Telegram/SourceFiles/ui/effects/toggle_arrow.cpp:L20-L77`
//! Upstream: `Telegram/SourceFiles/ui/effects/toggle_arrow.h:L12-L24`
//!
//! Animated rotating chevron arrow for expand/collapse buttons.
//! Rotates points by `alpha = (progress - 1.0) * PI` around the center.

use gpui::*;
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct ToggleArrowEffects {
    size: f32,
    progress: f32,
    is_expanded: bool,
}

impl ToggleArrowEffects {
    pub fn new(size: f32) -> Self {
        Self {
            size: if size > 0.0 { size } else { 12.0 },
            progress: 0.0,
            is_expanded: false,
        }
    }

    pub fn set_expanded(&mut self, expanded: bool, animated: bool) {
        self.is_expanded = expanded;
        if !animated {
            self.progress = if expanded { 1.0 } else { 0.0 };
        }
    }

    pub fn set_progress(&mut self, progress: f32) {
        self.progress = progress.clamp(0.0, 1.0);
    }

    /// C++: `toggle_arrow.cpp:L40`: `alpha = (progress - 1.) * M_PI`
    pub fn rotation_angle_rad(&self) -> f32 {
        (self.progress - 1.0) * PI
    }

    pub fn rotation_degrees(&self) -> f32 {
        self.rotation_angle_rad() * 180.0 / PI
    }

    pub fn is_expanded(&self) -> bool {
        self.is_expanded
    }
}

impl Default for ToggleArrowEffects {
    fn default() -> Self {
        Self::new(12.0)
    }
}

impl Render for ToggleArrowEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let deg = self.rotation_degrees();
        div()
            .size(px(self.size))
            .flex()
            .items_center()
            .justify_center()
            .text_size(px(self.size))
            .text_color(rgb(0x707579))
            .child(if deg.abs() > 90.0 { "▲" } else { "▼" })
    }
}
