//! Rich TON & Gram Crypto Transfer Cards (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_gram_transfer.cpp:L53-L125`

use gpui::*;
use gpui::prelude::FluentBuilder;

pub const K_ADDRESS_GROUP_SIZE: usize = 4;
pub const K_ADDRESS_GROUPS_PER_LINE: usize = 6;
pub const K_GLARE_DURATION_MS: u64 = 1100;
pub const K_GLARE_TIMEOUT_MS: u64 = 400;
pub const K_REVEAL_CLOCK_DURATION_MS: u64 = 43;
pub const K_REVEAL_FILL_DURATION_MS: u64 = 160;
pub const K_REVEAL_COLOR_DELAY_MS: u64 = 20;
pub const K_REVEAL_COLOR_DURATION_MS: u64 = 200;
pub const K_REVEAL_WORD_DELAY_MS: u64 = 33;
pub const K_REVEAL_WORD_DURATION_MS: u64 = 140;
pub const K_BUMP_AMPLITUDE: f64 = 0.07;
pub const K_READ_ROLL_FIRST_MS: u64 = 876;
pub const K_READ_ROLL_DURATION_MS: u64 = 1000;
pub const K_READ_ROLL_EASE: f64 = 1.5;
pub const K_BURST_STARS_PER_SIDE: usize = 48;

/// Upstream BumpCurve physics struct from `history_view_gram_transfer.cpp:L53-L64`
#[derive(Debug, Clone, Copy)]
pub struct BumpCurve {
    pub rise: u64,
    pub hold: u64,
    pub fall: u64,
    pub settle: u64,
    pub fall_ease: f64,
    pub undershoot: f64,
}

impl BumpCurve {
    pub const fn bump_duration(&self) -> u64 {
        self.rise + self.hold + self.fall + self.settle
    }
}

pub const K_SETTLE_BUMP: BumpCurve = BumpCurve {
    rise: 150,
    hold: 33,
    fall: 200,
    settle: 200,
    fall_ease: 1.2,
    undershoot: 0.043,
};

pub const K_READ_PRESS: BumpCurve = BumpCurve {
    rise: 33,
    hold: 0,
    fall: 167,
    settle: 300,
    fall_ease: 1.2,
    undershoot: 0.64,
};

/// Format TON raw address into 6 groups of 4 characters (`history_view_gram_transfer.cpp:L66-L67`)
pub fn format_ton_address_groups(address: &str) -> Vec<String> {
    let clean = address.trim();
    let mut groups = Vec::new();
    let chars: Vec<char> = clean.chars().collect();
    for chunk in chars.chunks(K_ADDRESS_GROUP_SIZE) {
        groups.push(chunk.iter().collect());
        if groups.len() >= K_ADDRESS_GROUPS_PER_LINE {
            break;
        }
    }
    groups
}

/// Convert NanoTON (1e9) to TON decimal string
pub fn format_nano_ton(nano: i64) -> String {
    let ton = (nano as f64) / 1_000_000_000.0;
    format!("{:.2} TON", ton)
}

#[derive(Debug, Clone)]
pub struct RichTonTransferCard {
    pub amount_nano_ton: i64,
    pub address: String,
    pub comment: Option<String>,
    pub is_outgoing: bool,
    pub tx_hash: Option<String>,
}

impl RichTonTransferCard {
    pub fn new(amount_nano_ton: i64, address: impl Into<String>, is_outgoing: bool) -> Self {
        Self {
            amount_nano_ton,
            address: address.into(),
            comment: None,
            is_outgoing,
            tx_hash: None,
        }
    }

    pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }

    pub fn render(self) -> impl IntoElement {
        let formatted_amount = format_nano_ton(self.amount_nano_ton);
        let groups = format_ton_address_groups(&self.address);
        let direction_label = if self.is_outgoing { "Sent TON" } else { "Received TON" };
        let comment = self.comment;

        div()
            .flex()
            .flex_col()
            .w(px(320.0))
            .p(px(14.0))
            .rounded(px(14.0))
            .bg(rgb(0x182533))
            .border_1()
            .border_color(rgb(0x0098EA))
            .gap(px(8.0))
            // Header with TON Crystal Icon
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .child(div().text_size(px(16.0)).child("💎"))
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0x0098EA))
                                    .child(direction_label),
                            ),
                    )
                    .child(
                        div()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child(formatted_amount),
                    ),
            )
            // Address chunked in groups of 4
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(4.0))
                    .p(px(6.0))
                    .rounded(px(6.0))
                    .bg(rgb(0x101921))
                    .children(groups.into_iter().map(|grp| {
                        div()
                            .font_family(".AppleSystemUIFontMonospaced")
                            .text_size(px(11.0))
                            .text_color(rgb(0x8A9AA8))
                            .child(grp)
                    })),
            )
            // Optional transfer comment / payload
            .when_some(comment, |el, c| {
                el.child(
                    div()
                        .text_size(px(12.0))
                        .text_color(rgb(0xCCCCCC))
                        .child(format!("💬 \"{}\"", c)),
                )
            })
            // Wallet Explorer link button
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .py(px(6.0))
                    .rounded(px(6.0))
                    .bg(rgb(0x233140))
                    .text_size(px(12.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(0x0098EA))
                    .child("View in TON Explorer"),
            )
    }
}
