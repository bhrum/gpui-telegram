//! Authentic GPUI implementation of Telegram Desktop's UsernameBox.
//! Upstream reference: `Telegram/SourceFiles/boxes/username_box.h:L14-L37`
//! and `Telegram/SourceFiles/boxes/username_box.cpp:L40-L120`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_MIN_USERNAME_LENGTH: usize = 5;
pub const K_MAX_USERNAME_LENGTH: usize = 32;

/// Username validation status per `username_box.cpp:L40-L100`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsernameCheckStatus {
    Empty,
    TooShort,
    TooLong,
    InvalidCharacters,
    StartsWithDigit,
    Available,
    Taken,
    PurchaseAvailableOnFragment,
}

/// View component for editing public @username handle with validation and Fragment auction link.
#[derive(Clone)]
pub struct UsernameBox {
    pub username_input: String,
    pub status: UsernameCheckStatus,
    pub is_closed: bool,
}

impl Default for UsernameBox {
    fn default() -> Self {
        Self {
            username_input: "cool_developer".into(),
            status: UsernameCheckStatus::Available,
            is_closed: false,
        }
    }
}

impl UsernameBox {
    pub fn new(current: String) -> Self {
        let mut b = Self {
            username_input: current,
            status: UsernameCheckStatus::Empty,
            is_closed: false,
        };
        b.validate_username();
        b
    }

    /// Performs validation mirroring `username_box.cpp:L40-L120`.
    pub fn validate_username(&mut self) {
        let u = self.username_input.trim();
        if u.is_empty() {
            self.status = UsernameCheckStatus::Empty;
            return;
        }
        if u.len() < K_MIN_USERNAME_LENGTH {
            self.status = UsernameCheckStatus::TooShort;
            return;
        }
        if u.len() > K_MAX_USERNAME_LENGTH {
            self.status = UsernameCheckStatus::TooLong;
            return;
        }
        if let Some(first) = u.chars().next() {
            if first.is_ascii_digit() {
                self.status = UsernameCheckStatus::StartsWithDigit;
                return;
            }
        }
        for ch in u.chars() {
            if !ch.is_ascii_alphanumeric() && ch != '_' {
                self.status = UsernameCheckStatus::InvalidCharacters;
                return;
            }
        }
        self.status = UsernameCheckStatus::Available;
    }

    pub fn set_input(&mut self, text: String) {
        self.username_input = text;
        self.validate_username();
    }

    pub fn close(&mut self) {
        self.is_closed = true;
    }
}

impl Render for UsernameBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        let mut card = div()
            .id("username_box")
            .w(px(380.0))
            .p(px(24.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(12.0))
            .shadow_lg()
            .flex()
            .flex_col()
            .gap(px(14.0));

        // Header Title
        card = card.child(
            div()
                .text_size(px(18.0))
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(0x1F2937))
                .child("Set Username"),
        );

        // Input Field Preview
        card = card.child(
            div()
                .flex()
                .items_center()
                .p(px(10.0))
                .rounded(px(8.0))
                .bg(rgb(0xF8FAFC))
                .border_1()
                .border_color(rgb(0xE2E8F0))
                .child(
                    div()
                        .text_color(rgb(0x94A3B8))
                        .font_weight(FontWeight::BOLD)
                        .child("@"),
                )
                .child(
                    div()
                        .ml(px(4.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(0x0F172A))
                        .child(self.username_input.clone()),
                ),
        );

        // Status Indicator
        let (status_text, status_col) = match self.status {
            UsernameCheckStatus::Available => ("Username is available.", rgb(0x16A34A)),
            UsernameCheckStatus::TooShort => ("Minimum 5 characters required.", rgb(0xDC2626)),
            UsernameCheckStatus::TooLong => ("Maximum 32 characters allowed.", rgb(0xDC2626)),
            UsernameCheckStatus::StartsWithDigit => ("Username cannot start with a number.", rgb(0xDC2626)),
            UsernameCheckStatus::InvalidCharacters => ("Only a-z, 0-9, and underscores allowed.", rgb(0xDC2626)),
            UsernameCheckStatus::Taken => ("This username is already taken.", rgb(0xDC2626)),
            UsernameCheckStatus::PurchaseAvailableOnFragment => ("Available on Fragment auction.", rgb(0x2563EB)),
            UsernameCheckStatus::Empty => ("You can choose a username on Telegram.", rgb(0x64748B)),
        };

        card = card.child(
            div()
                .text_size(px(13.0))
                .text_color(status_col)
                .child(status_text),
        );

        // Helper explanation & link
        card = card.child(
            div()
                .text_size(px(12.0))
                .line_height(relative(1.4))
                .text_color(rgb(0x64748B))
                .child(format!(
                    "People will be able to find you by this username and contact you without needing your phone number: https://t.me/{}",
                    self.username_input
                )),
        );

        // Save Button
        card = card.child(
            div()
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
                        .child("Save"),
                ),
        );

        card.into_any_element()
    }
}
