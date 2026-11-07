//! Authentic GPUI Chat Filters Tabs Drag-to-Reorder Controller.
//! Reference: Telegram/SourceFiles/ui/widgets/chat_filters_tabs_slider_reorder.cpp:L18-L60
//! Reference: Telegram/SourceFiles/ui/widgets/chat_filters_tabs_slider_reorder.h:L24-L60

#![allow(dead_code, unused_imports, unused_variables)]

/// Proportional edge auto-scrolling speed factor when dragging near viewport boundary.
/// Reference: `chat_filters_tabs_slider_reorder.cpp:L20`
pub const K_SCROLL_FACTOR: f32 = 0.05;

#[derive(Debug, Clone, PartialEq)]
pub struct ReorderEntry {
    pub initial_index: usize,
    pub current_index: usize,
    pub drag_offset_x: f32,
    pub is_dragging: bool,
}

/// Target symbol managing interactive tab reordering gestures.
#[derive(Debug, Clone)]
pub struct ChatFiltersTabsSliderReorderWidgets {
    pub is_active: bool,
    pub dragged_index: Option<usize>,
    pub start_mouse_x: f32,
    pub current_mouse_x: f32,
    pub scroll_offset: f32,
    pub scroll_factor: f32,
    pub entries: Vec<ReorderEntry>,
}

impl Default for ChatFiltersTabsSliderReorderWidgets {
    fn default() -> Self {
        Self::new()
    }
}

impl ChatFiltersTabsSliderReorderWidgets {
    pub fn new() -> Self {
        Self {
            is_active: false,
            dragged_index: None,
            start_mouse_x: 0.0,
            current_mouse_x: 0.0,
            scroll_offset: 0.0,
            scroll_factor: K_SCROLL_FACTOR,
            entries: Vec::new(),
        }
    }

    /// Initializes a drag-and-drop reorder operation.
    /// Reference: `chat_filters_tabs_slider_reorder.cpp:L48-L60`
    pub fn start(&mut self, item_count: usize, index: usize, mouse_x: f32) {
        if item_count < 2 || index >= item_count {
            return;
        }
        self.is_active = true;
        self.dragged_index = Some(index);
        self.start_mouse_x = mouse_x;
        self.current_mouse_x = mouse_x;
        self.entries = (0..item_count)
            .map(|i| ReorderEntry {
                initial_index: i,
                current_index: i,
                drag_offset_x: 0.0,
                is_dragging: i == index,
            })
            .collect();
    }

    /// Updates tab displacement as pointer moves.
    /// Reference: `chat_filters_tabs_slider_reorder.cpp:L55-L60`
    pub fn mouse_move(&mut self, mouse_x: f32, tab_widths: &[f32], viewport_width: f32) {
        if !self.is_active || self.dragged_index.is_none() {
            return;
        }
        self.current_mouse_x = mouse_x;
        let delta_x = mouse_x - self.start_mouse_x;

        if let Some(dragged) = self.dragged_index {
            if let Some(entry) = self.entries.get_mut(dragged) {
                entry.drag_offset_x = delta_x;
            }

            // Edge auto-scrolling calculation
            if mouse_x < 30.0 {
                let dist = 30.0 - mouse_x;
                self.scroll_offset -= dist * self.scroll_factor;
            } else if mouse_x > viewport_width - 30.0 {
                let dist = mouse_x - (viewport_width - 30.0);
                self.scroll_offset += dist * self.scroll_factor;
            }
        }
    }

    /// Cancels reorder and resets horizontal shifts.
    /// Reference: `chat_filters_tabs_slider_reorder.cpp:L37-L47`
    pub fn cancel(&mut self) {
        self.is_active = false;
        self.dragged_index = None;
        self.start_mouse_x = 0.0;
        self.current_mouse_x = 0.0;
        self.entries.clear();
    }

    /// Completes the reorder gesture and returns old and new indices if moved.
    pub fn finish(&mut self) -> Option<(usize, usize)> {
        if !self.is_active {
            return None;
        }
        let dragged = self.dragged_index?;
        let entry = self.entries.get(dragged)?;
        let result = if entry.initial_index != entry.current_index {
            Some((entry.initial_index, entry.current_index))
        } else {
            None
        };
        self.cancel();
        result
    }
}
