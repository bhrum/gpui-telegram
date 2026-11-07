//! Info Profile Emoji Status Panel component
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_emoji_status_panel.cpp:L1-L360`
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_emoji_status_panel.h:L1-L101`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmojiStatusDuration {
    Permanent,
    OneHour,
    OneDay,
    TwoDays,
    CustomSeconds(u64),
}

/// Panel allowing users to select and set a custom emoji status with expiry duration presets.
#[derive(Debug, Clone)]
pub struct InfoProfileEmojiStatusPanelProfile {
    pub selected_document_id: Option<u64>,
    pub selected_duration: EmojiStatusDuration,
    pub is_open: bool,
}

impl InfoProfileEmojiStatusPanelProfile {
    pub const LIMIT_FIRST_ROW: usize = 8;

    pub fn new() -> Self {
        Self {
            selected_document_id: None,
            selected_duration: EmojiStatusDuration::OneDay,
            is_open: false,
        }
    }

    pub fn open_panel(&mut self) {
        self.is_open = true;
    }

    pub fn close_panel(&mut self) {
        self.is_open = false;
    }

    pub fn select_emoji(&mut self, document_id: u64, duration: EmojiStatusDuration) {
        self.selected_document_id = Some(document_id);
        self.selected_duration = duration;
    }
}

impl Default for InfoProfileEmojiStatusPanelProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoProfileEmojiStatusPanelProfile {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .p(px(8.0))
            .bg(rgb(0x18222d))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_sm()
                    .text_color(rgb(0xffffff))
                    .child("Set Emoji Status")
            )
    }
}
