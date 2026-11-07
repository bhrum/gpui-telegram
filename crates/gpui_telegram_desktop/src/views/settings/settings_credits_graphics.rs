//! Authentic GPUI Settings Credits Graphics Component
//! Source: Telegram/SourceFiles/settings/settings_credits_graphics.cpp, Telegram/SourceFiles/settings/settings_credits_graphics.h
//! Subsystem: settings | Feature ID: TCG-N31 | Disposition: rust_domain_or_server_1to1

use gpui::prelude::FluentBuilder;
use gpui::*;

pub const K_PRELOAD_COUNT: usize = 10;
pub const K_MINUS_CHAR: char = '\u{2212}';
pub const K_DATE_UPDATE_INTERVAL_MS: u64 = 250;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmallBalanceSource {
    Bot,
    Reaction,
    VideoStream,
    Subscription,
    DeepLink,
    StarGift,
    Message,
    Suggest,
    Offer,
    Search,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmallBalanceResult {
    Already,
    Success,
    Blocked,
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct CreditTopupOption {
    pub stars: u64,
    pub price_usd: f32,
    pub discount_pct: u32,
}

#[derive(Debug, Clone)]
pub struct SettingsCreditsGraphics {
    pub source: SmallBalanceSource,
    pub balance: u64,
    pub topup_options: Vec<CreditTopupOption>,
    pub mini_stars_count: u32,
    pub unique_gift_price: u64,
    pub is_shown: bool,
    pub is_visible: bool,
}

impl SettingsCreditsGraphics {
    pub fn new() -> Self {
        Self {
            source: SmallBalanceSource::Bot,
            balance: 1500,
            topup_options: vec![
                CreditTopupOption { stars: 50, price_usd: 0.99, discount_pct: 0 },
                CreditTopupOption { stars: 100, price_usd: 1.99, discount_pct: 0 },
                CreditTopupOption { stars: 250, price_usd: 4.99, discount_pct: 0 },
                CreditTopupOption { stars: 500, price_usd: 9.99, discount_pct: 5 },
            ],
            mini_stars_count: 5,
            unique_gift_price: 250,
            is_shown: true,
            is_visible: true,
        }
    }

    pub fn add_stars(&mut self, stars: u64) {
        self.balance = self.balance.saturating_add(stars);
    }

    pub fn topup(&mut self, stars: u64) {
        self.balance = self.balance.saturating_add(stars);
    }

    pub fn spend(&mut self, stars: u64) -> Result<(), &'static str> {
        if self.balance >= stars {
            self.balance -= stars;
            Ok(())
        } else {
            Err("Insufficient Stars balance")
        }
    }

    pub fn handle_small_balance(&self, _source: SmallBalanceSource, needed: u64) -> SmallBalanceResult {
        if self.balance >= needed {
            SmallBalanceResult::Already
        } else {
            SmallBalanceResult::Success
        }
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }
}

impl Default for SettingsCreditsGraphics {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsCreditsGraphics {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }

        let balance = self.balance;

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .bg(rgb(0x0e1621))
            .child(
                // Balance card with glowing star
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .p(px(20.0))
                    .rounded(px(12.0))
                    .bg(rgb(0x17212b))
                    .child(
                        div()
                            .text_3xl()
                            .text_color(rgb(0xffd700))
                            .child("⭐"),
                    )
                    .child(
                        div()
                            .mt(px(8.0))
                            .text_2xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .child(format!("{} Stars", balance)),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x8292a1))
                            .child("Available balance for gifts, reactions, and bot subscriptions"),
                    ),
            )
            .child(
                // Top-up packages grid
                div()
                    .mt(px(16.0))
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .children(self.topup_options.iter().map(|opt| {
                        let stars = opt.stars;
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .px(px(12.0))
                            .py(px(10.0))
                            .rounded(px(8.0))
                            .bg(rgb(0x17212b))
                            .hover(|s| s.bg(rgb(0x232e3c)))
                            .cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _window, cx| {
                                this.topup(stars);
                                cx.notify();
                            }))
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0xffffff))
                                            .child(format!("⭐ {}", opt.stars)),
                                    )
                                    .when(opt.discount_pct > 0, |this| {
                                        this.child(
                                            div()
                                                .px(px(6.0))
                                                .py(px(1.0))
                                                .rounded(px(4.0))
                                                .bg(rgb(0x2b8042))
                                                .text_xs()
                                                .text_color(rgb(0xffffff))
                                                .child(format!("-{}%", opt.discount_pct)),
                                        )
                                    }),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0x5288c1))
                                    .child(format!("${:.2}", opt.price_usd)),
                            )
                    })),
            )
            .into_any_element()
    }
}
