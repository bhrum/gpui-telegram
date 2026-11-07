//! Info Community Widget view component
//! Upstream: `Telegram/SourceFiles/info/community/info_community_widget.cpp`
//! Upstream: `Telegram/SourceFiles/info/community/info_community_widget.h`
//! Subsystem: info | Feature ID: TCG-N23 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use serde::{Deserialize, Serialize};

/// Upstream `Info::Community::Memento` section memento.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommunityMemento {
    pub peer_id: i64,
    pub scroll_top: i32,
}

impl CommunityMemento {
    pub fn new(peer_id: i64) -> Self {
        Self {
            peer_id,
            scroll_top: 0,
        }
    }
}

/// Upstream `Info::Community::Widget` and `InfoCommunityWidgetCommunity`.
#[derive(Debug, Clone)]
pub struct InfoCommunityWidgetCommunity {
    pub peer_id: i64,
    pub title: String,
    pub description: String,
    pub linked_chat_count: usize,
    pub member_count: usize,
    pub is_loading: bool,
    pub back_enabled: bool,
    pub memento: Option<CommunityMemento>,
}

impl InfoCommunityWidgetCommunity {
    pub fn new(peer_id: i64, title: impl Into<String>) -> Self {
        Self {
            peer_id,
            title: title.into(),
            description: String::new(),
            linked_chat_count: 0,
            member_count: 0,
            is_loading: false,
            back_enabled: false,
            memento: Some(CommunityMemento::new(peer_id)),
        }
    }

    pub fn set_description(&mut self, description: impl Into<String>) {
        self.description = description.into();
    }

    pub fn set_counts(&mut self, chats: usize, members: usize) {
        self.linked_chat_count = chats;
        self.member_count = members;
    }

    pub fn enable_back_button(&mut self) {
        self.back_enabled = true;
    }

    pub fn show_finished(&mut self) {
        self.is_loading = false;
    }

    /// Upstream `chatsStatusValue`: format linked chats and members summary.
    pub fn chats_status_text(&self) -> String {
        if self.linked_chat_count == 1 {
            format!("1 linked chat, {} members", self.member_count)
        } else {
            format!("{} linked chats, {} members", self.linked_chat_count, self.member_count)
        }
    }
}

impl Default for InfoCommunityWidgetCommunity {
    fn default() -> Self {
        Self::new(0, "Community")
    }
}

impl Render for InfoCommunityWidgetCommunity {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let status = self.chats_status_text();

        let mut root = div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xFFFFFF))
            .p(px(16.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(12.0))
                    .child(
                        div()
                            .size(px(48.0))
                            .rounded_full()
                            .bg(rgb(0x2AABEE))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_color(rgb(0xFFFFFF))
                            .font_weight(FontWeight::BOLD)
                            .child(self.title.chars().next().unwrap_or('C').to_string())
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0x222222))
                                    .child(self.title.clone())
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0x707579))
                                    .child(status)
                            )
                    )
            );

        if !self.description.is_empty() {
            root = root.child(
                div()
                    .pt(px(12.0))
                    .text_sm()
                    .text_color(rgb(0x222222))
                    .child(self.description.clone())
            );
        }

        root
    }
}
