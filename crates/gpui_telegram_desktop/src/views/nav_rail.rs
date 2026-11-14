//! Left 52px Primary Navigation Rail (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.2).
//!
//! Provides navigation between main application destinations:
//! - Chats & Folders with unread badges
//! - Calls
//! - Global Task Center (CAP-02)
//! - MCP Tools Dock (CAP-03)
//! - Talk Mode Full-Duplex Orb (CAP-05)
//! - Remote Computer Workbench (CAP-04)
//! - Saved Messages & Settings
//! - Connection Status indicator

use gpui::prelude::FluentBuilder;
use gpui::*;
use serde::{Deserialize, Serialize};

use crate::i18n::tr_str;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NavDestination {
    Chats,
    Folders,
    Calls,
    TaskCenter,
    McpTools,
    TalkMode,
    RemoteComputer,
    SavedMessages,
    Settings,
}

impl NavDestination {
    pub fn icon_name(&self) -> IconName {
        match self {
            Self::Chats => IconName::FolderAll,
            Self::Folders => IconName::FolderChat,
            Self::Calls => IconName::PhoneCall,
            Self::TaskCenter => IconName::Poll,
            Self::McpTools => IconName::TerminalMcp,
            Self::TalkMode => IconName::Mic,
            Self::RemoteComputer => IconName::ComputerDesktop,
            Self::SavedMessages => IconName::SavedMessages,
            Self::Settings => IconName::SettingsGear,
        }
    }

