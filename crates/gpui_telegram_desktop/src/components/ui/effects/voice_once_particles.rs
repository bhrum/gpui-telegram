//! Upstream: `Telegram/SourceFiles/ui/effects/voice_once_particles.cpp:L17-L52`
//! Upstream: `Telegram/SourceFiles/ui/effects/voice_once_particles.h:L18-L78`
//!
//! View-once voice message waveform decay particle simulation.
//! Emits up to 3 particles per frame in a 140° cone with quadratic alpha falloff `(1 - progress)^2`.

use gpui::*;
use std::time::Duration;

pub const MAX_DELTA: Duration = Duration::from_millis(20);
pub const MAX_PER_FRAME: usize = 3;
pub const CONE_SPREAD_DEG: f32 = 140.0; // [-70°, +70°]

#[derive(Debug, Clone)]
pub struct WaveformParticle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub current_time_ms: f32,
    pub life_time_ms: f32,
    pub alpha: f32,
}

#[derive(Debug, Clone)]
pub struct VoiceOnceParticlesEffects {
    particles: Vec<WaveformParticle>,
    max_particles: usize,
}

impl VoiceOnceParticlesEffects {
    pub fn new(max_count: usize) -> Self {
        Self {
            particles: Vec::with_capacity(max_count),
            max_particles: if max_count > 0 { max_count } else { 32 },
        }
    }

    /// C++: `voice_once_particles.cpp:L24-L37`: `update(dt)`
    pub fn update(&mut self, dt_ms: f32) {
        self.particles.retain_mut(|p| {
            if p.current_time_ms >= p.life_time_ms {
                return false;
            }
            let progress = (p.current_time_ms / p.life_time_ms).clamp(0.0, 1.0);
            p.alpha = (1.0 - progress) * (1.0 - progress); // Quadratic falloff
            p.x += p.vx * dt_ms / 200.0;
            p.y += p.vy * dt_ms / 200.0;
            p.current_time_ms += dt_ms;
            true
        });
    }

    pub fn emit(&mut self, origin_x: f32, origin_y: f32, count: usize) {
        let emit_count = count.min(MAX_PER_FRAME).min(self.max_particles.saturating_sub(self.particles.len()));
        for i in 0..emit_count {
            let angle_deg = -70.0 + ((i * 45) as f32) % CONE_SPREAD_DEG;
            let angle_rad = angle_deg * std::f32::consts::PI / 180.0;
            self.particles.push(WaveformParticle {
                x: origin_x,
                y: origin_y,
                vx: angle_rad.cos() * 20.0,
                vy: angle_rad.sin() * 20.0,
                current_time_ms: 0.0,
                life_time_ms: 600.0,
                alpha: 1.0,
            });
        }
    }

    pub fn particle_count(&self) -> usize {
        self.particles.len()
    }
}

impl Default for VoiceOnceParticlesEffects {
    fn default() -> Self {
        Self::new(32)
    }
}

impl Render for VoiceOnceParticlesEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .relative()
            .size_full()
            .overflow_hidden()
    }
}
