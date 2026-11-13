//! Compose search bar in conversation stage with calendar jumping and filter pills.
//! Upstream reference: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_compose_search.cpp`
//! Upstream header: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_compose_search.h`

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposeSearchState {
    pub query: String,
    pub top_msg_id: i32,
    pub is_calendar_active: bool,
    pub match_count: usize,
    pub current_index: usize,
}

impl Default for ComposeSearchState {
    fn default() -> Self {
        Self {
            query: String::new(),
            top_msg_id: 0,
            is_calendar_active: false,
            match_count: 0,
            current_index: 0,
        }
    }
}

pub struct HistoryViewComposeSearchViewControls {
    pub state: ComposeSearchState,
    pub is_visible: bool,
}

impl HistoryViewComposeSearchViewControls {
    pub fn new() -> Self {
        Self {
            state: ComposeSearchState::default(),
            is_visible: false,
        }
    }

    pub fn set_query(&mut self, query: impl Into<String>) {
        self.state.query = query.into();
    }

    pub fn next_match(&mut self) {
        if self.state.match_count > 0 {
            self.state.current_index = (self.state.current_index + 1) % self.state.match_count;
        }
    }

    pub fn prev_match(&mut self) {
        if self.state.match_count > 0 {
            self.state.current_index = if self.state.current_index == 0 {
                self.state.match_count - 1
            } else {
                self.state.current_index - 1
            };
        }
    }
}

impl Default for HistoryViewComposeSearchViewControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewComposeSearchViewControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }

        let query = self.state.query.clone();
        let matches = format!("{}/{}", self.state.current_index + 1, self.state.match_count.max(1));

        div()
            .flex()
            .items_center()
            .justify_between()
            .h_10()
            .px_3()
            .bg(rgb(0x18222d))
            .border_b_1()
            .border_color(rgb(0x2a394a))
            .text_sm()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .text_color(rgb(0x8e8e93))
                            .child("🔍"),
                    )
                    .child(
                        div()
                            .text_color(rgb(0xffffff))
                            .child(if query.is_empty() {
                                "Search in chat...".to_string()
                            } else {
                                query
                            }),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x8e8e93))
                            .child(matches),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_0p5()
                            .rounded_sm()
                            .bg(rgb(0x2481cc))
                            .text_xs()
                            .text_color(rgb(0xffffff))
                            .child("Done"),
                    ),
            )
            .into_any_element()
    }
}
