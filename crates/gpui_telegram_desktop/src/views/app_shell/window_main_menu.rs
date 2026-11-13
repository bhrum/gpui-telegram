//! WindowMainMenu: Hamburger sliding navigation drawer with usercard, account switching, and actions
//! Upstream: Telegram/SourceFiles/window/window_main_menu.h, window_main_menu.cpp

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountEntry {
    pub account_id: u32,
    pub name: String,
    pub phone: String,
    pub unread_count: i32,
    pub is_active: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct OthersUnreadState {
    pub count: i32,
    pub all_muted: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MainMenuItem {
    MyProfile,
    NewGroup,
    NewChannel,
    Contacts,
    Calls,
    SavedMessages,
    Settings,
    NightMode,
}

#[derive(Debug, Clone)]
pub struct WindowMainMenu {
    pub is_open: bool,
    pub accounts: Vec<AccountEntry>,
    pub accounts_expanded: bool,
    pub night_mode: bool,
    pub app_version: String,
    pub selected_item: Option<MainMenuItem>,
}

impl WindowMainMenu {
    pub fn new() -> Self {
        Self {
            is_open: false,
            accounts: vec![
                AccountEntry {
                    account_id: 1,
                    name: "Primary User".to_string(),
                    phone: "+1 555 0199".to_string(),
                    unread_count: 0,
                    is_active: true,
                }
            ],
            accounts_expanded: false,
            night_mode: true,
            app_version: "Telegram Desktop 5.6.3 (GPUI Rust)".to_string(),
            selected_item: None,
        }
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.accounts_expanded = false;
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn toggle_accounts(&mut self) {
        self.accounts_expanded = !self.accounts_expanded;
    }

    pub fn toggle_night_mode(&mut self) -> bool {
        self.night_mode = !self.night_mode;
        self.night_mode
    }

    pub fn select_item(&mut self, item: MainMenuItem) {
        self.selected_item = Some(item);
        if item != MainMenuItem::NightMode {
            self.close();
        }
    }

    pub fn others_unread_state(&self) -> OthersUnreadState {
        let mut total = 0;
        for acc in &self.accounts {
            if !acc.is_active {
                total += acc.unread_count;
            }
        }
        OthersUnreadState {
            count: total,
            all_muted: false,
        }
    }

    pub fn switch_account(&mut self, target_id: u32) {
        for acc in &mut self.accounts {
            acc.is_active = acc.account_id == target_id;
        }
    }
}

impl Default for WindowMainMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowMainMenu {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_open {
            return div().into_any_element();
        }

        let primary_account = self.accounts.iter().find(|a| a.is_active).cloned();

        div()
            .absolute()
            .inset_0()
            .bg(rgba(0x00000080))
            .child(
                div()
                    .w(px(280.0))
                    .h_full()
                    .bg(rgb(0x17212B))
                    .border_r_1()
                    .border_color(rgb(0x242F3D))
                    .flex()
                    .flex_col()
                    .child(
                        // User Profile Header
                        div()
                            .p(px(16.0))
                            .bg(rgb(0x242F3D))
                            .child(
                                div()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .child(primary_account.as_ref().map(|a| a.name.clone()).unwrap_or_default())
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x82868A))
                                    .child(primary_account.as_ref().map(|a| a.phone.clone()).unwrap_or_default())
                            )
                    )
                    .child(
                        // Navigation Menu Items
                        div()
                            .flex_1()
                            .p(px(8.0))
                            .flex()
                            .flex_col()
                            .gap(px(4.0))
                            .text_color(rgb(0xFFFFFF))
                            .text_sm()
                            .child(div().p(px(8.0)).child("My Profile"))
                            .child(div().p(px(8.0)).child("New Group"))
                            .child(div().p(px(8.0)).child("New Channel"))
                            .child(div().p(px(8.0)).child("Contacts"))
                            .child(div().p(px(8.0)).child("Calls"))
                            .child(div().p(px(8.0)).child("Saved Messages"))
                            .child(div().p(px(8.0)).child("Settings"))
                            .child(
                                div()
                                    .p(px(8.0))
                                    .flex()
                                    .justify_between()
                                    .child("Night Mode")
                                    .child(if self.night_mode { "ON" } else { "OFF" })
                            )
                    )
                    .child(
                        // Version footer
                        div()
                            .p(px(12.0))
                            .text_xs()
                            .text_color(rgb(0x6C7883))
                            .child(self.app_version.clone())
                    )
            )
            .into_any_element()
    }
}
