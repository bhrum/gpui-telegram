//! Instant View Slash Insert Suggestions Controller (`iv_editor_insert_suggestions.h/.cpp`)
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

/// Complete set of 21 insertion commands supported via autocomplete popup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InsertSuggestionCommand {
    Heading1,
    Heading2,
    Heading3,
    Heading4,
    Heading5,
    Heading6,
    Blockquote,
    Pullquote,
    Code,
    Footer,
    BulletList,
    OrderedList,
    TaskList,
    Details,
    Button,
    Table,
    Math,
    Divider,
    Media,
    Audio,
    Map,
}

/// Item metadata for rendering and fuzzy matching in the suggestions list.
#[derive(Debug, Clone)]
pub struct SuggestionItem {
    pub command: InsertSuggestionCommand,
    pub title: &'static str,
    pub shortcut: &'static str,
    pub is_premium: bool,
}

pub fn all_suggestions() -> Vec<SuggestionItem> {
    vec![
        SuggestionItem { command: InsertSuggestionCommand::Heading1, title: "Heading 1", shortcut: "/h1", is_premium: false },
        SuggestionItem { command: InsertSuggestionCommand::Heading2, title: "Heading 2", shortcut: "/h2", is_premium: false },
        SuggestionItem { command: InsertSuggestionCommand::Heading3, title: "Heading 3", shortcut: "/h3", is_premium: false },
        SuggestionItem { command: InsertSuggestionCommand::Heading4, title: "Heading 4", shortcut: "/h4", is_premium: false },
        SuggestionItem { command: InsertSuggestionCommand::Heading5, title: "Heading 5", shortcut: "/h5", is_premium: false },
        SuggestionItem { command: InsertSuggestionCommand::Heading6, title: "Heading 6", shortcut: "/h6", is_premium: false },
        SuggestionItem { command: InsertSuggestionCommand::Blockquote, title: "Blockquote", shortcut: "/quote", is_premium: false },
        SuggestionItem { command: InsertSuggestionCommand::Pullquote, title: "Pullquote", shortcut: "/pullquote", is_premium: false },
        SuggestionItem { command: InsertSuggestionCommand::Code, title: "Code Block", shortcut: "/code", is_premium: false },
        SuggestionItem { command: InsertSuggestionCommand::Footer, title: "Article Footer", shortcut: "/footer", is_premium: false },
        SuggestionItem { command: InsertSuggestionCommand::BulletList, title: "Bullet List", shortcut: "/bullet", is_premium: false },
        SuggestionItem { command: InsertSuggestionCommand::OrderedList, title: "Numbered List", shortcut: "/number", is_premium: false },
        SuggestionItem { command: InsertSuggestionCommand::TaskList, title: "Task Checklist", shortcut: "/todo", is_premium: false },
        SuggestionItem { command: InsertSuggestionCommand::Details, title: "Collapsible Details", shortcut: "/details", is_premium: false },
        SuggestionItem { command: InsertSuggestionCommand::Button, title: "Interactive Button", shortcut: "/button", is_premium: false },
        SuggestionItem { command: InsertSuggestionCommand::Table, title: "2D Data Table", shortcut: "/table", is_premium: true },
        SuggestionItem { command: InsertSuggestionCommand::Math, title: "LaTeX Formula", shortcut: "/math", is_premium: true },
        SuggestionItem { command: InsertSuggestionCommand::Divider, title: "Horizontal Line", shortcut: "/hr", is_premium: false },
        SuggestionItem { command: InsertSuggestionCommand::Media, title: "Attach Photo/Video", shortcut: "/media", is_premium: false },
        SuggestionItem { command: InsertSuggestionCommand::Audio, title: "Audio Clip", shortcut: "/audio", is_premium: false },
        SuggestionItem { command: InsertSuggestionCommand::Map, title: "Location Map", shortcut: "/map", is_premium: false },
    ]
}

/// Instant View inline suggestions popup controller.
#[derive(Debug, Clone)]
pub struct IvEditorInsertSuggestionsEditor {
    pub query: String,
    pub is_active: bool,
    pub filtered: Vec<SuggestionItem>,
    pub selected_index: usize,
    pub premium_enabled: bool,
}

impl Default for IvEditorInsertSuggestionsEditor {
    fn default() -> Self {
        let all = all_suggestions();
        Self {
            query: String::new(),
            is_active: false,
            filtered: all,
            selected_index: 0,
            premium_enabled: false,
        }
    }
}

impl IvEditorInsertSuggestionsEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply_query(&mut self, query: &str) -> bool {
        self.query = query.to_string();
        let q = query.to_lowercase();
        let all = all_suggestions();

        self.filtered = all
            .into_iter()
            .filter(|item| {
                if !self.premium_enabled && item.is_premium {
                    return false;
                }
                if q.is_empty() {
                    return true;
                }
                item.shortcut.to_lowercase().contains(&q) || item.title.to_lowercase().contains(&q)
            })
            .collect();

        self.selected_index = 0;
        self.is_active = !self.filtered.is_empty();
        self.is_active
    }

    pub fn select_next(&mut self) {
        if !self.filtered.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.filtered.len();
        }
    }

    pub fn select_previous(&mut self) {
        if !self.filtered.is_empty() {
            if self.selected_index == 0 {
                self.selected_index = self.filtered.len() - 1;
            } else {
                self.selected_index -= 1;
            }
        }
    }

    pub fn choose_selected(&mut self) -> Option<InsertSuggestionCommand> {
        let item = self.filtered.get(self.selected_index).cloned();
        self.is_active = false;
        item.map(|i| i.command)
    }
}

impl Render for IvEditorInsertSuggestionsEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_active {
            return div().into_any_element();
        }

        let max_rows = 5.min(self.filtered.len());
        div()
            .flex()
            .flex_col()
            .w(px(260.0))
            .p(px(4.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(8.0))
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .shadow_md()
            .children(self.filtered.iter().take(max_rows).enumerate().map(|(idx, item)| {
                let is_selected = idx == self.selected_index;
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .px(px(8.0))
                    .py(px(6.0))
                    .rounded(px(4.0))
                    .bg(if is_selected { rgb(0x2AABEE) } else { rgb(0xFFFFFF) })
                    .text_color(if is_selected { rgb(0xFFFFFF) } else { rgb(0x222222) })
                    .child(
                        div()
                            .text_size(px(13.0))
                            .child(item.title)
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .opacity(0.8)
                            .child(item.shortcut)
                    )
            }))
            .into_any_element()
    }
}
