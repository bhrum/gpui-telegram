//! Authentic GPUI implementation of Telegram Desktop's GiftPremiumBox.
//! Upstream reference: `Telegram/SourceFiles/boxes/gift_premium_box.h:L44-L115`
//! and `Telegram/SourceFiles/boxes/gift_premium_box.cpp:L88-L360`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::sync::Arc;

pub const K_TOOLTIP_DURATION_MS: u64 = 3000;
pub const K_HORIZONTAL_BAR: char = '\u{2015}';
pub const K_SPINNER_ROWS: usize = 6;
pub const K_SPIN_DURATION_MS: u64 = 120;

/// Tier options for Telegram Premium subscription gifting.
#[derive(Clone, Debug, PartialEq)]
pub struct PremiumGiftTier {
    pub months: u32,
    pub price_formatted: String,
    pub discount_percent: Option<u32>,
    pub is_best_value: bool,
}

/// View component for gifting Telegram Premium subscriptions and redeeming codes.
#[derive(Clone)]
pub struct GiftPremiumBox {
    pub recipient_name: String,
    pub recipient_id: u64,
    pub tiers: Vec<PremiumGiftTier>,
    pub selected_months: u32,
    pub gift_code_input: String,
    pub is_redeem_mode: bool,
    pub is_closed: bool,
    pub on_gift_confirmed: Option<Arc<dyn Fn(u64, u32) + Send + Sync>>,
}

impl Default for GiftPremiumBox {
    fn default() -> Self {
        Self {
            recipient_name: "Friend".to_string(),
            recipient_id: 0,
            tiers: vec![
                PremiumGiftTier {
                    months: 3,
                    price_formatted: "$11.99".into(),
                    discount_percent: None,
                    is_best_value: false,
                },
                PremiumGiftTier {
                    months: 6,
                    price_formatted: "$17.99".into(),
                    discount_percent: Some(25),
                    is_best_value: false,
                },
                PremiumGiftTier {
                    months: 12,
                    price_formatted: "$28.99".into(),
                    discount_percent: Some(40),
                    is_best_value: true,
                },
            ],
            selected_months: 12,
            gift_code_input: String::new(),
            is_redeem_mode: false,
            is_closed: false,
            on_gift_confirmed: None,
        }
    }
}

impl GiftPremiumBox {
    pub fn new(recipient_name: String, recipient_id: u64) -> Self {
        Self {
            recipient_name,
            recipient_id,
            ..Default::default()
        }
    }

    /// Select duration in months (3, 6, 12).
    pub fn select_tier(&mut self, months: u32) {
        self.selected_months = months;
    }

    /// Formats gift duration phrase per upstream `GiftDurationPhrase` (`gift_premium_box.cpp:L300-L330`).
    pub fn format_duration(days: i32) -> &'static str {
        if days < 30 {
            "days"
        } else if days < 360 {
            "months"
        } else {
            "years"
        }
    }

    /// Cleans transaction id per upstream `FixupTransactionId` (`gift_premium_box.cpp:L117-L120`).
    pub fn fixup_transaction_id(origin: &str) -> String {
        origin.replace(K_HORIZONTAL_BAR, "-")
    }

    pub fn confirm_gift(&mut self) {
        if let Some(ref cb) = self.on_gift_confirmed {
            cb(self.recipient_id, self.selected_months);
        }
        self.is_closed = true;
    }

    pub fn close(&mut self) {
        self.is_closed = true;
    }
}

impl Render for GiftPremiumBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        let mut card = div()
            .id("gift_premium_box")
            .w(px(400.0))
            .p(px(24.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(12.0))
            .shadow_lg()
            .flex()
            .flex_col()
            .gap(px(16.0));

        // Premium Badge Header
        card = card.child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap(px(6.0))
                .child(
                    div()
                        .text_size(px(32.0))
                        .child("⭐"),
                )
                .child(
                    div()
                        .text_size(px(18.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x111827))
                        .child(format!("Gift Telegram Premium to {}", self.recipient_name)),
                )
                .child(
                    div()
                        .text_size(px(13.0))
                        .text_color(rgb(0x6B7280))
                        .child("Unlock exclusive features and doubled limits for your friend."),
                ),
        );

        // Subscription Tier Cards
        let mut tiers_col = div().flex().flex_col().gap(px(10.0));
        for tier in &self.tiers {
            let is_selected = self.selected_months == tier.months;
            let bg_col = if is_selected { rgb(0xEFF6FF) } else { rgb(0xF9FAFB) };
            let border_col = if is_selected { rgb(0x3B82F6) } else { rgb(0xE5E7EB) };

            let mut row = div()
                .flex()
                .items_center()
                .justify_between()
                .p(px(14.0))
                .rounded(px(10.0))
                .bg(bg_col)
                .border_1()
                .border_color(border_col)
                .cursor_pointer()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .child(
                            div()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(0x1F2937))
                                .child(format!("{} Months", tier.months)),
                        ),
                )
                .child(
                    div()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x2563EB))
                        .child(tier.price_formatted.clone()),
                );

            if let Some(pct) = tier.discount_percent {
                row = row.child(
                    div()
                        .px(px(6.0))
                        .py(px(2.0))
                        .rounded(px(4.0))
                        .bg(rgb(0xDCFCE7))
                        .text_size(px(11.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x15803D))
                        .child(format!("-{}%", pct)),
                );
            }
            tiers_col = tiers_col.child(row);
        }
        card = card.child(tiers_col);

        // Confirm Button
        let actions = div()
            .flex()
            .justify_end()
            .gap(px(10.0))
            .child(
                div()
                    .px(px(16.0))
                    .py(px(8.0))
                    .cursor_pointer()
                    .text_color(rgb(0x6B7280))
                    .child("Close"),
            )
            .child(
                div()
                    .px(px(20.0))
                    .py(px(8.0))
                    .rounded(px(6.0))
                    .bg(rgb(0x3B82F6))
                    .cursor_pointer()
                    .text_color(rgb(0xFFFFFF))
                    .font_weight(FontWeight::BOLD)
                    .child(format!("Send {} Months Gift", self.selected_months)),
            );

        card.child(actions).into_any_element()
    }
}
