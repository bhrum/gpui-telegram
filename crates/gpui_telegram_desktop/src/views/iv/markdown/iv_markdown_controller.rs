//! Target Symbol: IvMarkdownControllerMarkdown
//! Module 13: Instant View Article Navigation Controller & History Stack
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const ZOOM_STEP: f32 = 0.10;
pub const ZOOM_DROPDOWN_HIDE_DELAY_MS: u64 = 250;
pub const MAX_HISTORY_DEPTH: usize = 64;

#[derive(Debug, Clone, PartialEq)]
pub struct PageHistoryTarget {
    pub page_id: u64,
    pub url: String,
    pub scroll_y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ZoomDropdownState {
    pub is_open: bool,
    pub current_zoom: f32,
}

#[derive(Debug, Clone)]
pub struct IvMarkdownControllerMarkdown {
    pub current_url: String,
    pub current_page_id: u64,
    pub history_stack: Vec<PageHistoryTarget>,
    pub history_index: usize,
    pub zoom_state: ZoomDropdownState,
}

impl Default for IvMarkdownControllerMarkdown {
    fn default() -> Self {
        Self {
            current_url: String::new(),
            current_page_id: 0,
            history_stack: Vec::new(),
            history_index: 0,
            zoom_state: ZoomDropdownState {
                is_open: false,
                current_zoom: 1.0,
            },
        }
    }
}

impl IvMarkdownControllerMarkdown {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn navigate_to(&mut self, url: impl Into<String>, page_id: u64) {
        let url_str = url.into();
        if self.history_index < self.history_stack.len() {
            self.history_stack.truncate(self.history_index + 1);
        }

        self.history_stack.push(PageHistoryTarget {
            page_id,
            url: url_str.clone(),
            scroll_y: 0.0,
        });

        if self.history_stack.len() > MAX_HISTORY_DEPTH {
            self.history_stack.remove(0);
        }

        self.history_index = self.history_stack.len().saturating_sub(1);
        self.current_url = url_str;
        self.current_page_id = page_id;
    }

    pub fn can_go_back(&self) -> bool {
        self.history_index > 0
    }

    pub fn can_go_forward(&self) -> bool {
        self.history_index + 1 < self.history_stack.len()
    }

    pub fn go_back(&mut self) -> Option<&PageHistoryTarget> {
        if self.can_go_back() {
            self.history_index -= 1;
            let target = &self.history_stack[self.history_index];
            self.current_url = target.url.clone();
            self.current_page_id = target.page_id;
            Some(target)
        } else {
            None
        }
    }

    pub fn go_forward(&mut self) -> Option<&PageHistoryTarget> {
        if self.can_go_forward() {
            self.history_index += 1;
            let target = &self.history_stack[self.history_index];
            self.current_url = target.url.clone();
            self.current_page_id = target.page_id;
            Some(target)
        } else {
            None
        }
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom_state.current_zoom = zoom.clamp(0.5, 3.0);
    }

    pub fn toggle_zoom_dropdown(&mut self) {
        self.zoom_state.is_open = !self.zoom_state.is_open;
    }
}

impl Render for IvMarkdownControllerMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let can_back = self.can_go_back();
        let can_fwd = self.can_go_forward();
        let url_str = self.current_url.clone();

        div()
            .flex()
            .items_center()
            .gap_2()
            .px_3()
            .py_1()
            .bg(rgb(0x212529))
            .text_color(rgb(0xd0d5dc))
            .child(
                div()
                    .cursor_pointer()
                    .opacity(if can_back { 1.0 } else { 0.3 })
                    .child("<"),
            )
            .child(
                div()
                    .cursor_pointer()
                    .opacity(if can_fwd { 1.0 } else { 0.3 })
                    .child(">"),
            )
            .child(
                div()
                    .flex_1()
                    .text_xs()
                    .overflow_hidden()
                    .child(url_str),
            )
    }
}
