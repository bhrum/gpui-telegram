//! Authentic GPUI implementation of Telegram Desktop's SendCreditsBox.
//! Upstream reference: `Telegram/SourceFiles/boxes/send_credits_box.h:L35-L65`
//! and `Telegram/SourceFiles/boxes/send_credits_box.cpp:L50-L130`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::sync::Arc;

/// Paid media information extracted from invoice (`send_credits_box.cpp:L51-L97`).
#[derive(Clone, Debug)]
pub struct PaidMediaInfo {
    pub photo_count: u32,
    pub video_count: u32,
    pub sender_name: String,
    pub required_stars: u64,
}

/// View component for confirming Telegram Stars payments for paid media, bot invoices, or channel subscriptions.
#[derive(Clone)]
pub struct SendCreditsBox {
    pub invoice_title: String,
    pub stars_amount: u64,
    pub user_balance_stars: u64,
    pub paid_media: Option<PaidMediaInfo>,
    pub terms_accepted: bool,
    pub is_closed: bool,
    pub on_confirmed: Option<Arc<dyn Fn(u64) + Send + Sync>>,
}

impl Default for SendCreditsBox {
    fn default() -> Self {
        Self {
            invoice_title: "Purchase with Telegram Stars".to_string(),
            stars_amount: 50,
            user_balance_stars: 200,
            paid_media: None,
            terms_accepted: true,
            is_closed: false,
            on_confirmed: None,
        }
    }
}

impl SendCreditsBox {
    pub fn new(
        invoice_title: String,
        stars_amount: u64,
        user_balance_stars: u64,
        paid_media: Option<PaidMediaInfo>,
    ) -> Self {
        Self {
            invoice_title,
            stars_amount,
            user_balance_stars,
            paid_media,
            terms_accepted: true,
            is_closed: false,
            on_confirmed: None,
        }
    }

    /// Check whether user has enough Telegram Stars balance.
    pub fn has_sufficient_balance(&self) -> bool {
        self.user_balance_stars >= self.stars_amount
    }

    /// Calculate required top-up amount if balance is insufficient.
    pub fn missing_stars(&self) -> u64 {
        self.stars_amount.saturating_sub(self.user_balance_stars)
    }

    pub fn confirm_payment(&mut self) {
        if self.has_sufficient_balance() {
            if let Some(ref cb) = self.on_confirmed {
                cb(self.stars_amount);
            }
            self.is_closed = true;
        }
    }

    pub fn close(&mut self) {
        self.is_closed = true;
    }
}

impl Render for SendCreditsBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        let mut card = div()
            .id("send_credits_box")
            .w(px(380.0))
            .p(px(24.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(12.0))
            .shadow_lg()
            .flex()
            .flex_col()
            .gap(px(16.0));

        // Invoice Header
        card = card.child(
            div()
                .flex()
                .items_center()
                .gap(px(10.0))
                .child(
                    div()
                        .text_size(px(24.0))
                        .child("⭐"),
                )
                .child(
                    div()
                        .text_size(px(18.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x1F2937))
                        .child(self.invoice_title.clone()),
                ),
        );

        // Paid media details if present (`send_credits_box.cpp:L63-L97`)
        if let Some(ref media) = self.paid_media {
            card = card.child(
                div()
                    .p(px(12.0))
                    .bg(rgb(0xF9FAFB))
                    .rounded(px(8.0))
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(rgb(0x4B5563))
                            .child(format!("Media from {}", media.sender_name)),
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x111827))
                            .child(format!("{} Photos, {} Videos", media.photo_count, media.video_count)),
                    ),
            );
        }

        // Amount & Balance Info
        card = card.child(
            div()
                .flex()
                .justify_between()
                .items_center()
                .p(px(14.0))
                .bg(rgb(0xFEF3C7))
                .rounded(px(8.0))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .text_size(px(12.0))
                                .text_color(rgb(0x92400E))
                                .child("Price"),
                        )
                        .child(
                            div()
                                .text_size(px(18.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xB45309))
                                .child(format!("⭐ {}", self.stars_amount)),
                        ),
                )
                .child(
                    div()
                        .text_size(px(13.0))
                        .text_color(rgb(0x78350F))
                        .child(format!("Your Balance: ⭐ {}", self.user_balance_stars)),
                ),
        );

        // Action Buttons
        let sufficient = self.has_sufficient_balance();
        let btn_label = if sufficient {
            format!("Pay ⭐ {}", self.stars_amount)
        } else {
            format!("Top up ⭐ {}", self.missing_stars())
        };

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
                    .child("Cancel"),
            )
            .child(
                div()
                    .px(px(20.0))
                    .py(px(8.0))
                    .rounded(px(6.0))
                    .bg(if sufficient { rgb(0xF59E0B) } else { rgb(0x2563EB) })
                    .cursor_pointer()
                    .text_color(rgb(0xFFFFFF))
                    .font_weight(FontWeight::BOLD)
                    .child(btn_label),
            );

        card.child(actions).into_any_element()
    }
}
