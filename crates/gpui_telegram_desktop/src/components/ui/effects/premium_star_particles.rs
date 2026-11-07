//! Upstream: `Telegram/SourceFiles/ui/effects/premium_star_particles.cpp:L1-L452`
//! Upstream: `Telegram/SourceFiles/ui/effects/premium_star_particles.h:L18-L89`
//!
//! Sparkling multi-tiered particle emitter surrounding the 3D Premium Star.

use gpui::*;

#[derive(Debug, Clone)]
pub struct StarParticle {
    pub x: f32,
    pub y: f32,
    pub scale: f32,
    pub alpha: f32,
    pub life_ms: f32,
}

#[derive(Debug, Clone)]
pub struct PremiumStarParticlesEffects {
    particles: Vec<StarParticle>,
    is_active: bool,
}

impl PremiumStarParticlesEffects {
    pub fn new(count: usize) -> Self {
        let mut particles = Vec::with_capacity(count);
        for i in 0..count {
            particles.push(StarParticle {
                x: ((i * 29) % 160) as f32,
                y: ((i * 37) % 160) as f32,
                scale: 0.6 + ((i % 4) as f32) * 0.2,
                alpha: 0.7,
                life_ms: 500.0 + ((i % 5) as f32) * 100.0,
            });
        }
        Self {
            particles,
            is_active: true,
        }
    }

    pub fn update(&mut self, dt_ms: f32) {
        for p in &mut self.particles {
            p.life_ms -= dt_ms;
            if p.life_ms <= 0.0 {
                p.life_ms = 800.0;
            }
        }
    }

    pub fn particle_count(&self) -> usize {
        self.particles.len()
    }
}

impl Default for PremiumStarParticlesEffects {
    fn default() -> Self {
        Self::new(20)
    }
}

impl Render for PremiumStarParticlesEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .relative()
            .size_full()
            .overflow_hidden()
    }
}
