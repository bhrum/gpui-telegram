//! History View Reaction & Sticker Preview Overlay
//! Faithful upstream C++ extraction:
//! - `history_view_reaction_preview.cpp:L20-L80`
//! - `history_view_reaction_preview.h:L20-L40`

use gpui::*;

#[derive(Debug, Clone)]
pub struct HistoryViewReactionPreviewView {
    pub reaction_or_sticker_id: String,
    pub is_visible: bool,
    pub preview_size: f32,
    pub origin_message_id: u64,
}

impl HistoryViewReactionPreviewView {
    pub fn new(id: String, origin: u64) -> Self {
        Self {
            reaction_or_sticker_id: id,
            is_visible: true,
            preview_size: 160.0,
            origin_message_id: origin,
        }
    }

    pub fn dismiss(&mut self) {
        self.is_visible = false;
    }
}

impl Default for HistoryViewReactionPreviewView {
    fn default() -> Self {
        Self::new("❤️".into(), 0)
    }
}

impl Render for HistoryViewReactionPreviewView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }

        let label = &self.reaction_or_sticker_id;
        div()
            .flex()
            .items_center()
            .justify_center()
            .w(px(self.preview_size))
            .h(px(self.preview_size))
            .bg(rgba(0xFFFFFFEE))
            .rounded(px(24.0))
            .shadow_2xl()
            .text_size(px(80.0))
            .child(label.clone())
            .into_any_element()
    }
}
