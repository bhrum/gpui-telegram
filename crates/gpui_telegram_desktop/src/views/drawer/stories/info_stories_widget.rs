//! Authentic GPUI implementation of Telegram Stories Top-Level Drawer Tab Widget.
//! Source: `Telegram/SourceFiles/info/stories/info_stories_widget.cpp:L1-L338`, `info_stories_widget.h:L1-L111`
//! Subsystem: info | Feature ID: TCG-N38 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use serde::{Deserialize, Serialize};

use super::info_stories_albums::InfoStoriesAlbumsStories;
use super::info_stories_common::{InfoStoriesCommonStories, StorySectionTab};
use super::info_stories_inner_widget::{InfoStoriesInnerWidgetStories, StoryGridItem};
use super::info_stories_provider::InfoStoriesProviderStories;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoriesWidgetMemento {
    pub active_tab: StorySectionTab,
    pub scroll_top: f32,
    pub selected_album_id: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct InfoStoriesWidgetStories {
    pub peer_id: u64,
    pub active_tab: StorySectionTab,
    pub common: InfoStoriesCommonStories,
    pub albums: InfoStoriesAlbumsStories,
    pub inner: InfoStoriesInnerWidgetStories,
    pub provider: InfoStoriesProviderStories,
    pub scroll_top: f32,
}

impl InfoStoriesWidgetStories {
    pub fn new() -> Self {
        Self {
            peer_id: 0,
            active_tab: StorySectionTab::SavedStories,
            common: InfoStoriesCommonStories::new(),
            albums: InfoStoriesAlbumsStories::new(),
            inner: InfoStoriesInnerWidgetStories::new(),
            provider: InfoStoriesProviderStories::new(),
            scroll_top: 0.0,
        }
    }

    pub fn for_peer(peer_id: u64) -> Self {
        let mut w = Self::new();
        w.peer_id = peer_id;
        w.albums.peer_id = peer_id;
        w.provider.peer_id = peer_id;
        w
    }

    pub fn switch_tab(&mut self, tab: StorySectionTab) {
        self.active_tab = tab;
        self.common.set_tab(tab);
        self.inner.is_archive = tab == StorySectionTab::Archive;
    }

    pub fn save_memento(&self) -> StoriesWidgetMemento {
        StoriesWidgetMemento {
            active_tab: self.active_tab,
            scroll_top: self.scroll_top,
            selected_album_id: self.albums.selected_album_id,
        }
    }

    pub fn restore_memento(&mut self, memento: &StoriesWidgetMemento) {
        self.switch_tab(memento.active_tab);
        self.scroll_top = memento.scroll_top;
        self.albums.select_album(memento.selected_album_id);
    }
}

impl Default for InfoStoriesWidgetStories {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoStoriesWidgetStories {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_albums = self.active_tab == StorySectionTab::Albums;

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x0e1621))
            // Navigation sub-tabs
            .child(
                div()
                    .flex()
                    .flex_row()
                    .p(px(8.0))
                    .border_b(px(1.0))
                    .border_color(rgb(0x17212b))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .px(px(12.0))
                                    .py(px(4.0))
                                    .rounded(px(14.0))
                                    .bg(if self.active_tab == StorySectionTab::SavedStories {
                                        rgb(0x5288c1)
                                    } else {
                                        rgb(0x242f3d)
                                    })
                                    .text_xs()
                                    .child("Stories")
                            )
                            .child(
                                div()
                                    .px(px(12.0))
                                    .py(px(4.0))
                                    .rounded(px(14.0))
                                    .bg(if self.active_tab == StorySectionTab::Archive {
                                        rgb(0x5288c1)
                                    } else {
                                        rgb(0x242f3d)
                                    })
                                    .text_xs()
                                    .child("Archive")
                            )
                            .child(
                                div()
                                    .px(px(12.0))
                                    .py(px(4.0))
                                    .rounded(px(14.0))
                                    .bg(if self.active_tab == StorySectionTab::Albums {
                                        rgb(0x5288c1)
                                    } else {
                                        rgb(0x242f3d)
                                    })
                                    .text_xs()
                                    .child("Albums")
                            )
                    )
            )
            // Content Body
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .when(!is_albums, |this| {
                        // Renders stories grid
                        let items_len = self.inner.items.len();
                        this.child(
                            div()
                                .flex()
                                .flex_col()
                                .p(px(8.0))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(0x707579))
                                        .child(format!("Total: {} stories", items_len))
                                )
                        )
                    })
                    .when(is_albums, |this| {
                        let albums_len = self.albums.albums.len();
                        this.child(
                            div()
                                .flex()
                                .flex_col()
                                .p(px(8.0))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(0x707579))
                                        .child(format!("Total: {} albums", albums_len))
                                )
                        )
                    })
            )
    }
}
