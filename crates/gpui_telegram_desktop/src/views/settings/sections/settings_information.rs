//! Upstream Source: Telegram/SourceFiles/settings/sections/settings_information.cpp

use crate::i18n::tr_str;
use crate::theme::assets::IconName;
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::settings::settings_theme_ui::{
    settings_action_row, settings_card, settings_container,
    settings_section_header, settings_subsection_title,
};
use gpui::prelude::FluentBuilder;
use gpui::*;

pub const K_SAVE_BIO_TIMEOUT_MS: u64 = 1000;
pub const K_STANDARD_BIO_LIMIT: usize = 70;
pub const K_PREMIUM_BIO_LIMIT: usize = 140;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileEditField {
    None,
    FirstName,
    LastName,
    Username,
    Phone,
    Bio,
}

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
    pub active_field: ProfileEditField,
    pub pending_save_profile: bool,
    pub pending_logout: bool,
    pub status_message: Option<String>,
    pub is_visible: bool,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
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
            active_field: ProfileEditField::None,
            pending_save_profile: false,
            pending_logout: false,
            status_message: None,
            is_visible: true,
            palette: TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x5288C1, 14, 12),
            language_id: "en".into(),
        }
    }

    pub fn new_live() -> Self {
        let mut s = Self::new();
        s.first_name.clear();
        s.last_name.clear();
        s.bio.clear();
        s.username.clear();
        s.phone.clear();
        s.birthday = None;
        s.is_premium = false;
        s.accounts.clear();
        s
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) {
        self.palette = palette;
    }

    pub fn set_language(&mut self, lang: impl Into<String>) {
        self.language_id = lang.into();
    }

    pub fn sync_from_profile(
        &mut self,
        user_id: u64,
        first_name: String,
        last_name: String,
        username: String,
        phone: String,
        bio: String,
        is_premium: bool,
    ) {
        let full = format!("{} {}", first_name, last_name).trim().to_string();
        self.first_name = first_name;
        self.last_name = last_name;
        self.username = username;
        self.phone = phone.clone();
        self.bio = bio;
        self.is_premium = is_premium;
        self.save_pending = false;
        self.accounts = vec![AccountEntry {
            account_id: user_id,
            name: if full.is_empty() { format!("User #{}", user_id) } else { full },
            phone,
            unread_count: 0,
            is_active: true,
        }];
    }

    pub fn bio_limit(&self) -> usize {
        if self.is_premium { K_PREMIUM_BIO_LIMIT } else { K_STANDARD_BIO_LIMIT }
    }

    pub fn update_bio(&mut self, bio: &str) -> bool {
        if bio.len() > self.bio_limit() { false } else { self.set_bio(bio); true }
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
        self.username = username.into().chars().filter(|c| c.is_alphanumeric() || *c == '_').collect();
        self.save_pending = true;
    }

    pub fn active_field_text(&self) -> Option<String> {
        match self.active_field {
            ProfileEditField::None => None,
            ProfileEditField::FirstName => Some(self.first_name.clone()),
            ProfileEditField::LastName => Some(self.last_name.clone()),
            ProfileEditField::Username => Some(self.username.clone()),
            ProfileEditField::Phone => Some(self.phone.clone()),
            ProfileEditField::Bio => Some(self.bio.clone()),
        }
    }

    pub fn push_field_text(&mut self, text: &str) {
        match self.active_field {
            ProfileEditField::None => {}
            ProfileEditField::FirstName => {
                self.first_name.push_str(text);
                self.save_pending = true;
            }
            ProfileEditField::LastName => {
                self.last_name.push_str(text);
                self.save_pending = true;
            }
            ProfileEditField::Username => {
                for ch in text.chars() {
                    if ch.is_alphanumeric() || ch == '_' {
                        self.username.push(ch);
                    }
                }
                self.save_pending = true;
            }
            ProfileEditField::Phone => {
                for ch in text.chars() {
                    if ch.is_ascii_digit() || ch == '+' || ch == ' ' || ch == '-' {
                        self.phone.push(ch);
                    }
                }
                self.save_pending = true;
            }
            ProfileEditField::Bio => {
                let limit = self.bio_limit();
                for ch in text.chars() {
                    if self.bio.chars().count() < limit {
                        self.bio.push(ch);
                    }
                }
                self.save_pending = true;
            }
        }
    }

    pub fn backspace_field_text(&mut self) {
        match self.active_field {
            ProfileEditField::None => {}
            ProfileEditField::FirstName => { self.first_name.pop(); self.save_pending = true; }
            ProfileEditField::LastName => { self.last_name.pop(); self.save_pending = true; }
            ProfileEditField::Username => { self.username.pop(); self.save_pending = true; }
            ProfileEditField::Phone => { self.phone.pop(); self.save_pending = true; }
            ProfileEditField::Bio => { self.bio.pop(); self.save_pending = true; }
        }
    }

    pub fn clear_field_text(&mut self) {
        match self.active_field {
            ProfileEditField::None => {}
            ProfileEditField::FirstName => { self.first_name.clear(); self.save_pending = true; }
            ProfileEditField::LastName => { self.last_name.clear(); self.save_pending = true; }
            ProfileEditField::Username => { self.username.clear(); self.save_pending = true; }
            ProfileEditField::Phone => { self.phone.clear(); self.save_pending = true; }
            ProfileEditField::Bio => { self.bio.clear(); self.save_pending = true; }
        }
    }

    pub fn cycle_field(&mut self) {
        self.active_field = match self.active_field {
            ProfileEditField::None | ProfileEditField::Bio => ProfileEditField::FirstName,
            ProfileEditField::FirstName => ProfileEditField::LastName,
            ProfileEditField::LastName => ProfileEditField::Username,
            ProfileEditField::Username => ProfileEditField::Phone,
            ProfileEditField::Phone => ProfileEditField::Bio,
        };
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
    fn default() -> Self { Self::new() }
}

