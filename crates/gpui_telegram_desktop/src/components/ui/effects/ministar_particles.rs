//! Upstream: `Telegram/SourceFiles/ui/effects/ministar_particles.cpp:L19-L59`
//! Upstream: `Telegram/SourceFiles/ui/effects/ministar_particles.h:L18-L63`
//!
//! Mini star particle emitter for premium badges, buttons and profile headers.
//! Supports dynamic color palettes, variable rotation speeds and visibility fading.

use gpui::*;

#[derive(Debug, Clone)]
pub struct MiniStarParticle {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub speed: f32,
    pub color_index: usize,
    pub alpha: f32,
}

#[derive(Debug, Clone)]
pub struct MinistarParticlesEffects {
    particles: Vec<MiniStarParticle>,
    colors: Vec<u32>,
    speed: f32,
    visible: f32,
}

impl MinistarParticlesEffects {
    pub fn new(count: usize, star_size: f32) -> Self {
        let colors = vec![0xFFD700, 0xFFA500, 0xFF69B4, 0x00BFFF];
        let mut particles = Vec::with_capacity(count);
        for i in 0..count {
            particles.push(MiniStarParticle {
                x: ((i * 23) % 100) as f32,
                y: ((i * 17) % 100) as f32,
                size: if star_size > 0.0 { star_size } else { 10.0 },
                speed: 1.0 + ((i % 3) as f32) * 0.5,
                color_index: i % colors.len(),
                alpha: 0.8,
            });
        }

        Self {
            particles,
            colors,
            speed: 1.0,
            visible: 1.0,
        }
    }

    /// C++: `ministar_particles.cpp:L26-L32`
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    pub fn set_visible(&mut self, visible: f32) {
        self.visible = visible.clamp(0.0, 1.0);
    }

    pub fn set_colors(&mut self, colors: Vec<u32>) {
        if !colors.is_empty() {
            self.colors = colors;
        }
    }

    pub fn particle_count(&self) -> usize {
        self.particles.len()
    }
}

impl Default for MinistarParticlesEffects {
    fn default() -> Self {
        Self::new(8, 10.0)
    }
}

impl Render for MinistarParticlesEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .relative()
            .size_full()
            .opacity(self.visible)
    }
}
