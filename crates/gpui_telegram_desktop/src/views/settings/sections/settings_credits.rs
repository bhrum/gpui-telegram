//! Authentic GPUI Settings Credits Section Component
//! Source: Telegram/SourceFiles/sections/settings_credits.cpp, Telegram/SourceFiles/sections/settings_credits.h
//! Subsystem: settings | Feature ID: TCG-N31 | Disposition: rust_domain_or_server_1to1

use gpui::*;

pub const K_FIRST_PAGE_LIMIT: usize = 20;

#[derive(Debug, Clone)]
pub struct StarsTransaction {
    pub id: String,
    pub title: String,
    pub date: String,
    pub amount: i64,
    pub is_outgoing: bool,
}

#[derive(Debug, Clone)]
pub struct SettingsCreditsSections {
    pub balance_stars: u64,
    pub currency_code: String,
    pub transactions: Vec<StarsTransaction>,
    pub ton_balance: f64,
    pub is_withdrawal_enabled: bool,
    pub is_visible: bool,
}

impl SettingsCreditsSections {
    pub fn new() -> Self {
        Self {
            balance_stars: 1500,
            currency_code: "USD".into(),
            transactions: vec![
                StarsTransaction { id: "tx_1".into(), title: "Gift to @alice".into(), date: "Nov 5".into(), amount: -150, is_outgoing: true },
                StarsTransaction { id: "tx_2".into(), title: "Bot Reaction Reward".into(), date: "Nov 3".into(), amount: 50, is_outgoing: false },
                StarsTransaction { id: "tx_3".into(), title: "Top-up 1,000 Stars".into(), date: "Nov 1".into(), amount: 1000, is_outgoing: false },
            ],
            ton_balance: 14.25,
            is_withdrawal_enabled: true,
            is_visible: true,
        }
    }

    pub fn add_stars(&mut self, amount: u64) {
        self.topup(amount);
    }

    pub fn can_withdraw(&self) -> bool {
        self.is_withdrawal_enabled && self.balance_stars >= 1000
    }

    pub fn topup(&mut self, amount: u64) {
        self.balance_stars = self.balance_stars.saturating_add(amount);
        self.transactions.insert(0, StarsTransaction {
            id: format!("tx_{}", self.transactions.len() + 1),
            title: format!("Top-up {} Stars", amount),
            date: "Just now".into(),
            amount: amount as i64,
            is_outgoing: false,
        });
    }

    pub fn credits(&self) -> u64 {
        self.balance_stars
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }
}

impl Default for SettingsCreditsSections {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsCreditsSections {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }

        let balance = self.balance_stars;

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .bg(rgb(0x0e1621))
            .child(
                // Header card with Stars balance and Topup action
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .p(px(20.0))
                    .rounded(px(12.0))
                    .bg(rgb(0x17212b))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x8292a1))
                                    .child("Available Telegram Stars"),
                            )
                            .child(
                                div()
                                    .text_2xl()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xffd700))
                                    .child(format!("⭐ {}", balance)),
                            ),
                    )
                    .child(
                        div()
                            .px(px(16.0))
                            .py(px(8.0))
                            .rounded(px(8.0))
                            .bg(rgb(0x5288c1))
                            .hover(|s| s.bg(rgb(0x4277b0)))
                            .cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                this.topup(500);
                                cx.notify();
                            }))
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0xffffff))
                                    .child("+ Buy Stars"),
                            ),
                    ),
            )
            .child(
                // Withdrawal / TON summary row
                div()
                    .mt(px(12.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .p(px(12.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x17212b))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(div().text_sm().text_color(rgb(0xffffff)).child("Creator Rewards (TON)"))
                            .child(div().text_xs().text_color(rgb(0x8292a1)).child(format!("{:.2} TON available", self.ton_balance))),
                    )
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x5288c1))
                            .cursor_pointer()
                            .child("Withdraw via Fragment ›"),
                    ),
            )
            .child(
                // Transaction History Table
                div()
                    .mt(px(16.0))
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(rgb(0x5288c1)).child("RECENT TRANSACTIONS"))
                    .children(self.transactions.iter().map(|tx| {
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .px(px(12.0))
                            .py(px(8.0))
                            .rounded(px(6.0))
                            .bg(rgb(0x17212b))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(div().text_sm().text_color(rgb(0xffffff)).child(tx.title.clone()))
                                    .child(div().text_xs().text_color(rgb(0x6c7883)).child(tx.date.clone())),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(if tx.is_outgoing { rgb(0xd14e4e) } else { rgb(0x2b8042) })
                                    .child(if tx.is_outgoing { format!("{} Stars", tx.amount) } else { format!("+{} Stars", tx.amount) }),
                            )
                    })),
            )
            .into_any_element()
    }
}
