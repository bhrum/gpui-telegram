//! History View Reactions Tabs View
//! Faithful upstream C++ extraction:
//! - `history_view_reactions_tabs.cpp:L20-L80`

use gpui::*;
use crate::views::stage::reactions::button::ReactionKey;

#[derive(Debug, Clone)]
pub struct ReactionTabItem {
    pub key: Option<ReactionKey>, // None = All
    pub count: i32,
    pub is_selected: bool,
}

#[derive(Debug, Clone)]
pub struct HistoryViewReactionsTabsViewReactions {
    pub tabs: Vec<ReactionTabItem>,
    pub selected_index: usize,
}

impl HistoryViewReactionsTabsViewReactions {
    pub fn new(tabs: Vec<ReactionTabItem>) -> Self {
        Self {
            tabs,
            selected_index: 0,
        }
    }

    pub fn select_tab(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.selected_index = index;
            for (i, tab) in self.tabs.iter_mut().enumerate() {
                tab.is_selected = (i == index);
            }
        }
    }

    pub fn current_selected_key(&self) -> Option<ReactionKey> {
        self.tabs.get(self.selected_index).and_then(|t| t.key.clone())
    }
}

impl Default for HistoryViewReactionsTabsViewReactions {
    fn default() -> Self {
        Self::new(vec![
            ReactionTabItem { key: None, count: 12, is_selected: true },
            ReactionTabItem { key: Some(ReactionKey::Emoji("👍".into())), count: 8, is_selected: false },
            ReactionTabItem { key: Some(ReactionKey::Emoji("❤️".into())), count: 4, is_selected: false },
        ])
    }
}

impl Render for HistoryViewReactionsTabsViewReactions {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let tabs = &self.tabs;

        div()
            .flex()
            .items_center()
            .gap(px(6.0))
            .p(px(4.0))
            .children(tabs.iter().map(|tab| {
                let label = match &tab.key {
                    None => "All".to_string(),
                    Some(k) => k.display_label(),
                };

                div()
                    .flex()
                    .items_center()
                    .gap(px(4.0))
                    .px(px(8.0))
                    .h(px(24.0))
                    .rounded(px(12.0))
                    .bg(if tab.is_selected { rgb(0x2AABEE) } else { rgba(0x0000000A) })
                    .text_color(if tab.is_selected { rgb(0xFFFFFF) } else { rgb(0x333333) })
                    .text_size(px(12.0))
                    .child(label)
                    .child(format!("{}", tab.count))
            }))
    }
}
