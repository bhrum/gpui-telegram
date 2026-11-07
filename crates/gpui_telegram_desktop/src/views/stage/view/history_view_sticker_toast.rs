//! Sticker Added / Viewed Toast Notification View
//! Faithful upstream C++ extraction:
//! - `history_view_sticker_toast.cpp:L20-L75`
//! - `history_view_sticker_toast.h:L20-L40`

use gpui::*;

#[derive(Debug, Clone)]
pub struct HistoryViewStickerToastView {
    pub pack_title: String,
    pub sticker_emoji: String,
    pub is_added: bool,
    pub is_visible: bool,
}

impl HistoryViewStickerToastView {
    pub fn new(pack_title: String, sticker_emoji: String, is_added: bool) -> Self {
        Self {
            pack_title,
            sticker_emoji,
            is_added,
            is_visible: true,
        }
    }

    pub fn dismiss(&mut self) {
        self.is_visible = false;
    }
}

impl Default for HistoryViewStickerToastView {
    fn default() -> Self {
        Self::new("Hot Cherry".into(), "🍒".into(), true)
    }
}

impl Render for HistoryViewStickerToastView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }

        let title = &self.pack_title;
        let emoji = &self.sticker_emoji;
        let action = if self.is_added { "added to your stickers" } else { "view pack" };

        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .px(px(14.0))
            .py(px(8.0))
            .bg(rgba(0x1F2937EE))
            .rounded(px(16.0))
            .shadow_lg()
            .text_size(px(13.0))
            .text_color(rgb(0xFFFFFF))
            .child(
                div()
                    .text_size(px(18.0))
                    .child(emoji.clone())
            )
            .child(
                div()
                    .child(format!("{} {}", title, action))
            )
            .into_any_element()
    }
}
