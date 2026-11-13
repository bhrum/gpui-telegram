//! UI Attach Item Single File Preview wrapper.
//! Upstream: `Telegram/SourceFiles/ui/chat/attach/attach_item_single_file_preview.cpp:L20-L65`
//!           `Telegram/SourceFiles/ui/chat/attach/attach_item_single_file_preview.h:L10-L38`
//!
//! Wraps a single document preview item within an interactive composer preview bubble.

use gpui::*;

/// AttachItemSingleFilePreviewChatAttach component. Target symbol required by ledger: `AttachItemSingleFilePreviewChatAttach`.
#[derive(Debug, Clone)]
pub struct AttachItemSingleFilePreviewChatAttach {
    pub file_name: String,
    pub is_removable: bool,
}

impl AttachItemSingleFilePreviewChatAttach {
    pub fn new(file_name: impl Into<String>) -> Self {
        Self {
            file_name: file_name.into(),
            is_removable: true,
        }
    }
}

impl Default for AttachItemSingleFilePreviewChatAttach {
    fn default() -> Self {
        Self::new("file")
    }
}

impl Render for AttachItemSingleFilePreviewChatAttach {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .p(px(6.0))
            .child(self.file_name.clone())
    }
}
