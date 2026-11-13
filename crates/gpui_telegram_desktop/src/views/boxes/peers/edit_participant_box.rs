//! Edit Participant Permissions and Admin Roles Dialog Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peers/edit_participant_box.h:L1-L150`
//! - `Telegram/SourceFiles/boxes/peers/edit_participant_box.cpp:L1-L1279`
//! - `Telegram/SourceFiles/boxes/peers/edit_participant_box.cpp:L50-L200` (Admin role flags & custom title)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

pub const K_MAX_ADMIN_RANK_LENGTH: usize = 16;

/// Granular administrative rights assigned to a participant.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AdminRights {
    pub change_info: bool,
    pub post_messages: bool,
    pub edit_messages: bool,
    pub delete_messages: bool,
    pub ban_users: bool,
    pub invite_users: bool,
    pub pin_messages: bool,
    pub add_admins: bool,
    pub anonymous: bool,
    pub manage_video_chats: bool,
    pub manage_topics: bool,
}

/// Target symbol in ledger: `EditParticipantBoxPeers`.
#[derive(Debug, Clone)]
pub struct EditParticipantBoxPeers {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub chat_id: u64,
    pub user_id: u64,
    pub is_admin: bool,
    pub custom_title: String,
    pub rights: AdminRights,
}

impl Default for EditParticipantBoxPeers {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl EditParticipantBoxPeers {
    pub fn new(chat_id: u64, user_id: u64) -> Self {
        Self {
            is_visible: true,
            width: px(400.0),
            height: px(520.0),
            chat_id,
            user_id,
            is_admin: false,
            custom_title: String::new(),
            rights: AdminRights::default(),
        }
    }

    pub fn set_custom_title(&mut self, title: &str) {
        self.custom_title = title.chars().take(K_MAX_ADMIN_RANK_LENGTH).collect();
    }

    pub fn promote_to_admin(&mut self, rights: AdminRights) {
        self.is_admin = true;
        self.rights = rights;
    }

    pub fn demote_from_admin(&mut self) {
        self.is_admin = false;
        self.custom_title.clear();
        self.rights = AdminRights::default();
    }
}
