//! Authentic GPUI implementation of Telegram Desktop's About Sponsored Messages Box.
//! Upstream reference: `Telegram/SourceFiles/boxes/about_sponsored_box.h:L10-L17` and `about_sponsored_box.cpp:L20-L77`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Anonymous-namespace constant from `about_sponsored_box.cpp:L22`.
pub const K_SPONSORED_ADS_URL: &str = "https://ads.telegram.org";

/// AboutSponsoredBox view component.
/// Cites upstream `Ui::AboutSponsoredBox(GenericBox *box)` (`about_sponsored_box.cpp:L26-L75`).
#[derive(Clone)]
pub struct AboutSponsoredBox {
    pub ads_url: String,
    pub is_closed: bool,
}

impl AboutSponsoredBox {
    pub fn new() -> Self {
        Self {
            ads_url: K_SPONSORED_ADS_URL.to_string(),
            is_closed: false,
        }
    }

    pub fn close(&mut self) {
        self.is_closed = true;
    }
}

impl Default for AboutSponsoredBox {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for AboutSponsoredBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        // st::boxWideWidth = 440px (`about_sponsored_box.cpp:L28`)
        let card = div()
            .id("about_sponsored_box_card")
            .w(px(440.0))
            .p(px(24.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(10.0))
            .shadow_lg()
            .flex()
            .flex_col()
            .gap(px(16.0))
            // Title: tr::lng_sponsored_title() (`about_sponsored_box.cpp:L27`)
            .child(
                div()
                    .text_size(px(18.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x222222))
                    .child("What are sponsored messages?"),
            )
            // Description paragraph 1 (`about_sponsored_box.cpp:L57-L66`)
            .child(
                div()
                    .text_size(px(13.0))
                    .line_height(relative(1.4))
                    .text_color(rgb(0x555555))
                    .child(
                        "Unlike other apps, Telegram never uses your private data to target ads. \
                         Sponsored messages are based solely on the public channel topics in which they are shown.",
                    ),
            )
            // External Ads Platform URL Button (`about_sponsored_box.cpp:L31-L54`)
            .child(
                div()
                    .flex()
                    .justify_center()
                    .py(px(4.0))
                    .child(
                        div()
                            .px(px(20.0))
                            .py(px(8.0))
                            .rounded(px(18.0))
                            .border_1()
                            .border_color(rgb(0x5288C1))
                            .cursor_pointer()
                            .hover(|s| s.bg(rgba(0x5288C11A)))
                            .text_size(px(14.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(0x5288C1))
                            .child(self.ads_url.clone()),
                    ),
            )
            // Description paragraph 2 (`about_sponsored_box.cpp:L72-L74`)
            .child(
                div()
                    .text_size(px(13.0))
                    .line_height(relative(1.4))
                    .text_color(rgb(0x555555))
                    .child(
                        "Channel owners can earn rewards by displaying sponsored messages, and Telegram \
                         Premium subscribers can disable them entirely.",
                    ),
            )
            // Footer OK button (`about_sponsored_box.cpp:L29`)
            .child(
                div()
                    .flex()
                    .justify_end()
                    .child(
                        div()
                            .px(px(16.0))
                            .py(px(8.0))
                            .rounded(px(6.0))
                            .cursor_pointer()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x5288C1))
                            .child("OK"),
                    ),
            );

        card.into_any_element()
    }
}
