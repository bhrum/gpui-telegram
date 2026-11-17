//! Authentic Telegram Desktop 280px Left Slide-Over Hamburger Main Menu Drawer.
//! Upstream anchor: `Telegram/SourceFiles/window/window_main_menu.cpp` & `window_main_menu.style`.
//!
//! Features:
//! - Current Account Usercard with Avatar, Verified/Premium Star, Emoji Status Picker
//! - Expandable Multi-Account Switcher (up to 3 standard / 6 Premium accounts + Add Account)
//! - My Profile, New Group, New Channel, Contacts, Calls, Saved Messages, Settings
//! - Interactive Night Mode Toggle Switch
//! - TON Wallet & Telegram Stars, Export Chat Data, Keyboard Shortcuts, About

use crate::i18n::tr_str;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_telegram_client::AccountSessionRecord;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MainMenuCommand {
    SwitchAccount(u32),
    AddAccount,
    SetEmojiStatus(String),
    MyProfile,
    NewFabushiBot,
    NewGroup,
    NewChannel,
    Contacts,
    Calls,
    SavedMessages,
    TaskCenter,
    McpTools,
    RemoteComputer,
    Settings,
    ToggleNightMode,
    WalletAndStars,
    ExportData,
    ShortcutsLegend,
    AboutTelegram,
    Logout,
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
    pub pending_botfather_open: bool,
    pub pending_miniapps_center: bool,
    pub available_update_version: Option<String>,
    pub pending_trigger_update: bool,
    pub palette: TdesktopThemePalette,
}

impl MainMenuDrawer {
    pub fn new(accounts: Vec<AccountSessionRecord>, active_account_id: u32) -> Self {
        let palette = TdesktopThemePalette::default();
        Self {
            is_open: false,
            accounts_expanded: false,
            emoji_status_picker_open: false,
            accounts,
            active_account_id,
            is_night_mode: palette.mode.is_dark(),
            language_id: "en".to_string(),
            pending_command: None,
            pending_botfather_open: false,
            pending_miniapps_center: false,
            available_update_version: None,
            pending_trigger_update: false,
            palette,
        }
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) {
        self.is_night_mode = palette.mode.is_dark();
        self.palette = palette;
    }

    pub fn set_language(&mut self, lang_id: impl Into<String>) {
        self.language_id = lang_id.into();
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.emoji_status_picker_open = false;
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
        if !self.is_open {
            self.emoji_status_picker_open = false;
        }
    }

    pub fn take_command(&mut self) -> Option<MainMenuCommand> {
        self.pending_command.take()
    }

    pub fn take_botfather_open(&mut self) -> bool {
        let v = self.pending_botfather_open;
        self.pending_botfather_open = false;
        v
    }

    pub fn take_miniapps_center(&mut self) -> bool {
        let v = self.pending_miniapps_center;
        self.pending_miniapps_center = false;
        v
    }

    pub fn take_trigger_update(&mut self) -> bool {
        let v = self.pending_trigger_update;
        self.pending_trigger_update = false;
        v
    }

    pub fn set_available_update(&mut self, version: Option<String>) {
        self.available_update_version = version;
    }

    pub fn active_account(&self) -> AccountSessionRecord {
        self.accounts
            .iter()
            .find(|a| a.account_id == self.active_account_id)
            .cloned()
            .or_else(|| self.accounts.first().cloned())
            .unwrap_or_else(|| AccountSessionRecord {
                account_id: 1,
                display_name: "Desktop User".to_string(),
                username: String::new(),
                phone: String::new(),
                bio: String::new(),
                emoji_status: String::new(),
                is_premium: false,
                is_verified: false,
                avatar_initials: "TG".to_string(),
                avatar_color: 0x2AABEE,
                unread_chats: 0,
            })
    }
}

enum MenuRowEntry {
    Action {
        icon: IconName,
        zh: &'static str,
        en: &'static str,
        cmd: MainMenuCommand,
    },
    Custom {
        id: &'static str,
        icon: IconName,
        zh: &'static str,
        en: &'static str,
        kind: u8,
    },
    Divider,
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
        let palette = self.palette;

