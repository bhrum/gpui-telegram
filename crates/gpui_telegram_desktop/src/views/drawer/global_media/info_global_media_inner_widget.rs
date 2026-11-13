//! Upstream: Telegram/SourceFiles/info/global_media/info_global_media_inner_widget.h
//! Upstream: Telegram/SourceFiles/info/global_media/info_global_media_inner_widget.cpp
//! Subsystem: info | Feature ID: TCG-N23 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SharedMediaType {
    Photo,
    Video,
    File,
    Audio,
    Link,
    RoundVoiceFile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionAction {
    SelectAll,
    ClearSelection,
    DeleteSelected,
    ForwardSelected,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScrollToRequest {
    pub y: i32,
    pub min_height: i32,
}

#[derive(Debug, Clone)]
pub struct MediaItemEntry {
    pub id: u64,
    pub title: String,
    pub subtitle: String,
    pub media_type: SharedMediaType,
    pub height: i32,
}

#[derive(Debug, Clone)]
pub struct InfoGlobalMediaInnerWidgetGlobalMedia {
    pub media_type: SharedMediaType,
    pub search_query: String,
    pub items: Vec<MediaItemEntry>,
    pub selected_ids: HashSet<u64>,
    pub scroll_top: i32,
    pub viewport_height: i32,
    pub in_resize: bool,
}

impl InfoGlobalMediaInnerWidgetGlobalMedia {
    pub fn new(media_type: SharedMediaType) -> Self {
        Self {
            media_type,
            search_query: String::new(),
            items: Vec::new(),
            selected_ids: HashSet::new(),
            scroll_top: 0,
            viewport_height: 600,
            in_resize: false,
        }
    }

    pub fn set_search_query(&mut self, query: impl Into<String>) {
        self.search_query = query.into();
    }

    pub fn add_item(&mut self, item: MediaItemEntry) {
        self.items.push(item);
    }

    pub fn set_items(&mut self, items: Vec<MediaItemEntry>) {
        self.items = items;
    }

    pub fn list_height(&self) -> i32 {
        self.items.iter().map(|item| item.height).sum()
    }

    /// Upstream logic: `recountHeight`
    /// If listHeight > 0, empty illustration is hidden. Otherwise empty illustration height is counted.
    pub fn recount_height(&self) -> i32 {
        let lh = self.list_height();
        if lh > 0 {
            lh
        } else {
            // Upstream empty widget minimum height
            240
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn map_scroll_to_request(&self, item_y: i32) -> ScrollToRequest {
        ScrollToRequest {
            y: self.scroll_top + item_y,
            min_height: -1,
        }
    }

    pub fn select_item(&mut self, id: u64) {
        self.selected_ids.insert(id);
    }

    pub fn toggle_selection(&mut self, id: u64) {
        if self.selected_ids.contains(&id) {
            self.selected_ids.remove(&id);
        } else {
            self.selected_ids.insert(id);
        }
    }

    pub fn selection_action(&mut self, action: SelectionAction) {
        match action {
            SelectionAction::SelectAll => {
                self.selected_ids = self.items.iter().map(|it| it.id).collect();
            }
            SelectionAction::ClearSelection => {
                self.selected_ids.clear();
            }
            SelectionAction::DeleteSelected => {
                self.items.retain(|it| !self.selected_ids.contains(&it.id));
                self.selected_ids.clear();
            }
            SelectionAction::ForwardSelected => {
                // Keep selection for forward dialog dispatch
            }
        }
    }

    pub fn save_state(&self) -> (i32, Vec<u64>) {
        (self.scroll_top, self.selected_ids.iter().copied().collect())
    }

    pub fn restore_state(&mut self, state: (i32, Vec<u64>)) {
        self.scroll_top = state.0;
        self.selected_ids = state.1.into_iter().collect();
    }
}

impl Default for InfoGlobalMediaInnerWidgetGlobalMedia {
    fn default() -> Self {
        Self::new(SharedMediaType::Photo)
    }
}

impl Render for InfoGlobalMediaInnerWidgetGlobalMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_empty = self.is_empty();
        let query = self.search_query.clone();

        let mut root = div().flex().flex_col().size_full().p(px(12.0));

        if is_empty {
            root = root.child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .py(px(40.0))
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x707579))
                            .child(if query.is_empty() {
                                "No media files found".to_string()
                            } else {
                                format!("No results for \"{}\"", query)
                            }),
                    ),
            );
        } else {
            root = root.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .children(self.items.iter().map(|item| {
                        let is_selected = self.selected_ids.contains(&item.id);
                        div()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .items_center()
                            .p(px(8.0))
                            .rounded(px(6.0))
                            .bg(if is_selected { rgb(0x2b5278) } else { rgb(0x17212b) })
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(div().text_sm().child(item.title.clone()))
                                    .child(div().text_xs().text_color(rgb(0x707579)).child(item.subtitle.clone())),
                            )
                    })),
            );
        }

        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inner_widget_empty_state_and_height() {
        let mut widget = InfoGlobalMediaInnerWidgetGlobalMedia::new(SharedMediaType::Photo);
        assert!(widget.is_empty());
        assert_eq!(widget.recount_height(), 240);

        widget.add_item(MediaItemEntry {
            id: 101,
            title: "Photo 1".into(),
            subtitle: "1024x768".into(),
            media_type: SharedMediaType::Photo,
            height: 104,
        });

        assert!(!widget.is_empty());
        assert_eq!(widget.list_height(), 104);
        assert_eq!(widget.recount_height(), 104);
    }

    #[test]
    fn test_inner_widget_selection_actions() {
        let mut widget = InfoGlobalMediaInnerWidgetGlobalMedia::new(SharedMediaType::File);
        widget.add_item(MediaItemEntry {
            id: 1,
            title: "Doc1.pdf".into(),
            subtitle: "2.5 MB".into(),
            media_type: SharedMediaType::File,
            height: 56,
        });
        widget.add_item(MediaItemEntry {
            id: 2,
            title: "Doc2.zip".into(),
            subtitle: "15.0 MB".into(),
            media_type: SharedMediaType::File,
            height: 56,
        });

        widget.selection_action(SelectionAction::SelectAll);
        assert_eq!(widget.selected_ids.len(), 2);

        widget.toggle_selection(1);
        assert_eq!(widget.selected_ids.len(), 1);
        assert!(widget.selected_ids.contains(&2));

        widget.selection_action(SelectionAction::ClearSelection);
        assert!(widget.selected_ids.is_empty());

        widget.select_item(1);
        widget.selection_action(SelectionAction::DeleteSelected);
        assert_eq!(widget.items.len(), 1);
        assert_eq!(widget.items[0].id, 2);
    }

    #[test]
    fn test_scroll_mapping_and_memento() {
        let mut widget = InfoGlobalMediaInnerWidgetGlobalMedia::new(SharedMediaType::Video);
        widget.scroll_top = 150;
        let req = widget.map_scroll_to_request(50);
        assert_eq!(req.y, 200);
        assert_eq!(req.min_height, -1);

        widget.select_item(42);
        let saved = widget.save_state();
        assert_eq!(saved.0, 150);
        assert_eq!(saved.1, vec![42]);

        let mut restored = InfoGlobalMediaInnerWidgetGlobalMedia::new(SharedMediaType::Video);
        restored.restore_state(saved);
        assert_eq!(restored.scroll_top, 150);
        assert!(restored.selected_ids.contains(&42));
    }
}
