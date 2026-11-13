//! WindowThemeEditorBlockThemes: Collapsible category group within the theme editor
//! Upstream: Telegram/SourceFiles/window/themes/window_theme_editor_block.h, window_theme_editor_block.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct WindowThemeEditorBlockThemes {
    pub title: String,
    pub tokens: Vec<String>,
    pub is_expanded: bool,
}

impl WindowThemeEditorBlockThemes {
    pub fn new(title: impl Into<String>, tokens: Vec<String>) -> Self {
        Self {
            title: title.into(),
            tokens,
            is_expanded: true,
        }
    }

    pub fn toggle_expand(&mut self) -> bool {
        self.is_expanded = !self.is_expanded;
        self.is_expanded
    }
}

impl Default for WindowThemeEditorBlockThemes {
    fn default() -> Self {
        Self::new("General Colors", vec!["windowBg".to_string(), "windowFg".to_string()])
    }
}

impl Render for WindowThemeEditorBlockThemes {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0xFFFFFF))
            .child(format!("Block: {} ({} tokens)", self.title, self.tokens.len()))
    }
}
