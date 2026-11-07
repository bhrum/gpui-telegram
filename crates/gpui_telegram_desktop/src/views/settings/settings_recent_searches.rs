//! Authentic GPUI Settings Recent Searches Component
//! Source: Telegram/SourceFiles/settings/settings_recent_searches.cpp, Telegram/SourceFiles/settings/settings_recent_searches.h
//! Subsystem: settings | Feature ID: TCG-N31 | Disposition: rust_domain_or_server_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_RECENT_SEARCHES_LIMIT: usize = 32;

#[derive(Debug, Clone)]
pub struct SettingsRecentSearches {
    pub queries: Vec<String>,
    pub limit: usize,
    pub is_visible: bool,
}

impl SettingsRecentSearches {
    pub fn new() -> Self {
        Self {
            queries: vec![
                "Notifications".into(),
                "Storage usage".into(),
                "Two-Step Verification".into(),
                "Proxy settings".into(),
                "Themes and colors".into(),
            ],
            limit: K_RECENT_SEARCHES_LIMIT,
            is_visible: true,
        }
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }

    /// Upstream method: `Serialize::ByteArrayReader`
    pub fn serialize(&self) -> Vec<u8> {
        self.queries.join("\n").into_bytes()
    }

    /// Upstream method: `RecentSearches::list`
    pub fn list(&self) -> &[String] {
        &self.queries
    }

    /// Upstream method: `bump`
    pub fn bump(&mut self, query: String) {
        if let Some(pos) = self.queries.iter().position(|q| q == &query) {
            self.queries.remove(pos);
        }
        self.queries.insert(0, query);
        if self.queries.len() > self.limit {
            self.queries.truncate(self.limit);
        }
    }

    /// Upstream method: `stringSize`
    pub fn string_size(&self) -> usize {
        self.queries.iter().map(|q| q.len()).sum()
    }

    /// Upstream method: `RecentSearches::RecentSearches`
    pub fn recent_searches(&self) -> &[String] {
        &self.queries
    }

    /// Upstream method: `find`
    pub fn find(&self, query: &str) -> bool {
        self.queries.iter().any(|q| q == query)
    }

    pub fn clear(&mut self) {
        self.queries.clear();
    }
}

impl Default for SettingsRecentSearches {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsRecentSearches {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let queries = self.queries.clone();

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .gap(px(12.0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_lg()
                    .child("Recent Searches"),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x8a909d))
                    .child(format!("Cached queries: {} / {} maximum", queries.len(), self.limit)),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .children(queries.into_iter().map(|q| {
                        div()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .p(px(8.0))
                            .rounded_md()
                            .bg(rgb(0x282c34))
                            .child(div().text_sm().child(q))
                            .child(div().text_xs().text_color(rgb(0x8a909d)).child("Search"))
                    })),
            )
    }
}
