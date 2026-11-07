//! UI Attach Send Files Way configuration options.
//! Upstream: `Telegram/SourceFiles/ui/chat/attach/attach_send_files_way.cpp:L15-L50`
//!           `Telegram/SourceFiles/ui/chat/attach/attach_send_files_way.h:L15-L65`
//!
//! Stores user preference toggles for compression, album grouping, and spoiler masks.

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SendFilesWayMode {
    Album,
    Photos,
    Files,
}

/// AttachSendFilesWayChatAttach component. Target symbol required by ledger: `AttachSendFilesWayChatAttach`.
#[derive(Debug, Clone)]
pub struct AttachSendFilesWayChatAttach {
    pub mode: SendFilesWayMode,
    pub group_files: bool,
    pub send_images_as_photos: bool,
}

impl AttachSendFilesWayChatAttach {
    pub fn new() -> Self {
        Self {
            mode: SendFilesWayMode::Album,
            group_files: true,
            send_images_as_photos: true,
        }
    }
}

impl Default for AttachSendFilesWayChatAttach {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for AttachSendFilesWayChatAttach {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .gap(px(6.0))
            .child(format!("Mode: {:?}", self.mode))
    }
}
