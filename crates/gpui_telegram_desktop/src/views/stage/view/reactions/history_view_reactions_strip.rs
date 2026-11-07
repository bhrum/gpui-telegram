//! History View Reactions Strip View
//! Faithful upstream C++ extraction:
//! - `history_view_reactions_strip.cpp:L20-L80`

use gpui::*;
use crate::views::stage::reactions::button::ReactionKey;
use crate::views::stage::reactions::strip::{ReactionsStrip, K_HOVER_SCALE, K_HOVER_SCALE_DURATION_MS};

#[derive(Debug, Clone)]
pub struct HistoryViewReactionsStripViewReactions {
    pub inner: ReactionsStrip,
    pub is_visible: bool,
}

impl HistoryViewReactionsStripViewReactions {
    pub fn new() -> Self {
        Self {
            inner: ReactionsStrip::default(),
            is_visible: true,
        }
    }

    pub fn set_hovered(&mut self, index: Option<usize>) {
        self.inner.set_hovered(index);
    }

    pub fn select(&mut self, index: usize) -> Option<ReactionKey> {
        self.inner.select_reaction(index)
    }
}

impl Default for HistoryViewReactionsStripViewReactions {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewReactionsStripViewReactions {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }

        let hovered = self.inner.hovered_index;

        div()
            .flex()
            .items_center()
            .h(px(36.0))
            .px(px(8.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(18.0))
            .shadow_md()
            .gap(px(6.0))
            .children(self.inner.items.iter().enumerate().map(|(idx, item)| {
                let is_hovered = hovered == Some(idx);
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(26.0))
                    .h(px(26.0))
                    .text_size(if is_hovered { px(18.0) } else { px(15.0) })
                    .child(item.key.display_label())
            }))
            .into_any_element()
    }
}
