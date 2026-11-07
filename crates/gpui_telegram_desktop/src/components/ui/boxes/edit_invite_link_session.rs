//! Authentic GPUI Invite Link Star Subscriptions Session Component.
//! Reference: Telegram/SourceFiles/ui/boxes/edit_invite_link_session.cpp:L20-L154
//! Reference: Telegram/SourceFiles/ui/boxes/edit_invite_link_session.h:L15-L23

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Target symbol for invite link monthly Star subscription controller.
/// Reference: `edit_invite_link_session.h:L15-L23`
#[derive(Debug, Clone)]
pub struct EditInviteLinkSessionBoxes {
    pub subscription_enabled: bool,
    pub monthly_credits_amount: i32,
    pub min_credits: i32,
    pub max_credits: i32,
    pub peer_can_monetize: bool,
}

impl Default for EditInviteLinkSessionBoxes {
    fn default() -> Self {
        Self::new(true)
    }
}

impl EditInviteLinkSessionBoxes {
    pub fn new(peer_can_monetize: bool) -> Self {
        Self {
            subscription_enabled: false,
            monthly_credits_amount: 250, // 250 Stars default
            min_credits: 1,
            max_credits: 100_000,
            peer_can_monetize,
        }
    }

    /// Toggles recurring monthly Star subscription requirement.
    /// Reference: `edit_invite_link_session.cpp:L50-L75`
    pub fn toggle_subscription(&mut self, enabled: bool) -> bool {
        if self.peer_can_monetize {
            self.subscription_enabled = enabled;
            true
        } else {
            self.subscription_enabled = false;
            false
        }
    }

    pub fn set_monthly_amount(&mut self, amount: i32) {
        self.monthly_credits_amount = amount.clamp(self.min_credits, self.max_credits);
    }
}
