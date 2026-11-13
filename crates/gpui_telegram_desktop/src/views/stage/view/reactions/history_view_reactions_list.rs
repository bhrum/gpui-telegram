//! History View Reactions List View
//! Faithful upstream C++ extraction:
//! - `history_view_reactions_list.cpp:L38-L120`

use gpui::*;
use crate::views::stage::reactions::button::ReactionKey;
use crate::views::stage::reactions::list::{ReactorItem, WhoReactedList, K_PER_PAGE, K_PER_PAGE_FIRST};

#[derive(Debug, Clone)]
pub struct HistoryViewReactionsListViewReactions {
    pub message_id: u64,
    pub inner_list: WhoReactedList,
    pub page_size: usize,
    pub has_more: bool,
}

impl HistoryViewReactionsListViewReactions {
    pub fn new(message_id: u64, can_moderate: bool) -> Self {
        Self {
            message_id,
            inner_list: WhoReactedList::new(Vec::new(), can_moderate),
            page_size: K_PER_PAGE_FIRST,
            has_more: false,
        }
    }

    pub fn add_reactor(&mut self, peer_id: u64, name: String, reaction: ReactionKey, paid_stars: Option<u64>) {
        self.inner_list.items.push(ReactorItem {
            peer_id,
            name,
            avatar_url: None,
            reaction,
            date: chrono::Utc::now().timestamp(),
            paid_stars,
        });
    }

    pub fn set_tab(&mut self, tab: Option<ReactionKey>) {
        self.inner_list.active_tab = tab;
    }

    pub fn can_moderate(&self) -> bool {
        self.inner_list.can_moderate_by_delete_messages()
    }
}

impl Default for HistoryViewReactionsListViewReactions {
    fn default() -> Self {
        Self::new(0, false)
    }
}

impl Render for HistoryViewReactionsListViewReactions {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let entries = self.inner_list.filter_by_tab();

        div()
            .flex()
            .flex_col()
            .w(px(300.0))
            .p(px(8.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(10.0))
            .shadow_md()
            .children(entries.into_iter().map(|item| {
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .py(px(4.0))
                    .child(
                        div()
                            .text_size(px(13.0))
                            .child(item.name.clone())
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .child(item.reaction.display_label())
                    )
            }))
    }
}
