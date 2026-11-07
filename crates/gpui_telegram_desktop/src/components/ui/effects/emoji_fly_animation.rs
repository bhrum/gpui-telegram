//! Upstream: `Telegram/SourceFiles/ui/effects/emoji_fly_animation.cpp:L20-L28`, `L32-L75`
//! Upstream: `Telegram/SourceFiles/ui/effects/emoji_fly_animation.h:L18-L45`
//!
//! Flying custom emoji reaction animation overlay.
//! Computes frame size from emoji tag and animates trajectory toward peer badge.

use gpui::*;
use gpui::prelude::FluentBuilder;

#[derive(Debug, Clone)]
pub struct EmojiFlyAnimationEffects {
    fly_size: f32,
    progress: f32,
    start_pos: (f32, f32),
    target_pos: (f32, f32),
    is_finished: bool,
}

impl EmojiFlyAnimationEffects {
    pub fn new(fly_size: f32) -> Self {
        Self {
            fly_size: if fly_size > 0.0 { fly_size } else { 24.0 },
            progress: 0.0,
            start_pos: (0.0, 0.0),
            target_pos: (0.0, 0.0),
            is_finished: true,
        }
    }

    /// C++: `emoji_fly_animation.cpp:L20-L28`: `ComputeFlySize`
    pub fn compute_fly_size(normal_size: f32, frame_size: f32, normal_frame_size: f32) -> f32 {
        if (normal_frame_size - frame_size).abs() < 0.01 {
            normal_size
        } else {
            normal_size * (frame_size / normal_frame_size)
        }
    }

    pub fn start_flight(&mut self, from: (f32, f32), to: (f32, f32)) {
        self.start_pos = from;
        self.target_pos = to;
        self.progress = 0.0;
        self.is_finished = false;
    }

    pub fn set_progress(&mut self, progress: f32) {
        self.progress = progress.clamp(0.0, 1.0);
        if self.progress >= 1.0 {
            self.is_finished = true;
        }
    }

    pub fn current_pos(&self) -> (f32, f32) {
        let t = self.progress;
        let x = self.start_pos.0 + (self.target_pos.0 - self.start_pos.0) * t;
        let y = self.start_pos.1 + (self.target_pos.1 - self.start_pos.1) * t;
        (x, y)
    }

    pub fn is_finished(&self) -> bool {
        self.is_finished
    }

    pub fn fly_size(&self) -> f32 {
        self.fly_size
    }
}

impl Default for EmojiFlyAnimationEffects {
    fn default() -> Self {
        Self::new(24.0)
    }
}

impl Render for EmojiFlyAnimationEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let (x, y) = self.current_pos();
        div()
            .relative()
            .size_full()
            .when(!self.is_finished, |el| {
                el.child(
                    div()
                        .absolute()
                        .left(px(x))
                        .top(px(y))
                        .size(px(self.fly_size))
                        .rounded(px(self.fly_size / 2.0))
                        .bg(rgb(0xFFD700))
                )
            })
    }
}
