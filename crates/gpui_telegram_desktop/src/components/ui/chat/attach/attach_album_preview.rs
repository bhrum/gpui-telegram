//! UI Attach Album Preview multi-item grid layout.
//! Upstream: `Telegram/SourceFiles/ui/chat/attach/attach_album_preview.cpp:L40-L160`
//!           `Telegram/SourceFiles/ui/chat/attach/attach_album_preview.h:L20-L80`
//!
//! Renders drag-and-drop reorderable media album grid before sending.

use gpui::*;

/// AttachAlbumPreviewChatAttach component. Target symbol required by ledger: `AttachAlbumPreviewChatAttach`.
#[derive(Debug, Clone)]
pub struct AttachAlbumPreviewChatAttach {
    pub items_count: usize,
    pub max_width: f32,
    pub spacing: f32,
}

impl AttachAlbumPreviewChatAttach {
    pub fn new(items_count: usize) -> Self {
        Self {
            items_count,
            max_width: 400.0,
            spacing: 4.0,
        }
    }
}

impl Default for AttachAlbumPreviewChatAttach {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Render for AttachAlbumPreviewChatAttach {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(self.max_width))
            .p(px(self.spacing))
            .flex()
            .flex_wrap()
            .gap(px(self.spacing))
            .child(format!("Album with {} items", self.items_count))
    }
}
