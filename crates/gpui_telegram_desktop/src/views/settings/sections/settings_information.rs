//! Upstream Source: Telegram/SourceFiles/settings/sections/settings_information.cpp, Telegram/SourceFiles/settings/sections/settings_information.h
//! Subsystem: settings | Feature ID: TCG-N31 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui::prelude::FluentBuilder;

pub const K_SAVE_BIO_TIMEOUT_MS: u64 = 1000;
pub const K_STANDARD_BIO_LIMIT: usize = 70;
pub const K_PREMIUM_BIO_LIMIT: usize = 140;

#[derive(Debug, Clone)]
pub struct AccountEntry {
    pub account_id: u64,
    pub name: String,
    pub phone: String,
    pub unread_count: usize,
    pub is_active: bool,
}

#[derive(Debug, Clone)]
pub struct SettingsInformationSections {
    pub first_name: String,
    pub last_name: String,
    pub bio: String,
    pub username: String,
    pub phone: String,
    pub birthday: Option<String>,
    pub is_premium: bool,
    pub accounts: Vec<AccountEntry>,
    pub save_pending: bool,
    pub is_visible: bool,
}

impl SettingsInformationSections {
    pub fn new() -> Self {
        Self {
            first_name: "Pavel".into(),
            last_name: "Durov".into(),
            bio: "Exploring GPUI and high-performance Rust interfaces".into(),
            username: "durov".into(),
            phone: "+1 234 567 8900".into(),
            birthday: Some("October 10".into()),
            is_premium: true,
            accounts: vec![
                AccountEntry { account_id: 1, name: "Pavel Durov".into(), phone: "+1 234 567 8900".into(), unread_count: 0, is_active: true },
                AccountEntry { account_id: 2, name: "Work Account".into(), phone: "+1 415 555 0138".into(), unread_count: 4, is_active: false },
            ],
            save_pending: false,
            is_visible: true,
        }
    }

    pub fn bio_limit(&self) -> usize {
        if self.is_premium {
            K_PREMIUM_BIO_LIMIT
        } else {
            K_STANDARD_BIO_LIMIT
        }
    }

    pub fn update_bio(&mut self, bio: &str) -> bool {
        if bio.len() > self.bio_limit() {
            false
        } else {
            self.set_bio(bio);
            true
        }
    }

    pub fn set_bio(&mut self, bio: impl Into<String>) {
        let b = bio.into();
        let limit = self.bio_limit();
        self.bio = b.chars().take(limit).collect();
        self.save_pending = true;
    }

    pub fn set_names(&mut self, first: impl Into<String>, last: impl Into<String>) {
        self.first_name = first.into();
        self.last_name = last.into();
        self.save_pending = true;
    }

    pub fn set_username(&mut self, username: impl Into<String>) {
        let u: String = username.into().chars().filter(|c| c.is_alphanumeric() || *c == '_').collect();
        self.username = u;
        self.save_pending = true;
    }

    pub fn switch_account(&mut self, account_id: u64) {
        for acc in &mut self.accounts {
            acc.is_active = acc.account_id == account_id;
        }
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }
}

impl Default for SettingsInformationSections {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsInformationSections {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }

        let bio_len = self.bio.chars().count();
        let bio_limit = self.bio_limit();

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .bg(rgb(0x0e1621))
            .child(
                // Section Title & Avatar Photo Upload trigger
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(16.0))
                    .p(px(16.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x17212b))
                    .child(
                        div()
                            .size(px(56.0))
                            .rounded(px(28.0))
                            .bg(rgb(0x5288c1))
                            .flex()
                            .items_center()
                            .justify_center()
                            .cursor_pointer()
                            .hover(|s| s.bg(rgb(0x4277b0)))
                            .child("📷"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xffffff))
                                    .child(format!("{} {}", self.first_name, self.last_name)),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x5288c1))
                                    .cursor_pointer()
                                    .child("Set Profile Photo"),
                            ),
                    ),
            )
            .child(
                // Editable Profile Fields Form
                div()
                    .mt(px(12.0))
                    .flex()
                    .flex_col()
                    .gap(px(1.0))
                    .rounded(px(8.0))
                    .overflow_hidden()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .px(px(16.0))
                            .py(px(10.0))
                            .bg(rgb(0x17212b))
                            .child(div().text_xs().text_color(rgb(0x8292a1)).child("First Name"))
                            .child(div().text_sm().text_color(rgb(0xffffff)).child(self.first_name.clone())),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .px(px(16.0))
                            .py(px(10.0))
                            .bg(rgb(0x17212b))
                            .child(div().text_xs().text_color(rgb(0x8292a1)).child("Last Name"))
                            .child(div().text_sm().text_color(rgb(0xffffff)).child(self.last_name.clone())),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .px(px(16.0))
                            .py(px(10.0))
                            .bg(rgb(0x17212b))
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .justify_between()
                                    .child(div().text_xs().text_color(rgb(0x8292a1)).child("Bio"))
                                    .child(div().text_xs().text_color(rgb(0x6c7883)).child(format!("{}/{}", bio_len, bio_limit))),
                            )
                            .child(div().text_sm().text_color(rgb(0xffffff)).child(self.bio.clone())),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .px(px(16.0))
                            .py(px(10.0))
                            .bg(rgb(0x17212b))
                            .child(div().text_xs().text_color(rgb(0x8292a1)).child("Username"))
                            .child(div().text_sm().text_color(rgb(0x5288c1)).child(format!("@{}", self.username))),
                    ),
            )
            .child(
                // Multi-Account Switcher Section
                div()
                    .mt(px(16.0))
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(rgb(0x5288c1)).child("CONNECTED ACCOUNTS"))
                    .children(self.accounts.iter().map(|acc| {
                        let acc_id = acc.account_id;
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .p(px(12.0))
                            .rounded(px(8.0))
                            .bg(if acc.is_active { rgb(0x232e3c) } else { rgb(0x17212b) })
                            .hover(|s| s.bg(rgb(0x2b3846)))
                            .cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _window, cx| {
                                this.switch_account(acc_id);
                                cx.notify();
                            }))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(div().text_sm().font_weight(FontWeight::MEDIUM).text_color(rgb(0xffffff)).child(acc.name.clone()))
                                    .child(div().text_xs().text_color(rgb(0x8292a1)).child(acc.phone.clone())),
                            )
                            .when(acc.unread_count > 0, |this| {
                                this.child(
                                    div()
                                        .px(px(6.0))
                                        .py(px(2.0))
                                        .rounded(px(10.0))
                                        .bg(rgb(0x5288c1))
                                        .text_xs()
                                        .text_color(rgb(0xffffff))
                                        .child(acc.unread_count.to_string()),
                                )
                            })
                    })),
            )
            .into_any_element()
    }
}
