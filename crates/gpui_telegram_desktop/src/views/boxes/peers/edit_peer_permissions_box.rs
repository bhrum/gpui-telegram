//! Default Chat Permissions and Slow Mode Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/edit_peer_permissions_box.h:L1-L135`
//! - `Telegram/SourceFiles/boxes/peers/edit_peer_permissions_box.cpp:L1-L1571`
//! - `edit_peer_permissions_box.cpp:L50-L150` (Chat permissions checkboxes)
//! - `edit_peer_permissions_box.cpp:L200-L240` (Slow mode slider steps)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

pub const K_SLOWMODE_SECONDS: [u32; 7] = [0, 10, 30, 60, 300, 900, 3600];

/// Default member permissions for a group chat.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatPermissions {
    pub send_messages: bool,
    pub send_media: bool,
    pub send_stickers_and_gifs: bool,
    pub send_polls: bool,
    pub embed_links: bool,
    pub add_users: bool,
    pub pin_messages: bool,
    pub change_chat_info: bool,
}

impl Default for ChatPermissions {
    fn default() -> Self {
        Self {
            send_messages: true,
            send_media: true,
            send_stickers_and_gifs: true,
            send_polls: true,
            embed_links: true,
            add_users: true,
            pin_messages: false,
            change_chat_info: false,
        }
    }
}

/// Target symbol in ledger: `EditPeerPermissionsBoxPeers`.
#[derive(Debug, Clone)]
pub struct EditPeerPermissionsBoxPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub chat_id: u64,
    pub permissions: ChatPermissions,
    pub slowmode_seconds: u32,
    pub is_submitting: bool,
}

impl Default for EditPeerPermissionsBoxPeers {
    fn default() -> Self {
        Self::new(0)
    }
}

impl EditPeerPermissionsBoxPeers {
    pub fn new(chat_id: u64) -> Self {
        Self {
            is_visible: true,
            width: px(400.0),
            height: px(520.0),
            chat_id,
            permissions: ChatPermissions::default(),
            slowmode_seconds: 0,
            is_submitting: false,
        }
    }

    pub fn set_slowmode(&mut self, seconds: u32) {
        if K_SLOWMODE_SECONDS.contains(&seconds) {
            self.slowmode_seconds = seconds;
        }
    }
}
