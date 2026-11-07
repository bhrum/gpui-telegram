//! Authentic HistoryView Object Base Component (`history_view_object.h`).
//! Upstream reference: `Telegram/SourceFiles/history/view/history_view_object.h:L12-L73`

use gpui::*;
use serde::{Deserialize, Serialize};

/// Two-dimensional size descriptor used in HistoryView::Object layout.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct LayoutSize {
    pub width: i32,
    pub height: i32,
}

impl LayoutSize {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }
}

/// Authentic GPUI representation of `HistoryView::Object`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoryViewObjectView {
    pub max_width: i32,
    pub min_height: i32,
    pub width: i32,
    pub height: i32,
    pub is_initialized: bool,
}

impl HistoryViewObjectView {
    pub fn new() -> Self {
        Self {
            max_width: 0,
            min_height: 0,
            width: 0,
            height: 0,
            is_initialized: false,
        }
    }

    /// Sets optimal size during first layout pass (`history_view_object.h:L18-L20`).
    pub fn init_dimensions(&mut self, optimal: LayoutSize) {
        self.set_optimal_size(optimal);
        self.is_initialized = true;
    }

    /// Computes and updates current size given width constraint (`history_view_object.h:L21-L24`).
    pub fn resize_get_height(&mut self, new_width: i32, height: i32) -> i32 {
        self.set_current_size(LayoutSize::new(new_width, height));
        self.height
    }

    pub fn set_optimal_size(&mut self, size: LayoutSize) {
        self.max_width = size.width;
        self.min_height = size.height;
    }

    pub fn set_current_size(&mut self, size: LayoutSize) {
        self.width = size.width;
        self.height = size.height;
    }

    pub fn adjust_height(&mut self, delta: i32) {
        self.height += delta;
    }

    pub fn optimal_size(&self) -> LayoutSize {
        LayoutSize::new(self.max_width, self.min_height)
    }

    pub fn current_size(&self) -> LayoutSize {
        LayoutSize::new(self.width, self.height)
    }
}

impl Default for HistoryViewObjectView {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewObjectView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w(px(self.width as f32))
            .h(px(self.height as f32))
            .bg(rgb(0xFFFFFF))
    }
}
