//! Channel Giveaway Creation Modal Box
//! Upstream: `Telegram/SourceFiles/info/channel_statistics/boosts/create_giveaway_box.cpp:L1-L1577`
//! Upstream: `Telegram/SourceFiles/info/channel_statistics/boosts/create_giveaway_box.h:L1-L29`
//! Subsystem: info | Feature ID: TCG-N12 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_DONE_TOOLTIP_DURATION_MS: u64 = 5000;
pub const K_ADDITIONAL_PRIZE_LENGTH_MAX: usize = 128;
pub const BOOSTS_PER_PREMIUM_PRIZE: usize = 4;

/// Computes the default giveaway end time: 3 days from now, rounded to the next 5-minute interval.
pub fn compute_default_end_timestamp(now_unix_secs: u64) -> u64 {
    let three_days = 3 * 86400;
    let target = now_unix_secs + three_days;
    let remainder = target % 300;
    if remainder == 0 {
        target
    } else {
        target + (300 - remainder)
    }
}

#[derive(Debug, Clone)]
pub struct GiveawayConfig {
    pub prize_count: usize,
    pub premium_months: usize,
    pub additional_prize: Option<String>,
    pub only_new_subscribers: bool,
    pub target_channels: Vec<u64>,
    pub target_countries: Vec<String>,
    pub end_timestamp: u64,
}

impl Default for GiveawayConfig {
    fn default() -> Self {
        Self {
            prize_count: 5,
            premium_months: 6,
            additional_prize: None,
            only_new_subscribers: false,
            target_channels: vec![],
            target_countries: vec![],
            end_timestamp: compute_default_end_timestamp(1762500000),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateGiveawayBoxChannelStatisticsBoosts {
    pub config: GiveawayConfig,
    pub is_submitting: bool,
    pub validation_error: Option<String>,
    pub is_visible: bool,
}

impl CreateGiveawayBoxChannelStatisticsBoosts {
    pub fn new() -> Self {
        Self {
            config: GiveawayConfig::default(),
            is_submitting: false,
            validation_error: None,
            is_visible: true,
        }
    }

    pub fn set_prize_count(&mut self, count: usize) {
        self.config.prize_count = count;
    }

    pub fn set_premium_months(&mut self, months: usize) {
        self.config.premium_months = months;
    }

    pub fn set_additional_prize(&mut self, prize: String) -> Result<(), &'static str> {
        if prize.chars().count() > K_ADDITIONAL_PRIZE_LENGTH_MAX {
            self.validation_error = Some("Additional prize text is too long".into());
            Err("Prize text exceeds 128 characters")
        } else {
            self.validation_error = None;
            self.config.additional_prize = Some(prize);
            Ok(())
        }
    }

    pub fn total_boosts_unlocked(&self) -> usize {
        self.config.prize_count * BOOSTS_PER_PREMIUM_PRIZE
    }
}

impl Default for CreateGiveawayBoxChannelStatisticsBoosts {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for CreateGiveawayBoxChannelStatisticsBoosts {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let total_boosts = self.total_boosts_unlocked();
        let prize_count = self.config.prize_count;
        let months = self.config.premium_months;

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(20.0))
            .gap_4()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .child("Create Giveaway"),
                    )
                    .child(
                        div()
                            .px(px(8.0))
                            .py(px(3.0))
                            .rounded_full()
                            .bg(rgb(0x9b51e0))
                            .text_xs()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .child(format!("+{} Boosts", total_boosts)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x8e8e93))
                            .child("Quantity of Prizes"),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .children([1, 3, 5, 10, 25].into_iter().map(|count| {
                                let is_active = count == prize_count;
                                div()
                                    .px(px(12.0))
                                    .py(px(6.0))
                                    .rounded_md()
                                    .bg(if is_active { rgb(0x0088cc) } else { rgb(0x242f3d) })
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0xffffff))
                                    .child(count.to_string())
                            })),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x8e8e93))
                            .child("Subscription Duration"),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .children([(3, "3 months"), (6, "6 months"), (12, "1 year")].into_iter().map(|(m, label)| {
                                let is_active = m == months;
                                div()
                                    .px(px(12.0))
                                    .py(px(6.0))
                                    .rounded_md()
                                    .bg(if is_active { rgb(0x0088cc) } else { rgb(0x242f3d) })
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0xffffff))
                                    .child(label)
                            })),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .py(px(10.0))
                    .rounded_md()
                    .bg(rgb(0x0088cc))
                    .text_sm()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xffffff))
                    .child("Start Giveaway"),
            )
    }
}
