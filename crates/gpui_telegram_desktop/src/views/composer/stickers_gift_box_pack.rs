//! StickersGiftBoxPack: Telegram Premium gift boxes & TON gift sticker resolution.
//!
//! Upstream reference:
//! - `Telegram/SourceFiles/chat_helpers/stickers_gift_box_pack.h:L22-L60`
//! - `Telegram/SourceFiles/chat_helpers/stickers_gift_box_pack.cpp:L20-L75`
#![allow(dead_code)]

use gpui::*;

/// Premium subscription months dividers (upstream: `_premium.dividers = { 1, 3, 6, 12, 24 }`).
pub const PREMIUM_DIVIDERS: &[i32] = &[1, 3, 6, 12, 24];

/// TON gift amount dividers (upstream: `_ton.dividers = { 0, 10, 50 }`).
pub const TON_DIVIDERS: &[i32] = &[0, 10, 50];

/// Convert Telegram Stars to gifted subscription duration in months
/// (upstream: `GiftBoxPack::monthsForStars`, `stickers_gift_box_pack.cpp:L34-L42`).
pub fn months_for_stars(stars: i32) -> i32 {
    if stars <= 1000 {
        3
    } else if stars < 2500 {
        6
    } else {
        12
    }
}

/// Binary search divider index matching upstream lower_bound logic
/// (upstream: `stickers_gift_box_pack.cpp:L56-L74`).
pub fn find_divider_index(dividers: &[i32], target: i32, exact: bool) -> usize {
    if dividers.is_empty() {
        return 0;
    }
    match dividers.binary_search(&target) {
        Ok(idx) => idx,
        Err(idx) => {
            if idx == 0 {
                0
            } else if idx >= dividers.len() {
                dividers.len() - 1
            } else if exact {
                idx - 1
            } else {
                let prev_dist = (target - dividers[idx - 1]).abs();
                let next_dist = (dividers[idx] - target).abs();
                if prev_dist < next_dist {
                    idx - 1
                } else {
                    idx
                }
            }
        }
    }
}

/// Gift box document item.
#[derive(Debug, Clone)]
pub struct GiftBoxItem {
    pub divider: i32,
    pub document_id: u64,
    pub title: String,
}

/// Authentic StickersGiftBoxPack state.
pub struct StickersGiftBoxPack {
    pub premium_gifts: Vec<GiftBoxItem>,
    pub ton_gifts: Vec<GiftBoxItem>,
    pub updated_at: i64,
}

impl StickersGiftBoxPack {
    pub fn new() -> Self {
        Self {
            premium_gifts: Vec::new(),
            ton_gifts: Vec::new(),
            updated_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn add_premium_gift(&mut self, item: GiftBoxItem) {
        self.premium_gifts.push(item);
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn add_ton_gift(&mut self, item: GiftBoxItem) {
        self.ton_gifts.push(item);
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn lookup_premium_by_stars(&self, stars: i32) -> Option<&GiftBoxItem> {
        let months = months_for_stars(stars);
        let idx = find_divider_index(PREMIUM_DIVIDERS, months, false);
        self.premium_gifts.get(idx)
    }

    pub fn lookup_ton_by_amount(&self, amount: i32) -> Option<&GiftBoxItem> {
        let idx = find_divider_index(TON_DIVIDERS, amount, true);
        self.ton_gifts.get(idx)
    }
}

impl Default for StickersGiftBoxPack {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for StickersGiftBoxPack {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .gap(px(8.0))
            .p(px(8.0))
            .bg(rgb(0xFFFFFF))
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x333333))
                    .child(format!("Premium gifts: {}", self.premium_gifts.len()))
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x333333))
                    .child(format!("TON gifts: {}", self.ton_gifts.len()))
            )
    }
}
