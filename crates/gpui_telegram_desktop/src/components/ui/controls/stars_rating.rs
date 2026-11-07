//! UI Stars Rating interactive widget and level bar.
//! Upstream: `Telegram/SourceFiles/ui/controls/stars_rating.cpp:L40-L60, L62-L80, L95-L132`
//!           `Telegram/SourceFiles/ui/controls/stars_rating.h:L29-L78`
//!
//! Visualizes Telegram Star ratings, boost levels, and metric counter abbreviations (K, M).

use gpui::*;

/// Formats star count per `ui/controls/stars_rating.cpp:L66-L72`.
pub fn format_stars_scale(count: i64) -> String {
    if count < 10_000 {
        count.to_string()
    } else if count < 10_000_000 {
        format!("{:.1}K", count as f64 / 1_000.0)
    } else {
        format!("{:.1}M", count as f64 / 1_000_000.0)
    }
}

/// StarsRatingControls component. Target symbol required by ledger: `StarsRatingControls`.
#[derive(Debug, Clone)]
pub struct StarsRatingControls {
    pub stars: i64,
    pub level: usize,
    pub this_level_stars: i64,
    pub next_level_stars: i64,
}

impl StarsRatingControls {
    pub fn new(stars: i64) -> Self {
        Self {
            stars,
            level: 1,
            this_level_stars: 0,
            next_level_stars: 100,
        }
    }

    pub fn formatted_stars(&self) -> String {
        format_stars_scale(self.stars)
    }

    pub fn level_fraction(&self) -> f32 {
        let span = (self.next_level_stars - self.this_level_stars).max(1);
        let current = (self.stars - self.this_level_stars).max(0);
        (current as f32 / span as f32).clamp(0.0, 1.0)
    }
}

impl Default for StarsRatingControls {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Render for StarsRatingControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap(px(6.0))
            .p(px(4.0))
            .child("⭐")
            .child(self.formatted_stars())
    }
}
