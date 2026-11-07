//! Authentic GPUI implementation of Telegram Stories Grid & Selection Inner Widget.
//! Source: `Telegram/SourceFiles/info/stories/info_stories_inner_widget.cpp:L1-L1153`, `info_stories_inner_widget.h:L1-L180`
//! Subsystem: info | Feature ID: TCG-N38 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub const INFO_MEDIA_MIN_GRID_SIZE: f32 = 100.0;
pub const INFO_MEDIA_SKIP: f32 = 4.0;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoryGridItem {
    pub id: u64,
    pub date: i64,
    pub views_count: usize,
    pub reactions_count: usize,
    pub is_pinned: bool,
    pub caption: String,
}

#[derive(Debug, Clone)]
pub struct InfoStoriesInnerWidgetStories {
    pub items: Vec<StoryGridItem>,
    pub selected_ids: HashSet<u64>,
    pub is_selecting: bool,
    pub is_loading: bool,
    pub is_archive: bool,
}

impl InfoStoriesInnerWidgetStories {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            selected_ids: HashSet::new(),
            is_selecting: false,
            is_loading: false,
            is_archive: false,
        }
    }

    pub fn with_stories(items: Vec<StoryGridItem>, is_archive: bool) -> Self {
        Self {
            items,
            selected_ids: HashSet::new(),
            is_selecting: false,
            is_loading: false,
            is_archive,
        }
    }

    pub fn toggle_selection(&mut self, story_id: u64) {
        if self.selected_ids.contains(&story_id) {
            self.selected_ids.remove(&story_id);
            if self.selected_ids.is_empty() {
                self.is_selecting = false;
            }
        } else {
            self.selected_ids.insert(story_id);
            self.is_selecting = true;
        }
    }

    pub fn select_all(&mut self) {
        self.selected_ids = self.items.iter().map(|i| i.id).collect();
        self.is_selecting = !self.items.is_empty();
    }

    pub fn clear_selection(&mut self) {
        self.selected_ids.clear();
        self.is_selecting = false;
    }

    pub fn delete_selected(&mut self) -> Vec<u64> {
        let deleted: Vec<u64> = self.selected_ids.iter().copied().collect();
        self.items.retain(|i| !self.selected_ids.contains(&i.id));
        self.clear_selection();
        deleted
    }

    pub fn compute_columns(&self, available_width: f32) -> usize {
        let cols = ((available_width + INFO_MEDIA_SKIP) / (INFO_MEDIA_MIN_GRID_SIZE + INFO_MEDIA_SKIP)).floor() as usize;
        cols.max(1)
    }
}

impl Default for InfoStoriesInnerWidgetStories {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoStoriesInnerWidgetStories {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_empty = self.items.is_empty();
        let selected_count = self.selected_ids.len();

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(8.0))
            // Selection action toolbar
            .when(self.is_selecting, |this| {
                this.child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .p(px(8.0))
                        .rounded(px(6.0))
                        .bg(rgb(0x242f3d))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::SEMIBOLD)
                                .child(format!("{} selected", selected_count))
                        )
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .gap(px(8.0))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(0x5288c1))
                                        .child("Archive")
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(0xec3942))
                                        .child("Delete")
                                )
                        )
                )
            })
            // Empty view
            .when(is_empty, |this| {
                this.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .py(px(48.0))
                        .text_sm()
                        .text_color(rgb(0x707579))
                        .child(if self.is_archive {
                            "No archived stories"
                        } else {
                            "No active stories"
                        })
                )
            })
            // Stories Grid
            .when(!is_empty, |this| {
                let mut grid = div().flex().flex_wrap().gap(px(INFO_MEDIA_SKIP));
                for item in &self.items {
                    let is_sel = self.selected_ids.contains(&item.id);
                    grid = grid.child(
                        div()
                            .relative()
                            .w(px(INFO_MEDIA_MIN_GRID_SIZE))
                            .h(px(INFO_MEDIA_MIN_GRID_SIZE * 1.33))
                            .rounded(px(4.0))
                            .bg(rgb(0x17212b))
                            .overflow_hidden()
                            // Selected border
                            .border(px(2.0))
                            .border_color(if is_sel { rgb(0x5288c1) } else { rgb(0x0e1621) })
                            .child(
                                div()
                                    .absolute()
                                    .bottom(px(4.0))
                                    .left(px(4.0))
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap(px(4.0))
                                    .text_xs()
                                    .text_color(rgb(0xffffff))
                                    .child(format!("👁 {}", item.views_count))
                            )
                    );
                }
                this.child(grid)
            })
    }
}
