//! History View Chat Section Component (`history_view_chat_section`)
//!
//! Primary chat section controller and coordinator for Telegram Desktop
//! upstream: `Telegram/SourceFiles/history/view/history_view_chat_section.h` (L1-L730)
//! upstream: `Telegram/SourceFiles/history/view/history_view_chat_section.cpp` (L1-L6344)

use std::time::Duration;

/// Anonymous namespace constants from `history_view_chat_section.cpp:L147-L150`
pub const K_SHOW_MEMBERS_DROPDOWN_TIMEOUT_MS: u64 = 300;
pub const K_SCROLL_TO_VOICE_AFTER_SCROLLED_MS: Duration = Duration::from_millis(1000);
pub const K_PSA_ABOUT_PREFIX: &str = "cloud_lng_about_psa_";

/// Special Message IDs for navigation requests
/// Upstream: `history_view_chat_section.cpp:L168-L173`
pub const SHOW_AT_THE_END_MSG_ID: i64 = 0;
pub const SHOW_AND_START_BOT_MSG_ID: i64 = -1;
pub const SHOW_AND_MAYBE_START_BOT_MSG_ID: i64 = -2;
pub const SHOW_FOR_CHOOSE_MESSAGES_MSG_ID: i64 = -3;

#[inline]
pub fn is_special_show_at_msg_id(id: i64) -> bool {
    id == SHOW_AT_THE_END_MSG_ID
        || id == SHOW_AND_START_BOT_MSG_ID
        || id == SHOW_AND_MAYBE_START_BOT_MSG_ID
        || id == SHOW_FOR_CHOOSE_MESSAGES_MSG_ID
}

/// Send permissions check
/// Upstream: `history_view_chat_section.cpp:L142-L145`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SendPermission {
    Anything,
    Files,
}

/// Upstream: `history_view_chat_section.cpp:L175-L185`: `ResolveHighlightId`
/// When highlight ID is negative and migrated history exists, resolves to migrated peer ID!
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FullMsgId {
    pub peer_id: i64,
    pub msg_id: i64,
}

pub fn resolve_highlight_id(active_peer_id: i64, migrated_peer_id: Option<i64>, highlight_id: i64) -> FullMsgId {
    if highlight_id < 0 {
        if let Some(migrated_id) = migrated_peer_id {
            return FullMsgId {
                peer_id: migrated_id,
                msg_id: -highlight_id,
            };
        }
    }
    FullMsgId {
        peer_id: active_peer_id,
        msg_id: highlight_id,
    }
}

/// Upstream: `history_view_chat_section.cpp:L189-L210`: `ChatMemento`
#[derive(Debug, Clone, PartialEq)]
pub struct ChatMemento {
    pub peer_id: i64,
    pub topic_id: Option<i64>,
    pub highlight_id: i64,
    pub scroll_top: f32,
    pub activate_choose_for_report: bool,
    pub send_bot_start: bool,
    pub maybe_send_bot_start: bool,
}

impl ChatMemento {
    pub fn new(peer_id: i64, topic_id: Option<i64>, highlight_id: i64) -> Self {
        let is_special = is_special_show_at_msg_id(highlight_id);
        Self {
            peer_id,
            topic_id,
            highlight_id: if is_special { 0 } else { highlight_id },
            scroll_top: 0.0,
            activate_choose_for_report: highlight_id == SHOW_FOR_CHOOSE_MESSAGES_MSG_ID,
            send_bot_start: highlight_id == SHOW_AND_START_BOT_MSG_ID,
            maybe_send_bot_start: highlight_id == SHOW_AND_MAYBE_START_BOT_MSG_ID,
        }
    }
}

/// Upstream keyboard height reservation calculation
/// `keyboardReserve = std::min(_keyboard->height(), maxFieldHeight - (maxFieldHeight / 2))`
#[inline]
pub fn calculate_keyboard_reserve(keyboard_height: f32, max_field_height: f32) -> f32 {
    let half_max = max_field_height - (max_field_height / 2.0);
    keyboard_height.min(half_max).max(0.0)
}

/// Primary Chat Section Controller State
/// Upstream: `HistoryView::ChatSection`
#[derive(Debug, Clone)]
pub struct HistoryViewChatSectionState {
    pub peer_id: i64,
    pub topic_id: Option<i64>,
    pub migrated_peer_id: Option<i64>,
    pub can_send_anything: bool,
    pub can_send_files: bool,
    pub is_read_only: bool,
    pub pinned_bar_visible: bool,
    pub translate_bar_visible: bool,
    pub group_call_bar_visible: bool,
    pub requests_bar_visible: bool,
    pub composer_draft: String,
    pub keyboard_reserve_height: f32,
    pub active_memento: Option<ChatMemento>,
}

impl HistoryViewChatSectionState {
    pub fn new(peer_id: i64, topic_id: Option<i64>) -> Self {
        Self {
            peer_id,
            topic_id,
            migrated_peer_id: None,
            can_send_anything: true,
            can_send_files: true,
            is_read_only: false,
            pinned_bar_visible: false,
            translate_bar_visible: false,
            group_call_bar_visible: false,
            requests_bar_visible: false,
            composer_draft: String::new(),
            keyboard_reserve_height: 0.0,
            active_memento: None,
        }
    }

    pub fn can_send(&self, permission: SendPermission) -> bool {
        if self.is_read_only {
            return false;
        }
        match permission {
            SendPermission::Anything => self.can_send_anything,
            SendPermission::Files => self.can_send_files,
        }
    }

    pub fn set_bot_keyboard(&mut self, keyboard_height: f32, max_field_height: f32) {
        self.keyboard_reserve_height = calculate_keyboard_reserve(keyboard_height, max_field_height);
    }

    pub fn clear_bot_keyboard(&mut self) {
        self.keyboard_reserve_height = 0.0;
    }

    pub fn save_memento(&mut self, scroll_top: f32) -> ChatMemento {
        let mut m = ChatMemento::new(self.peer_id, self.topic_id, 0);
        m.scroll_top = scroll_top;
        self.active_memento = Some(m.clone());
        m
    }

    pub fn restore_memento(&mut self, memento: ChatMemento) {
        self.peer_id = memento.peer_id;
        self.topic_id = memento.topic_id;
        self.active_memento = Some(memento);
    }
}
