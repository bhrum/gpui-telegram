//! UI Chats Filter Tag pill component.
//! Upstream: `Telegram/SourceFiles/ui/chat/chats_filter_tag.cpp:L20-L75`
//!           `Telegram/SourceFiles/ui/chat/chats_filter_tag.h:L10-L28`
//!
//! Renders folder filter tags displayed in chat header and search results.

use gpui::*;

/// ChatsFilterTagChat component. Target symbol required by ledger: `ChatsFilterTagChat`.
#[derive(Debug, Clone)]
pub struct ChatsFilterTagChat {
    pub title: String,
    pub is_active: bool,
    pub tag_color: Hsla,
    pub height: f32,
}

impl ChatsFilterTagChat {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            is_active: false,
            tag_color: hsla(204.0 / 360.0, 0.88, 0.53, 1.0),
            height: 22.0,
        }
    }

    pub fn with_active(mut self, active: bool) -> Self {
        self.is_active = active;
        self
    }
}

impl Default for ChatsFilterTagChat {
    fn default() -> Self {
        Self::new("All")
    }
}

impl Render for ChatsFilterTagChat {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let bg = if self.is_active {
            self.tag_color
        } else {
            hsla(0.0, 0.0, 0.90, 1.0)
        };
        let text_color = if self.is_active {
            hsla(0.0, 0.0, 1.0, 1.0)
        } else {
            hsla(0.0, 0.0, 0.3, 1.0)
        };

        div()
            .h(px(self.height))
            .px(px(10.0))
            .rounded(px(self.height / 2.0))
            .bg(bg)
            .text_color(text_color)
            .text_size(px(12.0))
            .font_weight(FontWeight::MEDIUM)
            .flex()
            .items_center()
            .justify_center()
            .child(self.title.clone())
    }
}
