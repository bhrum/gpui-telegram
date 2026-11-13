//! Upstream: Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder.h
//! Upstream: Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder.cpp
//! Subsystem: info | Feature ID: TCG-N34 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use super::info_userpic_emoji_builder_common::{EmojiBuilderResult, EmojiBuilderStartData};

pub const K_BACK_BUTTON_POS_X: f32 = 8.0;
pub const K_BACK_BUTTON_POS_Y: f32 = 8.0;
pub const K_SAVE_BUTTON_POS_X: f32 = 7.0;
pub const K_SAVE_BUTTON_POS_Y: f32 = 12.0;

#[derive(Debug, Clone)]
pub struct InfoUserpicEmojiBuilderUserpic {
    pub is_layer_open: bool,
    pub start_data: EmojiBuilderStartData,
    pub last_result: Option<EmojiBuilderResult>,
}

impl InfoUserpicEmojiBuilderUserpic {
    pub fn new(start_data: EmojiBuilderStartData) -> Self {
        Self {
            is_layer_open: false,
            start_data,
            last_result: None,
        }
    }

    pub fn show_layer(&mut self) {
        self.is_layer_open = true;
    }

    pub fn close_layer(&mut self) {
        self.is_layer_open = false;
    }

    pub fn submit_result(&mut self, result: EmojiBuilderResult) {
        self.last_result = Some(result);
        self.is_layer_open = false;
    }
}

impl Default for InfoUserpicEmojiBuilderUserpic {
    fn default() -> Self {
        Self::new(EmojiBuilderStartData::default())
    }
}

impl Render for InfoUserpicEmojiBuilderUserpic {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_open = self.is_layer_open;

        let mut root = div().flex().flex_col().size_full();

        if is_open {
            root = root.child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .p(px(12.0))
                    .bg(rgb(0x17212b))
                    .border_b_1()
                    .border_color(rgb(0x242f3d))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .child("Emoji & Sticker Avatar"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x5288c1))
                                    .cursor_pointer()
                                    .child("Save"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x707579))
                                    .cursor_pointer()
                                    .child("Back"),
                            ),
                    ),
            );
        }

        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emoji_builder_lifecycle() {
        let mut builder = InfoUserpicEmojiBuilderUserpic::default();
        assert!(!builder.is_layer_open);
        assert!(builder.last_result.is_none());

        builder.show_layer();
        assert!(builder.is_layer_open);

        let result = EmojiBuilderResult {
            document_id: 12345,
            colors: vec![0x20e2cdff, 0x4d8dffff],
            is_forum: false,
        };
        builder.submit_result(result);
        assert!(!builder.is_layer_open);
        assert!(builder.last_result.is_some());
        assert_eq!(builder.last_result.unwrap().document_id, 12345);
    }
}
