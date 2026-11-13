//! Target Symbol: IvSearchController
//! Module 13: Instant View Search Indexer & Circular Navigation Controller
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchMatch {
    pub segment_id: usize,
    pub start: usize,
    pub end: usize,
    pub matched_text: String,
}

#[derive(Debug, Clone)]
pub struct SearchSourceSegment {
    pub segment_id: usize,
    pub text: String,
}

#[derive(Debug, Clone, Default)]
pub struct IvSearchController {
    pub sources: Vec<SearchSourceSegment>,
    pub matches: Vec<SearchMatch>,
    pub current_index: usize,
    pub active_query: String,
    pub is_searching: bool,
}

impl IvSearchController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_sources(&mut self, sources: Vec<SearchSourceSegment>) {
        self.sources = sources;
        if !self.active_query.is_empty() {
            let q = self.active_query.clone();
            self.perform_search(&q);
        }
    }

    pub fn perform_search(&mut self, query: &str) -> usize {
        self.active_query = query.to_string();
        self.matches.clear();
        self.current_index = 0;

        let q_trimmed = query.trim();
        if q_trimmed.is_empty() {
            self.is_searching = false;
            return 0;
        }

        self.is_searching = true;
        let query_lower = q_trimmed.to_lowercase();

        for source in &self.sources {
            let text_lower = source.text.to_lowercase();
            let mut search_from = 0;

            while let Some(pos) = text_lower[search_from..].find(&query_lower) {
                let actual_start = search_from + pos;
                let actual_end = actual_start + query_lower.len();

                let matched_text = if actual_end <= source.text.len() {
                    source.text[actual_start..actual_end].to_string()
                } else {
                    query_lower.clone()
                };

                self.matches.push(SearchMatch {
                    segment_id: source.segment_id,
                    start: actual_start,
                    end: actual_end,
                    matched_text,
                });

                search_from = actual_end.max(search_from + 1);
                if search_from >= text_lower.len() {
                    break;
                }
            }
        }

        self.matches.len()
    }

    pub fn next_match(&mut self) -> Option<&SearchMatch> {
        if self.matches.is_empty() {
            return None;
        }
        self.current_index = (self.current_index + 1) % self.matches.len();
        self.matches.get(self.current_index)
    }

    pub fn previous_match(&mut self) -> Option<&SearchMatch> {
        if self.matches.is_empty() {
            return None;
        }
        self.current_index = if self.current_index == 0 {
            self.matches.len() - 1
        } else {
            self.current_index - 1
        };
        self.matches.get(self.current_index)
    }

    pub fn current_match(&self) -> Option<&SearchMatch> {
        self.matches.get(self.current_index)
    }

    pub fn clear_search(&mut self) {
        self.active_query.clear();
        self.matches.clear();
        self.current_index = 0;
        self.is_searching = false;
    }
}

impl Render for IvSearchController {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let total = self.matches.len();
        let display_idx = if total == 0 { 0 } else { self.current_index + 1 };

        div()
            .flex()
            .flex_col()
            .p(px(8.0))
            .bg(rgb(0x1B2028))
            .rounded(px(6.0))
            .gap(px(4.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .text_size(px(12.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xCCD2DE))
                    .child(if self.is_searching {
                        format!("Search Results: {} of {}", display_idx, total)
                    } else {
                        "Search Indexer Idle".to_string()
                    }),
            )
            .child(
                div()
                    .text_size(px(11.0))
                    .text_color(rgb(0x757F90))
                    .child(format!(
                        "Indexed Segments: {} | Query: \"{}\"",
                        self.sources.len(),
                        self.active_query
                    )),
            )
    }
}
