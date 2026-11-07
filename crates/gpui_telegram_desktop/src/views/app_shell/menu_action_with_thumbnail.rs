//! MenuActionWithThumbnail: Context menu item with dynamic avatar or media thumbnail
//! Upstream: Telegram/SourceFiles/menu/menu_action_with_thumbnail.h, menu_action_with_thumbnail.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct MenuActionWithThumbnail {
    pub label: String,
    pub thumbnail_path: Option<String>,
    pub thumbnail_size: f32,
    pub is_enabled: bool,
}

impl MenuActionWithThumbnail {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            thumbnail_path: None,
            thumbnail_size: 20.0,
            is_enabled: true,
        }
    }

    pub fn with_thumbnail(mut self, path: impl Into<String>, size: f32) -> Self {
        self.thumbnail_path = Some(path.into());
        self.thumbnail_size = size;
        self
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }
}

impl Default for MenuActionWithThumbnail {
    fn default() -> Self {
        Self::new("Action")
    }
}

impl Render for MenuActionWithThumbnail {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .p(px(8.0))
            .text_xs()
            .text_color(rgb(0xFFFFFF))
            .children(self.thumbnail_path.as_ref().map(|_| {
                div().size(px(self.thumbnail_size)).rounded_full().bg(rgb(0x5288C1))
            }))
            .child(self.label.clone())
    }
}
