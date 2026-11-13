//! Upstream: `Telegram/SourceFiles/ui/effects/premium_promo_particles.cpp:L1-L833`
//! Upstream: `Telegram/SourceFiles/ui/effects/premium_promo_particles.h:L18-L42`
//!
//! Promo background atmospheric particle system for Telegram Premium subscription showcase.

use gpui::*;

#[derive(Debug, Clone)]
pub struct PromoParticle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub size: f32,
    pub alpha: f32,
}

#[derive(Debug, Clone)]
pub struct PremiumPromoParticlesEffects {
    particles: Vec<PromoParticle>,
    is_active: bool,
}

impl PremiumPromoParticlesEffects {
    pub fn new(count: usize) -> Self {
        let mut particles = Vec::with_capacity(count);
        for i in 0..count {
            particles.push(PromoParticle {
                x: ((i * 31) % 300) as f32,
                y: ((i * 47) % 200) as f32,
                vx: 0.2 + ((i % 3) as f32) * 0.1,
                vy: -0.4 - ((i % 4) as f32) * 0.15,
                size: 2.0 + ((i % 3) as f32),
                alpha: 0.5 + ((i % 5) as f32) * 0.1,
            });
        }
        Self {
            particles,
            is_active: true,
        }
    }

    pub fn update(&mut self, dt_ms: f32) {
        for p in &mut self.particles {
            p.x += p.vx * (dt_ms / 16.0);
            p.y += p.vy * (dt_ms / 16.0);
            if p.y < 0.0 {
                p.y = 200.0;
            }
        }
    }

    pub fn particle_count(&self) -> usize {
        self.particles.len()
    }
}

impl Default for PremiumPromoParticlesEffects {
    fn default() -> Self {
        Self::new(24)
    }
}

impl Render for PremiumPromoParticlesEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .relative()
            .size_full()
            .overflow_hidden()
    }
}
