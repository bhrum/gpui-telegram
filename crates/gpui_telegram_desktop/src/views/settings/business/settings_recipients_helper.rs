//! Upstream Source: Telegram/SourceFiles/settings/business/settings_recipients_helper.cpp, Telegram/SourceFiles/settings/business/settings_recipients_helper.h
//! Subsystem: settings | Feature ID: TCG-N31 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui::prelude::FluentBuilder;

pub const K_ALL_EXCEPT: u32 = 0;
pub const K_SELECTED_ONLY: u32 = 1;

pub const FLAG_CONTACTS: u32 = 1 << 0;
pub const FLAG_NON_CONTACTS: u32 = 1 << 1;
pub const FLAG_NEW_CHATS: u32 = 1 << 2;
pub const FLAG_EXISTING_CHATS: u32 = 1 << 3;

pub const K_QUICK_REPLIES_LIMIT: usize = 100;
pub const K_QUICK_REPLY_MESSAGES_LIMIT: usize = 20;

#[derive(Debug, Clone)]
pub struct RecipientsRule {
    pub mode: u32,
    pub flags: u32,
    pub included_peers: Vec<u64>,
    pub excluded_peers: Vec<u64>,
}

#[derive(Debug, Clone)]
pub struct SettingsRecipientsHelperBusiness {
    pub rule: RecipientsRule,
    pub is_bots_mode: bool,
    pub status_message: Option<String>,
}

impl SettingsRecipientsHelperBusiness {
    pub fn new() -> Self {
        Self {
            rule: RecipientsRule {
                mode: K_ALL_EXCEPT,
                flags: FLAG_CONTACTS | FLAG_NON_CONTACTS | FLAG_NEW_CHATS | FLAG_EXISTING_CHATS,
                included_peers: Vec::new(),
                excluded_peers: Vec::new(),
            },
            is_bots_mode: false,
            status_message: None,
        }
    }

    pub fn set_mode(&mut self, mode: u32) {
        self.rule.mode = mode;
    }

    pub fn toggle_flag(&mut self, flag: u32) {
        self.rule.flags ^= flag;
    }

    pub fn has_flag(&self, flag: u32) -> bool {
        (self.rule.flags & flag) != 0
    }

    pub fn add_excluded(&mut self, peer_id: u64) {
        if !self.rule.excluded_peers.contains(&peer_id) {
            self.rule.excluded_peers.push(peer_id);
            self.rule.included_peers.retain(|&id| id != peer_id);
        }
    }

    pub fn add_included(&mut self, peer_id: u64) {
        if !self.rule.included_peers.contains(&peer_id) {
            self.rule.included_peers.push(peer_id);
            self.rule.excluded_peers.retain(|&id| id != peer_id);
        }
    }

    pub fn is_valid(&self) -> bool {
        if self.rule.mode == K_SELECTED_ONLY {
            !self.rule.included_peers.is_empty() || (self.rule.flags != 0)
        } else {
            true
        }
    }
}

impl Default for SettingsRecipientsHelperBusiness {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsRecipientsHelperBusiness {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mode = self.rule.mode;
        let flags = self.rule.flags;
        let excluded_count = self.rule.excluded_peers.len();
        let included_count = self.rule.included_peers.len();

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .bg(rgb(0x0e1621))
            .gap(px(12.0))
            // Header
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .p(px(16.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x17212b))
                    .child(div().text_sm().font_weight(FontWeight::BOLD).text_color(rgb(0xffffff)).child("Business Recipients Filter"))
                    .child(div().text_xs().text_color(rgb(0x8292a1)).child("Choose which chats receive automated business responses.")),
            )
            // Mode selector
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(8.0))
                    .child(
                        div()
                            .flex_1()
                            .p(px(10.0))
                            .rounded(px(8.0))
                            .cursor_pointer()
                            .bg(if mode == K_ALL_EXCEPT { rgb(0x5288c1) } else { rgb(0x17212b) })
                            .text_xs()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                this.set_mode(K_ALL_EXCEPT);
                                cx.notify();
                            }))
                            .child("All Chats Except"),
                    )
                    .child(
                        div()
                            .flex_1()
                            .p(px(10.0))
                            .rounded(px(8.0))
                            .cursor_pointer()
                            .bg(if mode == K_SELECTED_ONLY { rgb(0x5288c1) } else { rgb(0x17212b) })
                            .text_xs()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                this.set_mode(K_SELECTED_ONLY);
                                cx.notify();
                            }))
                            .child("Only Selected Chats"),
                    ),
            )
            // Category checkboxes
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p(px(12.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x17212b))
                    .gap(px(8.0))
                    .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(rgb(0x8292a1)).child("CHAT CATEGORIES"))
                    .child(make_flag_row("Contacts", (flags & FLAG_CONTACTS) != 0, FLAG_CONTACTS, cx))
                    .child(make_flag_row("Non-Contacts", (flags & FLAG_NON_CONTACTS) != 0, FLAG_NON_CONTACTS, cx))
                    .child(make_flag_row("New Chats", (flags & FLAG_NEW_CHATS) != 0, FLAG_NEW_CHATS, cx))
                    .child(make_flag_row("Existing Chats", (flags & FLAG_EXISTING_CHATS) != 0, FLAG_EXISTING_CHATS, cx)),
            )
            // Summary count badge
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .p(px(12.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x17212b))
                    .child(div().text_sm().text_color(rgb(0xffffff)).child("Exceptions Configured"))
                    .child(div().text_xs().text_color(rgb(0x5288c1)).child(format!("Included: {} | Excluded: {}", included_count, excluded_count))),
            )
            .into_any_element()
    }
}

fn make_flag_row(
    label: &'static str,
    active: bool,
    flag: u32,
    cx: &mut Context<SettingsRecipientsHelperBusiness>,
) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .py(px(4.0))
        .cursor_pointer()
        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _window, cx| {
            this.toggle_flag(flag);
            cx.notify();
        }))
        .child(div().text_sm().text_color(rgb(0xffffff)).child(label))
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .text_color(if active { rgb(0x5288c1) } else { rgb(0x8292a1) })
                .child(if active { "INCLUDED" } else { "EXCLUDED" }),
        )
}
