//! Upstream: `Telegram/SourceFiles/ui/effects/reaction_fly_animation.cpp:L23-L31`, `L65-L100`
//! Upstream: `Telegram/SourceFiles/ui/effects/reaction_fly_animation.h:L30-L140`
//!
//! Message reaction flying animation with 7 sparkling mini copies.
//! Manages main reaction flight path and trailing particle cluster.

use gpui::*;
use std::time::Duration;

pub const FLY_DURATION: Duration = Duration::from_millis(300);
pub const MINI_COPIES_COUNT: usize = 7;
pub const MINI_COPIES_DURATION_MAX: Duration = Duration::from_millis(1400);
pub const MINI_COPIES_DURATION_MIN: Duration = Duration::from_millis(700);
pub const MINI_COPIES_SCALE_IN: Duration = Duration::from_millis(200);
pub const MINI_COPIES_SCALE_OUT: Duration = Duration::from_millis(200);

#[derive(Debug, Clone)]
pub struct MiniCopyParticle {
    pub offset_x: f32,
    pub offset_y: f32,
    pub scale: f32,
    pub alpha: f32,
}

#[derive(Debug, Clone)]
pub struct ReactionFlyAnimationEffects {
    reaction_id: String,
    progress: f32,
    size: f32,
    mini_copies: Vec<MiniCopyParticle>,
    is_finished: bool,
}

impl ReactionFlyAnimationEffects {
    pub fn new(reaction_id: impl Into<String>, size: f32) -> Self {
        let mut mini_copies = Vec::with_capacity(MINI_COPIES_COUNT);
        for i in 0..MINI_COPIES_COUNT {
            let angle = (i as f32) * (std::f32::consts::PI * 2.0 / MINI_COPIES_COUNT as f32);
            mini_copies.push(MiniCopyParticle {
                offset_x: angle.cos() * 16.0,
                offset_y: angle.sin() * 16.0,
                scale: 0.75,
                alpha: 0.8,
            });
        }

        Self {
            reaction_id: reaction_id.into(),
            progress: 0.0,
            size: if size > 0.0 { size } else { 28.0 },
            mini_copies,
            is_finished: false,
        }
    }

    pub fn set_progress(&mut self, progress: f32) {
        self.progress = progress.clamp(0.0, 1.0);
        if self.progress >= 1.0 {
            self.is_finished = true;
        }
        for (i, copy) in self.mini_copies.iter_mut().enumerate() {
            let delay = (i as f32) * 0.05;
            let local_t = ((self.progress - delay) / 0.7).clamp(0.0, 1.0);
            copy.alpha = 1.0 - local_t;
            copy.scale = 0.6 + 0.3 * local_t;
        }
    }

    pub fn is_finished(&self) -> bool {
        self.is_finished
    }

    pub fn reaction_id(&self) -> &str {
        &self.reaction_id
    }

    pub fn mini_copies(&self) -> &[MiniCopyParticle] {
        &self.mini_copies
    }
}

impl Default for ReactionFlyAnimationEffects {
    fn default() -> Self {
        Self::new("👍", 28.0)
    }
}

impl Render for ReactionFlyAnimationEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let size = self.size;
        div()
            .relative()
            .size(px(size))
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .text_size(px(size * 0.8))
                    .child(self.reaction_id.clone())
            )
    }
}
