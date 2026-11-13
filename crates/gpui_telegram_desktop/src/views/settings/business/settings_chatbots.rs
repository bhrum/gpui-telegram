//! Upstream Source: Telegram/SourceFiles/settings/business/settings_chatbots.cpp, Telegram/SourceFiles/settings/business/settings_chatbots.h
//! Subsystem: settings | Feature ID: TCG-N27 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui::prelude::FluentBuilder;

pub const K_DEBOUNCE_TIMEOUT_MS: u64 = 400;
pub const K_MAX_SEARCH_BOTS: usize = 3;

pub const PERM_VIEW_MESSAGES: u32 = 1 << 0;
pub const PERM_REPLY_MESSAGES: u32 = 1 << 1;
pub const PERM_TRANSFER_GIFTS: u32 = 1 << 2;
pub const PERM_SELL_GIFTS: u32 = 1 << 3;
pub const PERM_TRANSFER_STARS: u32 = 1 << 4;
pub const PERM_EDIT_USERNAME: u32 = 1 << 5;

#[derive(Debug, Clone)]
pub struct ConnectedBotInfo {
    pub id: u64,
    pub name: String,
    pub username: String,
}

#[derive(Debug, Clone)]
pub struct SettingsChatbotsBusiness {
    pub bot: Option<ConnectedBotInfo>,
    pub permissions: u32,
    pub search_query: String,
    pub warning_alert: Option<String>,
    pub status_message: Option<String>,
}

impl SettingsChatbotsBusiness {
    pub fn new() -> Self {
        Self {
            bot: None,
            permissions: PERM_VIEW_MESSAGES,
            search_query: String::new(),
            warning_alert: None,
            status_message: None,
        }
    }

    pub fn set_search_query(&mut self, text: String) {
        let clean = text.trim();
        let stripped = clean.strip_prefix('@').unwrap_or(clean);
        self.search_query = stripped.to_string();
    }

    pub fn add_bot(&mut self, id: u64, name: String, username: String) {
        self.bot = Some(ConnectedBotInfo { id, name, username });
        self.search_query.clear();
        self.status_message = Some("Bot connected".into());
    }

    pub fn remove_bot(&mut self) {
        self.bot = None;
        self.permissions = PERM_VIEW_MESSAGES;
        self.warning_alert = None;
        self.status_message = Some("Bot removed".into());
    }

    pub fn has_permission(&self, flag: u32) -> bool {
        (self.permissions & flag) != 0
    }

    pub fn toggle_permission(&mut self, flag: u32) {
        self.permissions ^= flag;
        if self.has_permission(flag) {
            if (flag & (PERM_TRANSFER_GIFTS | PERM_SELL_GIFTS)) != 0 && self.has_permission(PERM_TRANSFER_STARS) {
                self.warning_alert = Some("This bot can transfer gifts and spend Telegram Stars on your behalf.".into());
            } else if (flag & (PERM_TRANSFER_GIFTS | PERM_SELL_GIFTS)) != 0 {
                self.warning_alert = Some("This bot will be allowed to transfer or sell your collectible gifts.".into());
            } else if flag == PERM_TRANSFER_STARS {
                self.warning_alert = Some("This bot will be allowed to transfer Telegram Stars from your balance.".into());
            } else if flag == PERM_EDIT_USERNAME {
                self.warning_alert = Some("This bot will be allowed to modify your account username.".into());
            } else {
                self.warning_alert = None;
            }
        } else {
            self.warning_alert = None;
        }
    }

    pub fn dismiss_warning(&mut self) {
        self.warning_alert = None;
    }
}

