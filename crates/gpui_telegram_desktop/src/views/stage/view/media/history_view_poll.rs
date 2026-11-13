//! Genuine History View Poll Component (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_poll.h:L25-L140`
//! - `Telegram/SourceFiles/history/view/media/history_view_poll.cpp:L144-L210`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use crate::views::stage::media::poll::{
    count_nice_percent, adjust_percent_count, hash_poll_shuffle_value,
    PollType, RichPollCard,
    K_SHOW_RECENT_VOTERS_COUNT, K_ROTATE_SEGMENTS, K_ROTATE_AMPLITUDE,
    K_SCALE_SEGMENTS, K_SCALE_AMPLITUDE, K_ROLL_DURATION_MS,
    K_EXPIRING_VOTE_RESTRICTION_DURATION_MS, K_VOTE_RESTRICTION_TOAST_DURATION_MS,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PollThumbnailKind {
    None,
    Photo,
    Document,
    Audio,
    Emoji,
    Geo,
    Webpage,
}

#[derive(Debug, Clone)]
pub struct PollAnswerState {
    pub option_id: u32,
    pub text: String,
    pub votes: i32,
    pub percent: i32,
    pub is_chosen: bool,
    pub is_correct: bool,
}

#[derive(Debug, Clone)]
pub struct HistoryViewPollViewMedia {
    pub poll_id: u64,
    pub question: String,
    pub poll_type: PollType,
    pub total_votes: i32,
    pub is_closed: bool,
    pub answers: Vec<PollAnswerState>,
    pub selected_option: Option<u32>,
    pub solution: Option<String>,
}

impl HistoryViewPollViewMedia {
    pub fn new(poll_id: u64, question: impl Into<String>, poll_type: PollType) -> Self {
        Self {
            poll_id,
            question: question.into(),
            poll_type,
            total_votes: 0,
            is_closed: false,
            answers: Vec::new(),
            selected_option: None,
            solution: None,
        }
    }

    pub fn add_answer(&mut self, option_id: u32, text: impl Into<String>, votes: i32, is_correct: bool) {
        self.total_votes += votes;
        self.answers.push(PollAnswerState {
            option_id,
            text: text.into(),
            votes,
            percent: 0,
            is_chosen: false,
            is_correct,
        });
        self.recompute_percents();
    }

    pub fn recompute_percents(&mut self) {
        let votes: Vec<i32> = self.answers.iter().map(|a| a.votes).collect();
        let percents = count_nice_percent(&votes, self.total_votes);
        for (i, p) in percents.into_iter().enumerate() {
            if let Some(ans) = self.answers.get_mut(i) {
                ans.percent = p;
            }
        }
    }

    pub fn vote(&mut self, option_id: u32) -> bool {
        if self.is_closed {
            return false;
        }
        self.selected_option = Some(option_id);
        for ans in &mut self.answers {
            if ans.option_id == option_id {
                ans.is_chosen = true;
                ans.votes += 1;
                self.total_votes += 1;
            } else {
                ans.is_chosen = false;
            }
        }
        self.recompute_percents();
        true
    }

    pub fn close_poll(&mut self) {
        self.is_closed = true;
    }
}

impl Default for HistoryViewPollViewMedia {
    fn default() -> Self {
        Self::new(0, "Default Poll", PollType::Regular)
    }
}

impl Render for HistoryViewPollViewMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut card = RichPollCard::new(self.poll_id, &self.question, self.poll_type);
        card.is_closed = self.is_closed;
        card.selected_option_id = self.selected_option;
        card.solution = self.solution.clone();
        for ans in &self.answers {
            card.add_option(ans.option_id, &ans.text, ans.votes, ans.is_correct);
        }
        card.render()
    }
}
