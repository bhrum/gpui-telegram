//! Authentic Telegram Desktop 280px Left Slide-Over Hamburger Main Menu Drawer.
//! Upstream anchor: `Telegram/SourceFiles/window/window_main_menu.cpp` & `window_main_menu.style`.
//!
//! Features:
//! - Current Account Usercard with Avatar, Verified/Premium Star, Emoji Status Picker
//! - Expandable Multi-Account Switcher (up to 3 standard / 6 Premium accounts + Add Account)
//! - My Profile, New Group, New Channel, Contacts, Calls, Saved Messages, Settings
//! - Interactive Night Mode Toggle Switch
//! - TON Wallet & Telegram Stars, Export Chat Data, Keyboard Shortcuts, About

use crate::views::tdesktop_theme::tr_str;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_telegram_client::AccountSessionRecord;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MainMenuCommand {
    SwitchAccount(u32),
    AddAccount,
    SetEmojiStatus(String),
    MyProfile,
    NewGroup,
    NewChannel,
    Contacts,
    Calls,
    SavedMessages,
    Settings,
    ToggleNightMode,
    WalletAndStars,
    ExportData,
    ShortcutsLegend,
    AboutTelegram,
}

pub struct MainMenuDrawer {
    pub is_open: bool,
    pub accounts_expanded: bool,
    pub emoji_status_picker_open: bool,
    pub accounts: Vec<AccountSessionRecord>,
    pub active_account_id: u32,
    pub is_night_mode: bool,
    pub language_id: String,
    pub pending_command: Option<MainMenuCommand>,
}

impl MainMenuDrawer {
    pub fn new(accounts: Vec<AccountSessionRecord>, active_account_id: u32) -> Self {
        Self {
            is_open: false,
            accounts_expanded: false,
            emoji_status_picker_open: false,
            accounts,
            active_account_id,
            is_night_mode: true,
            language_id: "zh-CN".to_string(),
            pending_command: None,
        }
    }

    pub fn active_account(&self) -> AccountSessionRecord {
        self.accounts
            .iter()
            .find(|a| a.account_id == self.active_account_id)
            .cloned()
            .unwrap_or_else(|| AccountSessionRecord {
                account_id: 1,
                display_name: "Telegram User".to_string(),
                username: "user".to_string(),
                phone: "+1 555 0100".to_string(),
                bio: String::new(),
                emoji_status: "⭐".to_string(),
                is_premium: true,
                is_verified: false,
                avatar_initials: "TU".to_string(),
                avatar_color: 0x2AABEE,
                unread_chats: 0,
            })
    }
}

