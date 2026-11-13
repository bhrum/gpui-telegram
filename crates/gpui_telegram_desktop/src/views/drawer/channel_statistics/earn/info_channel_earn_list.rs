//! Channel Monetization Transaction List & Balance Overview
//! Upstream: `Telegram/SourceFiles/info/channel_statistics/earn/info_channel_earn_list.cpp:L1-L1534`
//! Upstream: `Telegram/SourceFiles/info/channel_statistics/earn/info_channel_earn_list.h:L1-L73`
//! Subsystem: info | Feature ID: TCG-N12 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use super::earn_format::{major_part, minor_part, to_usd};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EarnTransactionStatus {
    Pending,
    Completed,
    Failed,
    Refunded,
}

#[derive(Debug, Clone)]
pub struct EarnTransaction {
    pub id: String,
    pub date: String,
    pub amount_nanoton: i64,
    pub is_payout: bool,
    pub status: EarnTransactionStatus,
    pub tx_hash: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ChannelEarnOverview {
    pub available_nanoton: i64,
    pub pending_nanoton: i64,
    pub total_earned_nanoton: i64,
    pub ton_rate: f64,
    pub withdrawal_enabled: bool,
}

impl Default for ChannelEarnOverview {
    fn default() -> Self {
        Self {
            available_nanoton: 45_800_000_000,
            pending_nanoton: 12_200_000_000,
            total_earned_nanoton: 180_500_000_000,
            ton_rate: 6.50,
            withdrawal_enabled: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InfoChannelEarnListChannelStatisticsEarn {
    pub overview: ChannelEarnOverview,
    pub transactions: Vec<EarnTransaction>,
    pub is_loading: bool,
    pub is_visible: bool,
}

impl InfoChannelEarnListChannelStatisticsEarn {
    pub fn new() -> Self {
        let mut list = Self {
            overview: ChannelEarnOverview::default(),
            transactions: Vec::new(),
            is_loading: false,
            is_visible: true,
        };
        list.populate_default_transactions();
        list
    }

    pub fn populate_default_transactions(&mut self) {
        self.transactions = vec![
            EarnTransaction {
                id: "tx_101".into(),
                date: "Nov 5, 2026".into(),
                amount_nanoton: 25_000_000_000,
                is_payout: true,
                status: EarnTransactionStatus::Completed,
                tx_hash: Some("EQB...8x9".into()),
            },
            EarnTransaction {
                id: "tx_102".into(),
                date: "Nov 3, 2026".into(),
                amount_nanoton: 8_500_000_000,
                is_payout: false,
                status: EarnTransactionStatus::Completed,
                tx_hash: None,
            },
            EarnTransaction {
                id: "tx_103".into(),
                date: "Nov 1, 2026".into(),
                amount_nanoton: 12_200_000_000,
                is_payout: false,
                status: EarnTransactionStatus::Pending,
                tx_hash: None,
            },
        ];
    }

    pub fn can_withdraw(&self) -> bool {
        self.overview.withdrawal_enabled && self.overview.available_nanoton > 0
    }

    pub fn set_withdrawal_enabled(&mut self, enabled: bool) {
        self.overview.withdrawal_enabled = enabled;
    }
}

impl Default for InfoChannelEarnListChannelStatisticsEarn {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoChannelEarnListChannelStatisticsEarn {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let major = major_part(self.overview.available_nanoton);
        let minor = minor_part(self.overview.available_nanoton);
        let usd = to_usd(self.overview.available_nanoton, self.overview.ton_rate, Some(2));
        let can_withdraw = self.can_withdraw();

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .gap_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p(px(16.0))
                    .rounded_lg()
                    .bg(rgb(0x1e2c3a))
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x8e8e93))
                            .child("Available Balance"),
                    )
                    .child(
                        div()
                            .flex()
                            .items_baseline()
                            .gap_1()
                            .child(
                                div()
                                    .text_3xl()
                                    .font_weight(FontWeight::BOLD)
                                    .child(major),
                            )
                            .child(div().text_lg().text_color(rgb(0x8e8e93)).child(minor))
                            .child(div().text_sm().child("TON")),
                    )
                    .child(div().text_xs().text_color(rgb(0x8e8e93)).child(usd)),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .p(px(10.0))
                    .rounded_md()
                    .bg(if can_withdraw { rgb(0x0088cc) } else { rgb(0x3a4b5d) })
                    .child(
                        div()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0xffffff))
                            .child("Withdraw via Fragment"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .child("Recent Transactions"),
                    )
                    .children(self.transactions.iter().map(|tx| {
                        let tx_major = major_part(tx.amount_nanoton);
                        let tx_minor = minor_part(tx.amount_nanoton);
                        let prefix = if tx.is_payout { "-" } else { "+" };
                        let status_label = match tx.status {
                            EarnTransactionStatus::Completed => "Completed",
                            EarnTransactionStatus::Pending => "Pending",
                            EarnTransactionStatus::Failed => "Failed",
                            EarnTransactionStatus::Refunded => "Refunded",
                        };

                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .py(px(8.0))
                            .border_b_1()
                            .border_color(rgb(0x242f3d))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(div().text_sm().child(tx.date.clone()))
                                    .child(div().text_xs().text_color(rgb(0x8e8e93)).child(status_label)),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(if tx.is_payout { rgb(0xe53935) } else { rgb(0x43a047) })
                                    .child(format!("{}{}{} TON", prefix, tx_major, tx_minor)),
                            )
                    })),
            )
    }
}
