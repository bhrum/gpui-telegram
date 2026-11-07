//! Info Profile Tab Saved Adapter
//! Upstream: `Telegram/SourceFiles/info/profile/tabs/adapters/info_profile_tab_saved.cpp:L1-L210`
//! Upstream: `Telegram/SourceFiles/info/profile/tabs/adapters/info_profile_tab_saved.h:L1-L19`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use gpui::*;
use serde::{Deserialize, Serialize};

/// Sublist entry within the Saved Messages repository.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedSublistItem {
    pub sublist_id: i64,
    pub title: String,
    pub messages_count: usize,
    pub pinned: bool,
    pub tag: Option<String>,
}

/// Tag filter chip for Saved Messages.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedTagFilter {
    pub tag: String,
    pub count: usize,
}

/// Tab adapter bridging the profile tabs host with saved messages sublists.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoProfileTabSavedProfileTabsAdapters {
    pub peer_id: i64,
    pub sublists_count: usize,
    pub is_active: bool,
    pub search_query: String,
    pub active_tag: Option<String>,
    pub sublists: Vec<SavedSublistItem>,
    pub tags: Vec<SavedTagFilter>,
}

impl Default for InfoProfileTabSavedProfileTabsAdapters {
    fn default() -> Self {
        Self::new(0)
    }
}

impl InfoProfileTabSavedProfileTabsAdapters {
    pub fn new(peer_id: i64) -> Self {
        Self {
            peer_id,
            sublists_count: 0,
            is_active: false,
            search_query: String::new(),
            active_tag: None,
            sublists: Vec::new(),
            tags: Vec::new(),
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn set_search_query(&mut self, query: impl Into<String>) {
        self.search_query = query.into();
    }

    pub fn set_active_tag(&mut self, tag: Option<String>) {
        self.active_tag = tag;
    }

    pub fn add_sublist(&mut self, item: SavedSublistItem) {
        self.sublists.push(item);
        self.sublists_count = self.sublists.len();
    }

    pub fn add_tag(&mut self, tag: SavedTagFilter) {
        self.tags.push(tag);
    }

    pub fn filtered_sublists(&self) -> Vec<&SavedSublistItem> {
        let query = self.search_query.trim().to_lowercase();
        self.sublists
            .iter()
            .filter(|item| {
                if let Some(ref tag) = self.active_tag {
                    if item.tag.as_ref() != Some(tag) {
                        return false;
                    }
                }
                if !query.is_empty() && !item.title.to_lowercase().contains(&query) {
                    return false;
                }
                true
            })
            .collect()
    }
}

impl Render for InfoProfileTabSavedProfileTabsAdapters {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let filtered = self.filtered_sublists();
        let is_empty = filtered.is_empty();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xFFFFFF))
            .p(px(8.0))
            .gap(px(8.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(6.0))
                    .overflow_hidden()
                    .children(self.tags.iter().map(|tag| {
                        let is_active = self.active_tag.as_ref() == Some(&tag.tag);
                        div()
                            .px(px(8.0))
                            .py(px(4.0))
                            .rounded_full()
                            .bg(if is_active { rgb(0x2AABEE) } else { rgb(0xF0F2F5) })
                            .text_color(if is_active { rgb(0xFFFFFF) } else { rgb(0x222222) })
                            .text_xs()
                            .child(format!("#{} ({})", tag.tag, tag.count))
                    }))
            )
            .child(
                if is_empty {
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .h(px(140.0))
                        .text_sm()
                        .text_color(rgb(0x707579))
                        .child("No saved sublists found")
                } else {
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .children(filtered.into_iter().map(|item| {
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .justify_between()
                                .p(px(8.0))
                                .rounded_sm()
                                .hover(|s| s.bg(rgb(0xF5F7FA)))
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap(px(8.0))
                                        .child(
                                            div()
                                                .text_sm()
                                                .font_weight(FontWeight::MEDIUM)
                                                .text_color(rgb(0x222222))
                                                .child(item.title.clone())
                                        )
                                        .children(if item.pinned {
                                            Some(div().text_xs().text_color(rgb(0x707579)).child("📌"))
                                        } else {
                                            None
                                        })
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(0x707579))
                                        .child(format!("{} msgs", item.messages_count))
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
    fn test_saved_adapter_filtering_and_tags() {
        let mut adapter = InfoProfileTabSavedProfileTabsAdapters::new(3003);
        assert_eq!(adapter.peer_id, 3003);
        assert_eq!(adapter.sublists_count, 0);

        adapter.add_tag(SavedTagFilter { tag: "work".into(), count: 2 });
        adapter.add_tag(SavedTagFilter { tag: "personal".into(), count: 1 });

        adapter.add_sublist(SavedSublistItem {
            sublist_id: 1,
            title: "Project Notes".into(),
            messages_count: 42,
            pinned: true,
            tag: Some("work".into()),
        });
        adapter.add_sublist(SavedSublistItem {
            sublist_id: 2,
            title: "Shopping List".into(),
            messages_count: 5,
            pinned: false,
            tag: Some("personal".into()),
        });

        assert_eq!(adapter.filtered_sublists().len(), 2);

        adapter.set_active_tag(Some("work".into()));
        let work_items = adapter.filtered_sublists();
        assert_eq!(work_items.len(), 1);
        assert_eq!(work_items[0].title, "Project Notes");

        adapter.set_search_query("Shopping");
        assert_eq!(adapter.filtered_sublists().len(), 0); // tag work + query shopping = 0

        adapter.set_active_tag(None);
        let search_items = adapter.filtered_sublists();
        assert_eq!(search_items.len(), 1);
        assert_eq!(search_items[0].title, "Shopping List");
    }
}
