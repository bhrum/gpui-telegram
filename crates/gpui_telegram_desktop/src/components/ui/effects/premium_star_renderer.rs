//! Upstream: `Telegram/SourceFiles/ui/effects/premium_star_renderer.cpp:L1-L360`
//! Upstream: `Telegram/SourceFiles/ui/effects/premium_star_renderer.h:L18-L92`
//!
//! RHI/GPU pipeline renderer for 3D metallic and golden star shaders.

use gpui::*;

#[derive(Debug, Clone)]
pub struct PremiumStarRendererEffects {
    gradient1: u32,
    gradient2: u32,
    metallic: f32,
    specular_intensity: f32,
}

impl PremiumStarRendererEffects {
    pub fn new() -> Self {
        Self {
            gradient1: 0x8C5BE8,
            gradient2: 0x5028A8,
            metallic: 0.9,
            specular_intensity: 1.2,
        }
    }

    pub fn set_colors(&mut self, g1: u32, g2: u32) {
        self.gradient1 = g1;
        self.gradient2 = g2;
    }

    pub fn metallic(&self) -> f32 {
        self.metallic
    }
}

impl Default for PremiumStarRendererEffects {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for PremiumStarRendererEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_0()
            .invisible()
    }
}
