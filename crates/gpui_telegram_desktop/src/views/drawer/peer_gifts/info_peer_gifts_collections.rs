//! Authentic GPUI implementation of Telegram Star Gift Collections Manager.
//! Source: `Telegram/SourceFiles/info/peer_gifts/info_peer_gifts_collections.cpp:L1-L158`, `info_peer_gifts_collections.h:L1-L39`
//! Subsystem: info | Feature ID: TCG-N38 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use serde::{Deserialize, Serialize};

/// Upstream constant: `kCollectionNameLimit = 32`
pub const COLLECTION_NAME_LIMIT: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StarGiftCollection {
    pub id: u64,
    pub name: String,
    pub gift_ids: Vec<u64>,
    pub is_pinned: bool,
}

#[derive(Debug, Clone)]
pub struct InfoPeerGiftsCollectionsPeerGifts {
    pub peer_id: u64,
    pub collections: Vec<StarGiftCollection>,
    pub selected_collection_id: Option<u64>,
    pub is_editing: bool,
    pub editing_name: String,
}

impl InfoPeerGiftsCollectionsPeerGifts {
    pub fn new() -> Self {
        Self {
            peer_id: 0,
            collections: Vec::new(),
            selected_collection_id: None,
            is_editing: false,
            editing_name: String::new(),
        }
    }

    pub fn for_peer(peer_id: u64) -> Self {
        let mut s = Self::new();
        s.peer_id = peer_id;
        s
    }

    pub fn set_editing_name(&mut self, name: &str) {
        let trimmed: String = name.chars().take(COLLECTION_NAME_LIMIT).collect();
        self.editing_name = trimmed;
    }

    pub fn create_collection(&mut self, name: String) -> u64 {
        let clean: String = name.trim().chars().take(COLLECTION_NAME_LIMIT).collect();
        let final_name = if clean.is_empty() {
            "Untitled Collection".to_string()
        } else {
            clean
        };
        let new_id = (self.collections.len() as u64) + 1;
        self.collections.push(StarGiftCollection {
            id: new_id,
            name: final_name,
            gift_ids: Vec::new(),
            is_pinned: false,
        });
        self.selected_collection_id = Some(new_id);
        new_id
    }

    pub fn delete_collection(&mut self, id: u64) {
        self.collections.retain(|c| c.id != id);
        if self.selected_collection_id == Some(id) {
            self.selected_collection_id = None;
        }
    }

    pub fn pin_collection(&mut self, id: u64, pinned: bool) {
        if let Some(col) = self.collections.iter_mut().find(|c| c.id == id) {
            col.is_pinned = pinned;
        }
    }
}

impl Default for InfoPeerGiftsCollectionsPeerGifts {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoPeerGiftsCollectionsPeerGifts {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_empty = self.collections.is_empty();

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(12.0))
            .gap(px(8.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("Gift Collections")
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x707579))
                            .child(format!("{} collections", self.collections.len()))
                    )
            )
            .when(is_empty, |this| {
                this.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .py(px(24.0))
                        .text_xs()
                        .text_color(rgb(0x707579))
                        .child("No collections created yet")
                )
            })
            .when(!is_empty, |this| {
                let mut col = div().flex().flex_col().gap(px(4.0));
                for item in &self.collections {
                    let is_active = self.selected_collection_id == Some(item.id);
                    col = col.child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .p(px(8.0))
                            .rounded(px(6.0))
                            .bg(if is_active { rgb(0x242f3d) } else { rgb(0x17212b) })
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap(px(8.0))
                                    .child(div().text_sm().child("🎁"))
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .child(item.name.clone())
                                    )
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x707579))
                                    .child(format!("{} gifts", item.gift_ids.len()))
                            )
                    );
                }
                this.child(col)
            })
    }
}
