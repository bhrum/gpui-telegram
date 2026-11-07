//! WindowMediaPreview: Inline preview modal for stickers, animations, and photos
//! Upstream: Telegram/SourceFiles/window/window_media_preview.h, window_media_preview.cpp

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PreviewMediaType {
    Photo,
    Sticker,
    Gif,
    Video,
}

#[derive(Debug, Clone)]
pub struct WindowMediaPreview {
    pub is_visible: bool,
    pub media_type: PreviewMediaType,
    pub source_id: u64,
    pub width: u32,
    pub height: u32,
}

impl WindowMediaPreview {
    pub fn new() -> Self {
        Self {
            is_visible: false,
            media_type: PreviewMediaType::Photo,
            source_id: 0,
            width: 320,
            height: 240,
        }
    }

    pub fn show(&mut self, source_id: u64, media_type: PreviewMediaType, w: u32, h: u32) {
        self.source_id = source_id;
        self.media_type = media_type;
        self.width = w;
        self.height = h;
        self.is_visible = true;
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
    }
}

impl Default for WindowMediaPreview {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowMediaPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_center()
            .p(px(8.0))
            .bg(rgba(0x00000080))
            .child(format!("MediaPreview: {:?} (ID: {})", self.media_type, self.source_id))
    }
}
