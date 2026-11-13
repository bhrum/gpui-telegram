//! Info Profile Tab Polls Adapter
//! Upstream: `Telegram/SourceFiles/info/profile/tabs/adapters/info_profile_tab_polls.cpp:L1-L204`
//! Upstream: `Telegram/SourceFiles/info/profile/tabs/adapters/info_profile_tab_polls.h:L1-L17`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use gpui::*;
use serde::{Deserialize, Serialize};

/// Option within a poll question card.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PollOptionItem {
    pub text: String,
    pub vote_count: usize,
    pub vote_percentage: f32,
    pub is_chosen: bool,
}

/// Poll question card containing metadata and options.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PollCard {
    pub poll_id: u64,
    pub question: String,
    pub total_votes: usize,
    pub is_closed: bool,
    pub is_anonymous: bool,
    pub options: Vec<PollOptionItem>,
}

impl PollCard {
    pub fn update_percentages(&mut self) {
        let total: usize = self.options.iter().map(|o| o.vote_count).sum();
        self.total_votes = total;
        if total == 0 {
            for opt in &mut self.options {
                opt.vote_percentage = 0.0;
            }
        } else {
            for opt in &mut self.options {
                opt.vote_percentage = (opt.vote_count as f32 / total as f32) * 100.0;
            }
        }
    }
}

/// Tab adapter bridging the profile tabs host with channel polls archive.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InfoProfileTabPollsProfileTabsAdapters {
    pub channel_id: i64,
    pub polls_count: usize,
    pub is_active: bool,
    pub polls: Vec<PollCard>,
}

impl Default for InfoProfileTabPollsProfileTabsAdapters {
    fn default() -> Self {
        Self::new(0)
    }
}

impl InfoProfileTabPollsProfileTabsAdapters {
    pub fn new(channel_id: i64) -> Self {
        Self {
            channel_id,
            polls_count: 0,
            is_active: false,
            polls: Vec::new(),
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn add_poll(&mut self, mut poll: PollCard) {
        poll.update_percentages();
        self.polls.push(poll);
        self.polls_count = self.polls.len();
    }

    pub fn vote(&mut self, poll_id: u64, option_idx: usize) -> bool {
        if let Some(poll) = self.polls.iter_mut().find(|p| p.poll_id == poll_id) {
            if poll.is_closed || option_idx >= poll.options.len() {
                return false;
            }
            poll.options[option_idx].vote_count += 1;
            poll.options[option_idx].is_chosen = true;
            poll.update_percentages();
            true
        } else {
            false
        }
    }

    pub fn close_poll(&mut self, poll_id: u64) -> bool {
        if let Some(poll) = self.polls.iter_mut().find(|p| p.poll_id == poll_id) {
            poll.is_closed = true;
            true
        } else {
            false
        }
    }
}

impl Render for InfoProfileTabPollsProfileTabsAdapters {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_empty = self.polls.is_empty();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xF5F7FA))
            .p(px(12.0))
            .gap(px(12.0))
            .child(
                if is_empty {
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .h(px(160.0))
                        .text_sm()
                        .text_color(rgb(0x707579))
                        .child("No polls found in this channel")
                } else {
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(10.0))
                        .children(self.polls.iter().map(|poll| {
                            div()
                                .flex()
                                .flex_col()
                                .bg(rgb(0xFFFFFF))
                                .rounded_md()
                                .p(px(12.0))
                                .gap(px(8.0))
                                .border_1()
                                .border_color(rgb(0xE6E8EB))
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .justify_between()
                                        .child(
                                            div()
                                                .font_weight(FontWeight::BOLD)
                                                .text_sm()
                                                .text_color(rgb(0x222222))
                                                .child(poll.question.clone())
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(if poll.is_closed { rgb(0xE05353) } else { rgb(0x2AABEE) })
                                                .child(if poll.is_closed { "Closed" } else { "Open" })
                                        )
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap(px(6.0))
                                        .children(poll.options.iter().map(|opt| {
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(2.0))
                                                .child(
                                                    div()
                                                        .flex()
                                                        .flex_row()
                                                        .justify_between()
                                                        .text_xs()
                                                        .text_color(rgb(0x333333))
                                                        .child(opt.text.clone())
                                                        .child(format!("{:.1}%", opt.vote_percentage))
                                                )
                                                .child(
                                                    div()
                                                        .w_full()
                                                        .h(px(4.0))
                                                        .rounded_full()
                                                        .bg(rgb(0xE6E8EB))
                                                        .child(
                                                            div()
                                                                .h_full()
                                                                .rounded_full()
                                                                .bg(if opt.is_chosen { rgb(0x2AABEE) } else { rgb(0x708499) })
                                                        )
                                                )
                                        }))
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(0x707579))
                                        .child(format!("{} votes • {}", poll.total_votes, if poll.is_anonymous { "Anonymous" } else { "Public" }))
                                )
                        }))
                }
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polls_adapter_voting_and_closure() {
        let mut adapter = InfoProfileTabPollsProfileTabsAdapters::new(2002);
        assert_eq!(adapter.channel_id, 2002);
        assert_eq!(adapter.polls_count, 0);

        let poll = PollCard {
            poll_id: 101,
            question: "Which UI toolkit do you prefer?".into(),
            total_votes: 0,
            is_closed: false,
            is_anonymous: true,
            options: vec![
                PollOptionItem { text: "GPUI".into(), vote_count: 0, vote_percentage: 0.0, is_chosen: false },
                PollOptionItem { text: "Qt".into(), vote_count: 0, vote_percentage: 0.0, is_chosen: false },
            ],
        };
        adapter.add_poll(poll);
        assert_eq!(adapter.polls_count, 1);

        assert!(adapter.vote(101, 0));
        assert_eq!(adapter.polls[0].options[0].vote_count, 1);
        assert_eq!(adapter.polls[0].options[0].vote_percentage, 100.0);
        assert_eq!(adapter.polls[0].total_votes, 1);

        assert!(adapter.vote(101, 1));
        assert_eq!(adapter.polls[0].options[1].vote_count, 1);
        assert_eq!(adapter.polls[0].options[0].vote_percentage, 50.0);
        assert_eq!(adapter.polls[0].options[1].vote_percentage, 50.0);
        assert_eq!(adapter.polls[0].total_votes, 2);

        assert!(adapter.close_poll(101));
        assert!(adapter.polls[0].is_closed);
        assert!(!adapter.vote(101, 0)); // voting on closed poll fails
    }
}
