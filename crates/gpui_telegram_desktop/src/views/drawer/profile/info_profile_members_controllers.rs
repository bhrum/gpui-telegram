//! Info Profile Members Controllers & Member List Rows
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_members_controllers.cpp`
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_members_controllers.h`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use serde::{Deserialize, Serialize};

/// Upstream `MemberListRow::Rights`: role hierarchy in group and channel membership.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rights {
    Normal,
    Admin,
    Creator,
}

/// Upstream `MemberListRow::TagMode`: rendering mode for member rank / admin pills.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TagMode {
    None,
    AdminPill,
    NormalText,
    AddTag,
}

/// Upstream `MemberListRow::Type`: metadata describing member role, rank, and permissions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemberRowType {
    pub rights: Rights,
    pub rank: String,
    pub remove_text: String,
    pub can_add_tag: bool,
    pub can_edit_tag: bool,
    pub can_remove: bool,
}

impl Default for MemberRowType {
    fn default() -> Self {
        Self {
            rights: Rights::Normal,
            rank: String::new(),
            remove_text: String::new(),
            can_add_tag: false,
            can_edit_tag: false,
            can_remove: false,
        }
    }
}

/// Upstream `MemberListRow`: row element in the group/channel participants list.
#[derive(Debug, Clone)]
pub struct MemberListRow {
    pub peer_id: i64,
    pub user_id: i64,
    pub name: String,
    pub username: Option<String>,
    pub row_type: MemberRowType,
    pub tag_mode: TagMode,
    pub tag_text: String,
    pub remove_text: String,
}

impl MemberListRow {
    pub const TAG_ELEMENT: usize = 1;
    pub const REMOVE_ELEMENT: usize = 2;

    pub fn new(peer_id: i64, user_id: i64, name: impl Into<String>, row_type: MemberRowType) -> Self {
        let mut row = Self {
            peer_id,
            user_id,
            name: name.into(),
            username: None,
            row_type: row_type.clone(),
            tag_mode: TagMode::None,
            tag_text: String::new(),
            remove_text: String::new(),
        };
        row.set_type(row_type);
        row
    }

    /// Upstream `MemberListRow::setType()` state machine.
    pub fn set_type(&mut self, row_type: MemberRowType) {
        self.row_type = row_type;
        if self.row_type.can_add_tag {
            self.tag_mode = TagMode::AddTag;
            self.tag_text = "Add Tag".to_string(); // lng_context_add_my_tag
        } else if !self.row_type.rank.is_empty() {
            self.tag_mode = match self.row_type.rights {
                Rights::Admin | Rights::Creator => TagMode::AdminPill,
                Rights::Normal => TagMode::NormalText,
            };
            self.tag_text = self.row_type.rank.clone();
        } else if self.row_type.rights == Rights::Creator {
            self.tag_mode = TagMode::AdminPill;
            self.tag_text = "owner".to_string(); // lng_owner_badge
        } else if self.row_type.rights == Rights::Admin {
            self.tag_mode = TagMode::AdminPill;
            self.tag_text = "admin".to_string(); // lng_admin_badge
        } else {
            self.tag_mode = TagMode::None;
            self.tag_text.clear();
        }

        if self.row_type.can_remove {
            self.remove_text = if self.row_type.remove_text.is_empty() {
                "Remove".to_string() // lng_profile_kick
            } else {
                self.row_type.remove_text.clone()
            };
        } else {
            self.remove_text.clear();
        }
    }

    pub fn tag_interactive(&self) -> bool {
        self.row_type.can_edit_tag || self.row_type.can_add_tag
    }

    /// Upstream `MemberListRow::tagSize()`: capsule padding enforcing `max(w, h)`.
    pub fn tag_pill_size(&self, text_width: f32, font_height: f32) -> (f32, f32) {
        if self.tag_text.is_empty() {
            return (0.0, 0.0);
        }
        let padding_h = 10.0;
        let padding_v = 4.0;
        let h = font_height + padding_v * 2.0;
        let w = (text_width + padding_h * 2.0).max(h);
        (w, h)
    }
}

/// Upstream `Info::Profile::InfoProfileMembersControllersProfile`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoProfileMembersControllersProfile {
    pub channel_id: i64,
    pub can_add_members: bool,
    pub can_change_info: bool,
    pub can_delete_messages: bool,
    pub can_ban_users: bool,
    pub can_invite_users: bool,
    pub can_pin_messages: bool,
    pub can_post_stories: bool,
}

impl InfoProfileMembersControllersProfile {
    pub const TAG_ELEMENT: usize = 1;
    pub const REMOVE_ELEMENT: usize = 2;

    pub fn new(channel_id: i64) -> Self {
        Self {
            channel_id,
            can_add_members: true,
            can_change_info: false,
            can_delete_messages: false,
            can_ban_users: false,
            can_invite_users: true,
            can_pin_messages: false,
            can_post_stories: false,
        }
    }

    pub fn set_admin_rights(
        &mut self,
        change_info: bool,
        delete_messages: bool,
        ban_users: bool,
        invite_users: bool,
        pin_messages: bool,
    ) {
        self.can_change_info = change_info;
        self.can_delete_messages = delete_messages;
        self.can_ban_users = ban_users;
        self.can_invite_users = invite_users;
        self.can_pin_messages = pin_messages;
    }
}
