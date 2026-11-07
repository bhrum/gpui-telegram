//! Authentic GPUI implementation of Telegram Stories Data Provider.
//! Source: `Telegram/SourceFiles/info/stories/info_stories_provider.cpp:L1-L515`, `info_stories_provider.h:L1-L137`
//! Subsystem: info | Feature ID: TCG-N38 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use serde::{Deserialize, Serialize};

pub const PRELOADED_SCREENS_COUNT: usize = 3;
pub const PRELOADED_SCREENS_COUNT_FULL: usize = 5;
pub const MINIMAL_IDS_LIMIT: usize = 16;
pub const DEFAULT_AROUND_ID: u64 = 0;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoryRecord {
    pub id: u64,
    pub peer_id: u64,
    pub date: i64,
    pub expire_date: i64,
    pub is_unread: bool,
    pub views_count: usize,
    pub forwards_count: usize,
    pub reactions_count: usize,
}

#[derive(Debug, Clone)]
pub struct InfoStoriesProviderStories {
    pub peer_id: u64,
    pub records: Vec<StoryRecord>,
    pub is_loading: bool,
    pub has_more: bool,
    pub around_id: u64,
    pub slice_limit: usize,
}

impl InfoStoriesProviderStories {
    pub fn new() -> Self {
        Self {
            peer_id: 0,
            records: Vec::new(),
            is_loading: false,
            has_more: true,
            around_id: DEFAULT_AROUND_ID,
            slice_limit: MINIMAL_IDS_LIMIT,
        }
    }

    pub fn for_peer(peer_id: u64) -> Self {
        let mut p = Self::new();
        p.peer_id = peer_id;
        p
    }

    pub fn start_loading(&mut self, around_id: u64) {
        self.is_loading = true;
        self.around_id = around_id;
    }

    pub fn apply_slice(&mut self, new_records: Vec<StoryRecord>, has_more: bool) {
        for record in new_records {
            if let Some(pos) = self.records.iter().position(|r| r.id == record.id) {
                self.records[pos] = record;
            } else {
                self.records.push(record);
            }
        }
        self.records.sort_by(|a, b| b.date.cmp(&a.date));
        self.has_more = has_more;
        self.is_loading = false;
    }

    pub fn mark_story_read(&mut self, story_id: u64) {
        if let Some(rec) = self.records.iter_mut().find(|r| r.id == story_id) {
            rec.is_unread = false;
        }
    }

    pub fn total_unread_count(&self) -> usize {
        self.records.iter().filter(|r| r.is_unread).count()
    }
}

impl Default for InfoStoriesProviderStories {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoStoriesProviderStories {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let unread = self.total_unread_count();
        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .p(px(8.0))
            .bg(rgb(0x17212b))
            .rounded(px(4.0))
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x707579))
                    .child(format!("Loaded: {} stories", self.records.len()))
            )
            .when(unread > 0, |this| {
                this.child(
                    div()
                        .px(px(6.0))
                        .py(px(2.0))
                        .rounded(px(8.0))
                        .bg(rgb(0x5288c1))
                        .text_xs()
                        .text_color(rgb(0xffffff))
                        .child(format!("{} unread", unread))
                )
            })
    }
}
