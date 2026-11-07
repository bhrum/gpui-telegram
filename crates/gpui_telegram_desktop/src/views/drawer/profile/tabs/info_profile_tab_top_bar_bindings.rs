//! Info Profile Tab Top Bar Bindings
//! Upstream: `Telegram/SourceFiles/info/profile/tabs/info_profile_tab_top_bar_bindings.h:L1-L31`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use std::collections::HashMap;
use gpui::*;
use serde::{Deserialize, Serialize};

/// Dynamic binding linking active tab selection to profile top bar titles, counts,
/// search field activations, and selection actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoProfileTabTopBarBindingsProfileTabs {
    pub current_tab_title: String,
    pub current_tab_subtitle: Option<String>,
    pub show_search_action: bool,
    pub is_search_field_active: bool,
    pub search_query: String,
    pub is_selection_mode: bool,
    pub selected_count: usize,
    pub tab_counts: HashMap<String, usize>,
}

impl InfoProfileTabTopBarBindingsProfileTabs {
    pub fn new(tab_title: impl Into<String>) -> Self {
        Self {
            current_tab_title: tab_title.into(),
            current_tab_subtitle: None,
            show_search_action: true,
            is_search_field_active: false,
            search_query: String::new(),
            is_selection_mode: false,
            selected_count: 0,
            tab_counts: HashMap::new(),
        }
    }

    pub fn set_subtitle(&mut self, subtitle: impl Into<String>) {
        self.current_tab_subtitle = Some(subtitle.into());
    }

    pub fn set_tab_count(&mut self, tab_title: &str, count: usize) {
        self.tab_counts.insert(tab_title.to_string(), count);
    }

    pub fn format_subtitle_for_tab(&mut self, tab_title: &str, singular: &str, plural: &str) {
        let count = self.tab_counts.get(tab_title).copied().unwrap_or(0);
        let unit = if count == 1 { singular } else { plural };
        self.current_tab_subtitle = Some(format!("{} {}", count, unit));
    }

    pub fn toggle_search(&mut self) -> bool {
        self.is_search_field_active = !self.is_search_field_active;
        if !self.is_search_field_active {
            self.search_query.clear();
        }
        self.is_search_field_active
    }

    pub fn set_search_query(&mut self, query: impl Into<String>) {
        self.search_query = query.into();
    }

    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.is_search_field_active = false;
    }

    pub fn set_selection_mode(&mut self, enabled: bool, count: usize) {
        self.is_selection_mode = enabled;
        self.selected_count = if enabled { count } else { 0 };
    }
}

impl Default for InfoProfileTabTopBarBindingsProfileTabs {
    fn default() -> Self {
        Self::new("Media")
    }
}

impl Render for InfoProfileTabTopBarBindingsProfileTabs {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_selecting = self.is_selection_mode;
        let is_searching = self.is_search_field_active;

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .h(px(48.0))
            .px(px(12.0))
            .bg(rgb(0xFFFFFF))
            .border_b_1()
            .border_color(rgb(0xEBEEF2))
            .child(
                if is_selecting {
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(8.0))
                        .child(
                            div()
                                .font_weight(FontWeight::BOLD)
                                .text_sm()
                                .text_color(rgb(0x222222))
                                .child(format!("{} selected", self.selected_count))
                        )
                } else if is_searching {
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(8.0))
                        .flex_1()
                        .child(
                            div()
                                .text_xs()
                                .text_color(rgb(0x707579))
                                .child("🔍")
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(rgb(0x222222))
                                .child(if self.search_query.is_empty() {
                                    format!("Search in {}...", self.current_tab_title)
                                } else {
                                    self.search_query.clone()
                                })
                        )
                } else {
                    div()
                        .flex()
                        .flex_col()
                        .justify_center()
                        .child(
                            div()
                                .font_weight(FontWeight::BOLD)
                                .text_sm()
                                .text_color(rgb(0x222222))
                                .child(self.current_tab_title.clone())
                        )
                        .children(self.current_tab_subtitle.as_ref().map(|sub| {
                            div()
                                .text_xs()
                                .text_color(rgb(0x707579))
                                .child(sub.clone())
                        }))
                }
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .children(if self.show_search_action {
                        Some(
                            div()
                                .size(px(28.0))
                                .rounded_sm()
                                .bg(rgb(0xF0F2F5))
                                .flex()
                                .items_center()
                                .justify_center()
                                .cursor_pointer()
                                .text_xs()
                                .text_color(rgb(0x555555))
                                .child(if is_searching { "✕" } else { "🔍" })
                        )
                    } else {
                        None
                    })
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_bar_bindings_state() {
        let mut bindings = InfoProfileTabTopBarBindingsProfileTabs::new("Photos");
        assert_eq!(bindings.current_tab_title, "Photos");
        assert!(bindings.current_tab_subtitle.is_none());
        assert!(bindings.show_search_action);

        bindings.set_tab_count("Photos", 125);
        bindings.format_subtitle_for_tab("Photos", "photo", "photos");
        assert_eq!(bindings.current_tab_subtitle.as_deref(), Some("125 photos"));

        bindings.set_tab_count("Photos", 1);
        bindings.format_subtitle_for_tab("Photos", "photo", "photos");
        assert_eq!(bindings.current_tab_subtitle.as_deref(), Some("1 photo"));

        assert!(bindings.toggle_search());
        assert!(bindings.is_search_field_active);

        bindings.set_search_query("vacation");
        assert_eq!(bindings.search_query, "vacation");

        bindings.clear_search();
        assert!(!bindings.is_search_field_active);
        assert!(bindings.search_query.is_empty());

        bindings.set_selection_mode(true, 5);
        assert!(bindings.is_selection_mode);
        assert_eq!(bindings.selected_count, 5);

        bindings.set_selection_mode(false, 0);
        assert!(!bindings.is_selection_mode);
    }
}