        let drawer_bg = rgb(palette.dialogs_bg);
        let header_bg = if palette.mode.is_dark() {
            rgb(palette.modal_bg)
        } else {
            rgb(palette.nav_rail_bg)
        };
        let row_hover = rgb(palette.dialogs_hover_bg);
        let fg_primary = rgb(palette.text_primary);
        let fg_muted = rgb(palette.text_secondary);
        let icon_color = palette.icon_fg;
        let divider_col = rgb(palette.border_fg);
        let avatar_bg = rgb(if active.avatar_color != 0 {
            active.avatar_color
        } else {
            palette.peer_avatar_color(active.account_id as i64)
        });
        let subtitle_line = match (!active.phone.is_empty(), !active.username.is_empty()) {
            (true, true) => format!("{}  •  @{}", active.phone, active.username),
            (true, false) => active.phone.clone(),
            (false, true) => format!("@{}", active.username),
            (false, false) => tr_str(&lang, "未绑定手机号", "Connected session").to_string(),
        };

        let menu_rows = vec![
            MenuRowEntry::Action {
                icon: IconName::ContactsUsers,
                zh: "我的个人资料",
                en: "My Profile",
                cmd: MainMenuCommand::MyProfile,
            },
            MenuRowEntry::Divider,
            MenuRowEntry::Action {
                icon: IconName::BotRobot,
                zh: "新建智能体 (Fabushi Bot)",
                en: "New Fabushi Bot",
                cmd: MainMenuCommand::NewFabushiBot,
            },
            MenuRowEntry::Action {
                icon: IconName::GroupUsers,
                zh: "新建群组",
                en: "New Group",
                cmd: MainMenuCommand::NewGroup,
            },
            MenuRowEntry::Action {
                icon: IconName::ChannelMegaphone,
                zh: "新建频道",
                en: "New Channel",
                cmd: MainMenuCommand::NewChannel,
            },
            MenuRowEntry::Action {
                icon: IconName::ContactsUsers,
                zh: "联系人",
                en: "Contacts",
                cmd: MainMenuCommand::Contacts,
            },
            MenuRowEntry::Action {
                icon: IconName::PhoneCall,
                zh: "通话记录",
                en: "Calls",
                cmd: MainMenuCommand::Calls,
            },
            MenuRowEntry::Action {
                icon: IconName::SavedMessages,
                zh: "收藏夹",
                en: "Saved Messages",
                cmd: MainMenuCommand::SavedMessages,
            },
            MenuRowEntry::Divider,
            MenuRowEntry::Custom {
                id: "main-menu-botfather",
                icon: IconName::BotRobot,
                zh: "BotFather · 创建机器人/小程序",
                en: "BotFather · Create Bot/App",
                kind: 1,
            },
            MenuRowEntry::Custom {
                id: "main-menu-miniapps",
                icon: IconName::FolderBots,
                zh: "小程序中心 (Mini Apps)",
                en: "Mini Apps Center",
                kind: 2,
            },
            MenuRowEntry::Divider,
            MenuRowEntry::Action {
                icon: IconName::Poll,
                zh: "任务中心",
                en: "Task Center",
                cmd: MainMenuCommand::TaskCenter,
            },
            MenuRowEntry::Action {
                icon: IconName::TerminalMcp,
                zh: "MCP 工具",
                en: "MCP Tools",
                cmd: MainMenuCommand::McpTools,
            },
            MenuRowEntry::Action {
                icon: IconName::ComputerDesktop,
                zh: "远程电脑",
                en: "Remote Computer",
                cmd: MainMenuCommand::RemoteComputer,
            },
            MenuRowEntry::Divider,
            MenuRowEntry::Action {
                icon: IconName::SettingsGear,
                zh: "设置",
                en: "Settings",
                cmd: MainMenuCommand::Settings,
            },
        ];

