//! Genuine History View Gram/TON Transfer Component (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_gram_transfer.h:L17-L41`
//! - `Telegram/SourceFiles/history/view/media/history_view_gram_transfer.cpp:L53-L125`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use crate::views::stage::media::transfer::{
    RichTonTransferCard, format_ton_address_groups, format_nano_ton,
    BumpCurve, K_SETTLE_BUMP, K_READ_PRESS,
    K_ADDRESS_GROUP_SIZE, K_ADDRESS_GROUPS_PER_LINE, K_GLARE_DURATION_MS,
    K_GLARE_TIMEOUT_MS, K_BUMP_AMPLITUDE, K_BURST_STARS_PER_SIDE,
};

#[derive(Debug, Clone)]
pub struct HistoryViewGramTransferViewMedia {
    pub amount_nano_ton: i64,
    pub address: String,
    pub comment: Option<String>,
    pub is_outgoing: bool,
    pub tx_id: Option<String>,
}

impl HistoryViewGramTransferViewMedia {
    pub fn new(amount_nano_ton: i64, address: impl Into<String>, is_outgoing: bool) -> Self {
        Self {
            amount_nano_ton,
            address: address.into(),
            comment: None,
            is_outgoing,
            tx_id: None,
        }
    }

    pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }
}

impl Default for HistoryViewGramTransferViewMedia {
    fn default() -> Self {
        Self::new(1_000_000_000, "EQD4FPq-PRDieyQKkKZjmNu4", true)
    }
}

impl Render for HistoryViewGramTransferViewMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut card = RichTonTransferCard::new(self.amount_nano_ton, &self.address, self.is_outgoing);
        card.comment = self.comment.clone();
        card.render()
    }
}
