//! Upstream: Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder_menu_item.h
//! Upstream: Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder_menu_item.cpp
//! Subsystem: info | Feature ID: TCG-N34 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_STICKER_CYCLE_TIMEOUT_MS: u64 = 1500;
pub const K_HIGHLIGHT_CONTROL_ID: &str = "profile-photo/use-emoji";

#[derive(Debug, Clone)]
pub struct InfoUserpicEmojiBuilderMenuItemUserpic {
    pub title: String,
    pub documents: Vec<u64>,
    pub current_index: usize,
    pub is_forum: bool,
    pub icon_size: f32,
    pub slot_size: f32,
}

impl InfoUserpicEmojiBuilderMenuItemUserpic {
    pub fn new(documents: Vec<u64>, is_forum: bool) -> Self {
        Self {
            title: "Set Emoji or Sticker".into(),
            documents,
            current_index: 0,
            is_forum,
            icon_size: 20.0,
            slot_size: 24.0,
        }
    }

    pub fn set_documents(&mut self, mut docs: Vec<u64>) {
        // Deterministic pseudo-shuffle for tests / consistent cycle
        docs.sort_by(|a, b| b.cmp(a));
        self.documents = docs;
        self.current_index = 0;
    }

    pub fn current_document_id(&self) -> Option<u64> {
        if self.documents.is_empty() {
            None
        } else {
            Some(self.documents[self.current_index % self.documents.len()])
        }
    }

    /// Upstream logic: `_timer` callback advancing sticker index every 1500ms
    pub fn advance_sticker(&mut self) {
        if !self.documents.is_empty() {
            self.current_index = (self.current_index + 1) % self.documents.len();
        }
    }

    /// Upstream icon centering: `(slotSize - iconSize) / 2`
    pub fn compute_icon_offset(&self) -> (f32, f32) {
        let offset = ((self.slot_size - self.icon_size) / 2.0).max(0.0);
        (offset, offset)
    }
}

impl Default for InfoUserpicEmojiBuilderMenuItemUserpic {
    fn default() -> Self {
        Self::new(Vec::new(), false)
    }
}

impl Render for InfoUserpicEmojiBuilderMenuItemUserpic {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let (ox, oy) = self.compute_icon_offset();
        let doc_id = self.current_document_id();

        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(10.0))
            .p(px(8.0))
            .rounded(px(4.0))
            .cursor_pointer()
            .hover(|this| this.bg(rgb(0x242f3d)))
            .child(
                div()
                    .w(px(self.slot_size))
                    .h(px(self.slot_size))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .w(px(self.icon_size))
                            .h(px(self.icon_size))
                            .rounded(px(4.0))
                            .bg(rgb(0x5288c1))
                            .child(div().text_xs().text_color(rgb(0xffffff)).child("★")),
                    ),
            )
            .child(
                div()
                    .text_sm()
                    .child(self.title.clone()),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_item_sticker_cycling() {
        let mut item = InfoUserpicEmojiBuilderMenuItemUserpic::new(vec![10, 20, 30], false);
        assert_eq!(item.current_document_id(), Some(10));

        item.advance_sticker();
        assert_eq!(item.current_document_id(), Some(20));

        item.advance_sticker();
        assert_eq!(item.current_document_id(), Some(30));

        item.advance_sticker();
        assert_eq!(item.current_document_id(), Some(10)); // Wrapped around
    }

    #[test]
    fn test_menu_item_icon_offset() {
        let item = InfoUserpicEmojiBuilderMenuItemUserpic::default();
        let (ox, oy) = item.compute_icon_offset();
        assert_eq!(ox, (24.0 - 20.0) / 2.0); // 2.0
        assert_eq!(oy, 2.0);
    }
}
