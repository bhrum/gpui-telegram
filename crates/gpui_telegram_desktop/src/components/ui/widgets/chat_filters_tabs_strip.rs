//! Authentic GPUI Chat Filters Tabs Strip Container.
//! Reference: Telegram/SourceFiles/ui/widgets/chat_filters_tabs_strip.cpp:L45-L120
//! Reference: Telegram/SourceFiles/ui/widgets/chat_filters_tabs_strip.h:L18-L37

#![allow(dead_code, unused_imports, unused_variables)]

use super::chat_filters_tabs_slider::{ChatFiltersTabsSliderWidgets, FilterTabSection};
use super::chat_filters_tabs_slider_reorder::ChatFiltersTabsSliderReorderWidgets;

/// Supported context menu action types on chat folder tabs.
/// Reference: `chat_filters_tabs_strip.cpp:L24-L28`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FolderContextMenuAction {
    MarkAsRead,
    EditFilter,
    RemoveFilter,
    CreateNewFolder,
}

#[derive(Debug, Clone)]
pub struct ChatFolderTabItem {
    pub filter_id: i32,
    pub title: String,
    pub icon_name: Option<String>,
    pub unread_count: i32,
    pub is_muted: bool,
    pub is_locked: bool,
}

/// Target symbol for the entire chat filters tab strip with scroll viewport.
#[derive(Debug, Clone)]
pub struct ChatFiltersTabsStripWidgets {
    pub slider: ChatFiltersTabsSliderWidgets,
    pub reorder: ChatFiltersTabsSliderReorderWidgets,
    pub active_filter_id: Option<i32>,
    pub is_context_menu_open: bool,
    pub scroll_position: f32,
    pub max_scroll: f32,
    pub is_waiting_suggested: bool,
}

impl Default for ChatFiltersTabsStripWidgets {
    fn default() -> Self {
        Self::new()
    }
}

impl ChatFiltersTabsStripWidgets {
    pub fn new() -> Self {
        Self {
            slider: ChatFiltersTabsSliderWidgets::new(),
            reorder: ChatFiltersTabsSliderReorderWidgets::new(),
            active_filter_id: None,
            is_context_menu_open: false,
            scroll_position: 0.0,
            max_scroll: 0.0,
            is_waiting_suggested: false,
        }
    }

    pub fn set_folders(&mut self, items: Vec<ChatFolderTabItem>) {
        let mut sections = Vec::with_capacity(items.len());
        let mut locked_idx = None;

        for (idx, item) in items.iter().enumerate() {
            let mut sec = FilterTabSection::new(item.filter_id, &item.title);
            sec.icon_name = item.icon_name.clone();
            sec.unread_count = item.unread_count;
            sec.is_muted = item.is_muted;
            if item.is_locked && locked_idx.is_none() {
                locked_idx = Some(idx);
            }
            sections.push(sec);
        }

        self.slider.set_sections(sections);
        self.slider.set_locked_from(locked_idx);
    }

    pub fn select_filter(&mut self, filter_id: i32) {
        self.active_filter_id = Some(filter_id);
        if let Some(pos) = self.slider.sections.iter().position(|s| s.id == filter_id) {
            self.slider.set_active_index(pos);
        }
    }

    pub fn scroll_to_visible(&mut self, viewport_width: f32) {
        if let Some(sec) = self.slider.sections.get(self.slider.active_index) {
            let left = sec.left;
            let right = sec.left + sec.width;
            if left < self.scroll_position {
                self.scroll_position = left;
            } else if right > self.scroll_position + viewport_width {
                self.scroll_position = right - viewport_width;
            }
            self.scroll_position = self.scroll_position.clamp(0.0, self.max_scroll);
        }
    }

    pub fn open_context_menu(&mut self) {
        self.is_context_menu_open = true;
    }

    pub fn close_context_menu(&mut self) {
        self.is_context_menu_open = false;
    }
}