impl Default for SettingsChatbotsBusiness {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsChatbotsBusiness {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let has_bot = self.bot.is_some();
        let current_bot = self.bot.clone();
        let perms = self.permissions;
        let warning = self.warning_alert.clone();

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .bg(rgb(0x0e1621))
            .gap(px(12.0))
            // Header Info
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .p(px(16.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x17212b))
                    .child(div().text_sm().font_weight(FontWeight::BOLD).text_color(rgb(0xffffff)).child("Chat Automation & Bots"))
                    .child(div().text_xs().text_color(rgb(0x8292a1)).child("Connect a business bot to automate client responses.")),
            )
            // Warning Alert banner
            .when_some(warning, |el, msg| {
                el.child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .p(px(12.0))
                        .rounded(px(8.0))
                        .bg(rgb(0x382a17))
                        .child(div().text_xs().text_color(rgb(0xf2a33a)).child(msg))
                        .child(
                            div()
                                .cursor_pointer()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xf2a33a))
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                    this.dismiss_warning();
                                    cx.notify();
                                }))
                                .child("Dismiss"),
                        ),
                )
            })
            // Connected bot card or search entry
            .child(match current_bot {
                Some(b) => {
                    div()
                        .flex()
                        .flex_col()
                        .p(px(12.0))
                        .rounded(px(8.0))
                        .bg(rgb(0x17212b))
                        .gap(px(8.0))
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .child(div().text_sm().font_weight(FontWeight::BOLD).text_color(rgb(0xffffff)).child(b.name))
                                        .child(div().text_xs().text_color(rgb(0x5288c1)).child(format!("@{}", b.username))),
                                )
                                .child(
                                    div()
                                        .cursor_pointer()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0xe05555))
                                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                            this.remove_bot();
                                            cx.notify();
                                        }))
                                        .child("Remove Bot"),
                                ),
                        )
                }
                None => {
                    div()
                        .flex()
                        .flex_col()
                        .p(px(12.0))
                        .rounded(px(8.0))
                        .bg(rgb(0x17212b))
                        .gap(px(6.0))
                        .child(div().text_xs().text_color(rgb(0x8292a1)).child("Enter Bot Username"))
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .justify_between()
                                .items_center()
                                .child(div().text_sm().text_color(rgb(0xffffff)).child(if self.search_query.is_empty() { "@bot_username".into() } else { format!("@{}", self.search_query) }))
                                .child(
                                    div()
                                        .cursor_pointer()
                                        .px(px(8.0))
                                        .py(px(4.0))
                                        .rounded(px(4.0))
                                        .bg(rgb(0x5288c1))
                                        .text_xs()
                                        .text_color(rgb(0xffffff))
                                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                            this.add_bot(9001, "Sales Assistant".into(), "sales_bot".into());
                                            cx.notify();
                                        }))
                                        .child("Connect Demo Bot"),
                                ),
                        )
                }
            })
            // Permissions toggles when bot is active
            .when(has_bot, |container| {
                container.child(
                    div()
                        .flex()
                        .flex_col()
                        .p(px(12.0))
                        .rounded(px(8.0))
                        .bg(rgb(0x17212b))
                        .gap(px(8.0))
                        .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(rgb(0x8292a1)).child("PERMISSIONS"))
                        .child(make_perm_row("View Messages", (perms & PERM_VIEW_MESSAGES) != 0, PERM_VIEW_MESSAGES, cx))
                        .child(make_perm_row("Reply to Messages", (perms & PERM_REPLY_MESSAGES) != 0, PERM_REPLY_MESSAGES, cx))
                        .child(make_perm_row("Transfer Gifts", (perms & PERM_TRANSFER_GIFTS) != 0, PERM_TRANSFER_GIFTS, cx))
                        .child(make_perm_row("Sell Gifts", (perms & PERM_SELL_GIFTS) != 0, PERM_SELL_GIFTS, cx))
                        .child(make_perm_row("Transfer Telegram Stars", (perms & PERM_TRANSFER_STARS) != 0, PERM_TRANSFER_STARS, cx))
                        .child(make_perm_row("Edit Username", (perms & PERM_EDIT_USERNAME) != 0, PERM_EDIT_USERNAME, cx)),
                )
            })
            .into_any_element()
    }
}

fn make_perm_row(
    label: &'static str,
    active: bool,
    flag: u32,
    cx: &mut Context<SettingsChatbotsBusiness>,
) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .py(px(4.0))
        .cursor_pointer()
        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _window, cx| {
            this.toggle_permission(flag);
            cx.notify();
        }))
        .child(div().text_sm().text_color(rgb(0xffffff)).child(label))
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .text_color(if active { rgb(0x5288c1) } else { rgb(0x8292a1) })
                .child(if active { "ALLOWED" } else { "DISALLOWED" }),
        )
}
