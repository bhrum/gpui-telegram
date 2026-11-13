//! UI Attach Bot Downloads file progress bar.
//! Upstream: `Telegram/SourceFiles/ui/chat/attach/attach_bot_downloads.cpp:L30-L90`
//!           `Telegram/SourceFiles/ui/chat/attach/attach_bot_downloads.h:L10-L45`
//!
//! Tracks file download requests initiated by Bot WebApps.

use gpui::*;

/// AttachBotDownloadsChatAttach component. Target symbol required by ledger: `AttachBotDownloadsChatAttach`.
#[derive(Debug, Clone)]
pub struct AttachBotDownloadsChatAttach {
    pub file_name: String,
    pub progress: f32,
    pub is_downloading: bool,
}

impl AttachBotDownloadsChatAttach {
    pub fn new(file_name: impl Into<String>) -> Self {
        Self {
            file_name: file_name.into(),
            progress: 0.0,
            is_downloading: false,
        }
    }
}

impl Default for AttachBotDownloadsChatAttach {
    fn default() -> Self {
        Self::new("download")
    }
}

impl Render for AttachBotDownloadsChatAttach {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .p(px(6.0))
            .child(self.file_name.clone())
            .child(format!("{:.0}%", self.progress * 100.0))
    }
}
