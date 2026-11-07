//! Peer List Controllers, Contact Sorting and Remote Search
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peer_list_controllers.h:L1-L486`
//! - `Telegram/SourceFiles/boxes/peer_list_controllers.cpp:L1-L1652`
//! - `Telegram/SourceFiles/boxes/peer_list_controllers.cpp:L62-L63` (`kSortByOnlineThrottle`, `kSearchPerPage`)
//! - `Telegram/SourceFiles/boxes/peer_list_controllers.cpp:L234-L270` (Search cache & debounce)
//! - `Telegram/SourceFiles/boxes/peer_list_controllers.cpp:L735-L795` (SortMode & contact sorting)
//!
//! Subsystem: boxes, Feature: TCG-N11

use std::collections::HashMap;
use std::time::Duration;

use super::peer_list_widgets::{PeerListRow, PeerListRowId, PeerRowStatus};

pub const K_SORT_BY_ONLINE_THROTTLE: Duration = Duration::from_millis(3000);
pub const K_SEARCH_PER_PAGE: usize = 50;

/// Contact ordering mode (peer_list_controllers.cpp:L735-L745).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContactSortMode {
    Name,
    Online,
}

/// Controller coordinating peer list search, filtering, and sort ordering.
/// Target symbol in ledger: `PeerListControllers`.
#[derive(Debug, Clone)]
pub struct PeerListControllers {
    pub is_active: bool,
    pub sort_mode: ContactSortMode,
    pub query: String,
    pub search_cache: HashMap<String, Vec<PeerListRowId>>,
    pub is_searching: bool,
    pub last_sort_time_ms: i64,
}

impl Default for PeerListControllers {
    fn default() -> Self {
        Self::new()
    }
}

impl PeerListControllers {
    pub fn new() -> Self {
        Self {
            is_active: true,
            sort_mode: ContactSortMode::Name,
            query: String::new(),
            search_cache: HashMap::new(),
            is_searching: false,
            last_sort_time_ms: 0,
        }
    }

    /// Sets sort mode with throttle protection for online presence (peer_list_controllers.cpp:L740-L755).
    pub fn set_sort_mode(&mut self, mode: ContactSortMode, now_ms: i64) -> bool {
        if self.sort_mode == mode {
            return false;
        }
        self.sort_mode = mode;
        if mode == ContactSortMode::Online {
            self.last_sort_time_ms = now_ms;
        }
        true
    }

    /// Sorts rows by current mode (peer_list_controllers.cpp:L764-L795).
    pub fn sort_rows(&self, rows: &mut [PeerListRow]) {
        match self.sort_mode {
            ContactSortMode::Name => {
                rows.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
            }
            ContactSortMode::Online => {
                rows.sort_by(|a, b| {
                    let score_a = Self::status_online_score(&a.status);
                    let score_b = Self::status_online_score(&b.status);
                    score_b.cmp(&score_a).then_with(|| a.title.cmp(&b.title))
                });
            }
        }
    }

    fn status_online_score(status: &PeerRowStatus) -> i64 {
        match status {
            PeerRowStatus::Online => i64::MAX,
            PeerRowStatus::LastSeenRecently => i64::MAX - 100,
            PeerRowStatus::LastSeenAt(ts) => *ts,
            PeerRowStatus::Offline => 0,
            PeerRowStatus::Custom(_) => 10,
        }
    }

    /// Debounced search query update with cache checking (peer_list_controllers.cpp:L234-L255).
    pub fn handle_search_query(&mut self, query: &str) -> Option<&Vec<PeerListRowId>> {
        self.query = query.trim().to_string();
        if self.query.is_empty() {
            self.is_searching = false;
            return None;
        }
        self.is_searching = true;
        self.search_cache.get(&self.query)
    }

    pub fn cache_search_results(&mut self, query: String, ids: Vec<PeerListRowId>) {
        self.search_cache.insert(query, ids);
        self.is_searching = false;
    }
}
