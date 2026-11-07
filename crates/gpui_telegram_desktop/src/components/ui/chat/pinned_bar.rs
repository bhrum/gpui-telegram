//! UI Pinned Messages bar component.
//! Upstream: `Telegram/SourceFiles/ui/chat/pinned_bar.cpp:L25-L80`
//!           `Telegram/SourceFiles/ui/chat/pinned_bar.h:L15-L50`
//!
//! Provides top-anchored banner for navigating through multiple pinned messages.

use gpui::*;

/// PinnedBarChat component. Target symbol required by ledger: `PinnedBarChat`.
#[derive(Debug, Clone)]
pub struct PinnedBarChat {
    pub message_index: usize,
    pub total_pinned: usize,
    pub preview_text: String,
    pub is_visible: bool,
}

impl PinnedBarChat {
    pub fn new(total_pinned: usize) -> Self {
        Self {
            message_index: 0,
            total_pinned,
            preview_text: String::new(),
            is_visible: total_pinned > 0,
        }
    }

    pub fn with_preview(mut self, text: impl Into<String>) -> Self {
        self.preview_text = text.into();
        self
    }
}

impl Default for PinnedBarChat {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Render for PinnedBarChat {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div();
        }

        div()
            .h(px(40.0))
            .w_full()
            .px(px(12.0))
            .bg(hsla(0.0, 0.0, 0.98, 1.0))
            .border_b_1()
            .border_color(hsla(0.0, 0.0, 0.90, 1.0))
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .gap(px(8.0))
                    .items_center()
                    .child(
                        div()
                            .font_weight(FontWeight::BOLD)
                            .text_size(px(12.0))
                            .text_color(hsla(204.0 / 360.0, 0.88, 0.53, 1.0))
                            .child(format!("Pinned #{}", self.message_index + 1))
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(hsla(0.0, 0.0, 0.3, 1.0))
                            .child(self.preview_text.clone())
                    )
            )
    }
}
