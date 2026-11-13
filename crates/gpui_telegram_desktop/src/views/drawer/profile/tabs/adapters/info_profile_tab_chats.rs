//! Info Profile Tab Chats Adapter
//! Upstream: `Telegram/SourceFiles/info/profile/tabs/adapters/info_profile_tab_chats.cpp:L1-L218`
//! Upstream: `Telegram/SourceFiles/info/profile/tabs/adapters/info_profile_tab_chats.h:L1-L23`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use serde::{Deserialize, Serialize};

/// Tab adapter bridging the profile tabs host with common groups and similar channels lists.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoProfileTabChatsProfileTabsAdapters {
    pub peer_id: i64,
    pub loaded_chats_count: usize,
    pub search_query: String,
    pub is_loading: bool,
}

impl InfoProfileTabChatsProfileTabsAdapters {
    /// Upstream `kSearchRequestDelay = crl::time(900)` (`info_profile_tab_chats.cpp:L28`).
    pub const SEARCH_REQUEST_DELAY_MS: u64 = 900;
    /// Upstream `kSearchPreloadHeight = 500` (`info_profile_tab_chats.cpp:L29`).
    pub const SEARCH_PRELOAD_HEIGHT: f32 = 500.0;

    pub fn new(peer_id: i64) -> Self {
        Self {
            peer_id,
            loaded_chats_count: 0,
            search_query: String::new(),
            is_loading: false,
        }
    }

    pub fn set_search_query(&mut self, query: String) {
        self.search_query = query;
    }

    pub fn should_preload(&self, scroll_top: f32, viewport_height: f32, content_height: f32) -> bool {
        (content_height - (scroll_top + viewport_height)) <= Self::SEARCH_PRELOAD_HEIGHT
    }
}
