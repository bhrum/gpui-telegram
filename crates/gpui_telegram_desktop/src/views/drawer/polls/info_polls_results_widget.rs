//! Authentic GPUI implementation of Telegram Poll Results Container Widget.
//! Source: `Telegram/SourceFiles/info/polls/info_polls_results_widget.cpp:L1-L115`, `info_polls_results_widget.h:L1-L70`
//! Subsystem: info | Feature ID: TCG-N38 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use serde::{Deserialize, Serialize};

use super::info_polls_results_inner_widget::InfoPollsResultsInnerWidgetPolls;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollsResultsMemento {
    pub poll_id: u64,
    pub scroll_top: f32,
    pub selected_option_index: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct InfoPollsResultsWidgetPolls {
    pub poll_id: u64,
    pub question_text: String,
    pub inner: InfoPollsResultsInnerWidgetPolls,
    pub scroll_top: f32,
}

impl InfoPollsResultsWidgetPolls {
    pub fn new() -> Self {
        Self {
            poll_id: 0,
            question_text: String::new(),
            inner: InfoPollsResultsInnerWidgetPolls::new(),
            scroll_top: 0.0,
        }
    }

    pub fn for_poll(poll_id: u64, question: String) -> Self {
        let mut w = Self::new();
        w.poll_id = poll_id;
        w.question_text = question;
        w.inner.poll_id = poll_id;
        w
    }

    pub fn save_memento(&self) -> PollsResultsMemento {
        PollsResultsMemento {
            poll_id: self.poll_id,
            scroll_top: self.scroll_top,
            selected_option_index: self.inner.selected_option_index,
        }
    }

    pub fn restore_memento(&mut self, memento: &PollsResultsMemento) {
        self.poll_id = memento.poll_id;
        self.scroll_top = memento.scroll_top;
        self.inner.select_option(memento.selected_option_index);
    }
}

impl Default for InfoPollsResultsWidgetPolls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoPollsResultsWidgetPolls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x0e1621))
            .p(px(8.0))
            // Header Bar
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .pb(px(8.0))
                    .border_b(px(1.0))
                    .border_color(rgb(0x17212b))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .child("Poll Results")
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x707579))
                            .child(format!("{} total votes", self.inner.total_votes))
                    )
            )
            // Question header
            .child(
                div()
                    .py(px(8.0))
                    .text_base()
                    .font_weight(FontWeight::MEDIUM)
                    .child(self.question_text.clone())
            )
            // Results list
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x707579))
                            .child(format!("{} options", self.inner.options.len()))
                    )
            )
    }
}
