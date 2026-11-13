//! Common models and metadata tags for Stories Drawer Views.
//! Source: `Telegram/SourceFiles/info/stories/info_stories_common.h:L1-L29`
//! Subsystem: info | Feature ID: TCG-N38 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum StorySectionTab {
    #[default]
    SavedStories,
    Archive,
    Albums,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoryAlbumTag {
    pub album_id: u64,
    pub title: String,
    pub count: usize,
}

#[derive(Debug, Clone)]
pub struct InfoStoriesCommonStories {
    pub active_tab: StorySectionTab,
    pub target_album_id: Option<u64>,
    pub available_tags: Vec<StoryAlbumTag>,
    pub is_archive_enabled: bool,
}

impl InfoStoriesCommonStories {
    pub fn new() -> Self {
        Self {
            active_tab: StorySectionTab::SavedStories,
            target_album_id: None,
            available_tags: Vec::new(),
            is_archive_enabled: false,
        }
    }

    pub fn set_tab(&mut self, tab: StorySectionTab) {
        self.active_tab = tab;
    }

    /// Sets the album ID currently being populated (`addingToAlbumId`).
    pub fn set_adding_to_album_id(&mut self, album_id: Option<u64>) {
        self.target_album_id = album_id;
    }

    pub fn register_tag(&mut self, album_id: u64, title: String, count: usize) {
        if let Some(existing) = self.available_tags.iter_mut().find(|t| t.album_id == album_id) {
            existing.title = title;
            existing.count = count;
        } else {
            self.available_tags.push(StoryAlbumTag {
                album_id,
                title,
                count,
            });
        }
    }

    pub fn remove_tag(&mut self, album_id: u64) {
        self.available_tags.retain(|t| t.album_id != album_id);
    }
}

impl Default for InfoStoriesCommonStories {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoStoriesCommonStories {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .p(px(8.0))
            .child(
                div()
                    .px(px(10.0))
                    .py(px(4.0))
                    .rounded(px(12.0))
                    .bg(if self.active_tab == StorySectionTab::SavedStories {
                        rgb(0x5288c1)
                    } else {
                        rgb(0x242f3d)
                    })
                    .text_xs()
                    .child("Saved Stories")
            )
            .child(
                div()
                    .px(px(10.0))
                    .py(px(4.0))
                    .rounded(px(12.0))
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
                    .px(px(10.0))
                    .py(px(4.0))
                    .rounded(px(12.0))
                    .bg(if self.active_tab == StorySectionTab::Albums {
                        rgb(0x5288c1)
                    } else {
                        rgb(0x242f3d)
                    })
                    .text_xs()
                    .child("Albums")
            )
    }
}
