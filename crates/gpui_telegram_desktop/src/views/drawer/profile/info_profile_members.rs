//! Info Profile Members component
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_members.cpp:L1-L497`
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_members.h:L1-L149`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use serde::{Deserialize, Serialize};

/// Upstream `Info::Profile::MembersState`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileMembersState {
    pub search_query: Option<String>,
    pub group_by_role: bool,
}

/// Members section embedded in group profile drawers with inline search and role indicators.
#[derive(Debug, Clone)]
pub struct InfoProfileMembersProfile {
    pub channel_id: i64,
    pub total_count: usize,
    pub online_count: usize,
    pub search_query: String,
    pub is_search_enabled: bool,
    pub group_by_role: bool,
    pub is_loading: bool,
}

impl InfoProfileMembersProfile {
    /// Upstream `kEnableSearchMembersAfterCount = 20` (`info_profile_members.cpp:L40`).
    pub const ENABLE_SEARCH_AFTER_COUNT: usize = 20;

    pub fn new(channel_id: i64, total_count: usize) -> Self {
        let is_search_enabled = total_count >= Self::ENABLE_SEARCH_AFTER_COUNT;
        Self {
            channel_id,
            total_count,
            online_count: 0,
            search_query: String::new(),
            is_search_enabled,
            group_by_role: true,
            is_loading: false,
        }
    }

    pub fn set_search_query(&mut self, query: impl Into<String>) {
        self.search_query = query.into();
    }

    pub fn set_group_by_role(&mut self, grouped: bool) {
        self.group_by_role = grouped;
    }

    pub fn set_online_count(&mut self, count: usize) {
        self.online_count = count;
    }

    pub fn save_state(&self) -> ProfileMembersState {
        ProfileMembersState {
            search_query: if self.search_query.is_empty() {
                None
            } else {
                Some(self.search_query.clone())
            },
            group_by_role: self.group_by_role,
        }
    }

    pub fn restore_state(&mut self, state: ProfileMembersState) {
        if let Some(q) = state.search_query {
            self.search_query = q;
        }
        self.group_by_role = state.group_by_role;
    }
}

impl Default for InfoProfileMembersProfile {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Render for InfoProfileMembersProfile {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let count_text = if self.online_count > 0 {
            format!("{} members, {} online", self.total_count, self.online_count)
        } else {
            format!("{} members", self.total_count)
        };

        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(8.0))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .text_xs()
                    .text_color(rgb(0x708499))
                    .child(count_text)
            )
            .children(if self.is_search_enabled {
                Some(
                    div()
                        .pt(px(6.0))
                        .text_xs()
                        .text_color(rgb(0x999999))
                        .child("Search enabled"),
                )
            } else {
                None
            })
    }
}
