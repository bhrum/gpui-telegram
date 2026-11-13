//! Channel & Bot About View Theme Style
//! Faithful extraction from `history_view_about_view.style`

use gpui::{px, rgb, Pixels, Rgba};

#[derive(Debug, Clone)]
pub struct HistoryViewAboutViewViewStyleSheet {
    pub premium_required_width: Pixels,
    pub stars_per_message_width: Pixels,
    pub chat_intro_width: Pixels,
    pub new_peer_width: Pixels,
    pub new_bot_thread_top_skip: Pixels,
    pub new_thread_about_max_width: Pixels,
    pub new_thread_about_icon_outer: Pixels,
    pub padding: Pixels,
    pub margin: Pixels,
    pub border_radius: Pixels,
    pub bg_color: Rgba,
    pub text_color: Rgba,
    pub accent_color: Rgba,
    pub border_color: Rgba,
}

impl Default for HistoryViewAboutViewViewStyleSheet {
    fn default() -> Self {
        Self {
            premium_required_width: px(186.0),
            stars_per_message_width: px(226.0),
            chat_intro_width: px(224.0),
            new_peer_width: px(320.0),
            new_bot_thread_top_skip: px(24.0),
            new_thread_about_max_width: px(180.0),
            new_thread_about_icon_outer: px(60.0),
            padding: px(16.0),
            margin: px(11.0),
            border_radius: px(16.0),
            bg_color: rgb(0xFFFFFF),
            text_color: rgb(0x222222),
            accent_color: rgb(0x2AABEE),
            border_color: rgb(0xE0E0E0),
        }
    }
}

impl HistoryViewAboutViewViewStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }
}