        let secondary_rows = vec![
            MenuRowEntry::Action {
                icon: IconName::WalletStars,
                zh: "TON 钱包与 Stars",
                en: "Wallet & Stars",
                cmd: MainMenuCommand::WalletAndStars,
            },
            MenuRowEntry::Action {
                icon: IconName::FileDoc,
                zh: "导出聊天数据",
                en: "Export Chat Data",
                cmd: MainMenuCommand::ExportData,
            },
            MenuRowEntry::Action {
                icon: IconName::SelectCheck,
                zh: "键盘快捷键",
                en: "Keyboard Shortcuts",
                cmd: MainMenuCommand::ShortcutsLegend,
            },
            MenuRowEntry::Action {
                icon: IconName::BackArrow,
                zh: "退出登录",
                en: "Log Out",
                cmd: MainMenuCommand::Logout,
            },
        ];

        div()
            .id("window-main-menu-backdrop")
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .occlude()
            .bg(rgba(0x00000080))
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
                    .w(px(276.0))
                    .h_full()
                    .bg(drawer_bg)
                    .border_r_1()
                    .border_color(divider_col)
                    .shadow_lg()
                    .flex()
                    .flex_col()
                    .justify_between()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|_this, _event, _window, _cx| {
                            // Stop propagation so clicking inside drawer keeps it open
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
                            // 1. Top User Cover (Telegram Desktop window_main_menu.cpp)
                            .child(
                                div()
                                    .w_full()
                                    .px(px(18.0))
                                    .pt(px(20.0))
                                    .pb(px(14.0))
                                    .bg(header_bg)
                                    .border_b_1()
                                    .border_color(divider_col)
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.0))
                                    .child(
                                        div()
                                            .w_full()
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .child(
                                                div()
                                                    .w(px(50.0))
                                                    .h(px(50.0))
                                                    .rounded_full()
                                                    .bg(avatar_bg)
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
                                                    .child(
                                                        div()
                                                            .cursor_pointer()
                                                            .px(px(8.0))
                                                            .py(px(4.0))
                                                            .rounded_full()
                                                            .bg(rgba(0xFFFFFF18))
                                                            .hover(|s| s.bg(rgba(0xFFFFFF2E)))
                                                            .flex()
                                                            .items_center()
                                                            .gap(px(4.0))
                                                            .text_size(px(12.0))
                                                            .text_color(rgb(0xFFFFFF))
                                                            .on_mouse_down(
                                                                MouseButton::Left,
                                                                cx.listener(|this, _e, _w, cx| {
                                                                    this.emoji_status_picker_open =
                                                                        !this.emoji_status_picker_open;
                                                                    cx.notify();
                                                                }),
                                                            )
                                                            .child(tg_icon(
                                                                IconName::PremiumStar,
                                                                13.0,
                                                                0xFFFFFF,
                                                            ))
                                                            .child(if active.emoji_status.is_empty() {
                                                                tr_str(&lang, "设置状态", "Status")
                                                                    .to_string()
                                                            } else {
                                                                active.emoji_status.clone()
                                                            }),
                                                    )
                                                    .child(
                                                        div()
                                                            .cursor_pointer()
                                                            .w(px(28.0))
                                                            .h(px(28.0))
                                                            .rounded_full()
                                                            .bg(rgba(0xFFFFFF18))
                                                            .hover(|s| s.bg(rgba(0xFFFFFF2E)))
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .on_mouse_down(
                                                                MouseButton::Left,
                                                                cx.listener(|this, _e, _w, cx| {
                                                                    this.is_open = false;
                                                                    this.emoji_status_picker_open = false;
                                                                    cx.notify();
                                                                }),
                                                            )
                                                            .child(tg_icon(
                                                                IconName::Close,
                                                                13.0,
                                                                0xFFFFFF,
                                                            )),
                                                    ),
                                            ),
                                    )
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
                                                    ["✨", "🚀", "💻", "☕", "🎧", "🔥", "⭐", ""]
                                                        .into_iter()
                                                        .map(|em| {
                                                            let em_s = em.to_string();
                                                            div()
                                                                .cursor_pointer()
                                                                .w(px(28.0))
                                                                .h(px(28.0))
                                                                .rounded(px(6.0))
                                                                .hover(|s| s.bg(rgb(palette.accent)))
                                                                .flex()
                                                                .items_center()
                                                                .justify_center()
                                                                .text_size(px(14.0))
                                                                .text_color(rgb(0xFFFFFF))
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
                                                                .when(!em.is_empty(), |c| c.child(em))
                                                                .when(em.is_empty(), |c| {
                                                                    c.child(tg_icon(
                                                                        IconName::Close,
                                                                        12.0,
                                                                        0xFFFFFF,
                                                                    ))
                                                                })
                                                        }),
                                                ),
                                        )
                                    })
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
                                                    .gap(px(2.0))
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .items_center()
                                                            .gap(px(5.0))
                                                            .child(
                                                                div()
                                                                    .text_size(px(14.0))
                                                                    .font_weight(FontWeight::SEMIBOLD)
                                                                    .text_color(rgb(0xFFFFFF))
                                                                    .child(active.display_name.clone()),
                                                            )
                                                            .when(active.is_verified, |el| {
                                                                el.child(tg_icon(
                                                                    IconName::Verified,
                                                                    14.0,
                                                                    0xFFFFFF,
                                                                ))
                                                            })
                                                            .when(active.is_premium, |el| {
                                                                el.child(tg_icon(
                                                                    IconName::PremiumStar,
                                                                    13.0,
                                                                    0xFFFFFF,
                                                                ))
                                                            })
                                                            .when(!active.emoji_status.is_empty(), |el| {
                                                                el.child(
                                                                    div()
                                                                        .text_size(px(13.0))
                                                                        .child(active.emoji_status.clone()),
                                                                )
                                                            }),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(12.0))
                                                            .text_color(rgb(0xC2D1E0))
                                                            .child(subtitle_line),
                                                    ),
                                            )
                                            .child(tg_icon(
                                                if expanded {
                                                    IconName::ChevronDown
                                                } else {
                                                    IconName::ChevronRight
                                                },
                                                14.0,
                                                0xFFFFFF,
                                            )),
                                    ),
                            )
                            // 2. Expandable Multi-Account Switcher (+ Add Account)
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
                                            let acc_color = if acc.avatar_color != 0 {
                                                acc.avatar_color
                                            } else {
                                                palette.peer_avatar_color(acc_id as i64)
                                            };
                                            div()
                                                .cursor_pointer()
                                                .h(px(40.0))
                                                .px(px(18.0))
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
                                                        .gap(px(12.0))
                                                        .child(
                                                            div()
                                                                .w(px(26.0))
                                                                .h(px(26.0))
                                                                .rounded_full()
                                                                .bg(rgb(acc_color))
                                                                .flex()
                                                                .items_center()
                                                                .justify_center()
                                                                .text_size(px(10.5))
                                                                .font_weight(FontWeight::BOLD)
                                                                .text_color(rgb(0xFFFFFF))
                                                                .child(acc.avatar_initials),
                                                        )
                                                        .child(
                                                            div()
                                                                .text_size(px(13.5))
                                                                .font_weight(if is_cur {
                                                                    FontWeight::SEMIBOLD
                                                                } else {
                                                                    FontWeight::NORMAL
                                                                })
                                                                .text_color(fg_primary)
                                                                .child(acc.display_name),
                                                        ),
                                                )
                                                .when(is_cur, |el| {
                                                    el.child(tg_icon(
                                                        IconName::SelectCheck,
                                                        14.0,
                                                        palette.accent,
                                                    ))
                                                })
                                                .when(!is_cur && acc.unread_chats > 0, |el| {
                                                    el.child(
                                                        div()
                                                            .min_w(px(18.0))
                                                            .h(px(18.0))
                                                            .px(px(5.0))
                                                            .rounded_full()
                                                            .bg(rgb(palette.unread_badge_bg))
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .text_size(px(10.5))
                                                            .font_weight(FontWeight::BOLD)
                                                            .text_color(rgb(palette.unread_badge_fg))
                                                            .child(format!("{}", acc.unread_chats)),
                                                    )
                                                })
                                        }))
                                        .child(
                                            div()
                                                .cursor_pointer()
                                                .h(px(40.0))
                                                .px(px(18.0))
                                                .hover(move |s| s.bg(row_hover))
                                                .flex()
                                                .items_center()
                                                .gap(px(12.0))
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(|this, _e, _w, cx| {
                                                        this.pending_command =
                                                            Some(MainMenuCommand::AddAccount);
                                                        this.is_open = false;
                                                        cx.notify();
                                                    }),
                                                )
                                                .child(tg_icon(
                                                    IconName::ContactsUsers,
                                                    18.0,
                                                    palette.accent,
                                                ))
                                                .child(
                                                    div()
                                                        .text_size(px(13.5))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(rgb(palette.accent))
                                                        .child(tr_str(
                                                            &lang,
                                                            "+ 添加账号",
                                                            "+ Add Account",
                                                        )),
                                                ),
                                        ),
                                )
                            })
                            // 3. Native 40px Telegram Menu Rows
                            .child(
                                div()
                                    .w_full()
                                    .py(px(6.0))
                                    .flex()
                                    .flex_col()
                                    .children(menu_rows.into_iter().map(|entry| match entry {
                                        MenuRowEntry::Divider => div()
                                            .w_full()
                                            .h(px(1.0))
                                            .my(px(4.0))
                                            .bg(divider_col)
                                            .into_any_element(),
                                        MenuRowEntry::Custom {
                                            id,
                                            icon,
                                            zh,
                                            en,
                                            kind,
                                        } => {
                                            let label = tr_str(&lang, zh, en).to_string();
                                            div()
                                                .id(id)
                                                .cursor_pointer()
                                                .h(px(40.0))
                                                .px(px(18.0))
                                                .hover(move |s| s.bg(row_hover))
                                                .flex()
                                                .items_center()
                                                .gap(px(16.0))
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(move |this, _e, _w, cx| {
                                                        if kind == 1 {
                                                            this.pending_botfather_open = true;
                                                        } else {
                                                            this.pending_miniapps_center = true;
                                                        }
                                                        this.is_open = false;
                                                        cx.notify();
                                                    }),
                                                )
                                                .child(tg_icon(icon, 19.0, palette.accent))
                                                .child(
                                                    div()
                                                        .text_size(px(13.5))
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .text_color(fg_primary)
                                                        .child(label),
                                                )
                                                .into_any_element()
                                        }
                                        MenuRowEntry::Action { icon, zh, en, cmd } => {
                                            let label = tr_str(&lang, zh, en).to_string();
                                            div()
                                                .cursor_pointer()
                                                .h(px(40.0))
                                                .px(px(18.0))
                                                .hover(move |s| s.bg(row_hover))
                                                .flex()
                                                .items_center()
                                                .gap(px(16.0))
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(move |this, _e, _w, cx| {
                                                        this.pending_command = Some(cmd.clone());
                                                        this.is_open = false;
                                                        cx.notify();
                                                    }),
                                                )
                                                .child(tg_icon(icon, 19.0, icon_color))
                                                .child(
                                                    div()
                                                        .text_size(px(13.5))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(fg_primary)
                                                        .child(label),
                                                )
                                                .into_any_element()
                                        }
                                    }))
                                    // Instant Night Mode Toggle Row (with native switch track)
                                    .child(
                                        div()
                                            .cursor_pointer()
                                            .h(px(40.0))
                                            .px(px(18.0))
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
                                                    .gap(px(16.0))
                                                    .child(tg_icon(
                                                        IconName::NightMoon,
                                                        19.0,
                                                        icon_color,
                                                    ))
                                                    .child(
                                                        div()
                                                            .text_size(px(13.5))
                                                            .font_weight(FontWeight::MEDIUM)
                                                            .text_color(fg_primary)
                                                            .child(tr_str(
                                                                &lang,
                                                                "夜间模式",
                                                                "Night Mode",
                                                            )),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .w(px(34.0))
                                                    .h(px(18.0))
                                                    .rounded_full()
                                                    .p(px(2.0))
                                                    .bg(if night {
                                                        rgb(palette.accent)
                                                    } else {
                                                        rgb(palette.muted_badge_bg)
                                                    })
                                                    .flex()
                                                    .items_center()
                                                    .when(night, |el| el.justify_end())
                                                    .when(!night, |el| el.justify_start())
                                                    .child(
                                                        div()
                                                            .w(px(14.0))
                                                            .h(px(14.0))
                                                            .rounded_full()
                                                            .bg(rgb(0xFFFFFF)),
                                                    ),
                                            ),
                                    )
                                    .child(div().w_full().h(px(1.0)).my(px(4.0)).bg(divider_col))
                                    .children(secondary_rows.into_iter().map(|entry| match entry {
                                        MenuRowEntry::Divider => div().into_any_element(),
                                        MenuRowEntry::Custom {
                                            id,
                                            icon,
                                            zh,
                                            en,
                                            kind,
                                        } => {
                                            let label = tr_str(&lang, zh, en).to_string();
                                            div()
                                                .id(id)
                                                .cursor_pointer()
                                                .h(px(40.0))
                                                .px(px(18.0))
                                                .hover(move |s| s.bg(row_hover))
                                                .flex()
                                                .items_center()
                                                .gap(px(16.0))
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(move |this, _e, _w, cx| {
                                                        if kind == 1 {
                                                            this.pending_botfather_open = true;
                                                        } else {
                                                            this.pending_miniapps_center = true;
                                                        }
                                                        this.is_open = false;
                                                        cx.notify();
                                                    }),
                                                )
                                                .child(tg_icon(icon, 19.0, palette.accent))
                                                .child(
                                                    div()
                                                        .text_size(px(13.5))
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .text_color(fg_primary)
                                                        .child(label),
                                                )
                                                .into_any_element()
                                        }
                                        MenuRowEntry::Action { icon, zh, en, cmd } => {
                                            let label = tr_str(&lang, zh, en).to_string();
                                            div()
                                                .cursor_pointer()
                                                .h(px(40.0))
                                                .px(px(18.0))
                                                .hover(move |s| s.bg(row_hover))
                                                .flex()
                                                .items_center()
                                                .gap(px(16.0))
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(move |this, _e, _w, cx| {
                                                        this.pending_command = Some(cmd.clone());
                                                        this.is_open = false;
                                                        cx.notify();
                                                    }),
                                                )
                                                .child(tg_icon(icon, 19.0, icon_color))
                                                .child(
                                                    div()
                                                        .text_size(px(13.5))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(fg_primary)
                                                        .child(label),
                                                )
                                                .into_any_element()
                                        }
                                    })),
                            ),
                    )
                    // 4. Optional One-Click Update Pill + Footer Version Stamp
                    .when_some(self.available_update_version.clone(), |el, upd_ver| {
                        let upd_label = if lang.starts_with("zh") {
                            format!("发现新版本 v{upd_ver} • 点击一键更新")
                        } else {
                            format!("Update v{upd_ver} ready • One-Click Update")
                        };
                        el.child(
                            div()
                                .id("main-menu-oneclick-update-pill")
                                .cursor_pointer()
                                .mx(px(14.0))
                                .mt(px(6.0))
                                .mb(px(2.0))
                                .px(px(12.0))
                                .py(px(8.0))
                                .rounded(px(8.0))
                                .bg(rgb(palette.accent))
                                .hover(move |s| s.bg(rgb(palette.accent_hover)))
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_size(px(12.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(palette.active_button_fg))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        this.pending_trigger_update = true;
                                        this.is_open = false;
                                        cx.notify();
                                    }),
                                )
                                .child(upd_label),
                        )
                    })
                    .child(
                        div()
                            .cursor_pointer()
                            .w_full()
                            .px(px(18.0))
                            .py(px(12.0))
                            .hover(move |s| s.bg(row_hover))
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _e, _w, cx| {
                                    this.pending_command = Some(MainMenuCommand::AboutTelegram);
                                    this.is_open = false;
                                    cx.notify();
                                }),
                            )
                            .child(
                                div()
                                    .text_size(px(12.5))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(palette.text_link))
                                    .child("Fabushi Desktop"),
                            )
                            .child(
                                div()
                                    .text_size(px(11.5))
                                    .text_color(fg_muted)
                                    .child(concat!("Fabushi v", env!("CARGO_PKG_VERSION"))),
                            ),
                    ),
            )
            .into_any_element()
    }
}
