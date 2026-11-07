//! Authentic GPUI Expandable Peer List with Animated Foldout.
//! Reference: Telegram/SourceFiles/ui/widgets/expandable_peer_list.cpp:L30-L60
//! Reference: Telegram/SourceFiles/ui/widgets/expandable_peer_list.h:L20-L43

#![allow(dead_code, unused_imports, unused_variables)]

#[derive(Debug, Clone, PartialEq)]
pub struct ExpandablePeerItem {
    pub peer_id: u64,
    pub title: String,
    pub is_selected: bool,
}

/// Target symbol managing expandable participant list with expander button.
#[derive(Debug, Clone)]
pub struct ExpandablePeerListWidgets {
    pub is_expanded: bool,
    pub participants: Vec<ExpandablePeerItem>,
    pub hide_right_button: bool,
    pub check_top_on_all_inner: bool,
    pub skip_single: bool,
}

impl Default for ExpandablePeerListWidgets {
    fn default() -> Self {
        Self::new()
    }
}

impl ExpandablePeerListWidgets {
    pub fn new() -> Self {
        Self {
            is_expanded: false,
            participants: Vec::new(),
            hide_right_button: false,
            check_top_on_all_inner: true,
            skip_single: false,
        }
    }

    pub fn set_participants(&mut self, items: Vec<ExpandablePeerItem>) {
        self.participants = items;
    }

    pub fn toggle_expanded(&mut self) {
        self.is_expanded = !self.is_expanded;
    }

    /// Formats expander button title: e.g. "Expand (12)".
    /// Reference: `expandable_peer_list.cpp:L31-L35`
    pub fn expander_label(&self) -> String {
        let count = self.participants.len();
        if self.is_expanded {
            format!("Collapse ({count})")
        } else {
            format!("Expand ({count})")
        }
    }

    pub fn toggle_peer_selection(&mut self, peer_id: u64) {
        if let Some(peer) = self.participants.iter_mut().find(|p| p.peer_id == peer_id) {
            peer.is_selected = !peer.is_selected;
        }
    }

    pub fn select_all(&mut self, selected: bool) {
        for p in &mut self.participants {
            p.is_selected = selected;
        }
    }

    pub fn selected_count(&self) -> usize {
        self.participants.iter().filter(|p| p.is_selected).count()
    }
}
