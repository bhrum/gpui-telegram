//! History Unread Things (Mentions, Reactions, Poll Votes) Tracker.
//!
//! Citations:
//! - Upstream: `Telegram/SourceFiles/history/history_unread_things.h:L18-L75`
//! - Upstream: `Telegram/SourceFiles/history/history_unread_things.cpp:L22-L120`

use std::collections::BTreeSet;
use gpui::*;

/// Type of item addition: whether this is a newly arrived event or historical backfill.
/// C++ citation: `history_unread_things.h:L18-L21`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryUnreadAddType {
    New,
    Existing,
}

/// Category of unread badge item.
/// C++ citation: `history_unread_things.h:L23-L27`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryUnreadThingType {
    Mentions,
    Reactions,
    PollVotes,
}

/// Set of unread message IDs with min/max tracking.
/// C++ citation: `HistoryUnreadThings::List` (history_unread_things.h:L29-L69).
#[derive(Debug, Clone, Default)]
pub struct UnreadThingsList {
    count: Option<i32>,
    messages: BTreeSet<u64>,
}

impl UnreadThingsList {
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns count of loaded message IDs in memory.
    /// C++ citation: `history_unread_things.h:L31-L33`.
    pub fn loaded_count(&self) -> usize {
        self.messages.len()
    }

    /// Lowest MsgId among loaded messages.
    /// C++ citation: `history_unread_things.h:L34-L36`.
    pub fn min_loaded(&self) -> u64 {
        self.messages.iter().next().copied().unwrap_or(0)
    }

    /// Highest MsgId among loaded messages.
    /// C++ citation: `history_unread_things.h:L37-L39`.
    pub fn max_loaded(&self) -> u64 {
        self.messages.iter().next_back().copied().unwrap_or(0)
    }

    /// Unread count reported by server, or fallback if unknown.
    /// C++ citation: `history_unread_things.h:L40-L42`.
    pub fn count(&self, not_known_value: i32) -> i32 {
        self.count.unwrap_or(not_known_value)
    }

    /// Returns true if count > 0.
    /// C++ citation: `history_unread_things.h:L43-L45`.
    pub fn has(&self) -> bool {
        self.count(0) > 0
    }

    /// Checks if a message ID is in the unread set.
    /// C++ citation: `history_unread_things.h:L46-L48`.
    pub fn contains(&self, msg_id: u64) -> bool {
        self.messages.contains(&msg_id)
    }

    pub fn ids(&self) -> &BTreeSet<u64> {
        &self.messages
    }

    /// Updates server reported unread count.
    /// C++ citation: `history_unread_things.h:L52-L54`.
    pub fn set_count(&mut self, count: i32) {
        self.count = Some(count);
    }

    /// Inserts message ID into sorted set.
    /// C++ citation: `history_unread_things.h:L55-L57`.
    pub fn insert(&mut self, msg_id: u64) {
        self.messages.insert(msg_id);
    }

    /// Removes message ID from unread set.
    /// C++ citation: `history_unread_things.h:L58-L60`.
    pub fn erase(&mut self, msg_id: u64) {
        self.messages.remove(&msg_id);
    }

    /// Clears all loaded unread message IDs.
    /// C++ citation: `history_unread_things.h:L61-L63`.
    pub fn clear(&mut self) {
        self.messages.clear();
        self.count = Some(0);
    }
}

/// Aggregate tracker for all unread things in a history thread.
/// C++ citation: `HistoryUnreadThings::All` (history_unread_things.h:L71-L75).
pub struct HistoryUnreadThings {
    pub mentions: UnreadThingsList,
    pub reactions: UnreadThingsList,
    pub poll_votes: UnreadThingsList,
}

impl HistoryUnreadThings {
    pub fn new() -> Self {
        Self {
            mentions: UnreadThingsList::new(),
            reactions: UnreadThingsList::new(),
            poll_votes: UnreadThingsList::new(),
        }
    }

    /// Total sum of all unread items across mentions, reactions, and poll votes.
    pub fn total_unread_count(&self) -> i32 {
        self.mentions.count(0) + self.reactions.count(0) + self.poll_votes.count(0)
    }

    /// Whether any unread mentions or reactions are pending.
    pub fn has_any(&self) -> bool {
        self.mentions.has() || self.reactions.has() || self.poll_votes.has()
    }

    /// Finds the earliest unread message ID across all categories.
    pub fn earliest_unread_msg_id(&self) -> Option<u64> {
        let mut min_id = None;
        for list in [&self.mentions, &self.reactions, &self.poll_votes] {
            let m = list.min_loaded();
            if m > 0 {
                min_id = Some(min_id.map_or(m, |curr: u64| curr.min(m)));
            }
        }
        min_id
    }
}

impl Default for HistoryUnreadThings {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryUnreadThings {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.has_any() {
            return div().into_any_element();
        }

        let mention_count = self.mentions.count(0);
        let reaction_count = self.reactions.count(0);

        div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(
                if mention_count > 0 {
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .w(px(32.0))
                        .h(px(32.0))
                        .rounded_full()
                        .bg(rgb(0x2AABEE))
                        .text_color(rgb(0xFFFFFF))
                        .text_size(px(12.0))
                        .font_weight(FontWeight::BOLD)
                        .child("@")
                } else {
                    div()
                }
            )
            .child(
                if reaction_count > 0 {
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .w(px(32.0))
                        .h(px(32.0))
                        .rounded_full()
                        .bg(rgb(0xF59E0B))
                        .text_color(rgb(0xFFFFFF))
                        .text_size(px(12.0))
                        .child("♥")
                } else {
                    div()
                }
            )
            .into_any_element()
    }
}
