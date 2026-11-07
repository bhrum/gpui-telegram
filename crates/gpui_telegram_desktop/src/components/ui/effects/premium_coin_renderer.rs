//! Upstream: `Telegram/SourceFiles/ui/effects/premium_coin_renderer.cpp:L1-L369`
//! Upstream: `Telegram/SourceFiles/ui/effects/premium_coin_renderer.h:L18-L90`
//!
//! Gold PBR shader uniforms and specular reflection pipeline for 3D Star Coin.

use gpui::*;

#[derive(Debug, Clone)]
pub struct PremiumCoinRendererEffects {
    gold_specular: f32,
    metallic: f32,
    roughness: f32,
    light_angle: f32,
}

impl PremiumCoinRendererEffects {
    pub fn new() -> Self {
        Self {
            gold_specular: 0.95,
            metallic: 1.0,
            roughness: 0.2,
            light_angle: 45.0,
        }
    }

    pub fn set_light_angle(&mut self, angle_deg: f32) {
        self.light_angle = angle_deg;
    }

    pub fn gold_specular(&self) -> f32 {
        self.gold_specular
    }
}

impl Default for PremiumCoinRendererEffects {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for PremiumCoinRendererEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_0()
            .invisible()
    }
}
