//! History View Top Bar Widget Component (`history_view_top_bar_widget`)
//!
//! Adaptive top header bar displaying chat info, call actions, and selection controls
//! upstream: `Telegram/SourceFiles/history/view/history_view_top_bar_widget.h` (L1-L292)
//! upstream: `Telegram/SourceFiles/history/view/history_view_top_bar_widget.cpp` (L1-L2042)

use std::time::Duration;

/// Anonymous namespace constants from `history_view_top_bar_widget.cpp:L79-L85`
pub const K_EMOJI_INTERACTION_SEEN_DURATION: Duration = Duration::from_millis(3000);

/// Upstream `history_view_top_bar_widget.cpp:L111-L113`: `SwitchToChooseFromQuery()`
pub const SWITCH_TO_CHOOSE_FROM_QUERY: &str = "from:";

/// Upstream `history_view_top_bar_widget.cpp:L81-L85`: `HasGroupCallMenu`
#[inline]
pub fn has_group_call_menu(is_user: bool, has_active_group_call: bool, can_manage_group_call: bool) -> bool {
    !is_user && !has_active_group_call && can_manage_group_call
}

/// Upstream `history_view_top_bar_widget.cpp:L87-L100`: `TopBarNameText`
pub fn resolve_top_bar_title(
    peer_name: &str,
    is_saved_sublist: bool,
    is_self: bool,
    is_saved_hidden_author: bool,
) -> String {
    if is_saved_sublist {
        if is_self {
            return "My Notes".to_string();
        } else if is_saved_hidden_author {
            return "Messages from Hidden Users".to_string();
        }
    }
    peer_name.to_string()
}

/// Visual sections supported by the top bar
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopBarSection {
    Normal,
    Search,
    Selection,
    PinnedViewer,
}

/// Top bar button action types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopBarAction {
    Back,
    Call,
    GroupCall,
    Search,
    InfoToggle, // Toggles 320px ContextInspectorDrawer
    ClearSelection,
    ForwardSelected,
    DeleteSelected,
    SendNowSelected,
    CancelChoose,
}

/// Top Bar Widget state
/// Upstream: `HistoryView::TopBarWidget`
#[derive(Debug, Clone)]
pub struct HistoryViewTopBarWidgetState {
    pub peer_id: i64,
    pub title: String,
    pub subtitle: String,
    pub section: TopBarSection,
    pub is_user: bool,
    pub has_active_call: bool,
    pub can_manage_group_call: bool,
    pub selected_count: usize,
    pub is_scheduled_view: bool,
    pub info_drawer_open: bool,
    pub search_query: String,
}

impl HistoryViewTopBarWidgetState {
    pub fn new(peer_id: i64, title: String, is_user: bool) -> Self {
        Self {
            peer_id,
            title,
            subtitle: String::new(),
            section: TopBarSection::Normal,
            is_user,
            has_active_call: false,
            can_manage_group_call: false,
            selected_count: 0,
            is_scheduled_view: false,
            info_drawer_open: false,
            search_query: String::new(),
        }
    }

    pub fn set_selection_mode(&mut self, count: usize) {
        self.selected_count = count;
        self.section = if count > 0 {
            TopBarSection::Selection
        } else {
            TopBarSection::Normal
        };
    }

    pub fn can_show_group_call_button(&self) -> bool {
        has_group_call_menu(self.is_user, self.has_active_call, self.can_manage_group_call)
    }

    pub fn toggle_info_drawer(&mut self) -> bool {
        self.info_drawer_open = !self.info_drawer_open;
        self.info_drawer_open
    }

    pub fn start_search_from_user(&mut self) {
        self.section = TopBarSection::Search;
        self.search_query = SWITCH_TO_CHOOSE_FROM_QUERY.to_string();
    }
}
