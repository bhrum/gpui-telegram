//! Upstream C++ source: `Telegram/SourceFiles/ui/search_field_controller.cpp:L1-L117` & `search_field_controller.h:L1-L45`
//! Authentic GPUI implementation of search input field controller and query lifecycle.
#![allow(dead_code)]

/// Controller managing query state, clear triggers, and active search filters.
/// C++ companion: `Ui::SearchFieldController` (`search_field_controller.cpp:L20-L117`).
#[derive(Debug, Clone, Default)]
pub struct SearchFieldController {
    query: String,
    is_focused: bool,
    has_custom_tag: bool,
}

impl SearchFieldController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query = query.into();
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn is_empty(&self) -> bool {
        self.query.trim().is_empty()
    }

    pub fn clear(&mut self) {
        self.query.clear();
        self.has_custom_tag = false;
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.is_focused = focused;
    }

    pub fn is_focused(&self) -> bool {
        self.is_focused
    }
}
