//! Info Profile Tab Peer Lists Adapter
//! Upstream: `Telegram/SourceFiles/info/profile/tabs/adapters/info_profile_tab_peer_lists.cpp:L1-L244`
//! Upstream: `Telegram/SourceFiles/info/profile/tabs/adapters/info_profile_tab_peer_lists.h:L1-L23`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use gpui::*;
use serde::{Deserialize, Serialize};

/// Peer row entry within gifts collections or similar peers lists.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerRowItem {
    pub peer_id: i64,
    pub name: String,
    pub role: String,
    pub badge: Option<String>,
    pub action_text: String,
    pub is_action_active: bool,
}

/// Tab adapter managing gifts collections and similar peers list displays.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoProfileTabPeerListsProfileTabsAdapters {
    pub peer_id: i64,
    pub list_type: String,
    pub items_count: usize,
    pub is_active: bool,
    pub search_filter: String,
    pub rows: Vec<PeerRowItem>,
}

impl Default for InfoProfileTabPeerListsProfileTabsAdapters {
    fn default() -> Self {
        Self::new(0, "Peers")
    }
}

impl InfoProfileTabPeerListsProfileTabsAdapters {
    pub fn new(peer_id: i64, list_type: impl Into<String>) -> Self {
        Self {
            peer_id,
            list_type: list_type.into(),
            items_count: 0,
            is_active: false,
            search_filter: String::new(),
            rows: Vec::new(),
        }
    }

    pub fn set_count(&mut self, count: usize) {
        self.items_count = count;
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn set_search_filter(&mut self, filter: impl Into<String>) {
        self.search_filter = filter.into();
    }

    pub fn add_row(&mut self, row: PeerRowItem) {
        self.rows.push(row);
        self.items_count = self.rows.len();
    }

    pub fn remove_row(&mut self, peer_id: i64) -> bool {
        let initial_len = self.rows.len();
        self.rows.retain(|r| r.peer_id != peer_id);
        let removed = self.rows.len() < initial_len;
        if removed {
            self.items_count = self.rows.len();
        }
        removed
    }

    pub fn filtered_rows(&self) -> Vec<&PeerRowItem> {
        let q = self.search_filter.trim().to_lowercase();
        if q.is_empty() {
            self.rows.iter().collect()
        } else {
            self.rows
                .iter()
                .filter(|r| r.name.to_lowercase().contains(&q) || r.role.to_lowercase().contains(&q))
                .collect()
        }
    }
}

impl Render for InfoProfileTabPeerListsProfileTabsAdapters {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let filtered = self.filtered_rows();
        let is_empty = filtered.is_empty();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xFFFFFF))
            .p(px(8.0))
            .gap(px(6.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .px(px(4.0))
                    .py(px(4.0))
                    .border_b_1()
                    .border_color(rgb(0xEBEEF2))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x222222))
                            .child(self.list_type.clone())
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x707579))
                            .child(format!("{} entries", self.items_count))
                    )
            )
            .child(
                if is_empty {
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .h(px(140.0))
                        .text_sm()
                        .text_color(rgb(0x707579))
                        .child(format!("No {} available", self.list_type))
                } else {
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .children(filtered.into_iter().map(|row| {
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .justify_between()
                                .p(px(6.0))
                                .rounded_sm()
                                .hover(|s| s.bg(rgb(0xF5F7FA)))
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap(px(10.0))
                                        .child(
                                            div()
                                                .size(px(36.0))
                                                .rounded_full()
                                                .bg(rgb(0x2AABEE))
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .text_color(rgb(0xFFFFFF))
                                                .font_weight(FontWeight::BOLD)
                                                .text_sm()
                                                .child(row.name.chars().next().unwrap_or('?').to_string())
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .child(
                                                    div()
                                                        .flex()
                                                        .flex_row()
                                                        .items_center()
                                                        .gap(px(4.0))
                                                        .child(
                                                            div()
                                                                .text_sm()
                                                                .font_weight(FontWeight::MEDIUM)
                                                                .text_color(rgb(0x222222))
                                                                .child(row.name.clone())
                                                        )
                                                        .children(row.badge.as_ref().map(|b| {
                                                            div()
                                                                .px(px(4.0))
                                                                .rounded_xs()
                                                                .bg(rgb(0xFEF2D6))
                                                                .text_xs()
                                                                .text_color(rgb(0xB78103))
                                                                .child(b.clone())
                                                        }))
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(rgb(0x707579))
                                                        .child(row.role.clone())
                                                )
                                        )
                                )
                                .child(
                                    div()
                                        .px(px(10.0))
                                        .py(px(4.0))
                                        .rounded_sm()
                                        .bg(if row.is_action_active { rgb(0x2AABEE) } else { rgb(0xF0F2F5) })
                                        .text_color(if row.is_action_active { rgb(0xFFFFFF) } else { rgb(0x2AABEE) })
                                        .text_xs()
                                        .cursor_pointer()
                                        .child(row.action_text.clone())
                                )
                        }))
                }
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peer_lists_adapter_management() {
        let mut adapter = InfoProfileTabPeerListsProfileTabsAdapters::new(5005, "Star Gifts");
        assert_eq!(adapter.peer_id, 5005);
        assert_eq!(adapter.list_type, "Star Gifts");
        assert_eq!(adapter.items_count, 0);

        adapter.add_row(PeerRowItem {
            peer_id: 11,
            name: "Alice Walker".into(),
            role: "Sender".into(),
            badge: Some("★ Gift #42".into()),
            action_text: "View".into(),
            is_action_active: true,
        });
        adapter.add_row(PeerRowItem {
            peer_id: 12,
            name: "Bob Stone".into(),
            role: "Receiver".into(),
            badge: None,
            action_text: "Transfer".into(),
            is_action_active: false,
        });

        assert_eq!(adapter.items_count, 2);
        assert_eq!(adapter.filtered_rows().len(), 2);

        adapter.set_search_filter("Alice");
        assert_eq!(adapter.filtered_rows().len(), 1);
        assert_eq!(adapter.filtered_rows()[0].name, "Alice Walker");

        adapter.set_search_filter("");
        assert!(adapter.remove_row(11));
        assert_eq!(adapter.items_count, 1);
        assert!(!adapter.remove_row(999));
    }
}
