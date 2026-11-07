//! Upstream: `Telegram/SourceFiles/ui/effects/premium_stars_colored.cpp:L1-L350`
//! Upstream: `Telegram/SourceFiles/ui/effects/premium_stars_colored.h:L18-L59`
//!
//! Multi-colored glittering mini stars for premium animated emojis and badges.

use gpui::*;

#[derive(Debug, Clone)]
pub struct ColoredStar {
    pub x: f32,
    pub y: f32,
    pub color: u32,
    pub scale: f32,
}

#[derive(Debug, Clone)]
pub struct PremiumStarsColoredEffects {
    stars: Vec<ColoredStar>,
}

impl PremiumStarsColoredEffects {
    pub fn new() -> Self {
        let colors = [0xFFD700, 0x8C5BE8, 0x00BFFF, 0xFF69B4];
        let mut stars = Vec::with_capacity(12);
        for i in 0..12 {
            stars.push(ColoredStar {
                x: ((i * 19) % 100) as f32,
                y: ((i * 29) % 100) as f32,
                color: colors[i % colors.len()],
                scale: 0.8 + ((i % 3) as f32) * 0.15,
            });
        }
        Self { stars }
    }

    pub fn star_count(&self) -> usize {
        self.stars.len()
    }
}

impl Default for PremiumStarsColoredEffects {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for PremiumStarsColoredEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .relative()
            .size_full()
            .overflow_hidden()
    }
}
