//! Info Community Requests Widget view component
//! Upstream: `Telegram/SourceFiles/info/community_requests/info_community_requests_widget.cpp`
//! Upstream: `Telegram/SourceFiles/info/community_requests/info_community_requests_widget.h`
//! Subsystem: info | Feature ID: TCG-N23 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use serde::{Deserialize, Serialize};

/// Upstream `JoinRequestRow` entry in the moderation queue.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JoinRequestRow {
    pub user_id: i64,
    pub name: String,
    pub username: Option<String>,
    pub bio: Option<String>,
    pub requested_date: i64,
    pub is_pending: bool,
}

/// Upstream `Info::CommunityRequests::Widget` and `InfoCommunityRequestsWidgetCommunityRequests`.
#[derive(Debug, Clone)]
pub struct InfoCommunityRequestsWidgetCommunityRequests {
    pub peer_id: i64,
    pub title: String,
    pub requests: Vec<JoinRequestRow>,
    pub undo_queue: Vec<JoinRequestRow>,
    pub is_loading: bool,
    pub last_action_timestamp: Option<i64>,
}

impl InfoCommunityRequestsWidgetCommunityRequests {
    pub const PER_PAGE: usize = 100;
    pub const ACCEPT_BUTTON: usize = 1;
    pub const REJECT_BUTTON: usize = 2;
    pub const USER_LINK: usize = 3;
    pub const HIDDEN_PILL: usize = 4;
    pub const UNDO_TOAST_DURATION_MS: u64 = 3000;
    pub const PENDING_ROW_OPACITY: f32 = 0.4;

    pub fn new(peer_id: i64) -> Self {
        Self {
            peer_id,
            title: "Join Requests".into(),
            requests: Vec::new(),
            undo_queue: Vec::new(),
            is_loading: false,
            last_action_timestamp: None,
        }
    }

    pub fn add_request(&mut self, request: JoinRequestRow) {
        if !self.requests.iter().any(|r| r.user_id == request.user_id) {
            self.requests.push(request);
        }
    }

    pub fn accept_request(&mut self, user_id: i64) -> Option<JoinRequestRow> {
        if let Some(pos) = self.requests.iter().position(|r| r.user_id == user_id) {
            let mut removed = self.requests.remove(pos);
            removed.is_pending = true;
            self.undo_queue.push(removed.clone());
            Some(removed)
        } else {
            None
        }
    }

    pub fn reject_request(&mut self, user_id: i64) -> Option<JoinRequestRow> {
        if let Some(pos) = self.requests.iter().position(|r| r.user_id == user_id) {
            let mut removed = self.requests.remove(pos);
            removed.is_pending = true;
            self.undo_queue.push(removed.clone());
            Some(removed)
        } else {
            None
        }
    }

    pub fn process_all(&mut self, _accept: bool) -> usize {
        let count = self.requests.len();
        self.undo_queue.extend(self.requests.drain(..));
        count
    }

    pub fn undo_last_action(&mut self) -> bool {
        if let Some(restored) = self.undo_queue.pop() {
            let mut active = restored;
            active.is_pending = false;
            self.requests.push(active);
            true
        } else {
            false
        }
    }

    pub fn pending_count(&self) -> usize {
        self.requests.len()
    }
}

impl Default for InfoCommunityRequestsWidgetCommunityRequests {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Render for InfoCommunityRequestsWidgetCommunityRequests {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.requests.len();
        let header_text = if count == 1 {
            "1 request to join".to_string()
        } else {
            format!("{} requests to join", count)
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
                            .child(header_text)
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .children(self.requests.iter().map(|req| {
                        let opacity = if req.is_pending {
                            Self::PENDING_ROW_OPACITY
                        } else {
                            1.0
                        };
                        div()
                            .flex()
                            .justify_between()
                            .items_center()
                            .p(px(8.0))
                            .rounded(px(8.0))
                            .bg(rgb(0xF9FAFB))
                            .opacity(opacity)
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(rgb(0x222222))
                                            .child(req.name.clone())
                                    )
                                    .when_some(req.username.clone(), |this, u| {
                                        this.child(
                                            div()
                                                .text_xs()
                                                .text_color(rgb(0x707579))
                                                .child(format!("@{}", u))
                                        )
                                    })
                            )
                    }))
            )
    }
}
