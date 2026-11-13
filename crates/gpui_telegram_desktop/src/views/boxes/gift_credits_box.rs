//! Authentic GPUI implementation of Telegram Desktop's GiftCreditsBox.
//! Upstream reference: `Telegram/SourceFiles/boxes/gift_credits_box.h:L14-L20`
//! and `Telegram/SourceFiles/boxes/gift_credits_box.cpp:L40-L183`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::sync::Arc;

/// Preset Telegram Stars credit options per `gift_credits_box.cpp:L166-L180`.
pub const CREDIT_PRESETS: [u64; 6] = [50, 100, 250, 500, 1000, 2500];

/// Data model for a recipient user in frequent gifts list (`gift_credits_box.cpp:L44-L76`).
#[derive(Clone, Debug)]
pub struct GiftRecipient {
    pub peer_id: u64,
    pub name: String,
    pub username: Option<String>,
    pub is_frequent: bool,
}

/// View component for gifting Telegram Stars credits to contacts.
#[derive(Clone)]
pub struct GiftCreditsBox {
    pub selected_recipient: Option<GiftRecipient>,
    pub selected_stars: u64,
    pub custom_stars: Option<u64>,
    pub frequent_recipients: Vec<GiftRecipient>,
    pub is_submitting: bool,
    pub is_closed: bool,
    pub on_gifted: Option<Arc<dyn Fn(u64, u64) + Send + Sync>>,
}

impl Default for GiftCreditsBox {
    fn default() -> Self {
        Self {
            selected_recipient: None,
            selected_stars: 100,
            custom_stars: None,
            frequent_recipients: Vec::new(),
            is_submitting: false,
            is_closed: false,
            on_gifted: None,
        }
    }
}

impl GiftCreditsBox {
    pub fn new(
        recipient: Option<GiftRecipient>,
        frequent: Vec<GiftRecipient>,
        on_gifted: Option<Arc<dyn Fn(u64, u64) + Send + Sync>>,
    ) -> Self {
        Self {
            selected_recipient: recipient,
            selected_stars: 100,
            custom_stars: None,
            frequent_recipients: frequent,
            is_submitting: false,
            is_closed: false,
            on_gifted,
        }
    }

    /// Select a gift credit preset tier (`gift_credits_box.cpp:L166-L175`).
    pub fn select_preset(&mut self, amount: u64) {
        self.selected_stars = amount;
        self.custom_stars = None;
    }

    /// Select recipient peer from frequent list.
    pub fn select_recipient(&mut self, recipient: GiftRecipient) {
        self.selected_recipient = Some(recipient);
    }

    /// Confirm gifting action and invoke callback.
    pub fn confirm_gift(&mut self) {
        if let Some(ref r) = self.selected_recipient {
            let amount = self.custom_stars.unwrap_or(self.selected_stars);
            if let Some(ref cb) = self.on_gifted {
                cb(r.peer_id, amount);
            }
            self.is_closed = true;
        }
    }

    pub fn close(&mut self) {
        self.is_closed = true;
    }
}

impl Render for GiftCreditsBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        let mut card = div()
            .id("gift_credits_box")
            .w(px(420.0)) // st::boxWideWidth (420px)
            .p(px(24.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(12.0))
            .shadow_lg()
            .flex()
            .flex_col()
            .gap(px(16.0));

        // Header: Star icon and Title
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
                        .child("Gift Telegram Stars"),
                ),
        );

        // Recipient section
        if let Some(ref r) = self.selected_recipient {
            card = card.child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(12.0))
                    .p(px(12.0))
                    .bg(rgb(0xF9FAFB))
                    .rounded(px(8.0))
                    .child(
                        div()
                            .w(px(40.0))
                            .h(px(40.0))
                            .rounded_full()
                            .bg(rgb(0x3B82F6))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_color(rgb(0xFFFFFF))
                            .font_weight(FontWeight::BOLD)
                            .child(r.name.chars().next().unwrap_or('?').to_string()),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0x111827))
                                    .child(r.name.clone()),
                            )
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(rgb(0x6B7280))
                                    .child(r.username.clone().unwrap_or_default()),
                            ),
                    ),
            );
        }

        // Preset Chips Grid
        let mut presets_row = div().flex().flex_wrap().gap(px(8.0));
        for &amount in &CREDIT_PRESETS {
            let is_selected = self.selected_stars == amount && self.custom_stars.is_none();
            let bg_col = if is_selected { rgb(0xFEF3C7) } else { rgb(0xF3F4F6) };
            let border_col = if is_selected { rgb(0xF59E0B) } else { rgb(0xE5E7EB) };
            let text_col = if is_selected { rgb(0x92400E) } else { rgb(0x374151) };

            presets_row = presets_row.child(
                div()
                    .px(px(14.0))
                    .py(px(8.0))
                    .rounded(px(8.0))
                    .bg(bg_col)
                    .border_1()
                    .border_color(border_col)
                    .cursor_pointer()
                    .text_size(px(13.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_col)
                    .child(format!("⭐ {}", amount)),
            );
        }
        card = card.child(presets_row);

        // Action Buttons: Cancel and Send Gift
        let btns = div()
            .flex()
            .justify_end()
            .items_center()
            .gap(px(10.0))
            .child(
                div()
                    .px(px(16.0))
                    .py(px(8.0))
                    .rounded(px(6.0))
                    .cursor_pointer()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgb(0x6B7280))
                    .child("Cancel"),
            )
            .child(
                div()
                    .px(px(20.0))
                    .py(px(8.0))
                    .rounded(px(6.0))
                    .bg(rgb(0xF59E0B))
                    .cursor_pointer()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xFFFFFF))
                    .child(format!("Gift ⭐ {}", self.selected_stars)),
            );

        card.child(btns).into_any_element()
    }
}
