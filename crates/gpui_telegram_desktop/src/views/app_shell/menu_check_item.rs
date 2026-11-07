//! MenuCheckItem: Toggleable checkbox item within popup menus
//! Upstream: Telegram/SourceFiles/menu/menu_check_item.h, menu_check_item.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct MenuCheckItem {
    pub text: String,
    pub is_checked: bool,
    pub prevent_close: bool,
}

impl MenuCheckItem {
    pub fn new(text: impl Into<String>, checked: bool) -> Self {
        Self {
            text: text.into(),
            is_checked: checked,
            prevent_close: false,
        }
    }

    pub fn toggle(&mut self) -> bool {
        self.is_checked = !self.is_checked;
        self.is_checked
    }

    pub fn set_checked(&mut self, checked: bool) {
        self.is_checked = checked;
    }
}

impl Default for MenuCheckItem {
    fn default() -> Self {
        Self::new("Check Item", false)
    }
}

impl Render for MenuCheckItem {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_between()
            .p(px(8.0))
            .text_xs()
            .text_color(rgb(0xFFFFFF))
            .child(self.text.clone())
            .child(if self.is_checked { "✓" } else { "" })
    }
}
