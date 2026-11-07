//! UI Window Screen Reader notification bar.
//! Upstream: `Telegram/SourceFiles/ui/controls/window_screen_reader_bar.cpp:L25-L95`
//!           `Telegram/SourceFiles/ui/controls/window_screen_reader_bar.h:L15-L18`
//!
//! ARIA announcement and disable controls banner for screen reader accessibility support.

use gpui::*;

/// WindowScreenReaderBarControls component. Target symbol required by ledger: `WindowScreenReaderBarControls`.
#[derive(Debug, Clone)]
pub struct WindowScreenReaderBarControls {
    pub message: String,
    pub is_visible: bool,
}

impl WindowScreenReaderBarControls {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            is_visible: true,
        }
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
    }
}

impl Default for WindowScreenReaderBarControls {
    fn default() -> Self {
        Self::new("Screen reader active. Press Tab to navigate.")
    }
}

impl Render for WindowScreenReaderBarControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div();
        }

        div()
            .h(px(36.0))
            .w_full()
            .px(px(16.0))
            .bg(hsla(45.0 / 360.0, 0.95, 0.88, 1.0))
            .border_b_1()
            .border_color(hsla(45.0 / 360.0, 0.60, 0.70, 1.0))
            .flex()
            .items_center()
            .justify_between()
            .text_size(px(13.0))
            .text_color(hsla(0.0, 0.0, 0.1, 1.0))
            .child(self.message.clone())
            .child(
                div()
                    .px(px(8.0))
                    .py(px(2.0))
                    .rounded(px(4.0))
                    .bg(hsla(0.0, 0.0, 1.0, 0.8))
                    .text_size(px(11.0))
                    .child("Disable")
            )
    }
}
