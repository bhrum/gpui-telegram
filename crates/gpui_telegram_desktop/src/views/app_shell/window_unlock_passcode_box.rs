//! WindowUnlockPasscodeBox: Interactive modal box for passcode entry and verification
//! Upstream: Telegram/SourceFiles/window/window_unlock_passcode_box.h, window_unlock_passcode_box.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct WindowUnlockPasscodeBox {
    pub passcode_input: String,
    pub has_error: bool,
    pub error_message: Option<String>,
    pub is_submitting: bool,
}

impl WindowUnlockPasscodeBox {
    pub fn new() -> Self {
        Self {
            passcode_input: String::new(),
            has_error: false,
            error_message: None,
            is_submitting: false,
        }
    }

    pub fn set_input(&mut self, input: impl Into<String>) {
        self.passcode_input = input.into();
        self.has_error = false;
        self.error_message = None;
    }

    pub fn clear(&mut self) {
        self.passcode_input.clear();
        self.has_error = false;
        self.error_message = None;
    }

    pub fn set_error(&mut self, err: impl Into<String>) {
        self.has_error = true;
        self.error_message = Some(err.into());
        self.is_submitting = false;
    }

    pub fn submit(&mut self) -> String {
        self.is_submitting = true;
        self.passcode_input.clone()
    }
}

impl Default for WindowUnlockPasscodeBox {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowUnlockPasscodeBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(12.0))
            .p(px(20.0))
            .bg(rgb(0x17212B))
            .rounded(px(8.0))
            .child(div().font_weight(FontWeight::BOLD).text_color(rgb(0xFFFFFF)).child("Enter Passcode"))
            .child(
                div()
                    .h(px(36.0))
                    .px(px(8.0))
                    .bg(rgb(0x242F3D))
                    .rounded(px(4.0))
                    .flex()
                    .items_center()
                    .text_color(rgb(0xFFFFFF))
                    .child(if self.passcode_input.is_empty() {
                        "● ● ● ●"
                    } else {
                        "●".repeat(self.passcode_input.len()).leak()
                    })
            )
            .children(self.error_message.as_ref().map(|msg| {
                div().text_xs().text_color(rgb(0xE53935)).child(msg.clone())
            }))
    }
}
