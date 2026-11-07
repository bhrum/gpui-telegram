//! Paid Reaction Star Animation & Undo Toast
//! Faithful upstream C++ extraction:
//! - `history_view_paid_reaction_toast.cpp:L35-L80`

use gpui::*;

pub const PAID_REACTION_STAR_TIERS: [u64; 10] = [1, 5, 10, 50, 100, 500, 1000, 2500, 5000, 10000];
pub const UNDO_COUNTDOWN_TOTAL_MS: i64 = 5000;

#[derive(Debug, Clone)]
pub struct StarParticle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub opacity: f32,
    pub size: f32,
}

#[derive(Debug, Clone)]
pub struct PaidReactionToast {
    pub message_id: u64,
    pub stars_count: u64,
    pub is_anonymous: bool,
    pub remaining_ms: i64,
    pub total_ms: i64,
    pub is_undone: bool,
    pub particles: Vec<StarParticle>,
}

impl PaidReactionToast {
    pub fn new(message_id: u64, stars_count: u64, is_anonymous: bool) -> Self {
        let mut particles = Vec::with_capacity(16);
        for i in 0..16 {
            let angle = (i as f32 / 16.0) * std::f32::consts::PI * 2.0;
            particles.push(StarParticle {
                x: 0.0,
                y: 0.0,
                vx: angle.cos() * 4.0,
                vy: angle.sin() * 4.0,
                opacity: 1.0,
                size: 8.0,
            });
        }

        Self {
            message_id,
            stars_count,
            is_anonymous,
            remaining_ms: UNDO_COUNTDOWN_TOTAL_MS,
            total_ms: UNDO_COUNTDOWN_TOTAL_MS,
            is_undone: false,
            particles,
        }
    }

    pub fn tick_countdown(&mut self, elapsed_ms: i64) -> bool {
        self.remaining_ms = (self.remaining_ms - elapsed_ms).max(0);
        for p in &mut self.particles {
            p.x += p.vx;
            p.y += p.vy;
            p.opacity = (p.opacity - 0.05).max(0.0);
        }
        self.remaining_ms > 0 && !self.is_undone
    }

    pub fn undo(&mut self) {
        self.is_undone = true;
    }

    pub fn seconds_left(&self) -> i32 {
        ((self.remaining_ms + 999) / 1000) as i32
    }
}

impl Default for PaidReactionToast {
    fn default() -> Self {
        Self::new(0, 50, false)
    }
}

impl Render for PaidReactionToast {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let secs = self.seconds_left();
        let label = format!("Sent ⭐ {} Stars! Undo in {}s", self.stars_count, secs);

        div()
            .flex()
            .items_center()
            .justify_between()
            .px(px(14.0))
            .py(px(8.0))
            .bg(rgba(0x1F2937EE))
            .rounded(px(20.0))
            .shadow_lg()
            .text_size(px(13.0))
            .text_color(rgb(0xFFFFFF))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .child("⭐")
                    .child(label)
            )
            .child(
                div()
                    .px(px(8.0))
                    .py(px(2.0))
                    .rounded(px(10.0))
                    .bg(rgba(0xFFFFFF22))
                    .hover(|s| s.bg(rgba(0xFFFFFF33)))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x2AABEE))
                    .child(if self.is_undone { "Undone" } else { "Undo" })
            )
    }
}
