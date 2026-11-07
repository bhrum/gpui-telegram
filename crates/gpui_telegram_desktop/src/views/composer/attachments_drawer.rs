//! Attachments Drawer / Popover Menu (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.4).
//! Upstream reference: `tdesktop/Telegram/SourceFiles/chat_helpers/message_field.cpp`.

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttachmentKind {
    PhotoOrVideo,
    DocumentOrFile,
    AudioOrMusic,
    CreatePoll,
    ShareContact,
    SendLocation,
}

pub struct AttachmentsDrawer {
    pub is_open: bool,
}

impl AttachmentsDrawer {
    pub fn new() -> Self {
        Self { is_open: false }
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }

    pub fn render(&self) -> impl IntoElement {
        let items = [
            (AttachmentKind::PhotoOrVideo, "🖼️", "Photo or Video", 0x3498DB),
            (AttachmentKind::DocumentOrFile, "📄", "File / Document", 0x2ECC71),
            (AttachmentKind::AudioOrMusic, "🎵", "Audio Track", 0x9B59B6),
            (AttachmentKind::CreatePoll, "📊", "Create Poll", 0xF39C12),
            (AttachmentKind::ShareContact, "👤", "Share Contact", 0x1ABC9C),
            (AttachmentKind::SendLocation, "📍", "Send Location", 0xE74C3C),
        ];

        div()
            .w(px(200.0))
            .rounded(px(10.0))
            .bg(rgb(0x1E2B38))
            .border_1()
            .border_color(rgb(0x2B5278))
            .p(px(6.0))
            .flex()
            .flex_col()
            .gap(px(2.0))
            .children(items.iter().map(|&(_kind, icon, label, _accent)| {
                div()
                    .h(px(32.0))
                    .px(px(8.0))
                    .rounded(px(6.0))
                    .flex()
                    .items_center()
                    .gap(px(10.0))
                    .child(div().text_size(px(16.0)).child(icon))
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(rgb(0xFFFFFF))
                            .child(label),
                    )
            }))
    }
}

impl Default for AttachmentsDrawer {
    fn default() -> Self {
        Self::new()
    }
}
