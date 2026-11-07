//! Upstream: `Telegram/SourceFiles/ui/effects/peer_qr_emoji_field.cpp:L1-L525`
//! Upstream: `Telegram/SourceFiles/ui/effects/peer_qr_emoji_field.h:L20-L132`
//!
//! Custom emoji center emblem for personalized peer QR codes.
//! Embeds custom emoji document or glyph with white framing circle.

use gpui::*;

#[derive(Debug, Clone)]
pub struct PeerQrEmojiFieldEffects {
    emoji_text: String,
    center_size: f32,
    badge_radius: f32,
}

impl PeerQrEmojiFieldEffects {
    pub fn new(emoji: impl Into<String>) -> Self {
        Self {
            emoji_text: emoji.into(),
            center_size: 48.0,
            badge_radius: 24.0,
        }
    }

    pub fn set_emoji(&mut self, emoji: impl Into<String>) {
        self.emoji_text = emoji.into();
    }

    pub fn emoji(&self) -> &str {
        &self.emoji_text
    }

    pub fn center_size(&self) -> f32 {
        self.center_size
    }
}

impl Default for PeerQrEmojiFieldEffects {
    fn default() -> Self {
        Self::new("⭐")
    }
}

impl Render for PeerQrEmojiFieldEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size(px(self.center_size))
            .rounded(px(self.badge_radius))
            .bg(rgb(0xFFFFFF))
            .flex()
            .items_center()
            .justify_center()
            .text_size(px(24.0))
            .child(self.emoji_text.clone())
    }
}
