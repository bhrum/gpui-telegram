//! WindowFiltersMenu: Chat folder tab bar and filter selection manager
//! Upstream: Telegram/SourceFiles/window/window_filters_menu.h, window_filters_menu.cpp

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterTab {
    pub id: i32,
    pub title: String,
    pub unread_count: i32,
    pub icon_name: String,
}

#[derive(Debug, Clone)]
pub struct WindowFiltersMenu {
    pub active_filter_id: i32,
    pub tabs: Vec<FilterTab>,
}

impl WindowFiltersMenu {
    pub fn new() -> Self {
        Self {
            active_filter_id: 0,
            tabs: vec![
                FilterTab {
                    id: 0,
                    title: "All Chats".to_string(),
                    unread_count: 0,
                    icon_name: "chats".to_string(),
                },
                FilterTab {
                    id: 1,
                    title: "Personal".to_string(),
                    unread_count: 0,
                    icon_name: "user".to_string(),
                },
                FilterTab {
                    id: 2,
                    title: "Work".to_string(),
                    unread_count: 0,
                    icon_name: "briefcase".to_string(),
                },
            ],
        }
    }

    pub fn select_tab(&mut self, id: i32) {
        self.active_filter_id = id;
    }

    pub fn add_tab(&mut self, tab: FilterTab) {
        self.tabs.push(tab);
    }

    pub fn remove_tab(&mut self, id: i32) {
        self.tabs.retain(|t| t.id != id);
        if self.active_filter_id == id {
            self.active_filter_id = 0;
        }
    }

    pub fn active_tab(&self) -> Option<&FilterTab> {
        self.tabs.iter().find(|t| t.id == self.active_filter_id)
    }
}

impl Default for WindowFiltersMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowFiltersMenu {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap(px(4.0))
            .p(px(4.0))
            .bg(rgb(0x17212B))
            .children(self.tabs.iter().map(|tab| {
                let is_active = tab.id == self.active_filter_id;
                div()
                    .px(px(12.0))
                    .py(px(6.0))
                    .rounded(px(14.0))
                    .bg(if is_active { rgb(0x2B5278) } else { rgba(0x00000000) })
                    .text_color(if is_active { rgb(0xFFFFFF) } else { rgb(0x82868A) })
                    .text_xs()
                    .child(tab.title.clone())
            }))
    }
}
