//! History View Chat Section View Component (`history_view_chat_section`)
//!
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_chat_section.h` (L1-L730)
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_chat_section.cpp` (L1-L6344)

use crate::views::stage::chat_section::{
    calculate_keyboard_reserve, is_special_show_at_msg_id, resolve_highlight_id,
    ChatMemento, FullMsgId, HistoryViewChatSectionState, SendPermission,
    K_PSA_ABOUT_PREFIX, K_SCROLL_TO_VOICE_AFTER_SCROLLED_MS,
    K_SHOW_MEMBERS_DROPDOWN_TIMEOUT_MS, SHOW_AND_START_BOT_MSG_ID,
    SHOW_AT_THE_END_MSG_ID, SHOW_FOR_CHOOSE_MESSAGES_MSG_ID,
};

/// GPUI view component for conversation chat section
#[derive(Debug, Clone)]
pub struct HistoryViewChatSectionView {
    pub inner: HistoryViewChatSectionState,
    pub is_focused: bool,
    pub members_dropdown_open: bool,
}

impl HistoryViewChatSectionView {
    pub fn new(peer_id: i64, topic_id: Option<i64>) -> Self {
        Self {
            inner: HistoryViewChatSectionState::new(peer_id, topic_id),
            is_focused: true,
            members_dropdown_open: false,
        }
    }

    pub fn resolve_target_highlight(&self, highlight_id: i64) -> FullMsgId {
        resolve_highlight_id(self.inner.peer_id, self.inner.migrated_peer_id, highlight_id)
    }

    pub fn can_send_messages(&self) -> bool {
        self.inner.can_send(SendPermission::Anything)
    }

    pub fn can_send_media_files(&self) -> bool {
        self.inner.can_send(SendPermission::Files)
    }

    pub fn set_bot_keyboard(&mut self, height: f32, max_field_height: f32) {
        self.inner.set_bot_keyboard(height, max_field_height);
    }

    pub fn clear_bot_keyboard(&mut self) {
        self.inner.clear_bot_keyboard();
    }

    pub fn keyboard_reserve_height(&self) -> f32 {
        self.inner.keyboard_reserve_height
    }

    pub fn save_memento(&mut self, scroll_top: f32) -> ChatMemento {
        self.inner.save_memento(scroll_top)
    }

    pub fn restore_memento(&mut self, memento: ChatMemento) {
        self.inner.restore_memento(memento);
    }

    pub fn check_special_msg_id(&self, id: i64) -> bool {
        is_special_show_at_msg_id(id)
    }

    pub fn psa_prefix(&self) -> &'static str {
        K_PSA_ABOUT_PREFIX
    }
}
