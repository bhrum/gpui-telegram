//! Authentic GPUI implementation of Telegram Stories Album Management.
//! Source: `Telegram/SourceFiles/info/stories/info_stories_albums.cpp:L1-L150`, `info_stories_albums.h:L1-L39`
//! Subsystem: info | Feature ID: TCG-N38 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use serde::{Deserialize, Serialize};

/// Maximum characters allowed for story album name (`kAlbumNameLimit = 32`).
pub const ALBUM_NAME_LIMIT: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoryAlbumItem {
    pub id: u64,
    pub title: String,
    pub cover_story_id: u64,
    pub stories_count: usize,
    pub created_at: i64,
}

#[derive(Debug, Clone)]
pub struct InfoStoriesAlbumsStories {
    pub peer_id: u64,
    pub albums: Vec<StoryAlbumItem>,
    pub selected_album_id: Option<u64>,
    pub is_editing: bool,
    pub editing_title: String,
    pub search_query: String,
}

impl InfoStoriesAlbumsStories {
    pub fn new() -> Self {
        Self {
            peer_id: 0,
            albums: Vec::new(),
            selected_album_id: None,
            is_editing: false,
            editing_title: String::new(),
            search_query: String::new(),
        }
    }

    pub fn with_peer(peer_id: u64) -> Self {
        let mut s = Self::new();
        s.peer_id = peer_id;
        s
    }

    /// Sets album title enforcing `kAlbumNameLimit = 32` upstream boundary.
    pub fn set_editing_title(&mut self, title: &str) {
        let trimmed: String = title.chars().take(ALBUM_NAME_LIMIT).collect();
        self.editing_title = trimmed;
    }

    pub fn create_album(&mut self, title: String, cover_story_id: u64) -> u64 {
        let clean_title: String = title.trim().chars().take(ALBUM_NAME_LIMIT).collect();
        let final_title = if clean_title.is_empty() {
            "Untitled Album".to_string()
        } else {
            clean_title
        };
        let new_id = (self.albums.len() as u64) + 1;
        self.albums.push(StoryAlbumItem {
            id: new_id,
            title: final_title,
            cover_story_id,
            stories_count: 0,
            created_at: 0,
        });
        self.selected_album_id = Some(new_id);
        new_id
    }

    pub fn delete_album(&mut self, album_id: u64) {
        self.albums.retain(|a| a.id != album_id);
        if self.selected_album_id == Some(album_id) {
            self.selected_album_id = None;
        }
    }

    pub fn select_album(&mut self, album_id: Option<u64>) {
        self.selected_album_id = album_id;
    }

    pub fn toggle_editing(&mut self) {
        self.is_editing = !self.is_editing;
    }

    pub fn filtered_albums(&self) -> Vec<&StoryAlbumItem> {
        if self.search_query.trim().is_empty() {
            self.albums.iter().collect()
        } else {
            let q = self.search_query.to_lowercase();
            self.albums
                .iter()
                .filter(|a| a.title.to_lowercase().contains(&q))
                .collect()
        }
    }
}

impl Default for InfoStoriesAlbumsStories {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoStoriesAlbumsStories {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let albums = self.filtered_albums();
        let is_empty = albums.is_empty();

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .gap(px(12.0))
            // Header Bar
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_base()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("Story Albums")
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x707579))
                            .child(format!("{} albums", self.albums.len()))
                    )
            )
            // Empty state or Grid of albums
            .when(is_empty, |this| {
                this.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .py(px(32.0))
                        .text_sm()
                        .text_color(rgb(0x707579))
                        .child("No story albums yet")
                )
            })
            .when(!is_empty, |this| {
                let mut grid = div().flex().flex_wrap().gap(px(8.0));
                for album in albums {
                    grid = grid.child(
                        div()
                            .flex()
                            .flex_col()
                            .w(px(120.0))
                            .p(px(8.0))
                            .rounded(px(8.0))
                            .bg(rgb(0x2b5278))
                            .child(
                                div()
                                    .h(px(80.0))
                                    .rounded(px(6.0))
                                    .bg(rgb(0x17212b))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_xs()
                                    .child(format!("{} stories", album.stories_count))
                            )
                            .child(
                                div()
                                    .mt(px(4.0))
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .child(album.title.clone())
                            )
                    );
                }
                this.child(grid)
            })
    }
}
