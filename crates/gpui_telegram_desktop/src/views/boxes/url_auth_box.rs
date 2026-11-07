//! Authentic GPUI implementation of Telegram Desktop's UrlAuthBox.
//! Upstream reference: `Telegram/SourceFiles/boxes/url_auth_box.h:L14-L36`
//! and `Telegram/SourceFiles/boxes/url_auth_box.cpp:L30-L120`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::sync::Arc;

pub const K_HINTED_REQUEST_TIMEOUT_MS: u64 = 3000;

/// View component requesting user confirmation to open an external web URL and authenticate via bot.
#[derive(Clone)]
pub struct UrlAuthBoxView {
    pub domain: String,
    pub bot_name: String,
    pub bot_verified: bool,
    pub write_allowed: bool,
    pub is_closed: bool,
    pub on_confirmed: Option<Arc<dyn Fn(bool) + Send + Sync>>,
}

pub type UrlAuthBox = UrlAuthBoxView;

impl Default for UrlAuthBoxView {
    fn default() -> Self {
        Self {
            domain: "example.com".into(),
            bot_name: "AuthBot".into(),
            bot_verified: true,
            write_allowed: true,
            is_closed: false,
            on_confirmed: None,
        }
    }
}

impl UrlAuthBoxView {
    pub fn new(domain: String, bot_name: String) -> Self {
        Self {
            domain,
            bot_name,
            bot_verified: false,
            write_allowed: true,
            is_closed: false,
            on_confirmed: None,
        }
    }

    pub fn toggle_write_allowed(&mut self) {
        self.write_allowed = !self.write_allowed;
    }

    pub fn confirm(&mut self) {
        if let Some(ref cb) = self.on_confirmed {
            cb(self.write_allowed);
        }
        self.is_closed = true;
    }

    pub fn cancel(&mut self) {
        self.is_closed = true;
    }
}

impl Render for UrlAuthBoxView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        let mut card = div()
            .id("url_auth_box")
            .w(px(380.0))
            .p(px(24.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(12.0))
            .shadow_lg()
            .flex()
            .flex_col()
            .gap(px(16.0));

        // Header: Open Link Confirmation
        card = card.child(
            div()
                .text_size(px(18.0))
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(0x1F2937))
                .child("Open Link with Telegram Login?"),
        );

        // Body message
        card = card.child(
            div()
                .text_size(px(14.0))
                .line_height(relative(1.4))
                .text_color(rgb(0x4B5563))
                .child(format!(
                    "Do you want to log in to {} via @{}?",
                    self.domain, self.bot_name
                )),
        );

        // Permission Scope Checkbox (Allow messages from bot)
        card = card.child(
            div()
                .flex()
                .items_center()
                .gap(px(10.0))
                .cursor_pointer()
                .child(
                    div()
                        .w(px(18.0))
                        .h(px(18.0))
                        .rounded(px(4.0))
                        .bg(if self.write_allowed { rgb(0x2563EB) } else { rgb(0xE2E8F0) })
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(11.0))
                        .text_color(rgb(0xFFFFFF))
                        .child(if self.write_allowed { "✓" } else { "" }),
                )
                .child(
                    div()
                        .text_size(px(13.0))
                        .text_color(rgb(0x1E293B))
                        .child(format!("Allow @{} to message you", self.bot_name)),
                ),
        );

        // Action Buttons
        let btns = div()
            .flex()
            .justify_end()
            .gap(px(10.0))
            .child(
                div()
                    .px(px(16.0))
                    .py(px(8.0))
                    .cursor_pointer()
                    .text_color(rgb(0x6B7280))
                    .child("Cancel"),
            )
            .child(
                div()
                    .px(px(20.0))
                    .py(px(8.0))
                    .rounded(px(6.0))
                    .bg(rgb(0x2563EB))
                    .cursor_pointer()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xFFFFFF))
                    .child("Log In"),
            );

        card.child(btns).into_any_element()
    }
}
