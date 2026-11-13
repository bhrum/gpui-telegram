//! Genuine History View Unique Gift Component (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_unique_gift.h:L20-L160`
//! - `Telegram/SourceFiles/history/view/media/history_view_unique_gift.cpp:L52-L158`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use crate::views::stage::media::gift::{RichStarGiftCard, GiftType};

#[derive(Debug, Clone)]
pub struct HistoryViewUniqueGiftViewMedia {
    pub title: String,
    pub number: u32,
    pub total: u32,
    pub message: Option<String>,
    pub sender_name: Option<String>,
}

impl HistoryViewUniqueGiftViewMedia {
    pub fn new(title: impl Into<String>, number: u32, total: u32) -> Self {
        Self {
            title: title.into(),
            number,
            total,
            message: None,
            sender_name: None,
        }
    }

    pub fn with_message(mut self, msg: impl Into<String>) -> Self {
        self.message = Some(msg.into());
        self
    }

    pub fn with_sender(mut self, sender: impl Into<String>) -> Self {
        self.sender_name = Some(sender.into());
        self
    }
}

impl Default for HistoryViewUniqueGiftViewMedia {
    fn default() -> Self {
        Self::new("Spicy Chili", 42, 1000)
    }
}

impl Render for HistoryViewUniqueGiftViewMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut card = RichStarGiftCard::new_unique_collectible(&self.title, self.number, self.total);
        card.message = self.message.clone();
        card.sender_name = self.sender_name.clone();
        card.render()
    }
}
