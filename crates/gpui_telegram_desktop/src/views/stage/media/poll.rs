//! Rich Poll & Quiz Media Card (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_poll.cpp:L144-L210`
//! - `Telegram/SourceFiles/history/view/media/history_view_dice.cpp:L23-L106`
//! - `Telegram/SourceFiles/history/view/media/history_view_slot_machine.cpp:L21-L68`
//! - `Telegram/SourceFiles/history/view/media/history_view_custom_emoji.cpp:L38-L56`

use gpui::*;
use gpui::prelude::FluentBuilder;

// Upstream C++ constants from history_view_poll.cpp
pub const K_SHOW_RECENT_VOTERS_COUNT: usize = 3;
pub const K_ROTATE_SEGMENTS: usize = 8;
pub const K_ROTATE_AMPLITUDE: f32 = 3.0;
pub const K_SCALE_SEGMENTS: usize = 2;
pub const K_SCALE_AMPLITUDE: f32 = 0.03;
pub const K_ROLL_DURATION_MS: u64 = 400;
pub const K_EXPIRING_VOTE_RESTRICTION_DURATION_MS: u64 = 600_000;
pub const K_VOTE_RESTRICTION_TOAST_DURATION_MS: u64 = 5_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PollType {
    Regular,
    Quiz,
    Anonymous,
}

#[derive(Debug, Clone)]
pub struct PercentCounterItem {
    pub index: usize,
    pub percent: i32,
    pub remainder: i32,
}

/// Upstream AdjustPercentCount from `history_view_poll.cpp:L144-L171`
pub fn adjust_percent_count(items: &mut [PercentCounterItem], mut left: i32) {
    items.sort_by(|a, b| {
        if a.remainder != b.remainder {
            b.remainder.cmp(&a.remainder)
        } else {
            a.percent.cmp(&b.percent)
        }
    });

    let count = items.len();
    let mut i = 0;
    while i < count {
        let item_remainder = items[i].remainder;
        let item_percent = items[i].percent;
        let mut j = i + 1;
        while j < count {
            if items[j].percent != item_percent || items[j].remainder != item_remainder {
                break;
            }
            j += 1;
        }

        if items[i].remainder == 0 {
            break;
        }

        let equal = (j - i) as i32;
        if equal <= left {
            left -= equal;
            for k in i..j {
                items[k].percent += 1;
            }
        }
        i = j;
    }
}

/// Upstream CountNicePercent from `history_view_poll.cpp:L173-L200`
pub fn count_nice_percent(votes: &[i32], total: i32) -> Vec<i32> {
    let count = votes.len();
    if total <= 0 || count == 0 {
        return vec![0; count];
    }

    let mut items: Vec<PercentCounterItem> = Vec::with_capacity(count);
    let mut left = 100;

    for (index, &v) in votes.iter().enumerate() {
        let percent = (v * 100) / total;
        let remainder = (v * 100) - (percent * total);
        left -= percent;
        items.push(PercentCounterItem {
            index,
            percent,
            remainder,
        });
    }

    if left > 0 && left <= (count as i32) {
        adjust_percent_count(&mut items, left);
    }

    let mut result = vec![0; count];
    for item in items {
        result[item.index] = item.percent;
    }
    result
}

