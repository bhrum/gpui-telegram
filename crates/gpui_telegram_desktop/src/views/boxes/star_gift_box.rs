//! Authentic GPUI implementation of Telegram Desktop's Star Gift Box.
//! Upstream reference: `Telegram/SourceFiles/boxes/star_gift_box.h:L72-L234`
//! Upstream implementation: `Telegram/SourceFiles/boxes/star_gift_box.cpp:L140-L148`, `L1200-L1350`, `L2400-L2600`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::sync::Arc;

pub const K_PRICE_TAB_ALL: i32 = 0;
pub const K_PRICE_TAB_MY: i32 = -1;
pub const K_PRICE_TAB_COLLECTIBLES: i32 = -2;
pub const K_SENT_TOAST_DURATION_MS: u64 = 3000;
pub const K_SWITCH_UPGRADE_COVER_INTERVAL_MS: u64 = 3000;
pub const K_UPGRADE_DONE_TOAST_DURATION_MS: u64 = 4000;
pub const K_MESSAGE_RESTRICTED_TOAST_DURATION_MS: u64 = 4000;
pub const K_RESELL_PRICE_CACHE_LIFETIME_MS: u64 = 60000;

/// Standard Star gift catalog item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StarGiftItem {
    pub id: u64,
    pub title: String,
    pub stars_price: u64,
    pub is_limited: bool,
    pub remaining_count: u32,
    pub total_count: u32,
    pub icon_emoji: String,
    pub can_upgrade: bool,
}

/// Star Gift Box view component.
#[derive(Clone)]
pub struct StarGiftBox {
    pub recipient_id: u64,
    pub recipient_name: String,
    pub active_tab: i32,
    pub gifts: Vec<StarGiftItem>,
    pub selected_gift_id: Option<u64>,
    pub is_anonymous: bool,
    pub comment: String,
    pub is_closed: bool,
    pub on_send: Option<Arc<dyn Fn(u64, bool, String) + Send + Sync>>,
    pub on_close: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl StarGiftBox {
    pub fn new(recipient_id: u64, recipient_name: impl Into<String>) -> Self {
        Self {
            recipient_id,
            recipient_name: recipient_name.into(),
            active_tab: K_PRICE_TAB_ALL,
            gifts: Self::default_catalog(),
            selected_gift_id: Some(1),
            is_anonymous: false,
            comment: String::new(),
            is_closed: false,
            on_send: None,
            on_close: None,
        }
    }

    pub fn default_catalog() -> Vec<StarGiftItem> {
        vec![
            StarGiftItem { id: 1, title: "Birthday Star".into(), stars_price: 15, is_limited: false, remaining_count: 0, total_count: 0, icon_emoji: "⭐".into(), can_upgrade: false },
            StarGiftItem { id: 2, title: "Sweet Heart".into(), stars_price: 25, is_limited: false, remaining_count: 0, total_count: 0, icon_emoji: "❤️".into(), can_upgrade: false },
            StarGiftItem { id: 3, title: "Golden Rocket".into(), stars_price: 50, is_limited: false, remaining_count: 0, total_count: 0, icon_emoji: "🚀".into(), can_upgrade: true },
            StarGiftItem { id: 4, title: "Champagne".into(), stars_price: 100, is_limited: false, remaining_count: 0, total_count: 0, icon_emoji: "🍾".into(), can_upgrade: true },
            StarGiftItem { id: 5, title: "Diamond Ring".into(), stars_price: 500, is_limited: true, remaining_count: 142, total_count: 1000, icon_emoji: "💍".into(), can_upgrade: true },
            StarGiftItem { id: 6, title: "Cyber Skull #42".into(), stars_price: 2500, is_limited: true, remaining_count: 12, total_count: 500, icon_emoji: "💀".into(), can_upgrade: true },
        ]
    }

    pub fn stars_from_ton(ton_nanos: u64) -> u64 {
        ton_nanos / 20_000_000
    }

    pub fn ton_from_stars(stars: u64) -> u64 {
        stars * 20_000_000
    }

    pub fn resale_proceeds_and_fee(price: u64) -> (u64, u64) {
        let proceeds = price * 80 / 100;
        let fee = price - proceeds;
        (proceeds, fee)
    }

    pub fn select_gift(&mut self, id: u64) {
        self.selected_gift_id = Some(id);
    }

    pub fn toggle_anonymous(&mut self) {
        self.is_anonymous = !self.is_anonymous;
    }

    pub fn close(&mut self) {
        self.is_closed = true;
        if let Some(ref cb) = self.on_close {
            cb();
        }
    }

    pub fn submit(&mut self) {
        if let Some(id) = self.selected_gift_id {
            if let Some(ref cb) = self.on_send {
                cb(id, self.is_anonymous, self.comment.clone());
            }
            self.close();
        }
    }
}

impl Default for StarGiftBox {
    fn default() -> Self {
        Self::new(0, "Saved Messages")
    }
}

impl Render for StarGiftBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        let selected = self.gifts.iter().find(|g| Some(g.id) == self.selected_gift_id).cloned();
        let selected_cost = selected.as_ref().map(|g| g.stars_price).unwrap_or(0);

