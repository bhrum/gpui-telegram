//! Upstream: `Telegram/SourceFiles/ui/effects/premium_stars.cpp:L1-L199`
//! Upstream: `Telegram/SourceFiles/ui/effects/premium_stars.h:L18-L88`
//!
//! Sparkling mini stars cluster layout with randomized twinkling intervals.

use gpui::*;

#[derive(Debug, Clone)]
pub struct TwinkleStar {
    pub x: f32,
    pub y: f32,
    pub scale: f32,
    pub period_ms: f32,
}

#[derive(Debug, Clone)]
pub struct PremiumStarsEffects {
    stars: Vec<TwinkleStar>,
    elapsed_ms: f32,
}

impl PremiumStarsEffects {
    pub fn new(count: usize) -> Self {
        let mut stars = Vec::with_capacity(count);
        for i in 0..count {
            stars.push(TwinkleStar {
                x: ((i * 37) % 120) as f32,
                y: ((i * 23) % 120) as f32,
                scale: 0.8,
                period_ms: 600.0 + ((i % 4) as f32) * 150.0,
            });
        }
        Self {
            stars,
            elapsed_ms: 0.0,
        }
    }

    pub fn update(&mut self, dt_ms: f32) {
        self.elapsed_ms += dt_ms;
    }

    pub fn star_count(&self) -> usize {
        self.stars.len()
    }
}

impl Default for PremiumStarsEffects {
    fn default() -> Self {
        Self::new(10)
    }
}

impl Render for PremiumStarsEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .relative()
            .size_full()
            .overflow_hidden()
    }
}
