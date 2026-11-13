//! Info Profile Badge component
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_badge.cpp:L1-L322`
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_badge.h:L1-L107`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeerBadgeKind {
    Verified,
    Premium,
    BotVerification,
    Scam,
    Fake,
}

/// Peer status badge rendered adjacent to display names in profile cards.
#[derive(Debug, Clone)]
pub struct InfoProfileBadgeProfile {
    pub kind: PeerBadgeKind,
    pub custom_emoji_id: Option<u64>,
    pub tooltip_text: String,
}

impl InfoProfileBadgeProfile {
    pub fn new(kind: PeerBadgeKind) -> Self {
        let tooltip = match kind {
            PeerBadgeKind::Verified => "Verified account",
            PeerBadgeKind::Premium => "Telegram Premium subscriber",
            PeerBadgeKind::BotVerification => "Verified bot",
            PeerBadgeKind::Scam => "Warning: Scam account",
            PeerBadgeKind::Fake => "Warning: Impersonator / Fake",
        };
        Self {
            kind,
            custom_emoji_id: None,
            tooltip_text: tooltip.to_string(),
        }
    }

    pub fn with_custom_emoji(emoji_id: u64) -> Self {
        Self {
            kind: PeerBadgeKind::Premium,
            custom_emoji_id: Some(emoji_id),
            tooltip_text: "Custom Emoji Status".to_string(),
        }
    }
}

impl Default for InfoProfileBadgeProfile {
    fn default() -> Self {
        Self::new(PeerBadgeKind::Verified)
    }
}

impl Render for InfoProfileBadgeProfile {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let (icon, color) = match self.kind {
            PeerBadgeKind::Verified => ("✓", rgb(0x2ea6ff)),
            PeerBadgeKind::Premium => ("★", rgb(0x9b59b6)),
            PeerBadgeKind::BotVerification => ("⚙", rgb(0x2ea6ff)),
            PeerBadgeKind::Scam => ("!", rgb(0xe74c3c)),
            PeerBadgeKind::Fake => ("⚠", rgb(0xe67e22)),
        };

        div()
            .flex()
            .items_center()
            .justify_center()
            .w(px(16.0))
            .h(px(16.0))
            .rounded_full()
            .text_xs()
            .font_weight(FontWeight::BOLD)
            .text_color(color)
            .child(icon)
    }
}
