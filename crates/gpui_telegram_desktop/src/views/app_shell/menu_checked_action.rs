//! MenuCheckedAction: Radio-group selectable action within context menus
//! Upstream: Telegram/SourceFiles/menu/menu_checked_action.h, menu_checked_action.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct MenuCheckedAction {
    pub group_id: u32,
    pub action_id: u32,
    pub label: String,
    pub is_selected: bool,
}

impl MenuCheckedAction {
    pub fn new(group_id: u32, action_id: u32, label: impl Into<String>, selected: bool) -> Self {
        Self {
            group_id,
            action_id,
            label: label.into(),
            is_selected: selected,
        }
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.is_selected = selected;
    }
}

impl Default for MenuCheckedAction {
    fn default() -> Self {
        Self::new(0, 0, "Option", false)
    }
}

impl Render for MenuCheckedAction {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_between()
            .p(px(8.0))
            .text_xs()
            .text_color(rgb(0xFFFFFF))
            .child(self.label.clone())
            .child(if self.is_selected { "●" } else { "○" })
    }
}
