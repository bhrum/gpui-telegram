//! Authentic GPUI implementation of Telegram Desktop's Star Gift Auction Box.
//! Upstream reference: `Telegram/SourceFiles/boxes/star_gift_auction_box.h:L34-L83`
//! Upstream implementation: `Telegram/SourceFiles/boxes/star_gift_auction_box.cpp:L80-L98`, `L450-L520`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::sync::Arc;

pub const K_AUCTION_ABOUT_SHOWN_PREF: &str = "gift_auction_about_shown";
pub const K_BID_PLACED_TOAST_DURATION_MS: u64 = 5000;
pub const K_SWITCH_PREVIEW_COVER_INTERVAL_MS: u64 = 3000;
pub const K_MAX_SHOWN_BID: u64 = 30000;
pub const K_SHOW_TOP_PLACES: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BidType {
    Setting,
    Winning,
    Loosing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BidRow {
    pub position: usize,
    pub user_name: String,
    pub stars: u64,
    pub is_me: bool,
    pub bid_type: BidType,
}

#[derive(Clone)]
pub struct StarGiftAuctionBox {
    pub gift_id: u64,
    pub gift_title: String,
    pub round: u32,
    pub total_rounds: u32,
    pub seconds_remaining: u32,
    pub current_highest_bid: u64,
    pub min_bid_step: u64,
    pub my_bid: Option<u64>,
    pub my_bid_type: BidType,
    pub leaderboard: Vec<BidRow>,
    pub is_closed: bool,
    pub on_place_bid: Option<Arc<dyn Fn(u64) + Send + Sync>>,
    pub on_close: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl StarGiftAuctionBox {
    pub fn new(gift_id: u64, gift_title: impl Into<String>, highest_bid: u64) -> Self {
        Self {
            gift_id,
            gift_title: gift_title.into(),
            round: 1,
            total_rounds: 5,
            seconds_remaining: 354,
            current_highest_bid: highest_bid,
            min_bid_step: 100,
            my_bid: None,
            my_bid_type: BidType::Setting,
            leaderboard: vec![
                BidRow { position: 1, user_name: "Alex".into(), stars: highest_bid, is_me: false, bid_type: BidType::Winning },
                BidRow { position: 2, user_name: "Elena".into(), stars: highest_bid.saturating_sub(150), is_me: false, bid_type: BidType::Loosing },
                BidRow { position: 3, user_name: "Dmitry".into(), stars: highest_bid.saturating_sub(300), is_me: false, bid_type: BidType::Loosing },
            ],
            is_closed: false,
            on_place_bid: None,
            on_close: None,
        }
    }

    pub fn place_bid(&mut self, amount: u64) {
        if amount > self.current_highest_bid {
            self.current_highest_bid = amount;
            self.my_bid = Some(amount);
            self.my_bid_type = BidType::Winning;
            self.leaderboard.insert(0, BidRow {
                position: 1,
                user_name: "You".into(),
                stars: amount,
                is_me: true,
                bid_type: BidType::Winning,
            });
            for (i, row) in self.leaderboard.iter_mut().enumerate() {
                row.position = i + 1;
                if !row.is_me {
                    row.bid_type = BidType::Loosing;
                }
            }
            if let Some(ref cb) = self.on_place_bid {
                cb(amount);
            }
        }
    }

    pub fn close(&mut self) {
        self.is_closed = true;
        if let Some(ref cb) = self.on_close {
            cb();
        }
    }
}

impl Default for StarGiftAuctionBox {
    fn default() -> Self {
        Self::new(202, "Star Forge #007", 2500)
    }
}

impl Render for StarGiftAuctionBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        let next_min = self.current_highest_bid + self.min_bid_step;

        div()
            .id("star_gift_auction_box")
            .w(px(380.0))
            .p(px(20.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(12.0))
            .shadow_lg()
            .flex()
            .flex_col()
            .gap(px(14.0))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x222222))
                            .child(format!("Auction: {}", self.gift_title)),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .text_size(px(14.0))
                            .text_color(rgb(0x888888))
                            .child("✕"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .p(px(12.0))
                    .bg(rgb(0xFAF4E6))
                    .rounded(px(8.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(div().text_size(px(11.0)).text_color(rgb(0x8A6D3B)).child(format!("ROUND {} OF {}", self.round, self.total_rounds)))
                            .child(div().text_size(px(18.0)).font_weight(FontWeight::BOLD).text_color(rgb(0xB8860B)).child(format!("⭐ {}", self.current_highest_bid))),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_end()
                            .child(div().text_size(px(11.0)).text_color(rgb(0x8A6D3B)).child("TIME LEFT"))
                            .child(div().text_size(px(15.0)).font_weight(FontWeight::SEMIBOLD).text_color(rgb(0x222222)).child(format!("{}m {}s", self.seconds_remaining / 60, self.seconds_remaining % 60))),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .children(self.leaderboard.iter().take(K_SHOW_TOP_PLACES).map(|b| {
                        let medal = match b.position {
                            1 => "🥇",
                            2 => "🥈",
                            3 => "🥉",
                            _ => "•",
                        };
                        div()
                            .flex()
                            .justify_between()
                            .items_center()
                            .py(px(4.0))
                            .px(px(8.0))
                            .rounded(px(6.0))
                            .bg(if b.is_me { rgb(0xEBF5FF) } else { rgb(0xFFFFFF) })
                            .child(div().text_size(px(13.0)).text_color(rgb(0x333333)).child(format!("{} {}", medal, b.user_name)))
                            .child(div().text_size(px(13.0)).font_weight(FontWeight::SEMIBOLD).text_color(rgb(0x222222)).child(format!("⭐ {}", b.stars)))
                    })),
            )
            .child(
                div()
                    .w_full()
                    .py(px(10.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x2AABEE))
                    .cursor_pointer()
                    .flex()
                    .justify_center()
                    .items_center()
                    .child(
                        div()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child(format!("Place Bid ⭐ {}", next_min)),
                    ),
            )
            .into_any_element()
    }
}
