//! Authentic GPUI Chat Filters Tabs Slider Component.
//! Reference: Telegram/SourceFiles/ui/widgets/chat_filters_tabs_slider.cpp:L19-L100
//! Reference: Telegram/SourceFiles/ui/widgets/chat_filters_tabs_slider.h:L28-L80

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use super::chat_filters_tabs_mode::ChatsFiltersTabsMode;

/// Formatted badge string representation capping high counters at "99+".
/// Reference: `chat_filters_tabs_slider.cpp:L29`
pub const UNREAD_MAX_STRING: &str = "99+";

/// Spacing multiplier for unread badge placement: `st::lineWidth * 5`.
/// Reference: `chat_filters_tabs_slider.cpp:L30`
pub const UNREAD_SKIP_FACTOR: f32 = 5.0;

/// Default icon skip in dialog tabs.
/// Reference: `chat_filters_tabs_slider.cpp:L31`
pub const DEFAULT_ICON_SKIP: f32 = 8.0;

#[derive(Debug, Clone, PartialEq)]
pub struct FilterTabSection {
    pub id: i32,
    pub title: String,
    pub icon_name: Option<String>,
    pub unread_count: i32,
    pub is_muted: bool,
    pub horizontal_shift: f32,
    pub is_raised: bool,
    pub width: f32,
    pub left: f32,
}

impl FilterTabSection {
    pub fn new(id: i32, title: impl Into<String>) -> Self {
        Self {
            id,
            title: title.into(),
            icon_name: None,
            unread_count: 0,
            is_muted: false,
            horizontal_shift: 0.0,
            is_raised: false,
            width: 80.0,
            left: 0.0,
        }
    }

    pub fn unread_label(&self) -> Option<String> {
        if self.unread_count <= 0 {
            None
        } else if self.unread_count > 99 {
            Some(UNREAD_MAX_STRING.to_string())
        } else {
            Some(self.unread_count.to_string())
        }
    }
}

/// Target symbol for chat folder filter tab strip slider.
#[derive(Debug, Clone)]
pub struct ChatFiltersTabsSliderWidgets {
    pub sections: Vec<FilterTabSection>,
    pub active_index: usize,
    pub locked_from: Option<usize>,
    pub tabs_mode: ChatsFiltersTabsMode,
    pub bar_radius: f32,
    pub bar_left: f32,
    pub bar_width: f32,
    pub line_width: f32,
    pub is_reordering: bool,
}

impl Default for ChatFiltersTabsSliderWidgets {
    fn default() -> Self {
        Self::new()
    }
}

impl ChatFiltersTabsSliderWidgets {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            active_index: 0,
            locked_from: None,
            tabs_mode: ChatsFiltersTabsMode::Default,
            bar_radius: 3.0,
            bar_left: 0.0,
            bar_width: 0.0,
            line_width: 1.0,
            is_reordering: false,
        }
    }

    pub fn set_sections(&mut self, sections: Vec<FilterTabSection>) {
        self.sections = sections;
        if self.active_index >= self.sections.len() && !self.sections.is_empty() {
            self.active_index = 0;
        }
        self.recompute_layout();
    }

    pub fn set_active_index(&mut self, index: usize) {
        if index < self.sections.len() {
            self.active_index = index;
            self.recompute_bar();
        }
    }

    pub fn set_unread_count(&mut self, index: usize, count: i32, muted: bool) {
        if let Some(sec) = self.sections.get_mut(index) {
            sec.unread_count = count;
            sec.is_muted = muted;
            self.recompute_layout();
        }
    }

    pub fn set_locked_from(&mut self, index: Option<usize>) {
        self.locked_from = index;
    }

    pub fn set_horizontal_shift(&mut self, index: usize, shift: f32) {
        if let Some(sec) = self.sections.get_mut(index) {
            sec.horizontal_shift = shift;
        }
    }

    pub fn reorder_sections(&mut self, old_index: usize, new_index: usize) {
        if old_index < self.sections.len() && new_index < self.sections.len() && old_index != new_index {
            let item = self.sections.remove(old_index);
            self.sections.insert(new_index, item);
            if self.active_index == old_index {
                self.active_index = new_index;
            } else if old_index < self.active_index && new_index >= self.active_index {
                self.active_index = self.active_index.saturating_sub(1);
            } else if old_index > self.active_index && new_index <= self.active_index {
                self.active_index = self.active_index.saturating_add(1);
            }
            self.recompute_layout();
        }
    }

    pub fn recompute_layout(&mut self) {
        let mut x = 0.0f32;
        for sec in &mut self.sections {
            sec.left = x;
            let text_w = sec.title.len() as f32 * 8.0 + 16.0;
            let icon_w = if sec.icon_name.is_some() { 18.0 + DEFAULT_ICON_SKIP } else { 0.0 };
            let badge_w = if sec.unread_count > 0 { 20.0 + self.line_width * UNREAD_SKIP_FACTOR } else { 0.0 };
            sec.width = text_w + icon_w + badge_w;
            x += sec.width;
        }
        self.recompute_bar();
    }

    fn recompute_bar(&mut self) {
        if let Some(sec) = self.sections.get(self.active_index) {
            self.bar_left = sec.left;
            self.bar_width = sec.width;
        } else {
            self.bar_left = 0.0;
            self.bar_width = 0.0;
        }
    }

    /// Calculates the horizontal pixel coordinate where premium locked tabs begin.
    /// Reference: `chat_filters_tabs_slider.cpp:L79`
    pub fn calculate_locked_from_x(&self) -> Option<f32> {
        self.locked_from.and_then(|idx| self.sections.get(idx)).map(|sec| sec.left)
    }
}