impl Render for MainMenuDrawer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_open {
            return div()
                .absolute()
                .w(px(0.0))
                .h(px(0.0))
                .overflow_hidden()
                .into_any_element();
        }

        let active = self.active_account();
        let accounts_list = self.accounts.clone();
        let expanded = self.accounts_expanded;
        let emoji_open = self.emoji_status_picker_open;
        let night = self.is_night_mode;
        let lang = self.language_id.clone();

        let drawer_bg = if night { rgb(0x17212B) } else { rgb(0xFFFFFF) };
        let header_bg = if night { rgb(0x202B36) } else { rgb(0x5288C1) };
        let row_hover = if night { rgb(0x232E3C) } else { rgb(0xF1F3F4) };
        let fg_primary = if night { rgb(0xFFFFFF) } else { rgb(0x1C242B) };
        let fg_muted = if night { rgb(0x8A9AA8) } else { rgb(0x707579) };
        let divider_col = if night { rgb(0x0E1621) } else { rgb(0xE0E0E0) };

        let menu_items: Vec<(&str, &str, &str, MainMenuCommand)> = vec![
            ("👤", "我的个人资料", "My Profile", MainMenuCommand::MyProfile),
            ("👥", "新建群组", "New Group", MainMenuCommand::NewGroup),
            ("📢", "新建频道", "New Channel", MainMenuCommand::NewChannel),
            ("📒", "联系人", "Contacts", MainMenuCommand::Contacts),
            ("📞", "通话记录", "Calls", MainMenuCommand::Calls),
            ("🔖", "收藏夹 (Saved Messages)", "Saved Messages", MainMenuCommand::SavedMessages),
            ("💎", "TON 钱包与 Stars 星币", "TON Wallet & Stars", MainMenuCommand::WalletAndStars),
            ("📤", "导出聊天数据", "Export Telegram Data", MainMenuCommand::ExportData),
            ("⚙️", "设置 (Settings)", "Settings", MainMenuCommand::Settings),
            ("⌨️", "键盘快捷键指南", "Keyboard Shortcuts", MainMenuCommand::ShortcutsLegend),
            ("ℹ️", "关于 Telegram GPUI", "About Telegram Desktop", MainMenuCommand::AboutTelegram),
        ];

        div()
            .id("window-main-menu-backdrop")
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .occlude()
            .bg(rgba(0x00000088))
            .flex()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _event, _window, cx| {
                    this.is_open = false;
                    this.emoji_status_picker_open = false;
                    cx.notify();
                }),
            )
            .child(
                div()
                    .id("window-main-menu-panel")
                    .w(px(284.0))
                    .h_full()
                    .bg(drawer_bg)
                    .shadow_lg()
                    .flex()
                    .flex_col()
                    .justify_between()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|_this, _event, _window, _cx| {
                            // Consume click inside drawer so backdrop doesn't close it prematurely
                        }),
                    )
                    .child(
                        div()
                            .id("window-main-menu-scroll")
                            .flex_1()
                            .min_h(px(0.0))
                            .overflow_y_scroll()
                            .flex()
                            .flex_col()
                            // 1. User Profile Card Header
                            .child(
                                div()
                                    .w_full()
                                    .p(px(16.0))
                                    .bg(header_bg)
                                    .flex()
                                    .flex_col()
                                    .gap(px(10.0))
                                    .child(
                                        div()
                                            .w_full()
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .child(
                                                div()
                                                    .w(px(52.0))
                                                    .h(px(52.0))
                                                    .rounded_full()
                                                    .bg(rgb(active.avatar_color))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .text_size(px(18.0))
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(0xFFFFFF))
                                                    .child(active.avatar_initials.clone()),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap(px(8.0))
                                                    // Emoji Status Pill
                                                    .child(
                                                        div()
                                                            .cursor_pointer()
                                                            .px(px(8.0))
                                                            .py(px(4.0))
                                                            .rounded_full()
                                                            .bg(rgba(0x00000033))
                                                            .hover(|s| s.bg(rgba(0x00000055)))
                                                            .text_size(px(13.0))
                                                            .text_color(rgb(0xFFFFFF))
                                                            .on_mouse_down(
                                                                MouseButton::Left,
                                                                cx.listener(|this, _e, _w, cx| {
                                                                    this.emoji_status_picker_open =
                                                                        !this.emoji_status_picker_open;
                                                                    cx.notify();
                                                                }),
                                                            )
                                                            .child(format!("{} 状态", active.emoji_status)),
                                                    )
                                                    // Close Drawer Button
                                                    .child(
                                                        div()
                                                            .cursor_pointer()
                                                            .w(px(28.0))
                                                            .h(px(28.0))
                                                            .rounded_full()
                                                            .bg(rgba(0x00000033))
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .text_size(px(12.0))
                                                            .text_color(rgb(0xFFFFFF))
                                                            .on_mouse_down(
                                                                MouseButton::Left,
                                                                cx.listener(|this, _e, _w, cx| {
                                                                    this.is_open = false;
                                                                    cx.notify();
                                                                }),
                                                            )
                                                            .child("✕"),
                                                    ),
                                            ),
                                    )
                                    // Emoji Status Picker Popover Strip
                                    .when(emoji_open, |el| {
                                        el.child(
                                            div()
                                                .w_full()
                                                .p(px(6.0))
                                                .rounded(px(8.0))
                                                .bg(rgba(0x00000044))
                                                .flex()
                                                .items_center()
                                                .justify_between()
                                                .children(
                                                    ["🦀", "🚀", "💻", "☕", "🪷", "🔥", "🎧", "⭐"]
                                                        .into_iter()
                                                        .map(|em| {
                                                            let em_s = em.to_string();
                                                            div()
                                                                .cursor_pointer()
                                                                .w(px(28.0))
                                                                .h(px(28.0))
                                                                .rounded(px(6.0))
                                                                .hover(|s| s.bg(rgb(0x2AABEE)))
                                                                .flex()
                                                                .items_center()
                                                                .justify_center()
                                                                .text_size(px(15.0))
                                                                .on_mouse_down(
                                                                    MouseButton::Left,
                                                                    cx.listener(move |this, _e, _w, cx| {
                                                                        this.emoji_status_picker_open = false;
                                                                        this.pending_command = Some(
                                                                            MainMenuCommand::SetEmojiStatus(
                                                                                em_s.clone(),
                                                                            ),
                                                                        );
                                                                        cx.notify();
                                                                    }),
                                                                )
                                                                .child(em)
                                                        }),
                                                ),
                                        )
                                    })
                                    // Name + Phone + Multi-Account Chevron
                                    .child(
                                        div()
                                            .cursor_pointer()
                                            .w_full()
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(|this, _e, _w, cx| {
                                                    this.accounts_expanded = !this.accounts_expanded;
                                                    cx.notify();
                                                }),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .child(
                                                        div()
                                                            .text_size(px(14.0))
                                                            .font_weight(FontWeight::BOLD)
                                                            .text_color(rgb(0xFFFFFF))
                                                            .child(format!(
                                                                "{} {} {}",
                                                                active.display_name,
                                                                active.emoji_status,
                                                                if active.is_premium { "⭐" } else { "" }
                                                            )),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(12.0))
                                                            .text_color(rgb(0xB8C7D4))
                                                            .child(format!(
                                                                "{} • @{}",
                                                                active.phone, active.username
                                                            )),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(14.0))
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(0xFFFFFF))
                                                    .child(if expanded { "▲" } else { "▼" }),
                                            ),
                                    ),
                            )
                            // 2. Expandable Multi-Account Switcher (up to 6 Premium accounts)
                            .when(expanded, |el| {
                                el.child(
                                    div()
                                        .w_full()
                                        .py(px(6.0))
                                        .border_b_1()
                                        .border_color(divider_col)
                                        .flex()
                                        .flex_col()
                                        .children(accounts_list.into_iter().map(|acc| {
                                            let is_cur = acc.account_id == active.account_id;
                                            let acc_id = acc.account_id;
                                            div()
                                                .cursor_pointer()
                                                .h(px(42.0))
                                                .px(px(16.0))
                                                .hover(move |s| s.bg(row_hover))
                                                .flex()
                                                .items_center()
                                                .justify_between()
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(move |this, _e, _w, cx| {
                                                        this.pending_command =
                                                            Some(MainMenuCommand::SwitchAccount(acc_id));
                                                        this.is_open = false;
                                                        cx.notify();
                                                    }),
                                                )
                                                .child(
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap(px(10.0))
                                                        .child(
                                                            div()
                                                                .w(px(28.0))
                                                                .h(px(28.0))
                                                                .rounded_full()
                                                                .bg(rgb(acc.avatar_color))
                                                                .flex()
                                                                .items_center()
                                                                .justify_center()
                                                                .text_size(px(11.0))
                                                                .font_weight(FontWeight::BOLD)
                                                                .text_color(rgb(0xFFFFFF))
                                                                .child(acc.avatar_initials),
                                                        )
                                                        .child(
                                                            div()
                                                                .text_size(px(13.0))
                                                                .font_weight(if is_cur {
                                                                    FontWeight::BOLD
                                                                } else {
                                                                    FontWeight::NORMAL
                                                                })
                                                                .text_color(fg_primary)
                                                                .child(format!(
                                                                    "{} {}",
                                                                    acc.display_name, acc.emoji_status
                                                                )),
                                                        ),
                                                )
                                                .child(
                                                    div()
                                                        .text_size(px(12.0))
                                                        .text_color(rgb(0x2AABEE))
                                                        .child(if is_cur {
                                                            "✓".to_string()
                                                        } else if acc.unread_chats > 0 {
                                                            format!("{}", acc.unread_chats)
                                                        } else {
                                                            String::new()
                                                        }),
                                                )
                                        }))
                                        .child(
                                            div()
                                                .cursor_pointer()
                                                .h(px(38.0))
                                                .px(px(16.0))
                                                .hover(move |s| s.bg(row_hover))
                                                .flex()
                                                .items_center()
                                                .gap(px(10.0))
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(|this, _e, _w, cx| {
                                                        this.pending_command =
                                                            Some(MainMenuCommand::AddAccount);
                                                        this.is_open = false;
                                                        cx.notify();
                                                    }),
                                                )
                                                .child(
                                                    div()
                                                        .text_size(px(15.0))
                                                        .text_color(rgb(0x2AABEE))
                                                        .child("＋"),
                                                )
                                                .child(
                                                    div()
                                                        .text_size(px(13.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(rgb(0x2AABEE))
                                                        .child(tr_str(
                                                            &lang,
                                                            "添加账号 (最多支持 6 个)",
                                                            "Add Account (Up to 6)",
                                                        )),
                                                ),
                                        ),
                                )
                            })
                            // 3. Main Menu Action Rows
                            .child(
                                div()
                                    .w_full()
                                    .py(px(6.0))
                                    .flex()
                                    .flex_col()
                                    .children(menu_items.into_iter().map(|(icon, zh_l, en_l, cmd)| {
                                        let label = tr_str(&lang, zh_l, en_l).to_string();
                                        div()
                                            .cursor_pointer()
                                            .h(px(40.0))
                                            .px(px(18.0))
                                            .hover(move |s| s.bg(row_hover))
                                            .flex()
                                            .items_center()
                                            .gap(px(14.0))
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(move |this, _e, _w, cx| {
                                                    this.pending_command = Some(cmd.clone());
                                                    this.is_open = false;
                                                    cx.notify();
                                                }),
                                            )
                                            .child(div().text_size(px(16.0)).child(icon))
                                            .child(
                                                div()
                                                    .text_size(px(13.5))
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(fg_primary)
                                                    .child(label),
                                            )
                                    }))
                                    // Night Mode Toggle Row
                                    .child(
                                        div()
                                            .cursor_pointer()
                                            .h(px(42.0))
                                            .px(px(18.0))
                                            .border_t_1()
                                            .border_color(divider_col)
                                            .hover(move |s| s.bg(row_hover))
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(|this, _e, _w, cx| {
                                                    this.is_night_mode = !this.is_night_mode;
                                                    this.pending_command =
                                                        Some(MainMenuCommand::ToggleNightMode);
                                                    cx.notify();
                                                }),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap(px(14.0))
                                                    .child(div().text_size(px(16.0)).child("🌙"))
                                                    .child(
                                                        div()
                                                            .text_size(px(13.5))
                                                            .font_weight(FontWeight::MEDIUM)
                                                            .text_color(fg_primary)
                                                            .child(tr_str(
                                                                &lang,
                                                                "夜间模式 (Night Mode)",
                                                                "Night Mode",
                                                            )),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .px(px(8.0))
                                                    .py(px(2.0))
                                                    .rounded_full()
                                                    .bg(if night {
                                                        rgb(0x2AABEE)
                                                    } else {
                                                        rgb(0x9E9E9E)
                                                    })
                                                    .text_size(px(11.0))
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(0xFFFFFF))
                                                    .child(if night { "ON" } else { "OFF" }),
                                            ),
                                    ),
                            ),
                    )
                    // 4. Footer Version Stamp
                    .child(
                        div()
                            .w_full()
                            .px(px(18.0))
                            .py(px(10.0))
                            .border_t_1()
                            .border_color(divider_col)
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(fg_muted)
                                    .child("Telegram Desktop GPUI"),
                            )
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(fg_muted)
                                    .child("Version 7.4.2 • Metal/Vulkan 120fps • MTProto 2.0"),
                            ),
                    ),
            )
            .into_any_element()
    }
}
