//! WindowThemeEditorThemes: Interactive in-app palette inspector and theme editor
//! Upstream: Telegram/SourceFiles/window/themes/window_theme_editor.h, window_theme_editor.cpp

use gpui::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct WindowThemeEditorThemes {
    pub theme_name: String,
    pub palette_overrides: HashMap<String, u32>,
    pub is_dirty: bool,
    pub selected_token: Option<String>,
}

impl WindowThemeEditorThemes {
    pub fn new() -> Self {
        Self {
            theme_name: "Custom Theme".to_string(),
            palette_overrides: HashMap::new(),
            is_dirty: false,
            selected_token: None,
        }
    }

    pub fn set_token_color(&mut self, token: impl Into<String>, color: u32) {
        let t = token.into();
        self.palette_overrides.insert(t.clone(), color);
        self.selected_token = Some(t);
        self.is_dirty = true;
    }

    pub fn remove_token_override(&mut self, token: &str) -> Option<u32> {
        let res = self.palette_overrides.remove(token);
        if res.is_some() {
            self.is_dirty = !self.palette_overrides.is_empty();
        }
        res
    }

    pub fn reset_all(&mut self) {
        self.palette_overrides.clear();
        self.is_dirty = false;
        self.selected_token = None;
    }
}

impl Default for WindowThemeEditorThemes {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowThemeEditorThemes {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0x82868A))
            .child(format!(
                "ThemeEditor: '{}', {} overrides, dirty={}",
                self.theme_name,
                self.palette_overrides.len(),
                self.is_dirty
            ))
    }
}
