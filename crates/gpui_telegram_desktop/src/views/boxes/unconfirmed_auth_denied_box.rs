//! Authentic GPUI implementation of Telegram Desktop's UnconfirmedAuthDeniedBox.
//! Upstream reference: `Telegram/SourceFiles/boxes/unconfirmed_auth_denied_box.h:L14-L21`
//! and `Telegram/SourceFiles/boxes/unconfirmed_auth_denied_box.cpp:L30-L80`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_TIMER_SECONDS: u32 = 5;

/// View component notifying the user that an authorization action was denied due to an unconfirmed new session.
#[derive(Clone)]
pub struct UnconfirmedAuthDeniedBox {
    pub device_name: String,
    pub ip_location: String,
    pub remaining_seconds: u32,
    pub is_closed: bool,
}

impl Default for UnconfirmedAuthDeniedBox {
    fn default() -> Self {
        Self {
            device_name: "Desktop App".into(),
            ip_location: "London, UK (192.168.1.1)".into(),
            remaining_seconds: K_TIMER_SECONDS,
            is_closed: false,
        }
    }
}

impl UnconfirmedAuthDeniedBox {
    pub fn new(device_name: String, ip_location: String) -> Self {
        Self {
            device_name,
            ip_location,
            remaining_seconds: K_TIMER_SECONDS,
            is_closed: false,
        }
    }

    /// Ticks the countdown timer (`unconfirmed_auth_denied_box.cpp:L30`).
    pub fn tick_timer(&mut self) {
        if self.remaining_seconds > 0 {
            self.remaining_seconds -= 1;
        }
        if self.remaining_seconds == 0 {
            self.is_closed = true;
        }
    }

    pub fn close(&mut self) {
        self.is_closed = true;
    }
}

impl Render for UnconfirmedAuthDeniedBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        let mut card = div()
            .id("unconfirmed_auth_denied_box")
            .w(px(380.0))
            .p(px(24.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(14.0))
            .shadow_lg()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(16.0));

        // Security Warning Icon
        card = card.child(
            div()
                .w(px(64.0))
                .h(px(64.0))
                .rounded_full()
                .bg(rgb(0xFEF2F2))
                .flex()
                .items_center()
                .justify_center()
                .text_size(px(32.0))
                .child("🛡️"),
        );

        // Security Alert Title
        card = card.child(
            div()
                .text_size(px(18.0))
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(0x991B1B))
                .child("Authorization Denied"),
        );

        // Alert Body
        card = card.child(
            div()
                .text_size(px(13.0))
                .line_height(relative(1.4))
                .text_color(rgb(0x4B5563))
                .text_center()
                .child("For security reasons, sensitive actions cannot be confirmed from a newly logged-in device until verified."),
        );

        // Device & Location info box
        card = card.child(
            div()
                .w_full()
                .p(px(12.0))
                .bg(rgb(0xF9FAFB))
                .rounded(px(8.0))
                .flex()
                .flex_col()
                .gap(px(4.0))
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(rgb(0x6B7280))
                        .child(format!("Device: {}", self.device_name)),
                )
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(rgb(0x6B7280))
                        .child(format!("Location: {}", self.ip_location)),
                ),
        );

        // Auto-dismiss countdown button
        card = card.child(
            div()
                .w_full()
                .py(px(10.0))
                .rounded(px(8.0))
                .bg(rgb(0xDC2626))
                .cursor_pointer()
                .flex()
                .justify_center()
                .text_size(px(14.0))
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(0xFFFFFF))
                .child(format!("Dismiss ({})", self.remaining_seconds)),
        );

        card.into_any_element()
    }
}
