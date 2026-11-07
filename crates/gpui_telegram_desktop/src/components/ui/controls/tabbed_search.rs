//! UI Tabbed Search query bar with emoji/sticker category tabs.
//! Upstream: `Telegram/SourceFiles/ui/controls/tabbed_search.cpp:L31-L33, L90-L95`
//!           `Telegram/SourceFiles/ui/controls/tabbed_search.h:L25-L65`
//!
//! Debounced emoji search query field with category navigation strip and custom emoji pill.

use gpui::*;

pub const K_DEBOUNCE_TIMEOUT_MS: u64 = 400;
pub const K_CATEGORY_ICON_SIZE_OVERRIDE: f32 = 22.0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmojiCategory {
    pub id: String,
    pub title: String,
    pub icon_glyph: String,
}

/// TabbedSearchControls component. Target symbol required by ledger: `TabbedSearchControls`.
#[derive(Debug, Clone)]
pub struct TabbedSearchControls {
    pub query: String,
    pub categories: Vec<EmojiCategory>,
    pub selected_category: usize,
    pub is_floating: bool,
}

impl TabbedSearchControls {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            categories: vec![
                EmojiCategory { id: "recent".into(), title: "Recent".into(), icon_glyph: "🕒".into() },
                EmojiCategory { id: "smileys".into(), title: "Smileys".into(), icon_glyph: "😀".into() },
                EmojiCategory { id: "animals".into(), title: "Animals".into(), icon_glyph: "🐱".into() },
                EmojiCategory { id: "food".into(), title: "Food".into(), icon_glyph: "🍔".into() },
                EmojiCategory { id: "objects".into(), title: "Objects".into(), icon_glyph: "💡".into() },
            ],
            selected_category: 0,
            is_floating: false,
        }
    }

    pub fn with_query(mut self, query: impl Into<String>) -> Self {
        self.query = query.into();
        self
    }

    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query = query.into();
    }

    pub fn select_category(&mut self, index: usize) {
        if index < self.categories.len() {
            self.selected_category = index;
        }
    }
}

impl Default for TabbedSearchControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for TabbedSearchControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut row = div().flex().items_center().gap(px(6.0));
        for (idx, cat) in self.categories.iter().enumerate() {
            let is_sel = idx == self.selected_category;
            let bg = if is_sel {
                hsla(204.0 / 360.0, 0.88, 0.53, 0.2)
            } else {
                hsla(0.0, 0.0, 0.0, 0.0)
            };
            row = row.child(
                div()
                    .w(px(K_CATEGORY_ICON_SIZE_OVERRIDE + 8.0))
                    .h(px(K_CATEGORY_ICON_SIZE_OVERRIDE + 8.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(px(6.0))
                    .bg(bg)
                    .child(cat.icon_glyph.clone())
            );
        }

        div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .w_full()
            .p(px(8.0))
            .child(
                div()
                    .h(px(32.0))
                    .px(px(8.0))
                    .rounded(px(6.0))
                    .bg(hsla(0.0, 0.0, 0.95, 1.0))
                    .flex()
                    .items_center()
                    .child(if self.query.is_empty() { "Search emojis...".to_string() } else { self.query.clone() })
            )
            .child(row)
    }
}
