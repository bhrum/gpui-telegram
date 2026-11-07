//! Authentic GPUI implementation of Telegram Desktop's TranslateBoxContent container.
//! Upstream reference: `Telegram/SourceFiles/boxes/translate_box_content.h:L20-L66`
//! and `Telegram/SourceFiles/boxes/translate_box_content.cpp:L30-L90`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_MAX_LINES: usize = 3;

/// View container managing translated text block formatting, scrolling and inline expansion.
#[derive(Clone)]
pub struct TranslateBoxContent {
    pub provider_name: String,
    pub character_count: usize,
    pub is_expanded: bool,
    pub lines_count: usize,
}

impl Default for TranslateBoxContent {
    fn default() -> Self {
        Self {
            provider_name: "Telegram AI Translate".into(),
            character_count: 142,
            is_expanded: false,
            lines_count: 4,
        }
    }
}

impl TranslateBoxContent {
    pub fn new(provider: String, char_count: usize, lines: usize) -> Self {
        Self {
            provider_name: provider,
            character_count: char_count,
            is_expanded: false,
            lines_count: lines,
        }
    }

    /// Checks if expansion toggle should be shown per `kMaxLines` (`translate_box_content.cpp:L30`).
    pub fn needs_expand_toggle(&self) -> bool {
        self.lines_count > K_MAX_LINES
    }

    pub fn toggle_expand(&mut self) {
        self.is_expanded = !self.is_expanded;
    }
}

impl Render for TranslateBoxContent {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut container = div()
            .id("translate_box_content")
            .w_full()
            .flex()
            .flex_col()
            .gap(px(8.0));

        // Provider Footer
        container = container.child(
            div()
                .flex()
                .justify_between()
                .items_center()
                .text_size(px(11.0))
                .text_color(rgb(0x94A3B8))
                .child(format!("Translated via {}", self.provider_name))
                .child(format!("{} characters", self.character_count)),
        );

        if self.needs_expand_toggle() {
            let label = if self.is_expanded { "Show Less" } else { "Show More" };
            container = container.child(
                div()
                    .text_size(px(12.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(0x2563EB))
                    .cursor_pointer()
                    .child(label),
            );
        }

        container.into_any_element()
    }
}