impl Render for SettingsInformationSections {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible { return div().into_any_element(); }
        let pal = self.palette;
        let lang = self.language_id.clone();
        let active_field = self.active_field;
        let bio_count = self.bio.chars().count();
        let bio_limit = self.bio_limit();

        let render_edit_box = |field: ProfileEditField,
                               label: &str,
                               value: &str,
                               placeholder: &str,
                               cx: &mut Context<Self>|
         -> AnyElement {
            let is_focused = active_field == field;
            let shown = if value.is_empty() {
                if is_focused { "|".to_string() } else { placeholder.to_string() }
            } else if is_focused {
                format!("{}|", value)
            } else {
                value.to_string()
            };
            div()
                .w_full()
                .flex()
                .flex_col()
                .gap(px(4.0))
                .child(
                    div()
                        .text_size(px(11.5))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(if is_focused { pal.accent } else { pal.text_secondary }))
                        .child(label.to_string()),
                )
                .child(
                    div()
                        .w_full()
                        .px(px(11.0))
                        .py(px(8.0))
                        .rounded(px(8.0))
                        .bg(rgb(pal.input_bg))
                        .border_1()
                        .border_color(rgb(if is_focused { pal.accent } else { pal.border_fg }))
                        .cursor_text()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _e, _w, cx| {
                                this.active_field = field;
                                cx.notify();
                            }),
                        )
                        .child(
                            div()
                                .text_size(px(13.0))
                                .text_color(rgb(if value.is_empty() && !is_focused {
                                    pal.text_secondary
                                } else {
                                    pal.text_primary
                                }))
                                .child(shown),
                        ),
                )
                .into_any_element()
        };

        let first_el = render_edit_box(
            ProfileEditField::FirstName,
            tr_str(&lang, "名字", "First Name"),
            &self.first_name.clone(),
            tr_str(&lang, "输入名字", "Enter first name"),
            cx,
        );
        let last_el = render_edit_box(
            ProfileEditField::LastName,
            tr_str(&lang, "姓氏", "Last Name"),
            &self.last_name.clone(),
            tr_str(&lang, "输入姓氏（可选）", "Enter last name (optional)"),
            cx,
        );
        let user_el = render_edit_box(
            ProfileEditField::Username,
            tr_str(&lang, "用户名 (@username)", "Username (@handle)"),
            &self.username.clone(),
            tr_str(&lang, "输入用户名", "Enter username"),
            cx,
        );
        let phone_el = render_edit_box(
            ProfileEditField::Phone,
            tr_str(&lang, "手机号码", "Phone Number"),
            &self.phone.clone(),
            "+86 138 0000 0000",
            cx,
        );
        let bio_label = format!("{} ({}/{})", tr_str(&lang, "个人简介", "Bio"), bio_count, bio_limit);
        let bio_el = render_edit_box(
            ProfileEditField::Bio,
            &bio_label,
            &self.bio.clone(),
            tr_str(&lang, "填写几句关于你自己的介绍…", "Add a few words about yourself..."),
            cx,
        );

        settings_container(pal)
            .child(
                settings_card(pal)
                    .child(settings_section_header(
                        IconName::ContactsUsers,
                        tr_str(&lang, "编辑个人资料", "Edit Profile"),
                        Some(tr_str(&lang, "点击任意栏位直接修改并实时同步到服务器", "Click any field to edit and save to server")),
                        pal,
                    ))
                    .child(first_el)
                    .child(last_el)
                    .child(user_el)
                    .child(phone_el)
                    .child(bio_el)
                    .when_some(self.status_message.clone(), |el, msg| {
                        el.child(div().text_size(px(12.0)).text_color(rgb(pal.success)).child(msg))
                    })
                    .child(
                        div()
                            .pt(px(4.0))
                            .flex()
                            .justify_end()
                            .child(
                                div()
                                    .id("settings-info-save-btn")
                                    .px(px(16.0))
                                    .py(px(7.0))
                                    .rounded(px(8.0))
                                    .bg(rgb(pal.accent))
                                    .text_size(px(12.5))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(pal.active_button_fg))
                                    .cursor_pointer()
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _e, _w, cx| {
                                            this.pending_save_profile = true;
                                            this.save_pending = false;
                                            cx.notify();
                                        }),
                                    )
                                    .child(tr_str(&lang, "保存修改", "Save Profile")),
                            ),
                    ),
            )
            .when(!self.accounts.is_empty(), |el| {
                el.child(
                    settings_card(pal)
                        .child(settings_subsection_title(tr_str(&lang, "已登录账号", "Accounts"), pal))
                        .children(self.accounts.clone().into_iter().map(|acc| {
                            let id = acc.account_id;
                            settings_action_row(
                                IconName::ContactsUsers,
                                &acc.name,
                                if acc.is_active { tr_str(&lang, "当前在线", "Active") } else { &acc.phone },
                                pal,
                                false,
                                move |this| {
                                    this.switch_account(id);
                                },
                                cx,
                            )
                        })),
                )
            })
            .child(
                settings_card(pal).child(settings_action_row(
                    IconName::Close,
                    tr_str(&lang, "退出当前账号", "Log Out"),
                    "",
                    pal,
                    true,
                    |this| {
                        this.pending_logout = true;
                    },
                    cx,
                )),
            )
            .into_any_element()
    }
}