/// CRC32 poll option shuffle hash from `history_view_poll.cpp:L202-L210`
pub fn hash_poll_shuffle_value(user_id: u64, poll_id: u64, option: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    let bytes_uid = user_id.to_string();
    let bytes_pid = poll_id.to_string();
    for &b in bytes_uid.as_bytes().iter().chain(option).chain(bytes_pid.as_bytes()) {
        crc ^= b as u32;
        for _ in 0..8 {
            if (crc & 1) != 0 {
                crc = (crc >> 1) ^ 0xEDB8_8320;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}

/// Slot machine reel symbols from `history_view_slot_machine.cpp:L25-L31`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotReelSymbol {
    Bar = 0,
    Berries = 1,
    Lemon = 2,
    Seven = 3,
}

pub const K_SLOT_WIN_VALUE: u8 = 64;
pub const K_SLOT_SHIFTS: [usize; 3] = [3, 9, 15];

/// Slot machine 3-reel outcome decoder from `history_view_slot_machine.cpp:L49-L68`
pub fn decode_slot_machine_reels(value: u8) -> [SlotReelSymbol; 3] {
    if value == K_SLOT_WIN_VALUE {
        return [SlotReelSymbol::Seven, SlotReelSymbol::Seven, SlotReelSymbol::Seven];
    }
    let safe_val = value.clamp(1, 64) - 1;
    let r0 = match (safe_val >> (0 * 2)) & 0x03 {
        0 => SlotReelSymbol::Bar,
        1 => SlotReelSymbol::Berries,
        2 => SlotReelSymbol::Lemon,
        _ => SlotReelSymbol::Seven,
    };
    let r1 = match (safe_val >> (1 * 2)) & 0x03 {
        0 => SlotReelSymbol::Bar,
        1 => SlotReelSymbol::Berries,
        2 => SlotReelSymbol::Lemon,
        _ => SlotReelSymbol::Seven,
    };
    let r2 = match (safe_val >> (2 * 2)) & 0x03 {
        0 => SlotReelSymbol::Bar,
        1 => SlotReelSymbol::Berries,
        2 => SlotReelSymbol::Lemon,
        _ => SlotReelSymbol::Seven,
    };
    [r0, r1, r2]
}

/// Custom emoji scale factors from `history_view_custom_emoji.cpp:L38-L56`
pub fn custom_emoji_dimension_scale(count: usize) -> f32 {
    match count {
        1 => 1.0,
        2 => 0.7,
        3 => 0.52,
        4..=5 => 0.40,
        _ => 0.32,
    }
}

#[derive(Debug, Clone)]
pub struct RichPollOption {
    pub id: u32,
    pub text: String,
    pub votes: i32,
    pub is_correct: bool,
}

#[derive(Debug, Clone)]
pub struct RichPollCard {
    pub poll_id: u64,
    pub question: String,
    pub poll_type: PollType,
    pub options: Vec<RichPollOption>,
    pub total_votes: i32,
    pub is_closed: bool,
    pub selected_option_id: Option<u32>,
    pub solution: Option<String>,
}

impl RichPollCard {
    pub fn new(poll_id: u64, question: impl Into<String>, poll_type: PollType) -> Self {
        Self {
            poll_id,
            question: question.into(),
            poll_type,
            options: Vec::new(),
            total_votes: 0,
            is_closed: false,
            selected_option_id: None,
            solution: None,
        }
    }

    pub fn add_option(&mut self, id: u32, text: impl Into<String>, votes: i32, is_correct: bool) {
        self.total_votes += votes;
        self.options.push(RichPollOption {
            id,
            text: text.into(),
            votes,
            is_correct,
        });
    }

    pub fn compute_percentages(&self) -> Vec<i32> {
        let votes: Vec<i32> = self.options.iter().map(|o| o.votes).collect();
        count_nice_percent(&votes, self.total_votes)
    }

    pub fn render(self) -> impl IntoElement {
        let percentages = self.compute_percentages();
        let has_voted = self.selected_option_id.is_some() || self.is_closed;

        let type_label = match self.poll_type {
            PollType::Regular => "Poll",
            PollType::Quiz => "Quiz",
            PollType::Anonymous => "Anonymous Poll",
        };
        let is_closed = self.is_closed;
        let question = self.question;
        let selected_option_id = self.selected_option_id;
        let poll_type = self.poll_type;
        let options = self.options;
        let solution = self.solution;
        let total_votes = self.total_votes;

        div()
            .flex()
            .flex_col()
            .w(px(360.0))
            .p(px(14.0))
            .rounded(px(12.0))
            .bg(rgb(0x182533))
            .border_1()
            .border_color(rgb(0x233140))
            .gap(px(10.0))
            // Header
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x5288C1))
                            .child(type_label),
                    )
                    .when(is_closed, |el| {
                        el.child(
                            div()
                                .px(px(6.0))
                                .py(px(2.0))
                                .rounded(px(4.0))
                                .bg(rgba(0x00000044))
                                .text_size(px(10.0))
                                .text_color(rgb(0x8A9AA8))
                                .child("Final Results"),
                        )
                    }),
            )
            // Question
            .child(
                div()
                    .text_size(px(15.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(0xFFFFFF))
                    .line_height(px(20.0))
                    .child(question),
            )
            // Options list
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .children(options.into_iter().enumerate().map(move |(idx, opt)| {
                        let pct = percentages.get(idx).copied().unwrap_or(0);
                        let is_choice = selected_option_id == Some(opt.id);

                        div()
                            .flex()
                            .flex_col()
                            .gap(px(4.0))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(8.0))
                                            // Radio or Checkbox
                                            .child(
                                                div()
                                                    .w(px(18.0))
                                                    .h(px(18.0))
                                                    .rounded_full()
                                                    .border_2()
                                                    .border_color(if is_choice { rgb(0x2AABEE) } else { rgb(0x6B808D) })
                                                    .bg(if is_choice { rgb(0x2AABEE) } else { rgba(0x00000000) })
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .text_size(px(10.0))
                                                    .text_color(rgb(0xFFFFFF))
                                                    .child(if is_choice { "✓" } else { "" }),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(13.0))
                                                    .text_color(rgb(0xFFFFFF))
                                                    .child(opt.text),
                                            ),
                                    )
                                    .when(has_voted, |el| {
                                        el.child(
                                            div()
                                                .text_size(px(12.0))
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(rgb(0x8A9AA8))
                                                .child(format!("{}%", pct)),
                                        )
                                    }),
                            )
                            // Progress bar
                            .when(has_voted, move |el| {
                                let bar_w = ((pct as f32 / 100.0) * 332.0).clamp(4.0, 332.0);
                                let bar_color = if poll_type == PollType::Quiz && opt.is_correct {
                                    rgb(0x4CAF50)
                                } else if is_choice {
                                    rgb(0x2AABEE)
                                } else {
                                    rgb(0x3B4E61)
                                };

                                el.child(
                                    div()
                                        .w_full()
                                        .h(px(4.0))
                                        .rounded_full()
                                        .bg(rgb(0x1B2734))
                                        .child(
                                            div()
                                                .w(px(bar_w))
                                                .h(px(4.0))
                                                .rounded_full()
                                                .bg(bar_color),
                                        ),
                                )
                            })
                    })),
            )
            // Solution Explanation for Quiz
            .when_some(solution, |el, sol| {
                el.child(
                    div()
                        .p(px(8.0))
                        .rounded(px(6.0))
                        .bg(rgb(0x233140))
                        .text_size(px(12.0))
                        .text_color(rgb(0x8A9AA8))
                        .child(format!("💡 Explanation: {}", sol)),
                )
            })
            // Footer
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .pt(px(4.0))
                    .border_t_1()
                    .border_color(rgba(0x00000022))
                    .text_size(px(11.0))
                    .text_color(rgb(0x8A9AA8))
                    .child(format!("{} votes", total_votes)),
            )
    }
}
