//! History View Subsection Tabs View Component (`history_view_subsection_tabs`)
//!
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_subsection_tabs.h` (L1-L155)
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_subsection_tabs.cpp` (L1-L1018)

/// Default slice limit from `history_view_subsection_tabs.cpp:L50`
pub const K_DEFAULT_SUBSECTION_TABS_LIMIT: usize = 12;

/// Docking layout for subsection tabs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubsectionTabsMode {
    Top,
    Left,
}

/// Single forum topic tab entry
#[derive(Debug, Clone, PartialEq)]
pub struct TopicTabItem {
    pub topic_id: i64,
    pub title: String,
    pub icon_id: Option<i64>,
    pub unread_count: usize,
    pub is_closed: bool,
}

/// GPUI view component for forum topic subsection tabs
#[derive(Debug, Clone)]
pub struct HistoryViewSubsectionTabsView {
    pub peer_id: i64,
    pub mode: SubsectionTabsMode,
    pub tabs: Vec<TopicTabItem>,
    pub active_topic_id: Option<i64>,
    pub slice_limit: usize,
}

impl HistoryViewSubsectionTabsView {
    pub fn new(peer_id: i64, mode: SubsectionTabsMode) -> Self {
        Self {
            peer_id,
            mode,
            tabs: Vec::new(),
            active_topic_id: None,
            slice_limit: K_DEFAULT_SUBSECTION_TABS_LIMIT,
        }
    }

    pub fn set_tabs(&mut self, tabs: Vec<TopicTabItem>) {
        self.tabs = tabs;
    }

    pub fn select_topic(&mut self, topic_id: i64) -> bool {
        if self.tabs.iter().any(|t| t.topic_id == topic_id) {
            self.active_topic_id = Some(topic_id);
            true
        } else {
            false
        }
    }

    pub fn default_limit(&self) -> usize {
        K_DEFAULT_SUBSECTION_TABS_LIMIT
    }
}
