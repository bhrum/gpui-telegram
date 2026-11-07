//! Peer List Virtual Row Widgets and Item Rendering
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peer_list_widgets.h:L1-L111`
//! - `Telegram/SourceFiles/boxes/peer_list_widgets.cpp:L1-L398`
//! - `Telegram/SourceFiles/boxes/peer_list_widgets.cpp:L34-L125` (`paintRow`, check animation, name/status text layout)
//! - `Telegram/SourceFiles/boxes/peer_list_widgets.cpp:L153-L175` (`findRow`, `rowAt`)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

pub type PeerListRowId = u64;

/// Status and online presence indicator for peer row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PeerRowStatus {
    Online,
    LastSeenRecently,
    LastSeenAt(i64),
    Offline,
    Custom(String),
}

/// Single virtual peer row item model.
#[derive(Debug, Clone, PartialEq)]
pub struct PeerListRow {
    pub id: PeerListRowId,
    pub title: String,
    pub username: Option<String>,
    pub status: PeerRowStatus,
    pub is_checked: bool,
    pub is_disabled: bool,
    pub checked_ratio: f32,
    pub height: Pixels,
}

impl PeerListRow {
    pub fn new(id: PeerListRowId, title: String, status: PeerRowStatus) -> Self {
        Self {
            id,
            title,
            username: None,
            status,
            is_checked: false,
            is_disabled: false,
            checked_ratio: 0.0,
            height: px(56.0),
        }
    }

    /// Updates checked state and animates checkedRatio (peer_list_widgets.cpp:L113).
    pub fn set_checked(&mut self, checked: bool) {
        if self.is_disabled {
            return;
        }
        self.is_checked = checked;
        self.checked_ratio = if checked { 1.0 } else { 0.0 };
    }
}

/// Virtualized row container and list widget manager.
/// Target symbol in ledger: `PeerListWidgets`.
#[derive(Debug, Clone)]
pub struct PeerListWidgets {
    pub is_visible: bool,
    pub rows: Vec<PeerListRow>,
    pub selected_index: Option<usize>,
    pub item_height: Pixels,
}

impl Default for PeerListWidgets {
    fn default() -> Self {
        Self::new()
    }
}

impl PeerListWidgets {
    pub fn new() -> Self {
        Self {
            is_visible: true,
            rows: Vec::new(),
            selected_index: None,
            item_height: px(56.0),
        }
    }

    pub fn append_row(&mut self, row: PeerListRow) {
        self.rows.push(row);
    }

    pub fn find_row(&self, id: PeerListRowId) -> Option<&PeerListRow> {
        self.rows.iter().find(|r| r.id == id)
    }

    pub fn find_row_mut(&mut self, id: PeerListRowId) -> Option<&mut PeerListRow> {
        self.rows.iter_mut().find(|r| r.id == id)
    }

    pub fn row_at(&self, index: usize) -> Option<&PeerListRow> {
        self.rows.get(index)
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn set_row_checked(&mut self, id: PeerListRowId, checked: bool) -> bool {
        if let Some(row) = self.find_row_mut(id) {
            row.set_checked(checked);
            true
        } else {
            false
        }
    }

    pub fn collect_checked_ids(&self) -> Vec<PeerListRowId> {
        self.rows
            .iter()
            .filter(|r| r.is_checked)
            .map(|r| r.id)
            .collect()
    }

    pub fn clear(&mut self) {
        self.rows.clear();
        self.selected_index = None;
    }
}
