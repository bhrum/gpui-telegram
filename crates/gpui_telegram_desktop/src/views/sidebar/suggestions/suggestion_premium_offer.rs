//! Telegram Premium discounted offer suggestion.
//!
//! Upstream: `Telegram/SourceFiles/dialogs/suggestions/suggestion_premium_offer.cpp:L20-113`
//! Subsystem: dialogs, Feature: TCG-N03

use super::suggestion_manager::{SuggestionItem, SuggestionPriority};

#[derive(Debug, Clone, Default)]
pub struct SuggestionPremiumOfferSuggestions {
    pub discount_percent: u32,
}

impl SuggestionPremiumOfferSuggestions {
    pub const KEY: &'static str = "PREMIUM_OFFER";

    pub fn new() -> Self {
        Self { discount_percent: 40 }
    }

    pub fn to_item(&self) -> SuggestionItem {
        SuggestionItem {
            priority: SuggestionPriority::PremiumOffer,
            key: Self::KEY.to_string(),
            title: "Telegram Premium".to_string(),
            description: format!("Subscribe now and save {}% on annual billing.", self.discount_percent),
            button_text: Some("View Offer".to_string()),
            action_url: Some("internal:premium_offer".to_string()),
            can_close: true,
        }
    }
}
