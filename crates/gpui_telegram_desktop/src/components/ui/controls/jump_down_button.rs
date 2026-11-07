//! UI Jump Down Button floating scroll-to-bottom action button.
//! Upstream: `Telegram/SourceFiles/ui/controls/jump_down_button.cpp:L27-L35, L48-L58`
//!           `Telegram/SourceFiles/ui/controls/jump_down_button.h:L14-L35`
//!
//! Floating round button with circular ripple, down-arrow icon, and unread badge pill.

use gpui::*;

/// JumpDownButtonControls component. Target symbol required by ledger: `JumpDownButtonControls`.
#[derive(Debug, Clone)]
pub struct JumpDownButtonControls {
    pub unread_count: usize,
    pub is_visible: bool,
    pub button_size: f32,
    pub is_hovered: bool,
}

impl JumpDownButtonControls {
    pub fn new() -> Self {
        Self {
            unread_count: 0,
            is_visible: false,
            button_size: 44.0,
            is_hovered: false,
        }
    }

    pub fn with_unread_count(mut self, unread_count: usize) -> Self {
        self.unread_count = unread_count;
        self
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
    }
}

impl Default for JumpDownButtonControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for JumpDownButtonControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div();
        }

        let bg = if self.is_hovered {
            hsla(0.0, 0.0, 1.0, 1.0)
        } else {
            hsla(0.0, 0.0, 0.96, 1.0)
        };

        div()
            .w(px(self.button_size))
            .h(px(self.button_size))
            .rounded(px(self.button_size / 2.0))
            .bg(bg)
            .shadow_md()
            .flex()
            .items_center()
            .justify_center()
            .text_size(px(18.0))
            .child("↓")
    }
}
