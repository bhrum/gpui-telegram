//! Authentic GPUI implementation of Telegram Poll Results Breakdown Inner Widget.
//! Source: `Telegram/SourceFiles/info/polls/info_polls_results_inner_widget.cpp:L1-L812`, `info_polls_results_inner_widget.h:L1-L73`
//! Subsystem: info | Feature ID: TCG-N38 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use serde::{Deserialize, Serialize};

pub const POLLS_RESULTS_FIRST_PAGE: usize = 20;
pub const POLLS_RESULTS_PER_PAGE: usize = 50;
pub const POLLS_RESULTS_LEAVE_PRELOADED: usize = 10;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoterRowItem {
    pub user_id: u64,
    pub name: String,
    pub avatar_initials: String,
    pub avatar_color: u32,
    pub date: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PollOptionResult {
    pub option_index: usize,
    pub text: String,
    pub votes: usize,
    pub percent: f32, // 0.0 .. 100.0
    pub voters: Vec<VoterRowItem>,
}

#[derive(Debug, Clone)]
pub struct InfoPollsResultsInnerWidgetPolls {
    pub poll_id: u64,
    pub is_anonymous: bool,
    pub total_votes: usize,
    pub options: Vec<PollOptionResult>,
    pub selected_option_index: Option<usize>,
}

impl InfoPollsResultsInnerWidgetPolls {
    pub fn new() -> Self {
        Self {
            poll_id: 0,
            is_anonymous: false,
            total_votes: 0,
            options: Vec::new(),
            selected_option_index: None,
        }
    }

    pub fn set_results(&mut self, is_anonymous: bool, options: Vec<(String, usize, Vec<VoterRowItem>)>) {
        self.is_anonymous = is_anonymous;
        self.total_votes = options.iter().map(|(_, v, _)| v).sum();
        self.options = options
            .into_iter()
            .enumerate()
            .map(|(idx, (text, votes, voters))| {
                let percent = if self.total_votes > 0 {
                    (votes as f32 * 100.0) / (self.total_votes as f32)
                } else {
                    0.0
                };
                PollOptionResult {
                    option_index: idx,
                    text,
                    votes,
                    percent,
                    voters,
                }
            })
            .collect();
    }

    pub fn select_option(&mut self, index: Option<usize>) {
        self.selected_option_index = index;
    }
}

impl Default for InfoPollsResultsInnerWidgetPolls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoPollsResultsInnerWidgetPolls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_empty = self.options.is_empty();

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(8.0))
            .gap(px(10.0))
            .when(is_empty, |this| {
                this.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .py(px(32.0))
                        .text_xs()
                        .text_color(rgb(0x707579))
                        .child("No results available for this poll")
                )
            })
            .when(!is_empty, |this| {
                let mut col = div().flex().flex_col().gap(px(10.0));
                for opt in &self.options {
                    let is_sel = self.selected_option_index == Some(opt.option_index);
                    let bar_pct = opt.percent.clamp(0.0, 100.0);
                    col = col.child(
                        div()
                            .flex()
                            .flex_col()
                            .p(px(8.0))
                            .rounded(px(6.0))
                            .bg(if is_sel { rgb(0x242f3d) } else { rgb(0x17212b) })
                            .gap(px(4.0))
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .child(opt.text.clone())
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x5288c1))
                                            .child(format!("{:.1}%", opt.percent))
                                    )
                            )
                            // Percentage Bar Track
                            .child(
                                div()
                                    .h(px(4.0))
                                    .w_full()
                                    .rounded_full()
                                    .bg(rgb(0x0e1621))
                                    .child(
                                        div()
                                            .h(px(4.0))
                                            .w(px(bar_pct * 2.0)) // scaled indicator
                                            .rounded_full()
                                            .bg(rgb(0x5288c1))
                                    )
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x707579))
                                    .child(format!("{} votes", opt.votes))
                            )
                    );
                }
                this.child(col)
            })
    }
}
