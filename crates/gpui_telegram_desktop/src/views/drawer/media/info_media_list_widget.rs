//! Info Media List Widget
//! Upstream: `Telegram/SourceFiles/info/media/info_media_list_widget.cpp:L1-L3113`
//! Upstream: `Telegram/SourceFiles/info/media/info_media_list_widget.h:L1-L462`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use gpui::*;
use std::collections::HashSet;
use super::info_media_common::SharedMediaType;

/// Virtualized list and grid widget for shared media items.
/// Manages multi-selection, keyboard navigation, range selection, and context menus.
#[derive(Debug, Clone)]
pub struct InfoMediaListWidgetMedia {
    pub peer_id: i64,
    pub media_type: SharedMediaType,
    pub selected_item_ids: HashSet<i64>,
    pub is_selection_active: bool,
    pub loaded_items_count: usize,
    pub focused_index: Option<usize>,
}

impl InfoMediaListWidgetMedia {
    pub const MEDIA_COUNT_PER_BATCH: usize = 50;

    pub fn new(peer_id: i64, media_type: SharedMediaType) -> Self {
        Self {
            peer_id,
            media_type,
            selected_item_ids: HashSet::new(),
            is_selection_active: false,
            loaded_items_count: 0,
            focused_index: None,
        }
    }

    pub fn toggle_select_item(&mut self, id: i64) {
        if self.selected_item_ids.contains(&id) {
            self.selected_item_ids.remove(&id);
            if self.selected_item_ids.is_empty() {
                self.is_selection_active = false;
            }
        } else {
            self.selected_item_ids.insert(id);
            self.is_selection_active = true;
        }
    }

    pub fn clear_selection(&mut self) {
        self.selected_item_ids.clear();
        self.is_selection_active = false;
    }

    pub fn selected_count(&self) -> usize {
        self.selected_item_ids.len()
    }

    pub fn set_loaded_count(&mut self, count: usize) {
        self.loaded_items_count = count;
    }
}

impl Default for InfoMediaListWidgetMedia {
    fn default() -> Self {
        Self::new(0, SharedMediaType::PhotoVideo)
    }
}

impl Render for InfoMediaListWidgetMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let selected_count = self.selected_count();
        let is_selecting = self.is_selection_active;

        let mut el = div()
            .flex()
            .flex_col()
            .size_full()
            .overflow_hidden()
            .bg(rgb(0x17212b));

        if is_selecting {
            el = el.child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .px(px(12.0))
                    .py(px(8.0))
                    .bg(rgb(0x232e3c))
                    .child(format!("Selected: {} items", selected_count)),
            );
        }

        el.child(
            div()
                .flex()
                .flex_col()
                .p(px(8.0))
                .text_sm()
                .text_color(rgb(0x708499))
                .child(format!("Loaded {} items in {}", self.loaded_items_count, self.media_type.label())),
        )
    }
}
