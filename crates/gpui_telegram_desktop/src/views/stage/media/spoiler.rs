//! Authentic GPUI Media Spoiler & Paid/Sensitive Content Overlay.
//!
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_media_spoiler.h`
//! - `Telegram/SourceFiles/history/view/media/history_view_media.h`
//! - `Telegram/SourceFiles/history/view/media/history_view_media.cpp:L346-L476`

use gpui::*;
use gpui::prelude::FluentBuilder;

pub const SPOILER_FRAME_COUNT: usize = 60;
pub const SPOILER_PARTICLE_COUNT: usize = 64;

#[derive(Debug, Clone)]
pub struct SpoilerParticle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub alpha: f32,
    pub size: f32,
}

#[derive(Debug, Clone)]
pub struct MediaSpoilerState {
    pub is_revealed: bool,
    pub reveal_progress: f32, // 0.0 (hidden) .. 1.0 (fully revealed)
    pub current_frame: usize,
    pub particles: Vec<SpoilerParticle>,
}

impl Default for MediaSpoilerState {
    fn default() -> Self {
        let mut particles = Vec::with_capacity(SPOILER_PARTICLE_COUNT);
        for i in 0..SPOILER_PARTICLE_COUNT {
            let fi = i as f32;
            let x = ((fi * 17.3 + 5.1).sin() * 0.5 + 0.5).clamp(0.0, 1.0);
            let y = ((fi * 23.7 + 11.4).cos() * 0.5 + 0.5).clamp(0.0, 1.0);
            let vx = ((fi * 7.1 + 1.7).sin() * 0.02).clamp(-0.02, 0.02);
            let vy = ((fi * 11.3).cos() * 0.02).clamp(-0.02, 0.02);
            let alpha = 0.3 + 0.6 * ((fi * 13.9).sin() * 0.5 + 0.5);
            let size = 1.5 + 2.0 * ((fi * 19.1).cos() * 0.5 + 0.5);
            particles.push(SpoilerParticle { x, y, vx, vy, alpha, size });
        }
        Self {
            is_revealed: false,
            reveal_progress: 0.0,
            current_frame: 0,
            particles,
        }
    }
}

impl MediaSpoilerState {
    pub fn advance_frame(&mut self) {
        if self.is_revealed && self.reveal_progress < 1.0 {
            self.reveal_progress = (self.reveal_progress + 0.1).min(1.0);
        }
        self.current_frame = (self.current_frame + 1) % SPOILER_FRAME_COUNT;
        for p in &mut self.particles {
            p.x = (p.x + p.vx).rem_euclid(1.0);
            p.y = (p.y + p.vy).rem_euclid(1.0);
        }
    }

    pub fn opacity(&self) -> f32 {
        (1.0 - self.reveal_progress).clamp(0.0, 1.0)
    }
}

#[derive(Debug, Clone)]
pub struct MediaPaidTag {
    pub price: u64,
}

#[derive(Debug, Clone)]
pub struct MediaSensitiveTag {
    pub is_sensitive: bool,
}

pub struct MediaSpoilerView;

impl MediaSpoilerView {
    pub fn render(
        state: &MediaSpoilerState,
        paid_tag: Option<&MediaPaidTag>,
        sensitive_tag: Option<&MediaSensitiveTag>,
        width: f32,
        height: f32,
    ) -> impl IntoElement {
        let opacity = state.opacity();
        if opacity <= 0.0 {
            return div().w(px(0.0)).h(px(0.0));
        }

        let badge_label = if let Some(sensitive) = sensitive_tag {
            if sensitive.is_sensitive {
                Some("👁️ Sensitive Content".to_string())
            } else {
                None
            }
        } else if let Some(paid) = paid_tag {
            Some(format!("⭐ {} Stars", paid.price))
        } else {
            Some("✨ Tap to reveal".to_string())
        };

        div()
            .absolute()
            .inset_0()
            .w(px(width))
            .h(px(height))
            .bg(rgba(0x000000CC))
            .opacity(opacity)
            .flex()
            .items_center()
            .justify_center()
            .when_some(badge_label, |el, label| {
                el.child(
                    div()
                        .px(px(12.0))
                        .py(px(6.0))
                        .rounded_full()
                        .bg(rgba(0x2B5278EE))
                        .text_size(px(12.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0xFFFFFF))
                        .child(label),
                )
            })
    }
}
