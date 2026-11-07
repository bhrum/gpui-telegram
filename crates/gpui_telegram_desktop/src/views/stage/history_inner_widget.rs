//! History Inner Widget (Message Transcript Layout, Virtualization, Floating Date).
//!
//! Citations:
//! - Upstream: `Telegram/SourceFiles/history/history_inner_widget.h:L88-L160`
//! - Upstream: `Telegram/SourceFiles/history/history_inner_widget.cpp:L140-L145`
//! - Anonymous Constants:
//!   - `kScrollDateHideTimeout = 1000;` (1s, history_inner_widget.cpp:L142)
//!   - `kScrollDateHideOnDayCrossingTimeout = 3000;` (3s, history_inner_widget.cpp:L143)
//!   - `kUnloadHeavyPartsPages = 2;` (history_inner_widget.cpp:L144)
//!   - `kClearUserpicsAfter = 50;` (history_inner_widget.cpp:L145)

use gpui::*;
use super::history_message_selection::HistoryMessageSelection;

pub const SCROLL_DATE_HIDE_TIMEOUT_MS: u64 = 1000;
pub const SCROLL_DATE_HIDE_DAY_CROSSING_TIMEOUT_MS: u64 = 3000;
pub const UNLOAD_HEAVY_PARTS_PAGES: f32 = 2.0;
pub const CLEAR_USERPICS_AFTER: usize = 50;

/// Rendered transcript item bounding geometry.
#[derive(Debug, Clone)]
pub struct MessageItemGeometry {
    pub message_id: u64,
    pub top: f32,
    pub height: f32,
    pub date_timestamp: u64,
    pub text_snippet: String,
}

/// Inner transcript scroll widget managing virtualization, floating date, and selections.
/// C++ citation: `HistoryInner` / `HistoryInnerWidget` (history_inner_widget.h:L88-L160).
pub struct HistoryInnerWidget {
    items: Vec<MessageItemGeometry>,
    scroll_top: f32,
    visible_height: f32,
    total_height: f32,
    floating_date: Option<String>,
    floating_date_visible: bool,
    floating_date_timer_ms: u64,
    is_day_crossing: bool,
    userpic_cache_count: usize,
    pub selection: HistoryMessageSelection,
}

impl HistoryInnerWidget {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            scroll_top: 0.0,
            visible_height: 600.0,
            total_height: 0.0,
            floating_date: None,
            floating_date_visible: false,
            floating_date_timer_ms: 0,
            is_day_crossing: false,
            userpic_cache_count: 0,
            selection: HistoryMessageSelection::new(),
        }
    }

    /// Sets transcript items and recomputes total content height.
    pub fn set_items(&mut self, items: Vec<MessageItemGeometry>) {
        let mut curr_y = 0.0;
        let mut positioned = Vec::with_capacity(items.len());
        for mut item in items {
            item.top = curr_y;
            curr_y += item.height;
            positioned.push(item);
        }
        self.total_height = curr_y;
        self.items = positioned;
    }

    /// Updates viewport scroll position and updates sticky floating date.
    pub fn update_scroll(&mut self, scroll_top: f32, visible_height: f32, now_ms: u64) {
        let prev_top = self.scroll_top;
        self.scroll_top = scroll_top.clamp(0.0, (self.total_height - visible_height).max(0.0));
        self.visible_height = visible_height;

        // Find top-most visible item
        if let Some(item) = self.first_visible_item() {
            let date_str = Self::format_date_label(item.date_timestamp);
            let day_changed = if let Some(ref prev) = self.floating_date {
                prev != &date_str
            } else {
                false
            };
            self.floating_date = Some(date_str);
            self.floating_date_visible = true;
            self.is_day_crossing = day_changed;
            self.floating_date_timer_ms = now_ms;
        }

        // Periodic userpic cache prune
        if (prev_top - scroll_top).abs() > 200.0 {
            self.userpic_cache_count = self.userpic_cache_count.saturating_add(1);
            if self.userpic_cache_count >= CLEAR_USERPICS_AFTER {
                self.userpic_cache_count = 0;
            }
        }
    }

    /// Checks if floating date should fade out at `now_ms`.
    /// C++ citation: `history_inner_widget.cpp:L142-L143`.
    pub fn check_floating_date_fade(&mut self, now_ms: u64) {
        if !self.floating_date_visible {
            return;
        }
        let timeout = if self.is_day_crossing {
            SCROLL_DATE_HIDE_DAY_CROSSING_TIMEOUT_MS
        } else {
            SCROLL_DATE_HIDE_TIMEOUT_MS
        };
        if now_ms.saturating_sub(self.floating_date_timer_ms) >= timeout {
            self.floating_date_visible = false;
        }
    }

    /// Virtualization bounds check: determines if an item is within the 2-page render window.
    /// C++ citation: `history_inner_widget.cpp:L144` (`kUnloadHeavyPartsPages = 2`).
    pub fn is_item_in_render_window(&self, item: &MessageItemGeometry) -> bool {
        let buffer = self.visible_height * UNLOAD_HEAVY_PARTS_PAGES;
        let window_top = (self.scroll_top - buffer).max(0.0);
        let window_bottom = self.scroll_top + self.visible_height + buffer;
        item.top + item.height >= window_top && item.top <= window_bottom
    }

    /// Returns first item intersecting the current visible viewport.
    pub fn first_visible_item(&self) -> Option<&MessageItemGeometry> {
        self.items.iter().find(|item| {
            item.top + item.height > self.scroll_top && item.top < self.scroll_top + self.visible_height
        })
    }

    pub fn visible_items(&self) -> Vec<&MessageItemGeometry> {
        self.items
            .iter()
            .filter(|item| self.is_item_in_render_window(item))
            .collect()
    }

    fn format_date_label(timestamp: u64) -> String {
        if timestamp == 0 {
            "Today".to_string()
        } else {
            format!("Day {}", timestamp / 86400)
        }
    }

    pub fn total_height(&self) -> f32 {
        self.total_height
    }

    pub fn scroll_top(&self) -> f32 {
        self.scroll_top
    }

    pub fn floating_date(&self) -> Option<&str> {
        if self.floating_date_visible {
            self.floating_date.as_deref()
        } else {
            None
        }
    }
}

impl Default for HistoryInnerWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryInnerWidget {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let visible_items = self.visible_items();

        div()
            .relative()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(rgb(0xF3F4F6))
            .child(
                // Floating sticky date separator chip
                if let Some(date_text) = self.floating_date() {
                    div()
                        .absolute()
                        .top(px(8.0))
                        .left_0()
                        .right_0()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .px(px(10.0))
                                .py(px(3.0))
                                .rounded_full()
                                .bg(rgba(0x00000044))
                                .text_color(rgb(0xFFFFFF))
                                .text_size(px(12.0))
                                .font_weight(FontWeight::MEDIUM)
                                .child(date_text.to_string())
                        )
                } else {
                    div()
                }
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .p(px(16.0))
                    .gap(px(8.0))
                    .children(visible_items.into_iter().map(|item| {
                        let is_selected = self.selection.is_selected(item.message_id);
                        div()
                            .flex()
                            .items_center()
                            .p(px(8.0))
                            .rounded(px(8.0))
                            .bg(if is_selected { rgb(0xE0F2FE) } else { rgb(0xFFFFFF) })
                            .border_1()
                            .border_color(if is_selected { rgb(0x38BDF8) } else { rgb(0xE5E7EB) })
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(rgb(0x1F2937))
                                    .child(item.text_snippet.clone())
                            )
                    }))
            )
    }
}
