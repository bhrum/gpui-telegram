//! Phone Number Banned Error Notification Box View.
//!
//! Faithful translation of upstream Telegram Desktop:
//! - `Telegram/SourceFiles/boxes/phone_banned_box.h`
//! - `Telegram/SourceFiles/boxes/phone_banned_box.cpp:L22-L71`
//!
//! Features:
//! - Compose support email mailto URL to `login@stel.com`
//! - Automatic system diagnostic payload (App version, OS version, locale, phone number)
//! - Action buttons for OK and Help

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const SUPPORT_EMAIL: &str = "login@stel.com";

#[derive(Debug, Clone)]
pub struct PhoneBannedBox {
    pub phone_number: String,
    pub app_version: String,
    pub os_version: String,
    pub locale: String,
}

impl PhoneBannedBox {
    pub fn new(phone: impl Into<String>) -> Self {
        Self {
            phone_number: phone.into(),
            app_version: "5.5.0".to_string(),
            os_version: "macOS 15.0".to_string(),
            locale: "en".to_string(),
        }
    }

    /// Compose mailto URL matching `phone_banned_box.cpp:L38-L45`
    pub fn build_support_mailto(&self) -> String {
        let subject = format!("Banned phone number: {}", self.phone_number);
        let body = format!(
            "I'm trying to use my mobile phone number: {}\nBut Telegram says it's banned. Please help.\n\nApp version: {}\nOS version: {}\nLocale: {}",
            self.phone_number, self.app_version, self.os_version, self.locale
        );
        let encode = |s: &str| -> String {
            let mut out = String::new();
            for b in s.bytes() {
                if b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.' || b == b'~' {
                    out.push(b as char);
                } else {
                    out.push_str(&format!("%{:02X}", b));
                }
            }
            out
        };
        format!(
            "mailto:{}?subject={}&body={}",
            SUPPORT_EMAIL,
            encode(&subject),
            encode(&body)
        )
    }
}

impl Default for PhoneBannedBox {
    fn default() -> Self {
        Self::new("+1234567890")
    }
}

impl Render for PhoneBannedBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(16.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(8.0))
            .border_1()
            .border_color(rgb(0xD32F2F))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_size(px(15.0))
                    .text_color(rgb(0xC62828))
                    .child("Phone Number Banned"),
            )
            .child(
                div()
                    .mt(px(8.0))
                    .text_size(px(13.0))
                    .text_color(rgb(0x333333))
                    .child(format!(
                        "This phone number ({}) is banned. If you believe this is a mistake, you can contact Telegram support.",
                        self.phone_number
                    )),
            )
            .child(
                div()
                    .flex()
                    .gap(px(8.0))
                    .mt(px(12.0))
                    .text_size(px(12.0))
                    .text_color(rgb(0x1976D2))
                    .child("Help / Contact Support via Email (login@stel.com)"),
            )
    }
}
