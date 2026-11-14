use crate::i18n::tr_str;
use crate::theme::assets::IconName;
use crate::theme::palette::TdesktopThemePalette;
use crate::views::settings::settings_theme_ui::{
    settings_action_row, settings_card, settings_empty_state, settings_info_row,
    settings_section_header, settings_subsection_title, settings_toggle_row,
};
use crate::views::settings_workspace::{SettingsMutation, SettingsWorkspaceView};
use gpui::prelude::FluentBuilder;
use gpui::*;

pub fn render_premium_stars_page(
    view: &SettingsWorkspaceView,
    pal: TdesktopThemePalette,
    lang: &str,
    cx: &mut Context<SettingsWorkspaceView>,
) -> AnyElement {
    div().flex().flex_col().gap(px(12.0))
        .child(settings_card(pal)
            .child(settings_section_header(IconName::PremiumStar, tr_str(lang, "Telegram Premium 与星币", "Telegram Premium & Stars"), None, pal))
            .child(settings_info_row(tr_str(lang, "当前 Stars 余额", "Stars Balance"), &format!("{} Stars", view.state_snapshot.stars_balance), pal))
            .child(settings_action_row(IconName::WalletStars, tr_str(lang, "充值 100 Stars", "Top Up 100 Stars"), "+100", pal, false, |this| {
                this.pending_mutation = Some(SettingsMutation::BuyOrTopUpStars(100));
            }, cx))
            .child(settings_action_row(IconName::WalletStars, tr_str(lang, "充值 500 Stars", "Top Up 500 Stars"), "+500", pal, false, |this| {
                this.pending_mutation = Some(SettingsMutation::BuyOrTopUpStars(500));
            }, cx)))
        .into_any_element()
}

pub fn render_ton_wallet_page(
    view: &SettingsWorkspaceView,
    pal: TdesktopThemePalette,
    lang: &str,
    cx: &mut Context<SettingsWorkspaceView>,
) -> AnyElement {
    let txs = view.state_snapshot.wallet_transactions.clone();
    div().flex().flex_col().gap(px(12.0))
        .child(settings_card(pal)
            .child(settings_section_header(IconName::WalletStars, tr_str(lang, "TON 钱包", "TON Wallet"), None, pal))
            .child(settings_info_row(tr_str(lang, "TON 余额", "TON Balance"), &format!("{:.2} TON", view.state_snapshot.ton_balance), pal))
            .child(settings_action_row(IconName::SendPlane, tr_str(lang, "发送 TON", "Send TON"), "", pal, false, |this| {
                this.pending_mutation = Some(SettingsMutation::SendTonTransfer);
            }, cx)))
        .child(settings_card(pal)
            .child(settings_subsection_title(tr_str(lang, "近期交易记录", "Recent Transactions"), pal))
            .map(|el| {
                if txs.is_empty() {
                    el.child(settings_empty_state(IconName::WalletStars, tr_str(lang, "暂无钱包交易", "No transactions yet"), pal))
                } else {
                    el.children(txs.into_iter().map(|tx| {
                        settings_info_row(&tx.title, &format!("{:.2} TON", tx.amount_ton), pal)
                    }))
                }
            }))
        .into_any_element()
}

pub fn render_business_passport_page(
    view: &SettingsWorkspaceView,
    pal: TdesktopThemePalette,
    lang: &str,
    cx: &mut Context<SettingsWorkspaceView>,
) -> AnyElement {
    div().flex().flex_col().gap(px(12.0))
        .child(settings_card(pal)
            .child(settings_section_header(IconName::BotRobot, tr_str(lang, "商业版与 Passport", "Telegram Business & Passport"), None, pal))
            .child(settings_toggle_row(IconName::ClockSending, tr_str(lang, "展示营业时间", "Business Opening Hours"), None, view.business_hours_enabled, pal, |this| {
                this.business_hours_enabled = !this.business_hours_enabled;
            }, cx))
            .child(settings_info_row(tr_str(lang, "快捷回复模板", "Quick Replies"), &format!("{}", view.business_quick_replies), pal))
            .child(settings_info_row(tr_str(lang, "Telegram Passport 已验证证件", "Verified Passport Documents"), &format!("{}", view.passport_verified_docs), pal)))
        .into_any_element()
}
