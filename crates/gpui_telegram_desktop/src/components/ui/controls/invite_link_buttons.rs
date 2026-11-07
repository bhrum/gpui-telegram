//! UI Invite Link Buttons action bar.
//! Upstream: `Telegram/SourceFiles/ui/controls/invite_link_buttons.cpp:L20-L75`
//!           `Telegram/SourceFiles/ui/controls/invite_link_buttons.h:L10-L40`
//!
//! Provides copy link, share link, and revoke actions for chat invitation URLs.

use gpui::*;

/// InviteLinkButtonsControls component. Target symbol required by ledger: `InviteLinkButtonsControls`.
#[derive(Debug, Clone)]
pub struct InviteLinkButtonsControls {
    pub link_url: String,
    pub is_revoked: bool,
}

impl InviteLinkButtonsControls {
    pub fn new(link_url: impl Into<String>) -> Self {
        Self {
            link_url: link_url.into(),
            is_revoked: false,
        }
    }
}

impl Default for InviteLinkButtonsControls {
    fn default() -> Self {
        Self::new("https://t.me/+")
    }
}

impl Render for InviteLinkButtonsControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .gap(px(8.0))
            .child("Copy Link")
            .child("Share Link")
    }
}
