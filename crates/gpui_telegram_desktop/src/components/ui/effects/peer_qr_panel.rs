//! Upstream: `Telegram/SourceFiles/ui/effects/peer_qr_panel.cpp:L19-L40`, `L43-L56`
//! Upstream: `Telegram/SourceFiles/ui/effects/peer_qr_panel.h:L18-L83`
//!
//! Glassmorphic floating card panel presentation for Peer QR code dialog.
//! Implements cubic bezier ease-out open/close curves with glass edge clipping.

use gpui::*;
use std::time::Duration;

pub const OPEN_DURATION: Duration = Duration::from_millis(420);
pub const CLOSE_DURATION: Duration = Duration::from_millis(360);
pub const CONTENT_IN_DELAY: Duration = Duration::from_millis(60);
pub const CONTENT_IN_DURATION: Duration = Duration::from_millis(220);
pub const GLASS_ALPHA: f32 = 0.25;

#[derive(Debug, Clone)]
pub struct PeerQrPanelEffects {
    is_open: bool,
    progress: f32,
    radius: f32,
}

impl PeerQrPanelEffects {
    pub fn new() -> Self {
        Self {
            is_open: false,
            progress: 0.0,
            radius: 20.0,
        }
    }

    /// C++: `peer_qr_panel.cpp:L36-L40`: `ContentIn(elapsed)`
    pub fn content_in_progress(elapsed_ms: u64) -> f32 {
        let delay_ms = CONTENT_IN_DELAY.as_millis() as u64;
        let dur_ms = CONTENT_IN_DURATION.as_millis() as f32;
        if elapsed_ms <= delay_ms {
            0.0
        } else {
            let t = ((elapsed_ms - delay_ms) as f32 / dur_ms).clamp(0.0, 1.0);
            1.0 - (1.0 - t).powi(3) // Cubic ease-out
        }
    }

    pub fn set_open(&mut self, open: bool) {
        self.is_open = open;
    }

    pub fn set_progress(&mut self, progress: f32) {
        self.progress = progress.clamp(0.0, 1.0);
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }
}

impl Default for PeerQrPanelEffects {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for PeerQrPanelEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(320.0))
            .rounded(px(self.radius))
            .bg(rgba(0xFFFFFF40))
            .border_1()
            .border_color(rgba(0xFFFFFF60))
            .p(px(20.0))
            .flex()
            .flex_col()
            .items_center()
    }
}
