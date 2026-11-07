//! Upstream: `Telegram/SourceFiles/ui/effects/peer_qr_gift_field.cpp:L1-L426`
//! Upstream: `Telegram/SourceFiles/ui/effects/peer_qr_gift_field.h:L20-L100`
//!
//! Star gift emblem badge rendered at the center of peer QR code.

use gpui::*;

#[derive(Debug, Clone)]
pub struct PeerQrGiftFieldEffects {
    gift_name: String,
    stars_value: u64,
    badge_size: f32,
}

impl PeerQrGiftFieldEffects {
    pub fn new(gift_name: impl Into<String>, stars: u64) -> Self {
        Self {
            gift_name: gift_name.into(),
            stars_value: stars,
            badge_size: 48.0,
        }
    }

    pub fn stars(&self) -> u64 {
        self.stars_value
    }

    pub fn gift_name(&self) -> &str {
        &self.gift_name
    }
}

impl Default for PeerQrGiftFieldEffects {
    fn default() -> Self {
        Self::new("Golden Star", 100)
    }
}

impl Render for PeerQrGiftFieldEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size(px(self.badge_size))
            .rounded(px(self.badge_size / 2.0))
            .bg(rgb(0xFFD700))
            .flex()
            .items_center()
            .justify_center()
            .text_size(px(18.0))
            .child("🎁")
    }
}
