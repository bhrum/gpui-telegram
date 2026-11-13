//! Upstream: Telegram/SourceFiles/info/global_media/info_global_media_widget.h
//! Upstream: Telegram/SourceFiles/info/global_media/info_global_media_widget.cpp
//! Subsystem: info | Feature ID: TCG-N23 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use super::info_global_media_inner_widget::{
    InfoGlobalMediaInnerWidgetGlobalMedia, SelectionAction, SharedMediaType,
};

#[derive(Debug, Clone)]
pub struct GlobalMediaMemento {
    pub media_type: SharedMediaType,
    pub scroll_top: i32,
    pub selected_ids: Vec<u64>,
}

#[derive(Debug, Clone)]
pub struct InfoGlobalMediaWidgetGlobalMedia {
    pub inner: InfoGlobalMediaInnerWidgetGlobalMedia,
    pub title: String,
    pub show_delete_all_confirmation: bool,
}

impl InfoGlobalMediaWidgetGlobalMedia {
    pub fn new(media_type: SharedMediaType) -> Self {
        Self {
            inner: InfoGlobalMediaInnerWidgetGlobalMedia::new(media_type),
            title: "Shared Media".into(),
            show_delete_all_confirmation: false,
        }
    }

    pub fn media_type(&self) -> SharedMediaType {
        self.inner.media_type
    }

    pub fn set_search_query(&mut self, query: impl Into<String>) {
        self.inner.set_search_query(query);
    }

    /// Upstream logic: `fillTopBarMenu` -> "Delete All Files"
    pub fn request_delete_all_files(&mut self) {
        self.show_delete_all_confirmation = true;
    }

    pub fn confirm_delete_all_files(&mut self) {
        self.inner.items.clear();
        self.inner.selected_ids.clear();
        self.show_delete_all_confirmation = false;
    }

    pub fn cancel_delete_all_files(&mut self) {
        self.show_delete_all_confirmation = false;
    }

    pub fn selection_action(&mut self, action: SelectionAction) {
        self.inner.selection_action(action);
    }

    pub fn create_memento(&self) -> GlobalMediaMemento {
        let (scroll_top, selected_ids) = self.inner.save_state();
        GlobalMediaMemento {
            media_type: self.inner.media_type,
            scroll_top,
            selected_ids,
        }
    }

    pub fn restore_from_memento(&mut self, memento: &GlobalMediaMemento) {
        self.inner.media_type = memento.media_type;
        self.inner.restore_state((memento.scroll_top, memento.selected_ids.clone()));
    }
}

impl Default for InfoGlobalMediaWidgetGlobalMedia {
    fn default() -> Self {
        Self::new(SharedMediaType::Photo)
    }
}

impl Render for InfoGlobalMediaWidgetGlobalMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let title = self.title.clone();
        let show_confirm = self.show_delete_all_confirmation;

        let mut root = div().flex().flex_col().size_full();

        root = root.child(
            div()
                .flex()
                .flex_row()
                .justify_between()
                .items_center()
                .p(px(16.0))
                .border_b_1()
                .border_color(rgb(0x242f3d))
                .child(
                    div()
                        .font_weight(FontWeight::BOLD)
                        .text_base()
                        .child(title),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(0x5288c1))
                        .cursor_pointer()
                        .child("Delete All"),
                ),
        );

        if show_confirm {
            root = root.child(
                div()
                    .p(px(12.0))
                    .bg(rgb(0x281c1c))
                    .rounded(px(6.0))
                    .m(px(12.0))
                    .child(div().text_sm().child("Are you sure you want to delete all cached files?"))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(px(8.0))
                            .mt(px(8.0))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0xec3942))
                                    .cursor_pointer()
                                    .child("Confirm Delete"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x707579))
                                    .cursor_pointer()
                                    .child("Cancel"),
                            ),
                    ),
            );
        }

        root = root.child(
            div()
                .flex_1()
                .overflow_hidden()
                .p(px(12.0))
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(0x707579))
                        .child(format!("{} items loaded", self.inner.items.len())),
                ),
        );

        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::info_global_media_inner_widget::MediaItemEntry;

    #[test]
    fn test_widget_lifecycle_and_delete_all() {
        let mut widget = InfoGlobalMediaWidgetGlobalMedia::new(SharedMediaType::Photo);
        widget.inner.add_item(MediaItemEntry {
            id: 1,
            title: "Item 1".into(),
            subtitle: "100 KB".into(),
            media_type: SharedMediaType::Photo,
            height: 100,
        });

        assert_eq!(widget.inner.items.len(), 1);
        assert!(!widget.show_delete_all_confirmation);

        widget.request_delete_all_files();
        assert!(widget.show_delete_all_confirmation);

        widget.confirm_delete_all_files();
        assert!(widget.inner.items.is_empty());
        assert!(!widget.show_delete_all_confirmation);
    }

    #[test]
    fn test_widget_memento_roundtrip() {
        let mut widget = InfoGlobalMediaWidgetGlobalMedia::new(SharedMediaType::File);
        widget.inner.scroll_top = 220;
        widget.inner.select_item(99);

        let memento = widget.create_memento();
        assert_eq!(memento.scroll_top, 220);
        assert_eq!(memento.selected_ids, vec![99]);

        let mut fresh = InfoGlobalMediaWidgetGlobalMedia::new(SharedMediaType::Photo);
        fresh.restore_from_memento(&memento);
        assert_eq!(fresh.media_type(), SharedMediaType::File);
        assert_eq!(fresh.inner.scroll_top, 220);
        assert!(fresh.inner.selected_ids.contains(&99));
    }
}
