//! Info Profile Tab Sub-Controller
//! Upstream: `Telegram/SourceFiles/info/profile/tabs/adapters/info_profile_tab_sub_controller.cpp:L1-L74`
//! Upstream: `Telegram/SourceFiles/info/profile/tabs/adapters/info_profile_tab_sub_controller.h:L1-L46`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use std::collections::HashMap;
use gpui::*;
use serde::{Deserialize, Serialize};

/// Controller managing nested scroll synchronization, header collapse offset,
/// and tab-switch scroll position memoization for profile tab adapters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InfoProfileTabSubControllerProfileTabsAdapters {
    pub current_scroll_top: f32,
    pub is_scrolling: bool,
    pub active_item_index: Option<usize>,
    pub header_max_height: f32,
    pub header_min_height: f32,
    pub header_collapse_offset: f32,
    pub viewport_height: f32,
    pub content_height: f32,
    pub tab_scroll_cache: HashMap<String, f32>,
}

impl Default for InfoProfileTabSubControllerProfileTabsAdapters {
    fn default() -> Self {
        Self {
            current_scroll_top: 0.0,
            is_scrolling: false,
            active_item_index: None,
            header_max_height: 240.0,
            header_min_height: 56.0,
            header_collapse_offset: 0.0,
            viewport_height: 600.0,
            content_height: 1200.0,
            tab_scroll_cache: HashMap::new(),
        }
    }
}

impl InfoProfileTabSubControllerProfileTabsAdapters {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_scroll_top(&mut self, top: f32) {
        self.current_scroll_top = top.max(0.0);
        self.update_header_collapse();
    }

    pub fn set_active_index(&mut self, index: Option<usize>) {
        self.active_item_index = index;
    }

    pub fn set_scrolling(&mut self, is_scrolling: bool) {
        self.is_scrolling = is_scrolling;
    }

    pub fn update_scroll_state(&mut self, scroll_top: f32, viewport_height: f32, content_height: f32) {
        self.current_scroll_top = scroll_top.max(0.0);
        self.viewport_height = viewport_height.max(1.0);
        self.content_height = content_height.max(self.viewport_height);
        self.update_header_collapse();
    }

    fn update_header_collapse(&mut self) {
        let max_collapse = (self.header_max_height - self.header_min_height).max(0.0);
        self.header_collapse_offset = self.current_scroll_top.clamp(0.0, max_collapse);
    }

    pub fn collapse_ratio(&self) -> f32 {
        let max_collapse = self.header_max_height - self.header_min_height;
        if max_collapse <= 0.0 {
            0.0
        } else {
            (self.header_collapse_offset / max_collapse).clamp(0.0, 1.0)
        }
    }

    pub fn save_tab_scroll(&mut self, tab_id: &str, scroll_top: f32) {
        self.tab_scroll_cache.insert(tab_id.to_string(), scroll_top.max(0.0));
    }

    pub fn restore_tab_scroll(&self, tab_id: &str) -> f32 {
        self.tab_scroll_cache.get(tab_id).copied().unwrap_or(0.0)
    }

    pub fn is_at_top(&self) -> bool {
        self.current_scroll_top <= 0.5
    }

    pub fn is_at_bottom(&self, threshold: f32) -> bool {
        let remaining = self.content_height - (self.current_scroll_top + self.viewport_height);
        remaining <= threshold
    }
}

impl Render for InfoProfileTabSubControllerProfileTabsAdapters {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let ratio = self.collapse_ratio();

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .h(px(28.0))
            .px(px(8.0))
            .bg(rgb(0xF5F7FA))
            .border_b_1()
            .border_color(rgb(0xE6E8EB))
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x707579))
                    .child(format!("Scroll: {:.0}px", self.current_scroll_top))
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(6.0))
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x2AABEE))
                            .child(format!("Collapse: {:.0}%", ratio * 100.0))
                    )
                    .child(
                        div()
                            .w(px(40.0))
                            .h(px(4.0))
                            .bg(rgb(0xDFE3E8))
                            .rounded_full()
                            .child(
                                div()
                                    .w(px(40.0 * ratio))
                                    .h_full()
                                    .bg(rgb(0x2AABEE))
                                    .rounded_full()
                            )
                    )
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_controller_scroll_and_collapse() {
        let mut ctrl = InfoProfileTabSubControllerProfileTabsAdapters::new();
        assert_eq!(ctrl.current_scroll_top, 0.0);
        assert_eq!(ctrl.collapse_ratio(), 0.0);
        assert!(ctrl.is_at_top());

        // Header max 240, min 56 -> max collapse 184
        ctrl.set_scroll_top(92.0);
        assert_eq!(ctrl.collapse_ratio(), 0.5);
        assert!(!ctrl.is_at_top());

        ctrl.set_scroll_top(300.0);
        assert_eq!(ctrl.header_collapse_offset, 184.0);
        assert_eq!(ctrl.collapse_ratio(), 1.0);

        // Tab scroll memoization
        ctrl.save_tab_scroll("media", 150.0);
        ctrl.save_tab_scroll("members", 320.0);
        assert_eq!(ctrl.restore_tab_scroll("media"), 150.0);
        assert_eq!(ctrl.restore_tab_scroll("members"), 320.0);
        assert_eq!(ctrl.restore_tab_scroll("unknown"), 0.0);

        // Bottom check
        ctrl.update_scroll_state(500.0, 500.0, 1100.0); // remaining = 100
        assert!(ctrl.is_at_bottom(150.0));
        assert!(!ctrl.is_at_bottom(50.0));
    }
}
