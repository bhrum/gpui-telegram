//! Genuine History View Poll Menu Component (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/menu/history_view_poll_menu.h`
//! - `Telegram/SourceFiles/history/view/media/menu/history_view_poll_menu.cpp`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PollMenuAction {
    RetractVote,
    SubmitVote,
    ViewResults,
    ClosePoll,
}

#[derive(Debug, Clone)]
pub struct HistoryViewPollMenuViewMediaMenu {
    pub poll_id: u64,
    pub can_retract: bool,
    pub can_submit: bool,
    pub can_close: bool,
}

impl HistoryViewPollMenuViewMediaMenu {
    pub fn new(poll_id: u64) -> Self {
        Self {
            poll_id,
            can_retract: false,
            can_submit: false,
            can_close: false,
        }
    }

    pub fn with_retract(mut self, can: bool) -> Self {
        self.can_retract = can;
        self
    }

    pub fn with_close(mut self, can: bool) -> Self {
        self.can_close = can;
        self
    }
}

impl Default for HistoryViewPollMenuViewMediaMenu {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Render for HistoryViewPollMenuViewMediaMenu {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w(px(200.0))
            .p(px(6.0))
            .rounded(px(8.0))
            .bg(rgb(0x182533))
            .border_1()
            .border_color(rgb(0x233140))
            .gap(px(4.0))
            .when(self.can_retract, |el| {
                el.child(
                    div()
                        .px(px(10.0))
                        .py(px(6.0))
                        .rounded(px(4.0))
                        .text_size(px(12.0))
                        .text_color(rgb(0xE55B5B))
                        .child("Retract Vote"),
                )
            })
            .when(self.can_close, |el| {
                el.child(
                    div()
                        .px(px(10.0))
                        .py(px(6.0))
                        .rounded(px(4.0))
                        .text_size(px(12.0))
                        .text_color(rgb(0xFFFFFF))
                        .child("Stop Poll"),
                )
            })
    }
}
