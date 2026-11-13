//! Target Symbol: IvSearchBar
//! Module 13: Instant View Search Bar Overlay Component
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const SEARCH_BAR_HEIGHT: f32 = 48.0;
pub const PILL_RADIUS: f32 = 16.0;
pub const PILL_PAD: f32 = 8.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SearchBarMode {
    #[default]
    WindowStrip,
    EditorPill,
}

#[derive(Debug, Clone)]
pub struct IvSearchBar {
    pub mode: SearchBarMode,
    pub query: String,
    pub current_match: usize,
    pub total_matches: usize,
    pub is_shown: bool,
    pub is_focused: bool,
}

impl Default for IvSearchBar {
    fn default() -> Self {
        Self {
            mode: SearchBarMode::WindowStrip,
            query: String::new(),
            current_match: 0,
            total_matches: 0,
            is_shown: false,
            is_focused: false,
        }
    }
}

impl IvSearchBar {
    pub fn new(mode: SearchBarMode) -> Self {
        Self {
            mode,
            query: String::new(),
            current_match: 0,
            total_matches: 0,
            is_shown: false,
            is_focused: false,
        }
    }

    pub fn set_results(&mut self, current: usize, total: usize) {
        self.current_match = current;
        self.total_matches = total;
    }

    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query = query.into();
    }

    pub fn toggle(&mut self) {
        self.is_shown = !self.is_shown;
        if self.is_shown {
            self.is_focused = true;
        }
    }

    pub fn show(&mut self) {
        self.is_shown = true;
        self.is_focused = true;
    }

    pub fn hide(&mut self) {
        self.is_shown = false;
        self.is_focused = false;
    }

    pub fn next_match(&mut self) {
        if self.total_matches > 0 {
            self.current_match = (self.current_match % self.total_matches) + 1;
        }
    }

    pub fn previous_match(&mut self) {
        if self.total_matches > 0 {
            self.current_match = if self.current_match <= 1 {
                self.total_matches
            } else {
                self.current_match - 1
            };
        }
    }
}

impl Render for IvSearchBar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_shown {
            return div().into_any_element();
        }

        let results_badge = if self.query.is_empty() {
            String::new()
        } else if self.total_matches == 0 {
            "No matches".to_string()
        } else {
            format!("{} of {}", self.current_match, self.total_matches)
        };

        match self.mode {
            SearchBarMode::WindowStrip => div()
                .flex()
                .flex_row()
                .w_full()
                .h(px(SEARCH_BAR_HEIGHT))
                .px(px(14.0))
                .bg(rgb(0x1E222A))
                .border_b_1()
                .border_color(rgb(0x2E3440))
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(10.0))
                        .child(
                            div()
                                .text_size(px(13.0))
                                .text_color(rgb(0x8D95A5))
                                .child("🔍"),
                        )
                        .child(
                            div()
                                .text_size(px(13.0))
                                .text_color(if self.query.is_empty() {
                                    rgb(0x5A6272)
                                } else {
                                    rgb(0xFFFFFF)
                                })
                                .child(if self.query.is_empty() {
                                    "Search in page...".to_string()
                                } else {
                                    self.query.clone()
                                }),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(8.0))
                        .child(
                            div()
                                .text_size(px(12.0))
                                .text_color(rgb(0x88C0D0))
                                .child(results_badge),
                        )
                        .child(
                            div()
                                .px(px(6.0))
                                .py(px(2.0))
                                .rounded(px(4.0))
                                .bg(rgb(0x2C3340))
                                .text_color(rgb(0xD8DEE9))
                                .text_size(px(11.0))
                                .child("▲"),
                        )
                        .child(
                            div()
                                .px(px(6.0))
                                .py(px(2.0))
                                .rounded(px(4.0))
                                .bg(rgb(0x2C3340))
                                .text_color(rgb(0xD8DEE9))
                                .text_size(px(11.0))
                                .child("▼"),
                        )
                        .child(
                            div()
                                .px(px(6.0))
                                .py(px(2.0))
                                .rounded(px(4.0))
                                .bg(rgb(0x2C3340))
                                .text_color(rgb(0xED6A5E))
                                .text_size(px(11.0))
                                .child("✕"),
                        ),
                )
                .into_any_element(),
            SearchBarMode::EditorPill => div()
                .flex()
                .flex_row()
                .items_center()
                .px(px(12.0))
                .py(px(6.0))
                .bg(rgb(0x232934))
                .rounded(px(PILL_RADIUS))
                .shadow_lg()
                .border_1()
                .border_color(rgb(0x353E4E))
                .gap(px(8.0))
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(rgb(0xFFFFFF))
                        .child(if self.query.is_empty() {
                            "Search...".to_string()
                        } else {
                            self.query.clone()
                        }),
                )
                .child(
                    div()
                        .text_size(px(11.0))
                        .text_color(rgb(0x88C0D0))
                        .child(results_badge),
                )
                .into_any_element(),
        }
    }
}
