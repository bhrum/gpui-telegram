//! Tabbed Panel Responsive Container
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/chat_helpers/tabbed_panel.h:L1-L132`
//! - `Telegram/SourceFiles/chat_helpers/tabbed_panel.cpp:L1-L526`
//! - `Telegram/SourceFiles/chat_helpers/tabbed_panel.cpp:L25-L26` (`kHideTimeoutMs = 300`, `kDelayedHideTimeoutMs = 3000`)
//! - `Telegram/SourceFiles/chat_helpers/tabbed_panel.cpp:L194-L222` (`updateContentHeight` math)
//!
//! Subsystem: chat_helpers, Feature: TCG-N11

use gpui::{px, Pixels};
use std::time::Duration;

pub const K_HIDE_TIMEOUT: Duration = Duration::from_millis(300);
pub const K_DELAYED_HIDE_TIMEOUT: Duration = Duration::from_millis(3000);
pub const K_OPTION_TABBED_PANEL_SHOW_ON_CLICK: &str = "tabbed-panel-show-on-click";

/// Geometry and auto-hide controller for the Tabbed Panel floating window.
/// Target symbol in ledger: `TabbedPanel`.
#[derive(Debug, Clone)]
pub struct TabbedPanel {
    pub is_visible: bool,
    pub is_animating: bool,
    pub drop_down: bool,
    pub show_on_click_only: bool,

    // Height ratio and constraints from tabbed_panel.cpp:L71-L73
    pub height_ratio: f32,
    pub min_content_height: Pixels,
    pub max_content_height: Pixels,
    pub content_height: Pixels,
    pub panel_width: Pixels,

    // Position coordinates
    pub bottom: Pixels,
    pub top: Pixels,
    pub right: Pixels,
    pub computed_top: Pixels,
    pub computed_right: Pixels,
}

impl Default for TabbedPanel {
    fn default() -> Self {
        Self {
            is_visible: false,
            is_animating: false,
            drop_down: false,
            show_on_click_only: false,
            height_ratio: 0.75,
            min_content_height: px(278.0),
            max_content_height: px(640.0),
            content_height: px(400.0),
            panel_width: px(345.0),
            bottom: px(0.0),
            top: px(0.0),
            right: px(0.0),
            computed_top: px(0.0),
            computed_right: px(0.0),
        }
    }
}

impl TabbedPanel {
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate dynamic content height from parent dimensions.
    /// Directly mirrors `tabbed_panel.cpp:L194-L222`.
    pub fn update_content_height(
        &mut self,
        parent_height: Pixels,
        padding_vertical: Pixels,
        margins_height: Pixels,
    ) {
        let available_height = if self.drop_down {
            f32::from(parent_height) - f32::from(self.top) - f32::from(margins_height)
        } else {
            f32::from(self.bottom) - f32::from(margins_height)
        };

        let wanted = (self.height_ratio * available_height).round();
        let wanted_content = wanted - f32::from(padding_vertical);
        let clamped = wanted_content.clamp(
            f32::from(self.min_content_height),
            f32::from(self.max_content_height),
        );

        self.content_height = px(f32::from(margins_height) + clamped);

        self.computed_top = if self.drop_down {
            self.top
        } else {
            px(f32::from(self.bottom) - f32::from(padding_vertical) - f32::from(self.content_height))
        };
    }

    /// Calculate horizontal offset.
    /// Directly mirrors `tabbed_panel.cpp:L255-L263`.
    pub fn update_horizontal_position(&mut self, parent_width: Pixels) {
        let width = f32::from(self.panel_width);
        let right_bound = f32::from(self.right).max(width);
        let right_pos = (f32::from(parent_width) - right_bound).max(0.0);
        self.computed_right = px(right_pos);
    }

    pub fn show(&mut self) {
        self.is_visible = true;
    }

    pub fn hide_fast(&mut self) {
        self.is_visible = false;
        self.is_animating = false;
    }
}
