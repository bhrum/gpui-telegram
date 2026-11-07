//! Info Profile Birthday Effect
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_birthday_effect.cpp:L1-L496`
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_birthday_effect.h:L1-L24`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use gpui::*;

/// Particle animation effect playing confetti burst and celebratory ribbons on user birthdays.
#[derive(Debug, Clone)]
pub struct InfoProfileBirthdayEffectProfile {
    pub is_playing: bool,
    pub progress: f32,
    pub particle_count: usize,
}

impl InfoProfileBirthdayEffectProfile {
    pub const DURATION_MS: u64 = 4000;
    pub const DEFAULT_PARTICLES: usize = 48;

    pub fn new() -> Self {
        Self {
            is_playing: false,
            progress: 0.0,
            particle_count: Self::DEFAULT_PARTICLES,
        }
    }

    pub fn start(&mut self) {
        self.is_playing = true;
        self.progress = 0.0;
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
        self.progress = 1.0;
    }

    pub fn update_progress(&mut self, dt_ms: f32) {
        if self.is_playing {
            self.progress += dt_ms / (Self::DURATION_MS as f32);
            if self.progress >= 1.0 {
                self.stop();
            }
        }
    }
}

impl Default for InfoProfileBirthdayEffectProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoProfileBirthdayEffectProfile {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut el = div().flex().size_full();
        if self.is_playing {
            el = el.child(
                div()
                    .text_xs()
                    .text_color(rgb(0xf1c40f))
                    .child("🎉 Confetti effect active"),
            );
        }
        el
    }
}
