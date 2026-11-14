use gpui::*;
use crate::i18n::tr_str;
use crate::theme::assets::{tg_icon, IconName};
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::settings::settings_theme_ui::{
    settings_card, settings_container, settings_empty_state, settings_section_header,
    settings_subsection_title,
};

pub const K_FIRST_PAGE_LIMIT: usize = 20;

#[derive(Debug, Clone)]
pub struct StarsTransaction {
    pub id: String,
    pub title: String,
    pub date: String,
    pub amount: i64,
    pub is_outgoing: bool,
}

pub struct SettingsCreditsSections {
    pub balance_stars: u64,
    pub currency_code: String,
    pub transactions: Vec<StarsTransaction>,
    pub ton_balance: f64,
    pub is_withdrawal_enabled: bool,
    pub is_visible: bool,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
    pub pending_topup_stars: Option<u64>,
    pub pending_withdraw: bool,
}

impl SettingsCreditsSections {
    pub fn new() -> Self {
        Self {
            balance_stars: 1500,
            currency_code: "XTR".into(),
            transactions: vec![
                StarsTransaction { id: "tx_1".into(), title: "Bot Payment (@ShopBot)".into(), date: "Today, 14:20".into(), amount: -250, is_outgoing: true },
                StarsTransaction { id: "tx_2".into(), title: "Purchased Telegram Stars".into(), date: "Yesterday, 10:15".into(), amount: 1000, is_outgoing: false },
                StarsTransaction { id: "tx_3".into(), title: "Creator Channel Subscription".into(), date: "Oct 12, 2024".into(), amount: -100, is_outgoing: true },
            ],
            ton_balance: 12.45,
            is_withdrawal_enabled: true,
            is_visible: true,
            palette: TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x3390ec, 14, 12),
            language_id: "en".into(),
            pending_topup_stars: None,
            pending_withdraw: false,
        }
    }

    pub fn new_live(palette: TdesktopThemePalette, language_id: &str) -> Self {
        Self {
            balance_stars: 0,
            currency_code: "XTR".into(),
            transactions: Vec::new(),
            ton_balance: 0.0,
            is_withdrawal_enabled: false,
            is_visible: true,
            palette,
            language_id: language_id.into(),
            pending_topup_stars: None,
            pending_withdraw: false,
        }
    }

    pub fn sync_wallet(&mut self, stars: u64, ton_balance: f64, transactions: Vec<StarsTransaction>) {
        self.balance_stars = stars;
        self.ton_balance = ton_balance;
        self.is_withdrawal_enabled = stars >= 1000 || ton_balance > 0.0;
        self.transactions = transactions;
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) { self.palette = palette; }
    pub fn set_language(&mut self, language_id: impl Into<String>) { self.language_id = language_id.into(); }
    pub fn add_stars(&mut self, amount: u64) { self.balance_stars += amount; }
    pub fn can_withdraw(&self) -> bool { self.is_withdrawal_enabled && self.balance_stars >= 1000 }

    pub fn topup(&mut self, amount: u64) {
        self.balance_stars += amount;
        self.pending_topup_stars = Some(amount);
        self.transactions.insert(0, StarsTransaction {
            id: format!("tx_{}", self.transactions.len() + 1),
            title: "Stars Top-Up".into(),
            date: "Just now".into(),
            amount: amount as i64,
            is_outgoing: false,
        });
    }

    pub fn credits(&self) -> u64 { self.balance_stars }
    pub fn toggle_visibility(&mut self) { self.is_visible = !self.is_visible; }
}

impl Default for SettingsCreditsSections {
    fn default() -> Self { Self::new() }
}

impl Render for SettingsCreditsSections {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible { return div().into_any_element(); }
        let pal = self.palette.clone();
        let lang = self.language_id.clone();
        let balance = self.balance_stars;
        let txs = self.transactions.clone();

        settings_container(pal.clone())
            .child(settings_section_header(IconName::WalletStars, tr_str(&lang, "Telegram 星币", "Telegram Stars"), Some("Digital goods, channel subscriptions, and creator rewards"), pal.clone()))
            .child(
                settings_card(pal.clone()).child(
                    div().flex().flex_row().items_center().justify_between()
                        .child(div().flex().flex_col().gap(px(4.0))
                            .child(div().text_xs().text_color(rgb(pal.text_secondary)).child(tr_str(&lang, "可用余额", "Available Balance")))
                            .child(div().flex().flex_row().items_center().gap(px(8.0))
                                .child(tg_icon(IconName::PremiumStar, 22.0, pal.accent))
                                .child(div().text_2xl().font_weight(FontWeight::BOLD).text_color(rgb(pal.text_primary)).child(format!("{} {}", balance, self.currency_code)))))
                        .child(div().px(px(14.0)).py(px(8.0)).rounded(px(8.0)).bg(rgb(pal.accent)).text_xs().font_weight(FontWeight::SEMIBOLD).text_color(rgb(pal.active_button_fg)).cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| { this.topup(500); cx.notify(); }))
                            .child(format!("+ {}", tr_str(&lang, "购买星币", "Buy Stars")))),
                ),
            )
            .child(
                settings_card(pal.clone()).child(
                    div().flex().flex_row().items_center().justify_between()
                        .child(div().flex().flex_col()
                            .child(div().text_sm().font_weight(FontWeight::SEMIBOLD).text_color(rgb(pal.text_primary)).child("Creator Rewards (TON)"))
                            .child(div().text_xs().text_color(rgb(pal.text_secondary)).child(format!("{:.2} TON available", self.ton_balance))))
                        .child(div().text_xs().font_weight(FontWeight::SEMIBOLD).text_color(rgb(if self.can_withdraw() { pal.text_link } else { pal.text_secondary })).cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| { if this.can_withdraw() { this.pending_withdraw = true; cx.notify(); } }))
                            .child("Withdraw ›")),
                ),
            )
            .child(settings_subsection_title(tr_str(&lang, "交易记录", "Transaction History"), pal.clone()))
            .child(if txs.is_empty() {
                settings_empty_state(IconName::WalletStars, tr_str(&lang, "暂无星币交易记录", "No Stars transactions yet."), pal).into_any_element()
            } else {
                settings_card(pal.clone()).children(txs.into_iter().map(|tx| {
                    let color = if tx.is_outgoing { pal.danger } else { pal.success };
                    let sign = if tx.amount > 0 { "+" } else { "" };
                    div().flex().flex_row().items_center().justify_between().px(px(10.0)).py(px(6.0))
                        .child(div().flex().flex_col().child(div().text_sm().text_color(rgb(pal.text_primary)).child(tx.title)).child(div().text_xs().text_color(rgb(pal.text_secondary)).child(tx.date)))
                        .child(div().text_sm().font_weight(FontWeight::BOLD).text_color(rgb(color)).child(format!("{}{} ★", sign, tx.amount)))
                })).into_any_element()
            })
            .into_any_element()
    }
}
