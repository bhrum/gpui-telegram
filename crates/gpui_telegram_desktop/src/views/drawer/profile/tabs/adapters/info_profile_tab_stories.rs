//! Info Profile Tab Stories Adapter
//! Upstream: `Telegram/SourceFiles/info/profile/tabs/adapters/info_profile_tab_stories.cpp:L1-L168`
//! Upstream: `Telegram/SourceFiles/info/profile/tabs/adapters/info_profile_tab_stories.h:L1-L17`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use gpui::*;
use serde::{Deserialize, Serialize};

/// Story item rendered inside story preview bubbles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoryItem {
    pub story_id: u64,
    pub is_viewed: bool,
    pub views_count: usize,
    pub caption: Option<String>,
}

/// Grouping of stories by date header.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoryDateGroup {
    pub date_label: String,
    pub stories: Vec<StoryItem>,
}

/// Tab adapter bridging the profile tabs host with peer stories grid and archive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoProfileTabStoriesProfileTabsAdapters {
    pub peer_id: i64,
    pub stories_count: usize,
    pub is_archive: bool,
    pub is_active: bool,
    pub groups: Vec<StoryDateGroup>,
}

impl Default for InfoProfileTabStoriesProfileTabsAdapters {
    fn default() -> Self {
        Self::new(0, false)
    }
}

impl InfoProfileTabStoriesProfileTabsAdapters {
    pub fn new(peer_id: i64, is_archive: bool) -> Self {
        Self {
            peer_id,
            stories_count: 0,
            is_archive,
            is_active: false,
            groups: Vec::new(),
        }
    }

    pub fn set_stories_count(&mut self, count: usize) {
        self.stories_count = count;
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn toggle_archive(&mut self) -> bool {
        self.is_archive = !self.is_archive;
        self.is_archive
    }

    pub fn add_story(&mut self, date_label: impl Into<String>, story: StoryItem) {
        let label = date_label.into();
        if let Some(group) = self.groups.iter_mut().find(|g| g.date_label == label) {
            group.stories.push(story);
        } else {
            self.groups.push(StoryDateGroup {
                date_label: label,
                stories: vec![story],
            });
        }
        self.stories_count = self.groups.iter().map(|g| g.stories.len()).sum();
    }

    pub fn mark_story_viewed(&mut self, story_id: u64) -> bool {
        for group in &mut self.groups {
            for story in &mut group.stories {
                if story.story_id == story_id {
                    story.is_viewed = true;
                    return true;
                }
            }
        }
        false
    }

    pub fn total_views(&self) -> usize {
        self.groups
            .iter()
            .flat_map(|g| &g.stories)
            .map(|s| s.views_count)
            .sum()
    }
}

impl Render for InfoProfileTabStoriesProfileTabsAdapters {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_empty = self.groups.is_empty();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xFFFFFF))
            .p(px(12.0))
            .gap(px(10.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(rgb(0xEBEEF2))
                    .pb(px(8.0))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x222222))
                            .child(if self.is_archive { "Archived Stories" } else { "Active Stories" })
                    )
                    .child(
                        div()
                            .px(px(8.0))
                            .py(px(4.0))
                            .rounded_sm()
                            .bg(rgb(0xF0F2F5))
                            .text_xs()
                            .text_color(rgb(0x2AABEE))
                            .child(format!("{} stories", self.stories_count))
                    )
            )
            .child(
                if is_empty {
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .h(px(150.0))
                        .text_sm()
                        .text_color(rgb(0x707579))
                        .child("No stories available")
                } else {
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.0))
                        .children(self.groups.iter().map(|group| {
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(6.0))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(rgb(0x707579))
                                        .child(group.date_label.clone())
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .gap(px(8.0))
                                        .children(group.stories.iter().map(|story| {
                                            div()
                                                .flex()
                                                .flex_col()
                                                .items_center()
                                                .gap(px(4.0))
                                                .child(
                                                    div()
                                                        .size(px(64.0))
                                                        .rounded_full()
                                                        .border_2()
                                                        .border_color(if story.is_viewed { rgb(0xCCCCCC) } else { rgb(0x2AABEE) })
                                                        .bg(rgb(0xEEF2F5))
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .text_xs()
                                                        .text_color(rgb(0x555555))
                                                        .child("Story")
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(rgb(0x707579))
                                                        .child(format!("👁 {}", story.views_count))
                                                )
                                        }))
                                )
                        }))
                }
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stories_adapter_archive_and_views() {
        let mut adapter = InfoProfileTabStoriesProfileTabsAdapters::new(4004, false);
        assert_eq!(adapter.peer_id, 4004);
        assert!(!adapter.is_archive);
        assert_eq!(adapter.stories_count, 0);

        assert!(adapter.toggle_archive());
        assert!(adapter.is_archive);
        assert!(!adapter.toggle_archive());
        assert!(!adapter.is_archive);

        adapter.add_story("Today", StoryItem {
            story_id: 10,
            is_viewed: false,
            views_count: 150,
            caption: Some("Coffee break".into()),
        });
        adapter.add_story("Today", StoryItem {
            story_id: 11,
            is_viewed: true,
            views_count: 220,
            caption: None,
        });

        assert_eq!(adapter.stories_count, 2);
        assert_eq!(adapter.total_views(), 370);

        assert!(adapter.mark_story_viewed(10));
        assert!(adapter.groups[0].stories[0].is_viewed);
        assert!(!adapter.mark_story_viewed(999)); // non-existent
    }
}
