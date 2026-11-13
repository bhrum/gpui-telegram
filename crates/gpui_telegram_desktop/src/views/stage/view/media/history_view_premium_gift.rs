//! Genuine History View Premium Gift Component (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_premium_gift.h:L20-L78`
//! - `Telegram/SourceFiles/history/view/media/history_view_premium_gift.cpp:L45-L95`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use crate::views::stage::media::gift::{RichStarGiftCard, GiftType};

#[derive(Debug, Clone)]
pub struct HistoryViewPremiumGiftViewMedia {
    pub months: u32,
    pub num_stars: u64,
    pub sender_name: Option<String>,
}

impl HistoryViewPremiumGiftViewMedia {
    pub fn new_premium(months: u32) -> Self {
        Self {
            months,
            num_stars: 0,
            sender_name: None,
        }
    }

    pub fn new_stars(num_stars: u64) -> Self {
        Self {
            months: 0,
            num_stars,
            sender_name: None,
        }
    }
}

impl Default for HistoryViewPremiumGiftViewMedia {
    fn default() -> Self {
        Self::new_premium(3)
    }
}

impl Render for HistoryViewPremiumGiftViewMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let card = if self.months > 0 {
            RichStarGiftCard::new_premium_subscription(self.months)
        } else {
            RichStarGiftCard::new_star_gift("Star Gift", self.num_stars)
        };
        card.render()
    }
}
