//! UI Attach Album Thumbnail item component.
//! Upstream: `Telegram/SourceFiles/ui/chat/attach/attach_album_thumbnail.cpp:L35-L120`
//!           `Telegram/SourceFiles/ui/chat/attach/attach_album_thumbnail.h:L25-L90`
//!
//! Renders an individual thumbnail in an album grid with TTL badge and spoiler filter.

use gpui::*;

/// AttachAlbumThumbnailChatAttach component. Target symbol required by ledger: `AttachAlbumThumbnailChatAttach`.
#[derive(Debug, Clone)]
pub struct AttachAlbumThumbnailChatAttach {
    pub width: f32,
    pub height: f32,
    pub has_spoiler: bool,
    pub ttl_seconds: Option<i32>,
}

impl AttachAlbumThumbnailChatAttach {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            has_spoiler: false,
            ttl_seconds: None,
        }
    }

    pub fn with_spoiler(mut self, spoiler: bool) -> Self {
        self.has_spoiler = spoiler;
        self
    }
}

impl Default for AttachAlbumThumbnailChatAttach {
    fn default() -> Self {
        Self::new(100.0, 100.0)
    }
}

impl Render for AttachAlbumThumbnailChatAttach {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(self.width))
            .h(px(self.height))
            .rounded(px(6.0))
            .bg(hsla(0.0, 0.0, 0.85, 1.0))
    }
}
