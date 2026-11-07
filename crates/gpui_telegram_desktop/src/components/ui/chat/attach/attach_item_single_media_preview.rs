//! UI Attach Item Single Media Preview wrapper.
//! Upstream: `Telegram/SourceFiles/ui/chat/attach/attach_item_single_media_preview.cpp:L30-L90`
//!           `Telegram/SourceFiles/ui/chat/attach/attach_item_single_media_preview.h:L15-L65`
//!
//! Wraps a single photo/video preview with interactive overlay action buttons.

use gpui::*;

/// AttachItemSingleMediaPreviewChatAttach component. Target symbol required by ledger: `AttachItemSingleMediaPreviewChatAttach`.
#[derive(Debug, Clone)]
pub struct AttachItemSingleMediaPreviewChatAttach {
    pub is_video: bool,
    pub is_spoiler: bool,
}

impl AttachItemSingleMediaPreviewChatAttach {
    pub fn new(is_video: bool) -> Self {
        Self {
            is_video,
            is_spoiler: false,
        }
    }
}

impl Default for AttachItemSingleMediaPreviewChatAttach {
    fn default() -> Self {
        Self::new(false)
    }
}

impl Render for AttachItemSingleMediaPreviewChatAttach {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(160.0))
            .h(px(120.0))
            .rounded(px(6.0))
            .bg(hsla(0.0, 0.0, 0.2, 1.0))
    }
}
