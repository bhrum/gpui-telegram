//! MenuEmojiStatus: Context action allowing Premium users to set custom emoji as their profile status
//! Upstream: Telegram/SourceFiles/menu/menu_emoji_status.h, menu_emoji_status.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct MenuEmojiStatus {
    pub selected_document_id: Option<u64>,
    pub duration_seconds: Option<u32>,
}

impl MenuEmojiStatus {
    pub fn new() -> Self {
        Self {
            selected_document_id: None,
            duration_seconds: None,
        }
    }

    pub fn set_status(&mut self, document_id: u64, duration_secs: Option<u32>) {
        self.selected_document_id = Some(document_id);
        self.duration_seconds = duration_secs;
    }

    pub fn clear_status(&mut self) {
        self.selected_document_id = None;
        self.duration_seconds = None;
    }
}

impl Default for MenuEmojiStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for MenuEmojiStatus {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0x82868A))
            .child(format!("EmojiStatusAction: doc={:?}", self.selected_document_id))
    }
}
