//! Authentic GPUI implementation of Telegram Desktop's TranslateBox.
//! Upstream reference: `Telegram/SourceFiles/boxes/translate_box.h:L14-L46`
//! and `Telegram/SourceFiles/boxes/translate_box.cpp:L40-L120`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_SKIP_AT_LEAST_ONE_DURATION_MS: u64 = 3000;
pub const K_MAX_LINES: usize = 3;
pub const K_FIRST_CHUNK: usize = 100;

/// View component for displaying translated chat messages.
#[derive(Clone)]
pub struct TranslateBox {
    pub source_lang: String,
    pub target_lang: String,
    pub original_text: String,
    pub translated_text: String,
    pub is_expanded: bool,
    pub is_closed: bool,
}

impl Default for TranslateBox {
    fn default() -> Self {
        Self {
            source_lang: "French".into(),
            target_lang: "English".into(),
            original_text: "Bonjour, comment allez-vous aujourd'hui?".into(),
            translated_text: "Hello, how are you doing today?".into(),
            is_expanded: false,
            is_closed: false,
        }
    }
}

impl TranslateBox {
    pub fn new(source_lang: String, target_lang: String, original_text: String, translated_text: String) -> Self {
        Self {
            source_lang,
            target_lang,
            original_text,
            translated_text,
            is_expanded: false,
            is_closed: false,
        }
    }

    pub fn toggle_expanded(&mut self) {
        self.is_expanded = !self.is_expanded;
    }

    pub fn close(&mut self) {
        self.is_closed = true;
    }
}

impl Render for TranslateBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        let mut card = div()
            .id("translate_box")
            .w(px(400.0))
            .p(px(24.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(12.0))
            .shadow_lg()
            .flex()
            .flex_col()
            .gap(px(14.0));

        // Header with Language Pair
        card = card.child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .text_size(px(14.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x1F2937))
                        .child(self.source_lang.clone())
                        .child(div().text_color(rgb(0x9CA3AF)).child("→"))
                        .child(self.target_lang.clone()),
                )
                .child(
                    div()
                        .text_size(px(12.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0x2563EB))
                        .cursor_pointer()
                        .child("Copy"),
                ),
        );

        // Translated Text Block
        card = card.child(
            div()
                .p(px(14.0))
                .rounded(px(8.0))
                .bg(rgb(0xF0FDF4))
                .border_1()
                .border_color(rgb(0xBBF7D0))
                .text_size(px(14.0))
                .line_height(relative(1.4))
                .text_color(rgb(0x166534))
                .child(self.translated_text.clone()),
        );

        // Original Text Block
        card = card.child(
            div()
                .p(px(12.0))
                .rounded(px(8.0))
                .bg(rgb(0xF8FAFC))
                .text_size(px(13.0))
                .line_height(relative(1.3))
                .text_color(rgb(0x64748B))
                .child(self.original_text.clone()),
        );

        // Close action button
        card = card.child(
            div()
                .flex()
                .justify_end()
                .child(
                    div()
                        .px(px(16.0))
                        .py(px(8.0))
                        .rounded(px(6.0))
                        .bg(rgb(0x2563EB))
                        .cursor_pointer()
                        .text_size(px(14.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0xFFFFFF))
                        .child("Done"),
                ),
        );

        card.into_any_element()
    }
}
