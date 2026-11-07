//! History View Service Box View Component (`history_view_service_box`)
//!
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/media/history_view_service_box.h` (L1-L148)
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/media/history_view_service_box.cpp` (L1-L493)
//!
//! Renders rich interactive service message cards for Telegram Star Gifts, Premium Gifts,
//! Channel Boost acknowledgments, and Service Notifications.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Standard width of a gift/service box card.
/// Upstream: `history_view_service_box.cpp:L36` (`st::msgServiceGiftBoxSize.width()`) (~260px)
pub const K_SERVICE_GIFT_BOX_WIDTH: f32 = 260.0;

/// Top margin skip inside service gift card.
/// Upstream: `history_view_service_box.cpp:L83` (`st::msgServiceGiftBoxTopSkip`) (~8px)
pub const K_SERVICE_GIFT_BOX_TOP_SKIP: f32 = 8.0;

/// Category of service box message card.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceBoxCategory {
    StarGift { star_amount: u64 },
    PremiumGift { duration_months: u32 },
    ChannelBoost { boost_multiplier: u32 },
    SystemNotice,
}

/// Authentic GPUI View for Service Box cards in message history.
/// Upstream: `history_view_service_box.h:L72-L146`, `history_view_service_box.cpp:L39-L140`
#[derive(Debug, Clone)]
pub struct HistoryViewServiceBoxViewMedia {
    pub category: ServiceBoxCategory,
    pub title_text: String,
    pub author_text: String,
    pub subtitle_text: String,
    pub action_button_text: Option<String>,
    pub box_width: f32,
    pub is_active_card: bool,
}

impl HistoryViewServiceBoxViewMedia {
    /// Creates a generic service box card.
    pub fn new(title: impl Into<String>, subtitle: impl Into<String>) -> Self {
        Self {
            category: ServiceBoxCategory::SystemNotice,
            title_text: title.into(),
            author_text: String::new(),
            subtitle_text: subtitle.into(),
            action_button_text: None,
            box_width: K_SERVICE_GIFT_BOX_WIDTH,
            is_active_card: true,
        }
    }

    /// Creates a Star Gift service card.
    pub fn new_star_gift(sender: &str, star_amount: u64, message: &str) -> Self {
        Self {
            category: ServiceBoxCategory::StarGift { star_amount },
            title_text: format!("{} Telegram Stars", star_amount),
            author_text: format!("from {}", sender),
            subtitle_text: message.to_string(),
            action_button_text: Some("View Gift".to_string()),
            box_width: K_SERVICE_GIFT_BOX_WIDTH,
            is_active_card: true,
        }
    }

    /// Creates a Telegram Premium gift service card.
    pub fn new_premium_gift(sender: &str, months: u32) -> Self {
        Self {
            category: ServiceBoxCategory::PremiumGift { duration_months: months },
            title_text: format!("Telegram Premium for {} Months", months),
            author_text: format!("from {}", sender),
            subtitle_text: "Access exclusive stickers, increased limits and faster downloads.".to_string(),
            action_button_text: Some("Open Gift".to_string()),
            box_width: K_SERVICE_GIFT_BOX_WIDTH,
            is_active_card: true,
        }
    }

    /// Computes optimal card size.
    /// Upstream: `history_view_service_box.cpp:L81-L105`
    pub fn count_optimal_dimensions(&self) -> (f32, f32) {
        let height = K_SERVICE_GIFT_BOX_TOP_SKIP + 160.0 + if self.action_button_text.is_some() { 36.0 } else { 0.0 };
        (self.box_width, height)
    }
}

impl Default for HistoryViewServiceBoxViewMedia {
    fn default() -> Self {
        Self::new("Service Notification", "Details about this notification.")
    }
}

impl Render for HistoryViewServiceBoxViewMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let (icon, badge_color) = match self.category {
            ServiceBoxCategory::StarGift { .. } => ("⭐", rgb(0xFFD700)),
            ServiceBoxCategory::PremiumGift { .. } => ("🎁", rgb(0xA744E3)),
            ServiceBoxCategory::ChannelBoost { .. } => ("⚡", rgb(0x27A6E5)),
            ServiceBoxCategory::SystemNotice => ("ℹ️", rgb(0x888888)),
        };

        div()
            .flex()
            .flex_col()
            .items_center()
            .w_full()
            .py(px(8.0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .w(px(self.box_width))
                    .p(px(14.0))
                    .rounded(px(12.0))
                    .bg(rgb(0xFFFFFF))
                    .border_1()
                    .border_color(rgb(0xE6E8EB))
                    .shadow_sm()
                    // Icon badge
                    .child(
                        div()
                            .w(px(56.0))
                            .h(px(56.0))
                            .rounded_full()
                            .bg(badge_color)
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(28.0))
                            .mb(px(8.0))
                            .child(icon)
                    )
                    // Title
                    .child(
                        div()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x222222))
                            .text_align(TextAlign::Center)
                            .child(self.title_text.clone())
                    )
                    // Author
                    .children(if !self.author_text.is_empty() {
                        Some(
                            div()
                                .text_size(px(12.0))
                                .text_color(rgb(0x777777))
                                .mb(px(4.0))
                                .child(self.author_text.clone())
                        )
                    } else {
                        None
                    })
                    // Subtitle / message
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(0x555555))
                            .text_align(TextAlign::Center)
                            .mb(px(10.0))
                            .child(self.subtitle_text.clone())
                    )
                    // Action button
                    .children(if let Some(ref btn_text) = self.action_button_text {
                        Some(
                            div()
                                .w_full()
                                .py(px(6.0))
                                .rounded(px(6.0))
                                .bg(rgb(0x27A6E5))
                                .text_color(rgb(0xFFFFFF))
                                .text_size(px(13.0))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_align(TextAlign::Center)
                                .child(btn_text.clone())
                        )
                    } else {
                        None
                    })
            )
    }
}
