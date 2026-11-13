//! Upstream: `Telegram/SourceFiles/ui/effects/glare.cpp:L14-L78`
//! Upstream: `Telegram/SourceFiles/ui/effects/glare.h:L10-L32`
//!
//! Shimmer glare effect for gradient buttons and banners.
//! Generates a linear gradient sweep with max opacity 0.5 at the center.

use gpui::*;

pub const MAX_GLARE_OPAQUE: f32 = 0.5;

#[derive(Debug, Clone)]
pub struct GlareEffects {
    width: f32,
    progress: f32,
    is_paused: bool,
    glare_color: u32,
}

impl GlareEffects {
    pub fn new(width: f32) -> Self {
        Self {
            width: if width > 0.0 { width } else { 120.0 },
            progress: 0.0,
            is_paused: false,
            glare_color: 0xFFFFFF,
        }
    }

    /// C++: `glare.cpp:L19-L22`: `progress(now) = (now - birthTime) / float64(deathTime - birthTime)`
    pub fn set_progress(&mut self, progress: f32) {
        self.progress = progress.clamp(0.0, 1.0);
    }

    /// C++: `glare.cpp:L24-L40`: compute gradient stops with 0.5 center alpha
    pub fn alpha_at_offset(&self, relative_x: f32) -> f32 {
        let x = relative_x.clamp(0.0, 1.0);
        if x <= 0.5 {
            (x / 0.5) * MAX_GLARE_OPAQUE
        } else {
            ((1.0 - x) / 0.5) * MAX_GLARE_OPAQUE
        }
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.is_paused = paused;
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn progress(&self) -> f32 {
        self.progress
    }
}

impl Default for GlareEffects {
    fn default() -> Self {
        Self::new(120.0)
    }
}

impl Render for GlareEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let offset = self.progress * self.width;
        div()
            .relative()
            .w(px(self.width))
            .h_full()
            .overflow_hidden()
            .child(
                div()
                    .absolute()
                    .left(px(offset - 40.0))
                    .w(px(40.0))
                    .h_full()
                    .bg(rgba(0xFFFFFF33))
            )
    }
}
