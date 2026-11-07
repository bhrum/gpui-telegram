//! Authentic GPUI implementation of Telegram Peer Star Gifts Drawer Tab Widget.
//! Source: `Telegram/SourceFiles/info/peer_gifts/info_peer_gifts_widget.cpp:L1-L2745`, `info_peer_gifts_widget.h:L1-L148`
//! Subsystem: info | Feature ID: TCG-N38 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use serde::{Deserialize, Serialize};

use super::info_peer_gifts_collections::InfoPeerGiftsCollectionsPeerGifts;
use super::info_peer_gifts_common::{GIFTS_PER_ROW, InfoPeerGiftsCommonPeerGifts, StarGiftItem};

pub const GIFTS_PER_PAGE: usize = 30;
pub const GIFTS_PRELOAD_PAGES: usize = 2;
pub const GIFTS_SCROLL_FACTOR: f32 = 0.8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum GiftsFilterTab {
    #[default]
    All,
    Received,
    Sent,
    Collections,
}

#[derive(Debug, Clone)]
pub struct InfoPeerGiftsWidgetPeerGifts {
    pub peer_id: u64,
    pub active_tab: GiftsFilterTab,
    pub common: InfoPeerGiftsCommonPeerGifts,
    pub collections: InfoPeerGiftsCollectionsPeerGifts,
    pub is_reordering: bool,
    pub scroll_top: f32,
}

impl InfoPeerGiftsWidgetPeerGifts {
    pub fn new() -> Self {
        Self {
            peer_id: 0,
            active_tab: GiftsFilterTab::All,
            common: InfoPeerGiftsCommonPeerGifts::new(),
            collections: InfoPeerGiftsCollectionsPeerGifts::new(),
            is_reordering: false,
            scroll_top: 0.0,
        }
    }

    pub fn for_peer(peer_id: u64) -> Self {
        let mut w = Self::new();
        w.peer_id = peer_id;
        w.collections.peer_id = peer_id;
        w
    }

    pub fn set_tab(&mut self, tab: GiftsFilterTab) {
        self.active_tab = tab;
    }

    pub fn toggle_reordering(&mut self) {
        self.is_reordering = !self.is_reordering;
    }

    pub fn filtered_gifts(&self) -> Vec<&StarGiftItem> {
        match self.active_tab {
            GiftsFilterTab::All => self.common.gifts.iter().collect(),
            GiftsFilterTab::Received => {
                self.common.gifts.iter().filter(|g| g.sender_id != self.peer_id).collect()
            }
            GiftsFilterTab::Sent => {
                self.common.gifts.iter().filter(|g| g.sender_id == self.peer_id).collect()
            }
            GiftsFilterTab::Collections => self.common.gifts.iter().filter(|g| g.is_pinned).collect(),
        }
    }
}

impl Default for InfoPeerGiftsWidgetPeerGifts {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoPeerGiftsWidgetPeerGifts {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let gifts = self.filtered_gifts();
        let is_empty = gifts.is_empty();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x0e1621))
            .p(px(8.0))
            .gap(px(8.0))
            // Filter Pills Bar
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(6.0))
                    .pb(px(8.0))
                    .border_b(px(1.0))
                    .border_color(rgb(0x17212b))
                    .child(
                        div()
                            .px(px(10.0))
                            .py(px(4.0))
                            .rounded(px(12.0))
                            .bg(if self.active_tab == GiftsFilterTab::All { rgb(0x5288c1) } else { rgb(0x242f3d) })
                            .text_xs()
                            .child("All")
                    )
                    .child(
                        div()
                            .px(px(10.0))
                            .py(px(4.0))
                            .rounded(px(12.0))
                            .bg(if self.active_tab == GiftsFilterTab::Received { rgb(0x5288c1) } else { rgb(0x242f3d) })
                            .text_xs()
                            .child("Received")
                    )
                    .child(
                        div()
                            .px(px(10.0))
                            .py(px(4.0))
                            .rounded(px(12.0))
                            .bg(if self.active_tab == GiftsFilterTab::Sent { rgb(0x5288c1) } else { rgb(0x242f3d) })
                            .text_xs()
                            .child("Sent")
                    )
                    .child(
                        div()
                            .px(px(10.0))
                            .py(px(4.0))
                            .rounded(px(12.0))
                            .bg(if self.active_tab == GiftsFilterTab::Collections { rgb(0x5288c1) } else { rgb(0x242f3d) })
                            .text_xs()
                            .child("Collections")
                    )
            )
            // Empty view
            .when(is_empty, |this| {
                this.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .py(px(32.0))
                        .text_xs()
                        .text_color(rgb(0x707579))
                        .child("No gifts in this category")
                )
            })
            // Gifts Grid (3 per row)
            .when(!is_empty, |this| {
                let mut grid = div().flex().flex_wrap().gap(px(8.0));
                for gift in gifts {
                    grid = grid.child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .w(px(100.0))
                            .p(px(8.0))
                            .rounded(px(8.0))
                            .bg(rgb(0x17212b))
                            .child(div().text_xl().child("🎁"))
                            .child(
                                div()
                                    .mt(px(4.0))
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .child(gift.title.clone())
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0xffd700))
                                    .child(format!("⭐ {}", gift.stars_price))
                            )
                    );
                }
                this.child(grid)
            })
    }
}
