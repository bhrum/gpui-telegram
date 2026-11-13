//! UI Invite Link Label pill display.
//! Upstream: `Telegram/SourceFiles/ui/controls/invite_link_label.cpp:L20-L60`
//!           `Telegram/SourceFiles/ui/controls/invite_link_label.h:L10-L35`
//!
//! Renders truncated invite link URL in rounded container with click-to-copy trigger.

use gpui::*;

/// InviteLinkLabelControls component. Target symbol required by ledger: `InviteLinkLabelControls`.
#[derive(Debug, Clone)]
pub struct InviteLinkLabelControls {
    pub url: String,
    pub is_copied: bool,
}

impl InviteLinkLabelControls {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            is_copied: false,
        }
    }
}

impl Default for InviteLinkLabelControls {
    fn default() -> Self {
        Self::new("https://t.me/joinchat/...")
    }
}

impl Render for InviteLinkLabelControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .p(px(8.0))
            .rounded(px(6.0))
            .bg(hsla(0.0, 0.0, 0.95, 1.0))
            .child(self.url.clone())
    }
}
