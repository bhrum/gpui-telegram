//! WindowThemeEditorBoxThemes: Modal box for saving, naming, and exporting custom themes
//! Upstream: Telegram/SourceFiles/window/themes/window_theme_editor_box.h, window_theme_editor_box.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct WindowThemeEditorBoxThemes {
    pub theme_title: String,
    pub author_name: String,
    pub is_open: bool,
}

impl WindowThemeEditorBoxThemes {
    pub fn new() -> Self {
        Self {
            theme_title: "My Custom Theme".to_string(),
            author_name: String::new(),
            is_open: false,
        }
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn dismiss(&mut self) {
        self.is_open = false;
    }
}

impl Default for WindowThemeEditorBoxThemes {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowThemeEditorBoxThemes {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0x82868A))
            .child(format!("ThemeEditorBox: title='{}', open={}", self.theme_title, self.is_open))
    }
}
