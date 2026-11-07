//! History View Top Bar Widget View Component (`history_view_top_bar_widget`)
//!
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_top_bar_widget.h` (L1-L292)
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_top_bar_widget.cpp` (L1-L2042)

use crate::views::stage::top_bar::{
    has_group_call_menu, resolve_top_bar_title, HistoryViewTopBarWidgetState,
    TopBarAction, TopBarSection, K_EMOJI_INTERACTION_SEEN_DURATION,
    SWITCH_TO_CHOOSE_FROM_QUERY,
};

/// GPUI view component for top bar widget
#[derive(Debug, Clone)]
pub struct HistoryViewTopBarWidgetView {
    pub inner: HistoryViewTopBarWidgetState,
    pub height: f32,
    pub is_narrow_mobile: bool,
}

impl HistoryViewTopBarWidgetView {
    pub fn new(peer_id: i64, title: String, is_user: bool) -> Self {
        Self {
            inner: HistoryViewTopBarWidgetState::new(peer_id, title, is_user),
            height: 56.0,
            is_narrow_mobile: false,
        }
    }

    pub fn set_selection(&mut self, count: usize) {
        self.inner.set_selection_mode(count);
    }

    pub fn is_selection_active(&self) -> bool {
        self.inner.section == TopBarSection::Selection
    }

    pub fn can_show_group_call(&self) -> bool {
        self.inner.can_show_group_call_button()
    }

    pub fn toggle_info_panel(&mut self) -> bool {
        self.inner.toggle_info_drawer()
    }

    pub fn search_query_prefix(&self) -> &'static str {
        SWITCH_TO_CHOOSE_FROM_QUERY
    }

    pub fn emoji_seen_duration(&self) -> std::time::Duration {
        K_EMOJI_INTERACTION_SEEN_DURATION
    }

    pub fn resolve_title(&self, is_sublist: bool, is_self: bool, is_hidden: bool) -> String {
        resolve_top_bar_title(&self.inner.title, is_sublist, is_self, is_hidden)
    }
}
