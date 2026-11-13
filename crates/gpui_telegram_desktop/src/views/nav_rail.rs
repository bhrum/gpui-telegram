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

use gpui::*;
use gpui::prelude::FluentBuilder;
use serde::{Deserialize, Serialize};

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
    pub account_initials: String,
    pub is_connected: bool,
    pub selected_index: usize,
    pub logout_requested: bool,
    pub trigger_counter: u64,
}

impl NavRail {
    pub fn new() -> Self {
        let items = vec![
            NavItem {
                destination: NavDestination::Chats,
                icon_label: "💬".to_string(),
                tooltip: "All Chats".to_string(),
                unread_badge: 5,
            },
            NavItem {
                destination: NavDestination::Folders,
                icon_label: "📁".to_string(),
                tooltip: "Chat Folders".to_string(),
                unread_badge: 0,
            },
            NavItem {
                destination: NavDestination::Calls,
                icon_label: "📞".to_string(),
                tooltip: "Calls".to_string(),
                unread_badge: 0,
            },
            NavItem {
                destination: NavDestination::TaskCenter,
                icon_label: "📋".to_string(),
                tooltip: "Global Task Center (CAP-02)".to_string(),
                unread_badge: 0,
            },
            NavItem {
                destination: NavDestination::McpTools,
                icon_label: "🧩".to_string(),
                tooltip: "MCP & Agent Tools (CAP-03)".to_string(),
                unread_badge: 0,
            },
            NavItem {
                destination: NavDestination::TalkMode,
                icon_label: "🎙️".to_string(),
                tooltip: "Talk Mode Orb (CAP-05)".to_string(),
                unread_badge: 0,
            },
            NavItem {
                destination: NavDestination::RemoteComputer,
                icon_label: "🖥️".to_string(),
                tooltip: "Remote Computer Workbench (CAP-04)".to_string(),
                unread_badge: 0,
            },
            NavItem {
                destination: NavDestination::SavedMessages,
                icon_label: "🔖".to_string(),
                tooltip: "Saved Messages".to_string(),
                unread_badge: 0,
            },
            NavItem {
                destination: NavDestination::Settings,
                icon_label: "⚙️".to_string(),
                tooltip: "Settings".to_string(),
                unread_badge: 0,
            },
        ];

        Self {
            active_destination: NavDestination::Chats,
            items,
            account_initials: "TG".to_string(),
            is_connected: true,
            selected_index: 0,
            logout_requested: false,
            trigger_counter: 0,
        }
    }

    pub fn select(&mut self, dest: NavDestination) {
        self.active_destination = dest;
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

impl Render for NavRail {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let active_dest = self.active_destination;
        let is_connected = self.is_connected;
        let initials = self.account_initials.clone();

        div()
            .w(px(NAV_RAIL_WIDTH))
            .flex_shrink_0()
            .h_full()
            .bg(rgb(0x0E1621))
            .flex()
            .flex_col()
            .items_center()
            .py(px(12.0))
            .justify_between()
            // Top Section: Avatar + Divider + Main Navigation Items
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(10.0))
                    // Account Avatar (Click to switch account / log out to Login Screen)
                    .child(
                        div()
                            .relative()
                            .w(px(USERPIC_DIAMETER))
                            .h(px(USERPIC_DIAMETER))
                            .rounded_full()
                            .bg(rgb(0x2AABEE))
                            .hover(|s| s.bg(rgb(0x40A7E3)))
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
                                    this.logout_requested = true;
                                    cx.notify();
                                }),
                            )
                            .child(initials)
                            // Online Dot
                            .child(
                                div()
                                    .absolute()
                                    .bottom_0()
                                    .right_0()
                                    .w(px(ONLINE_DOT_DIAMETER))
                                    .h(px(ONLINE_DOT_DIAMETER))
                                    .rounded_full()
                                    .bg(if is_connected { rgb(0x4CAF50) } else { rgb(0xFF9800) })
                                    .border_1()
                                    .border_color(rgb(0x0E1621)),
                            ),
                    )
                    // Divider Line
                    .child(
                        div()
                            .w(px(NAV_DIVIDER_WIDTH))
                            .h(px(NAV_DIVIDER_HEIGHT))
                            .bg(rgba(0xFFFFFF20)),
                    )
                    // Navigation Item Icons
                    .children(self.items.iter().map(|item| {
                        let dest = item.destination;
                        let is_active = dest == active_dest;
                        let bg_color = if is_active {
                            rgb(0x2B5278)
                        } else {
                            rgba(0x00000000)
                        };
                        let text_color = if is_active {
                            rgb(0xFFFFFF)
                        } else {
                            rgb(0x8A9AA8)
                        };

                        div()
                            .relative()
                            .w(px(BUTTON_SIZE))
                            .h(px(BUTTON_SIZE))
                            .rounded(px(8.0))
                            .bg(bg_color)
                            .hover(|s| s.bg(rgb(0x1F3B57)))
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(18.0))
                            .text_color(text_color)
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, _window, cx| {
                                    this.select(dest);
                                    cx.notify();
                                }),
                            )
                            .child(item.icon_label.clone())
                            // Unread Badge Pill
                            .when(item.unread_badge > 0, |el| {
                                el.child(
                                    div()
                                        .absolute()
                                        .top(px(2.0))
                                        .right(px(2.0))
                                        .min_w(px(16.0))
                                        .h(px(16.0))
                                        .px(px(4.0))
                                        .rounded_full()
                                        .bg(rgb(0x40A7E3))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_size(px(10.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0xFFFFFF))
                                        .child(format!("{}", item.unread_badge)),
                                )
                            })
                    })),
            )
            // Bottom Section: Logout Button & Connection status dot
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div()
                            .w(px(36.0))
                            .h(px(28.0))
                            .rounded(px(6.0))
                            .bg(rgba(0xE5393522))
                            .hover(|s| s.bg(rgba(0xE5393555)))
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(12.0))
                            .text_color(rgb(0xE53935))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _window, cx| {
                                    this.logout_requested = true;
                                    cx.notify();
                                }),
                            )
                            .child("🚪"),
                    )
                    .child(
                        div()
                            .w(px(8.0))
                            .h(px(8.0))
                            .rounded_full()
                            .bg(if is_connected { rgb(0x4CAF50) } else { rgb(0xFF9800) }),
                    )
                    .child(
                        div()
                            .text_size(px(9.0))
                            .text_color(rgb(0x6B808D))
                            .child(if is_connected { "OK" } else { "..." }),
                    ),
            )
    }
}
