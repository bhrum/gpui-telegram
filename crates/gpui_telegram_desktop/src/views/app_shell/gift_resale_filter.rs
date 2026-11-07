//! GiftResaleFilter: Context filter action for collectible gifts, rarity, and custom emoji status
//! Upstream: Telegram/SourceFiles/menu/gift_resale_filter.h, gift_resale_filter.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct GiftResaleFilter {
    pub is_checked: bool,
    pub color_emoji: Option<String>,
    pub min_price_stars: u64,
    pub max_price_stars: u64,
}

impl GiftResaleFilter {
    pub fn new() -> Self {
        Self {
            is_checked: false,
            color_emoji: None,
            min_price_stars: 0,
            max_price_stars: 100_000,
        }
    }

    pub fn set_checked(&mut self, checked: bool) {
        self.is_checked = checked;
    }

    pub fn set_color_emoji(&mut self, emoji: impl Into<String>) {
        self.color_emoji = Some(emoji.into());
    }

    pub fn set_price_range(&mut self, min_stars: u64, max_stars: u64) {
        self.min_price_stars = min_stars;
        self.max_price_stars = max_stars;
    }

    pub fn is_ready(&self) -> bool {
        true
    }
}

impl Default for GiftResaleFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for GiftResaleFilter {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .p(px(8.0))
            .child(format!("Gift Resale Filter [checked={}]", self.is_checked))
    }
}
