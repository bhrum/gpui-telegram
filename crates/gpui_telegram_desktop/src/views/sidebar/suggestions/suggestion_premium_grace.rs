//! Telegram Premium payment grace period suggestion banner.
//!
//! Upstream: `Telegram/SourceFiles/dialogs/suggestions/suggestion_premium_grace.cpp:L20-70`
//! Subsystem: dialogs, Feature: TCG-N03

use super::suggestion_manager::{SuggestionItem, SuggestionPriority};

#[derive(Debug, Clone, Default)]
pub struct SuggestionPremiumGraceSuggestions {
    pub is_grace_period: bool,
}

impl SuggestionPremiumGraceSuggestions {
    pub const KEY: &'static str = "PREMIUM_GRACE";

    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_item(&self) -> SuggestionItem {
        SuggestionItem {
            priority: SuggestionPriority::PremiumGrace,
            key: Self::KEY.to_string(),
            title: "Premium Subscription Paused".to_string(),
            description: "Update payment method to keep Telegram Premium active.".to_string(),
            button_text: Some("Renew".to_string()),
            action_url: Some("https://t.me/premiumbot?start=status".to_string()),
            can_close: true,
        }
    }
}
