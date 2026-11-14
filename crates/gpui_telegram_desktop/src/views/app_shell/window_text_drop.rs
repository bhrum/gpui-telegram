//! WindowTextDrop: Drag-and-drop file and text reception coordinator for window regions
//! Upstream: Telegram/SourceFiles/window/window_text_drop.h, window_text_drop.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct WindowTextDrop {
    pub is_drag_active: bool,
    pub dropped_text: Option<String>,
    pub dropped_files: Vec<String>,
}

impl WindowTextDrop {
    pub fn new() -> Self {
        Self {
            is_drag_active: false,
            dropped_text: None,
            dropped_files: Vec::new(),
        }
    }

    pub fn on_drag_enter(&mut self) {
        self.is_drag_active = true;
    }

    pub fn on_drag_leave(&mut self) {
        self.is_drag_active = false;
    }

    pub fn on_drop_text(&mut self, text: impl Into<String>) {
        self.dropped_text = Some(text.into());
        self.is_drag_active = false;
    }

    pub fn on_drop_files(&mut self, files: Vec<String>) {
        self.dropped_files = files;
        self.is_drag_active = false;
    }
}

impl Default for WindowTextDrop {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowTextDrop {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_drag_active {
            return div().into_any_element();
        }
        div()
            .absolute()
            .inset(px(12.0))
            .rounded(px(12.0))
            .border_2()
            .border_color(rgb(0x5288C1))
            .bg(rgba(0x17212BE0))
            .flex()
            .items_center()
            .justify_center()
            .text_sm()
            .text_color(rgb(0xFFFFFF))
            .child("Drop files or text here to attach")
            .into_any_element()
    }
}
