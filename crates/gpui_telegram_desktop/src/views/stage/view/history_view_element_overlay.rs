//! Authentic HistoryView Element Overlay Host Component (`history_view_element_overlay.h/cpp`).
//! Upstream reference: `Telegram/SourceFiles/history/view/history_view_element_overlay.cpp:L1-L133`

use gpui::*;
use serde::{Deserialize, Serialize};

/// Interactive overlay state matching `HistoryView::ElementOverlayHost`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryViewElementOverlayView {
    pub is_active: bool,
    pub item_id: u64,
    pub top_offset: i32,
    pub close_requested: bool,
    pub submit_requested: bool,
}

impl HistoryViewElementOverlayView {
    pub fn new() -> Self {
        Self {
            is_active: false,
            item_id: 0,
            top_offset: 0,
            close_requested: false,
            submit_requested: false,
        }
    }

    pub fn show(&mut self, item_id: u64, top_offset: i32) {
        self.item_id = item_id;
        self.top_offset = top_offset;
        self.is_active = true;
        self.close_requested = false;
        self.submit_requested = false;
    }

    pub fn hide(&mut self) {
        self.is_active = false;
        self.item_id = 0;
        self.close_requested = true;
    }

    pub fn handle_click_outside(&mut self) {
        self.hide();
    }

    pub fn trigger_submit(&mut self) {
        self.submit_requested = true;
    }
}

impl Default for HistoryViewElementOverlayView {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewElementOverlayView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_active {
            return div();
        }

        div()
            .absolute()
            .top(px(self.top_offset as f32))
            .left_0()
            .right_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgb(0x000000))
    }
}
