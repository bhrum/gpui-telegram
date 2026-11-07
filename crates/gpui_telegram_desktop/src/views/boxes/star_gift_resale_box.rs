//! Authentic GPUI implementation of Telegram Desktop's Star Gift Resale Box.
//! Upstream reference: `Telegram/SourceFiles/boxes/star_gift_resale_box.h:L32-L55`
//! Upstream implementation: `Telegram/SourceFiles/boxes/star_gift_resale_box.cpp:L42-L79`, `L220-L310`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::sync::Arc;

pub const K_FILTERS_COUNT: usize = 4;
pub const K_RESALE_BOUGHT_TOAST_DURATION_MS: u64 = 4000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResaleGiftsSort {
    Date,
    Price,
    Number,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResaleGiftEntry {
    pub gift_id: u64,
    pub title: String,
    pub number: u32,
    pub seller_name: String,
    pub price_stars: u64,
    pub model_token: String,
}

#[derive(Clone)]
pub struct StarGiftResaleBox {
    pub base_gift_id: u64,
    pub title: String,
    pub sort_mode: ResaleGiftsSort,
    pub entries: Vec<ResaleGiftEntry>,
    pub selected_entry: Option<u64>,
    pub is_closed: bool,
    pub on_buy: Option<Arc<dyn Fn(u64, u64) + Send + Sync>>,
    pub on_close: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl StarGiftResaleBox {
    pub fn new(base_gift_id: u64, title: impl Into<String>) -> Self {
        Self {
            base_gift_id,
            title: title.into(),
            sort_mode: ResaleGiftsSort::Price,
            entries: vec![
                ResaleGiftEntry { gift_id: 1001, title: "Cyber Skull".into(), number: 42, seller_name: "cryptoking".into(), price_stars: 1200, model_token: "💀".into() },
                ResaleGiftEntry { gift_id: 1002, title: "Cyber Skull".into(), number: 88, seller_name: "star_collector".into(), price_stars: 1450, model_token: "💀".into() },
                ResaleGiftEntry { gift_id: 1003, title: "Cyber Skull".into(), number: 7, seller_name: "ton_whale".into(), price_stars: 2800, model_token: "💀".into() },
            ],
            selected_entry: Some(1001),
            is_closed: false,
            on_buy: None,
            on_close: None,
        }
    }

    pub fn calculate_commission(price: u64) -> (u64, u64) {
        let seller_receives = price * 80 / 100;
        let platform_fee = price - seller_receives;
        (seller_receives, platform_fee)
    }

    pub fn set_sort(&mut self, sort: ResaleGiftsSort) {
        self.sort_mode = sort;
        match sort {
            ResaleGiftsSort::Price => self.entries.sort_by_key(|e| e.price_stars),
            ResaleGiftsSort::Number => self.entries.sort_by_key(|e| e.number),
            ResaleGiftsSort::Date => self.entries.sort_by(|a, b| b.gift_id.cmp(&a.gift_id)),
        }
    }

    pub fn buy(&mut self) {
        if let Some(id) = self.selected_entry {
            if let Some(entry) = self.entries.iter().find(|e| e.gift_id == id) {
                if let Some(ref cb) = self.on_buy {
                    cb(entry.gift_id, entry.price_stars);
                }
            }
            self.close();
        }
    }

    pub fn close(&mut self) {
        self.is_closed = true;
        if let Some(ref cb) = self.on_close {
            cb();
        }
    }
}

impl Default for StarGiftResaleBox {
    fn default() -> Self {
        Self::new(505, "Cyber Skull Secondary Market")
    }
}

impl Render for StarGiftResaleBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        let selected = self.entries.iter().find(|e| Some(e.gift_id) == self.selected_entry).cloned();
        let cost = selected.as_ref().map(|e| e.price_stars).unwrap_or(0);
        let (seller_gets, fee) = Self::calculate_commission(cost);

        div()
            .id("star_gift_resale_box")
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
                            .child(self.title.clone()),
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
                    .gap(px(8.0))
                    .child(
                        div()
                            .px(px(8.0))
                            .py(px(3.0))
                            .rounded(px(6.0))
                            .bg(if self.sort_mode == ResaleGiftsSort::Price { rgb(0xEEF8FF) } else { rgb(0xF1F3F5) })
                            .text_size(px(12.0))
                            .text_color(if self.sort_mode == ResaleGiftsSort::Price { rgb(0x2AABEE) } else { rgb(0x666666) })
                            .child("By Price"),
                    )
                    .child(
                        div()
                            .px(px(8.0))
                            .py(px(3.0))
                            .rounded(px(6.0))
                            .bg(if self.sort_mode == ResaleGiftsSort::Number { rgb(0xEEF8FF) } else { rgb(0xF1F3F5) })
                            .text_size(px(12.0))
                            .text_color(if self.sort_mode == ResaleGiftsSort::Number { rgb(0x2AABEE) } else { rgb(0x666666) })
                            .child("By Number"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .children(self.entries.iter().map(|entry| {
                        let is_sel = Some(entry.gift_id) == self.selected_entry;
                        div()
                            .flex()
                            .justify_between()
                            .items_center()
                            .p(px(10.0))
                            .rounded(px(8.0))
                            .border_1()
                            .border_color(if is_sel { rgb(0x2AABEE) } else { rgb(0xEEEEEE) })
                            .bg(if is_sel { rgb(0xEEF8FF) } else { rgb(0xFFFFFF) })
                            .cursor_pointer()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(8.0))
                                    .child(div().text_size(px(20.0)).child(entry.model_token.clone()))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(div().text_size(px(13.0)).font_weight(FontWeight::MEDIUM).text_color(rgb(0x222222)).child(format!("{} #{}", entry.title, entry.number)))
                                            .child(div().text_size(px(11.0)).text_color(rgb(0x888888)).child(format!("Seller: @{}", entry.seller_name))),
                                    ),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xEAA400))
                                    .child(format!("⭐ {}", entry.price_stars)),
                            )
                    })),
            )
            .child(
                div()
                    .p(px(8.0))
                    .rounded(px(6.0))
                    .bg(rgb(0xF8F9FA))
                    .text_size(px(11.0))
                    .text_color(rgb(0x666666))
                    .child(format!("Includes 20% platform fee (Seller gets ⭐ {}, Fee ⭐ {})", seller_gets, fee)),
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
                            .child(format!("Buy for ⭐ {} Stars", cost)),
                    ),
            )
            .into_any_element()
    }
}
