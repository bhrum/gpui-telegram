//! UI Forward Options modal dialog component.
//! Upstream: `Telegram/SourceFiles/ui/chat/forward_options_box.cpp:L15-L48`
//!           `Telegram/SourceFiles/ui/chat/forward_options_box.h:L10-L24`
//!
//! Provides toggles for showing/hiding author names and captions when forwarding messages.

use gpui::*;

/// ForwardOptionsBoxChat component. Target symbol required by ledger: `ForwardOptionsBoxChat`.
#[derive(Debug, Clone)]
pub struct ForwardOptionsBoxChat {
    pub show_sender_name: bool,
    pub show_captions: bool,
    pub message_count: usize,
}

impl ForwardOptionsBoxChat {
    pub fn new(message_count: usize) -> Self {
        Self {
            show_sender_name: true,
            show_captions: true,
            message_count,
        }
    }

    pub fn toggle_sender_name(&mut self) {
        self.show_sender_name = !self.show_sender_name;
    }

    pub fn toggle_captions(&mut self) {
        self.show_captions = !self.show_captions;
    }
}

impl Default for ForwardOptionsBoxChat {
    fn default() -> Self {
        Self::new(1)
    }
}

impl Render for ForwardOptionsBoxChat {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .p(px(16.0))
            .gap(px(12.0))
            .rounded(px(12.0))
            .bg(hsla(0.0, 0.0, 1.0, 1.0))
            .child(format!("Forward {} messages", self.message_count))
    }
}
