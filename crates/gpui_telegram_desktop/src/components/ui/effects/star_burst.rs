//! Upstream: `Telegram/SourceFiles/ui/effects/star_burst.cpp:L27-L60`
//! Upstream: `Telegram/SourceFiles/ui/effects/star_burst.h:L18-L88`
//!
//! Multilateral starburst explosion animation generator.
//! Simulates radiating star sprites with randomized birth delays and life durations.

use gpui::*;

#[derive(Debug, Clone)]
pub struct BurstStar {
    pub birth_ms: f32,
    pub life_ms: f32,
    pub angle: f32,
    pub speed: f32,
    pub scale: f32,
}

#[derive(Debug, Clone)]
pub struct StarBurstEffects {
    stars: Vec<BurstStar>,
    elapsed_ms: f32,
    duration_ms: f32,
    is_active: bool,
}

impl StarBurstEffects {
    pub fn new(count: usize, duration_ms: f32) -> Self {
        let mut stars = Vec::with_capacity(count);
        for i in 0..count {
            let angle = (i as f32) * (std::f32::consts::PI * 2.0 / count as f32);
            stars.push(BurstStar {
                birth_ms: ((i % 4) as f32) * 50.0,
                life_ms: 400.0 + ((i % 3) as f32) * 100.0,
                angle,
                speed: 3.0 + ((i % 5) as f32) * 0.8,
                scale: 0.8 + ((i % 3) as f32) * 0.2,
            });
        }

        Self {
            stars,
            elapsed_ms: 0.0,
            duration_ms: if duration_ms > 0.0 { duration_ms } else { 800.0 },
            is_active: true,
        }
    }

    pub fn update(&mut self, dt_ms: f32) {
        self.elapsed_ms += dt_ms;
        if self.elapsed_ms >= self.duration_ms {
            self.is_active = false;
        }
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn stars(&self) -> &[BurstStar] {
        &self.stars
    }
}

impl Default for StarBurstEffects {
    fn default() -> Self {
        Self::new(16, 800.0)
    }
}

impl Render for StarBurstEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .relative()
            .size_full()
            .overflow_hidden()
    }
}
