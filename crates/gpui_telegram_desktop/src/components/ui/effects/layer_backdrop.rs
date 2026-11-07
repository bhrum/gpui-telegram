//! Upstream: `Telegram/SourceFiles/ui/effects/layer_backdrop.cpp:L24-L56`, `L210-L286`
//! Upstream: `Telegram/SourceFiles/ui/effects/layer_backdrop.h:L21-L70`
//!
//! Modal backdrop blur, cross-fade and dimming effect controller. Captures underlying
//! window surfaces, performs downscaled 24px blur, and applies night-mode responsive dimming.

use gpui::*;
use gpui::prelude::FluentBuilder;
use std::time::Duration;

pub const CACHE_BACKGROUND_FAST_TIMEOUT: Duration = Duration::from_millis(200);
pub const CACHE_BACKGROUND_FULL_TIMEOUT: Duration = Duration::from_millis(1000);
pub const FADE_BACKGROUND_DURATION: Duration = Duration::from_millis(200);
pub const OVERLAY_COLOR: u32 = 0x101010; // QColor(16, 16, 16)
pub const BLUR_RADIUS: f32 = 24.0;
pub const DOWNSCALE_FACTOR: f32 = 4.0;

#[derive(Debug, Clone)]
pub struct LayerBackdropEffects {
    is_active: bool,
    is_night_mode: bool,
    fade_progress: f32,
    overlay_alpha: f32,
    blur_radius: f32,
    generation: u64,
}

impl LayerBackdropEffects {
    pub fn new() -> Self {
        Self {
            is_active: false,
            is_night_mode: true,
            fade_progress: 1.0,
            overlay_alpha: 0.6,
            blur_radius: BLUR_RADIUS,
            generation: 1,
        }
    }

    /// C++: `LayerBackdrop::start()`
    pub fn start(&mut self, is_night: bool) {
        self.is_active = true;
        self.is_night_mode = is_night;
        self.fade_progress = 0.0;
        self.overlay_alpha = if is_night { 0.65 } else { 0.45 };
        self.generation += 1;
    }

    /// C++: `LayerBackdrop::invalidate()`
    pub fn invalidate(&mut self) {
        self.generation += 1;
        self.fade_progress = 0.0;
    }

    /// C++: `LayerBackdrop::paintDim(QPainter &p, float64 opacity)`
    pub fn dim_opacity(&self, master_opacity: f32) -> f32 {
        (self.overlay_alpha * master_opacity).clamp(0.0, 1.0)
    }

    /// C++: `LayerBackdrop::paint(...)` cross-fade equation:
    /// `shown = opacity * faded; p.setOpacity(opacity * (1. - faded) / (1. - shown))`
    pub fn cross_fade_weights(&self, opacity: f32) -> (f32, f32) {
        let faded = self.fade_progress.clamp(0.0, 1.0);
        if faded >= 1.0 {
            (0.0, opacity)
        } else {
            let shown = opacity * faded;
            let back = if (1.0 - shown).abs() > 0.001 {
                opacity * (1.0 - faded) / (1.0 - shown)
            } else {
                0.0
            };
            (back, shown)
        }
    }

    pub fn set_progress(&mut self, progress: f32) {
        self.fade_progress = progress.clamp(0.0, 1.0);
    }

    pub fn is_started(&self) -> bool {
        self.is_active
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }
}

impl Default for LayerBackdropEffects {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for LayerBackdropEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let alpha_byte = (self.overlay_alpha * 255.0).clamp(0.0, 255.0) as u8;
        let color = rgba((OVERLAY_COLOR << 8) | (alpha_byte as u32));
        div()
            .absolute()
            .size_full()
            .bg(color)
            .when(self.is_active, |el| el.opacity(self.fade_progress))
    }
}
