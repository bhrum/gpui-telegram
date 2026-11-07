//! Sponsored Message Click Handler & Disclosure Dialog
//! Faithful upstream C++ extraction:
//! - `history_view_sponsored_click_handler.cpp:L1-L38`
//! - `history_view_sponsored_click_handler.h:L1-L20`

use gpui::*;

#[derive(Debug, Clone)]
pub struct HistoryViewSponsoredClickHandlerView {
    pub link_url: String,
    pub is_internal: bool,
    pub sponsored_title: String,
    pub is_disclosure_open: bool,
}

impl HistoryViewSponsoredClickHandlerView {
    pub fn new(link: String, is_internal: bool, title: String) -> Self {
        Self {
            link_url: link,
            is_internal,
            sponsored_title: title,
            is_disclosure_open: false,
        }
    }

    pub fn handle_click(&mut self) -> String {
        if self.is_internal {
            format!("internal_tg://{}", self.link_url)
        } else {
            self.link_url.clone()
        }
    }

    pub fn open_disclosure(&mut self) {
        self.is_disclosure_open = true;
    }
}

impl Default for HistoryViewSponsoredClickHandlerView {
    fn default() -> Self {
        Self::new("https://telegram.org".into(), false, "Sponsored Message".into())
    }
}

impl Render for HistoryViewSponsoredClickHandlerView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let title = &self.sponsored_title;

        div()
            .flex()
            .items_center()
            .justify_between()
            .px(px(8.0))
            .py(px(4.0))
            .bg(rgba(0x00000006))
            .rounded(px(6.0))
            .text_size(px(11.0))
            .text_color(rgb(0x888888))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(4.0))
                    .child("📢")
                    .child(title.clone())
            )
            .child(
                div()
                    .hover(|s| s.underline())
                    .child("What is this?")
            )
    }
}
