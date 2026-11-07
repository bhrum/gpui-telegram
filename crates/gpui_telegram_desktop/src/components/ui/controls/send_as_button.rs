//! UI Send As Button author identity toggle button.
//! Upstream: `Telegram/SourceFiles/ui/controls/send_as_button.cpp:L22-L37, L39-L68`
//!           `Telegram/SourceFiles/ui/controls/send_as_button.h:L19-L38`
//!
//! Compact avatar button positioned next to the message composer for switching author persona.

use gpui::*;

/// SendAsButtonControls component. Target symbol required by ledger: `SendAsButtonControls`.
#[derive(Debug, Clone)]
pub struct SendAsButtonControls {
    pub is_active: bool,
    pub size: f32,
    pub avatar_initials: String,
}

impl SendAsButtonControls {
    pub fn new() -> Self {
        Self {
            is_active: false,
            size: 28.0,
            avatar_initials: "U".to_string(),
        }
    }

    pub fn with_initials(mut self, initials: impl Into<String>) -> Self {
        self.avatar_initials = initials.into();
        self
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }
}

impl Default for SendAsButtonControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SendAsButtonControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(self.size))
            .h(px(self.size))
            .rounded(px(self.size / 2.0))
            .bg(hsla(204.0 / 360.0, 0.70, 0.50, 1.0))
            .text_color(hsla(0.0, 0.0, 1.0, 1.0))
            .text_size(px(11.0))
            .font_weight(FontWeight::BOLD)
            .flex()
            .items_center()
            .justify_center()
            .child(self.avatar_initials.clone())
    }
}
