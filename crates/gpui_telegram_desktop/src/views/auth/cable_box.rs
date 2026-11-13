//! Upstream Source: Telegram/SourceFiles/webauthn/cable_box.cpp, cable_box.h
//! Subsystem: webauthn | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CableSheet {
    #[default]
    Qr,
    Connecting,
    Continue,
    Error,
}

#[derive(Debug, Clone, Default)]
pub struct CableBoxState {
    pub sheet: CableSheet,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CableBox {
    pub state: CableBoxState,
    pub qr_text: String,
    pub is_register: bool,
    pub bluetooth_available: bool,
    pub is_closed: bool,
}

impl Default for CableBox {
    fn default() -> Self {
        Self::new("FIDO:/1234567890", true, true)
    }
}

impl CableBox {
    pub fn new(qr_text: impl Into<String>, is_register: bool, bluetooth_available: bool) -> Self {
        Self {
            state: CableBoxState::default(),
            qr_text: qr_text.into(),
            is_register,
            bluetooth_available,
            is_closed: false,
        }
    }

    pub fn set_sheet(&mut self, sheet: CableSheet) {
        self.state.sheet = sheet;
    }

    pub fn set_error(&mut self, err: impl Into<String>) {
        self.state.error_message = Some(err.into());
        self.state.sheet = CableSheet::Error;
    }

    pub fn close(&mut self) {
        self.is_closed = true;
    }
}

impl Render for CableBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let title = if self.is_register {
            "Create a Passkey"
        } else {
            "Sign In with a Passkey"
        };

        let content = match self.state.sheet {
            CableSheet::Qr => div()
                .flex()
                .flex_col()
                .items_center()
                .child(
                    div()
                        .size(px(160.0))
                        .p(px(8.0))
                        .bg(rgb(0xffffff))
                        .rounded(px(8.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .size(px(144.0))
                                .bg(rgb(0x17212b))
                                .rounded(px(4.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_xl()
                                .child("🔑"),
                        ),
                )
                .child(
                    div()
                        .mt(px(12.0))
                        .text_xs()
                        .text_color(rgb(0x8292a1))
                        .child("Scan with your phone's camera to authenticate"),
                ),
            CableSheet::Connecting => div()
                .py(px(24.0))
                .text_sm()
                .text_color(rgb(0x2aabee))
                .child("Connecting to phone..."),
            CableSheet::Continue => div()
                .py(px(24.0))
                .text_sm()
                .text_color(rgb(0x4fae4e))
                .child("Please confirm biometric on your phone"),
            CableSheet::Error => div()
                .py(px(24.0))
                .text_sm()
                .text_color(rgb(0xee5655))
                .child(self.state.error_message.clone().unwrap_or_else(|| "Authentication failed".to_string())),
        };

        div()
            .flex()
            .flex_col()
            .items_center()
            .w(px(340.0))
            .p(px(20.0))
            .rounded(px(10.0))
            .bg(rgb(0x17212b))
            .border_1()
            .border_color(rgb(0x242f3d))
            .shadow_xl()
            .child(
                div()
                    .text_base()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xffffff))
                    .child(title),
            )
            .child(
                div()
                    .my(px(16.0))
                    .w_full()
                    .flex()
                    .justify_center()
                    .child(content),
            )
            .child(
                // Use Security Key button
                div()
                    .w_full()
                    .h(px(36.0))
                    .rounded(px(6.0))
                    .bg(rgb(0x242f3d))
                    .hover(|s| s.bg(rgb(0x2d3a4b)))
                    .cursor_pointer()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0xffffff))
                            .child("USE SECURITY KEY INSTEAD"),
                    ),
            )
    }
}
