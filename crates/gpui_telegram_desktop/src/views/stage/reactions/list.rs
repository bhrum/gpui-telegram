//! Who Reacted List Popover & Detailed Breakdown
//! Faithful upstream C++ extraction:
//! - `history_view_reactions_list.cpp:L38-L120`

use gpui::*;
use super::button::ReactionKey;

pub const K_PER_PAGE_FIRST: usize = 20;
pub const K_PER_PAGE: usize = 100;

#[derive(Debug, Clone)]
pub struct ReactorItem {
    pub peer_id: u64,
    pub name: String,
    pub avatar_url: Option<String>,
    pub reaction: ReactionKey,
    pub date: i64,
    pub paid_stars: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct WhoReactedList {
    pub items: Vec<ReactorItem>,
    pub active_tab: Option<ReactionKey>, // None = All
    pub can_moderate: bool,
    pub is_loading: bool,
}

impl WhoReactedList {
    pub fn new(items: Vec<ReactorItem>, can_moderate: bool) -> Self {
        Self {
            items,
            active_tab: None,
            can_moderate,
            is_loading: false,
        }
    }

    pub fn filter_by_tab(&self) -> Vec<&ReactorItem> {
        match &self.active_tab {
            None => self.items.iter().collect(),
            Some(tab) => self.items.iter().filter(|i| &i.reaction == tab).collect(),
        }
    }

    pub fn can_moderate_by_delete_messages(&self) -> bool {
        self.can_moderate
    }
}

impl Default for WhoReactedList {
    fn default() -> Self {
        Self::new(Vec::new(), false)
    }
}

impl Render for WhoReactedList {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let entries = self.filter_by_tab();

        div()
            .flex()
            .flex_col()
            .w(px(320.0))
            .max_h(px(400.0))
            .p(px(8.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(12.0))
            .shadow_lg()
            .border_1()
            .border_color(rgb(0xE0E0E0))
            // Header
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .pb(px(8.0))
                    .border_b_1()
                    .border_color(rgb(0xF0F0F0))
                    .child(
                        div()
                            .font_weight(FontWeight::BOLD)
                            .text_size(px(14.0))
                            .text_color(rgb(0x222222))
                            .child("Reactions")
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(0x888888))
                            .child(format!("{} reactors", self.items.len()))
                    )
            )
            // Scrollable list
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .pt(px(6.0))
                    .children(entries.into_iter().map(|item| {
                        let reaction_str = item.reaction.display_label();
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .p(px(6.0))
                            .rounded(px(6.0))
                            .hover(|s| s.bg(rgba(0x00000006)))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .w(px(28.0))
                                            .h(px(28.0))
                                            .rounded_full()
                                            .bg(rgb(0x2AABEE))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_size(px(12.0))
                                            .text_color(rgb(0xFFFFFF))
                                            .child(item.name.chars().next().unwrap_or('?').to_string())
                                    )
                                    .child(
                                        {
                                            let mut info = div()
                                                .flex()
                                                .flex_col()
                                                .child(
                                                    div()
                                                        .text_size(px(13.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(rgb(0x222222))
                                                        .child(item.name.clone())
                                                );
                                            if let Some(stars) = item.paid_stars {
                                                info = info.child(
                                                    div()
                                                        .text_size(px(11.0))
                                                        .text_color(rgb(0xFFA500))
                                                        .child(format!("⭐ {} Stars", stars))
                                                );
                                            }
                                            info
                                        }
                                    )
                            )
                            .child(
                                div()
                                    .text_size(px(16.0))
                                    .child(reaction_str)
                            )
                    }))
            )
    }
}
