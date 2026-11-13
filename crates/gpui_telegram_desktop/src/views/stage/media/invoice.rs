//! Rich Invoice, Payment Checkout, Call & Decision Media Cards (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_invoice.cpp:L26-L112`
//! - `Telegram/SourceFiles/history/view/media/history_view_call.cpp:L33-L125`
//! - `Telegram/SourceFiles/history/view/media/history_view_suggest_decision.cpp:L28-L58`

use gpui::*;
use gpui::prelude::FluentBuilder;

pub const K_CREDITS_CURRENCY: &str = "XTR"; // Telegram Stars
pub const K_DESC_MAX_LINES: usize = 4096;

/// Upstream IsOrdinaryUnpaidCreditsInvoice from `history_view_invoice.cpp:L26-L31`
pub fn is_ordinary_unpaid_credits_invoice(
    currency: &str,
    receipt_msg_id: u64,
    is_test: bool,
    is_paid_media: bool,
) -> bool {
    currency == K_CREDITS_CURRENCY && receipt_msg_id == 0 && !is_test && !is_paid_media
}

/// Upstream format amount & currency helper
pub fn format_amount_and_currency(amount: i64, currency: &str) -> String {
    if currency == K_CREDITS_CURRENCY {
        format!("⭐ {}", amount)
    } else {
        format!("{:.2} {}", (amount as f64) / 100.0, currency)
    }
}

/// Upstream ComputeDuration from `history_view_call.cpp:L33-L37`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallState {
    Incoming,
    Outgoing,
    Missed,
    Busy,
}

pub fn compute_call_duration(state: CallState, duration: u32) -> u32 {
    if state != CallState::Missed && state != CallState::Busy {
        duration
    } else {
        0
    }
}

#[derive(Debug, Clone)]
pub struct RichInvoiceCard {
    pub title: String,
    pub description: String,
    pub amount: i64,
    pub currency: String,
    pub receipt_msg_id: u64,
    pub is_test: bool,
    pub is_paid_media: bool,
    pub photo_url: Option<String>,
}

impl RichInvoiceCard {
    pub fn new(title: impl Into<String>, amount: i64, currency: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: String::new(),
            amount,
            currency: currency.into(),
            receipt_msg_id: 0,
            is_test: false,
            is_paid_media: false,
            photo_url: None,
        }
    }

    pub fn with_receipt(mut self, receipt_id: u64) -> Self {
        self.receipt_msg_id = receipt_id;
        self
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    pub fn is_receipt(&self) -> bool {
        self.receipt_msg_id != 0
    }

    pub fn status_label(&self) -> String {
        if self.is_receipt() {
            if self.is_test {
                "TEST RECEIPT".to_string()
            } else {
                "RECEIPT".to_string()
            }
        } else if self.is_test {
            "TEST INVOICE".to_string()
        } else {
            "INVOICE".to_string()
        }
    }

    pub fn render(self) -> impl IntoElement {
        let formatted_price = format_amount_and_currency(self.amount, &self.currency);
        let status = self.status_label();
        let is_paid = self.is_receipt();
        let title = self.title;
        let description = self.description;

        div()
            .flex()
            .flex_col()
            .w(px(340.0))
            .p(px(14.0))
            .rounded(px(12.0))
            .bg(rgb(0x182533))
            .border_1()
            .border_color(rgb(0x233140))
            .gap(px(8.0))
            // Header status
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(11.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(if is_paid { rgb(0x4CAF50) } else { rgb(0x2AABEE) })
                            .child(status),
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child(formatted_price.clone()),
                    ),
            )
            // Title
            .child(
                div()
                    .text_size(px(15.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xFFFFFF))
                    .child(title),
            )
            // Description
            .when(!description.is_empty(), |el| {
                el.child(
                    div()
                        .text_size(px(12.0))
                        .text_color(rgb(0x8A9AA8))
                        .line_height(px(16.0))
                        .child(description),
                )
            })
            // Pay / Receipt Button
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .mt(px(4.0))
                    .py(px(8.0))
                    .px(px(14.0))
                    .rounded(px(8.0))
                    .bg(if is_paid { rgb(0x233140) } else { rgb(0x2AABEE) })
                    .text_size(px(13.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xFFFFFF))
                    .child(if is_paid {
                        format!("Show Receipt ({})", formatted_price)
                    } else {
                        format!("Pay {}", formatted_price)
                    }),
            )
    }
}

#[derive(Debug, Clone)]
pub struct RichCallCard {
    pub state: CallState,
    pub duration_seconds: u32,
    pub is_video: bool,
    pub is_conference: bool,
    pub timestamp_str: String,
}

impl RichCallCard {
    pub fn new(state: CallState, duration: u32, is_video: bool) -> Self {
        Self {
            state,
            duration_seconds: compute_call_duration(state, duration),
            is_video,
            is_conference: false,
            timestamp_str: String::new(),
        }
    }

    pub fn render(self) -> impl IntoElement {
        let is_missed = self.state == CallState::Missed || self.state == CallState::Busy;
        let call_label = match (self.is_video, self.is_conference, self.state) {
            (true, _, CallState::Incoming) => "Incoming Video Call",
            (true, _, CallState::Outgoing) => "Outgoing Video Call",
            (true, _, _) => "Missed Video Call",
            (false, true, _) => "Conference Call",
            (false, false, CallState::Incoming) => "Incoming Call",
            (false, false, CallState::Outgoing) => "Outgoing Call",
            (false, false, CallState::Busy) => "Busy Call",
            (false, false, CallState::Missed) => "Missed Call",
        };
        let duration_seconds = self.duration_seconds;
        let timestamp_str = self.timestamp_str;
        let is_video = self.is_video;
        let is_conference = self.is_conference;

        div()
            .flex()
            .items_center()
            .justify_between()
            .w(px(280.0))
            .p(px(10.0))
            .rounded(px(10.0))
            .bg(rgb(0x182533))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(if is_missed { rgb(0xE55B5B) } else { rgb(0xFFFFFF) })
                            .child(call_label),
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(0x8A9AA8))
                            .child(if duration_seconds > 0 {
                                format!("{}s", duration_seconds)
                            } else {
                                timestamp_str
                            }),
                    ),
            )
            .child(
                div()
                    .text_size(px(16.0))
                    .child(if is_video { "📹" } else if is_conference { "👥" } else { "📞" }),
            )
    }
}