    pub fn localized_label<'a>(&self, lang_id: &str) -> &'a str {
        match self {
            Self::Chats => tr_str(lang_id, "全部会话", "All Chats"),
            Self::Folders => tr_str(lang_id, "会话分组", "Folders"),
            Self::Calls => tr_str(lang_id, "通话", "Calls"),
            Self::TaskCenter => tr_str(lang_id, "任务中心", "Task Center"),
            Self::McpTools => tr_str(lang_id, "MCP 工具", "MCP Tools"),
            Self::TalkMode => tr_str(lang_id, "语音对话", "Talk Mode"),
            Self::RemoteComputer => tr_str(lang_id, "远程电脑", "Remote Computer"),
            Self::SavedMessages => tr_str(lang_id, "收藏夹", "Saved Messages"),
            Self::Settings => tr_str(lang_id, "设置", "Settings"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavFolderItem {
    pub folder_id: i32,
    pub title: String,
    pub icon_hint: String,
    pub unread_badge: u32,
    pub is_muted: bool,
}

impl NavFolderItem {
    pub fn icon_name(&self) -> IconName {
        match self.icon_hint.to_ascii_lowercase().as_str() {
            "all" => IconName::FolderAll,
            "bots" | "bot" => IconName::FolderBots,
            "work" | "briefcase" | "folder_work" => IconName::FolderWork,
            "groups" | "group" => IconName::GroupUsers,
            "channels" | "channel" => IconName::ChannelMegaphone,
            "contacts" | "private" => IconName::ContactsUsers,
            _ => IconName::FolderChat,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavItem {
    pub destination: NavDestination,
    pub icon_label: String,
    pub tooltip: String,
    pub unread_badge: u32,
}

pub struct NavRail {
    pub active_destination: NavDestination,
    pub items: Vec<NavItem>,
    pub custom_folders: Vec<NavFolderItem>,
    pub active_folder_id: Option<i32>,
    pub archive_unread_count: u32,
    pub show_archive_in_rail: bool,
    pub account_initials: String,
    pub account_name: String,
    pub is_connected: bool,
    pub selected_index: usize,
    pub logout_requested: bool,
    pub menu_open_requested: bool,
    pub archive_open_requested: bool,
    pub theme_toggle_requested: bool,
    pub selected_folder_requested: Option<i32>,
    pub trigger_counter: u64,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
}

impl NavRail {
    pub fn new() -> Self {
        let items = vec![
            NavItem {
                destination: NavDestination::Chats,
                icon_label: "chats".to_string(),
                tooltip: "All Chats".to_string(),
                unread_badge: 0,
            },
            NavItem {
                destination: NavDestination::Folders,
                icon_label: "folders".to_string(),
                tooltip: "Chat Folders".to_string(),
                unread_badge: 0,
            },
            NavItem {
                destination: NavDestination::Calls,
                icon_label: "calls".to_string(),
                tooltip: "Calls".to_string(),
                unread_badge: 0,
            },
            NavItem {
                destination: NavDestination::TaskCenter,
                icon_label: "tasks".to_string(),
                tooltip: "Task Center".to_string(),
                unread_badge: 0,
            },
            NavItem {
                destination: NavDestination::McpTools,
                icon_label: "mcp".to_string(),
                tooltip: "MCP Tools".to_string(),
                unread_badge: 0,
            },
            NavItem {
                destination: NavDestination::TalkMode,
                icon_label: "talk".to_string(),
                tooltip: "Talk Mode".to_string(),
                unread_badge: 0,
            },
            NavItem {
                destination: NavDestination::RemoteComputer,
                icon_label: "remote".to_string(),
                tooltip: "Remote Computer".to_string(),
                unread_badge: 0,
            },
            NavItem {
                destination: NavDestination::SavedMessages,
                icon_label: "saved".to_string(),
                tooltip: "Saved Messages".to_string(),
                unread_badge: 0,
            },
            NavItem {
                destination: NavDestination::Settings,
                icon_label: "settings".to_string(),
                tooltip: "Settings".to_string(),
                unread_badge: 0,
            },
        ];

        Self {
            active_destination: NavDestination::Chats,
            items,
            custom_folders: Vec::new(),
            active_folder_id: None,
            archive_unread_count: 0,
            show_archive_in_rail: false,
            account_initials: "TG".to_string(),
            account_name: String::new(),
            is_connected: false,
            selected_index: 0,
            logout_requested: false,
            menu_open_requested: false,
            archive_open_requested: false,
            theme_toggle_requested: false,
            selected_folder_requested: None,
            trigger_counter: 0,
            palette: TdesktopThemePalette::default(),
            language_id: "en".to_string(),
        }
    }

    pub fn select(&mut self, dest: NavDestination) {
        self.active_destination = dest;
        if dest == NavDestination::Chats {
            self.active_folder_id = None;
        }
        self.trigger_counter = self.trigger_counter.wrapping_add(1);
        if let Some(idx) = self.items.iter().position(|it| it.destination == dest) {
            self.selected_index = idx;
        }
    }

    /// Programmatically synchronize the active destination without incrementing `trigger_counter`.
    /// This prevents spurious observer re-triggers when returning to previous panels or closing overlays.
    pub fn set_active_destination(&mut self, dest: NavDestination) {
        self.active_destination = dest;
        if let Some(idx) = self.items.iter().position(|it| it.destination == dest) {
            self.selected_index = idx;
        }
    }

    pub fn set_unread(&mut self, dest: NavDestination, count: u32) {
        if let Some(it) = self.items.iter_mut().find(|it| it.destination == dest) {
            it.unread_badge = count;
        }
    }

    pub fn unread_total(&self) -> u32 {
        self.items.iter().map(|it| it.unread_badge).sum()
    }

    pub fn set_connected(&mut self, connected: bool) {
        self.is_connected = connected;
    }

    pub fn is_active(&self, dest: NavDestination) -> bool {
        self.active_destination == dest
    }

    pub fn set_folders(&mut self, folders: Vec<NavFolderItem>) {
        self.custom_folders = folders;
        if let Some(active_id) = self.active_folder_id {
            if !self.custom_folders.iter().any(|f| f.folder_id == active_id) {
                self.active_folder_id = None;
            }
        }
    }

    pub fn select_folder(&mut self, folder_id: Option<i32>) {
        self.active_destination = NavDestination::Chats;
        self.active_folder_id = folder_id;
        self.selected_folder_requested = folder_id;
        self.selected_index = 0;
        self.trigger_counter = self.trigger_counter.wrapping_add(1);
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) {
        self.palette = palette;
    }

    pub fn set_language(&mut self, lang_id: impl Into<String>) {
        self.language_id = lang_id.into();
    }

    pub fn set_account(&mut self, initials: impl Into<String>, name: impl Into<String>) {
        let init = initials.into();
        if !init.trim().is_empty() {
            self.account_initials = init;
        }
        self.account_name = name.into();
    }
}

pub const NAV_RAIL_WIDTH: f32 = 52.0;
pub const USERPIC_DIAMETER: f32 = 36.0;
pub const NAV_AVATAR_SIZE: f32 = USERPIC_DIAMETER;
pub const ONLINE_DOT_DIAMETER: f32 = 9.0;
pub const NAV_ONLINE_DOT_SIZE: f32 = ONLINE_DOT_DIAMETER;
pub const BUTTON_SIZE: f32 = 40.0;
pub const NAV_BUTTON_SIZE: f32 = BUTTON_SIZE;
pub const NAV_DIVIDER_WIDTH: f32 = 32.0;
pub const NAV_DIVIDER_HEIGHT: f32 = 1.0;

impl Default for NavRail {
    fn default() -> Self {
        Self::new()
    }
}

fn is_color_dark(hex: u32) -> bool {
    let r = ((hex >> 16) & 0xFF) as f32;
    let g = ((hex >> 8) & 0xFF) as f32;
    let b = (hex & 0xFF) as f32;
    (0.299 * r + 0.587 * g + 0.114 * b) < 140.0
}

fn format_compact_badge(count: u32) -> String {
    if count > 999 {
        "999+".to_string()
    } else {
        count.to_string()
    }
}

fn render_rail_tile(
    icon: IconName,
    icon_size: f32,
    is_active: bool,
    unread_badge: u32,
    is_muted: bool,
    dark_rail: bool,
    palette: &TdesktopThemePalette,
) -> Div {
    let active_bg = if dark_rail {
        rgba(0xFFFFFF1E)
    } else {
        rgb(palette.dialogs_active_bg)
    };
    let hover_bg = if dark_rail {
        rgba(0xFFFFFF12)
    } else {
        rgb(palette.dialogs_hover_bg)
    };
    let icon_color = if is_active {
        palette.active_button_fg
    } else {
        palette.icon_fg
    };
    let badge_bg = if is_muted {
        palette.muted_badge_bg
    } else {
        palette.unread_badge_bg
    };

    div()
        .relative()
        .w(px(BUTTON_SIZE))
        .h(px(BUTTON_SIZE))
        .rounded(px(10.0))
        .when(is_active, |el| el.bg(active_bg))
        .hover(move |s| s.bg(hover_bg))
        .cursor_pointer()
        .flex()
        .items_center()
        .justify_center()
        .when(is_active, |el| {
            el.child(
                div()
                    .absolute()
                    .left(px(-6.0))
                    .w(px(3.0))
                    .h(px(20.0))
                    .rounded_r(px(2.0))
                    .bg(rgb(palette.accent)),
            )
        })
        .child(tg_icon(icon, icon_size, icon_color))
        .when(unread_badge > 0, |el| {
            el.child(
                div()
                    .absolute()
                    .top(px(2.0))
                    .right(px(2.0))
                    .min_w(px(16.0))
                    .h(px(16.0))
                    .px(px(4.0))
                    .rounded_full()
                    .bg(rgb(badge_bg))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(9.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(palette.unread_badge_fg))
                    .child(format_compact_badge(unread_badge)),
            )
        })
}

impl Render for NavRail {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let palette = self.palette.clone();
        let lang = self.language_id.as_str();
        let active_dest = self.active_destination;
        let active_folder = self.active_folder_id;
        let is_connected = self.is_connected;
        let initials = self.account_initials.clone();
        let dark_rail = is_color_dark(palette.nav_rail_bg);
        let divider_bg = if dark_rail {
            rgba(0xFFFFFF22)
        } else {
            rgb(palette.border_fg)
        };
        let idle_fg = palette.icon_fg;
        let avatar_grad = palette.peer_avatar_gradient(1);

        div()
            .w(px(NAV_RAIL_WIDTH))
            .flex_shrink_0()
            .h_full()
            .bg(rgb(palette.nav_rail_bg))
            .border_r_1()
            .border_color(rgb(palette.border_fg))
            .flex()
            .flex_col()
            .items_center()
            .py(px(10.0))
            .justify_between()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(6.0))
                    // Hamburger Drawer Toggle
                    .child(
                        div()
                            .w(px(BUTTON_SIZE))
                            .h(px(32.0))
                            .rounded(px(8.0))
                            .hover(move |s| {
                                s.bg(if dark_rail {
                                    rgba(0xFFFFFF14)
                                } else {
                                    rgb(palette.dialogs_hover_bg)
                                })
                            })
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .justify_center()
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _window, cx| {
                                    this.menu_open_requested = true;
                                    this.trigger_counter = this.trigger_counter.wrapping_add(1);
                                    cx.notify();
                                }),
                            )
                            .child(tg_icon(IconName::HamburgerMenu, 18.0, idle_fg)),
                    )
                    // Account Profile Avatar -> opens MainMenuDrawer
                    .child(
                        div()
                            .relative()
                            .w(px(USERPIC_DIAMETER))
                            .h(px(USERPIC_DIAMETER))
                            .rounded_full()
                            .bg(rgb(avatar_grad.0))
                            .hover(|s| s.bg(rgb(palette.accent_hover)))
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_color(rgb(0xFFFFFF))
                            .font_weight(FontWeight::BOLD)
                            .text_size(px(13.0))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _window, cx| {
                                    this.menu_open_requested = true;
                                    this.trigger_counter = this.trigger_counter.wrapping_add(1);
                                    cx.notify();
                                }),
                            )
                            .child(initials)
                            .child(
                                div()
                                    .absolute()
                                    .bottom_0()
                                    .right_0()
                                    .w(px(ONLINE_DOT_DIAMETER))
                                    .h(px(ONLINE_DOT_DIAMETER))
                                    .rounded_full()
                                    .bg(if is_connected {
                                        rgb(palette.online_dot_fg)
                                    } else {
                                        rgb(palette.muted_badge_bg)
                                    })
                                    .border_1()
                                    .border_color(rgb(palette.nav_rail_bg)),
                            ),
                    )
                    .child(
                        div()
                            .w(px(NAV_DIVIDER_WIDTH))
                            .h(px(NAV_DIVIDER_HEIGHT))
                            .my(px(2.0))
                            .bg(divider_bg),
                    )
                    // All Chats primary folder button
                    .when_some(self.items.first().cloned(), |parent, all_item| {
                        let is_active =
                            active_dest == NavDestination::Chats && active_folder.is_none();
                        parent.child(
                            render_rail_tile(
                                IconName::FolderAll,
                                20.0,
                                is_active,
                                all_item.unread_badge,
                                false,
                                dark_rail,
                                &palette,
                            )
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _window, cx| {
                                    this.select_folder(None);
                                    cx.notify();
                                }),
                            ),
                        )
                    })
                    // Server-driven custom Folders
                    .children(self.custom_folders.iter().map(|folder| {
                        let fid = folder.folder_id;
                        let is_active =
                            active_dest == NavDestination::Chats && active_folder == Some(fid);
                        render_rail_tile(
                            folder.icon_name(),
                            19.0,
                            is_active,
                            folder.unread_badge,
                            folder.is_muted,
                            dark_rail,
                            &palette,
                        )
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _window, cx| {
                                this.select_folder(Some(fid));
                                cx.notify();
                            }),
                        )
                    }))
                    // Optional Archive shortcut when archived chats exist
                    .when(
                        self.show_archive_in_rail || self.archive_unread_count > 0,
                        |parent| {
                            let is_active =
                                active_dest == NavDestination::Chats && active_folder == Some(-1);
                            parent.child(
                                render_rail_tile(
                                    IconName::ArchivedChats,
                                    19.0,
                                    is_active,
                                    self.archive_unread_count,
                                    true,
                                    dark_rail,
                                    &palette,
                                )
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _, _window, cx| {
                                        this.archive_open_requested = true;
                                        this.select_folder(Some(-1));
                                        cx.notify();
                                    }),
                                ),
                            )
                        },
                    )
                    .when(!self.custom_folders.is_empty(), |parent| {
                        parent.child(
                            div()
                                .w(px(NAV_DIVIDER_WIDTH))
                                .h(px(NAV_DIVIDER_HEIGHT))
                                .my(px(2.0))
                                .bg(divider_bg),
                        )
                    })
                    // Core destinations (Calls, Saved, Task Center, MCP, Talk, Remote, Settings)
                    .children(self.items.iter().skip(1).map(|item| {
                        let dest = item.destination;
                        let is_active = dest == active_dest;
                        render_rail_tile(
                            dest.icon_name(),
                            19.0,
                            is_active,
                            item.unread_badge,
                            false,
                            dark_rail,
                            &palette,
                        )
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _window, cx| {
                                this.select(dest);
                                cx.notify();
                            }),
                        )
                    })),
            )
            // Bottom Section: Theme Toggle + Logout + Connection Dot
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(6.0))
                    .child(
                        div()
                            .w(px(36.0))
                            .h(px(30.0))
                            .rounded(px(8.0))
                            .hover(move |s| {
                                s.bg(if dark_rail {
                                    rgba(0xFFFFFF14)
                                } else {
                                    rgb(palette.dialogs_hover_bg)
                                })
                            })
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .justify_center()
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _window, cx| {
                                    this.theme_toggle_requested = true;
                                    this.trigger_counter = this.trigger_counter.wrapping_add(1);
                                    cx.notify();
                                }),
                            )
                            .child(tg_icon(
                                if palette.mode.is_dark() {
                                    IconName::SunDay
                                } else {
                                    IconName::NightMoon
                                },
                                16.0,
                                idle_fg,
                            )),
                    )
                    .child(
                        div()
                            .w(px(36.0))
                            .h(px(28.0))
                            .rounded(px(6.0))
                            .bg(rgba(0xE5393522))
                            .hover(|s| s.bg(rgba(0xE5393544)))
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .justify_center()
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _window, cx| {
                                    this.logout_requested = true;
                                    cx.notify();
                                }),
                            )
                            .child(tg_icon(IconName::BackArrow, 15.0, palette.danger)),
                    )
                    .child(
                        div()
                            .w(px(8.0))
                            .h(px(8.0))
                            .rounded_full()
                            .bg(if is_connected {
                                rgb(palette.online_dot_fg)
                            } else {
                                rgb(palette.muted_badge_bg)
                            }),
                    )
                    .child(
                        div()
                            .text_size(px(9.0))
                            .text_color(if dark_rail && !palette.mode.is_dark() {
                                rgb(palette.icon_fg)
                            } else {
                                rgb(palette.text_secondary)
                            })
                            .child(if is_connected {
                                tr_str(lang, "在线", "Live")
                            } else {
                                tr_str(lang, "离线", "Idle")
                            }),
                    ),
            )
    }
}
