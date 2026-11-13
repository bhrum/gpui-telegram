//! GPUI Telegram Chat Stage Item View Representation
//!
//! Citations:
//! - `Telegram/SourceFiles/history/history_item.h:L89-L107`
//! - `Telegram/SourceFiles/history/history_item_components.h:L109-L128`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use crate::views::stage::history_item::HistoryItem;
use crate::views::stage::history_item_components::HistoryItemComponents;
use crate::views::stage::history_item_reply_markup::HistoryItemReplyMarkup;

/// Unified visual stage presentation of a history item
#[derive(Debug, Clone, Default)]
pub struct StageItemView {
    pub item: HistoryItem,
    pub components: HistoryItemComponents,
    pub reply_markup: Option<HistoryItemReplyMarkup>,
    pub is_highlighted: bool,
}

impl StageItemView {
    pub fn new(item: HistoryItem) -> Self {
        Self {
            item,
            components: HistoryItemComponents::default(),
            reply_markup: None,
            is_highlighted: false,
        }
    }

    pub fn with_components(mut self, components: HistoryItemComponents) -> Self {
        self.components = components;
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: HistoryItemReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

    pub fn set_highlighted(&mut self, highlighted: bool) {
        self.is_highlighted = highlighted;
        self.item.set_highlighted(highlighted);
    }
}

impl Render for StageItemView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut container = div().flex().flex_col().w_full().gap(px(4.0));

        if self.components.has_components() {
            container = container.child(self.components.render_view());
        }

        container = container.child(self.item.render_view());

        if let Some(markup) = &self.reply_markup {
            container = container.child(markup.render_view());
        }

        container
    }
}
