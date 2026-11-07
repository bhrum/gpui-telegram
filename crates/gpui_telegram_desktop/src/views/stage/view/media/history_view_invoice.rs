//! Genuine History View Invoice Component (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_invoice.h:L25-L106`
//! - `Telegram/SourceFiles/history/view/media/history_view_invoice.cpp:L26-L112`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use crate::views::stage::media::invoice::{
    RichInvoiceCard, is_ordinary_unpaid_credits_invoice,
    format_amount_and_currency, K_CREDITS_CURRENCY, K_DESC_MAX_LINES,
};

#[derive(Debug, Clone)]
pub struct HistoryViewInvoiceViewMedia {
    pub title: String,
    pub description: String,
    pub amount: i64,
    pub currency: String,
    pub receipt_msg_id: u64,
    pub is_test: bool,
    pub is_paid_media: bool,
}

impl HistoryViewInvoiceViewMedia {
    pub fn new(title: impl Into<String>, amount: i64, currency: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: String::new(),
            amount,
            currency: currency.into(),
            receipt_msg_id: 0,
            is_test: false,
            is_paid_media: false,
        }
    }

    pub fn with_receipt(mut self, receipt_id: u64) -> Self {
        self.receipt_msg_id = receipt_id;
        self
    }

    pub fn is_unpaid_credits(&self) -> bool {
        is_ordinary_unpaid_credits_invoice(&self.currency, self.receipt_msg_id, self.is_test, self.is_paid_media)
    }
}

impl Default for HistoryViewInvoiceViewMedia {
    fn default() -> Self {
        Self::new("Telegram Stars", 100, K_CREDITS_CURRENCY)
    }
}

impl Render for HistoryViewInvoiceViewMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut card = RichInvoiceCard::new(&self.title, self.amount, &self.currency);
        card.description = self.description.clone();
        card.receipt_msg_id = self.receipt_msg_id;
        card.is_test = self.is_test;
        card.is_paid_media = self.is_paid_media;
        card.render()
    }
}
