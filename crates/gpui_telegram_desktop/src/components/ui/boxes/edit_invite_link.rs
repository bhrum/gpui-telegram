//! Authentic GPUI Chat Invite Link Editor Component.
//! Reference: Telegram/SourceFiles/ui/boxes/edit_invite_link.cpp:L35-L474
//! Reference: Telegram/SourceFiles/ui/boxes/edit_invite_link.h:L20-L51

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Unbounded usage limit representation: INT_MAX.
/// Reference: `edit_invite_link.cpp:L38`
pub const K_MAX_LIMIT: i32 = i32::MAX;

/// One hour duration in seconds: 3600.
/// Reference: `edit_invite_link.cpp:L39`
pub const K_HOUR: i64 = 3600;

/// One day duration in seconds: 86400.
/// Reference: `edit_invite_link.cpp:L40`
pub const K_DAY: i64 = 86400;

/// Maximum length for internal invite link label: 32.
/// Reference: `edit_invite_link.cpp:L41`
pub const K_MAX_LABEL_LENGTH: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InviteLinkFields {
    pub link: String,
    pub label: String,
    pub expire_date: i64,
    pub usage_limit: i32,
    pub subscription_credits: i32,
    pub request_approval: bool,
    pub is_group: bool,
    pub is_public: bool,
    pub global_request_approval: bool,
    pub guard_bot_username: String,
}

/// Target symbol for editing and creating invite links.
/// Reference: `edit_invite_link.h:L35-L51`
#[derive(Debug, Clone)]
pub struct EditInviteLinkBoxes {
    pub fields: InviteLinkFields,
    pub is_create_mode: bool,
    pub is_saved: bool,
}

impl Default for EditInviteLinkBoxes {
    fn default() -> Self {
        Self::new(InviteLinkFields {
            link: "https://t.me/+AbCdEfGhIjK".to_string(),
            label: "Community Invite".to_string(),
            expire_date: 0,
            usage_limit: 0,
            subscription_credits: 0,
            request_approval: false,
            is_group: false,
            is_public: false,
            global_request_approval: false,
            guard_bot_username: String::new(),
        }, false)
    }
}

impl EditInviteLinkBoxes {
    pub fn new(fields: InviteLinkFields, is_create: bool) -> Self {
        Self {
            fields,
            is_create_mode: is_create,
            is_saved: false,
        }
    }

    /// Sets lifetime preset: 1 hour, 1 day, or 1 week.
    /// Reference: `edit_invite_link.cpp:L180-L200`
    pub fn set_lifetime_preset(&mut self, now: i64, preset: &str) {
        match preset {
            "hour" => self.fields.expire_date = now + K_HOUR,
            "day" => self.fields.expire_date = now + K_DAY,
            "week" => self.fields.expire_date = now + K_DAY * 7,
            _ => self.fields.expire_date = 0, // No limit
        }
    }

    pub fn set_label(&mut self, label: &str) {
        self.fields.label = label.chars().take(K_MAX_LABEL_LENGTH).collect();
    }

    pub fn save(&mut self) -> InviteLinkFields {
        self.is_saved = true;
        self.fields.clone()
    }
}
