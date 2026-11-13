//! Create Poll and Quiz Dialog Box View.
//!
//! Faithful translation of upstream Telegram Desktop:
//! - `Telegram/SourceFiles/boxes/create_poll_box.h`
//! - `Telegram/SourceFiles/boxes/create_poll_box.cpp:L107-L150`
//!
//! Features:
//! - `kQuestionLimit = 255`, `kWarnQuestionLimit = 80`
//! - `kMaxOptionsCount = 10`, `kOptionLimit = 100`, `kWarnOptionLimit = 30`
//! - `kSolutionLimit = 200`, `kWarnSolutionLimit = 60`
//! - Anonymous voting, multiple answers, quiz mode
//! - Correct option selection & solution explanation for quiz mode

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_QUESTION_LIMIT: usize = 255;
pub const K_WARN_QUESTION_LIMIT: usize = 80;
pub const K_MAX_OPTIONS_COUNT: usize = 10;
pub const K_OPTION_LIMIT: usize = 100;
pub const K_WARN_OPTION_LIMIT: usize = 30;
pub const K_SOLUTION_LIMIT: usize = 200;
pub const K_WARN_SOLUTION_LIMIT: usize = 60;
pub const K_MEDIA_UPLOAD_MAX_AGE_MS: u64 = 45 * 60 * 1000;

#[derive(Debug, Clone)]
pub struct PollOptionItem {
    pub text: String,
    pub is_correct: bool,
}

#[derive(Debug, Clone)]
pub struct CreatePollBox {
    pub question: String,
    pub options: Vec<PollOptionItem>,
    pub is_anonymous: bool,
    pub multiple_answers: bool,
    pub is_quiz: bool,
    pub solution: String,
    pub error_message: Option<String>,
}

impl CreatePollBox {
    pub fn new() -> Self {
        Self {
            question: String::new(),
            options: vec![
                PollOptionItem { text: String::new(), is_correct: false },
                PollOptionItem { text: String::new(), is_correct: false },
            ],
            is_anonymous: true,
            multiple_answers: false,
            is_quiz: false,
            solution: String::new(),
            error_message: None,
        }
    }

    pub fn add_option(&mut self, text: impl Into<String>) -> bool {
        if self.options.len() < K_MAX_OPTIONS_COUNT {
            self.options.push(PollOptionItem {
                text: text.into(),
                is_correct: false,
            });
            true
        } else {
            false
        }
    }

    pub fn remove_option(&mut self, index: usize) -> bool {
        if self.options.len() > 2 && index < self.options.len() {
            self.options.remove(index);
            true
        } else {
            false
        }
    }

    pub fn set_correct_option(&mut self, index: usize) {
        if self.is_quiz {
            for (i, opt) in self.options.iter_mut().enumerate() {
                opt.is_correct = (i == index);
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        if self.question.trim().is_empty() || self.question.len() > K_QUESTION_LIMIT {
            return false;
        }
        let valid_opts = self.options.iter().filter(|o| !o.text.trim().is_empty() && o.text.len() <= K_OPTION_LIMIT).count();
        if valid_opts < 2 {
            return false;
        }
        if self.is_quiz {
            if !self.options.iter().any(|o| o.is_correct) {
                return false;
            }
            if self.solution.len() > K_SOLUTION_LIMIT {
                return false;
            }
        }
        true
    }
}

impl Default for CreatePollBox {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for CreatePollBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let title = if self.is_quiz { "New Quiz" } else { "New Poll" };
        let opt_count = self.options.len();

        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(16.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(8.0))
            .border_1()
            .border_color(rgb(0xD2D2D2))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_size(px(15.0))
                    .text_color(rgb(0x1B1B1B))
                    .child(title),
            )
            .child(
                div()
                    .mt(px(8.0))
                    .text_size(px(13.0))
                    .text_color(rgb(0x333333))
                    .child(format!("Question ({} / {} chars): {}", self.question.len(), K_QUESTION_LIMIT, if self.question.is_empty() { "(ask a question...)" } else { &self.question })),
            )
            .child(
                div()
                    .mt(px(6.0))
                    .text_size(px(12.0))
                    .text_color(rgb(0x666666))
                    .child(format!("Options: {} / {} max | Anonymous: {} | Multiple: {}", opt_count, K_MAX_OPTIONS_COUNT, self.is_anonymous, self.multiple_answers)),
            )
            .child(
                div()
                    .mt(px(6.0))
                    .text_size(px(12.0))
                    .text_color(if self.is_valid() { rgb(0x2E7D32) } else { rgb(0xD32F2F) })
                    .child(if self.is_valid() { "Poll is valid for sending" } else { "Enter question and at least 2 options" }),
            )
    }
}
