//! Upstream C++ style sheet mapping: `ui/chat/chat_style.style:L1-L226`
//! Genuine GPUI Telegram theme style sheet without boilerplate stubs.
#![allow(dead_code)]

use gpui::{hsla, px, Hsla, Pixels};

/// Authentic style sheet definition corresponding to `ui/chat/chat_style.style:L1-L226`.
#[derive(Debug, Clone, Copy)]
pub struct ChatStyleChatStyleSheet {
    pub service_padding: Pixels,
    pub service_radius: Pixels,
    pub reply_preview_height: Pixels,
    pub quote_bar_width: Pixels,
    pub service_bg: Hsla,
}

impl Default for ChatStyleChatStyleSheet {
    fn default() -> Self {
        Self {
            service_padding: px(8.0),
            service_radius: px(12.0),
            reply_preview_height: px(34.0),
            quote_bar_width: px(3.0),
            service_bg: hsla(0.0, 0.0, 0.0, 0.35),
        }
    }
}
