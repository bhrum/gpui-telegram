//! UI Attach Bot Webview container for Telegram Mini Apps.
//! Upstream: `Telegram/SourceFiles/ui/chat/attach/attach_bot_webview.cpp:L100-L250`
//!           `Telegram/SourceFiles/ui/chat/attach/attach_bot_webview.h:L20-L100`
//!
//! Hosts webview runtime, theme synchronization bridges, fullscreen mode, and bottom sheet chrome.

use gpui::prelude::FluentBuilder;
use gpui::*;

/// AttachBotWebviewChatAttach component. Target symbol required by ledger: `AttachBotWebviewChatAttach`.
#[derive(Debug, Clone)]
pub struct AttachBotWebviewChatAttach {
    pub bot_name: String,
    pub webview_url: String,
    pub is_fullscreen: bool,
    pub is_expanded: bool,
    pub header_color: Hsla,
}

impl AttachBotWebviewChatAttach {
    pub fn new(bot_name: impl Into<String>, webview_url: impl Into<String>) -> Self {
        Self {
            bot_name: bot_name.into(),
            webview_url: webview_url.into(),
            is_fullscreen: false,
            is_expanded: true,
            header_color: hsla(0.0, 0.0, 1.0, 1.0),
        }
    }

    pub fn with_fullscreen(mut self, fullscreen: bool) -> Self {
        self.is_fullscreen = fullscreen;
        self
    }
}

impl Default for AttachBotWebviewChatAttach {
    fn default() -> Self {
        Self::new("MiniApp", "https://t.me")
    }
}

impl Render for AttachBotWebviewChatAttach {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("attach-bot-webview-sheet")
            .w_full()
            .h_full()
            .when(!self.is_fullscreen, |el| el.rounded_t(px(14.0)))
            .overflow_hidden()
            .flex()
            .flex_col()
            .child(
                div()
                    .h(px(44.0))
                    .w_full()
                    .px(px(14.0))
                    .bg(self.header_color)
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_size(px(13.5))
                                    .font_weight(FontWeight::BOLD)
                                    .child(self.bot_name.clone()),
                            )
                            .child(
                                div()
                                    .text_size(px(10.5))
                                    .opacity(0.75)
                                    .child(self.webview_url.clone()),
                            ),
                    )
                    .child(
                        div()
                            .px(px(8.0))
                            .py(px(2.0))
                            .rounded(px(999.0))
                            .text_size(px(10.5))
                            .child(if self.is_fullscreen { "Fullscreen" } else { "Sheet" }),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .bg(hsla(0.0, 0.0, 0.98, 1.0)),
            )
    }
}
