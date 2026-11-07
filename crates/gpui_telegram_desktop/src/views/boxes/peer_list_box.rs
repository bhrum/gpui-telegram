//! Peer List Modal Box Container
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peer_list_box.h:L1-L1281`
//! - `Telegram/SourceFiles/boxes/peer_list_box.cpp:L1-L2902`
//! - `Telegram/SourceFiles/boxes/peer_list_box.cpp:L100-L130` (Multi-select creation & metrics)
//! - `Telegram/SourceFiles/boxes/peer_list_box.cpp:L165-L180` (`topScrollSkip` calculation)
//! - `Telegram/SourceFiles/boxes/peer_list_box.cpp:L630-L660` (`collectSelectedIds`)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

use super::peer_list_controllers::PeerListControllers;
use super::peer_list_section_headers::PeerListSectionHeaders;
use super::peer_list_section_index::PeerListSectionIndex;
use super::peer_list_widgets::{PeerListRow, PeerListRowId, PeerListWidgets};

/// Selected chip item in the multi-select header bar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiSelectItem {
    pub id: PeerListRowId,
    pub title: String,
}

/// Modal dialog box for peer selection with search, virtual list, and multi-selection header.
/// Target symbol in ledger: `PeerListBox`.
#[derive(Debug, Clone)]
pub struct PeerListBox {
    pub is_visible: bool,
    pub title: String,
    pub width: Pixels,
    pub height: Pixels,
    pub search_height: Pixels,
    pub multi_select_height: Pixels,
    pub has_multi_select: bool,
    pub above_search: bool,
    pub selected_items: Vec<MultiSelectItem>,
    pub widgets: PeerListWidgets,
    pub controllers: PeerListControllers,
    pub headers: PeerListSectionHeaders,
    pub index_bar: PeerListSectionIndex,
}

impl Default for PeerListBox {
    fn default() -> Self {
        Self::new("Select Peers".to_string(), true)
    }
}

impl PeerListBox {
    pub fn new(title: String, multi_select: bool) -> Self {
        Self {
            is_visible: true,
            title,
            width: px(380.0),
            height: px(460.0),
            search_height: px(48.0),
            multi_select_height: px(0.0),
            has_multi_select: multi_select,
            above_search: false,
            selected_items: Vec::new(),
            widgets: PeerListWidgets::new(),
            controllers: PeerListControllers::new(),
            headers: PeerListSectionHeaders::new(),
            index_bar: PeerListSectionIndex::new(),
        }
    }

    /// Calculates top scroll skip according to header layout (peer_list_box.cpp:L165-L180).
    pub fn top_scroll_skip(&self) -> Pixels {
        if self.above_search {
            self.search_height + self.multi_select_height
        } else {
            self.multi_select_height
        }
    }

    /// Toggles row selection and updates multi-select chips (peer_list_box.cpp:L363-L385).
    pub fn toggle_row_selection(&mut self, id: PeerListRowId) -> bool {
        if let Some(row) = self.widgets.find_row_mut(id) {
            let next_checked = !row.is_checked;
            row.set_checked(next_checked);

            if next_checked {
                if !self.has_multi_select {
                    self.selected_items.clear();
                }
                self.selected_items.push(MultiSelectItem {
                    id: row.id,
                    title: row.title.clone(),
                });
            } else {
                self.selected_items.retain(|item| item.id != id);
            }
            self.update_multi_select_metrics();
            true
        } else {
            false
        }
    }

    fn update_multi_select_metrics(&mut self) {
        if self.selected_items.is_empty() || !self.has_multi_select {
            self.multi_select_height = px(0.0);
        } else {
            let rows = ((self.selected_items.len() as f32) / 3.0).ceil();
            self.multi_select_height = px((rows * 36.0 + 8.0).min(120.0));
        }
    }

    pub fn selected_ids(&self) -> Vec<PeerListRowId> {
        self.selected_items.iter().map(|item| item.id).collect()
    }

    pub fn selected_count(&self) -> usize {
        self.selected_items.len()
    }

    pub fn add_row(&mut self, row: PeerListRow) {
        self.widgets.append_row(row);
    }
}
