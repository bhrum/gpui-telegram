//! GIF Search Engine and Inline Query Controller
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/chat_helpers/gifs_list_widget.cpp:L52-L55`
//! - `Telegram/SourceFiles/chat_helpers/gifs_list_widget.h:L1-L227`
//!
//! Subsystem: chat_helpers, Feature: TCG-N11

use super::gifs_list_widget::{GifItem, K_SEARCH_REQUEST_DELAY};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchState {
    Idle,
    Debouncing { query: String },
    Searching { query: String, offset: String },
    Completed { query: String, has_more: bool },
    Failed { query: String, error: String },
}

/// GIF search state machine managing query debounce, pagination, and caching.
#[derive(Debug, Clone)]
pub struct GifsSearch {
    pub current_query: String,
    pub next_offset: String,
    pub state: SearchState,
    pub debounce_duration: Duration,
    pub results_cache: Vec<GifItem>,
    pub inline_bot_username: String,
}

impl Default for GifsSearch {
    fn default() -> Self {
        Self {
            current_query: String::new(),
            next_offset: String::new(),
            state: SearchState::Idle,
            debounce_duration: K_SEARCH_REQUEST_DELAY,
            results_cache: Vec::new(),
            inline_bot_username: "gif".to_string(),
        }
    }
}

impl GifsSearch {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_query(&mut self, query: &str) {
        let q = query.trim().to_string();
        if self.current_query != q {
            self.current_query = q.clone();
            self.next_offset.clear();
            if self.current_query.is_empty() {
                self.state = SearchState::Idle;
                self.results_cache.clear();
            } else {
                self.state = SearchState::Debouncing { query: q };
            }
        }
    }

    pub fn on_debounce_elapsed(&mut self) -> Option<String> {
        if let SearchState::Debouncing { query } = &self.state {
            let q = query.clone();
            self.state = SearchState::Searching {
                query: q.clone(),
                offset: self.next_offset.clone(),
            };
            Some(q)
        } else {
            None
        }
    }

    pub fn handle_results(&mut self, items: Vec<GifItem>, next_offset: Option<String>) {
        let has_more = next_offset.is_some();
        self.next_offset = next_offset.unwrap_or_default();
        self.results_cache.extend(items);
        self.state = SearchState::Completed {
            query: self.current_query.clone(),
            has_more,
        };
    }
}
