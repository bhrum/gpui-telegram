//! Info Members Widget view component
//! Upstream: `Telegram/SourceFiles/info/members/info_members_widget.cpp`
//! Upstream: `Telegram/SourceFiles/info/members/info_members_widget.h`
//! Subsystem: info | Feature ID: TCG-N23 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use serde::{Deserialize, Serialize};

/// Upstream `PeerListState` and `MembersState` memoization.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MembersState {
    pub scroll_top: i32,
    pub search_query: Option<String>,
    pub selected_peer_ids: Vec<i64>,
}

/// Upstream `Info::Members::Memento` section memento.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MembersMemento {
    pub peer_id: i64,
    pub migrated_peer_id: Option<i64>,
    pub state: Option<MembersState>,
}

impl MembersMemento {
    pub fn new(peer_id: i64, migrated_peer_id: Option<i64>) -> Self {
        Self {
            peer_id,
            migrated_peer_id,
            state: None,
        }
    }

    pub fn set_state(&mut self, state: MembersState) {
        self.state = Some(state);
    }

    pub fn take_state(&mut self) -> Option<MembersState> {
        self.state.take()
    }
}

/// Upstream `Info::Members::Widget` and `InfoMembersWidgetMembers`.
#[derive(Debug, Clone)]
pub struct InfoMembersWidgetMembers {
    pub peer_id: i64,
    pub migrated_peer_id: Option<i64>,
    pub is_channel: bool,
    pub total_count: usize,
    pub online_count: usize,
    pub search_query: String,
    pub is_search_enabled: bool,
    pub memento: Option<MembersMemento>,
}

impl InfoMembersWidgetMembers {
    pub const ENABLE_SEARCH_AFTER_COUNT: usize = 20;

    pub fn new(peer_id: i64, is_channel: bool, total_count: usize) -> Self {
        let is_search_enabled = total_count >= Self::ENABLE_SEARCH_AFTER_COUNT;
        Self {
            peer_id,
            migrated_peer_id: None,
            is_channel,
            total_count,
            online_count: 0,
            search_query: String::new(),
            is_search_enabled,
            memento: Some(MembersMemento::new(peer_id, None)),
        }
    }

    pub fn with_migrated_peer(peer_id: i64, migrated: i64, is_channel: bool, count: usize) -> Self {
        let mut widget = Self::new(peer_id, is_channel, count);
        widget.migrated_peer_id = Some(migrated);
        widget.memento = Some(MembersMemento::new(peer_id, Some(migrated)));
        widget
    }

    /// Upstream `Widget::title()`: returns `lng_profile_participants_section` ("Members" vs "Subscribers").
    pub fn title(&self) -> &'static str {
        if self.is_channel {
            "Subscribers"
        } else {
            "Members"
        }
    }

    pub fn apply_search_query(&mut self, query: String) {
        self.search_query = query;
    }

    pub fn set_online_count(&mut self, count: usize) {
        self.online_count = count;
    }

    pub fn save_state(&self) -> MembersState {
        MembersState {
            scroll_top: 0,
            search_query: if self.search_query.is_empty() {
                None
            } else {
                Some(self.search_query.clone())
            },
            selected_peer_ids: Vec::new(),
        }
    }

    pub fn restore_state(&mut self, state: MembersState) {
        if let Some(q) = state.search_query {
            self.search_query = q;
        }
    }
}

impl Default for InfoMembersWidgetMembers {
    fn default() -> Self {
        Self::new(0, false, 0)
    }
}

impl Render for InfoMembersWidgetMembers {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let title_text = self.title();
        let subtitle = if self.online_count > 0 {
            format!("{}, {} online", self.total_count, self.online_count)
        } else {
            format!("{}", self.total_count)
        };

        let mut root = div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xFFFFFF))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .px(px(16.0))
                    .py(px(12.0))
                    .child(
                        div()
                            .text_base()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x222222))
                            .child(title_text)
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x707579))
                            .child(subtitle)
                    )
            );

        if self.is_search_enabled {
            root = root.child(
                div()
                    .px(px(16.0))
                    .py(px(6.0))
                    .child(
                        div()
                            .px(px(12.0))
                            .py(px(6.0))
                            .rounded(px(8.0))
                            .bg(rgb(0xF1F2F4))
                            .text_sm()
                            .text_color(rgb(0x707579))
                            .child(if self.search_query.is_empty() {
                                "Search members...".to_string()
                            } else {
                                self.search_query.clone()
                            })
                    )
            );
        }

        root
    }
}
