//! History View Group Members Widget View Component (`history_view_group_members_widget`)
//!
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_group_members_widget.h` (L1-L58)
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_group_members_widget.cpp` (L1-L112)

/// Member item representation for group members dropdown
#[derive(Debug, Clone, PartialEq)]
pub struct GroupMemberRow {
    pub user_id: i64,
    pub name: String,
    pub is_admin: bool,
    pub has_stories: bool,
    pub online_status: String,
}

/// GPUI view component for inline group members dropdown
#[derive(Debug, Clone)]
pub struct HistoryViewGroupMembersWidgetView {
    pub peer_id: i64,
    pub members: Vec<GroupMemberRow>,
    pub show_stories: bool,
    pub is_open: bool,
}

impl HistoryViewGroupMembersWidgetView {
    pub fn new(peer_id: i64) -> Self {
        Self {
            peer_id,
            members: Vec::new(),
            show_stories: true,
            is_open: false,
        }
    }

    pub fn set_members(&mut self, members: Vec<GroupMemberRow>) {
        self.members = members;
    }

    pub fn count(&self) -> usize {
        self.members.len()
    }

    pub fn toggle(&mut self) -> bool {
        self.is_open = !self.is_open;
        self.is_open
    }
}
