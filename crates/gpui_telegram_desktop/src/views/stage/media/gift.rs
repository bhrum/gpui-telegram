//! Rich Star Gifts, Unique Collectibles, Giveaways & Birthday Cards (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_unique_gift.cpp:L52-L158`
//! - `Telegram/SourceFiles/history/view/media/history_view_premium_gift.cpp:L45-L95`
//! - `Telegram/SourceFiles/history/view/media/history_view_giveaway.cpp:L30-L60`
//! - `Telegram/SourceFiles/history/view/media/history_view_birthday_suggestion.cpp:L43-L60`
//! - `Telegram/SourceFiles/history/view/media/history_view_story_mention.cpp:L44`

use gpui::*;
use gpui::prelude::FluentBuilder;

pub const K_OUTLINE_RATIO: f64 = 0.85;
pub const K_READ_OUTLINE_ALPHA: f64 = 0.5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GiftType {
    PremiumSubscription,
    StarGift,
    UniqueCollectible,
    Giveaway,
}

#[derive(Debug, Clone)]
pub struct RichStarGiftCard {
    pub gift_type: GiftType,
    pub title: String,
    pub num_stars: u64,
    pub months: u32,
    pub collectible_number: Option<u32>,
    pub total_issued: Option<u32>,
    pub message: Option<String>,
    pub sender_name: Option<String>,
}

impl RichStarGiftCard {
    pub fn new_star_gift(title: impl Into<String>, num_stars: u64) -> Self {
        Self {
            gift_type: GiftType::StarGift,
            title: title.into(),
            num_stars,
            months: 0,
            collectible_number: None,
            total_issued: None,
            message: None,
            sender_name: None,
        }
    }

    pub fn new_unique_collectible(
        title: impl Into<String>,
        number: u32,
        total: u32,
    ) -> Self {
        Self {
            gift_type: GiftType::UniqueCollectible,
            title: title.into(),
            num_stars: 0,
            months: 0,
            collectible_number: Some(number),
            total_issued: Some(total),
            message: None,
            sender_name: None,
        }
    }

    pub fn new_premium_subscription(months: u32) -> Self {
        Self {
            gift_type: GiftType::PremiumSubscription,
            title: format!("Telegram Premium for {} months", months),
            num_stars: 0,
            months,
            collectible_number: None,
            total_issued: None,
            message: None,
            sender_name: None,
        }
    }

    pub fn render(self) -> impl IntoElement {
        let badge_text = match self.gift_type {
            GiftType::UniqueCollectible => {
                if let (Some(num), Some(total)) = (self.collectible_number, self.total_issued) {
                    format!("Collectible #{} of {}", num, total)
                } else {
                    "Unique Collectible".to_string()
                }
            }
            GiftType::PremiumSubscription => format!("{} Months Premium", self.months),
            GiftType::StarGift => format!("⭐ {} Stars", self.num_stars),
            GiftType::Giveaway => "Giveaway Prize".to_string(),
        };
        let gift_type = self.gift_type;
        let title = self.title;
        let message = self.message;
        let sender_name = self.sender_name;

        div()
            .flex()
            .flex_col()
            .w(px(320.0))
            .p(px(14.0))
            .rounded(px(14.0))
            .bg(rgb(0x182533))
            .border_1()
            .border_color(match gift_type {
                GiftType::UniqueCollectible => rgb(0xAF52DE),
                GiftType::PremiumSubscription => rgb(0x8E52DE),
                _ => rgb(0xFFB300),
            })
            .gap(px(8.0))
            // Header with Gift emoji / badge
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .child(div().text_size(px(18.0)).child("🎁"))
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFB300))
                                    .child(badge_text),
                            ),
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
            // Personal gift message
            .when_some(message, |el, msg| {
                el.child(
                    div()
                        .p(px(8.0))
                        .rounded(px(8.0))
                        .bg(rgb(0x233140))
                        .text_size(px(12.0))
                        .text_color(rgb(0xCCCCCC))
                        .child(format!("\"{}\"", msg)),
                )
            })
            // Sender info
            .when_some(sender_name, |el, sender| {
                el.child(
                    div()
                        .text_size(px(11.0))
                        .text_color(rgb(0x8A9AA8))
                        .child(format!("From {}", sender)),
                )
            })
            // Open / Transfer button
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .py(px(6.0))
                    .rounded(px(6.0))
                    .bg(rgb(0x233140))
                    .text_size(px(12.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xFFFFFF))
                    .child(if gift_type == GiftType::UniqueCollectible {
                        "View in Profile"
                    } else {
                        "View Gift"
                    }),
            )
    }
}
