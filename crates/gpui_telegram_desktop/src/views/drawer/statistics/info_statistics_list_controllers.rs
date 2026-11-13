//! Statistics Sub-List Controllers for Forwards and Active Members
//! Upstream: `Telegram/SourceFiles/info/statistics/info_statistics_list_controllers.cpp:L1-L1608`
//! Upstream: `Telegram/SourceFiles/info/statistics/info_statistics_list_controllers.h:L1-L75`
//! Subsystem: info | Feature ID: TCG-N12 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// Computes unique deterministic row ID from entry parameters matching upstream C++:
/// `entry.id + (refunded ? '1':'0') + (pending ? '1':'0') + (failed ? '1':'0') + (in ? '1':'0') + (giftOffer ? '1':'0')`
pub fn unique_row_id_from_entry(
    id: &str,
    refunded: bool,
    pending: bool,
    failed: bool,
    is_in: bool,
    gift_offer: bool,
) -> u64 {
    let key = format!(
        "{}{}{}{}{}{}",
        id,
        if refunded { '1' } else { '0' },
        if pending { '1' } else { '0' },
        if failed { '1' } else { '0' },
        if is_in { '1' } else { '0' },
        if gift_offer { '1' } else { '0' },
    );
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish()
}

#[derive(Debug, Clone)]
pub struct PublicForwardRow {
    pub peer_id: u64,
    pub title: String,
    pub views_count: usize,
    pub date: String,
}

#[derive(Debug, Clone)]
pub struct TopMemberRow {
    pub user_id: u64,
    pub name: String,
    pub messages_count: usize,
    pub reactions_count: usize,
}

#[derive(Debug, Clone)]
pub struct InfoStatisticsListControllersStatistics {
    pub forwards: Vec<PublicForwardRow>,
    pub top_members: Vec<TopMemberRow>,
    pub limit: usize,
    pub is_visible: bool,
}

impl InfoStatisticsListControllersStatistics {
    pub fn new() -> Self {
        let mut s = Self {
            forwards: Vec::new(),
            top_members: Vec::new(),
            limit: 20,
            is_visible: true,
        };
        s.init_sample_data();
        s
    }

    pub fn init_sample_data(&mut self) {
        self.forwards = vec![
            PublicForwardRow {
                peer_id: 301,
                title: "Tech Chronicle".into(),
                views_count: 14_200,
                date: "Nov 5".into(),
            },
            PublicForwardRow {
                peer_id: 302,
                title: "Open Source Daily".into(),
                views_count: 8_940,
                date: "Nov 4".into(),
            },
        ];
        self.top_members = vec![
            TopMemberRow {
                user_id: 401,
                name: "Sarah".into(),
                messages_count: 342,
                reactions_count: 1_280,
            },
            TopMemberRow {
                user_id: 402,
                name: "David".into(),
                messages_count: 215,
                reactions_count: 870,
            },
        ];
    }

    pub fn set_limit(&mut self, limit: usize) {
        self.limit = limit;
    }
}

impl Default for InfoStatisticsListControllersStatistics {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoStatisticsListControllersStatistics {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .gap_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .child("Public Shares & Forwards"),
                    )
                    .children(self.forwards.iter().map(|f| {
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .p(px(8.0))
                            .rounded_md()
                            .bg(rgb(0x1e2c3a))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(div().text_sm().font_weight(FontWeight::SEMIBOLD).text_color(rgb(0xffffff)).child(f.title.clone()))
                                    .child(div().text_xs().text_color(rgb(0x8e8e93)).child(f.date.clone())),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0x0088cc))
                                    .font_weight(FontWeight::BOLD)
                                    .child(format!("{} views", f.views_count)),
                            )
                    })),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .child("Top Active Members"),
                    )
                    .children(self.top_members.iter().map(|m| {
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .p(px(8.0))
                            .rounded_md()
                            .bg(rgb(0x1e2c3a))
                            .child(div().text_sm().font_weight(FontWeight::SEMIBOLD).text_color(rgb(0xffffff)).child(m.name.clone()))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x8e8e93))
                                    .child(format!("{} msgs • {} reacts", m.messages_count, m.reactions_count)),
                            )
                    })),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_row_id() {
        let id1 = unique_row_id_from_entry("tx_1", false, false, false, true, false);
        let id2 = unique_row_id_from_entry("tx_1", true, false, false, true, false);
        let id3 = unique_row_id_from_entry("tx_1", false, false, false, true, false);
        assert_ne!(id1, id2);
        assert_eq!(id1, id3);
    }
}
