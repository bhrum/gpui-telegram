//! Info Requests List Widget view component
//! Upstream: `Telegram/SourceFiles/info/requests_list/info_requests_list_widget.cpp`
//! Upstream: `Telegram/SourceFiles/info/requests_list/info_requests_list_widget.h`
//! Subsystem: info | Feature ID: TCG-N23 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use serde::{Deserialize, Serialize};

/// Upstream `Info::RequestsList::Memento` section memento.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestsListMemento {
    pub peer_id: i64,
    pub scroll_top: i32,
    pub search_query: Option<String>,
}

impl RequestsListMemento {
    pub fn new(peer_id: i64) -> Self {
        Self {
            peer_id,
            scroll_top: 0,
            search_query: None,
        }
    }
}

/// Upstream `Info::RequestsList::Widget` and `InfoRequestsListWidgetRequestsList`.
#[derive(Debug, Clone)]
pub struct InfoRequestsListWidgetRequestsList {
    pub peer_id: i64,
    pub title: String,
    pub search_query: String,
    pub total_pending: usize,
    pub memento: Option<RequestsListMemento>,
}

impl InfoRequestsListWidgetRequestsList {
    pub fn new(peer_id: i64) -> Self {
        Self {
            peer_id,
            title: "Join Requests".into(),
            search_query: String::new(),
            total_pending: 0,
            memento: Some(RequestsListMemento::new(peer_id)),
        }
    }

    /// Upstream `Widget::title()`: returns `lng_profile_join_requests_section`.
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn set_search_query(&mut self, query: impl Into<String>) {
        self.search_query = query.into();
    }

    pub fn set_total_pending(&mut self, count: usize) {
        self.total_pending = count;
    }

    pub fn save_state(&self) -> RequestsListMemento {
        RequestsListMemento {
            peer_id: self.peer_id,
            scroll_top: 0,
            search_query: if self.search_query.is_empty() {
                None
            } else {
                Some(self.search_query.clone())
            },
        }
    }

    pub fn restore_state(&mut self, memento: RequestsListMemento) {
        if let Some(q) = memento.search_query {
            self.search_query = q;
        }
    }
}

impl Default for InfoRequestsListWidgetRequestsList {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Render for InfoRequestsListWidgetRequestsList {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let count_str = if self.total_pending == 1 {
            "1 pending request".to_string()
        } else {
            format!("{} pending requests", self.total_pending)
        };

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xFFFFFF))
            .p(px(16.0))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .pb(px(12.0))
                    .child(
                        div()
                            .text_base()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x222222))
                            .child(self.title.clone())
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x707579))
                            .child(count_str)
                    )
            )
            .child(
                div()
                    .px(px(12.0))
                    .py(px(6.0))
                    .rounded(px(8.0))
                    .bg(rgb(0xF1F2F4))
                    .text_sm()
                    .text_color(rgb(0x707579))
                    .child(if self.search_query.is_empty() {
                        "Search requests...".to_string()
                    } else {
                        self.search_query.clone()
                    })
            )
    }
}
