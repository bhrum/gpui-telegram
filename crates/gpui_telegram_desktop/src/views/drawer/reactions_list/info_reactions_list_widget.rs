//! Authentic GPUI implementation of Telegram Message Reactions List Viewer.
//! Source: `Telegram/SourceFiles/info/reactions_list/info_reactions_list_widget.cpp:L1-L351`, `info_reactions_list_widget.h:L1-L82`
//! Subsystem: info | Feature ID: TCG-N14 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactionReactorItem {
    pub user_id: u64,
    pub name: String,
    pub avatar_initials: String,
    pub avatar_color: u32,
    pub reaction_emoji: String,
    pub date: i64,
    pub is_read: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactionCountChip {
    pub emoji: String,
    pub count: usize,
}

#[derive(Debug, Clone)]
pub struct InfoReactionsListWidgetReactionsList {
    pub message_id: u64,
    pub peer_id: u64,
    pub reactors: Vec<ReactionReactorItem>,
    pub selected_filter_emoji: Option<String>,
    pub who_read_ids: Vec<u64>,
}

impl InfoReactionsListWidgetReactionsList {
    pub fn new() -> Self {
        Self {
            message_id: 0,
            peer_id: 0,
            reactors: Vec::new(),
            selected_filter_emoji: None,
            who_read_ids: Vec::new(),
        }
    }

    pub fn for_message(peer_id: u64, message_id: u64) -> Self {
        let mut w = Self::new();
        w.peer_id = peer_id;
        w.message_id = message_id;
        w
    }

    pub fn add_reactor(&mut self, reactor: ReactionReactorItem) {
        if !self.reactors.iter().any(|r| r.user_id == reactor.user_id && r.reaction_emoji == reactor.reaction_emoji) {
            self.reactors.push(reactor);
        }
    }

    pub fn set_filter_emoji(&mut self, emoji: Option<String>) {
        self.selected_filter_emoji = emoji;
    }

    pub fn reaction_counts(&self) -> Vec<ReactionCountChip> {
        let mut map: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
        for r in &self.reactors {
            *map.entry(r.reaction_emoji.clone()).or_insert(0) += 1;
        }
        map.into_iter()
            .map(|(emoji, count)| ReactionCountChip { emoji, count })
            .collect()
    }

    pub fn filtered_reactors(&self) -> Vec<&ReactionReactorItem> {
        if let Some(ref filter) = self.selected_filter_emoji {
            self.reactors.iter().filter(|r| &r.reaction_emoji == filter).collect()
        } else {
            self.reactors.iter().collect()
        }
    }

    pub fn mark_read(&mut self, user_id: u64) {
        if !self.who_read_ids.contains(&user_id) {
            self.who_read_ids.push(user_id);
        }
        for r in self.reactors.iter_mut().filter(|r| r.user_id == user_id) {
            r.is_read = true;
        }
    }
}

impl Default for InfoReactionsListWidgetReactionsList {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoReactionsListWidgetReactionsList {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let counts = self.reaction_counts();
        let total = self.reactors.len();
        let list = self.filtered_reactors();
        let is_empty = list.is_empty();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x0e1621))
            .p(px(8.0))
            // Reactions Filter Bar
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(6.0))
                    .pb(px(8.0))
                    .border_b(px(1.0))
                    .border_color(rgb(0x17212b))
                    .child(
                        div()
                            .px(px(10.0))
                            .py(px(4.0))
                            .rounded(px(12.0))
                            .bg(if self.selected_filter_emoji.is_none() {
                                rgb(0x5288c1)
                            } else {
                                rgb(0x242f3d)
                            })
                            .text_xs()
                            .child(format!("All {}", total))
                    )
            )
            // Reactor rows
            .when(is_empty, |this| {
                this.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .py(px(32.0))
                        .text_xs()
                        .text_color(rgb(0x707579))
                        .child("No reactions yet")
                )
            })
            .when(!is_empty, |this| {
                let mut col = div().flex().flex_col().gap(px(4.0)).pt(px(6.0));
                for r in list {
                    col = col.child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .p(px(6.0))
                            .rounded(px(6.0))
                            .bg(rgb(0x17212b))
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .w(px(32.0))
                                            .h(px(32.0))
                                            .rounded_full()
                                            .bg(rgb(r.avatar_color))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_xs()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0xffffff))
                                            .child(r.avatar_initials.clone())
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .child(r.name.clone())
                                            )
                                    )
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap(px(4.0))
                                    .child(div().text_base().child(r.reaction_emoji.clone()))
                            )
                    );
                }
                this.child(col)
            })
    }
}
