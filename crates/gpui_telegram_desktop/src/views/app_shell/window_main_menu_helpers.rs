//! WindowMainMenuHelpers: Layout calculators, icon generators, and badge formatters for main menu items
//! Upstream: Telegram/SourceFiles/window/window_main_menu_helpers.h, window_main_menu_helpers.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct WindowMainMenuHelpers;

impl WindowMainMenuHelpers {
    pub fn new() -> Self {
        Self
    }

    pub fn format_badge_count(count: i32) -> String {
        if count <= 0 {
            String::new()
        } else if count > 999 {
            "999+".to_string()
        } else {
            count.to_string()
        }
    }

    pub fn menu_item_height() -> f32 {
        44.0
    }

    pub fn icon_size() -> f32 {
        24.0
    }

    pub fn drawer_width() -> f32 {
        280.0
    }
}

impl Default for WindowMainMenuHelpers {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowMainMenuHelpers {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0x82868A))
            .child("MainMenuHelpers")
    }
}
