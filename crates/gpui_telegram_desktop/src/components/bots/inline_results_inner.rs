//! InlineResultsInner: Inner scrollable container for inline bot results gallery and list.
//!
//! Upstream reference:
//! - `Telegram/SourceFiles/inline_bots/inline_results_inner.h:L53-L100`
//! - `Telegram/SourceFiles/inline_bots/inline_results_inner.cpp:L43-L98`
#![allow(dead_code)]

use gpui::*;
use super::inline_bot_result::InlineBotResult;

/// Minimum repaint interval for inline animations (upstream: `kMinRepaintDelay = 33ms`).
pub const K_MIN_REPAINT_DELAY_MS: u64 = 33;

/// Minimum delay after scroll update before repainting (upstream: `kMinAfterScrollDelay = 33ms`).
pub const K_MIN_AFTER_SCROLL_DELAY_MS: u64 = 33;

/// Cached inline query response page (upstream: `InlineBots::Layout::CacheEntry`).
#[derive(Debug, Clone, Default)]
pub struct InlineCacheEntry {
    pub next_offset: String,
    pub switch_pm_text: Option<String>,
    pub switch_pm_start_token: Option<String>,
    pub switch_pm_url: Option<String>,
    pub results: Vec<InlineBotResult>,
    pub is_gallery: bool,
}

/// Authentic InlineResultsInner component state.
pub struct InlineResultsInner {
    pub cache: InlineCacheEntry,
    pub selected_index: Option<usize>,
    pub visible_top: f32,
    pub visible_bottom: f32,
    pub is_loading_more: bool,
    pub updated_at: i64,
}

impl InlineResultsInner {
    pub fn new() -> Self {
        Self {
            cache: InlineCacheEntry::default(),
            selected_index: None,
            visible_top: 0.0,
            visible_bottom: 400.0,
            is_loading_more: false,
            updated_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn set_results(&mut self, entry: InlineCacheEntry) {
        self.cache = entry;
        self.selected_index = if self.cache.results.is_empty() {
            None
        } else {
            Some(0)
        };
        self.is_loading_more = false;
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn append_results(&mut self, mut new_results: Vec<InlineBotResult>, next_offset: String) {
        self.cache.results.append(&mut new_results);
        self.cache.next_offset = next_offset;
        self.is_loading_more = false;
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn select_next(&mut self) {
        if self.cache.results.is_empty() {
            return;
        }
        let next = self.selected_index.map_or(0, |i| (i + 1) % self.cache.results.len());
        self.selected_index = Some(next);
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn select_prev(&mut self) {
        if self.cache.results.is_empty() {
            return;
        }
        let count = self.cache.results.len();
        let prev = self.selected_index.map_or(0, |i| if i == 0 { count - 1 } else { i - 1 });
        self.selected_index = Some(prev);
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn selected_result(&self) -> Option<&InlineBotResult> {
        self.selected_index.and_then(|idx| self.cache.results.get(idx))
    }

    pub fn can_load_more(&self) -> bool {
        !self.cache.next_offset.is_empty() && !self.is_loading_more
    }
}

impl Default for InlineResultsInner {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InlineResultsInner {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_gallery = self.cache.is_gallery;
        let selected = self.selected_index;

        div()
            .flex()
            .flex_col()
            .w_full()
            .bg(rgb(0xFFFFFF))
            .p(px(8.0))
            .gap(px(4.0))
            .children(if let Some(pm_text) = &self.cache.switch_pm_text {
                Some(
                    div()
                        .p(px(8.0))
                        .rounded(px(6.0))
                        .bg(rgb(0xF0F7FD))
                        .text_color(rgb(0x2AABEE))
                        .font_weight(FontWeight::BOLD)
                        .text_size(px(13.0))
                        .child(pm_text.clone())
                )
            } else {
                None
            })
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap(px(4.0))
                    .children(self.cache.results.iter().enumerate().map(|(idx, res)| {
                        let is_sel = selected == Some(idx);
                        if is_gallery {
                            div()
                                .w(px(72.0))
                                .h(px(72.0))
                                .rounded(px(4.0))
                                .bg(if is_sel { rgb(0xD0E5F7) } else { rgb(0xF2F2F2) })
                                .border_1()
                                .border_color(if is_sel { rgb(0x2AABEE) } else { rgb(0xE0E0E0) })
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(
                                    div()
                                        .text_size(px(11.0))
                                        .child(res.title.as_deref().unwrap_or("Media").to_string())
                                )
                        } else {
                            div()
                                .w_full()
                                .p(px(6.0))
                                .rounded(px(4.0))
                                .bg(if is_sel { rgb(0xF0F7FD) } else { rgb(0xFFFFFF) })
                                .border_b_1()
                                .border_color(rgb(0xEEEEEE))
                                .child(
                                    div()
                                        .font_weight(FontWeight::BOLD)
                                        .text_size(px(13.0))
                                        .child(res.title.as_deref().unwrap_or("Untitled").to_string())
                                )
                        }
                    }))
            )
    }
}
