//! Submodule: crates/gpui_telegram_desktop/src/views/sidebar/suggestions
pub mod suggestion_manager;
pub use suggestion_manager::{SuggestionItem, SuggestionManager, SuggestionPriority, UnreviewedAuthDetails};

pub mod suggestion;
pub use suggestion::SuggestionSuggestions;
pub mod suggestion_birthday_contacts;
pub use suggestion_birthday_contacts::SuggestionBirthdayContactsSuggestions;
pub mod suggestion_birthday_setup;
pub use suggestion_birthday_setup::SuggestionBirthdaySetupSuggestions;
pub mod suggestion_custom_promo;
pub use suggestion_custom_promo::SuggestionCustomPromoSuggestions;
pub mod suggestion_gift_auctions;
pub use suggestion_gift_auctions::SuggestionGiftAuctionsSuggestions;
pub mod suggestion_low_credits_subs;
pub use suggestion_low_credits_subs::SuggestionLowCreditsSubsSuggestions;
pub mod suggestion_premium_grace;
pub use suggestion_premium_grace::SuggestionPremiumGraceSuggestions;
pub mod suggestion_premium_offer;
pub use suggestion_premium_offer::SuggestionPremiumOfferSuggestions;
pub mod suggestion_unreviewed_auth;
pub use suggestion_unreviewed_auth::SuggestionUnreviewedAuthSuggestions;
pub mod suggestion_userpic_setup;
pub use suggestion_userpic_setup::SuggestionUserpicSetupSuggestions;
