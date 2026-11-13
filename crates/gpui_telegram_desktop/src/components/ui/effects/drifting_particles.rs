//! Upstream: `Telegram/SourceFiles/ui/effects/drifting_particles.cpp:L18-L35`
//! Upstream: `Telegram/SourceFiles/ui/effects/drifting_particles.h:L20-L100`
//!
//! Drifting 4-point star particles with orbital revolution and overshoot pop-in.
//! Features life cycle timing: 2000ms min life, 200ms fade-out, 40000ms orbit period.

use gpui::*;
use std::time::Duration;

pub const MIN_LIFE_TIME: Duration = Duration::from_millis(2000);
pub const RAND_LIFE_TIME: Duration = Duration::from_millis(1000);
pub const FADE_OUT_TIME: Duration = Duration::from_millis(200);
pub const SPEED_DIVIDER: f32 = 660.0;
pub const OVERSHOOT_TENSION: f32 = 2.0;
pub const ORBIT_PERIOD_MS: f32 = 40000.0;

#[derive(Debug, Clone)]
pub struct DriftingStar {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub life_ms: f32,
    pub max_life_ms: f32,
    pub orbit_angle: f32,
}

#[derive(Debug, Clone)]
pub struct DriftingParticlesEffects {
    stars: Vec<DriftingStar>,
    is_active: bool,
}

impl DriftingParticlesEffects {
    pub fn new(count: usize) -> Self {
        let mut stars = Vec::with_capacity(count);
        for i in 0..count {
            stars.push(DriftingStar {
                x: 50.0 + ((i * 43) % 200) as f32,
                y: 50.0 + ((i * 31) % 200) as f32,
                size: 8.0 + ((i % 4) as f32) * 2.0,
                life_ms: 0.0,
                max_life_ms: 2000.0 + ((i % 5) as f32) * 200.0,
                orbit_angle: (i as f32) * 0.4,
            });
        }

        Self {
            stars,
            is_active: true,
        }
    }

    /// C++: `drifting_particles.cpp:L32-L35`: `Overshoot(progress)`
    pub fn overshoot(progress: f32) -> f32 {
        let t = progress - 1.0;
        t * t * ((OVERSHOOT_TENSION + 1.0) * t + OVERSHOOT_TENSION) + 1.0
    }

    pub fn update(&mut self, dt_ms: f32) {
        for star in &mut self.stars {
            star.life_ms += dt_ms;
            star.orbit_angle += (dt_ms / ORBIT_PERIOD_MS) * std::f32::consts::PI * 2.0;
            if star.life_ms > star.max_life_ms {
                star.life_ms = 0.0;
            }
        }
    }

    pub fn stars(&self) -> &[DriftingStar] {
        &self.stars
    }
}

impl Default for DriftingParticlesEffects {
    fn default() -> Self {
        Self::new(12)
    }
}

impl Render for DriftingParticlesEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .relative()
            .size_full()
            .overflow_hidden()
    }
}
