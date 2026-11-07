//! Authentic GPUI implementation of Telegram Desktop's PremiumLimitsBox.
//! Upstream reference: `Telegram/SourceFiles/boxes/premium_limits_box.h:L44-L85`
//! and `Telegram/SourceFiles/boxes/premium_limits_box.cpp:L544-L1080`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Limit resource type corresponding to upstream limit box triggers (`premium_limits_box.h:L44-L85`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LimitType {
    Channels,
    Pins,
    PublicLinks,
    Filters,
    FileSize,
    Caption,
    Accounts,
}

impl LimitType {
    pub fn default_limit(&self) -> u64 {
        match self {
            Self::Channels => 500,
            Self::Pins => 5,
            Self::PublicLinks => 10,
            Self::Filters => 10,
            Self::FileSize => 2 * 1024 * 1024 * 1024, // 2 GB
            Self::Caption => 1024,
            Self::Accounts => 3,
        }
    }

    pub fn premium_limit(&self) -> u64 {
        match self {
            Self::Channels => 1000,
            Self::Pins => 10,
            Self::PublicLinks => 20,
            Self::Filters => 20,
            Self::FileSize => 4 * 1024 * 1024 * 1024, // 4 GB
            Self::Caption => 2048,
            Self::Accounts => 4,
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Self::Channels => "Channels & Groups Limit",
            Self::Pins => "Pinned Chats Limit",
            Self::PublicLinks => "Public Links Limit",
            Self::Filters => "Chat Folders Limit",
            Self::FileSize => "File Upload Size Limit",
            Self::Caption => "Caption Character Limit",
            Self::Accounts => "Connected Accounts Limit",
        }
    }
}

/// View component displaying user's current limit usage vs doubled Premium limits.
#[derive(Clone)]
pub struct PremiumLimitsBox {
    pub limit_type: LimitType,
    pub current_usage: u64,
    pub is_premium: bool,
    pub is_closed: bool,
}

impl Default for PremiumLimitsBox {
    fn default() -> Self {
        Self {
            limit_type: LimitType::Channels,
            current_usage: 500,
            is_premium: false,
            is_closed: false,
        }
    }
}

impl PremiumLimitsBox {
    pub fn new(limit_type: LimitType, current_usage: u64, is_premium: bool) -> Self {
        Self {
            limit_type,
            current_usage,
            is_premium,
            is_closed: false,
        }
    }

    /// Calculate percentage fill of progress bar.
    pub fn usage_ratio(&self) -> f32 {
        let max_val = self.limit_type.premium_limit() as f32;
        (self.current_usage as f32 / max_val).clamp(0.0, 1.0)
    }

    pub fn close(&mut self) {
        self.is_closed = true;
    }
}

impl Render for PremiumLimitsBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        let def_limit = self.limit_type.default_limit();
        let prem_limit = self.limit_type.premium_limit();

        let mut card = div()
            .id("premium_limits_box")
            .w(px(400.0))
            .p(px(24.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(12.0))
            .shadow_lg()
            .flex()
            .flex_col()
            .gap(px(16.0));

        // Header Title
        card = card.child(
            div()
                .flex()
                .items_center()
                .gap(px(10.0))
                .child(
                    div()
                        .text_size(px(22.0))
                        .child("⭐"),
                )
                .child(
                    div()
                        .text_size(px(18.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x1F2937))
                        .child(self.limit_type.title()),
                ),
        );

        // Usage Infographic Progress Bar
        let ratio = self.usage_ratio();
        card = card.child(
            div()
                .flex()
                .flex_col()
                .gap(px(6.0))
                .child(
                    div()
                        .flex()
                        .justify_between()
                        .text_size(px(13.0))
                        .text_color(rgb(0x4B5563))
                        .child(format!("Current: {}", self.current_usage))
                        .child(format!("Doubled: {}", prem_limit)),
                )
                .child(
                    div()
                        .w_full()
                        .h(px(8.0))
                        .rounded_full()
                        .bg(rgb(0xE5E7EB))
                        .child(
                            div()
                                .h_full()
                                .rounded_full()
                                .bg(rgb(0x8B5CF6))
                                .w(relative(ratio)),
                        ),
                ),
        );

        // Comparison Rows
        let comparison = div()
            .flex()
            .justify_around()
            .p(px(12.0))
            .bg(rgb(0xF9FAFB))
            .rounded(px(8.0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(0x6B7280))
                            .child("Free Tier"),
                    )
                    .child(
                        div()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x374151))
                            .child(def_limit.to_string()),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(0x8B5CF6))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("Premium (2x)"),
                    )
                    .child(
                        div()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x7C3AED))
                            .child(prem_limit.to_string()),
                    ),
            );
        card = card.child(comparison);

        // Action Button: Upgrade to Telegram Premium
        let btn = div()
            .flex()
            .justify_end()
            .child(
                div()
                    .px(px(20.0))
                    .py(px(10.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x7C3AED))
                    .cursor_pointer()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xFFFFFF))
                    .child("Double Your Limits"),
            );

        card.child(btn).into_any_element()
    }
}
