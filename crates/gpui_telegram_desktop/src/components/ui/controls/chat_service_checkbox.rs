//! UI Chat Service Checkbox component.
//! Upstream: `Telegram/SourceFiles/ui/controls/chat_service_checkbox.cpp:L15-L55`
//!           `Telegram/SourceFiles/ui/controls/chat_service_checkbox.h:L10-L30`
//!
//! Styled toggle checkbox used in service dialogs (e.g., delete confirmation prompts).

use gpui::*;

/// ChatServiceCheckboxControls component. Target symbol required by ledger: `ChatServiceCheckboxControls`.
#[derive(Debug, Clone)]
pub struct ChatServiceCheckboxControls {
    pub is_checked: bool,
    pub label: String,
    pub size: f32,
}

impl ChatServiceCheckboxControls {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            is_checked: false,
            label: label.into(),
            size: 20.0,
        }
    }

    pub fn toggle(&mut self) {
        self.is_checked = !self.is_checked;
    }
}

impl Default for ChatServiceCheckboxControls {
    fn default() -> Self {
        Self::new("Confirm option")
    }
}

impl Render for ChatServiceCheckboxControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let border_color = if self.is_checked {
            hsla(204.0 / 360.0, 0.88, 0.53, 1.0)
        } else {
            hsla(0.0, 0.0, 0.70, 1.0)
        };
        let bg_color = if self.is_checked {
            hsla(204.0 / 360.0, 0.88, 0.53, 1.0)
        } else {
            hsla(0.0, 0.0, 1.0, 1.0)
        };

        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(
                div()
                    .w(px(self.size))
                    .h(px(self.size))
                    .rounded(px(4.0))
                    .border_2()
                    .border_color(border_color)
                    .bg(bg_color)
                    .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(if self.is_checked { "✓" } else { "" })
            )
            .child(
                div()
                    .text_size(px(13.0))
                    .child(self.label.clone())
            )
    }
}
