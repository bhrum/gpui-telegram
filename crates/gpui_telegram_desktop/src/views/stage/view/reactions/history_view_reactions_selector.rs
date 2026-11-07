//! History View Reactions Selector View
//! Faithful upstream C++ extraction:
//! - `history_view_reactions_selector.cpp:L45-L160`

use gpui::*;
use crate::views::stage::reactions::selector::{ReactionsSelector, SelectorState, ReactionCategory};
use crate::views::stage::reactions::button::ReactionKey;

#[derive(Debug, Clone)]
pub struct HistoryViewReactionsSelectorViewReactions {
    pub inner: ReactionsSelector,
    pub is_bubble_up: bool,
    pub is_expand_down: bool,
    pub special_expand_top_skip: f32,
}

impl HistoryViewReactionsSelectorViewReactions {
    pub fn new() -> Self {
        Self {
            inner: ReactionsSelector::new(),
            is_bubble_up: false,
            is_expand_down: false,
            special_expand_top_skip: 0.0,
        }
    }

    pub fn set_expanded(&mut self, expanded: bool) {
        self.inner.set_expanded(expanded);
    }

    pub fn update_show_state(&mut self, progress: f32, opacity: f32, appearing: bool) {
        self.inner.update_show_state(progress, opacity, appearing);
    }

    pub fn choose_reaction(&mut self, _key: ReactionKey) {
        // Selection event
    }
}

impl Default for HistoryViewReactionsSelectorViewReactions {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewReactionsSelectorViewReactions {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let items = self.inner.filtered_reactions();

        div()
            .flex()
            .flex_col()
            .p(px(8.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(14.0))
            .shadow_lg()
            .w(px(260.0))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap(px(4.0))
                    .children(items.into_iter().map(|item| {
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(26.0))
                            .h(px(26.0))
                            .child(item.key.display_label())
                    }))
            )
    }
}
