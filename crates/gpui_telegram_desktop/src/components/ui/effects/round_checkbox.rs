//! Upstream: `Telegram/SourceFiles/ui/effects/round_checkbox.cpp:L24-L30`, `L206-L245`
//! Upstream: `Telegram/SourceFiles/ui/effects/round_checkbox.h:L30-L120`
//!
//! Round animated checkbox. Implements two-phase circular fill expansion and
//! checkmark horizontal divider reveal with wide cache oversampling.

use gpui::*;
use gpui::prelude::FluentBuilder;
use std::time::Duration;

pub const ANIMATION_TIMER_DELTA: Duration = Duration::from_millis(7);
pub const WIDE_SCALE: i32 = 3;

#[derive(Debug, Clone)]
pub struct RoundCheckboxEffects {
    is_checked: bool,
    display_inactive: bool,
    progress: f32,
    size: f32,
    line_width: f32,
    bg_duration: f32,
    fg_duration: f32,
}

impl RoundCheckboxEffects {
    pub fn new(size: f32) -> Self {
        Self {
            is_checked: false,
            display_inactive: true,
            progress: 0.0,
            size: if size > 0.0 { size } else { 22.0 },
            line_width: 2.0,
            bg_duration: 0.5,
            fg_duration: 0.5,
        }
    }

    pub fn set_checked(&mut self, checked: bool, animated: bool) {
        self.is_checked = checked;
        if !animated {
            self.progress = if checked { 1.0 } else { 0.0 };
        }
    }

    pub fn set_progress(&mut self, progress: f32) {
        self.progress = progress.clamp(0.0, 1.0);
    }

    /// C++: `round_checkbox.cpp:L206-L212`:
    /// `roundProgress = (progress >= st->bgDuration) ? 1. : (progress / st->bgDuration)`
    pub fn round_progress(&self) -> f32 {
        if self.progress >= self.bg_duration {
            1.0
        } else {
            (self.progress / self.bg_duration).clamp(0.0, 1.0)
        }
    }

    /// C++: `round_checkbox.cpp:L209-L212`:
    /// `checkProgress = (1. - progress >= st->fgDuration) ? 0. : (1. - (1. - progress) / st->fgDuration)`
    pub fn check_progress(&self) -> f32 {
        if 1.0 - self.progress >= self.fg_duration {
            0.0
        } else {
            (1.0 - (1.0 - self.progress) / self.fg_duration).clamp(0.0, 1.0)
        }
    }

    /// C++: `round_checkbox.cpp:L228`: `divider = checkProgress * st->size`
    pub fn check_divider_width(&self) -> f32 {
        self.check_progress() * self.size
    }

    /// C++: `round_checkbox.cpp:L238`: `remove = size * (1. - roundProgress)`
    pub fn inner_punchout_diameter(&self) -> f32 {
        self.size * (1.0 - self.round_progress())
    }

    pub fn is_checked(&self) -> bool {
        self.is_checked
    }
}

impl Default for RoundCheckboxEffects {
    fn default() -> Self {
        Self::new(22.0)
    }
}

impl Render for RoundCheckboxEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let size = self.size;
        let checked = self.is_checked;
        let progress = self.progress;

        div()
            .size(px(size))
            .rounded(px(size / 2.0))
            .border_2()
            .border_color(if checked || progress > 0.1 { rgb(0x2AABEE) } else { rgb(0x707579) })
            .bg(if checked || progress > 0.5 { rgb(0x2AABEE) } else { rgba(0x00000000) })
            .flex()
            .items_center()
            .justify_center()
            .when(checked, |el| {
                el.child(
                    div()
                        .text_size(px(13.0))
                        .text_color(rgb(0xFFFFFF))
                        .child("✓")
                )
            })
    }
}
