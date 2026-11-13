//! Core Conversation History Model and Thread Transcript Container.
//!
//! Citations:
//! - Upstream: `Telegram/SourceFiles/history/history.h:L52-L130`
//! - Upstream: `Telegram/SourceFiles/history/history.cpp:L87-L91`
//! - Anonymous Constants:
//!   - `kNewBlockEachMessage = 50;` (history.cpp:L89)
//!   - `kSkipCloudDraftsFor = 2;` (history.cpp:L90)
//!   - `kCountUnreadMessagesLimit = 10000;` (history.cpp:L91)

use std::collections::BTreeMap;
use gpui::*;

use super::history_translation::HistoryTranslation;
use super::history_unread_things::HistoryUnreadThings;
use super::history_streamed_drafts::HistoryStreamedDrafts;

pub const NEW_BLOCK_EACH_MESSAGE: usize = 50;
pub const SKIP_CLOUD_DRAFTS_FOR_SECS: u64 = 2;
pub const COUNT_UNREAD_MESSAGES_LIMIT: u32 = 10_000;

/// Clear type determining the scope of memory and database purging.
/// C++ citation: `History::ClearType` (history.h:L122-L126).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryClearType {
    Unload,
    DeleteChat,
    ClearHistory,
}

/// Message classification when new items are added to history.
/// C++ citation: `NewMessageType` (history.h:L52-L56).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryNewMessageType {
    Unread,
    Last,
    Existing,
}

/// Core Telegram conversation history model.
/// C++ citation: `History` (history.h:L64-L130).
pub struct History {
    pub peer_id: u64,
    pub topic_root_id: Option<u64>,
    pub unread_count: u32,
    pub unread_mark: bool,
    pub is_muted: bool,
    pub is_forum: bool,
    pub min_available_id: u64,
    pub has_pending_resized_items: bool,
    pub translation: HistoryTranslation,
    pub unread_things: HistoryUnreadThings,
    pub streamed_drafts: HistoryStreamedDrafts,
    message_ids: BTreeMap<u64, String>, // msg_id -> snippet
}

impl History {
    pub fn new(peer_id: u64) -> Self {
        Self {
            peer_id,
            topic_root_id: None,
            unread_count: 0,
            unread_mark: false,
            is_muted: false,
            is_forum: false,
            min_available_id: 1,
            has_pending_resized_items: false,
            translation: HistoryTranslation::new(None),
            unread_things: HistoryUnreadThings::new(),
            streamed_drafts: HistoryStreamedDrafts::new(),
            message_ids: BTreeMap::new(),
        }
    }

    pub fn with_topic(mut self, topic_id: u64) -> Self {
        self.topic_root_id = Some(topic_id);
        self.is_forum = true;
        self
    }

    /// Sets muted state for conversation notifications.
    /// C++ citation: `history.h:L80-L86`.
    pub fn set_muted(&mut self, muted: bool) {
        self.is_muted = muted;
    }

    /// Updates unread message count clamped by `kCountUnreadMessagesLimit`.
    /// C++ citation: `history.cpp:L91`.
    pub fn set_unread_count(&mut self, count: u32) {
        self.unread_count = count.min(COUNT_UNREAD_MESSAGES_LIMIT);
    }

    /// Adds a message entry to the local history index.
    pub fn add_message(&mut self, msg_id: u64, snippet: impl Into<String>, msg_type: HistoryNewMessageType) {
        self.message_ids.insert(msg_id, snippet.into());
        if msg_type == HistoryNewMessageType::Unread {
            self.set_unread_count(self.unread_count.saturating_add(1));
        }
    }

    /// Destroys a message from local history index.
    /// C++ citation: `history.h:L153`.
    pub fn destroy_message(&mut self, msg_id: u64) -> bool {
        self.message_ids.remove(&msg_id).is_some()
    }

    /// Clears chat history according to specified clear type.
    /// C++ citation: `history.h:L127`.
    pub fn clear(&mut self, clear_type: HistoryClearType) {
        self.message_ids.clear();
        self.unread_count = 0;
        self.unread_mark = false;
        if clear_type == HistoryClearType::ClearHistory || clear_type == HistoryClearType::DeleteChat {
            self.min_available_id = u64::MAX;
        }
    }

    /// Difference sync range (from_min_id, to_max_id).
    /// C++ citation: `history.h:L97` (`rangeForDifferenceRequest`).
    pub fn range_for_difference_request(&self) -> (u64, u64) {
        let min_id = self.message_ids.keys().next().copied().unwrap_or(0);
        let max_id = self.message_ids.keys().next_back().copied().unwrap_or(0);
        (min_id, max_id)
    }

    pub fn is_empty(&self) -> bool {
        self.message_ids.is_empty()
    }

    pub fn message_count(&self) -> usize {
        self.message_ids.len()
    }

    pub fn has_pending_resized_items(&self) -> bool {
        self.has_pending_resized_items
    }

    pub fn set_has_pending_resized_items(&mut self, pending: bool) {
        self.has_pending_resized_items = pending;
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Render for History {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(12.0))
            .bg(rgb(0xFFFFFF))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_size(px(14.0))
                    .text_color(rgb(0x18181B))
                    .child(format!("Chat #{}", self.peer_id))
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x71717A))
                    .child(format!("{} messages loaded, {} unread", self.message_count(), self.unread_count))
            )
    }
}
