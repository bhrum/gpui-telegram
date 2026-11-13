//! Upstream: `Telegram/SourceFiles/ui/effects/thanos_effect_session.cpp:L1-L33`
//! Upstream: `Telegram/SourceFiles/ui/effects/thanos_effect_session.h:L10-L20`
//!
//! Session context lifecycle attachment for message disintegration instances.

use gpui::*;

#[derive(Debug, Clone)]
pub struct ThanosEffectSessionEffects {
    session_id: u64,
    is_active: bool,
}

impl ThanosEffectSessionEffects {
    pub fn new(session_id: u64) -> Self {
        Self {
            session_id,
            is_active: true,
        }
    }

    pub fn destroy(&mut self) {
        self.is_active = false;
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn session_id(&self) -> u64 {
        self.session_id
    }
}

impl Default for ThanosEffectSessionEffects {
    fn default() -> Self {
        Self::new(1)
    }
}

impl Render for ThanosEffectSessionEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_0()
            .invisible()
    }
}
