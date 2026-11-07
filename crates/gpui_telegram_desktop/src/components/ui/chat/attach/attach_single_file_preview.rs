//! UI Attach Single File Preview view component.
//! Upstream: `Telegram/SourceFiles/ui/chat/attach/attach_single_file_preview.cpp:L20-L70`
//!           `Telegram/SourceFiles/ui/chat/attach/attach_single_file_preview.h:L15-L40`
//!
//! Renders a standalone document preview with file icon, filename, and file size.

use gpui::*;

/// AttachSingleFilePreviewChatAttach component. Target symbol required by ledger: `AttachSingleFilePreviewChatAttach`.
#[derive(Debug, Clone)]
pub struct AttachSingleFilePreviewChatAttach {
    pub file_name: String,
    pub formatted_size: String,
}

impl AttachSingleFilePreviewChatAttach {
    pub fn new(file_name: impl Into<String>, formatted_size: impl Into<String>) -> Self {
        Self {
            file_name: file_name.into(),
            formatted_size: formatted_size.into(),
        }
    }
}

impl Default for AttachSingleFilePreviewChatAttach {
    fn default() -> Self {
        Self::new("document.pdf", "1.2 MB")
    }
}

impl Render for AttachSingleFilePreviewChatAttach {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap(px(10.0))
            .p(px(8.0))
            .rounded(px(8.0))
            .bg(hsla(0.0, 0.0, 0.95, 1.0))
            .child("📄")
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(self.file_name.clone())
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                            .child(self.formatted_size.clone())
                    )
            )
    }
}
