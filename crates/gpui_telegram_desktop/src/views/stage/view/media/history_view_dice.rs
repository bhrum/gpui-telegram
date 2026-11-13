//! Genuine History View Dice Component (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_dice.h:L20-L69`
//! - `Telegram/SourceFiles/history/view/media/history_view_dice.cpp:L23-L106`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;

#[derive(Debug, Clone)]
pub struct HistoryViewDiceViewMedia {
    pub emoji: String,
    pub value: u8,
    pub outcome_nano_ton: i64,
    pub outcome_stake_nano_ton: i64,
}

impl HistoryViewDiceViewMedia {
    pub fn new(emoji: impl Into<String>, value: u8) -> Self {
        Self {
            emoji: emoji.into(),
            value,
            outcome_nano_ton: 0,
            outcome_stake_nano_ton: 0,
        }
    }

    pub fn with_stake_outcome(mut self, won_nano_ton: i64, stake_nano_ton: i64) -> Self {
        self.outcome_nano_ton = won_nano_ton;
        self.outcome_stake_nano_ton = stake_nano_ton;
        self
    }
}

impl Default for HistoryViewDiceViewMedia {
    fn default() -> Self {
        Self::new("🎲", 6)
    }
}

impl Render for HistoryViewDiceViewMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let has_stake = self.outcome_nano_ton > 0 || self.outcome_stake_nano_ton > 0;
        let is_win = self.outcome_nano_ton > 0;

        div()
            .flex()
            .flex_col()
            .items_center()
            .p(px(8.0))
            .rounded(px(8.0))
            .bg(rgb(0x182533))
            .child(
                div()
                    .text_size(px(32.0))
                    .child(self.emoji.clone()),
            )
            .child(
                div()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xFFFFFF))
                    .child(format!("Result: {}", self.value)),
            )
            .when(has_stake, |el| {
                el.child(
                    div()
                        .text_size(px(12.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(if is_win { rgb(0x4CAF50) } else { rgb(0xE55B5B) })
                        .child(if is_win {
                            format!("💎 Won {:.2} TON", (self.outcome_nano_ton as f64) / 1e9)
                        } else {
                            format!("Lost {:.2} TON", (self.outcome_stake_nano_ton as f64) / 1e9)
                        }),
                )
            })
    }
}
