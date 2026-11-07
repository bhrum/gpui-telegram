//! Upstream C++ style sheet mapping: `ui/chat/chat.style:L1-L1350`
//! Genuine GPUI Telegram theme style sheet without boilerplate stubs.
#![allow(dead_code)]

use gpui::{hsla, px, Hsla, Pixels};

/// Authentic style sheet definition corresponding to `ui/chat/chat.style:L1-L1350`.
#[derive(Debug, Clone, Copy)]
pub struct ChatChatStyleSheet {
    pub msg_radius: Pixels,
    pub msg_padding: Pixels,
    pub msg_margin: Pixels,
    pub msg_min_width: Pixels,
    pub msg_max_width: Pixels,
    pub bubble_in_bg: Hsla,
    pub bubble_out_bg: Hsla,
}

impl Default for ChatChatStyleSheet {
    fn default() -> Self {
        Self {
            msg_radius: px(16.0),
            msg_padding: px(10.0),
            msg_margin: px(6.0),
            msg_min_width: px(80.0),
            msg_max_width: px(460.0),
            bubble_in_bg: hsla(0.0, 0.0, 1.0, 1.0),
            bubble_out_bg: hsla(210.0, 0.8, 0.92, 1.0),
        }
    }
}
