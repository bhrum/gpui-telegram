//! Multi-List Peer Picker Box Container
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peer_lists_box.h:L1-L114`
//! - `Telegram/SourceFiles/boxes/peer_lists_box.cpp:L1-L463`
//! - `Telegram/SourceFiles/boxes/peer_lists_box.cpp:L22-L75` (`makeList`, `makeLists`, multi-list tab routing)
//! - `Telegram/SourceFiles/boxes/peer_lists_box.cpp:L81-L125` (`createMultiSelect`)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

use super::peer_list_box::MultiSelectItem;
use super::peer_list_widgets::{PeerListRow, PeerListRowId, PeerListWidgets};

/// Sub-list descriptor inside a multi-list dialog (e.g. Contacts vs Recent Chats).
#[derive(Debug, Clone)]
pub struct PeerSubList {
    pub title: String,
    pub widgets: PeerListWidgets,
}

impl PeerSubList {
    pub fn new(title: String) -> Self {
        Self {
            title,
            widgets: PeerListWidgets::new(),
        }
    }
}

/// Multi-tab or composite peer picker modal dialog box.
/// Target symbol in ledger: `PeerListsBox`.
#[derive(Debug, Clone)]
pub struct PeerListsBox {
    pub is_visible: bool,
    pub title: String,
    pub width: Pixels,
    pub height: Pixels,
    pub lists: Vec<PeerSubList>,
    pub active_tab_index: usize,
    pub selected_items: Vec<MultiSelectItem>,
}

impl Default for PeerListsBox {
    fn default() -> Self {
        Self::new("Choose Chats".to_string())
    }
}

impl PeerListsBox {
    pub fn new(title: String) -> Self {
        Self {
            is_visible: true,
            title,
            width: px(400.0),
            height: px(500.0),
            lists: Vec::new(),
            active_tab_index: 0,
            selected_items: Vec::new(),
        }
    }

    pub fn add_sub_list(&mut self, list: PeerSubList) {
        self.lists.push(list);
    }

    pub fn set_active_tab(&mut self, index: usize) {
        if index < self.lists.len() {
            self.active_tab_index = index;
        }
    }

    pub fn active_sub_list(&self) -> Option<&PeerSubList> {
        self.lists.get(self.active_tab_index)
    }

    pub fn active_sub_list_mut(&mut self) -> Option<&mut PeerSubList> {
        self.lists.get_mut(self.active_tab_index)
    }

    pub fn toggle_row_selection(&mut self, list_idx: usize, row_id: PeerListRowId) -> bool {
        if let Some(list) = self.lists.get_mut(list_idx) {
            if let Some(row) = list.widgets.find_row_mut(row_id) {
                let next_checked = !row.is_checked;
                row.set_checked(next_checked);
                if next_checked {
                    self.selected_items.push(MultiSelectItem {
                        id: row.id,
                        title: row.title.clone(),
                    });
                } else {
                    self.selected_items.retain(|item| item.id != row_id);
                }
                return true;
            }
        }
        false
    }

    pub fn collect_selected_ids(&self) -> Vec<PeerListRowId> {
        self.selected_items.iter().map(|item| item.id).collect()
    }
}
