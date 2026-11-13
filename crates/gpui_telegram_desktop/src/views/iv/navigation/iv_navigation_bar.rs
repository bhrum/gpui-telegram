//! Navigation bar for Instant View
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

#[derive(Debug, Clone, Default)]
pub struct IvNavigationBar {
    pub title: String,
    pub can_back: bool,
    pub can_forward: bool,
}

impl IvNavigationBar {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            can_back: false,
            can_forward: false,
        }
    }

    pub fn set_navigation_state(&mut self, can_back: bool, can_forward: bool) {
        self.can_back = can_back;
        self.can_forward = can_forward;
    }
}

impl Render for IvNavigationBar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let back_color = if self.can_back { rgb(0xffffff) } else { rgb(0x666666) };
        let forward_color = if self.can_forward { rgb(0xffffff) } else { rgb(0x666666) };

        div()
            .flex()
            .items_center()
            .justify_between()
            .h_12()
            .px_4()
            .bg(rgb(0x18191c))
            .border_b_1()
            .border_color(rgb(0x282a2e))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(div().text_color(back_color).child("←"))
                    .child(div().text_color(forward_color).child("→")),
            )
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xffffff))
                    .child(self.title.clone()),
            )
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(div().text_color(rgb(0xaaaaaa)).child("Share"))
                    .child(div().text_color(rgb(0xaaaaaa)).child("✕")),
            )
    }
}
