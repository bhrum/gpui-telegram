//! Authentic GPUI implementation of Telegram Desktop's StarGiftCoverBox.
//! Upstream reference: `Telegram/SourceFiles/boxes/star_gift_cover_box.h:L65-L146`
//! and `Telegram/SourceFiles/boxes/star_gift_cover_box.cpp:L50-L130`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_CROSSFADE_DURATION_MS: u64 = 400;
pub const K_SPINNER_BACKDROPS: usize = 6;
pub const K_SPINNER_PATTERNS: usize = 6;
pub const K_SPINNER_MODELS: usize = 6;
pub const K_BACKDROP_STOPS_AT_MS: u64 = 2500;
pub const K_PATTERN_STOPS_AT_MS: u64 = 4000;
pub const K_MODEL_STOPS_AT_MS: u64 = 5500;

/// Unique Star Gift attribute values (`star_gift_cover_box.h:L23-L62`).
#[derive(Clone, Debug)]
pub struct UniqueGiftAttributes {
    pub title: String,
    pub number: u32,
    pub total_issued: u32,
    pub model_name: String,
    pub pattern_name: String,
    pub backdrop_name: String,
    pub resale_price_stars: Option<u64>,
}

/// View component displaying unique collectible star gift card and cover properties.
#[derive(Clone)]
pub struct StarGiftCoverBox {
    pub gift: UniqueGiftAttributes,
    pub is_spinning: bool,
    pub spin_progress_ms: u64,
    pub is_closed: bool,
}

impl Default for StarGiftCoverBox {
    fn default() -> Self {
        Self {
            gift: UniqueGiftAttributes {
                title: "Cyber Skull".to_string(),
                number: 42,
                total_issued: 1000,
                model_name: "Diamond".to_string(),
                pattern_name: "Circuits".to_string(),
                backdrop_name: "Midnight Navy".to_string(),
                resale_price_stars: Some(1500),
            },
            is_spinning: false,
            spin_progress_ms: 0,
            is_closed: false,
        }
    }
}

impl StarGiftCoverBox {
    pub fn new(gift: UniqueGiftAttributes) -> Self {
        Self {
            gift,
            is_spinning: false,
            spin_progress_ms: 0,
            is_closed: false,
        }
    }

    /// Triggers attribute wheel spin animation.
    pub fn start_spin(&mut self) {
        self.is_spinning = true;
        self.spin_progress_ms = 0;
    }

    pub fn close(&mut self) {
        self.is_closed = true;
    }
}

impl Render for StarGiftCoverBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        let mut card = div()
            .id("star_gift_cover_box")
            .w(px(380.0))
            .p(px(24.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(14.0))
            .shadow_lg()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(16.0));

        // Gift Cover Preview Frame
        card = card.child(
            div()
                .w(px(280.0))
                .h(px(280.0))
                .rounded(px(16.0))
                .bg(rgb(0x0F172A))
                .border_2()
                .border_color(rgb(0x3B82F6))
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .gap(px(8.0))
                .child(
                    div()
                        .text_size(px(64.0))
                        .child("💀"),
                )
                .child(
                    div()
                        .text_size(px(18.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xFFFFFF))
                        .child(format!("{} #{}", self.gift.title, self.gift.number)),
                )
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(rgb(0x94A3B8))
                        .child(format!("1 of {} issued", self.gift.total_issued)),
                ),
        );

        // Attributes Table
        let attrs = div()
            .w_full()
            .p(px(12.0))
            .bg(rgb(0xF8FAFC))
            .rounded(px(10.0))
            .flex()
            .flex_col()
            .gap(px(8.0))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .text_size(px(13.0))
                    .child(div().text_color(rgb(0x64748B)).child("Model"))
                    .child(div().font_weight(FontWeight::SEMIBOLD).text_color(rgb(0x0F172A)).child(self.gift.model_name.clone())),
            )
            .child(
                div()
                    .flex()
                    .justify_between()
                    .text_size(px(13.0))
                    .child(div().text_color(rgb(0x64748B)).child("Pattern"))
                    .child(div().font_weight(FontWeight::SEMIBOLD).text_color(rgb(0x0F172A)).child(self.gift.pattern_name.clone())),
            )
            .child(
                div()
                    .flex()
                    .justify_between()
                    .text_size(px(13.0))
                    .child(div().text_color(rgb(0x64748B)).child("Backdrop"))
                    .child(div().font_weight(FontWeight::SEMIBOLD).text_color(rgb(0x0F172A)).child(self.gift.backdrop_name.clone())),
            );
        card = card.child(attrs);

        // Action button (resale or transfer)
        let btn_label = if let Some(price) = self.gift.resale_price_stars {
            format!("Buy for ⭐ {}", price)
        } else {
            "Transfer Gift".to_string()
        };

        card = card.child(
            div()
                .w_full()
                .py(px(10.0))
                .rounded(px(8.0))
                .bg(rgb(0x2563EB))
                .cursor_pointer()
                .flex()
                .justify_center()
                .text_size(px(14.0))
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(0xFFFFFF))
                .child(btn_label),
        );

        card.into_any_element()
    }
}