        div()
            .id("star_gift_box_modal")
            .w(px(420.0))
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
                            .text_size(px(17.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x222222))
                            .child(format!("Send a Gift to {}", self.recipient_name)),
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
                    .p(px(2.0))
                    .bg(rgb(0xF1F3F5))
                    .rounded(px(8.0))
                    .child(
                        div()
                            .px(px(12.0))
                            .py(px(4.0))
                            .rounded(px(6.0))
                            .bg(if self.active_tab == K_PRICE_TAB_ALL { rgb(0xFFFFFF) } else { rgb(0xF1F3F5) })
                            .text_size(px(13.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(0x222222))
                            .child("All Gifts"),
                    )
                    .child(
                        div()
                            .px(px(12.0))
                            .py(px(4.0))
                            .rounded(px(6.0))
                            .bg(if self.active_tab == K_PRICE_TAB_MY { rgb(0xFFFFFF) } else { rgb(0xF1F3F5) })
                            .text_size(px(13.0))
                            .text_color(rgb(0x666666))
                            .child("My Gifts"),
                    )
                    .child(
                        div()
                            .px(px(12.0))
                            .py(px(4.0))
                            .rounded(px(6.0))
                            .bg(if self.active_tab == K_PRICE_TAB_COLLECTIBLES { rgb(0xFFFFFF) } else { rgb(0xF1F3F5) })
                            .text_size(px(13.0))
                            .text_color(rgb(0x666666))
                            .child("Collectibles"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap(px(10.0))
                    .children(self.gifts.iter().map(|item| {
                        let is_sel = Some(item.id) == self.selected_gift_id;
                        div()
                            .w(px(116.0))
                            .p(px(8.0))
                            .rounded(px(8.0))
                            .border_1()
                            .border_color(if is_sel { rgb(0x2AABEE) } else { rgb(0xEEEEEE) })
                            .bg(if is_sel { rgb(0xEEF8FF) } else { rgb(0xFAFAFA) })
                            .cursor_pointer()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap(px(4.0))
                            .child(div().text_size(px(28.0)).child(item.icon_emoji.clone()))
                            .child(div().text_size(px(12.0)).font_weight(FontWeight::MEDIUM).text_color(rgb(0x222222)).child(item.title.clone()))
                            .child(div().text_size(px(11.0)).font_weight(FontWeight::BOLD).text_color(rgb(0xEAA400)).child(format!("⭐ {}", item.stars_price)))
                    })),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .cursor_pointer()
                    .child(
                        div()
                            .size(px(16.0))
                            .rounded(px(4.0))
                            .border_1()
                            .border_color(rgb(0x2AABEE))
                            .bg(if self.is_anonymous { rgb(0x2AABEE) } else { rgb(0xFFFFFF) })
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(if self.is_anonymous { div().text_size(px(11.0)).text_color(rgb(0xFFFFFF)).child("✓") } else { div() }),
                    )
                    .child(div().text_size(px(13.0)).text_color(rgb(0x444444)).child("Send anonymously")),
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
                            .child(format!("Send for ⭐ {} Stars", selected_cost)),
                    ),
            )
            .into_any_element()
    }
}
