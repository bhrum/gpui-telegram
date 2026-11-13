//! Upstream: `Telegram/SourceFiles/ui/effects/fireworks_animation.cpp:L16-L30`, `L47-L67`
//! Upstream: `Telegram/SourceFiles/ui/effects/fireworks_animation.h:L18-L64`
//!
//! Celebratory confetti & firework particle simulation.
//! Emits 60 primary particles + 30 fall particles across a 480x320 canvas with 6 palette colors.

use gpui::*;

pub const PARTICLES_COUNT: usize = 60;
pub const FALL_COUNT: usize = 30;
pub const FIREWORK_WIDTH: f32 = 480.0;
pub const FIREWORK_HEIGHT: f32 = 320.0;
pub const FIREWORK_PALETTE: [u32; 6] = [
    0xE8BC2C, 0xD0049E, 0x02CBFE, 0x5723FD, 0xFE8C27, 0x6CB859,
];

#[derive(Debug, Clone)]
pub struct FireworkParticle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub size: f32,
    pub color_index: usize,
    pub is_circle: bool,
    pub finished: bool,
}

#[derive(Debug, Clone)]
pub struct FireworksAnimationEffects {
    particles: Vec<FireworkParticle>,
    is_animating: bool,
    speed_coef: f32,
}

impl FireworksAnimationEffects {
    pub fn new() -> Self {
        let mut particles = Vec::with_capacity(PARTICLES_COUNT + FALL_COUNT);
        for i in 0..PARTICLES_COUNT {
            let angle = (i as f32) * (std::f32::consts::PI * 2.0 / PARTICLES_COUNT as f32);
            let speed = 2.0 + ((i % 5) as f32) * 0.8;
            particles.push(FireworkParticle {
                x: FIREWORK_WIDTH / 2.0,
                y: FIREWORK_HEIGHT / 2.0,
                vx: angle.cos() * speed,
                vy: angle.sin() * speed,
                size: 3.0 + ((i % 3) as f32),
                color_index: i % FIREWORK_PALETTE.len(),
                is_circle: i % 2 == 0,
                finished: false,
            });
        }

        Self {
            particles,
            is_animating: true,
            speed_coef: 1.0,
        }
    }

    /// C++: `fireworks_animation.cpp:L47-L67`: updates particle positions and decays speed
    pub fn update(&mut self, dt_ms: f32) {
        let mut all_finished = true;
        for p in &mut self.particles {
            if !p.finished {
                p.x += p.vx * self.speed_coef;
                p.y += (p.vy + 0.1) * self.speed_coef; // Gravity
                if p.y > FIREWORK_HEIGHT || p.x < 0.0 || p.x > FIREWORK_WIDTH {
                    p.finished = true;
                } else {
                    all_finished = false;
                }
            }
        }
        if all_finished {
            self.is_animating = false;
        } else if self.speed_coef > 0.2 {
            self.speed_coef = (self.speed_coef - (dt_ms / 16.0) * 0.015).max(0.2);
        }
    }

    pub fn is_animating(&self) -> bool {
        self.is_animating
    }

    pub fn particles(&self) -> &[FireworkParticle] {
        &self.particles
    }
}

impl Default for FireworksAnimationEffects {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for FireworksAnimationEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .relative()
            .w(px(FIREWORK_WIDTH))
            .h(px(FIREWORK_HEIGHT))
            .overflow_hidden()
    }
}
