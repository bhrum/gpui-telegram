//! Authentic GPUI Settings Premium Section Component
//! Source: Telegram/SourceFiles/settings/sections/settings_premium.cpp, Telegram/SourceFiles/settings/sections/settings_premium.h
//! Subsystem: settings | Feature ID: TCG-N31 | Disposition: rust_domain_or_server_1to1

use gpui::prelude::FluentBuilder;
use gpui::*;

#[derive(Debug, Clone)]
pub struct PremiumTier {
    pub months: u32,
    pub title: &'static str,
    pub price_per_month: f32,
    pub total_price: f32,
    pub discount_pct: u32,
}

#[derive(Debug, Clone)]
pub struct SettingsPremiumSections {
    pub is_premium_active: bool,
    pub selected_months: u32,
    pub tiers: Vec<PremiumTier>,
    pub is_visible: bool,
}

impl SettingsPremiumSections {
    pub fn new() -> Self {
        Self {
            is_premium_active: true,
            selected_months: 12,
            tiers: vec![
                PremiumTier { months: 1, title: "1 Month", price_per_month: 4.99, total_price: 4.99, discount_pct: 0 },
                PremiumTier { months: 6, title: "6 Months", price_per_month: 3.99, total_price: 23.99, discount_pct: 20 },
                PremiumTier { months: 12, title: "12 Months", price_per_month: 2.99, total_price: 35.99, discount_pct: 40 },
            ],
            is_visible: true,
        }
    }

    pub fn select_tier(&mut self, months: u32) {
        self.selected_months = months;
    }

    pub fn subscribe(&mut self, months: u32) {
        self.selected_months = months;
        self.is_premium_active = true;
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }
}

impl Default for SettingsPremiumSections {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsPremiumSections {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }

        let selected = self.selected_months;
        let is_active = self.is_premium_active;

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .bg(rgb(0x0e1621))
            .child(
                // Hero Banner with Animated Star Badge
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .p(px(20.0))
                    .rounded(px(12.0))
                    .bg(rgb(0x17212b))
                    .child(
                        div()
                            .size(px(48.0))
                            .rounded(px(24.0))
                            .bg(rgb(0x9b51e0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_2xl()
                            .child("👑"),
                    )
                    .child(
                        div()
                            .mt(px(8.0))
                            .text_xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .child("Telegram Premium"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(if is_active { rgb(0x4fae57) } else { rgb(0x8292a1) })
                            .child(if is_active { "Active Subscriber" } else { "Subscribe to unlock exclusive features" }),
                    ),
            )
            .child(
                // Subscription Tier Selector Cards
                div()
                    .mt(px(12.0))
                    .flex()
                    .flex_row()
                    .gap(px(8.0))
                    .children(self.tiers.iter().map(|tier| {
                        let months = tier.months;
                        let is_sel = months == selected;
                        div()
                            .flex_1()
                            .p(px(12.0))
                            .rounded(px(8.0))
                            .bg(if is_sel { rgb(0x232e3c) } else { rgb(0x17212b) })
                            .border_1()
                            .border_color(if is_sel { rgb(0x9b51e0) } else { rgb(0x17212b) })
                            .cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _window, cx| {
                                this.select_tier(months);
                                cx.notify();
                            }))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .justify_between()
                                            .child(div().text_sm().font_weight(FontWeight::BOLD).text_color(rgb(0xffffff)).child(tier.title))
                                            .when(tier.discount_pct > 0, |this| {
                                                this.child(
                                                    div()
                                                        .px(px(4.0))
                                                        .py(px(1.0))
                                                        .rounded(px(3.0))
                                                        .bg(rgb(0x2b8042))
                                                        .text_xs()
                                                        .text_color(rgb(0xffffff))
                                                        .child(format!("-{}%", tier.discount_pct)),
                                                )
                                            }),
                                    )
                                    .child(
                                        div()
                                            .mt(px(4.0))
                                            .text_xs()
                                            .text_color(rgb(0x9b51e0))
                                            .child(format!("${:.2}/mo", tier.price_per_month)),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x6c7883))
                                            .child(format!("${:.2} total", tier.total_price)),
                                    ),
                            )
                    })),
            )
            .child(
                // Feature Showcase List
                div()
                    .mt(px(16.0))
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(rgb(0x9b51e0)).child("PREMIUM PERKS"))
                    .children(vec![
                        ("⚡ 4 GB Uploads", "Send documents and media up to 4 GB each"),
                        ("🚀 Faster Download Speed", "No speed limits when downloading files and media"),
                        ("🎙️ Voice-to-Text Transcription", "Read transcripts of any voice and video message"),
                        ("✨ Infinite Custom Reactions", "React with thousands of animated emoji icons"),
                        ("🏷️ Tags for Saved Messages", "Filter and organize your saved messages by tag"),
                        ("🚫 No Ads", "No sponsored messages in public channels"),
                    ].into_iter().map(|(perk, desc)| {
                        div()
                            .flex()
                            .flex_col()
                            .px(px(12.0))
                            .py(px(8.0))
                            .rounded(px(6.0))
                            .bg(rgb(0x17212b))
                            .child(div().text_sm().font_weight(FontWeight::MEDIUM).text_color(rgb(0xffffff)).child(perk))
                            .child(div().text_xs().text_color(rgb(0x6c7883)).child(desc))
                    })),
            )
            .into_any_element()
    }
}
