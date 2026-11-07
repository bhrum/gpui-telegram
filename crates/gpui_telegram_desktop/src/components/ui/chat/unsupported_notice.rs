//! UI Unsupported Message Notice fallback component.
//! Upstream: `Telegram/SourceFiles/ui/chat/unsupported_notice.cpp:L20-L60`
//!           `Telegram/SourceFiles/ui/chat/unsupported_notice.h:L10-L28`
//!
//! Renders notice for messages containing features not yet supported by current app version.

use gpui::*;

/// UnsupportedNoticeChat component. Target symbol required by ledger: `UnsupportedNoticeChat`.
#[derive(Debug, Clone)]
pub struct UnsupportedNoticeChat {
    pub text: String,
    pub update_link_text: String,
}

impl UnsupportedNoticeChat {
    pub fn new() -> Self {
        Self {
            text: "This message contains content not supported by this version of Telegram.".to_string(),
            update_link_text: "Please update the app to view it.".to_string(),
        }
    }
}

impl Default for UnsupportedNoticeChat {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for UnsupportedNoticeChat {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .p(px(10.0))
            .rounded(px(8.0))
            .bg(hsla(0.0, 0.0, 0.95, 1.0))
            .text_size(px(12.0))
            .text_color(hsla(0.0, 0.0, 0.4, 1.0))
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(self.text.clone())
            .child(
                div()
                    .text_color(hsla(204.0 / 360.0, 0.88, 0.53, 1.0))
                    .font_weight(FontWeight::MEDIUM)
                    .child(self.update_link_text.clone())
            )
    }
}
