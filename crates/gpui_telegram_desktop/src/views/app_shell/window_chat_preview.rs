//! WindowChatPreview: Hover / peek preview popup showing recent messages without marking read
//! Upstream: Telegram/SourceFiles/window/window_chat_preview.h, window_chat_preview.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct WindowChatPreview {
    pub is_active: bool,
    pub peer_id: Option<u64>,
    pub preview_title: String,
    pub recent_snippets: Vec<String>,
}

impl WindowChatPreview {
    pub fn new() -> Self {
        Self {
            is_active: false,
            peer_id: None,
            preview_title: String::new(),
            recent_snippets: Vec::new(),
        }
    }

    pub fn show_preview(&mut self, peer_id: u64, title: impl Into<String>, snippets: Vec<String>) {
        self.peer_id = Some(peer_id);
        self.preview_title = title.into();
        self.recent_snippets = snippets;
        self.is_active = true;
    }

    pub fn dismiss(&mut self) {
        self.is_active = false;
        self.peer_id = None;
        self.recent_snippets.clear();
    }
}

impl Default for WindowChatPreview {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowChatPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .p(px(16.0))
            .bg(rgb(0x17212B))
            .rounded(px(8.0))
            .shadow_lg()
            .child(div().font_weight(FontWeight::BOLD).text_color(rgb(0xFFFFFF)).child(self.preview_title.clone()))
    }
}
