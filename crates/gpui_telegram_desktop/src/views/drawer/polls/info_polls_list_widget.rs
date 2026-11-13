//! Authentic GPUI implementation of Telegram Shared Polls List Drawer Widget.
//! Source: `Telegram/SourceFiles/info/polls/info_polls_list_widget.cpp:L1-L1100`, `info_polls_list_widget.h:L1-L117`
//! Subsystem: info | Feature ID: TCG-N38 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatPollSummary {
    pub poll_id: u64,
    pub message_id: u64,
    pub question: String,
    pub total_voters: usize,
    pub is_closed: bool,
    pub is_quiz: bool,
    pub date: i64,
}

#[derive(Debug, Clone)]
pub struct InfoPollsListWidgetPolls {
    pub peer_id: u64,
    pub polls: Vec<ChatPollSummary>,
    pub selected_poll_id: Option<u64>,
    pub search_query: String,
}

impl InfoPollsListWidgetPolls {
    pub fn new() -> Self {
        Self {
            peer_id: 0,
            polls: Vec::new(),
            selected_poll_id: None,
            search_query: String::new(),
        }
    }

    pub fn for_peer(peer_id: u64) -> Self {
        let mut p = Self::new();
        p.peer_id = peer_id;
        p
    }

    pub fn add_poll(&mut self, poll: ChatPollSummary) {
        if !self.polls.iter().any(|p| p.poll_id == poll.poll_id) {
            self.polls.push(poll);
        }
    }

    pub fn select_poll(&mut self, poll_id: Option<u64>) {
        self.selected_poll_id = poll_id;
    }

    pub fn filtered_polls(&self) -> Vec<&ChatPollSummary> {
        if self.search_query.trim().is_empty() {
            self.polls.iter().collect()
        } else {
            let q = self.search_query.to_lowercase();
            self.polls
                .iter()
                .filter(|p| p.question.to_lowercase().contains(&q))
                .collect()
        }
    }
}

impl Default for InfoPollsListWidgetPolls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoPollsListWidgetPolls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let list = self.filtered_polls();
        let is_empty = list.is_empty();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x0e1621))
            .p(px(8.0))
            .gap(px(6.0))
            .when(is_empty, |this| {
                this.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .py(px(32.0))
                        .text_xs()
                        .text_color(rgb(0x707579))
                        .child("No shared polls found")
                )
            })
            .when(!is_empty, |this| {
                let mut col = div().flex().flex_col().gap(px(4.0));
                for poll in list {
                    let is_active = self.selected_poll_id == Some(poll.poll_id);
                    col = col.child(
                        div()
                            .flex()
                            .flex_col()
                            .p(px(10.0))
                            .rounded(px(6.0))
                            .bg(if is_active { rgb(0x242f3d) } else { rgb(0x17212b) })
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x5288c1))
                                            .child(if poll.is_quiz { "Quiz" } else { "Poll" })
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x707579))
                                            .child(if poll.is_closed { "Final Results" } else { "Open" })
                                    )
                            )
                            .child(
                                div()
                                    .mt(px(2.0))
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .child(poll.question.clone())
                            )
                            .child(
                                div()
                                    .mt(px(4.0))
                                    .text_xs()
                                    .text_color(rgb(0x707579))
                                    .child(format!("{} votes", poll.total_voters))
                            )
                    );
                }
                this.child(col)
            })
    }
}
