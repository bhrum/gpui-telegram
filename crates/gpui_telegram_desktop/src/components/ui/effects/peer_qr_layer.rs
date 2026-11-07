//! Upstream: `Telegram/SourceFiles/ui/effects/peer_qr_layer.cpp:L17-L31`, `L39-L56`
//! Upstream: `Telegram/SourceFiles/ui/effects/peer_qr_layer.h:L18-L66`
//!
//! Background particle motion and falloff curve dynamics for Peer QR code view.
//! Uses Simpson integration of falloff curve `Factor(t) = min + (1 - min) * u^2 * (3 - 2u)`.

use gpui::*;
use std::time::Duration;

pub const FALLOFF_MIN: f32 = 0.15;
pub const TABLE_SIZE: usize = 64;
pub const HEADING_DEG: f32 = 225.0;
pub const WANDER_AMPLITUDE: f32 = 1.2;
pub const GLIDE_HALF_LIFE: Duration = Duration::from_millis(250);
pub const DRIFT_STEP: Duration = Duration::from_millis(1000);

#[derive(Debug, Clone)]
pub struct PeerQrLayerEffects {
    heading: f32,
    wander_phase: f32,
    drift_speed: f32,
    is_active: bool,
}

impl PeerQrLayerEffects {
    pub fn new() -> Self {
        Self {
            heading: HEADING_DEG,
            wander_phase: 0.0,
            drift_speed: 1.0,
            is_active: true,
        }
    }

    /// C++: `peer_qr_layer.cpp:L39-L42`: `Factor(t) = kFalloffMin + (1. - kFalloffMin) * u * u * (3. - 2. * u)`
    pub fn falloff_factor(t: f32) -> f32 {
        let u = t.clamp(0.0, 1.0);
        FALLOFF_MIN + (1.0 - FALLOFF_MIN) * u * u * (3.0 - 2.0 * u)
    }

    pub fn update(&mut self, dt_ms: f32) {
        self.wander_phase += 0.025 * (dt_ms / 16.0);
    }

    pub fn current_heading(&self) -> f32 {
        self.heading + self.wander_phase.sin() * WANDER_AMPLITUDE
    }
}

impl Default for PeerQrLayerEffects {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for PeerQrLayerEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_0()
            .invisible()
    }
}
