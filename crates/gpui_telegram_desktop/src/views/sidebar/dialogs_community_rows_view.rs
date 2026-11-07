//! Community rows view container.
//!
//! Upstream:
//! - `Telegram/SourceFiles/dialogs/dialogs_community_rows_view.h:L1-L56`
//! - `Telegram/SourceFiles/dialogs/dialogs_community_rows_view.cpp:L1-L112`
//!
//! Subsystem: dialogs, Feature: TCG-N11
//!
//! Key Algorithm:
//! - Cumulative prefix-sum `_tops` array generation (`finalize` L43-51)
//! - Binary search for vertical index lookup (`indexByY` L64-70)

pub use gpui_telegram_client::dialogs::community::{
    CommunityChatEntry, CommunityRowsView as ClientCommunityRowsView,
};
use serde::{Deserialize, Serialize};

/// Visual container managing vertical community chat rows and pixel top offsets.
/// Upstream: `Dialogs::CommunityRowsView` (`dialogs_community_rows_view.h:L17-53`)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialogsCommunityRowsView {
    pub rows: Vec<CommunityChatEntry>,
    pub tops: Vec<i32>,
    pub is_visible: bool,
}

impl DialogsCommunityRowsView {
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            tops: Vec::new(),
            is_visible: true,
        }
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn height(&self) -> i32 {
        self.tops.last().copied().unwrap_or(0)
    }

    pub fn add(&mut self, entry: CommunityChatEntry) {
        self.rows.push(entry);
    }

    /// Recompute prefix-sum array of row top positions.
    /// Upstream: `dialogs_community_rows_view.cpp:L43-51`
    pub fn finalize(&mut self, row_height: i32) {
        self.tops.resize(self.rows.len() + 1, 0);
        let mut top = 0;
        for i in 0..self.rows.len() {
            self.tops[i] = top;
            top += row_height;
        }
        if let Some(last) = self.tops.last_mut() {
            *last = top;
        }
    }

    /// Binary search for row index at pixel y.
    /// Upstream: `dialogs_community_rows_view.cpp:L64-70`
    pub fn index_by_y(&self, y: i32) -> Option<usize> {
        if self.tops.is_empty() || y < 0 || y >= self.height() {
            return None;
        }
        let idx = self.tops.partition_point(|&top| top <= y);
        if idx > 0 {
            Some(idx - 1)
        } else {
            None
        }
    }
}
