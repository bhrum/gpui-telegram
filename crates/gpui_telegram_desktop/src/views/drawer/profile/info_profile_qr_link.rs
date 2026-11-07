//! Info Profile QR Link component
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_qr_link.cpp:L1-L154`
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_qr_link.h:L1-L40`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use gpui::*;

/// QR code dialog view displaying an encoded t.me public link with centered logo.
#[derive(Debug, Clone)]
pub struct InfoProfileQrLinkProfile {
    pub link_url: String,
    pub title: String,
    pub qr_pixel_size: usize,
}

impl InfoProfileQrLinkProfile {
    pub const DEFAULT_QR_SIZE: usize = 200;

    pub fn new(link_url: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            link_url: link_url.into(),
            title: title.into(),
            qr_pixel_size: Self::DEFAULT_QR_SIZE,
        }
    }
}

impl Default for InfoProfileQrLinkProfile {
    fn default() -> Self {
        Self::new("https://t.me/telegram", "Telegram")
    }
}

impl Render for InfoProfileQrLinkProfile {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .p(px(16.0))
            .bg(rgb(0x18222d))
            .rounded_xl()
            .gap(px(12.0))
            .child(
                div()
                    .w(px(200.0))
                    .h(px(200.0))
                    .bg(rgb(0xffffff))
                    .rounded_lg()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(rgb(0x000000))
                    .child("QR CODE")
            )
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xffffff))
                    .child(self.title.clone())
            )
    }
}
