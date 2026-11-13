//! UI Attach Abstract Single Preview base component.
//! Upstream: `Telegram/SourceFiles/ui/chat/attach/attach_abstract_single_preview.h:L10-L24`
//!
//! Abstract foundation for single file and single media attachment preview boxes.

use gpui::*;

/// AttachAbstractSinglePreviewChatAttach component. Target symbol required by ledger: `AttachAbstractSinglePreviewChatAttach`.
#[derive(Debug, Clone)]
pub struct AttachAbstractSinglePreviewChatAttach {
    pub file_name: String,
    pub file_size_bytes: i64,
}

impl AttachAbstractSinglePreviewChatAttach {
    pub fn new(file_name: impl Into<String>, file_size_bytes: i64) -> Self {
        Self {
            file_name: file_name.into(),
            file_size_bytes,
        }
    }
}

impl Default for AttachAbstractSinglePreviewChatAttach {
    fn default() -> Self {
        Self::new("file", 0)
    }
}

impl Render for AttachAbstractSinglePreviewChatAttach {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .p(px(8.0))
            .child(self.file_name.clone())
    }
}
