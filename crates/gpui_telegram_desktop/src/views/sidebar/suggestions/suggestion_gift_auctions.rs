//! Gift auctions active state suggestion.
//!
//! Upstream: `Telegram/SourceFiles/dialogs/suggestions/suggestion_gift_auctions.cpp:L20-75`
//! Subsystem: dialogs, Feature: TCG-N03

use super::suggestion_manager::{SuggestionItem, SuggestionPriority};

#[derive(Debug, Clone, Default)]
pub struct SuggestionGiftAuctionsSuggestions {
    pub has_active_auctions: bool,
    pub top_bid: u64,
}

impl SuggestionGiftAuctionsSuggestions {
    pub const KEY: &'static str = "GIFT_AUCTIONS";

    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_item(&self) -> SuggestionItem {
        SuggestionItem {
            priority: SuggestionPriority::GiftAuctions,
            key: Self::KEY.to_string(),
            title: "Live Gift Auction".to_string(),
            description: format!("Current highest bid: {} Stars", self.top_bid),
            button_text: Some("View Auction".to_string()),
            action_url: Some("internal:gift_auction".to_string()),
            can_close: true,
        }
    }
}
