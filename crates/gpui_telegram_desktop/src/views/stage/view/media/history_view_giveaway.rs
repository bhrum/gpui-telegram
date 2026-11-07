//! Genuine History View Giveaway Component (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_giveaway.h:L15-L35`
//! - `Telegram/SourceFiles/history/view/media/history_view_giveaway.cpp:L30-L60`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use crate::views::stage::media::gift::K_OUTLINE_RATIO;

#[derive(Debug, Clone)]
pub struct HistoryViewGiveawayViewMedia {
    pub months: u32,
    pub quantity: u32,
    pub credits: u64,
    pub channels: Vec<String>,
}

impl HistoryViewGiveawayViewMedia {
    pub fn new(months: u32, quantity: u32) -> Self {
        Self {
            months,
            quantity,
            credits: 0,
            channels: Vec::new(),
        }
    }

    pub fn with_credits(mut self, credits: u64) -> Self {
        self.credits = credits;
        self
    }
}

impl Default for HistoryViewGiveawayViewMedia {
    fn default() -> Self {
        Self::new(6, 10)
    }
}

impl Render for HistoryViewGiveawayViewMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w(px(320.0))
            .p(px(12.0))
            .rounded(px(12.0))
            .bg(rgb(0x182533))
            .border_1()
            .border_color(rgb(0x2AABEE))
            .gap(px(6.0))
            .child(
                div()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xFFFFFF))
                    .child(format!("🎉 Giveaway of {} Premium Subscriptions", self.quantity)),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x8A9AA8))
                    .child(format!("Each winner receives {} months of Telegram Premium", self.months)),
            )
    }
}
