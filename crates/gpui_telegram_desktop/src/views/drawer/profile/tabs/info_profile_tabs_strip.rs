//! Info Profile Tabs Strip component
//! Upstream: `Telegram/SourceFiles/info/profile/tabs/info_profile_tabs_strip.cpp:L1-L533`
//! Upstream: `Telegram/SourceFiles/info/profile/tabs/info_profile_tabs_strip.h:L1-L110`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use super::info_profile_tab_content::ProfileTabKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StripTabItem {
    pub kind: ProfileTabKind,
    pub title: String,
    pub count: Option<usize>,
}

/// Upstream `TabsStrip`: scrollable tab bar with active indicator underline and wheel physics.
#[derive(Debug, Clone)]
pub struct InfoProfileTabsStripProfileTabs {
    pub tabs: Vec<StripTabItem>,
    pub selected_index: usize,
    pub indicator_x: f32,
    pub indicator_width: f32,
    pub scroll_left: f32,
}

impl InfoProfileTabsStripProfileTabs {
    /// Upstream `kScrollDuration = 150ms` (`info_profile_tabs_strip.cpp:L25`).
    pub const SCROLL_DURATION_MS: u64 = 150;
    /// Upstream `kVerticalScrollGrace = 250ms` (`info_profile_tabs_strip.cpp:L26`).
    pub const VERTICAL_SCROLL_GRACE_MS: u64 = 250;
    /// Upstream `kVerticalScrollRest = 1000ms` (`info_profile_tabs_strip.cpp:L27`).
    pub const VERTICAL_SCROLL_REST_MS: u64 = 1000;
    /// Upstream `kF = 0.5` (`info_profile_tabs_strip.cpp:L515`).
    pub const K_F: f32 = 0.5;

    pub fn new(tabs: Vec<StripTabItem>) -> Self {
        Self {
            tabs,
            selected_index: 0,
            indicator_x: 0.0,
            indicator_width: 48.0,
            scroll_left: 0.0,
        }
    }

    pub fn select_tab(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.selected_index = index;
            self.update_indicator();
        }
    }

    pub fn update_indicator(&mut self) {
        // Tab width approximation: 64px tab spacing
        let tab_width = 64.0;
        self.indicator_x = self.selected_index as f32 * tab_width;
        self.indicator_width = tab_width;
    }

    pub fn scroll_by(&mut self, delta: f32) {
        self.scroll_left = (self.scroll_left + delta * Self::K_F).max(0.0);
    }
}

impl Default for InfoProfileTabsStripProfileTabs {
    fn default() -> Self {
        Self::new(vec![
            StripTabItem { kind: ProfileTabKind::Media, title: "Media".into(), count: None },
            StripTabItem { kind: ProfileTabKind::Files, title: "Files".into(), count: None },
            StripTabItem { kind: ProfileTabKind::Links, title: "Links".into(), count: None },
        ])
    }
}

impl Render for InfoProfileTabsStripProfileTabs {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let selected = self.selected_index;

        div()
            .flex()
            .flex_row()
            .items_center()
            .w_full()
            .h(px(40.0))
            .px(px(8.0))
            .bg(rgb(0xFFFFFF))
            .border_b_1()
            .border_color(rgb(0xE6E8EB))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(16.0))
                    .children(self.tabs.iter().enumerate().map(|(i, tab)| {
                        let is_active = i == selected;
                        div()
                            .flex()
                            .items_center()
                            .h_full()
                            .px(px(4.0))
                            .font_weight(if is_active { FontWeight::BOLD } else { FontWeight::NORMAL })
                            .text_color(if is_active { rgb(0x2AABEE) } else { rgb(0x707579) })
                            .child(tab.title.clone())
                    }))
            )
    }
}
