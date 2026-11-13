//! UI Sponsored Message Bar promotional banner.
//! Upstream: `Telegram/SourceFiles/ui/chat/sponsored_message_bar.cpp:L35-L95`
//!           `Telegram/SourceFiles/ui/chat/sponsored_message_bar.h:L10-L24`
//!
//! Renders sponsored ad banner anchored to the bottom of public channels with disclosure tags.

use gpui::*;

/// SponsoredMessageBarChat component. Target symbol required by ledger: `SponsoredMessageBarChat`.
#[derive(Debug, Clone)]
pub struct SponsoredMessageBarChat {
    pub title: String,
    pub text: String,
    pub sponsor_label: String,
    pub is_visible: bool,
}

impl SponsoredMessageBarChat {
    pub fn new(title: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            text: text.into(),
            sponsor_label: "Sponsored".to_string(),
            is_visible: true,
        }
    }
}

impl Default for SponsoredMessageBarChat {
    fn default() -> Self {
        Self::new("Promoted", "Message details")
    }
}

impl Render for SponsoredMessageBarChat {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div();
        }

        div()
            .w_full()
            .p(px(10.0))
            .bg(hsla(210.0 / 360.0, 0.40, 0.95, 1.0))
            .rounded(px(8.0))
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .child(
                        div()
                            .font_weight(FontWeight::BOLD)
                            .text_size(px(13.0))
                            .child(self.title.clone())
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                            .child(self.sponsor_label.clone())
                    )
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(hsla(0.0, 0.0, 0.25, 1.0))
                    .child(self.text.clone())
            )
    }
}
