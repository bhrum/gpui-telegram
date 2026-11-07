//! Low credits subscription warning suggestion.
//!
//! Upstream: `Telegram/SourceFiles/dialogs/suggestions/suggestion_low_credits_subs.cpp:L1-130`
//! Subsystem: dialogs, Feature: TCG-N03

use super::suggestion_manager::{SuggestionItem, SuggestionPriority};

#[derive(Debug, Clone, Default)]
pub struct SuggestionLowCreditsSubsSuggestions {
    pub subscription_name: String,
    pub days_remaining: u32,
}

impl SuggestionLowCreditsSubsSuggestions {
    pub const KEY: &'static str = "LOW_CREDITS_SUBS";

    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_item(&self) -> SuggestionItem {
        SuggestionItem {
            priority: SuggestionPriority::LowCreditsSubs,
            key: Self::KEY.to_string(),
            title: "Subscription Expiring".to_string(),
            description: format!(
                "Your subscription to {} expires in {} days. Top up stars to continue.",
                self.subscription_name, self.days_remaining
            ),
            button_text: Some("Top Up".to_string()),
            action_url: Some("internal:topup_stars".to_string()),
            can_close: true,
        }
    }
}
