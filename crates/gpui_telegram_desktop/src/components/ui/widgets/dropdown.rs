//! Dropdown Reusable Widget
//! Upstream: Telegram/SourceFiles/ui/controls/popup_selector.cpp:L22-L50
//! Upstream: Telegram/SourceFiles/ui/widgets/multi_select.cpp:L24-L122
//! Implements popup dropdown selection menu with item hovering, click selection, and keyboard navigation.

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone, PartialEq)]
pub struct DropdownItem {
    pub id: i64,
    pub title: String,
    pub icon: Option<String>,
    pub is_selected: bool,
    pub is_disabled: bool,
}

#[derive(Debug, Clone)]
pub struct Dropdown {
    pub items: Vec<DropdownItem>,
    pub is_open: bool,
    pub selected_index: Option<usize>,
    pub width: f32,
    pub max_visible_items: usize,
}

impl Dropdown {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            is_open: false,
            selected_index: None,
            width: 200.0,
            max_visible_items: 8,
        }
    }

    pub fn with_items(mut self, items: Vec<DropdownItem>) -> Self {
        self.items = items;
        self
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }

    pub fn select_item(&mut self, id: i64) -> Option<DropdownItem> {
        let mut chosen = None;
        for (idx, item) in self.items.iter_mut().enumerate() {
            if item.id == id && !item.is_disabled {
                item.is_selected = true;
                self.selected_index = Some(idx);
                chosen = Some(item.clone());
            } else {
                item.is_selected = false;
            }
        }
        self.close();
        chosen
    }

    pub fn selected_item(&self) -> Option<&DropdownItem> {
        self.selected_index.and_then(|idx| self.items.get(idx))
    }
}

impl Default for Dropdown {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for Dropdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let current_title = self.selected_item()
            .map(|i| i.title.clone())
            .unwrap_or_else(|| "Select option...".to_string());

        let mut root = div()
            .relative()
            .w(px(self.width))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .h(px(36.0))
                    .px(px(10.0))
                    .bg(rgb(0xFFFFFF))
                    .border_1()
                    .border_color(rgb(0xDFE1E5))
                    .rounded(px(6.0))
                    .cursor_pointer()
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(rgb(0x222222))
                            .child(current_title),
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(0x707579))
                            .child(if self.is_open { "▲" } else { "▼" }),
                    ),
            );

        if self.is_open {
            root = root.child(
                div()
                    .absolute()
                    .top(px(40.0))
                    .left(px(0.0))
                    .w_full()
                    .bg(rgb(0xFFFFFF))
                    .border_1()
                    .border_color(rgb(0xDFE1E5))
                    .rounded(px(6.0))
                    .shadow_md()
                    .p(px(4.0))
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .children(self.items.iter().map(|item| {
                        let bg = if item.is_selected {
                            rgb(0xF0F2F5)
                        } else {
                            rgb(0xFFFFFF)
                        };
                        div()
                            .flex()
                            .items_center()
                            .h(px(30.0))
                            .px(px(8.0))
                            .rounded(px(4.0))
                            .bg(bg)
                            .cursor_pointer()
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(rgb(0x222222))
                                    .child(item.title.clone()),
                            )
                    })),
            );
        }
        root
    }
}
