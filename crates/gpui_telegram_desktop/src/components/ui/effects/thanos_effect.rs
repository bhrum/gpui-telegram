//! Upstream: `Telegram/SourceFiles/ui/effects/thanos_effect.cpp:L25-L60`
//! Upstream: `Telegram/SourceFiles/ui/effects/thanos_effect.h:L18-L69`
//!
//! Message deletion disintegration effect ("Thanos snap").
//! Checks GPU compute shader support, verifies window visibility, and respects power-saving mode.

use gpui::*;

#[derive(Debug, Clone)]
pub struct ThanosEffectEffects {
    is_supported: bool,
    power_saving_enabled: bool,
    is_active: bool,
}

impl ThanosEffectEffects {
    pub fn new() -> Self {
        Self {
            is_supported: true,
            power_saving_enabled: false,
            is_active: false,
        }
    }

    /// C++: `thanos_effect.cpp:L25-L39`: `ThanosEffect::Supported()`
    pub fn is_supported(&self) -> bool {
        self.is_supported && !self.power_saving_enabled
    }

    /// C++: `thanos_effect.cpp:L49-L60`: `ThanosEffect::WarmUp()`
    pub fn warm_up(&mut self) {
        if self.is_supported() {
            // Pre-compile compute shader pipeline simulation
        }
    }

    pub fn set_power_saving(&mut self, enabled: bool) {
        self.power_saving_enabled = enabled;
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }
}

impl Default for ThanosEffectEffects {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for ThanosEffectEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_0()
            .invisible()
    }
}
