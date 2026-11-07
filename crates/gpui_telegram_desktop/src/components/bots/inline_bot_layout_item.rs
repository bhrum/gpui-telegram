//! InlineBotLayoutItem: Base layout item for inline bot results with hit-testing & preloading.
//!
//! Upstream reference:
//! - `Telegram/SourceFiles/inline_bots/inline_bot_layout_item.h:L27-L80`
//! - `Telegram/SourceFiles/inline_bots/inline_bot_layout_item.cpp:L32-L90`
#![allow(dead_code)]

use gpui::*;

/// State of an inline result item within the layout container.
#[derive(Debug, Clone, PartialEq)]
pub struct InlineItemLayoutState {
    pub id: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub is_selected: bool,
    pub is_preloaded: bool,
    pub has_thumbnail: bool,
    pub document_id: Option<u64>,
    pub photo_id: Option<u64>,
}

impl InlineItemLayoutState {
    pub fn contains_point(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= (self.x + self.width) && py >= self.y && py <= (self.y + self.height)
    }
}

/// Authentic InlineBotLayoutItem state.
pub struct InlineBotLayoutItem {
    pub item_state: InlineItemLayoutState,
    pub updated_at: i64,
}

impl InlineBotLayoutItem {
    pub fn new(id: impl Into<String>, x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            item_state: InlineItemLayoutState {
                id: id.into(),
                x,
                y,
                width,
                height,
                is_selected: false,
                is_preloaded: false,
                has_thumbnail: false,
                document_id: None,
                photo_id: None,
            },
            updated_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn preload(&mut self) {
        self.item_state.is_preloaded = true;
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.item_state.is_selected = selected;
        self.updated_at = chrono::Utc::now().timestamp();
    }
}

impl Default for InlineBotLayoutItem {
    fn default() -> Self {
        Self::new("default_item", 0.0, 0.0, 100.0, 100.0)
    }
}

impl Render for InlineBotLayoutItem {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_sel = self.item_state.is_selected;

        div()
            .w(px(self.item_state.width))
            .h(px(self.item_state.height))
            .rounded(px(4.0))
            .bg(if is_sel { rgb(0xE0E0E0) } else { rgb(0xF5F5F5) })
            .border_1()
            .border_color(if is_sel { rgb(0x2AABEE) } else { rgb(0xDDDDDD) })
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .text_size(px(11.0))
                    .text_color(rgb(0x666666))
                    .child(self.item_state.id.clone())
            )
    }
}
