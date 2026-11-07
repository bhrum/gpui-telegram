// Source: Telegram/SourceFiles/settings/sections/settings_blocked_peers.cpp, Telegram/SourceFiles/settings/sections/settings_blocked_peers.h
// Subsystem: settings | Feature ID: TCG-N11
// Upstream: Blocked::title, tr::lng_settings_blocked_users, tr::lng_blocked_list_empty_description

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockedPeerItem {
    pub user_id: u64,
    pub name: String,
    pub blocked_date: String,
}

#[derive(Debug, Clone)]
pub struct SettingsBlockedPeersSections {
    pub blocked_list: Vec<BlockedPeerItem>,
    pub search_query: String,
    pub is_loading: bool,
}

impl SettingsBlockedPeersSections {
    pub fn new() -> Self {
        Self {
            blocked_list: Vec::new(),
            search_query: String::new(),
            is_loading: false,
        }
    }

    pub fn block_peer(&mut self, user_id: u64, name: impl Into<String>, date: impl Into<String>) {
        if !self.blocked_list.iter().any(|p| p.user_id == user_id) {
            self.blocked_list.push(BlockedPeerItem {
                user_id,
                name: name.into(),
                blocked_date: date.into(),
            });
        }
    }

    pub fn unblock_peer(&mut self, user_id: u64) -> bool {
        let initial_len = self.blocked_list.len();
        self.blocked_list.retain(|p| p.user_id != user_id);
        self.blocked_list.len() < initial_len
    }

    pub fn set_search_query(&mut self, query: impl Into<String>) {
        self.search_query = query.into();
    }

    pub fn filtered_peers(&self) -> Vec<&BlockedPeerItem> {
        let q = self.search_query.trim().to_lowercase();
        if q.is_empty() {
            self.blocked_list.iter().collect()
        } else {
            self.blocked_list
                .iter()
                .filter(|p| p.name.to_lowercase().contains(&q))
                .collect()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.blocked_list.is_empty()
    }
}

impl Default for SettingsBlockedPeersSections {
    fn default() -> Self {
        let mut s = Self::new();
        s.block_peer(1001, "Spam Bot", "2026-10-15");
        s.block_peer(1002, "Unknown Caller", "2026-11-01");
        s
    }
}

impl Render for SettingsBlockedPeersSections {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let peers = self.filtered_peers();

        let mut root = div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(20.0))
            .gap(px(16.0))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .font_weight(FontWeight::BOLD)
                            .text_xl()
                            .child("Blocked Users")
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x2481cc))
                            .child("+ Block user")
                    )
            )
            .child(
                div()
                    .p(px(8.0))
                    .bg(rgb(0xf4f4f5))
                    .rounded(px(6.0))
                    .text_xs()
                    .text_color(rgb(0x8e8e93))
                    .child("Blocked users cannot send you messages or add you to groups.")
            );

        if self.is_empty() {
            root = root.child(
                div()
                    .py(px(40.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_sm()
                    .text_color(rgb(0x8e8e93))
                    .child("You haven't blocked anyone yet.")
            );
        } else {
            root = root.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .children(peers.into_iter().map(|peer| {
                        div()
                            .flex()
                            .justify_between()
                            .items_center()
                            .p(px(10.0))
                            .rounded(px(6.0))
                            .bg(rgb(0xfafafa))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(div().font_weight(FontWeight::MEDIUM).child(peer.name.clone()))
                                    .child(div().text_xs().text_color(rgb(0x8e8e93)).child(format!("Blocked on {}", peer.blocked_date)))
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0xcc2424))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Unblock")
                            )
                    }))
            );
        }
        root
    }
}
