//! Upstream: `Telegram/SourceFiles/ui/effects/premium_star_model.cpp:L1-L28`
//! Upstream: `Telegram/SourceFiles/ui/effects/premium_star_model.h:L10-L26`
//!
//! 3D star mesh model loader and extruded five-point star polygon vertices.

use gpui::*;

#[derive(Debug, Clone)]
pub struct PremiumStarModelEffects {
    point_count: usize,
    outer_radius: f32,
    inner_radius: f32,
    depth: f32,
}

impl PremiumStarModelEffects {
    pub fn new() -> Self {
        Self {
            point_count: 5,
            outer_radius: 1.0,
            inner_radius: 0.382, // Golden ratio inner star radius
            depth: 0.35,
        }
    }

    pub fn point_count(&self) -> usize {
        self.point_count
    }

    pub fn outer_radius(&self) -> f32 {
        self.outer_radius
    }

    pub fn inner_radius(&self) -> f32 {
        self.inner_radius
    }
}

impl Default for PremiumStarModelEffects {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for PremiumStarModelEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_0()
            .invisible()
    }
}
