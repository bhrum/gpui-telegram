//! Authentic GPUI Telegram Dialogs Widget container.
//!
//! Based on upstream Telegram Desktop:
//! - `Telegram/SourceFiles/dialogs/dialogs_widget.h:L93-468`
//! - `Telegram/SourceFiles/dialogs/dialogs_widget.cpp:L112-230`
//!
//! Subsystem: dialogs, Feature: TCG-N03

use serde::{Deserialize, Serialize};

/// Upstream constants from `dialogs_widget.cpp:L114-116`.
pub const K_SEARCH_PER_PAGE: usize = 50;
pub const K_STORIES_EXPAND_DURATION_MS: u64 = 200;
pub const K_SEARCH_REQUEST_DELAY_MS: u64 = 900;
pub const K_OPTION_FORUM_HIDE_CHATS_LIST: &'static str = "forum-hide-chats-list";

/// Checks if character stream should be redirected into search input (`dialogs_widget.cpp:L138-145`).
pub fn redirect_text_to_search(text: &str) -> bool {
    text.chars().any(|ch| ch as u32 >= 32)
}

/// Search input state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchInputState {
    Idle,
    Typing,
    Debouncing,
    Searching,
    Completed,
}

impl Default for SearchInputState {
    fn default() -> Self {
        Self::Idle
    }
}

/// Menu Under-Button invisible hit-area (`dialogs_widget.cpp:L129-136`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DialogsWidgetMenuunderbutton {
    pub is_hit: bool,
}

/// Narrow button for adding chats in community view (`dialogs_widget.cpp:L199-230`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DialogsWidgetCommunityaddchatnarrowbutton {
    pub is_enabled: bool,
}

/// Dialogs widget controller structure.
#[derive(Debug, Clone)]
pub struct DialogsWidget {
    pub search_query: String,
    pub search_state: SearchInputState,
    pub active_folder_id: u32,
    pub is_locked: bool,
    pub debounce_timer_ms: u64,
    pub per_page_limit: usize,
    pub hide_chats_in_forum: bool,
}

impl Default for DialogsWidget {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            search_state: SearchInputState::Idle,
            active_folder_id: 0,
            is_locked: false,
            debounce_timer_ms: K_SEARCH_REQUEST_DELAY_MS,
            per_page_limit: K_SEARCH_PER_PAGE,
            hide_chats_in_forum: false,
        }
    }
}

impl DialogsWidget {
    pub fn new() -> Self {
        Self::default()
    }

    /// Updates query string and transitions search state machine.
    pub fn on_search_text_changed(&mut self, query: String) {
        self.search_query = query;
        if self.search_query.is_empty() {
            self.search_state = SearchInputState::Idle;
        } else {
            self.search_state = SearchInputState::Debouncing;
        }
    }

    /// Triggers execution of server/local search after debounce timer expires.
    pub fn execute_search(&mut self) {
        if !self.search_query.is_empty() {
            self.search_state = SearchInputState::Searching;
        }
    }

    pub fn set_folder(&mut self, folder_id: u32) {
        self.active_folder_id = folder_id;
    }
}
